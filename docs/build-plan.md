# Veil OS — Build & Coding Plan

The engineering companion to [`roadmap.md`](./roadmap.md). The roadmap answers
*what and when* in business terms; this document answers *how we actually build
it* — tools, repo mechanics, the order code gets written, and the definition of
"done" for each milestone.

---

## 0. Guiding principles

1. **We do not build a distro from scratch.** No Linux From Scratch, no custom
   package manager. We build a *bootc image* on top of Fedora Atomic. Our work
   is a `Containerfile` plus our own apps — not a kernel and 10,000 packages.
2. **Every milestone is a bootable, installable image.** We never spend a month
   on something we cannot boot. If it does not produce an `.iso` or an OCI image
   that `bootc switch` can pull, it is not a milestone.
3. **CI builds, not the laptop.** Images are built by GitHub Actions and pushed
   to `ghcr.io`. The dev machine only *tests* (in a VM). This keeps builds
   reproducible and frees us from "works on my machine".
4. **Two parallel tracks.** Track A = the OS image (integration work). Track B =
   Veil Agent + native apps (real software engineering). They meet at install
   time. A solo dev alternates; a team splits.
5. **Honesty about scope.** A polished AI-native OS is a multi-year effort. The
   plan front-loads a *usable, installable* system in ~6 months and treats the
   app suite as the long tail.

---

## 1. Developer environment

The maintainer works on **Windows 10**. bootc images must be built on Linux with
`podman`. Options, in order of recommendation:

| Option | Use for | Notes |
|---|---|---|
| **Fedora VM (Hyper-V / VirtualBox)** | local image builds + testing | 8 GB RAM, 60 GB disk min. Primary dev box. |
| **GitHub Actions** | the canonical builds | free tier is enough early; this is the source of truth. |
| **WSL2 (Fedora)** | quick `podman build` checks | no nested-virt for full ISO test, but fine for layer builds. |
| **Cheap cloud builder** ($10–20/mo Hetzner) | when CI minutes run out / big model layers | optional, later. |

Required tooling on the Linux dev box:

```
podman            # OCI image builds
bootc             # image-based OS management / local switch tests
bootc-image-builder  # OCI image -> bootable ISO / qcow2 / raw
just              # task runner (Makefile replacement, friendlier)
git, gh           # version control + GitHub CLI (already installed on host)
qemu/kvm + virt-manager  # boot and test images locally
```

VM testing on the Windows host itself: **Hyper-V** or **VirtualBox** to boot the
produced ISO. Enable nested virtualization if testing WayDroid/VMs inside Veil.

---

## 2. Repo mechanics

Current repo: `agent4bnk-spec/synapse-os` (private, codename — brand is `Veil OS`).

```
recipes/        # Containerfiles per edition + reusable modules   <- Track A
agent/          # Veil Agent daemon + apps                         <- Track B
iso/            # ISO build wrappers, kickstart/install configs
assets/         # branding (logo, plymouth, wallpapers)
docs/           # vision, roadmap, build-plan, compatibility
.github/workflows/   # CI: build images, build ISO, run tests
justfile        # entry point for every common task
```

Conventions:

- **Branching:** trunk-based. `main` is always buildable. Feature branches +
  PR even solo (CI must pass before merge).
- **Versioning:** image tags `vYYYY.MM.N` (date-based, like Fedora Atomic).
- **Registry:** images pushed to `ghcr.io/agent4bnk-spec/veil-<edition>`.
- **Signing:** images signed with `cosign` from Phase 1 onward (supply-chain
  trust is part of the privacy promise). Keys in GitHub Actions secrets.
- **No binaries in git:** models, ISOs, qcow2 stay out (already in `.gitignore`);
  they live in GitHub Releases / object storage.

---

## 3. The build pipeline

```
 recipes/<edition>/Containerfile
        │  (podman build, in GitHub Actions)
        ▼
 OCI image  ──push──►  ghcr.io/.../veil-standard:vYYYY.MM.N   (signed w/ cosign)
        │
        ├──► users update via:  bootc switch ghcr.io/.../veil-standard:latest
        │
        └──► bootc-image-builder
                   │
                   ▼
            installer ISO  +  qcow2  +  raw disk   ──►  GitHub Release
```

