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
logging, graceful shutdown, and a local `/health` + `/version` API. No MCP tools
yet — those are the next increments.

```
agent/
  Cargo.toml            # workspace
  veil-agentd/          # the daemon crate
    src/main.rs         # entry point: args -> config -> logging -> serve
    src/lib.rs          # library root
    src/config.rs       # TOML config + defaults (tested)
    src/server.rs       # axum API: /health, /version (tested)
    agent.example.toml  # sample config
```

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
