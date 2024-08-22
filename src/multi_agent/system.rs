use super::commander::CommanderAgent;
use super::sub_agent::SubAgent;
use crate::client::Client;
use crate::error::AgentError;
use tokio::sync::mpsc;
use std::collections::HashMap;

pub struct MultiAgentSystem {
    commander: CommanderAgent,
    client: Client,
}

impl MultiAgentSystem {
    pub fn new(client: Client) -> Self {
        Self {
            commander: CommanderAgent::new(client.clone()),
            client,
        }
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
            let role = role.clone();
            tokio::spawn(async move {
                let agent = SubAgent::new(role.clone(), client);
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