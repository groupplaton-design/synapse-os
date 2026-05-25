# DeamOS — Vision and Architecture

> Working codename: **synapse-os** (in repo). Public brand: **DeamOS** — pending trademark and domain verification before public launch.

---

## One-liner

**AI-native Linux operating system for ML engineers, creators, and privacy-conscious users.**

Local-first AI agent woven into every layer of the system — from semantic file search to system-level automation. Familiar UX from Windows and macOS on top of an immutable, atomically-updated Linux base. NVIDIA / AMD / Intel GPUs and modern NPUs work out of the box.

---

## Positioning

Not "ChatGPT bundled with Linux." DeamOS is an **operating system that understands what the user wants to do and has hands inside the system to actually do it** — while keeping all data on the device by default.

Three target audiences (in priority order):

1. **ML / AI engineers** — tired of fighting CUDA drivers on Ubuntu, want a workstation that just works.
2. **Creators** (Stable Diffusion, ComfyUI, video, music) — want preconfigured AI pipelines, GPU acceleration OOTB.
3. **Privacy-conscious users** — refuse Copilot+ PC and Apple Intelligence, want sovereign AI on their own hardware.

Linux desktop will never be a mass market. DeamOS targets a niche $50M+ business with strong margins, not a Windows replacement for grandmothers.

---

## Foundation

| Layer | Choice | Rationale |
|---|---|---|
| **Base distribution** | Fedora Atomic (Silverblue / Kinoite) + **bootc** | Immutable root → user cannot break the system. Atomic updates via OCI images. The right model for the next decade. |
| **Filesystem** | Btrfs with automatic snapshots before every system change | One-click rollback. Time Machine–style browsing. |
| **Display server** | **Wayland only** | X11 is dead. No legacy support. |
| **Kernel** | Fedora kernel + patches tuned for AI workloads | Larger huge pages, lower inference latency, better VRAM management. |
| **Desktop Environment** | **KDE Plasma 6** with custom shell layouts | Best Wayland support, highly customizable, performant. |
| **Init / service manager** | systemd | Standard Linux. |

---

## Desktop Experience

First-boot wizard asks: "Where are you coming from?" Loads one of these layouts:

- **Windows mode** — taskbar at bottom, Start menu, Alt+Tab, Dolphin file manager configured Explorer-like.
- **macOS mode** — top bar + bottom dock, Cmd ↔ Super mapping, Spotlight (Cmd+Space).
- **Power user mode** — tiling plugin, no decorations, keyboard-driven.
- **Newbie mode** — minimal panel, large icons, simple search.

Layouts can be swapped at any time without reinstall. One base theme: dark + light auto-switch by time of day, Material You–style accent color from the wallpaper.

---

## AI Layer (the differentiating core)

### Deamos Agent — system-level AI daemon

An MCP-based daemon with controlled access to:

- Filesystem (with user permissions)
- Active windows and their content (via AT-SPI)
- Terminal
- Browser (via extension)
- Calendar, contacts, notes

Default models:

- **Local LLM** for routine work — Llama 3.3 70B Q4 on >24 GB VRAM machines, Qwen 2.5 32B for mid-range, Phi-4 for low-end.
- **Cloud passthrough** (Claude / OpenAI / Gemini / OpenRouter) — opt-in, with explicit user consent. Used for heavy reasoning only.
- **Auto-routing** — lightweight queries go to the local model, complex queries get user confirmation before going to the cloud.

Triggers:

- **Voice** — `whisper.cpp` locally, hotword "Hey Deamos"
- **Text** — global hotkey (Super+Space) opens the omnibox (Spotlight + ChatGPT + launcher in one)
- **Context** — right-click any file or text → "Ask Deamos about this"

### Semantic file search

Background process indexes all user files with local embedding models (`nomic-embed-text` or `BGE-M3`):

- Documents → text embeddings
- Images → CLIP embeddings
- Audio → Whisper transcription + embeddings
- PDF / EPUB → parse + embeddings

Queries like *"find that spreadsheet about marketing from last spring"* work locally. Vector store: LanceDB or Qdrant in embedded mode.

### Deamos Models — local Model Hub

