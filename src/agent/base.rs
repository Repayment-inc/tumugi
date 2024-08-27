// use crate::client::{ChatMessage, ChatRequest, Client};
// use crate::error::AgentError;
// use super::task_executor::TaskExecutor;
// use std::collections::HashMap;

// /// エージェントの状態を表す列挙型
// #[derive(Debug, Clone)]
// pub enum AgentState {
//     Idle,        // アイドル状態
//     Processing,  // 処理中
//     AwaitingInput, // 入力待ち
// }

// /// AIエージェントを表す構造体
// pub struct Agent {
//     name: String,
//     role: String,
//     client: Client,
//     memory: Vec<ChatMessage>,
//     state: AgentState,
//     task_executors: HashMap<String, Box<dyn TaskExecutor>>,
// }

// impl Agent {
//     /// 新しいエージェントを作成する
//     pub fn new(name: String, role: String, client: Client) -> Self {
//         Self {
//             name,
//             role,
//             client,
//             memory: Vec::new(),
//             state: AgentState::Idle,
//             task_executors: HashMap::new(),
//         }
//     }

//     /// ユーザーの入力を処理し、応答を生成する
//     pub async fn process_input(&mut self, input: &str) -> Result<String, AgentError> {
//         self.state = AgentState::Processing;
//         self.memory.push(ChatMessage::user(input.to_string()));

//         // システムメッセージを作成
//         let system_message = ChatMessage::system(format!("You are {}, acting as {}. Respond accordingly.", self.name, self.role));
//         let mut messages = vec![system_message];
//         messages.extend(self.memory.clone());

//         // チャットリクエストを作成し、AI模型に送信
//         let chat_req = ChatRequest::new(self.client.model().to_string(), messages);
//         let chat_res = self.client.create_chat_completion(chat_req).await?;

//         let response = chat_res.choices[0].message.content.clone();
//         self.memory.push(ChatMessage::assistant(response.clone()));

//         self.state = AgentState::Idle;
//         Ok(response)
//     }

//     /// 指定されたタスクを実行する
//     pub async fn execute_task(&mut self, task: &str) -> Result<String, AgentError> {
//         self.state = AgentState::Processing;
//         let prompt = format!("Execute the following task: {}", task);
//         println!("{:?}", prompt);
//         let result = self.process_input(&prompt).await?;
//         self.state = AgentState::Idle;
//         Ok(result)
//     }

//     /// 現在のエージェントの状態を取得する
//     pub fn get_state(&self) -> AgentState {
//         self.state.clone()
//     }

//     /// エージェントのメモリをクリアする
//     pub fn clear_memory(&mut self) {
//         self.memory.clear();
//     }

//     /// 新しいタスク実行機能を追加する
//     pub fn add_task_executor(&mut self, name: String, executor: Box<dyn TaskExecutor>) {
//         self.task_executors.insert(name, executor);
//     }

//     /// 特定のタスクを実行する
//     pub async fn execute_specific_task(&mut self, task_name: &str, input: &str) -> Result<String, AgentError> {
//         if let Some(executor) = self.task_executors.get(task_name) {
//             executor.execute(input).map_err(|e| AgentError::TaskExecutionError(e.to_string()))
//         } else {
//             Err(AgentError::TaskNotFound(task_name.to_string()))
//         }
//     }
// }