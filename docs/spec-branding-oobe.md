# Spec — Branding & OOBE overhaul (boot splash, wallpapers, themes, setup order)

Captures the boss's 2026-05-30 requirements after the first hardware install.
Source of truth for the next image cycle. Status: planned.

Touches boot/login/session → **must be verified on a real install** before we
call any item done. Batch all of it into ONE ISO rebuild (boot-splash + login +
wallpapers + themes + OOBE order) rather than many cycles.

---

## Requirements (verbatim intent)

1. **Language picked during installation.** The installer must present a
   language choice to the user.
2. **Ask "which OS did you come from?" ONCE, up front** — together with language,
   at the very start, not after the desktop is already up. After the user
   answers once, the system applies the matching settings and never asks again.
3. **Kill the Aurora "человечки" everywhere.** The cartoon-people art must not
   appear on the splash/desktop/login. Replace Aurora's default look-and-feel art
   with ours at image-build time so it is gone entirely, not just per-user.
4. **Plymouth boot splash = ours.** Background = our system art (waveform +
   logo). The progress indicator must NOT be Aurora's spinning circle — instead
   the **"D" logo** whose outline fills / changes colour saturation as it loads
   (same idea as the circular throbber, but shaped like the D).
5. **Desktop wallpapers are missing — add them.** (Specs below.)
6. **Themes: ship at least 4.** Confirm we can and do.

---

## 1 + 2 — OOBE order (language + "which OS" once, up front)

Current flow (split, feels late):
- Anaconda installer → `initial-setup.service` (lang/keyboard/tz/account on first
  boot, only if no user) → SDDM login → **first GUI login** autostart phase-2
  `deamos-welcome.sh` → kdialog "which OS?" → apply preset + wallpaper.

Target flow (one upfront setup, one flag):
- **Install time (item 1):** ensure Anaconda shows the language screen — do NOT
  pin `lang` in any kickstart; keep the interactive language/locale step. Verify
  on the ISO; if the bootc Anaconda flow skips it, add the welcome/language spoke
  back via the ISO `flatpak`/`anaconda` config.
- **First boot (item 2):** `initial-setup` already does language + account before
  login — keep it as the single language step. Fold the **"which OS?"** question
  into the *same first-run moment* and apply everything once:
  - Rename `deamos-welcome.*` → `deamos-setup.*`. It runs at first login (autostart
    phase 1, before the rest of the session settles), guarded by ONE flag
    `~/.local/state/deamos/setup-done`.
  - Steps in order: (a) confirm/echo the language chosen at install (no second
    locale prompt unless none set), (b) "which OS did you come from?" picker,
    (c) apply layout preset + wallpaper + theme, (d) write the flag.
  - Because the flag is set, it never asks again. The model-pull
    (`deamos-firstboot`) stays separate (background, its own flag).

DoD: a fresh install asks language (installer) + OS-origin (once, first login),
applies the matching desktop, and never re-prompts.

---

## 3 + 4 — Boot splash & killing the Aurora art

### Plymouth theme `deamos` (item 4)
- New `recipes/modules/plymouth/` → a **script-type** Plymouth theme installed to
  `/usr/share/plymouth/themes/deamos/`:
  - `deamos.plymouth` (Type=script, ModuleName=script),
  - `deamos.script` — draws the background image, then animates the **D mark**:
    two sprites (a dim D and a bright/saturated D); the bright one's alpha (or a
    vertical clip mask) ramps 0→1 on a loop to read as "the outline fills with
    colour". This replaces the spinning-circle throbber.
  - background = our boot art.
- Set as default: `plymouth-set-default-theme deamos` + **regenerate initramfs**
  (`rpm-ostree`/`dracut` in the Containerfile). Initramfs regen is the risky bit
  → smoke-test boot in a VM before shipping to hardware.

### Remove the "человечки" (item 3)
- The cartoon art is Aurora's default Global Theme / wallpaper. Make ours the
  build-time default for ALL surfaces:
  - **Desktop:** set DeamOS look-and-feel + wallpaper as the system default
    (kdeglobals `LookAndFeelPackage=org.deamos.dark`, default wallpaper → our
    package) via `/etc/skel` **and** the global Plasma defaults, not only per-user.
  - **Login (SDDM):** set the SDDM theme background to our login art
    (`/etc/sddm.conf.d/`), or ship a themed SDDM config.
  - **Lock screen:** `kscreenlockerrc` already points at our wallpaper — extend to
    the new art.
  - Belt-and-suspenders: physically overwrite the upstream default wallpaper file
    the Aurora global theme references, so even its theme renders our image.

