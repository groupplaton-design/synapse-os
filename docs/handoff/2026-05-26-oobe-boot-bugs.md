# Session handoff — DeamOS OOBE & boot-test bugs

> **Purpose:** continue this DeamOS work from any device. A fresh Claude Code /
> Claude session can read this file in the repo, regain full context, and pick up
> exactly where we stopped. Last updated **2026-05-29**.

## TL;DR — where we are

DeamOS (AI-native Linux on Universal Blue **Aurora** → KDE Plasma 6 / Fedora
Atomic / bootc) now has:

- Full **Veil → DeamOS** rebrand (os-release, hostname, /etc/issue, GRUB
  distributor). Spelling is **DeamOS** (D-E-A-M, away from "daemon"; leans on
  "demos" = people; slogan *"a product of free people"*).
- A no-lockout **OOBE**: `initial-setup` (language + account on first boot),
  a first-login **welcome** kdialog that asks "which OS did you come from?"
  and applies a familiar **layout preset** (Windows/macOS/GNOME/KDE).
- ISO delivery via GitHub **Release assets**, split into <2 GiB parts
  (`build-iso.yml`) — sidesteps the Actions artifact-storage quota wall.
- Agent (`deamos-agentd`, Rust) + AI stack (Ollama) baked into `standard`.

The ISO **builds, boots, and installs**. Verified: download → reassemble →
SHA256 matched (`caf2a59e…a429c1`), GPT + 3 partitions, `bootc` writes correctly.

## ✅ TWO BUGS — FIXED IN CODE 2026-05-29, awaiting rebuild + on-device verify

> **Status:** both high-confidence fixes implemented and pushed to `main`
> (commit `dd4a0c9`). `build-image` ✅ green, `build-iso` ✅ green — ISO published as
> Release **`iso-20260529-1810-standard`** (sha256 of rejoined install.iso:
> `f5b021b3c23590df21191291220cb2bc63708b49bffdbade9327a9d81893f522`).
> **Not yet verified on hardware** — flash DD-mode, boot, confirm wallpaper is
> DeamOS (not Aurora) and that picking "Windows" actually rebuilds the panels.
>
> - **Wallpaper fix** (`recipes/modules/welcome/deamos-welcome.sh`): after the
>   picker, run `plasma-apply-wallpaperimage /usr/share/wallpapers/Deamos/`
>   per-user at first login (ID-independent), with the explicit
>   `…/contents/images/1920x1080.svg` as a fallback. Skel kept as last resort.
> - **Layout fix** (`recipes/modules/ergonomics/apply-preset.sh`): new
>   `plasma_dbus()` helper routes Plasma/KWin calls through `gdbus → qdbus6 →
>   qdbus` with fully-qualified method names, replacing the `qdbus`-only calls
>   that silently no-op'd on Plasma 6.
>
> If verification fails, run the diagnostics below on the installed box and
> adjust. Original bug report kept for context:

### Original report (confirmed on real install 2026-05-26)

User installed DeamOS on a PC and reported:

1. **Wallpaper didn't apply** — booted to the Aurora cartoon ("человечки"),
   not our DeamOS wallpaper. Root-cause hypothesis: the skel
   `plasma-org.kde.plasma.desktop-appletsrc` targets containment **ID 1**, which
   likely doesn't match the real desktop containment Plasma creates → our
   `Image=/usr/share/wallpapers/Deamos/` is ignored; Aurora's look-and-feel wins.
2. **Layout preset did nothing** — the "which OS?" picker *appeared* (so
   autostart + kdialog work), but choosing **Windows** changed no panels.
   Root-cause hypothesis: `apply-preset.sh` calls **`qdbus`**, which is likely
   **absent on Plasma 6** (it ships `qdbus6` / one uses `gdbus`) →
   `evaluateScript` never runs, silently fails.

### Planned fixes (high-confidence, do these unless diagnostics say otherwise)

- **Wallpaper:** stop relying on the skel containment-ID hack. In
  `recipes/modules/welcome/deamos-welcome.sh`, after the picker, run
  `plasma-apply-wallpaperimage /usr/share/wallpapers/Deamos/` (per-user, at first
  login — robust, ID-independent). Keep skel as a fallback.
- **Layout preset:** rewrite `recipes/modules/ergonomics/apply-preset.sh` to use
  `gdbus call --session --dest org.kde.plasmashell --object-path /PlasmaShell
  --method org.kde.PlasmaShell.evaluateScript "<js>"` with a `qdbus6` fallback,
  instead of `qdbus`. Same for the final `org.kde.KWin /KWin reconfigure` call.

### Diagnostic to confirm before the blind rebuild

On the installed DeamOS, open Konsole and run:

