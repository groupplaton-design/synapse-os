# agent/

Source code of **Veil Agent** — the system-level AI daemon (`veil-agentd`).

Veil Agent is an MCP-based service that exposes a unified AI interface to all
native Veil OS apps, with controlled access to filesystem, active windows
(AT-SPI), terminal, browser, calendar/contacts/notes, and the local Veil Models
endpoints.

## Status — B-M1 skeleton (current)

Language decision: **Rust** (single static binary, strong async/MCP ecosystem,
memory safety for a daemon with filesystem access). See `docs/build-plan.md`.

What exists now: process lifecycle, TOML config with defaults, structured
logging, graceful shutdown, a local `/health` + `/version` API, the **first
permission-gated tool** (`fs_read`), and **local LLM chat** via Ollama.

```
agent/
  Cargo.toml            # workspace
  veil-agentd/          # the daemon crate
    src/main.rs         # entry point: `run` (daemon) and `chat` (omnibox stub)
    src/lib.rs          # library root
    src/config.rs       # TOML config + defaults (tested)
    src/server.rs       # axum API: /health, /version, /chat, /tools/fs/read
    src/permissions.rs  # Approver trait + FsPolicy guardrails (tested)
    src/tools.rs        # fs_read tool (tested)
    src/llm.rs          # ChatBackend trait + Ollama backend (tested)
    agent.example.toml  # sample config
```

### Talk to a local model

With [Ollama](https://ollama.com) running (`ollama serve` + `ollama pull
llama3.2`):

```
cd agent
# one-shot from the terminal (omnibox stand-in):
cargo run -p veil-agentd -- chat "summarize the idea of Veil OS"

# or via the running daemon:
cargo run -p veil-agentd            # in one shell
curl -s localhost:7421/chat -d '{"prompt":"hello"}' -H 'content-type: application/json'
```

The `ChatBackend` trait keeps the model source pluggable — cloud passthrough and
local↔cloud routing (per `docs/vision.md`) implement the same trait later.

### Permission model

Every tool action passes two layers:

1. **Approver** (decision point) — the interactive omnibox prompt will implement
   this. The skeleton default `RequireInteractiveApproval` *denies* anything
   needing approval, so nothing acts behind the user's back until the UI lands.
2. **FsPolicy** (hard guardrails) — reads must resolve (after canonicalization,
   which neutralizes `..` traversal) under a configured `fs_allowed_roots`, be a
   regular file, and fit `fs_max_read_bytes`.

Defaults are locked down: no roots permitted, approval required.

### Run locally (needs Rust installed)

```
cd agent
cargo run -p veil-agentd
# in another shell:
curl http://127.0.0.1:7421/health
# -> {"status":"ok","service":"veil-agentd","version":"0.1.0","uptime_secs":N}
```

Config is read from `/etc/veil/agent.toml` (override with `--config PATH`);
defaults apply when it is absent.

## Planned structure (later increments)

```
agent/
  veil-agentd/          # core daemon (this crate, grows)
  mcp-servers/          # built-in MCP servers: files, browser, terminal, calendar
  ui/                   # omnibox + tray UI (Qt/QML, KDE-native)
  voice/                # Whisper.cpp + Piper integration
  semantic-search/      # file indexing pipeline (embeddings, LanceDB/Qdrant)
  routing/              # local vs cloud auto-routing logic
```

Next up (B-M1 completion): first MCP tool end-to-end — **filesystem read with a
user permission prompt** — then the omnibox UI talking to local Ollama.
