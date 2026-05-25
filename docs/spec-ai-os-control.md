# Spec — "AI operates the OS" (Deamos Agent v0.1)

The wedge feature: you tell DeamOS what you want in plain language, and it
**does it** — not a chatbot that explains how, an agent that performs the action.
This is the 15-second landing demo and the daily value of the OS.

Status: design. Builds on what exists today — `deamos-agentd` (Rust, tokio/axum),
the agent loop with tool-calling over local Ollama, the permission gate
(`Approver` → kdialog/zenity), and the omnibox (`Meta+A`).

---

## 1. Scope for v0.1 (alpha)

In: a small set of **typed capability tools** the agent can call, each backed by a
known system interface (D-Bus / a specific CLI), each gated by an approval with a
human-readable description. Out: arbitrary freeform shell as the primary path,
voice ("Hey Deamos" = Deamos Voice, Tier 2), multi-app workflows.

Design rule: **the LLM never runs raw commands to achieve these.** It calls a
named tool with typed args; Rust executes a vetted backend. `terminal_exec` stays
as an explicit, separately-gated escape hatch (off by default), not the mechanism.

---

## 2. Architecture

```
omnibox / chat  ──▶  deamos-agentd  ──▶  agent loop (Ollama, tool-calling)
                                            │
                                            ├─ advertises capability tools (ToolDef)
                                            ▼
                                   action dispatch (Rust)
                                            │  each action: typed args + risk + description
                                            ├─ gate(): Approver shows "DeamOS wants to … Allow?"
                                            ▼
                                   backend: zbus (KDE/NM D-Bus) | known CLI
                                   (plasma-apply-*, nmcli, flatpak, spectacle, wpctl, brightnessctl)
```

New in the agent: an `actions` module. Each action implements a small trait:

```rust
trait Action {
    fn name(&self) -> &str;                 // tool name advertised to the LLM
    fn schema(&self) -> serde_json::Value;  // JSON-schema args
    fn risk(&self) -> Risk;                  // Cosmetic | Connectivity | Install | System
    fn describe(&self, args: &Value) -> String; // human text for the approval prompt
    fn run(&self, args: &Value) -> Result<String, ActionError>; // execute backend
}
```

The agent loop already gates every tool call; we extend the gate to read
`risk()` + `describe()` so the prompt says exactly what will happen
("DeamOS wants to install **Telegram** (org.telegram.desktop). Allow?").

Backends prefer **D-Bus via `zbus`** where stable (Plasma look-and-feel, brightness,
media), else shell to a **specific** vetted binary (never an arbitrary string):
`nmcli`, `flatpak`, `spectacle`, `wpctl`/`pactl`, `brightnessctl`,
`plasma-apply-lookandfeel`, `plasma-apply-colorscheme`, `plasma-apply-wallpaperimage`.

The daemon runs in the **user session** (it already is a systemd *user* service),
so it has the session bus + KDE access. System actions (NM, flatpak) go through
polkit, which those CLIs handle.

---

## 3. Capability set for alpha

| Tool | Args | Backend | Risk |
|---|---|---|---|
| `set_color_scheme` | `mode: dark\|light` | `plasma-apply-lookandfeel` / colorscheme | Cosmetic |
| `set_wallpaper` | `name\|path` | `plasma-apply-wallpaperimage` | Cosmetic |
| `set_volume` | `percent: 0-100` or `mute: bool` | `wpctl`/`pactl` | Cosmetic |
| `set_brightness` | `percent: 0-100` | `brightnessctl` | Cosmetic |
| `wifi_list` | — | `nmcli -t dev wifi` | Cosmetic (read) |
| `wifi_connect` | `ssid, password?` | `nmcli dev wifi connect` | Connectivity |
| `app_search` | `query` | `flatpak search` | Cosmetic (read) |
| `app_install` | `flatpak_id` | `flatpak install -y flathub <id>` | Install |
| `app_launch` | `app_id` | `gtk-launch`/`kstart` | Cosmetic |
| `open_settings` | `module` (e.g. `kcm_bluetooth`) | `systemsettings`/`kcmshell6` | Cosmetic |
| `screenshot_explain` | `region: full\|window` | `spectacle -b` → OCR (tesseract) → LLM | System (read) |
| `system_info` | `topic: battery\|disk\|updates` | D-Bus / `df` / `rpm-ostree status` | Cosmetic (read) |

Name→app resolution (e.g. "Telegram" → `org.telegram.desktop`) lives in a small
curated alias map first; `app_search` covers the long tail. Keeps the common demo
apps reliable.

---

## 4. Permission tiers

The approver maps `Risk` → behaviour (configurable in `agent.toml`):

- **Cosmetic / read** (theme, wallpaper, volume, brightness, lists, screenshot,
  info): default **auto-allow** (instant, reversible, no data leaves device).
- **Connectivity** (wifi_connect): **confirm once**, remember per-network.
- **Install** (app_install): **always confirm**, show app id + source (Flathub).
- **System / terminal_exec**: **always confirm**; `terminal_exec` also requires
  `enable_terminal=true`.

The confirmation dialog is the human safety net for LLM mistakes — it always
shows the resolved, concrete action, never the raw model output.

---

## 5. Landing demo script (the phrases we guarantee work)

1. "Сделай тёмную тему" → `set_color_scheme(dark)`
2. "Подключись к Wi-Fi HomeNet, пароль 12345" → `wifi_connect(HomeNet, 12345)`
3. "Поставь Telegram" → `app_install(org.telegram.desktop)` (confirm)
4. "Сделай скриншот и объясни, что за ошибка" → `screenshot_explain(full)`
5. "Поставь громкость на 30%" → `set_volume(30)`
6. "Прибавь яркость" → `set_brightness(+10)`
7. "Поменяй обои на тёмные" → `set_wallpaper(Deamos)`
8. "Открой настройки Bluetooth" → `open_settings(kcm_bluetooth)`

These 8 are the acceptance test for the demo and the recorded landing GIFs.

---

## 6. Build milestones

- **C-M1 — action framework + cosmetic set.** `actions` trait + dispatch + gate
  reading risk/describe. Wire `set_color_scheme`, `set_wallpaper`, `set_volume`,
  `set_brightness`. Zero-risk, instant visible demo. (zbus or vetted CLIs.)
- **C-M2 — connectivity + apps.** `wifi_list`/`wifi_connect`, `app_search`/
  `app_install` + alias map. Strong confirmation UX.
- **C-M3 — showpiece.** `screenshot_explain` (spectacle + tesseract + LLM).
  Plus `open_settings`, `system_info`, `app_launch`.
- **C-M4 — assistant UX.** Omnibox → streaming answers, action chips ("Installing
  Telegram… ✓"), tray indicator. (Replaces the kdialog stopgap.)

Voice ("Hey Deamos", Whisper.cpp + Piper) is a fast-follow after C-M4, tracked as
Deamos Voice (Tier 2).

---

## 7. Reliability & safety

- Few, well-named tools + tight JSON schemas + a system prompt with examples →
  the model picks the right tool far more reliably than with many fuzzy tools.
- Every state change is confirmed with the concrete resolved action; the user is
  the backstop. `MAX_STEPS` caps runaway loops (already enforced).
- All backends are specific vetted binaries / D-Bus calls — never
  model-authored shell. `terminal_exec` stays the explicit, off-by-default
  escape hatch.
- Hybrid AI (local model or bring-your-own cloud key) decides *which* brain runs
  the loop; the action layer is identical either way.
