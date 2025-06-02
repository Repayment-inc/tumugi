use super::models::{Message, ResponseFormat, Tool, ToolChoice};
use super::error::ChatOpenAIError;
use super::api::ChatCompletionResponse;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use reqwest::Client;
use dotenv::dotenv;
use std::env;

/// Represents a configured ChatOpenAI instance
#[derive(Debug, Clone, Serialize)]
pub struct ChatOpenAI {
    pub messages: Vec<Message>,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<i32, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<HashMap<String, bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    // 下記非推奨パラメータ (2023年9月16日時点)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<serde_json::Value>>,
}



impl ChatOpenAI {
    /// Sends a request to the OpenAI API and returns the response
    pub async fn send_request(&self) -> Result<ChatCompletionResponse, ChatOpenAIError> {
        dotenv().ok();
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let client = Client::new();
        let url = "https://api.openai.com/v1/chat/completions";

        println!("============chat_openai: {:?}", self);

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&self)
            .send()
            .await; // ここで Result<Response, Error> を受け取る

        // ここで Result をマッチしてエラーハンドリングを行う
        let response = match response {
            Ok(res) => res,
            Err(err) => return Err(ChatOpenAIError::ApiError(err.to_string())),
        };

        // ここで response.status() を呼び出す
        if response.status().is_success() {
            let chat_response = response
                .json::<ChatCompletionResponse>()
                .await
                .map_err(|err| ChatOpenAIError::ApiError(err.to_string()))?; // エラーハンドリングを追加
            Ok(chat_response)
        } else {
            let error_message = response
                .text()
                .await
                .map_err(|err| ChatOpenAIError::ApiError(err.to_string()))?; // エラーハンドリングを追加
            Err(ChatOpenAIError::ApiError(error_message))
        }
    }
}