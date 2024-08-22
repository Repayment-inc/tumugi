use crate::client::{ChatMessage, ChatRequest, Client};
use crate::error::AgentError;

pub struct SubAgent {
    role: String,
    client: Client,
}

impl SubAgent {
    pub fn new(role: String, client: Client) -> Self {
        Self { role, client }
    }

    pub async fn process(&self, goal: &str) -> Result<String, AgentError> {
        let prompt = format!(
            "あなたは{}です。次の目標を達成するために必要なことをあなたの専門知識から、箇条書きで助言してください: {}",
            self.role, goal
        );
        let chat_req = ChatRequest::new(
            self.client.model().to_string(),
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
