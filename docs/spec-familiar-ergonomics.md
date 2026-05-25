# Spec — Familiar ergonomics (layout presets by prior OS)

**Principle:** a new user should *not* relearn the desktop. On first boot DeamOS
asks "which OS are you coming from?" and reshapes the desktop so their existing
muscle memory works — Start menu where Windows users reach, Dock and left window
buttons where Mac users reach. They use known skills in a familiar shell.

This is feasible because KDE Plasma is the most configurable desktop in
existence; we drive it with scripts, no third-party theme packs required for the
core layout.

**Long-term (Phase 5, ~3-5 years):** converge all of these toward a single,
opinionated DeamOS ergonomic. NOT now — for years 1-2 we *meet users where they
are*. The convergence is a far-future bet, tracked in the roadmap, not an alpha
goal.

---

## How it's applied

The OOBE "which OS" step writes the chosen preset and runs `apply-preset.sh`:

1. **Panels & widgets** — load a Plasma layout script via the shell scripting API:
   `qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript "$(cat layout-<preset>.js)"`.
   The script clears panels and recreates them (position, height, applets, order).
2. **Window decorations & behaviour** — `kwriteconfig6` into `kwinrc` /
   `kdeglobals`, then `qdbus org.kde.KWin /KWin reconfigure`.
3. Fully reversible: re-runnable from "DeamOS Welcome" or settings; switching
   preset re-applies cleanly.

Files: `recipes/modules/ergonomics/` → `layout-windows.js`, `layout-macos.js`,
`layout-gnome.js`, `apply-preset.sh` (classic KDE = no-op, keep Aurora default).

All applets used are stock Plasma (Kickoff, Icons-Only Task Manager, Global Menu,
System Tray, Digital Clock) — nothing third-party in the core layout.

---

## Presets

### Windows-style (default for most switchers)
- One **bottom** panel, full width (~44px).
- Bottom-left: **Application Launcher (Kickoff)** = the Start menu, with search.
- Task manager: **Icons-Only Task Manager**, left-aligned after the launcher.
- Right: **System Tray**, **Digital Clock**, Show-Desktop.
- Window buttons: **right** (`kwinrc [org.kde.kdecoration2] ButtonsOnRight=IAX`).
- `kdeglobals [KDE] SingleClick=false` (Windows opens on double-click).
- Net effect: Start bottom-left, taskbar with app icons, tray+clock bottom-right.

### macOS-style
- **Top** panel (~28px): left = **Global Menu** (app name + menu bar), right =
  System Tray + Clock. Spotlight ≈ KRunner on the usual shortcut.
- **Bottom Dock**: floating, centred, auto-hide — **Icons-Only Task Manager** +
  pinned launchers (rounded/padded floating panel = the Dock).
- Window buttons: **left** (`ButtonsOnLeft=XIA`).
- Optional: hot-corner top-left → "Overview/Present Windows" (Mission Control feel).

### GNOME / Ubuntu-style
- **Top** panel: left launcher ("Activities" feel), centre clock, right tray.
- **Left vertical dock** (dash-to-dock feel): Icons-Only Task Manager, left edge.
- Window buttons: **right**. Meta / hot-corner → Overview effect.

### Classic KDE
- No changes — stock Aurora/Plasma (bottom panel, Kickoff). For users already on
  Linux/KDE who want the native feel.

---

## What we deliberately do NOT do (in alpha)
- Pixel-perfect clones of Win11/macOS (icon-for-icon, animations). That is the
  Phase-5 convergence territory and a maintenance trap now. Aim: *"feels
  familiar in 5 seconds"*, not *"indistinguishable"*.
- Bundle third-party global-theme packs for the core layout (licensing +
  breakage on Plasma updates). Tasteful icon/color polish can come later and
  optional.

---

## Ties into
- OOBE picker (the "which OS did you come from?" step) — `docs/roadmap.md` Phase 1.
- `initial-setup` runs first (language + account), then this ergonomics step.
- A "DeamOS Welcome" app re-exposes the picker post-setup.