App Store, but for AI models:

- Catalog of LLMs (Hugging Face mirror), diffusion, TTS, STT, embedding
- **Hardware-aware install** — detects hardware, recommends optimal quantization (Q4 / Q5 / Q8 / FP16)
- One-click install → model becomes available to all apps via a unified Ollama-compatible API endpoint
- Auto-update

### Native app integration

Every native app has AI pipelines built in:

- **Files** — right-click → Summarize, Translate, Tag automatically, Rename based on content
- **Screenshots** — automatic OCR + LLM explanation
- **Terminal** — `??` command — natural-language task description → AI-generated bash
- **Code editor** (VSCodium fork) — Continue.dev preinstalled with local Qwen 2.5 Coder
- **Image viewer** — AI upscale (Real-ESRGAN), background removal, inpainting (ComfyUI backend)

---

## Hardware Support

The biggest pain point on Linux. DeamOS solves it.

| Hardware | Support |
|---|---|
| **NVIDIA** | Drivers + CUDA 12.x preinstalled. No `apt install nvidia-driver-535` after install. |
| **AMD** | ROCm preinstalled with overrides for consumer cards (7900 XTX, 7800 XT work for inference immediately). |
| **Intel Arc** | IPEX-LLM built in. |
| **Intel NPU** (Lunar Lake / Arrow Lake) | OpenVINO routing for compatible models. |
| **AMD XDNA NPU** (Ryzen AI) | `xrt` + `xdna-driver` preinstalled. |
| **Apple Silicon** | Separate build via Asahi Linux fork. |
| **Wi-Fi / Bluetooth** | Intel and modern Broadcom OOTB. Realtek with non-free firmware on first boot. |
| **Printers / scanners** | CUPS + driverless IPP + SANE preinstalled. |
| **Graphics tablets** | Wacom, Huion, XP-Pen configured by default. Krita + GIMP preconfigured. |
| **Fingerprint readers** | fprintd. |
| **Game controllers** | xpadneo (Xbox), DualSense, Steam Controller — preinstalled. |
| **Secure Boot** | Custom signing keys, supported. |

**First-boot benchmark**: installer detects hardware, runs a quick AI benchmark, recommends an edition and a default local model:

> "On your hardware, Llama 3.3 70B Q4 runs at 25 t/s. Install?"

---

## Compatibility with Software From Other OSes

| Source OS | Tech | Coverage | Examples |
|---|---|---|---|
| **Linux** | Native (Flatpak, AppImage, RPM, Distrobox) | 100% | Everything |
| **Windows** | Wine + Bottles + Proton + DXVK | ~85% | MS Office, Photoshop CS6, AutoCAD, most Steam games, .NET apps, FL Studio, OBS |
| **Android** | WayDroid | ~95% | TikTok, Instagram, WhatsApp, banking apps, games (except Pokémon GO and anti-cheat–protected) |
| **DOS / Retro** | DOSBox, ScummVM, RetroArch | 100% | Doom, Quake, console emulators |
| **Web apps** | Chromium / Firefox PWA + system integration | 100% | Notion, Figma, Slack, Discord, Google Docs |
| **macOS** (Darling) | Very limited | ~5% | Only console / simple apps. Final Cut, Logic — no. |
| **iOS** | **Impossible** | 0% | Apple blocks it intentionally. |

Adobe Creative Cloud (current) and competitive shooters with kernel-level anti-cheat (Valorant, Fortnite, EA Sports Connect) — **not our market**. We do not pretend to please everyone.

---

## Privacy & Security

This is the positioning anchor against Copilot+ PC and Apple Intelligence.

### Defaults

- All AI operations are **local by default**. Cloud is opt-in with explicit consent.
- **No telemetry** without consent. Crash reports — opt-in.
- **Encrypted home** (LUKS2) by default.
- **MAC randomization** on Wi-Fi by default.
- **DNS-over-HTTPS** (Cloudflare 1.1.1.1 / Quad9 / NextDNS — user picks at onboarding).
- **Firefox** with arkenfox tweaks preinstalled, or LibreWolf.

### Encryption stack

