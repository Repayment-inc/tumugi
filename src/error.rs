use thiserror::Error;

#[derive(Error, Debug)]
pub enum TumugiError {
    #[error("AI API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("AI response parsing failed: {0}")]
    ParseError(String),
    #[error("Provider specific error: {0}")]
    ProviderError(String),
    #[error("Unsupported AI model: {0}")]
    UnsupportedModel(String),
    #[error("API error: {0}")]
    ApiError(String),
}

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Client error: {0}")]
    ClientError(#[from] TumugiError),
    #[error("Task execution error: {0}")]
    TaskExecutionError(String),
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Communication error")]
    CommunicationError,
    #[error("Other error: {0}")]
    Other(String),
}

impl From<serde_json::Error> for TumugiError {
    fn from(err: serde_json::Error) -> Self {
        TumugiError::ParseError(err.to_string())
    }
}
