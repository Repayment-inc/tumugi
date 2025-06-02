use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChatOpenAIError {
    #[error("Invalid temperature value")]
    InvalidTemperature,
    #[error("API error: {0}")]
    ApiError(String),
    // 他のエラー型
}