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
logging, graceful shutdown, a local `/health` + `/version` API, and the **first
permission-gated tool** — `fs_read`.

```
agent/
  Cargo.toml            # workspace
  veil-agentd/          # the daemon crate
    src/main.rs         # entry point: args -> config -> logging -> serve
    src/lib.rs          # library root
    src/config.rs       # TOML config + defaults (tested)
    src/server.rs       # axum API: /health, /version, /tools/fs/read (tested)
    src/permissions.rs  # Approver trait + FsPolicy guardrails (tested)
    src/tools.rs        # fs_read tool (tested)
    agent.example.toml  # sample config
```

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
