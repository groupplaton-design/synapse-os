# agent/

Source code of **Veil Agent** — the system-level AI daemon.

Veil Agent is an MCP-based service that exposes a unified AI interface to all native Veil OS apps, with controlled access to:

- Filesystem (with user permissions)
- Active windows and their content (AT-SPI)
- Terminal
- Browser (via extension)
- Calendar, contacts, notes
- Veil Models (local LLM / diffusion endpoints)

Planned structure:

```
agent/
  core/              # Rust or Go MCP server core
  ui/                # Omnibox + tray UI (KDE / Qt)
  mcp-servers/       # Built-in MCP servers: files, browser, terminal, calendar, etc.
  voice/             # Whisper.cpp + Piper integration
  semantic-search/   # File indexing pipeline (embeddings, LanceDB/Qdrant)
  routing/           # Local vs cloud auto-routing logic
```

Tech-stack decision pending — choice between Rust and Go for the core daemon will be made in Phase 1.

Files will arrive in Phase 1 of the roadmap.
