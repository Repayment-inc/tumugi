use tumugi::client::{ChatMessage, ChatRequest, ClientFactory};
use dotenv::dotenv;

// 現在使用可能なモデル
const MODEL_OPENAI: &str = "gpt-4o-mini";
const MODEL_OPENAI_2: &str = "gpt-4o";
const MODEL_GROQ: &str = "llama3-8b-8192";
const MODEL_GROQ_2: &str = "llama3-70b-8192";
const MODEL_ANTHROPIC: &str = "claude-3-5-sonnet-20240620";
const MODEL_ANTHROPIC_2: &str = "claude-3-haiku-20240307";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("GROQ_API_KEY must be set");
    let model = MODEL_ANTHROPIC;
    
    let client = ClientFactory::create_client(api_key, model.to_string())?;

    let system = "あなたはAIアシスタントです。常に日本語で答えてください。";
    let question = "2015年、北米で行われた『North American Parkour Championships』という大会で優勝を果たした人物は誰ですか?";
    let answer = "ZENです。";
    let question_2 = "北米のどこで行われましたか?";
    let answer_2 = "カナダ・バンクーバーです。";
    let question_3 = "ZENはどこの国の人ですか?";

    let chat_req = ChatRequest::new(model.to_string(), vec![
        ChatMessage::system(system.to_string()),
        ChatMessage::user(question),
        ChatMessage::assistant(answer),
        ChatMessage::user(question_2),
        ChatMessage::assistant(answer_2),
        ChatMessage::user(question_3),
    ]);

    let chat_res = client.create_chat_completion(chat_req).await?;
    println!("Response: {:?}", chat_res.choices[0].message.content);
    assert!(!chat_res.choices.is_empty());

	Ok(())
}