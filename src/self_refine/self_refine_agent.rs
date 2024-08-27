use crate::client::{AIClient, ClientFactory, ChatMessage, ChatRequest};
use crate::error::{AgentError, TumugiError};
use crate::memory::ConversationBufferWindowMemory;
use std::sync::Arc;

pub struct SelfRefineAgent {
    role: String,
    client: Arc<dyn AIClient>,
    memory: ConversationBufferWindowMemory,
    model: String,
}

impl SelfRefineAgent {
    pub fn new(role: String, api_key: String, model: String, memory_size: usize) -> Result<Self, TumugiError> {
        let client = ClientFactory::create_client(api_key, model.clone())?;
        Ok(Self {
            role,
            client,
            model,
            memory: ConversationBufferWindowMemory::new(memory_size),
        })
    }

    pub async fn generate(&mut self, prompt: &str) -> Result<String, AgentError> {
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![
                ChatMessage::system(format!("あなたは{}です。与えられた指示に従って文章を生成してください。", self.role)),
                ChatMessage::user(prompt.to_string()),
            ],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;
        let response = chat_res.choices[0].message.content.clone();
        
        // メモリに追加
        self.memory.add_message(ChatMessage::user(prompt.to_string()));
        self.memory.add_message(ChatMessage::assistant(response.clone()));
        
        Ok(response)
    }

    pub async fn evaluate(&mut self, text: &str) -> Result<String, AgentError> {
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![
                ChatMessage::system("あなたは文章評価者です。与えられた文章を評価し、改善点を具体的に指摘してください。".to_string()),
                ChatMessage::user(text.to_string()),
            ],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;
        let response = chat_res.choices[0].message.content.clone();
        
        // メモリに追加
        self.memory.add_message(ChatMessage::user("文章を評価してください。".to_string()));
        self.memory.add_message(ChatMessage::assistant(response.clone()));
        
        Ok(response)
    }

    pub async fn refine(&mut self, original_text: &str, feedback: &str) -> Result<String, AgentError> {
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![
                ChatMessage::system(format!("あなたは{}です。与えられた文章を改善してください。", self.role)),
                ChatMessage::user(format!("元の文章: {}\n\nフィードバック: {}\n\n改善された文章を生成してください。", original_text, feedback)),
            ],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;
        let response = chat_res.choices[0].message.content.clone();
        
        // メモリに追加
        self.memory.add_message(ChatMessage::user("文章を改善してください。".to_string()));
        self.memory.add_message(ChatMessage::assistant(response.clone()));
        
        Ok(response)
    }

    pub fn get_conversation_history(&self) -> Vec<ChatMessage> {
        self.memory.get_messages()
    }
}