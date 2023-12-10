use std::collections::LinkedList;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/**
 * The request body to send to the OpenAI API.
 */
#[derive(Serialize, Deserialize)]
pub struct OpenAiRequestBody {
    pub model: String,
    pub messages: LinkedList<Message>,
    pub max_tokens: u16,
}

impl AiRequestBody for OpenAiRequestBody {
    fn to_json(&self) -> Value {
        json!(self)
    }

    fn get_messages(&self) -> &LinkedList<Message> {
        &self.messages
    }

    fn parse_response(&self, response_json: Value) -> Value {
        response_json
    }
}

/**
 * The request body to send to the ai API.
 */
pub trait AiRequestBody {
    fn to_json(&self) -> Value;

    fn get_messages(&self) -> &LinkedList<Message>;

    fn parse_response(&self, response_json: Value) -> Value;
}

/**
 * The request body to send to the DashScope API.
 */
#[derive(Serialize, Deserialize)]
pub struct DashScopeRequestBody {
    pub model: String,
    pub max_tokens: u16,
    pub input: DashScopeRequestInput,
    pub parameters: DashScopeRequestParameters,
}

impl AiRequestBody for DashScopeRequestBody {
    fn to_json(&self) -> Value {
        json!(self)
    }

    fn get_messages(&self) -> &LinkedList<Message> {
        &self.input.messages
    }

    fn parse_response(&self, response_json: Value) -> Value {
        let a = &response_json["output"];
        a.clone()
    }
}

impl DashScopeRequestBody {
    pub fn new(model: String, max_tokens: u16, messages: LinkedList<Message>) -> Self {
        Self {
            model,
            max_tokens,
            input: DashScopeRequestInput {
                messages,
            },
            parameters: DashScopeRequestParameters {
                result_format: "message".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DashScopeRequestInput {
    pub messages: LinkedList<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct DashScopeRequestParameters {
    pub result_format: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub(crate) role: String,
    pub(crate) content: String,
}

pub enum AiApiBody {
    OpenAI(OpenAiRequestBody),
    DashScope(DashScopeRequestBody),
}

impl AiRequestBody for AiApiBody {
    fn to_json(&self) -> Value {
        match self {
            AiApiBody::OpenAI(body) => body.to_json(),
            AiApiBody::DashScope(body) => body.to_json(),
        }
    }

    fn get_messages(&self) -> &LinkedList<Message> {
        match self {
            AiApiBody::OpenAI(body) => body.get_messages(),
            AiApiBody::DashScope(body) => body.get_messages(),
        }
    }

    fn parse_response(&self, response_json: Value) -> Value {
        match self {
            AiApiBody::OpenAI(body) => body.parse_response(response_json),
            AiApiBody::DashScope(body) => body.parse_response(response_json),
        }
    }
}