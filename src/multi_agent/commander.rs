use crate::client::{AIClient, ChatMessage, ChatRequest};
use crate::error::AgentError;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CommanderAgent {
    client: Arc<dyn AIClient>,
    model: String,
}

impl CommanderAgent {
    pub fn new(client: Arc<dyn AIClient>, model: String) -> Self {
        Self { client, model }
    }

    pub async fn determine_roles(&self, goal: &str, max_roles: usize) -> Result<Vec<String>, AgentError> {
        let prompt = format!(
            "目標: {}。この目標を達成するために必要な役割を、最大{}個まで挙げてください。回答は以下の形式に厳密に従ってください：\n\nROLES:\n- [役割1]\n- [役割2]\n- [役割3]\n\n他の説明は不要です。",
            goal, max_roles
        );
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(prompt)],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;
                ////////////////////////////////////////////////
                println!("===========役割res===========\n
                {:?}\n
                ====================================", &chat_res);
                ////////////////////////////////////////////////
        let response = chat_res.choices[0].message.content.clone();
        
        // let re = Regex::new(r"ROLES:\s*(?:\n*-\s*(.+))+").unwrap();
        // let roles: Vec<String> = re.captures_iter(&response)
        //     .filter_map(|cap| cap.get(1))
        //     .map(|m| m.as_str().trim().to_string())
        //     .take(max_roles)
        //     .collect();

        let roles: Vec<String> = response
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(|line| line.trim_start_matches("- ").trim().to_string())
        .take(max_roles)
        .collect();

        ////////////////////////////////////////////////
        println!("===========役割決定===========\n
        {:?}\n
        ====================================", &roles);
        ////////////////////////////////////////////////
        if roles.is_empty() {
            Err(AgentError::Other("No valid roles found in the response".to_string()))
        } else {
            Ok(roles)
        }
    }

    pub async fn summarize_results(&self, goal: &str, results: &HashMap<String, String>) -> Result<String, AgentError> {
        let summary = results
            .iter()
            .map(|(role, result)| format!("{}の意見:\n{}", role, result))
            .collect::<Vec<_>>()
            .join("\n\n");

            let prompt = format!(
                "目標: {}
            
            以下の各専門家の意見を踏まえて、次の項目を作成してください。各専門家の意見は可能な限り保持し、詳細を維持してください：

            1. 統合（INTEGRATION）: 各専門家の意見を統合し、内容のチェックを行ってください。

            2. 計画（PLAN）: 目標達成のための包括的かつ詳細な計画を、段階的に箇条書きで示してください。各専門家の意見を反映させ、必要に応じて計画を拡張または修正してください。

            3. チーム構成（TEAMS）: 必要なチームを列挙し、各チームの役割を詳細に説明してください。専門家の意見に基づいて、必要に応じて新しいチームを追加してください。

            4. タスク割り当て（TASK ASSIGNMENTS）: 計画を実行するための具体的なタスクを、担当チームとともに順番に示してください。以下の点に注意してください：
            - 各専門家の意見から抽出したタスクを含める
            - タスクの粒度を適切に調整し、必要に応じて分割または統合する
            - タスクの順序を論理的に並べ替える
            - 各タスクに最適なチームを割り当てる
            - 重複するタスクを統合し、不足しているタスクを追加する

            回答は以下の形式に従ってください：
            
            SUMMARY:
            [専門家の意見を統合した包括的な要約]

            PLAN:
            1. [計画項目1]
            2. [計画項目2]
            3. [計画項目3]
            ...

            TEAMS:
            - [チーム1]: [詳細な役割の説明]
            - [チーム2]: [詳細な役割の説明]
            - [チーム3]: [詳細な役割の説明]
            ...

            TASK ASSIGNMENTS:
            1. [具体的なタスク1] - 担当: [チーム名]
            2. [具体的なタスク2] - 担当: [チーム名]
            3. [具体的なタスク3] - 担当: [チーム名]
            ...

            専門家の意見:
            {}",
                goal, summary
            );
        let chat_req = ChatRequest::new(
            self.model.clone(),
            vec![ChatMessage::user(prompt)],
        );
        let chat_res = self.client.create_chat_completion(chat_req).await?;

        ////////////////////////////////////////////////
        println!("===========要約した内容===========\n 
        {:?}\n
        ====================================", &chat_res);
        ////////////////////////////////////////////////

        Ok(chat_res.choices[0].message.content.clone())
    }
}