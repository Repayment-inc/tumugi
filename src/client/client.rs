use serde::{Serialize, Deserialize};
pub use thiserror::Error;
use serde_json::json;
use std::sync::Arc;

/// チャット完了リクエスト 最小構成
#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>
}

impl ChatRequest {
    pub fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model,
            messages: messages,
        }
    }
}

/// チャットエラーを表す列挙型
#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Chat API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Chat response parsing failed: {0}")]
    ParseError(String),
}

// region:    --- ChatMessage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
	// pub role: ChatRole,
    pub role: String,
	pub content: String,
}

/// Constructors
impl ChatMessage { 
	pub fn system(content: String) -> Self {
		Self {
            role: "system".to_string(),
			content: content.into(),
		}
	}

	pub fn assistant(content: impl Into<String>) -> Self {
		Self {
            role: "assistant".to_string(),
			content: content.into(),
		}
	}

	pub fn user(content: impl Into<String>) -> Self {
		Self {
            role: "user".to_string(),
			content: content.into(),
		}
	}
}
// endregion: --- ChatMessage

#[derive(Clone)]
pub struct Client {
    client: Arc<reqwest::Client>,
    // client: reqwest::Client // groqの際はArcを外さないとできないっぽい
    model: String,
    api_key: String,
    base_url: String,
}


/// チャット完了レスポンスの構造体
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub choices: Vec<Choice>,
}

/// チャットの選択肢を表す構造体
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResMessage {
	pub role: String,
	pub content: String,
}


impl Client {
    pub fn new(api_key: String, model: String) -> Self {
        let base_url = match model.as_str() {
            model if model.starts_with("gpt-4o") => "https://api.openai.com/v1/chat/completions",
            model if model.starts_with("claude-3") => "https://api.anthropic.com/v1/messages",
            model if model.starts_with("llama3") => "https://api.groq.com/openai/v1/chat/completions",
            _ => "https://api.openai.com/v1/chat/completions", // デフォルトはOpenAI
        };
        Self {
            client: Arc::new(reqwest::Client::new()), 
            // client: reqwest::Client::new() // groqの際はArcを外さないとできないっぽい
            model,
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    /// チャット完了を作成する非同期メソッド
    pub async fn create_chat_completion(
        &self,
        request: ChatRequest,
    ) -> Result<ChatResponse, ChatError> {
        let url = format!("{}", self.base_url);

        // リクエスト内容をデバッグ出力 //
        let request_json = serde_json::to_string_pretty(&request).unwrap();
        println!("URL: {}", url);
        // println!("Authorization: Bearer {}", self.api_key);
        println!("Request JSON: {}", request_json);
        /////////////////////////

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let chat_response = response.json().await?;

            // リクエスト内容をデバッグ出力 //
            println!("Response JSON: {:?}", &chat_response);
            /////////////////////////
            
            Ok(chat_response)
        } else {
            Err(ChatError::ParseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
}