Two CI workflows:

1. **`build-image.yml`** — on push to `main` and on tags: `podman build` every
   edition, run smoke checks (image boots in a container sanity test, key
   binaries present), sign with cosign, push to GHCR. Fast (~10–20 min).
2. **`build-iso.yml`** — on tag only (releases are deliberate): pull the signed
   image, run `bootc-image-builder` to produce ISO + qcow2, attach checksums +
   signatures, publish a GitHub Release. Slow (~30–60 min).

A nightly **`vm-test.yml`** later: boot the qcow2 in QEMU headless, assert
first-boot wizard launches, Ollama answers, agent socket is up.

---

## 4. Track A — the OS image (integration)

Incremental milestones. Each ends in a bootable artifact.

### A-M0 — "It boots and it's ours" (week 1–2)
- Fork a known-good base. **Decision:** start from **Universal Blue / Bluefin
  or Bazzite** images (battle-tested bootc + hardware enablement) rather than
  raw Fedora Atomic — saves months of driver pain.
- Minimal `recipes/base/Containerfile`: `FROM` the base, change `/etc/os-release`
  to Veil OS branding, add our package overlay placeholder.
- Get `build-image.yml` green; produce first ISO via `build-iso.yml`.
- **Done when:** the ISO installs in a VM and shows "Veil OS" at login.

### A-M1 — Desktop identity (week 3–4)
- KDE Plasma 6, Wayland-only confirmed on base.
- Layout presets (Windows / macOS / power-user) shipped as selectable KDE
  layout scripts.
- Plymouth boot splash, default wallpaper, GTK/Qt theme, cursor/icon set from
  `assets/`.
- **Done when:** a freshly installed VM looks like Veil, not stock Fedora KDE.

### A-M2 — AI stack baked in (month 2)
- `recipes/modules/ai-stack`: Ollama (systemd service), preselected models
  pulled on first boot (not baked into the image — too big), `whisper.cpp`,
  `piper`, embeddings runtime, LanceDB.
- `recipes/modules/nvidia`, `/amd-rocm`, `/intel`: GPU driver layers, selected
  at build time per edition or detected at first boot.
- **Done when:** after install, `ollama run` works and uses the GPU.

### A-M3 — First-boot wizard (month 3)
- Hardware benchmark → model recommendation (which local LLM fits VRAM/RAM).
- Layout picker, privacy defaults (LUKS already set at install), telemetry =
  off and stated plainly.
- **Done when:** a new user goes from install to a working AI desktop without a
  terminal.

### A-M4 — Compatibility & privacy layers (month 4–5)
- `recipes/modules/compat`: Bottles + Proton/Steam, WayDroid preconfigured.
- `recipes/modules/privacy`: layered anonymity stack from `vision.md`
  (AmneziaWG, Tor + Snowflake, NymVPN client), kill switch, DNS-over-HTTPS.
- LUKS2 + duress password v1, TPM2 binding.
- **Done when:** a Windows `.exe`, an Android `.apk`, and Stealth-mode all work
  out of the box.

### A-M5 — Editions split (month 5–6)
- `lite` / `standard` / `studio` / `server` Containerfiles compose the modules.
- Alpha release candidate ISO for ~50–200 invited users.
- **Done when:** four signed editions build from shared modules in one CI run.

---

## 5. Track B — Veil Agent & apps (software engineering)

This is where the real product differentiation and the bulk of the code lives.

### B-M0 — Tech-stack decision (week 1)
- **Core daemon language: decide Rust vs Go.** Recommendation: **Rust** (single
  static binary, strong MCP/async ecosystem via `tokio`, memory safety matters
  for a system daemon with filesystem access). Go is the fallback if velocity
  beats everything.
- Pick UI toolkit: **Qt/QML (KDE-native)** for the omnibox + tray to feel
  first-class on Plasma.

