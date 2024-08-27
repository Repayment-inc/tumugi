use tumugi::self_refine::SelfRefineSystem;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let model = "gpt-4o-mini".to_string();
    let iterations = 2;
    let memory_size = 10;
    

    let mut self_refine_system = SelfRefineSystem::new(api_key, model, iterations, memory_size)?; // 3回の反復、メモリサイズ10

    let prompt = "AIの倫理的な使用について、500字程度の文章を書いてください。";
    let result = self_refine_system.process(prompt).await?;

    println!("\n最終結果:\n{}", result);

    println!("\n会話履歴:");
    for (i, message) in self_refine_system.get_conversation_history().iter().enumerate() {
        println!("{}. {}", i + 1, message);
    }

    Ok(())
}