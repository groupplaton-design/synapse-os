# Veil OS — Native Products Backlog

Tier ordering is by criticality, not by build order. Many items can be a thin custom layer over an existing open-source project on first release.

---

## Tier 1 — System won't ship without these

| Product | Base / fork of | Purpose |
|---|---|---|
| **Veil Agent** | Custom (MCP server in Rust/Go) | The system-level AI daemon. Speaks to all other apps via MCP. Main differentiator of the OS. |
| **Veil Files** | Fork of Dolphin (KDE) | File manager with semantic search, AI tagging, right-click "Summarize / Translate / Explain". |
| **Veil Browser** | Fork of LibreWolf or Floorp | Hardened Firefox with built-in AI sidebar, arkenfox defaults, anti-fingerprinting. |
| **Veil Models** | Custom | Model Hub — App Store for AI models. Hardware-aware install, unified Ollama-compatible API. |
| **Veil Capture** | Fork of Spectacle | Screenshot tool with built-in OCR and "Explain this screenshot" via local LLM. |
| **Veil Vault** | Fork of KeePassXC | Password manager, 2FA, secret storage, hardware key support. |
| **Veil Backup** | Wrapper over restic + bootc | GUI for cloud backup + System-as-Code via bootc → personal GitHub repo. |
| **Veil Restore Center** | Custom over btrfs + rpm-ostree | Time Machine UX for btrfs snapshots. Rollback from GUI or from GRUB at boot. |

---

## Tier 2 — Needed for everyday work

| Product | Base / fork of | Purpose |
|---|---|---|
| **Veil Office** | Fork of OnlyOffice | Word/Excel/PowerPoint-compatible docs/tables/slides. AI ghostwriter and explainer in sidebar. |
| **Veil Mail** | Fork of Thunderbird (Supernova) or Geary | Email client with AI-agent assistant (reply suggestions, summarize threads, smart filters). |
| **Veil Notes** | Fork of Logseq or AppFlowy | Local-first knowledge base with AI Q&A over notes. |
| **Veil Code** | Fork of VSCodium + Continue.dev | IDE with Continue.dev preinstalled, our default local code model (Qwen 2.5 Coder). |
| **Veil Voice** | Custom over Whisper.cpp + Piper | Voice assistant. Wake word "Hey Veil". Fully on-device. |

---

## Tier 3 — Hub / power-user features

| Product | Base / fork of | Purpose |
|---|---|---|
| **Veil Hub Manager** | Custom (Tauri / Electron / KDE) | GUI for server mode: Docker, Tailscale Funnel, federated AI inference, SSH config. |
| **Veil Sync** | Custom over Syncthing | Sync settings, AI agent memory, files between Veil devices. |
| **Veil Wallet Hub** | Aggregator | Unified dashboard reading from Sparrow / Cake / Rabby / Phantom / BlueWallet. Net worth, transactions, hardware wallet management. |
| **Veil Photos** | Fork of Immich client | Gallery with local CLIP search, face recognition. Sync to self-hosted Immich. |
| **Veil Smart Home** | UI over Home Assistant | Native KDE UI for Home Assistant. Voice control via Veil Voice. |

---

## Tier 4 — Pro features / Studio

| Product | Base / fork of | Purpose |
|---|---|---|
| **Veil Studio** | ComfyUI + custom | Curated AI pipelines for creative work: video upscale, voice cloning, image generation, stem separation. |
| **Veil Drive** | Wrapper over rclone | Mount and browse cloud storages (S3, B2, R2, Storj) as if they were local folders, with AI search across them. |
| **Veil Companion (iOS / Android)** | Native mobile apps | Remote-controls a user's Veil home machine — sends AI queries, manages backups, controls smart home. Lets users keep iOS / Android phones. |

---

## Out of scope (intentionally)

- Own crypto wallet implementation (too much liability)
- Own DEX, own token, own NFT marketplace
- Own video conferencing service (use Jitsi or web)
- Own music streaming (use Spotify / YouTube Music / self-hosted Navidrome)
- Own mobile OS — far future, not in years 1-2
- macOS app compatibility (Darling is too immature)

---

## Build strategy

1. **Don't build from scratch where a quality open-source project exists.** Fork and re-skin first; rebuild later only if differentiation demands it.
2. **Veil Agent + Veil Models are the only hard-from-scratch components** in Tier 1. Everything else is a thin custom layer.
3. **Ship Tier 1 in v1.0, Tier 2 in v1.5, Tier 3-4 in v2.0+.**
4. **Veil Companion mobile apps** can ship parallel to v1.0 — they don't depend on the OS being finished.
