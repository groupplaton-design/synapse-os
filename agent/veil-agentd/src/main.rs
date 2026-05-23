//! `veil-agentd` entry point: parse args, load config, then either run the
//! daemon or perform a one-shot chat (omnibox stand-in).

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::sync::Arc;
use veil_agentd::agent::Agent;
use veil_agentd::config::Config;
use veil_agentd::llm::{ChatBackend, OllamaBackend};
use veil_agentd::permissions::{Approver, AutoApprover, FsPolicy, RequireInteractiveApproval};

#[derive(Parser, Debug)]
#[command(
    name = "veil-agentd",
    version,
    about = "Veil Agent — system-level AI daemon for Veil OS"
)]
struct Cli {
    /// Path to the TOML config file. Built-in defaults are used if it is absent.
    #[arg(short, long, default_value = "/etc/veil/agent.toml", global = true)]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run the agent daemon (default).
    Run,
    /// Ask the local model once and print the answer (omnibox stand-in).
    Chat {
        /// The prompt (all trailing words are joined).
        prompt: Vec<String>,
        /// Override the model; defaults to config `default_model`.
        #[arg(short, long)]
        model: Option<String>,
    },
    /// Ask the agent (model may call tools, gated by permissions).
    Ask {
        /// The prompt (all trailing words are joined).
        prompt: Vec<String>,
        /// Override the model; defaults to config `default_model`.
        #[arg(short, long)]
        model: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = Config::load(&cli.config)?;

    match cli.command.unwrap_or(Command::Run) {
        Command::Run => {
            let filter = tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.log_level));
            tracing_subscriber::fmt().with_env_filter(filter).init();

            tracing::info!(version = veil_agentd::VERSION, "starting veil-agentd");
            veil_agentd::server::serve(&config).await
        }
        Command::Chat { prompt, model } => {
            let backend = OllamaBackend::new(config.ollama_url.clone());
            let model = model.unwrap_or_else(|| config.default_model.clone());
            let answer = backend.generate(&model, &prompt.join(" ")).await?;
            println!("{answer}");
            Ok(())
        }
        Command::Ask { prompt, model } => {
            let backend = Arc::new(OllamaBackend::new(config.ollama_url.clone()));
            let fs_policy = Arc::new(FsPolicy {
                allowed_roots: config.fs_allowed_roots.clone(),
                max_read_bytes: config.fs_max_read_bytes,
            });
            let approver: Arc<dyn Approver> = if config.require_approval {
                Arc::new(RequireInteractiveApproval)
            } else {
                Arc::new(AutoApprover)
            };
            let model = model.unwrap_or_else(|| config.default_model.clone());
            let agent = Agent::new(backend, model, fs_policy, approver);
            let outcome = agent.run(&prompt.join(" ")).await?;
            for step in &outcome.steps {
                eprintln!("[tool] {} {} -> {}", step.tool, step.argument, step.outcome);
            }
            println!("{}", outcome.answer);
            Ok(())
        }
    }
}
