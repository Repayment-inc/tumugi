use super::self_refine_agent::SelfRefineAgent;
use crate::error::{AgentError, TumugiError};

pub struct SelfRefineSystem {
    agent: SelfRefineAgent,
    iterations: usize,
}

impl SelfRefineSystem {
    pub fn new(api_key: String, model: String, iterations: usize, memory_size: usize) -> Result<Self, TumugiError> {
        let agent = SelfRefineAgent::new("自己改善AI".to_string(), api_key, model, memory_size)?;
        Ok(Self {
            agent,
            iterations,
        })
    }

    pub async fn process(&mut self, prompt: &str) -> Result<String, AgentError> {
        let mut current_text = self.agent.generate(prompt).await?;
        println!("初期生成文: {}", current_text);
        
        for iteration in 0..self.iterations {
            println!("反復 {}/{}", iteration + 1, self.iterations);
            
            let feedback = self.agent.evaluate(&current_text).await?;
            println!("フィードバック: {}", feedback);
            
            current_text = self.agent.refine(&current_text, &feedback).await?;
            println!("改善された文章: {}", current_text);
        }
        
        Ok(current_text)
    }

    pub fn get_conversation_history(&self) -> Vec<String> {
        self.agent.get_conversation_history()
            .into_iter()
            .map(|msg| format!("{}: {}", msg.role, msg.content))
            .collect()
    }
}