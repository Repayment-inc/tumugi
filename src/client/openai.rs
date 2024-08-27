use super::common::{AIClient, ChatRequest, ChatResponse};
use crate::error::TumugiError;
use async_trait::async_trait;

pub struct OpenAIClient {
    api_key: String,
    /// The model to be used for this client instance.
    /// Currently not used in requests, but may be utilized in future implementations.
    model: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIClient for OpenAIClient {
    async fn create_chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, TumugiError> {
        let url = "https://api.openai.com/v1/chat/completions";

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_response = response.json().await?;
            Ok(chat_response)
        } else {
            Err(TumugiError::ApiError(format!("OpenAI API request failed with status: {}", response.status())))
        }
    }
}