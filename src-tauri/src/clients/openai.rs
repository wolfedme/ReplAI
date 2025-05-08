use async_trait::async_trait;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};

use super::{AiClient, ClientError};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenAiConfig {
    pub api_url: String,
    pub default_model: String, // TODO: enum?
    pub timeout_secs: u64,
}

impl Default for OpenAiConfig {
    fn default() -> Self {
        OpenAiConfig {
            api_url: String::from("https://api.openai.com/v1/chat/completions"),
            default_model: String::from("gpt-3.5-turbo"),
            timeout_secs: 10,
        }
    }
}

pub struct OpenAiClient {
    config: OpenAiConfig,
    http_client: HttpClient,
}

#[async_trait]
impl AiClient for OpenAiClient {
    async fn send_prompt(
        &self,
        prompt: impl Serialize + Send + Sync,
    ) -> Result<String, ClientError> {
        // TODO: Implementation for sending a prompt to the ChatGPT API
        Ok("response".to_string())
    }
}