### B-M1 — Agent v0.1 skeleton (month 2–3, parallel to A-M2/M3)
- MCP server core: process lifecycle, config, logging, a `health` endpoint.
- One MCP tool end-to-end: **filesystem read with user permission prompt**.
- Omnibox UI (global hotkey) → talks to local Ollama → answers.
- Ships *inside* A-M3 image.
- **Done when:** hit hotkey, ask "summarize this folder", get an answer from a
  local model, with a permission gate.

### B-M2 — Agent v0.5 (month 4–6)
- More MCP servers: terminal, active-window (AT-SPI), browser extension bridge.
- Local↔cloud routing logic (local default, opt-in cloud passthrough).
- Semantic file search pipeline: index → embeddings → LanceDB → query.
- **Done when:** agent can act across files, terminal, and the focused app, and
  find files by meaning.

### B-M3+ — Tier 1 native apps (Phase 2 territory)
- Veil Files, Vault, Capture, Backup, Restore Center, Browser. Each its own
  crate/module under `agent/` (or a sibling `apps/`), each shippable
  independently as a Flatpak so they update faster than the base image.

> **Packaging note:** native apps ship as **Flatpaks** layered on the image, not
> baked into the OCI base. This decouples app release cadence from OS releases —
> critical for a small team.

---

## 6. Testing strategy

| Layer | How |
|---|---|
| Image sanity | container smoke test in `build-image.yml` (files/services present) |
| Boot test | nightly QEMU headless boot of qcow2, assert services up |
| Agent unit/integration | Rust `cargo test` / Go `go test` in `agent/` CI |
| Manual QA matrix | NVIDIA + AMD + Intel laptop/VM before each release |
| Alpha feedback | invited users, issue templates, telemetry-free crash opt-in |

---

## 7. Release & trust

- Every image signed with **cosign**; users verify via documented public key.
- ISOs published as **GitHub Releases** with SHA-256 + signature files.
- Reproducible-ish builds: pin base image digests, lock module versions.
- Update path for users: `bootc switch` / built-in updater pulls signed image,
  atomic A/B with automatic rollback on boot failure (Btrfs + bootc give this
  for free).

---

## 8. First 30 days (concrete start)

A checklist to go from "repo with docs" to "an ISO I installed myself".

- [ ] Stand up a Fedora VM dev box (Hyper-V/VirtualBox) with podman + bootc + bootc-image-builder + just.
- [ ] Add `justfile` with `build`, `iso`, `test`, `lint` targets.
- [ ] Pick the base image (Bluefin/Bazzite vs raw Fedora Atomic) and record the decision in `recipes/base/README.md`.
- [ ] Write `recipes/base/Containerfile` — rebrand os-release, set repo metadata. (**A-M0**)
- [ ] Add `.github/workflows/build-image.yml` — build + push to GHCR on push to main.
- [ ] Add `.github/workflows/build-iso.yml` — build ISO on tag, publish Release.
- [ ] Cut tag `v2026.06.0`, let CI produce the first ISO.
- [ ] Install that ISO in a fresh VM. **Milestone reached when it boots to a "Veil OS" login.**
- [ ] In parallel: scaffold `agent/` — choose Rust, `cargo new`, a `health` endpoint, CI `cargo test`. (**B-M0/B-M1 start**)
- [ ] Open GitHub Issues for A-M1 and B-M1; switch sprint tracking to GitHub Projects.

---

## 9. Sequencing summary

```
Month 1   A-M0, A-M1   |  B-M0           -> branded bootable ISO
Month 2   A-M2         |  B-M1 start      -> AI stack on GPU
Month 3   A-M3         |  B-M1 done       -> first-boot wizard + agent v0.1
Month 4-5 A-M4         |  B-M2            -> compat + privacy + agent v0.5
Month 6   A-M5         |                  -> 4 editions, closed alpha
```

This is Phase 1 of the roadmap expressed as code milestones. Phases 2+ (Pro
infra, app suite, Hub mode, mobile) inherit this pipeline and add tracks.
