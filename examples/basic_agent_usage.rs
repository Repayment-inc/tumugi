// use tumugi::agent::{Agent, Calculator};
// use tumugi::client::Client;
// use tumugi::error::AgentError;
// use dotenv::dotenv;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();
//     // APIキーを環境変数から取得（実際の使用時はdotenvなどを使用することをお勧めします）
//     let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    
//     // クライアントとエージェントの作成
//     let client = Client::new(api_key, "llama3-8b-8192".to_string());
//     let mut agent = Agent::new("助手".to_string(), "親切なAIアシスタント".to_string(), client);

//     // 基本的な対話
//     println!("基本的な対話:");
//     let response = agent.process_input("こんにちは。今日の天気はどうですか？").await?;
//     println!("エージェント: {}", response);

//     // タスクの実行
//     println!("\nタスクの実行:");
//     let task_result = agent.execute_task("日本の首都について3つの興味深い事実を教えてください。").await?;
//     println!("タスク結果: {}", task_result);

//     // 計算機能の追加と使用
//     println!("\n計算機能の使用:");
//     let calculator = Calculator;
//     agent.add_task_executor("calculator".to_string(), Box::new(calculator));
    
//     match agent.execute_specific_task("calculator", "15 + 7").await {
//         Ok(result) => println!("計算結果: {}", result),
//         Err(AgentError::TaskExecutionError(e)) => println!("計算エラー: {}", e),
//         Err(e) => println!("その他のエラー: {:?}", e),
//     }

//     // エラーハンドリングの例
//     println!("\nエラーハンドリングの例:");
//     match agent.execute_specific_task("non_existent_task", "some input").await {
//         Ok(_) => println!("タスクが実行されました（ありえない）"),
//         Err(AgentError::TaskNotFound(task)) => println!("タスクが見つかりません: {}", task),
//         Err(e) => println!("その他のエラー: {:?}", e),
//     }

//     Ok(())
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ハローワールド");
    Ok(())
}
