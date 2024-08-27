use tumugi::client::{ChatMessage, ChatRequest, ClientFactory};
use tumugi::memory::ConversationBufferWindowMemory;
use dotenv::dotenv;

const MODEL_GROQ: &str = "llama3-8b-8192";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    let model = MODEL_GROQ.to_string();

    let client = ClientFactory::create_client(api_key, model.to_string())?;
    
    // ConversationBufferWindowMemoryの初期化
    let mut memory = ConversationBufferWindowMemory::new(8);

    // システムメッセージを設定
    memory.set_system_message(ChatMessage::system("常に日本語で回答して".to_string()));

    // ユーザーとの対話をシミュレート
    let conversations = vec![
        "今からいう文章を覚えてくれる?",
        "わたしはりんごが嫌いです", // これより前は窓外になる
        "みかんは好きです",
        "私は何が好きですか?",
        "最初の質問は何でしたか?",
        "私は何が嫌いですか?",
    ];

    for (i, user_input) in conversations.iter().enumerate() {
        println!("\nConversation turn {}", i + 1);
        
        // ユーザーメッセージを追加
        memory.add_message(ChatMessage::user(user_input.to_string()));

        // APIリクエストの作成
        let chat_req = ChatRequest::new(MODEL_GROQ.to_string(), memory.get_messages());

        // APIリクエストの送信
        let chat_res = client.create_chat_completion(chat_req).await?;
        
        // レスポンスの表示
        println!("User: {}", user_input);
        println!("Assistant: {}", chat_res.choices[0].message.content);

        // アシスタントの返答をメモリに追加
        memory.add_message(ChatMessage::assistant(chat_res.choices[0].message.content.clone()));

        // 現在のメモリの状態を表示
        println!("\nCurrent memory state:");
        for (j, msg) in memory.get_messages().iter().enumerate() {
            println!("  [{}] {}: {}", j, msg.role, msg.content);
        }
    }

    Ok(())
}