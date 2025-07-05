use std::collections::HashMap;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LLMMessage {
    pub role: String,
    pub content: String,
    pub images: Option<Vec<String>>
}

#[async_trait]
pub trait LLM: Send + Sync {
    async fn complete(
        &self,
        messages: Vec<LLMMessage>,
        temperature: Option<f32>,
        format: Option<HashMap<String, serde_json::Value>>,
    ) -> LLMMessage;
}
