use async_trait::async_trait;
use serde::Serialize;
use thiserror::Error;

pub mod deepseek;
pub mod gemini;
pub mod openai;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
    #[error("Unhandled exception: {0}")]
    UnknownError(String),
}

#[async_trait]
pub trait AiClient {
    async fn send_prompt(
        &self,
        prompt: impl Serialize + Send + Sync,
    ) -> Result<String, ClientError>;
}
