# Synapse OS

AI-native Linux operating system for ML engineers, creators, and privacy-conscious users.

Built on Fedora Atomic + bootc with a local-first AI agent woven into every layer of the system — from semantic file search to system-level automation. Familiar UX from Windows and macOS on top of an immutable, atomically-updated Linux base. NVIDIA/AMD/Intel GPUs and modern NPUs work out of the box.

## Status

Early-stage. Vision and architecture phase. See [docs/vision.md](docs/vision.md).

## Foundation

- **Base:** Fedora Atomic (Silverblue/Kinoite) + bootc
- **Desktop:** KDE Plasma 6 with Windows / macOS / Power-user layout presets
- **Display server:** Wayland-only
- **AI runtime:** Local-first (Ollama, llama.cpp, Whisper.cpp, Piper) with opt-in cloud passthrough
- **Hardware:** NVIDIA CUDA, AMD ROCm, Intel IPEX, plus NPU support (Intel OpenVINO, AMD XDNA) out of the box
- **Compatibility:** Bottles + Steam/Proton for Windows software, WayDroid for Android apps

## Repository layout

- [docs/](docs/) — vision, architecture, roadmap
- [recipes/](recipes/) — bootc / Containerfile recipes for OS image builds
- [agent/](agent/) — Synapse Agent (local AI agent, MCP server, system integrations)
- [iso/](iso/) — ISO build scripts and outputs
- [assets/](assets/) — branding, icons, wallpapers

## License

Apache License 2.0 — see [LICENSE](LICENSE).
