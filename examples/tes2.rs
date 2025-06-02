use dotenv::dotenv;
use tokio;
use std::any::type_name;

// 既存のコードをインポートするか、同じファイルに含めると仮定します
use tumugi::partner::ChatOpenAIBuilder;
use tumugi::partner::openai::{ResponseFormat, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .envファイルから環境変数を読み込む
    dotenv().ok();

    let model = ChatOpenAIBuilder::new("gpt-4o-mini".to_string())
        .add_message(Role::User, "あなたは優秀なエンジニアです.".to_string())
        .build();

    println!("model type: {}", std::any::type_name::<typeof(&model)>());

    Ok(())
}