use tumugi::client::{ChatMessage, ChatRequest, Client};
use dotenv::dotenv;

//　現在使用可能なモデル
const MODEL_OPENAI: &str = "gpt-4o-mini";
const MODEL_OPENAI_2: &str = "gpt-4o";
const MODEL_GROQ: &str = "llama3-8b-8192";
const MODEL_GROQ_2: &str = "llama3-70b-8192";
// 対応予定　const MODEL_ANTHROPIC: &str = "claude-3-5-sonnet-20240620";
// 対応予定　const MODEL_ANTHROPIC_2: &str = "claude-3-haiku-20240307";
	

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
	let model =  MODEL_GROQ;
    let client = Client::new(api_key, model.to_string()	);

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
	Ok(())
}