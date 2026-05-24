//! Daemon configuration: loaded from a TOML file, falling back to built-in
//! defaults so the daemon always starts even with no config present.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};

/// Runtime configuration for `veil-agentd`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Address the local API binds to. Loopback by default — the agent is a
    /// local system service, not a network service.
    pub listen_addr: IpAddr,
    /// Port for the local API.
    pub listen_port: u16,
    /// Tracing log level used when `RUST_LOG` is not set.
    pub log_level: String,
    /// Filesystem roots the fs tools may read from. Empty = nothing permitted.
    pub fs_allowed_roots: Vec<PathBuf>,
    /// Largest single fs read permitted, in bytes.
    pub fs_max_read_bytes: u64,
    /// Require interactive user approval for tool actions. When true and no
    /// interactive approver is attached (the current skeleton), guarded actions
    /// are denied — the safe default until the omnibox prompt lands.
    pub require_approval: bool,
    /// Base URL of the local Ollama server.
    pub ollama_url: String,
    /// Model used for chat when the request does not name one.
    pub default_model: String,
    /// Allow the agent's terminal_exec tool. Off by default — even with
    /// approval, arbitrary command execution stays disabled until opted in.
    pub enable_terminal: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
            listen_port: 7421,
            log_level: "info".to_string(),
            fs_allowed_roots: Vec::new(),
            fs_max_read_bytes: 1_048_576,
            require_approval: true,
            ollama_url: "http://127.0.0.1:11434".to_string(),
            default_model: "llama3.2".to_string(),
            enable_terminal: false,
        }
    }
}

impl Config {
    /// Load config from `path`. Missing file → defaults. Present but invalid →
    /// error (we do not silently ignore a broken config).
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if path.exists() {
            let text = std::fs::read_to_string(path)?;
            let cfg: Config = toml::from_str(&text)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }

    /// The socket address the API listens on.
    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.listen_addr, self.listen_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_bind_loopback() {
        let c = Config::default();
        assert_eq!(c.listen_port, 7421);
        assert!(c.socket_addr().ip().is_loopback());
    }

    #[test]
    fn defaults_are_locked_down() {
        let c = Config::default();
        assert!(c.fs_allowed_roots.is_empty());
        assert!(c.require_approval);
    }

    #[test]
    fn partial_toml_fills_defaults() {
        // Only one field provided; the rest must come from Default.
        let c: Config = toml::from_str("listen_port = 9000").unwrap();
        assert_eq!(c.listen_port, 9000);
        assert_eq!(c.log_level, "info");
        assert!(c.require_approval);
        assert!(c.socket_addr().ip().is_loopback());
    }
}
