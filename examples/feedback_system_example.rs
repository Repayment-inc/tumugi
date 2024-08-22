// examples/feedback_system_example.rs

use tumugi::client::Client;
use tumugi::feedback_system::FeedbackSystem;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let generator_api_key = std::env::var("OPENAI_API_KEY").expect("AI_API_KEY must be set");
    let checker_api_key = std::env::var("OPENAI_API_KEY").expect("AI_API_KEY must be set");

    let generator_client = Client::new(generator_api_key, "gpt-4o-mini".to_string());
    let checker_client = Client::new(checker_api_key, "gpt-4o".to_string());

    let feedback_system = FeedbackSystem::new(generator_client, checker_client, 3);

    let input = "AIエージェントの種類と活用方法について教えて";
    let result = feedback_system.process(input).await?;

    println!("最終結果:\n{}", result);

    Ok(())
}