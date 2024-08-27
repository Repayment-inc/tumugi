use super::{AIClient, OpenAIClient, AnthropicClient, GroqClient};
use crate::error::TumugiError;
use std::sync::Arc;

pub struct ClientFactory;

impl ClientFactory {
    pub fn create_client(api_key: String, model: String) -> Result<Arc<dyn AIClient>, TumugiError> {
        match model.as_str() {
            model if model.starts_with("gpt-4o") => Ok(Arc::new(OpenAIClient::new(api_key, model.to_string()))),
            model if model.starts_with("claude-3") => Ok(Arc::new(AnthropicClient::new(api_key, model.to_string()))),
            model if model.starts_with("llama3") => Ok(Arc::new(GroqClient::new(api_key, model.to_string()))),
            _ => Err(TumugiError::UnsupportedModel(model)),
        }
    }
}