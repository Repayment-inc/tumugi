use tumugi::client::{ChatMessage, ChatRequest, Client};
use tumugi::memory::ConversationBufferWindowMemory;
use dotenv::dotenv;

const MODEL_GROQ: &str = "llama3-8b-8192";

#[tokio::test]
async fn test_conversation_buffer_window_memory() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    let client = Client::new(api_key, MODEL_GROQ.to_string());
    
    // ConversationBufferWindowMemoryの初期化（ウィンドウサイズ4）
    let mut memory = ConversationBufferWindowMemory::new(8);

    // システムメッセージを追加
    memory.add_message(ChatMessage::system("あなたの名前はジェニファーです。常に日本語で回答して".to_string()));

    // ユーザーとの対話をシミュレート
    let conversations = vec![
        "はじめまして",
        "あなたの名前はなんですか?",
        "私はみかんが好きです",
        "私は何が好きですか?",
        "私は最初になんと言いましたか?",
    ];

    for user_input in conversations.iter() {
        // ユーザーメッセージを追加
        memory.add_message(ChatMessage::user(user_input.to_string()));

        // APIリクエストの作成
        let chat_req = ChatRequest::new(MODEL_GROQ.to_string(), memory.get_messages());

        // APIリクエストの送信
        let chat_res = client.create_chat_completion(chat_req).await?;
        
        // アシスタントの返答をメモリに追加
        memory.add_message(ChatMessage::assistant(chat_res.choices[0].message.content.clone()));

        // レスポンスが空でないことを確認
        assert!(!chat_res.choices.is_empty());
        assert!(!chat_res.choices[0].message.content.is_empty());
    }


    // 現在のメモリの状態を表示
    println!("\nCurrent memory state:");
    for (i, msg) in memory.get_messages().iter().enumerate() {
        println!("  [{}] {}: {}", i, msg.role, msg.content);
    }
    println!("--------------------------------------------------");

    // 4番目のアシスタントの返答が期待される内容を含むことを確認
    let res = memory.get_messages().get(4).unwrap().clone();
    println!("Response: {:?}", res.content);
    assert!(res.role == "assistant");
    assert!(res.content.contains("みかん"));

    
    // 最後のアシスタントの返答が期待される内容を含むことを確認
    let last_response = memory.get_messages().last().unwrap().clone();
    println!("Response: {:?}", last_response.content);
    assert!(last_response.role == "assistant");
    assert!(!last_response.content.contains("はじめまして"));

    Ok(())
}
