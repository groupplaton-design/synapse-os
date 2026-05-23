//! Local LLM access. The daemon talks to a local **Ollama** instance by
//! default; the [`ChatBackend`] trait keeps that pluggable so cloud passthrough
//! and routing can be added later (per docs/vision.md) without touching the
//! call sites.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Errors a chat backend can return.
#[derive(Debug)]
pub enum LlmError {
    Backend(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::Backend(m) => write!(f, "llm backend error: {m}"),
        }
    }
}

impl std::error::Error for LlmError {}

/// A text-generation backend. Implemented by [`OllamaBackend`] today; cloud
/// providers and a routing layer implement the same trait later.
#[async_trait]
pub trait ChatBackend: Send + Sync {
    async fn generate(&self, model: &str, prompt: &str) -> Result<String, LlmError>;
}

/// Talks to a local Ollama server over HTTP.
pub struct OllamaBackend {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaBackend {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[async_trait]
impl ChatBackend for OllamaBackend {
    async fn generate(&self, model: &str, prompt: &str) -> Result<String, LlmError> {
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));
        let body = GenerateRequest {
            model,
            prompt,
            stream: false,
        };
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Backend(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(LlmError::Backend(format!(
                "ollama returned HTTP {}",
                resp.status()
            )));
        }

        let parsed: GenerateResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Backend(e.to_string()))?;
        Ok(parsed.response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_body_serializes() {
        let r = GenerateRequest {
            model: "llama3.2",
            prompt: "hi",
            stream: false,
        };
        let j = serde_json::to_value(&r).unwrap();
        assert_eq!(j["model"], "llama3.2");
        assert_eq!(j["prompt"], "hi");
        assert_eq!(j["stream"], false);
    }

    struct MockBackend;

    #[async_trait]
    impl ChatBackend for MockBackend {
        async fn generate(&self, _model: &str, prompt: &str) -> Result<String, LlmError> {
            Ok(format!("echo: {prompt}"))
        }
    }

    #[tokio::test]
    async fn backend_trait_generates() {
        let backend = MockBackend;
        let out = backend.generate("any", "hello").await.unwrap();
        assert_eq!(out, "echo: hello");
    }
}