| Layer | Tech | Default |
|---|---|---|
| Full Disk Encryption | LUKS2 + dm-crypt, AES-256-XTS | Enabled |
| Key derivation | Argon2id | Default |
| TPM 2.0 binding | Key sealed in TPM, auto-unlock | Opt-in |
| Secure Boot | Custom signing keys | Enabled |
| Encrypted swap | Random key per boot | Always |
| Encrypted /boot | LUKS2 without TPM | Opt-in (advanced) |
| Per-folder encryption | fscrypt / btrfs native | `~/private/` and AI agent memory always encrypted |
| Encrypted hibernation | TPM-bound | Default if TPM present |
| AI data | Embeddings + chat history in encrypted volume | **Hard requirement** |

### Anti-coercion features

- **Duress password** — second password that wipes encryption keys in 5 seconds. Plausible deniability against legal coercion.
- **Hidden encrypted volume** — VeraCrypt-style: inside the visible encrypted system there's a hidden one with a different password. Impossible to prove it exists.
- **TPM dictionary attack lockout** — after 5 failed attempts, TPM locks for 1 hour.
- **Auto-lock on tamper** — opening the lid of a powered-off laptop forces password re-entry (Cold Boot Attack mitigation).
- **Network kill on boot** — until the user enters the password, no network traffic. Defeats tracking.

### Algorithm notes

