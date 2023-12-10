use std::collections::LinkedList;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::request_body::{AiApiBody, DashScopeRequestBody, Message, OpenAiRequestBody};

#[derive(Serialize, Deserialize)]
pub struct RuntimeConfig {
    /**
     * The OpenAI token to use for authentication.
     */
    openai_token: String,
    /**
     * The base URL to use for the OpenAI API. Default value: https://api.openai.com/v1
     */
    base_url: String,
    /**
     * The path to use for the OpenAI API. Default value: /v1/chat/completions
     */
    request_path: Option<String>,
    /**
     * The maximum number of tokens to use for the OpenAI API. Default value: 1000
     */
    max_tokens: u16,
    /**
     * The model to use for the OpenAI API. Default value: gpt-3.5-turbo
     */
    model: String,
    /**
     * The default shell to use for the OpenAI API. If not set, the user will be guided through the shell selection process.
     */
    default_shell: Option<String>,

    /**
     * The prompt to use for the OpenAI API.
     */
    system_prompt: String,

    /**
     * The manufacturer of the AI model.
     * see [AiApiBody](./request_body/struct.AiApiBody.html) for more information.
     */
    ai_type: Option<String>,
}

impl RuntimeConfig {
    pub fn openai_token(&self) -> String {
        self.openai_token.to_string()
    }

    pub fn base_url(&self) -> String {
        self.base_url.to_string()
    }

    pub fn request_path(&self) -> String {
        match &self.request_path {
            Some(path) => path.to_string(),
            None => "/v1/chat/completions".to_string()
        }
    }

    pub fn default_shell(&self) -> String {
        match &self.default_shell {
            Some(shell) => shell.to_string(),
            // unknown shell
            None => "unknown".to_string()
        }
    }

    pub fn system_prompt(&self) -> String {
        self.system_prompt.to_string()
    }

    pub fn copy_from_json(&mut self, json: String) {
        let config: RuntimeConfig = serde_json::from_str(&json).unwrap();
        if !config.base_url.is_empty() {
            self.base_url = config.base_url.clone();
        }

        if !config.request_path.is_none() {
            self.request_path = config.request_path.clone();
        }

        if !config.openai_token.is_empty() {
            self.openai_token = config.openai_token.clone();
        }

        if !config.model.is_empty() {
            self.model = config.model.clone();
        }

        if !config.default_shell.is_none() {
            self.default_shell = config.default_shell.clone();
        }

        if !config.system_prompt.is_empty() {
            self.system_prompt = config.system_prompt.clone();
        }

        if config.max_tokens > 0 {
            self.max_tokens = config.max_tokens;
        }

        if !config.ai_type.is_none() {
            self.ai_type = config.ai_type.clone();
        }
    }

    pub fn create_request_body(&self, messages: LinkedList<Message>) -> AiApiBody {
        let type_string = &self.ai_type;
        match type_string {
            Some(type_string) => {
                match type_string.as_str() {
                    "DashScope" => {
                        let request_body = AiApiBody::DashScope(
                            DashScopeRequestBody::new(
                                self.model.clone(),
                                self.max_tokens,
                                messages,
                            )
                        );
                        request_body
                    },
                    _ => {
                        let request_body = AiApiBody::OpenAI(
                            OpenAiRequestBody {
                                model: self.model.clone(),
                                max_tokens: self.max_tokens,
                                messages,
                            }
                        );
                        request_body
                    }
                }
            },
            None => {
                let request_body = AiApiBody::OpenAI(
                    OpenAiRequestBody {
                        model: self.model.clone(),
                        max_tokens: self.max_tokens,
                        messages,
                    }
                );
                request_body
            }
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /**
     * Returns a new RuntimeConfig with default values.
     */
    pub fn default() -> RuntimeConfig {
        RuntimeConfig {
            openai_token: "".to_string(),
            base_url: "https://api.openai.com".to_string(),
            request_path: None,
            max_tokens: 1000,
            model: "gpt-3.5-turbo".to_string(),
            default_shell: None,
            system_prompt: "".to_string(),
            ai_type: None,
        }
    }
}

pub static GLOBAL_CONFIG: Lazy<RwLock<RuntimeConfig>> = Lazy::new(|| {
    RwLock::new(
        RuntimeConfig {
            openai_token: "".to_string(),
            base_url: "https://api.openai.com".to_string(),
            request_path: None,
            max_tokens: 1000,
            model: "gpt-3.5-turbo".to_string(),
            default_shell: None,
            system_prompt: "
If the terminal is capable of displaying emoji, they may be used appropriately at the beginning or end of lines in the Explanation section to enhance readability and engagement.
Goal: To provide users with executable command line instructions for the current environment and terminal, and offer clear explanations of the commands without describing user operation steps. The explanation should be concise and formatted for easy readability in the console, using line breaks or tabs to enhance readability. If the user's language is clear, respond in kind; otherwise, default to English.
Please format the response with a clear separation between the command and the explanation. The response should not contain Markdown formatting and should be structured as follows:
Command:
[The command line instruction that is executable in the current environment and terminal]

Explanation:
[A step-by-step explanation in the user's language, if discernible, or in English. Use line breaks and tabs to avoid overly long text and improve readability.]

Ensure there is a blank line between the 'Command:' and 'Explanation:' sections.
    ".to_string(),
            ai_type: None,
        }
    )
});