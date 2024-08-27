use super::common::{AIClient, ChatRequest, ChatResponse, Choice, ResMessage, ChatMessage};
use crate::error::TumugiError;
use async_trait::async_trait;
use serde_json::json;

pub struct AnthropicClient {
    api_key: String,
    model: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AIClient for AnthropicClient {
    async fn create_chat_completion(&self, request: ChatRequest) -> Result<ChatResponse, TumugiError> {
        let url = "https://api.anthropic.com/v1/messages";

        let (system_messages, other_messages): (Vec<ChatMessage>, Vec<ChatMessage>) = 
            request.messages.into_iter().partition(|msg| msg.role == "system");

        let system_content = system_messages.first().map(|msg| msg.content.clone()).unwrap_or_default();

        let anthropic_request = json!({
            "model": self.model,
            "messages": other_messages,
            "system": system_content,
            "max_tokens": 1000
        });

        let response = self.client
            .post(url)
            .header("X-API-Key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&anthropic_request)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            let anthropic_response: serde_json::Value = serde_json::from_str(&body)?;

            let chat_response = ChatResponse {
                id: anthropic_response["id"].as_str().unwrap_or("").to_string(),
                choices: vec![Choice {
                    message: ResMessage {
                        role: "assistant".to_string(),
                        content: anthropic_response["content"].as_array()
                            .and_then(|arr| arr.first())
                            .and_then(|obj| obj["text"].as_str())
                            .unwrap_or("")
                            .to_string(),
                    },
                }],
            };

            Ok(chat_response)
        } else {
            Err(TumugiError::ApiError(format!("Anthropic API request failed with status: {}. Error: {}", status, body)))
        }
    }
}