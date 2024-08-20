use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use dotenv::dotenv;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use tumugi::client::{ChatMessage, ChatRequest, Client};
use tumugi::memory::ConversationBufferWindowMemory;


const MODEL_GROQ: &str = "llama3-8b-8192";


// ユーザーIDとメモリのマッピングを管理する構造体
struct AppState {
    memories: Mutex<HashMap<String, ConversationBufferWindowMemory>>,
    client: Client,
}

#[derive(Deserialize)]
struct ChatInput {
    user_id: String,
    message: String,
}

#[derive(Serialize)]
struct ChatOutput {
    response: String,
}

async fn chat(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ChatInput>,
) -> Json<ChatOutput> {
    let mut memories = state.memories.lock().await;
    let memory = memories.entry(input.user_id.clone())
        .or_insert_with(|| ConversationBufferWindowMemory::new(2));

    // システムメッセージを設定
    memory.set_system_message(ChatMessage::system("あなたの名前はしまじろうです。常に日本語で回答して".to_string()));
    memory.add_message(ChatMessage::user(input.message));

    let chat_req = ChatRequest::new(
        MODEL_GROQ.to_string(),
        memory.get_messages(),
    );

    let chat_res = state.client.create_chat_completion(chat_req).await.unwrap();
    let assistant_message = chat_res.choices[0].message.content.clone();

    memory.add_message(ChatMessage::assistant(assistant_message.clone()));

    // 永続化の処理を入れたい場合
    // 例: save_memories_to_database(&memories).await;

    Json(ChatOutput {
        response: assistant_message,
    })
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("GROQ_API_KEY").expect("API_KEY must be set");
    let client = Client::new(api_key, MODEL_GROQ.to_string());

    let app_state = Arc::new(AppState {
        memories: Mutex::new(HashMap::new()),
        client,
    });

    let app = Router::new()
        .route("/chat", post(chat))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}