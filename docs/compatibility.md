# Veil OS — Compatibility Matrix

What runs on Veil OS, what doesn't, and what we plan to do about the gaps.

---

## Software compatibility

| Source OS | Technology | Coverage | Works well | Works poorly / not at all |
|---|---|---|---|---|
| **Linux** | Native (Flatpak, AppImage, RPM, Distrobox) | 100% | Everything in standard repos and Flatpaks | — |
| **Windows** | Wine + Bottles + Proton + DXVK | ~85% | MS Office (2016+ via Bottles), Photoshop CS6, AutoCAD, .NET apps, FL Studio, OBS, most Steam games | Adobe Creative Cloud (current), Valorant / Fortnite / EA Sports Connect (kernel anti-cheat), Photoshop 2024+ |
| **Android** | WayDroid | ~95% | TikTok, Instagram, WhatsApp, banking apps, most games | Pokémon GO (GPS-based), Genshin Impact (anti-cheat) |
| **DOS / Retro** | DOSBox, ScummVM, RetroArch | 100% | Doom, Quake, console emulators, classic adventure games | — |
| **Web apps** | Chromium / Firefox PWA + system integration | 100% | Notion, Figma, Slack, Discord, Google Docs, ChatGPT | DRM-protected 4K video (Netflix 4K, Apple TV+ 4K — Linux capped at 1080p) |
| **macOS** | Darling | ~5% | Some console / simple GUI apps | Final Cut, Logic Pro, most modern macOS apps |
| **iOS** | — | 0% | — | **Technically impossible.** Apple intentionally blocks. |
| **ChromeOS apps** | Chromium | 100% | Anything that's a Chrome PWA / extension | — |

---

## Hardware compatibility

### Day 1 support (works out of the box)

- **CPUs**: x86-64 (Intel / AMD all generations from Haswell onward), ARM64 (Snapdragon X Elite, Apple Silicon via Asahi fork — separate build)
- **GPUs**: NVIDIA (CUDA), AMD (ROCm), Intel Arc (IPEX)
- **NPUs**: Intel Lunar Lake / Arrow Lake (OpenVINO), AMD XDNA (Ryzen AI), Apple Neural Engine (via Asahi)
- **Wi-Fi**: Intel (all modern), Broadcom (recent), Realtek (with non-free firmware on first boot)
- **Bluetooth**: Intel, Broadcom, Realtek — full audio codec support (LDAC, aptX-HD, LC3) via PipeWire
- **Storage**: NVMe, SATA, USB-attached, eSATA
- **Printers**: CUPS + driverless IPP — HP, Brother, Canon, Epson out of the box
- **Scanners**: SANE
- **Graphics tablets**: Wacom, Huion, XP-Pen (preconfigured with Krita / GIMP)
- **Game controllers**: Xbox (xpadneo), DualSense, Steam Controller, generic HID
- **Fingerprint readers**: most fprintd-supported (Goodix, Synaptics, Validity)
- **Webcams**: UVC standard, Logitech, Razer
- **TPMs**: TPM 2.0 (Intel PTT, AMD fTPM, discrete chips)
- **Secure Boot**: yes, with custom signing keys

### Working with caveats

- **eGPU** (external GPU via USB4/Thunderbolt) — works for inference, requires explicit power-toggle on hot-plug
- **Fingerprint readers on newer ThinkPads** (Goodix newer chipsets) — partial, requires libfprint master
- **Hybrid graphics** (NVIDIA Optimus on laptops) — works via nvidia-prime, occasionally needs manual GPU offload
- **Touchscreens / styluses** on 2-in-1 — work, but per-device calibration may be needed
- **Modems / 5G** (Lenovo X1 cellular, Framework cellular kit) — works via ModemManager, GUI in development

### Not supported / problematic

- Wi-Fi cards requiring Windows-only drivers (some old Broadcom in pre-2018 Macs)
- Printers / scanners requiring vendor's proprietary blob (some Brother multifunction)
- Hardware DRM-locked drives (Self-Encrypting Drives with vendor firmware bugs — use LUKS instead)
- Anti-cheat in competitive shooters (kernel-level on Linux is non-starter for these games)

---

## Mobile / tablet compatibility

| Device class | Veil OS installable? | Veil Companion app? |
|---|---|---|
| Microsoft Surface (Pro / Go / Laptop) | **Yes, Day 1** | — |
| Framework Laptop 13 / 16 | **Yes, Day 1** | — |
| 2-in-1 / convertible (ThinkPad X1 Yoga, HP Spectre x360, Dell XPS 2-in-1) | **Yes, Day 1**, with touch + stylus | — |
| iPad | **No (impossible)** | **Yes** (App Store) |
| Android tablet (Samsung Tab, Lenovo) | Very hard, not in years 1-2 | **Yes** (Google Play) |
| PineTab 2 / PineTab-V | **Yes, Day 1** | — |
| PinePhone Pro / Librem 5 | Day 90+ after desktop stabilizes | — |
| Android phones (general) | Possible via postmarketOS approach, not in years 1-2 | **Yes** (Google Play) |
| iPhone | **No (impossible)** | **Yes** (App Store) |

---

## Strategy for gaps

| Gap | Strategy |
|---|---|
| Adobe Creative Cloud | Recommend GIMP / Krita / Inkscape / DaVinci. For users who must have Adobe — keep a Windows VM via libvirt + GPU passthrough as a documented workflow. |
| Competitive shooters with kernel anti-cheat | Not our market. Document the limitation honestly. |
| Netflix 4K / Apple TV+ 4K | DRM is the issue. Document, no fix possible from our side. |
| macOS-only apps (Final Cut, Logic Pro, etc.) | Recommend Linux alternatives (Kdenlive / DaVinci, Ardour / Reaper). Not our market. |
| iOS / iPad | Not our market. Offer **Veil Companion iOS app** for those who want some Veil functionality on Apple devices. |
| Random new Wi-Fi chips | Maintain a hardware compatibility wiki. Detect at install, warn before commit. |
