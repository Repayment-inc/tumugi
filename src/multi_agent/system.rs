use super::commander::CommanderAgent;
use super::sub_agent::SubAgent;
use crate::client::{AIClient, ClientFactory};
use crate::error::{AgentError, TumugiError};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::Arc;

pub struct MultiAgentSystem {
    commander: CommanderAgent,
    client: Arc<dyn AIClient>,
    model: String,
}

impl MultiAgentSystem {
    pub fn new(api_key: String, model: String) -> Result<Self, TumugiError> {
        let client = ClientFactory::create_client(api_key.clone(), model.clone())?;
        Ok(Self {
            commander: CommanderAgent::new(client.clone(), model.clone()),
            client,
            model,
        })
    }

    pub async fn process(&self, goal: &str, max_roles: usize) -> Result<String, AgentError> {
        // 1. 必要な役割を決定
        let roles = self.commander.determine_roles(goal, max_roles).await?;

        // 2. サブエージェントを作成
        let (tx, mut rx) = mpsc::channel(roles.len());
        for role in &roles {
            let tx = tx.clone();
            let goal = goal.to_string();
            let client = self.client.clone();
            let model = self.model.clone();
            let role = role.clone();
            tokio::spawn(async move {
                let agent = SubAgent::new(role.clone(), client, model.to_string());
                let result = agent.process(&goal).await;
                tx.send((role, result)).await.unwrap();
            });
        }

        // 3. 結果を集約
        let mut results = HashMap::new();
        for _ in 0..roles.len() {
            let (role, result) = rx.recv().await.ok_or(AgentError::CommunicationError)?;
            results.insert(role, result?);
        }

        // 4. 最終的な結果をまとめる
        self.commander.summarize_results(goal, &results).await
    }
}