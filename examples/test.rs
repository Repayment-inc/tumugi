use dotenv::dotenv;
use tokio;

// 既存のコードをインポートするか、同じファイルに含めると仮定します
use tumugi::partner::ChatOpenAIBuilder;
use tumugi::partner::openai::{ResponseFormat, Role};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .envファイルから環境変数を読み込む
    dotenv().ok();

    // ChatOpenAIインスタンスを構築
    let chat_openai = ChatOpenAIBuilder::new("gpt-4o-mini".to_string())
         .add_message(Role::User, "あなたは優秀なエンジニアです.".to_string())
        // .add_system_message("rustの勉強プロセスを教えてください".to_string(), None)
        // .response_format(ResponseFormat::Text)
        // .temperature(0.7)
        // .max_completion_tokens(100)
        // .stream(true)
        .build()?;


    // リクエストを送信
    match chat_openai.send_request().await {
        Ok(response) => {
            println!("Response ID: {}", response.id);
            println!("Model used: {}", response.model);
            for choice in response.choices {
                println!("AI: {}", choice.message.content.unwrap_or_default());
            }
            println!("Usage: {} prompt tokens, {} completion tokens", 
                     response.usage.prompt_tokens, 
                     response.usage.completion_tokens);
        },
        Err(e) => {
            eprintln!("Error occurred: {:?}", e);
        }
    }

    Ok(())
}