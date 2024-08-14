pub use reqwest;
pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;
pub use dotenv::dotenv;

/// メッセージの構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// チャット完了リクエストの構造体
#[derive(Debug, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// チャット完了レスポンスの構造体
#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

/// チャットの選択肢を表す構造体
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

/// チャットエラーを表す列挙型
#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Chat API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Chat response parsing failed: {0}")]
    ParseError(String),
}

/// チャットクライアントの構造体
pub struct ChatClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl ChatClient {
    /// 新しいチャットクライアントを作成する
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    /// チャット完了を作成する非同期メソッド
    pub async fn create_chat_completion(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse, ChatError> {
        let url = format!("{}/chat/completions", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_response = response.json().await?;
            Ok(chat_response)
        } else {
            Err(ChatError::ParseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
}