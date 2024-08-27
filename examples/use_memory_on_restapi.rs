use axum::{
    extract::{State, Json},
    routing::post,
    Router,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use dotenv::dotenv;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use tumugi::client::{ChatMessage, ChatRequest, ClientFactory, AIClient};
use tumugi::memory::ConversationBufferWindowMemory;
use tumugi::error::TumugiError;

const MODEL_OPENAI: &str = "gpt-4o-mini";
const MODEL_GROQ: &str = "llama3-8b-8192";

// ユーザーIDとメモリのマッピングを管理する構造体
struct AppState {
    memories: Mutex<HashMap<String, ConversationBufferWindowMemory>>,
    client: Arc<dyn AIClient>,
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

// TumugiErrorをラップする新しい型
struct AppError(TumugiError);

// AppErrorに対してIntoResponseを実装
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0 {
            TumugiError::RequestFailed(_) => (StatusCode::BAD_REQUEST, "Request failed"),
            TumugiError::ParseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Parse error"),
            TumugiError::ProviderError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Provider error"),
            TumugiError::UnsupportedModel(_) => (StatusCode::BAD_REQUEST, "Unsupported model"),
            TumugiError::ApiError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "API error"),
        };
        (status, error_message).into_response()
    }
}

// TumugiErrorからAppErrorへの変換を実装
impl From<TumugiError> for AppError {
    fn from(error: TumugiError) -> Self {
        AppError(error)
    }
}

async fn chat(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ChatInput>,
) -> Result<Json<ChatOutput>, AppError> {
    let mut memories = state.memories.lock().await;
    let memory = memories.entry(input.user_id.clone())
        .or_insert_with(|| ConversationBufferWindowMemory::new(5));

    // システムメッセージを設定
    memory.set_system_message(ChatMessage::system("あなたの名前はしまじろうです。常に日本語で回答して".to_string()));
    memory.add_message(ChatMessage::user(input.message));

    let chat_req = ChatRequest::new(
        MODEL_OPENAI.to_string(),
        memory.get_messages(),
    );

    let chat_res = state.client.create_chat_completion(chat_req).await?;
    let assistant_message = chat_res.choices[0].message.content.clone();

    memory.add_message(ChatMessage::assistant(assistant_message.clone()));

    // 永続化の処理を入れたい場合
    // 例: save_memories_to_database(&memories).await;

    Ok(Json(ChatOutput {
        response: assistant_message,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = std::env::var("OPENAI_API_KEY").expect("API_KEY must be set");
    let client = ClientFactory::create_client(api_key, MODEL_OPENAI.to_string())?;

    let app_state = Arc::new(AppState {
        memories: Mutex::new(HashMap::new()),
        client,
    });

    let app = Router::new()
        .route("/chat", post(chat))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}