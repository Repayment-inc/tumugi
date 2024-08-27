use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use crate::error::TumugiError;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn create_chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, TumugiError>;
}

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResMessage {
    pub role: String,
    pub content: String,
}

impl ChatRequest {
    pub fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self { model, messages }
    }
}

impl ChatMessage {
    pub fn system(content: String) -> Self {
        Self { role: "system".to_string(), content }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".to_string(), content: content.into() }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".to_string(), content: content.into() }
    }
}