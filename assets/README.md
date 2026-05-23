# assets/

Branding and visual assets.

## Shipped (A-M1, wired in `recipes/base/Containerfile`)

- `logo.svg` — Veil OS mark → image `/usr/share/pixmaps/veil-logo.svg`.
- `wallpapers/Veil/` — KDE wallpaper package (KPackage `metadata.json` +
  `contents/images/1920x1080.svg`) → `/usr/share/wallpapers/Veil/`. Appears in
  the wallpaper picker and is set as the default for new users via the
  `/etc/skel` Plasma config.

All of the above is **additive** — copies and a skel config drop-in — so it
cannot break the build or boot.

## Needs a boot test before wiring (A-M1 remainder)

These touch boot / session and must be verified on real hardware or a VM:

- Plymouth boot-splash theme (initramfs regeneration involved).
- SDDM login-screen theme.
- KDE layout presets (Windows / macOS / power-user) as selectable Plasma
  layout scripts.

## Later

- PNG renders of the logo at various sizes, color palette spec, typography,
  marketing assets. Brand polish continues alongside the website (Phase 0).
