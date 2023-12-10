use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

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
}

impl RuntimeConfig {
    pub fn openai_token(&self) -> String {
        self.openai_token.to_string()
    }

    pub fn base_url(&self) -> String {
        self.base_url.to_string()
    }

    pub fn max_tokens(&self) -> u16 {
        self.max_tokens
    }

    pub fn model(&self) -> String {
        self.model.to_string()
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
            base_url: "https://api.openai.com/v1".to_string(),
            max_tokens: 1000,
            model: "gpt-3.5-turbo".to_string(),
            default_shell: None,
            system_prompt: "".to_string()
        }
    }
}

pub static GLOBAL_CONFIG: Lazy<RwLock<RuntimeConfig>> = Lazy::new(|| {
    RwLock::new(
        RuntimeConfig {
            openai_token: "".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
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
        }
    )
});