use std::{collections::HashMap, env};

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{LLM, LLMMessage};

#[derive(Serialize)]
struct OllamaBody {
    model: String,
    messages: Vec<LLMMessage>,
    stream: bool,
    options: OllamaOptions,
    format: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Serialize)]
struct OllamaOptions {
    num_ctx: i32,
    temperature: Option<f32>,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: LLMMessage,
}

pub struct Ollama {
    model: String,
    num_ctx: i32,
    url: String,
    http_client: Client,
}

impl Ollama {
    pub fn new() -> Self {
        Self {
            model: env::var("OLLAMA_MODEL").unwrap_or("gemma3".to_string()),
            num_ctx: env::var("NUM_CTX")
                .unwrap_or("2048".to_string())
                .parse()
                .unwrap(),
            url: env::var("OLLAMA_URL").unwrap_or("http://localhost:11434".to_string()),
            http_client: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }
}

#[async_trait]
impl LLM for Ollama {
    async fn complete(
        &self,
        messages: Vec<LLMMessage>,
        temperature: Option<f32>,
        format: Option<HashMap<String, serde_json::Value>>,
    ) -> LLMMessage {
        let request_url = format!("{}/api/chat", self.url.clone());
        let body = OllamaBody {
            model: self.model.clone(),
            messages,
            stream: false,
            options: OllamaOptions {
                num_ctx: self.num_ctx,
                temperature,
            },
            format,
        };

        let response = self.http_client.post(request_url).json(&body).send().await;

        if response.is_err() {
            return LLMMessage {
                role: "assistant".to_string(),
                content: "".to_string(),
                images: None
            };
        }

        let json_data = response.unwrap().json::<OllamaResponse>().await;

        if json_data.is_err() {
            return LLMMessage {
                role: "assistant".to_string(),
                content: "".to_string(),
                images: None
            };
        }

        json_data.unwrap().message
    }
}