```sh
plasma-apply-wallpaperimage /usr/share/wallpapers/Deamos/      # does wallpaper change live?
which gdbus qdbus qdbus6 qdbus-qt6 plasma-apply-wallpaperimage kwriteconfig6
/usr/libexec/deamos/apply-preset.sh windows                    # does layout change? any error?
```

The exact present/absent binary names decide the final form. If the user just
says "продолжай" without running these, implement the high-confidence fixes
above (gdbus + plasma-apply-wallpaperimage) and rebuild.

## How to rebuild & redeliver after fixing

1. Push fixes to `main`.
2. GitHub Actions → run **build-image** (so `ghcr.io/.../deamos-standard:latest`
   refreshes), then **build-iso** (`workflow_dispatch`, edition `standard`).
3. ISO is published as a **Release** tagged `iso-YYYYMMDD-HHMM-standard`,
   split into `deamos-install.iso.part*` + `SHA256SUMS` + `REASSEMBLE.txt`.
4. Download all parts, rejoin (see REASSEMBLE.txt), verify SHA256, flash **DD/raw**
   (Rufus DD-mode) — **never** ISO/file mode (FAT32 4 GB limit truncates the
   embedded >4 GB container → install hangs at `/run/install/repo/container`).

## Hard-won facts (don't relitigate)

- **Flash DD-mode only.** ISO/file mode breaks bootc Anaconda ISOs.
- **Counterfeit USBs** (fake 976 GB) silently truncate; use a genuine stick
  (Transcend 16 GB worked).
- **Lockout cause:** Anaconda finished with no user/root account; `initial-setup`
  now prevents this. composefs `/sysroot` is read-only — rd.break recovery can't
  remount rw, so don't go down that road; reinstall instead.
- **Artifact quota** is exhausted by a single >2 GiB ISO and recalculates only
  every 6-12 h; **Release assets don't count** against it → we split + release.
- **Build on top of Aurora** via Containerfile; do NOT rebuild Fedora.
- **PowerShell gotchas:** `git commit -m "..."` mangles quotes → use
  `git commit -F file`. `jq` splits on spaces → parse JSON with
  `ConvertFrom-Json`. `gh` needs `-R agent4bnk-spec/synapse-os` outside the repo.

## Key files

| File | Role |
|---|---|
| `recipes/modules/rebrand.sh` | os-release/hostname/issue/GRUB → DeamOS |
| `recipes/modules/initial-setup/install.sh` | enables first-boot account/lang wizard |
| `recipes/modules/welcome/deamos-welcome.sh` | first-login OS picker → apply-preset **(add wallpaper apply here)** |
| `recipes/modules/welcome/deamos-welcome.desktop` | autostart phase 2 |
| `recipes/modules/ergonomics/apply-preset.sh` | applies layout **(switch qdbus → gdbus)** |
| `recipes/modules/ergonomics/layout-{windows,macos,gnome}.js` | Plasma panel scripts |
| `recipes/base/skel/plasma-org.kde.plasma.desktop-appletsrc` | skel wallpaper (containment-ID hack, unreliable) |
| `recipes/base/skel/kscreenlockerrc` | lockscreen wallpaper |
| `assets/wallpapers/Deamos/contents/images/1920x1080.svg` | DeamOS wallpaper |
| `recipes/base/Containerfile`, `recipes/standard-nvidia/Containerfile` | image builds |
| `.github/workflows/build-iso.yml` | split-release ISO delivery |
| `docs/spec-ai-os-control.md` | "AI operates the OS" wedge (C-M1…C-M4) |
| `docs/spec-familiar-ergonomics.md` | layout presets + Phase-5 convergence |
| `docs/roadmap.md` | launch differentiators, cloud install, phases |

## Project constraints (persist verbatim)

- **Work without asking permission** in synapse-os, EXCEPT: deleting data, and
  public distribution to public access. Private-repo Releases = internal = OK.
- Solo Russian dev, ~$10k capital, criminal record expiring ~2035 (legal
  structure: Georgia + foundation). Goal: $1M.

## Next, after the two bugs

- Validate Plymouth boot-splash + SDDM login theming (still upstream Aurora).
- Fine-tune layout presets against real Plasma 6 behaviour.
- Build **C-M1**: AI-operates-the-OS (typed capability tools — set_wallpaper,
  set_color_scheme, set_volume, set_brightness first). **Implementation plan
  ready:** [`docs/plan-c-m1-action-framework.md`](../plan-c-m1-action-framework.md)
  (new `src/actions/` module + risk-aware gate, grounded in the current agent).
- Cloud install (restore your saved OS image) — see roadmap differentiators.
- Housekeeping: delete orphaned `veil-*` GHCR packages (needs
  `gh auth refresh -s delete:packages`).
