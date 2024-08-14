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