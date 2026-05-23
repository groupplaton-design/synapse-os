//! Local HTTP API for the daemon. For the B-M1 skeleton this is just health
//! and version; MCP routes are added in later increments.

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::time::Instant;

use crate::config::Config;

/// Shared application state handed to every request.
#[derive(Clone)]
pub struct AppState {
    /// When the daemon started, for uptime reporting.
    pub started: Instant,
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
        service: "veil-agentd",
        version: crate::VERSION,
        uptime_secs: started.elapsed().as_secs(),
    }
}

/// Construct the router with all routes and shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/version", get(version))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<Health> {
    Json(health_payload(state.started))
}

async fn version() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "version": crate::VERSION }))
}

/// Bind the configured address and serve until a shutdown signal arrives.
pub async fn serve(config: &Config) -> anyhow::Result<()> {
    let state = AppState {
        started: Instant::now(),
    };
    let app = router(state);
    let addr = config.socket_addr();

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "veil-agentd listening");

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
        assert_eq!(h.service, "veil-agentd");
        assert_eq!(h.version, crate::VERSION);
    }
}
