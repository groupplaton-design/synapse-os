//! Veil Agent daemon (`veil-agentd`) — the system-level AI service for Veil OS.
//!
//! Milestone B-M1 (skeleton): process lifecycle, config, structured logging,
//! and a local health/version endpoint. MCP tool servers (filesystem,
//! terminal, browser, ...) land in later increments — see
//! `docs/build-plan.md` Track B.

pub mod agent;
pub mod config;
pub mod llm;
pub mod permissions;
pub mod server;
pub mod tools;

/// Crate version, sourced from Cargo at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
