use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("AI API request failed: {0}")]
    RequestFailed(String),
    #[error("AI response parsing failed: {0}")]
    ParseError(String),
    #[error("Provider specific error: {0}")]
    ProviderError(String),
}

/// エージェントに関連するエラーを表す列挙型
#[derive(Error, Debug)]
pub enum AgentError {
    /// クライアントエラー
    #[error("Client error: {0}")]
    ClientError(#[from] crate::client::ChatError),

    /// タスク実行エラー
    #[error("Task execution error: {0}")]
    TaskExecutionError(String),

    /// タスクが見つからないエラー
    #[error("Task not found: {0}")]
    TaskNotFound(String),

    /// その他のエラー
    #[error("Other error: {0}")]
    Other(String),
}