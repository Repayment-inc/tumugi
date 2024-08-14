use tumugi::{ChatClient, ChatCompletionRequest, Message};
use dotenv::dotenv;

/// チャット完了をテストする非同期関数
#[tokio::test]
async fn test_chat_completion() {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = ChatClient::new(api_key);

    let request = ChatCompletionRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "What's the weather like today?".to_string(),
            },
        ],
        temperature: Some(0.7),
        max_tokens: Some(150),
    };

    let response = client.create_chat_completion(request).await.unwrap();
    println!("Response: {:?}", response.choices[0].message.content);

    assert!(!response.choices.is_empty());
}