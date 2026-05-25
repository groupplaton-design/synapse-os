# DeamOS — Roadmap

High-level phases. Detailed sprint planning will live in GitHub Issues / Projects once the repo is operational.

---

## Progress snapshot — 2026-05

Most of **Phase 1's build pipeline is done** and green in CI. What exists:

- **Image pipeline**: `deamos-base`, `deamos-standard`, `deamos-standard-nvidia`
  build on GitHub Actions → GHCR; installer ISO via `bootc-image-builder`
  (base ~4.7 GiB, standard ~6.7 GiB, artifact). See `docs/build-plan.md`.
- **Base**: Universal Blue Aurora (KDE/Fedora Atomic); NVIDIA via the
  open-kernel-module `aurora-nvidia-open`.
- **AI stack in the OS**: Ollama as a loopback systemd service.
- **Deamos Agent (`deamos-agentd`)**: Rust daemon baked into the image (systemd
  user service) — `/health`, `/version`, `/chat`, `/agent`, `/tools/fs/read`;
  tools `fs_read` + `terminal_exec`; agent loop with permission gating.
- **Permissions**: interactive approver (kdialog/zenity) — tool use prompts
  the user; locked-down defaults (no fs roots, terminal off, approval on).
- **Omnibox**: kdialog prompt → `/agent`, launcher + Meta+A shortcut.
- **First-boot (A-M3)**: hardware detection → model recommendation →
  auto-pull a model sized to the machine.
- **Branding**: SVG logo + wallpaper package + default wallpaper.

Not yet (needs a boot test or later phase): GUI first-boot wizard, plymouth/
SDDM themes, KDE layout presets, LUKS duress features, WayDroid/Bottles
wiring, public ISO hosting, docs site, closed alpha.

---

## Phase 0 — Foundation (Month 0-1)

**Goal:** legal, branding, and decision-making infrastructure in place before any code is written.

- [ ] Trademark verification for "DeamOS" in USPTO, EUIPO, WIPO Madrid Protocol
- [ ] Domain acquisition (deamosos.com / deamosos.io / deamos-os.org — whichever is available)
- [ ] Legal entity selection and registration (Armenia / Estonia / UAE / Delaware — see legal.md when written)
- [ ] Bank account + Stripe / Wise setup on the legal entity
- [ ] Brand assets: logo, color palette, typography
- [ ] Public landing page (one-pager) — `coming soon` waitlist
- [ ] Privacy Policy + ToS first drafts
- [ ] Final repo rename `synapse-os` → `deamos-os` once brand is confirmed legal

---

## Phase 1 — MVP build (Month 2-6)

**Goal:** working ISO that boots, runs Deamos Agent v0.1, ships to a closed alpha of ~50-200 users.

- [x] Fork Universal Blue as base (Aurora KDE)
- [x] Containerfile recipes in `recipes/` — base + standard editions
- [x] GitHub Actions pipeline: Containerfile → OCI image → installer ISO
- [~] First-boot setup — hardware detection + model recommendation + auto-pull done; GUI layout picker pending
- [x] Deamos Agent v0.1 — Rust daemon, omnibox, local Ollama, agent loop with tools
- [x] Deamos Models v0.1 — hardware-aware model recommendation + first-boot pull
- [ ] LUKS2 default with anti-coercion features (duress password v1)
- [~] Driver bundle: NVIDIA (open) + AMD + Intel via Aurora base; NPU pending
- [ ] Compatibility layer: Bottles + Steam/Proton + WayDroid preinstalled
- [ ] Documentation site (`docs.deamosos.com`)
- [ ] Discord / Matrix community
- [ ] Closed alpha invite via waitlist

---

## Phase 2 — Public beta (Month 7-12)

**Goal:** open public beta, monetization live, 1,000-5,000 active users.

- [ ] Open the repo to the public (move from private to public, MIT/Apache dual-license review)
- [ ] Deamos Pro subscription infrastructure — anonymous payment via Lightning, Monero, USDT-TRC20, plus traditional Stripe
- [ ] Deamos Cloud Backup service (Pro tier)
- [ ] Tier 1 native products: Deamos Files, Deamos Browser, Deamos Capture, Deamos Vault, Deamos Backup, Deamos Restore Center
- [ ] Deamos Agent v1.0 — long-term memory, semantic file search across the whole system
- [ ] Hub mode v1 — server profile, distributed local inference across LAN
- [ ] Deamos Companion mobile apps (iOS + Android) v1.0
- [ ] PPP pricing engine in Stripe / payment provider
- [ ] Lifetime deal launch (500 × $199)
- [ ] HackerNews + r/LocalLLaMA + ProductHunt launch
- [ ] First localization pass: EN (default), RU, DE, ES, JP

---

## Phase 3 — Growth (Year 2)

**Goal:** $50k-200k ARR, 10,000+ active users, first enterprise customers.

- [ ] Tier 2 native products: Deamos Office, Deamos Mail, Deamos Notes, Deamos Code, Deamos Voice
- [ ] Tier 3 products: Deamos Hub Manager, Deamos Sync, Deamos Wallet Hub, Deamos Photos, Deamos Smart Home
- [ ] Deamos Enterprise tier: MDM, SSO, audit logs, on-premise model hub
- [ ] First hardware partner pilot — preinstalled DeamOS laptops via ODM (Tongfang / Clevo / BIWIN)
- [ ] Censorship circumvention bundle (Shadowsocks / V2Ray / Xray clients + AmneziaVPN deep integration)
- [ ] Tor integration v2 (Snowflake bridge opt-in, optional Tor middle relay in Hub mode)
- [ ] Smart Home edition + voice satellite (Wyoming protocol)
- [ ] First conference talk / sponsorship — All Things Open, FOSDEM, Linux Foundation events

---

## Phase 4 — Scale (Year 3)

**Goal:** $500k-1M ARR, sustainable team, established niche brand.

- [ ] Deamos Studio (Pro creator tier) — AI pipelines, ComfyUI workflows, video AI stack
- [ ] Federated AI marketplace — users sell GPU time on their Hub machines to other users
- [ ] Enterprise expansion: 3-5 reference customers in legal / medical / defense
- [ ] Hardware bundle program — own-branded Deamos laptops at scale
- [ ] Open-source community governance — CLA, technical steering committee
- [ ] Localization expansion: FR, ZH-CN, PT-BR, KO, HI, AR
- [ ] First investor conversation (only if needed — bootstrap is the default)

---

## Phase 5 — Long-term bets (Year 4-5)

- **Deamos Mobile OS** — fork of postmarketOS / Mobile NixOS for PinePhone Pro, Librem 5, possibly own ODM phone
- **NPU-first inference engine** — when 50+ TOPS NPUs are standard, Deamos leads the integration
- **WebGPU agents** — browser-as-AI-platform integration
- **Multi-modal default interaction** — voice + gaze + gesture on camera-equipped devices
- **Confidential computing** — Intel TDX / AMD SEV-SNP for truly private cloud inference

---

## Anti-goals

What we intentionally do NOT pursue:

- Pleasing Adobe / Final Cut / Logic users
- Pleasing competitive-shooter gamers with kernel anti-cheat
- Building our own crypto wallet, DEX, token, or NFT marketplace
- Building our own video conferencing or music streaming
- Becoming a generic "Linux for everyone" distribution
- Massive growth at the cost of privacy promises