---

## 5 — Wallpaper / image specs (what to hand over)

Provide PNG (lossless — crisp logo/text edges); cinematic photo-style variants
may be high-quality JPG. Colour space **sRGB**. Keep logo/text inside the central
~80% safe zone so other aspect ratios don't crop it.

### Desktop wallpapers (KPackage, multi-resolution)
Ship each wallpaper as a folder with several sizes; Plasma picks the best fit:
- **3840×2160** (4K, 16:9) — master, required.
- **2560×1440** (QHD) — recommended.
- **1920×1080** (FHD) — required (low-DPI / VM).
- Optional **3440×1440** (21:9 ultrawide) and a **1440×2560** portrait variant.

### Boot splash (Plymouth)
- Background: **1920×1080 PNG** required; **3840×2160 PNG** recommended for 4K
  panels. 16:9.
- **"D" mark** for the progress animation: **SVG preferred** (vector, transparent
  background), or **512×512 PNG transparent**. Two states help: a dim version and
  a bright/saturated version (or just the clean D — I can derive both).

### Login (SDDM) + lock screen
- **1920×1080** and **3840×2160 PNG**, 16:9, same art family as boot.

### Logo (already have `logo.svg`)
- Keep the master **SVG**. Also useful: **512×512** and **1024×1024** PNG
  (transparent) for icons/about/installer.

### File drop — names & location
Save the files here and I wire them into the repo:
`Демос/brand-assets/` (create it), with these names:

```
wallpaper-hero.png         # img 2 (cinematic "Product of free people")  3840x2160
wallpaper-logo.png         # img 1 (logo + Продукт свободных людей)       3840x2160
wallpaper-wave.png         # img 3/4 (logo + pulse wave, minimal)         3840x2160
boot-splash.png            # boot background (wave + logo)                3840x2160 (or 1920x1080)
login-bg.png               # SDDM/lock background                         3840x2160
logo-D.svg                 # the D mark alone, transparent  (or logo-D.png 512x512)
```
If you only have 1920×1080 of some, that's fine — give the largest you have.

---

## 6 — Themes (yes, we can; ship 4)

KDE theming layers: **color scheme** (`.colors`) → **Plasma style** → **window
decoration** → **Global Theme / Look-and-Feel** (bundles them + wallpaper +
splash). We ship 4 DeamOS **Look-and-Feel** packages, each = a color scheme +
default wallpaper + splash, selectable in System Settings → Global Theme.

Planned 4 (palette from the brand art — black-navy bg, electric-blue→cyan accent,
white text):
1. **DeamOS Dark** (default) — deep black-blue, blue accent, the logo wallpaper.
2. **DeamOS Light** — light surfaces, blue accent, for daytime.
3. **DeamOS Neon** — darkest bg, cyan accent, the cyberpunk "hero" wallpaper.
4. **DeamOS Graphite** — neutral graphite dark, muted blue, low-distraction, the
   wave wallpaper.

Step 1 (no images needed, done now): the **4 color schemes** →
`assets/color-schemes/DeamOS*.colors` → `/usr/share/color-schemes/`.
Step 2 (after wallpapers land): wrap each into a Look-and-Feel KPackage under
`assets/look-and-feel/org.deamos.*` → `/usr/share/plasma/look-and-feel/`, set
`org.deamos.dark` as the build-time default.

---

## Implementation order — status 2026-05-30

1. ✅ 4 color schemes → `/usr/share/color-schemes` (Dark/Light/Neon/Graphite).
2. ✅ OOBE: first-run setup at autostart **phase 1**, reframed, shows the set
   language, single flag. Item 1: no kickstart pins `lang` → Anaconda shows its
   language screen by default.
3. ✅ Wallpapers DeamosHero/Logo/Wave (KPackage). Desktop default = **DeamosWave**
   (dark D + waveform), set in skel + applied at first login → kills Aurora art.
4. ✅ Lock screen → DeamosWave; SDDM login → Breeze + `login-bg.png` (logo art).
5. ✅ Plymouth `deamos`: hero background + pulsing D, with LUKS password/question/
   message callbacks; installs `plymouth-plugin-script`, regenerates initramfs.
   ISO `iso-20260530-*-standard` built. **VM-test boot + LUKS unlock before HW.**
6. ⏭️ 4 Look-and-Feel (Global Theme) packages bundling scheme + wallpaper — polish
   follow-up; the 4 color schemes already satisfy "4 themes" for now.

> Images are only **1672×941** (below FHD) → boot/desktop will be soft on 1080p/4K.
> Regenerate at **3840×2160** for the real release.
