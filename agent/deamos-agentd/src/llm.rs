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

/// A chat message exchanged with the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tool_name: Option<String>,
}

impl Message {
    pub fn system(content: impl Into<String>) -> Self {
        Self::plain("system", content)
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self::plain("user", content)
    }
    /// A tool-result message fed back to the model.
    pub fn tool(name: &str, content: String) -> Self {
        Self {
            role: "tool".to_string(),
            content,
            tool_calls: None,
            tool_name: Some(name.to_string()),
        }
    }
    fn plain(role: &str, content: impl Into<String>) -> Self {
        Self {
            role: role.to_string(),
            content: content.into(),
            tool_calls: None,
            tool_name: None,
        }
    }
}

/// A tool call requested by the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    /// Arguments as a JSON object, e.g. `{ "path": "/etc/hostname" }`.
    pub arguments: serde_json::Value,
}

/// A tool advertised to the model.
#[derive(Debug, Clone, Serialize)]
pub struct ToolDef {
    #[serde(rename = "type")]
    pub kind: String,
    pub function: FunctionDef,
}

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    /// JSON-schema describing the arguments.
    pub parameters: serde_json::Value,
}

/// A text/chat backend. Implemented by [`OllamaBackend`] today; cloud providers
/// and a routing layer implement the same trait later.
#[async_trait]
pub trait ChatBackend: Send + Sync {
    /// Single-shot completion (no tools). Used by `/chat` and the `chat` CLI.
    async fn generate(&self, model: &str, prompt: &str) -> Result<String, LlmError>;

    /// Multi-turn chat with optional tool calling. Used by the agent loop.
    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        tools: &[ToolDef],
    ) -> Result<Message, LlmError>;
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

    fn endpoint(&self, path: &str) -> String {
        format!("{}{path}", self.base_url.trim_end_matches('/'))
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

#[derive(Serialize)]
struct ChatApiRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    tools: &'a [ToolDef],
    stream: bool,
}

#[derive(Deserialize)]
struct ChatApiResponse {
    message: Message,
}

#[async_trait]
impl ChatBackend for OllamaBackend {
    async fn generate(&self, model: &str, prompt: &str) -> Result<String, LlmError> {
        let body = GenerateRequest {
            model,
            prompt,
            stream: false,
        };
        let resp = self
            .client
            .post(self.endpoint("/api/generate"))
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

    async fn chat(
        &self,
        model: &str,
        messages: &[Message],
        tools: &[ToolDef],
    ) -> Result<Message, LlmError> {
        let body = ChatApiRequest {
            model,
            messages,
            tools,
            stream: false,
        };
        let resp = self
            .client
            .post(self.endpoint("/api/chat"))
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
        let parsed: ChatApiResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Backend(e.to_string()))?;
        Ok(parsed.message)
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

    #[test]
    fn tool_message_carries_name() {
        let m = Message::tool("fs_read", "data".to_string());
        let j = serde_json::to_value(&m).unwrap();
        assert_eq!(j["role"], "tool");
        assert_eq!(j["tool_name"], "fs_read");
        assert!(j.get("tool_calls").is_none());
    }

    struct MockBackend;

    #[async_trait]
    impl ChatBackend for MockBackend {
        async fn generate(&self, _model: &str, prompt: &str) -> Result<String, LlmError> {
            Ok(format!("echo: {prompt}"))
        }
        async fn chat(
            &self,
            _model: &str,
            _messages: &[Message],
            _tools: &[ToolDef],
        ) -> Result<Message, LlmError> {
            Ok(Message::plain("assistant", "ok"))
        }
    }

    #[tokio::test]
    async fn backend_trait_generates() {
        let backend = MockBackend;
        let out = backend.generate("any", "hello").await.unwrap();
        assert_eq!(out, "echo: hello");
    }
}
