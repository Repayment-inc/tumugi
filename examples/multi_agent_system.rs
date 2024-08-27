use tumugi::multi_agent::system::MultiAgentSystem;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("API_KEY must be set");
    let model = "gpt-4o-mini".to_string();
    
    let system = MultiAgentSystem::new(api_key, model)?;
    
    let goal = "会計ソフトを作りたい";
    let max_sub_agents = 3;
    let result = system.process(goal, max_sub_agents).await?;
    
    println!("最終結果: {}", result);
    
    Ok(())
}