use tumugi::client::{ChatMessage, ChatRequest, Client};
use dotenv::dotenv;

#[tokio::test]
async fn test_chat_completion_api() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    let model = "llama3-8b-8192";
    let client = Client::new(api_key, model.to_string());

    let question = "猫の猫種は何種類?";

    let chat_req = ChatRequest::new(model.to_string(), vec![
        ChatMessage {
            role: "system".to_string(),
            content: "あなたはAIアシスタントです。常に日本語で答えてください。".to_string(),
        },
        ChatMessage {
            role: "user".to_string(),
            content: question.to_string(),
        },
    ]);

    let chat_res = client.create_chat_completion(chat_req).await.unwrap();
    println!("Response: {:?}", chat_res.choices[0].message.content);

    assert!(!chat_res.choices.is_empty());
    // 猫の猫種は何種類?に対する回答が含まれていることを確認
    assert!(chat_res.choices[0].message.content.contains("猫"));

    Ok(())
}