- AES-256-XTS with AES-NI runs faster than no encryption on modern CPUs (~2-5% penalty).
- ARM / Snapdragon X without AES-NI → switch to **Adiantum** (Google's algorithm for low-end devices).
- Post-quantum is not yet needed for LUKS2 symmetric crypto. For Pro subscription token signing — use **ML-DSA** (post-quantum signature) from day one.

### Anonymity tooling

- **Anonymous Pro subscription** — payment via Lightning Network / Monero / USDT-TRC20. No email, no name. UUID only. (Mullvad VPN model.)
- **System-wide kill switch** — single button disables all network. Creator / journalist mode.

#### Layered anonymity model

There is no single "anonymity button" — different threats need different tools. Deamos Agent helps the user pick the right layer for the task. We ship three, from light to paranoid:

| Layer | Tool | Threat it defeats | Cost |
|---|---|---|---|
| **1. Unblock** | AmneziaWG / obfuscated VPN (Shadowsocks, Cloak) | DPI, regional blocks (RU / Iran / China) | fast, low latency |
| **2. Hide IP** | **Tor** (+ Snowflake bridge if Tor itself is blocked) | local observer, IP-based tracking, access to `.onion` | medium latency |
| **3. Hide metadata** | **Nym mixnet** ("Stealth / Paranoid mode") | **global observer + traffic-pattern / timing correlation** | high latency |

- **Tor button** in tray — one-click routes all traffic through Tor. Snowflake bridge opt-in at onboarding helps censored users reach Tor.
- **Nym mixnet (Stealth mode)** — 5-hop mixnet with cover traffic and packet mixing. Unlike Tor (onion-routing, hides IP only), Nym also hides *traffic patterns and metadata*, defeating the global-observer timing-correlation attack that Tor's own threat model concedes. Slower than a plain VPN — reserved for the highest-stakes sessions. Nym's unlinkable credential payments align ideologically with our anonymous-subscription model.

> Tor is **not** deprecated and Nym does **not** replace it — they sit side by side under different threat models. Tor remains the workhorse for anonymous browsing and `.onion`; Nym is the metadata-grade upgrade for users facing a global adversary.

---

## Networking & Multi-device

### Base stack

- **NetworkManager** for Wi-Fi / Ethernet (familiar UI).
- **iwd** under the hood (Intel Wi-Fi daemon, faster than wpa_supplicant).
- **systemd-resolved** + DNS-over-HTTPS by default.

### Mesh / multi-device

| Tech | Role | Default |
|---|---|---|
| **Tailscale** | Zero-config mesh VPN between all user devices | Preinstalled, opt-in |
| **Yggdrasil** | Fully decentralized mesh | Option for enthusiasts |
| **mDNS / Avahi** | Local discovery — Deamos devices see each other in LAN | Enabled |
| **WiFi Direct + KDE Connect** | AirDrop-style file transfer Deamos ↔ Android | Default |
| **Deamos LAN protocol** | Deamos machines in LAN auto-federate: share AI inference, common model hub, shared agent memory | Default |

### VPN integration

Preinstalled (opt-in import of configs):

- **Mullvad** (recommended privacy default)
- **ProtonVPN**
- **WireGuard** (raw configs)
- **AmneziaVPN** — Russian open-source VPN with strong anti-censorship protocols (OpenVPN+Cloak, Shadowsocks, WireGuard, AmneziaWG). Critical for Russia / Iran / China markets.
- **Shadowsocks / V2Ray / Xray** clients — for censorship circumvention so AI cloud APIs can be reached from restricted regions.
- **NymVPN** — mixnet client powering Stealth mode (see *Layered anonymity model* above). Metadata-grade privacy, opt-in.

### Distributed Local Inference (killer feature)

Home workstation with 24 GB VRAM + laptop without GPU + old PC with 16 GB RAM.
In ordinary OS — the laptop chokes on AI.
In DeamOS — laptop sees workstation via Tailscale / LAN, **transparently uses its GPU** for inference. The small voice-command model runs locally; heavy work is routed to the home server. No cloud, no subscriptions, no data leaves your network.

Implementation: Ollama + RPC layer + mDNS service discovery.

---

## Backup, Restore, System-as-Code

### Three layers of backup

| Layer | Tech | Purpose |
|---|---|---|
| **Local snapshots** | Btrfs auto-snapshot before every update or package install | Second-level rollback, no data loss |
| **Full system image** | `bootc image push` → OCI image to USB / external SSD | Complete clone, bootable on any compatible hardware |
| **Cloud backup** | restic / kopia + age (or Borg) — zero-knowledge | Hardware loss protection |

### Cloud backup destinations (user-managed)

- Backblaze B2 (~$6/TB/month)
- AWS S3 / Cloudflare R2 / Wasabi
- **Storj / Filecoin / Sia** — decentralized, end-to-end encrypted by design
- **Deamos Cloud Backup** (Pro tier, +$5/month) — our infrastructure, but we technically cannot decrypt

### System-as-Code via bootc (killer feature)

The entire OS is described as a `Containerfile` in your private GitHub repo:

- **One file = your entire OS** — packages, configs, app stack, customizations
- **Migration to new hardware in 30 minutes** — install base Deamos → `bootc switch ghcr.io/yourname/my-machine:latest` → identical OS (apps, wallpaper, shortcuts, AI agent settings)
- **Rollback to any date** — every change is a git commit. Want your OS from 6 months ago? `git checkout` + `bootc switch`
- **Team / family** — one Containerfile = shared OS profile for you and your partner / colleagues

Nobody else offers this. Real wow-factor for developers.

### Deamos Restore Center

GUI in system settings — list of snapshots with previews of changes (Time Machine UX). Available in GRUB at boot too — recover without entering the system if an update broke boot.

---

## Crypto Integration

**Rule #1: we do not build our own wallet.** Too much responsibility. Instead, deep integration with proven open-source wallets.

### Preinstalled (optional at install)

| Wallet | Purpose |
|---|---|
| **Sparrow Wallet** | BTC, watch-only, hardware wallet support, Coin Control |
| **Cake Wallet / Feather** | Monero (privacy-first) |
| **Rabby Wallet** | EVM chains — better than MetaMask (shows what you're signing) |
| **Phantom** | Solana |
| **BlueWallet desktop** | Lightning Network |

**Hardware wallet OOTB**: Ledger, Trezor, ColdCard, BitBox — udev rules + libraries preinstalled. Plug in USB → it works.

**Encrypted keystore**: software-wallet private keys never live in plaintext. Stored in `~/.deamos/vault/` (separate fscrypt volume) + TPM-bound. Access requires biometric / PIN.

### Killer features

1. **Anonymous Pro subscription** — pay via Lightning Network (~2000 sats / month), Monero, or USDT-TRC20. No email, no name. UUID only.
2. **Pay-per-token cloud AI** — your Lightning wallet pays per-request directly to LLM providers via L402 / Lightning Service Provider. (Sphinx, Predict, etc. model — emerging.)
3. **AI agent with wallet permissions** — `transfer 100 USDT to address X`. Only with **explicit confirmation per transaction** via hardware wallet or PIN. AI **never has the seed phrase**.
4. **Privacy mode by default** — Monero for spending, CoinJoin (Whirlpool / JoinMarket) for BTC.

What we do NOT do: our own DEX, our own token, our own NFT marketplace. We do not slide into the crypto sewer. We are an OS — crypto is a tool for privacy and payments, not a product feature.

---

## Smart Home

In Hub mode, the Deamos machine becomes the **local AI brain of the home**. No Amazon, no Google, no Apple. Everything on user hardware.

### Preinstalled stack (optional Hub edition)

| Component | Role |
|---|---|
| **Home Assistant OS** in Podman | Main IoT hub |
| **Matter protocol** (via chip-tool) | Universal new IoT standard |
| **Zigbee2MQTT** + driver for ConBee II / Sonoff stick | Zigbee device support |
| **ESPHome** dashboard | DIY ESP32 devices |
| **Frigate** + Coral TPU support | AI cameras, local person / pet detection |
| **HACS** | Home Assistant Community Store |
| **Voice satellite (Wyoming protocol)** | Room microphones / speakers talk to local AI |

### Killer feature — cloudless voice assistant

Any Deamos machine (or Deamos-tablet in a room) becomes a voice assistant:

1. Wake word "Hey Deamos"
2. `whisper.cpp` STT locally
3. Local LLM understands
4. Action via Home Assistant API
5. Piper TTS responds

**Everything local. Everything private.** Neither Amazon nor Google hear it.

Context-aware: *"Open the blinds where I am right now"* — knows you're in the kitchen (because your phone is there via Tailscale).

Privacy mode features (killer differentiation over Alexa / Google Home):

- Physical mute switch on microphones (recommend ReSpeaker hardware)
- When muted — hardware red LED
- Wake-word detection runs on-device; audio leaves the device **only after wake-word triggers** a command
- Conversation logs encrypted, optional auto-delete after N days

---

## Business Model

OS is free. Revenue comes from:

### 1. Deamos Pro — $12/month or $99/year

- Cloud inference passthrough (OpenRouter under the hood): $5/month credits inside → 30-40% margin
- Settings sync, encrypted backup of AI agent memory, multi-device
- Premium voice model (ElevenLabs-grade TTS)
- Priority support

### 2. Hardware bundles

Partnership with ODMs (Tongfang, Clevo, BIWIN) — sell AI workstation laptops / mini-PCs with preinstalled DeamOS. $200-500 margin per device. System76 / Tuxedo model.

### 3. Deamos Enterprise — $50-150/seat/month

Privacy-first AI workstation for companies that cannot go to the cloud (legal, medical, defense, banking).

- Centralized management, MDM, audit logs, SSO
- On-premise model hub
- **Where the real money is.** 100 seats × $100 × 12 = $120k ARR from one client.

### 4. Deamos Studio (separate paid tier, $29/month)

Creator-focused build with DaVinci, preconfigured ComfyUI workflows, Blender + AI plugins.

### Pricing rules

- **PPP pricing (Purchasing Power Parity)** — Pro should be $4-5 in RU / UA / IN / LatAm, not $12.
- **Students and OSS maintainers** — free Pro by verification (GitHub Student Pack / SourceHut).
- **Free tier limits** — 50-100 cloud requests / month. Enough to taste, not enough to live on.
- **Trial** — 14 days Pro free on first launch.
- **Lifetime deal** — first 500 users at $199. Generates ~$100k cash instantly.

---

## Editions and Footprint

| Edition | ISO | Disk install | Contents |
|---|---|---|---|
| **Lite** | ~3 GB | 20 GB | Base OS + KDE + browser. AI is agent-API only, no local models. |
| **Standard** | ~7 GB | 80 GB | + preinstalled Phi-4 / Qwen 7B Q4, Whisper.cpp, base ComfyUI |
| **Studio** | ~18 GB | 200 GB | + diffusion models, video AI stack, full ComfyUI workflows |
| **Server (Hub)** | ~2.5 GB | 30 GB | Headless, for home server / federated AI node |

### RAM requirements

| Use | RAM | VRAM |
|---|---|---|
| Lite (cloud only) | 8 GB | — |
| Standard (7B local model) | 16 GB | 8 GB or unified memory |
| Power (32B local) | 32 GB | 24 GB |
| Studio (70B + Stable Diffusion concurrent) | 64 GB | 48 GB+ |

CPU-only inference works (llama.cpp) but at 5-10 t/s vs 50 t/s on GPU/NPU. NPU/GPU detection + edition recommendation at install — must-have.

---

## Mobile and Tablet Strategy

Three scenarios:

### A. DeamOS *installed on* tablet / mobile — partial, not everywhere

| Device | Possible? | Timeline |
|---|---|---|
| Microsoft Surface (Pro / Go / Laptop) | Yes, full | Day 1 (x86) |
| Framework Laptop 13 / 16 | Yes, full | Day 1 |
| 2-in-1 / convertibles (ThinkPad X1 Yoga, HP Spectre x360) | Yes + touch / stylus | Day 1 |
| **iPad** | **No** | Never (Apple blocks) |
| Android tablet (Samsung Tab, Lenovo) | Very hard — proprietary ARM blobs | Not in years 1-2 |
| PineTab 2 / PineTab-V | Yes natively | Day 1 |
| **PinePhone Pro / Librem 5** | Yes | Day 90+ |
| Android phones generally | Possible via postmarketOS approach — but every model = separate port | Not in years 1-2 |
| **iPhone** | **No** | Never |

### B. Deamos Companion app for existing iOS / Android — yes, ship it

Compromise: user's phone stays on iOS / Android, but installs **Deamos Companion** app that:

- Connects to user's home Deamos machine via Tailscale / LAN
- Sends AI queries there, gets answers
- Backs up photos / contacts to user's Deamos Cloud (zero-knowledge)
- Controls smart home through the home machine
- Sends agent commands: *"find that March presentation"* → 10 sec → result

**More realistic path to mobile users** than building a phone OS from scratch.

### C. Deamos Mobile OS — long-term dream, not now

In 2-3 years, if the main product takes off — fork postmarketOS / Mobile NixOS, support PinePhone Pro / Librem 5 / Furi Phone first. Possibly partnership with a Chinese ODM for a Deamos Phone. **Far future, not in the first roadmap.**

---

## Forward-looking Bets (3-5 years)

- **NPU-first inference** — when 50+ TOPS NPUs are standard in laptops, we are first to actually use them well.
- **WebGPU agents** — browser as AI platform, we integrate deeper than anyone.
- **Agentic OS** — not an assistant, but a full agent with long-term memory, planning multi-step tasks. Today we are just approaching this (Claude Code as example). In 2-3 years this will be the expectation for any OS.
- **Multi-modal default** — voice + text + gestures + gaze (on camera-equipped devices) as first-class input.
- **Federated learning** — users can optionally improve models on their own data without exposing the data itself.

---

## Realistic Timeline

**Solo with AI, 6-12 months:**
- Fork of Universal Blue / Bazzite with customization
- Own brand, builds via GitHub Actions (bootc → OCI image)
- Preinstalled AI stack (Ollama, Whisper, ComfyUI, all via rpm-ostree layered packages)
- Basic Deamos Agent on top of existing open-source (Open WebUI + MCP)
- Website, documentation, Discord
- **1,000-5,000 first users** from r/LocalLLaMA, HackerNews, X community.

**Requires team and capital:**
- Own shell / DE (beyond Plasma config — native UI)
- Enterprise tier with MDM and SSO
- Hardware bundles
- Serious agent with long-term memory and learnable habits

**Honest financial horizon:**
- Year 1: $0 → MVP, community
- Year 2: $50k-200k revenue from Pro subscriptions
- Year 3: $500k-1M ARR if we hit enterprise pivot or hardware deal

Linux desktop will never be mass-market in the conventional sense. **But a niche $50M business is realistic** — System76 and Tuxedo live off this.
