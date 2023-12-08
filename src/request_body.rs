use std::collections::LinkedList;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenAiRequestBody {
    pub model: String,
    pub messages: LinkedList<Message>,
    pub max_tokens: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub(crate) role: String,
    pub(crate) content: String
}