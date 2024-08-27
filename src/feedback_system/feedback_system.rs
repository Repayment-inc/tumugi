use super::feedback_agent::FeedbackAgent;
use crate::client::ClientFactory;
use crate::error::{AgentError, TumugiError};

pub struct FeedbackSystem {
    generator: FeedbackAgent,
    checker: FeedbackAgent,
    max_iterations: usize,
}

impl FeedbackSystem {
    pub fn new(generator_api_key: String, generator_model: String, 
               checker_api_key: String, checker_model: String, 
               max_iterations: usize) -> Result<Self, TumugiError> {
        let generator_client = ClientFactory::create_client(generator_api_key, generator_model.clone())?;
        let checker_client = ClientFactory::create_client(checker_api_key, checker_model.clone())?;
        
        Ok(Self {
            generator: FeedbackAgent::new("生成用AI".to_string(), generator_client, generator_model),
            checker: FeedbackAgent::new("チェッカー".to_string(), checker_client, checker_model),
            max_iterations,
        })
    }

    pub async fn process(&self, input: &str) -> Result<String, AgentError> {
        let mut current_input = input.to_string();
        let mut iteration = 0;
        let mut conversation_history = Vec::new();

        while iteration < self.max_iterations {
            let (generated_text, _) = self.generator.process(&current_input, false).await?;
            conversation_history.push(format!("生成: {}", generated_text));

            let (feedback, is_ok) = self.checker.process(&generated_text, true).await?;
            conversation_history.push(format!("フィードバック: {}", feedback));

            if is_ok {
                return Ok(generated_text);
            }

            current_input = format!(
                "以下の会話履歴を踏まえて、文章を改善してください：\n{}\n\n元の指示: {}",
                conversation_history.join("\n"),
                input
            );
            iteration += 1;
        }

        Err(AgentError::Other("最大反復回数に達しました。".to_string()))
    }
}