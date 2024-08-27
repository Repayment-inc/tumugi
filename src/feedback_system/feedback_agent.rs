use crate::client::{AIClient, ChatMessage, ChatRequest};
use crate::error::AgentError;
use std::sync::Arc;

pub struct FeedbackAgent {
    role: String,
    client: Arc<dyn AIClient>,
    model: String,
}

impl FeedbackAgent {
    pub fn new(role: String, client: Arc<dyn AIClient>, model: String) -> Self {
        Self { role, client, model }
    }

    pub async fn process(&self, input: &str, is_checker: bool) -> Result<(String, bool), AgentError> {
        let prompt = if is_checker {
            format!(
                "あなたは{}です。以下の文章を評価し、フィードバックを提供してください。
                改善点がなければ「**OK**」と言い、さらに改善できる点があればそれを指摘してください。
                文章: {}",
                self.role, input
            )
        } else {
            format!(
                "あなたは{}です。以下の指示に従って文章を生成してください: {}",
                self.role, input
            )
        };

        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(prompt)],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;
        let response = chat_res.choices[0].message.content.clone();

        if is_checker {
            let is_ok = response.to_lowercase().contains("**ok**");
            Ok((response, is_ok))
        } else {
            Ok((response, true))
        }
    }
}