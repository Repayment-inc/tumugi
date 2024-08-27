use crate::client::{AIClient, ChatMessage, ChatRequest};
use crate::error::AgentError;
use std::sync::Arc;

pub struct SubAgent {
    role: String,
    client: Arc<dyn AIClient>,
    model: String,
}

impl SubAgent {
    pub fn new(role: String, client: Arc<dyn AIClient>, model: String) -> Self {
        Self { role, client, model }
    }

    pub async fn process(&self, goal: &str) -> Result<String, AgentError> {
        let prompt = format!(
            "あなたは{}です。次の目標を達成するために必要なことをあなたの専門知識から、箇条書きで助言してください: {}",
            self.role, goal
        );
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(prompt)],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;

        ////////////////////////////////////////////////
        println!("===========サブエージェントの意見===========\n
        {:?}\n
        ====================================", &chat_res);
        ////////////////////////////////////////////////
                
        Ok(chat_res.choices[0].message.content.clone())
    }
}
