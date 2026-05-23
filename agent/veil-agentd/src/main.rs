//! `veil-agentd` entry point: parse args, load config, init logging, serve.

use clap::Parser;
use std::path::PathBuf;
use veil_agentd::config::Config;

#[derive(Parser, Debug)]
#[command(
    name = "veil-agentd",
    version,
    about = "Veil Agent — system-level AI daemon for Veil OS"
)]
struct Cli {
    /// Path to the TOML config file. Built-in defaults are used if it is absent.
    #[arg(short, long, default_value = "/etc/veil/agent.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load(&cli.config)?;

    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log_level));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!(version = veil_agentd::VERSION, "starting veil-agentd");
    veil_agentd::server::serve(&config).await
}
