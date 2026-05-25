//! Local HTTP API for the daemon: health/version plus permission-gated tools.

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use crate::agent::{Agent, AgentOutcome};
use crate::config::Config;
use crate::llm::{ChatBackend, OllamaBackend};
use crate::permissions::{
    Approver, AutoApprover, Decision, FsPolicy, InteractiveApprover, PermissionRequest,
};
use crate::tools::{fs_read, FsReadResponse, ToolError};

/// Shared application state handed to every request.
#[derive(Clone)]
pub struct AppState {
    pub started: Instant,
    pub fs_policy: Arc<FsPolicy>,
    pub approver: Arc<dyn Approver>,
    pub chat: Arc<dyn ChatBackend>,
    pub default_model: String,
    pub enable_terminal: bool,
}

/// Health response body.
#[derive(Debug, Serialize)]
pub struct Health {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
    pub uptime_secs: u64,
}

/// Build the health payload. Kept pure so it is trivially unit-testable.
pub fn health_payload(started: Instant) -> Health {
    Health {
        status: "ok",
        service: "deamos-agentd",
        version: crate::VERSION,
        uptime_secs: started.elapsed().as_secs(),
    }
}

/// Construct the router with all routes and shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .route("/chat", post(chat))
        .route("/agent", post(agent))
        .route("/tools/fs/read", post(tools_fs_read))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<Health> {
    Json(health_payload(state.started))
}

async fn version() -> Json<serde_json::Value> {
    Json(json!({ "version": crate::VERSION }))
}

#[derive(Debug, Deserialize)]
struct ChatRequest {
    prompt: String,
    #[serde(default)]
    model: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    model: String,
    response: String,
}

async fn chat(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<serde_json::Value>)> {
    let model = req.model.unwrap_or_else(|| state.default_model.clone());
    match state.chat.generate(&model, &req.prompt).await {
        Ok(text) => Ok(Json(ChatResponse {
            model,
            response: text,
        })),
        Err(e) => {
            tracing::warn!(%model, error = %e, "chat backend failed");
            Err((
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": "llm_error", "reason": e.to_string() })),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
struct AgentRequest {
    prompt: String,
    #[serde(default)]
    model: Option<String>,
}

async fn agent(
    State(state): State<AppState>,
    Json(req): Json<AgentRequest>,
) -> Result<Json<AgentOutcome>, (StatusCode, Json<serde_json::Value>)> {
    let model = req.model.unwrap_or_else(|| state.default_model.clone());
    let agent = Agent::new(
        state.chat.clone(),
        model,
        state.fs_policy.clone(),
        state.approver.clone(),
        state.enable_terminal,
    );
    match agent.run(&req.prompt).await {
        Ok(outcome) => Ok(Json(outcome)),
        Err(e) => {
            tracing::warn!(error = %e, "agent run failed");
            Err((
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": "agent_error", "reason": e.to_string() })),
            ))
        }
    }
}

#[derive(Debug, Deserialize)]
struct FsReadRequest {
    path: String,
}

async fn tools_fs_read(
    State(state): State<AppState>,
    Json(req): Json<FsReadRequest>,
) -> Result<Json<FsReadResponse>, (StatusCode, Json<serde_json::Value>)> {
    let request = PermissionRequest {
        tool: "fs".to_string(),
        action: "read".to_string(),
        resource: req.path.clone(),
    };

    // Permission gate: the interactive omnibox approver will plug in here.
    if let Decision::Deny(reason) = state.approver.decide(&request) {
        tracing::info!(resource = %request.resource, %reason, "tool request denied");
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "denied", "reason": reason })),
        ));
    }

    match fs_read(&state.fs_policy, Path::new(&req.path)) {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(tool_error_response(e)),
    }
}

fn tool_error_response(e: ToolError) -> (StatusCode, Json<serde_json::Value>) {
    let (code, kind, reason) = match e {
        ToolError::Denied(r) => (StatusCode::FORBIDDEN, "denied", r),
        ToolError::NotAFile => (
            StatusCode::BAD_REQUEST,
            "not_a_file",
            "path is not a regular file".to_string(),
        ),
        ToolError::TooLarge { max, actual } => (
            StatusCode::PAYLOAD_TOO_LARGE,
            "too_large",
            format!("file is {actual} bytes, limit is {max}"),
        ),
        ToolError::Io(m) => (StatusCode::INTERNAL_SERVER_ERROR, "io_error", m),
    };
    (code, Json(json!({ "error": kind, "reason": reason })))
}

/// Bind the configured address and serve until a shutdown signal arrives.
pub async fn serve(config: &Config) -> anyhow::Result<()> {
    let fs_policy = Arc::new(FsPolicy {
        allowed_roots: config.fs_allowed_roots.clone(),
        max_read_bytes: config.fs_max_read_bytes,
    });
    let approver: Arc<dyn Approver> = if config.require_approval {
        Arc::new(InteractiveApprover)
    } else {
        Arc::new(AutoApprover)
    };
    let chat: Arc<dyn ChatBackend> = Arc::new(OllamaBackend::new(config.ollama_url.clone()));
    let state = AppState {
        started: Instant::now(),
        fs_policy,
        approver,
        chat,
        default_model: config.default_model.clone(),
        enable_terminal: config.enable_terminal,
    };

    let addr = config.socket_addr();
    let app = router(state);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "deamos-agentd listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Resolve when the process is asked to stop (Ctrl-C / SIGINT).
async fn shutdown_signal() {
    if tokio::signal::ctrl_c().await.is_err() {
        tracing::error!("failed to install shutdown handler");
    }
    tracing::info!("shutdown signal received, stopping");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_reports_ok() {
        let h = health_payload(Instant::now());
        assert_eq!(h.status, "ok");
        assert_eq!(h.service, "deamos-agentd");
        assert_eq!(h.version, crate::VERSION);
    }
}
