# Plan — C-M1: action framework + cosmetic capability set

Implementation plan for the first "AI operates the OS" milestone. Companion to
the design in [`spec-ai-os-control.md`](./spec-ai-os-control.md); this document
maps that design onto the **code that exists today** in `agent/deamos-agentd/`
and defines the exact files, types, and tests to land.

Status: planned (2026-05-29). Not yet implemented.

---

## 0. Goal & definition of done

Ship a typed **action framework** plus four **cosmetic** actions the LLM can
call to change the running desktop. Zero-risk, instantly visible, fully
reversible — the first real "tell it, it does it" demo.

**Done when:**
- `deamos-agentd` advertises `set_color_scheme`, `set_wallpaper`, `set_volume`,
  `set_brightness` as tools alongside `fs_read`.
- On a real Plasma 6 session these phrases perform the action:
  *«сделай тёмную тему» / «поставь громкость на 30%» / «прибавь яркость» /
  «смени обои на тёмные»*.
- Every backend is a **specific vetted binary** — never `sh -c`, never
  model-authored shell.
- Cosmetic actions either auto-allow (default) or show a concrete
  `describe()`-based approval prompt; the gate path is unchanged for `fs_read`.
- `cargo test` + `fmt` + `clippy` green (the existing `agent-ci.yml` gates).

**Out of scope (later milestones):** `wifi_connect`/`app_install` (C-M2),
`screenshot_explain` (C-M3), omnibox streaming UX + action chips (C-M4). The
`Connectivity`/`Install`/`System` risk tiers are *defined* here but only
`Cosmetic` actions ship.

---

## 1. What exists today (grounding)

- **`agent.rs`** — `Agent { backend, model, fs_policy, approver, enable_terminal }`.
  - `tools(&self) -> Vec<ToolDef>` hard-codes `fs_read` (+ optional `terminal_exec`).
  - `execute(&call)` matches on `call.function.name` → `exec_fs_read` / `exec_terminal`.
  - `gate(tool, action, resource) -> Option<String>` builds a `PermissionRequest`
    and consults the `Approver`.
- **`permissions.rs`** — `PermissionRequest { tool, action, resource }`,
  `Decision::{Allow, Deny(String)}`, `Approver::decide`. `InteractiveApprover`
  prompts via `kdialog`/`zenity`: *"Deamos Agent wants to {action} {resource}."*
- **`tools.rs`** — free functions returning typed structs + `ToolError`.
- **`llm.rs`** — `ToolDef { kind, function: FunctionDef { name, description,
  parameters } }`, `ToolCall { function: FunctionCall { name, arguments: Value } }`.
- **`config.rs`** — `Config` (`#[serde(default)]`); `require_approval`,
  `enable_terminal`, etc.
- **`server.rs`** — `AppState` carries `fs_policy/approver/chat/...`; routes
  `/agent`, `/chat`, `/tools/fs/read`.

The action framework extends this shape; it does not replace it.

---

## 2. New module: `src/actions/`

Small, well-named files (one domain each):

```
src/actions/
  mod.rs         # Risk, ActionError, Action trait, Registry, vetted run_cmd()
  appearance.rs  # SetColorScheme, SetWallpaper
  audio.rs       # SetVolume
  display.rs     # SetBrightness
```

### 2.1 Core types (`actions/mod.rs`)

```rust
pub enum Risk { Cosmetic, Connectivity, Install, System }

pub enum ActionError {
    BadArgs(String),     // schema/range violation -> fed back to the model
    Backend(String),     // the vetted CLI failed
    Unsupported(String), // required binary absent on this host
}

/// One capability the agent can invoke. Backed by a vetted CLI / D-Bus call.
pub trait Action: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;        // advertised to the LLM
    fn schema(&self) -> serde_json::Value;        // JSON-schema for arguments
    fn risk(&self) -> Risk;
    fn describe(&self, args: &serde_json::Value) -> String; // human approval text
    /// Build the exact argv WITHOUT running it. Pure -> unit-testable.
    fn command(&self, args: &serde_json::Value) -> Result<Vec<String>, ActionError>;
    /// Default impl runs `command()` via the vetted runner; actions rarely override.
    fn run(&self, args: &serde_json::Value) -> Result<String, ActionError> {
        let argv = self.command(args)?;
        run_cmd(&argv)
    }
}
```

Key safety property: **`command()` returns a `Vec<String>` argv** that
`run_cmd` executes with `std::process::Command::new(argv[0]).args(&argv[1..])`
— no shell, no string interpolation. Splitting `command()` (pure) from `run()`
(effectful) is what makes the backends unit-testable without touching the host.

```rust
/// Run a vetted command (argv[0] = binary). Never a shell. Caps output.
fn run_cmd(argv: &[String]) -> Result<String, ActionError> { /* Command + which check */ }
```

### 2.2 Registry (`actions/mod.rs`)

```rust
pub struct Registry { actions: Vec<Box<dyn Action>> }

impl Registry {
    pub fn cosmetic() -> Self { /* the 4 C-M1 actions */ }
    pub fn get(&self, name: &str) -> Option<&dyn Action>;
    pub fn tool_defs(&self) -> Vec<ToolDef>;   // name/description/schema -> ToolDef
}
```

`tool_defs()` adapts each `Action` into the existing `llm::ToolDef`, so the loop
in `agent.rs` advertises actions exactly like `fs_read`.

### 2.3 The four cosmetic actions

| Action | Args | Vetted backend (argv) |
|---|---|---|
| `set_color_scheme` | `mode: "dark"\|"light"` | `plasma-apply-lookandfeel -a org.kde.breezedark.desktop` / `org.kde.breeze.desktop` |
| `set_wallpaper` | `name` or `path` | `plasma-apply-wallpaperimage <resolved-path>` (alias `deamos`→`/usr/share/wallpapers/Deamos/`) |
| `set_volume` | `percent: 0-100` \| `mute: bool` | `wpctl set-volume @DEFAULT_AUDIO_SINK@ <p>%` / `wpctl set-mute … 1` (fallback `pactl`) |
| `set_brightness` | `percent: 0-100` (or `+N`/`-N`) | `brightnessctl set <p>%` |

All `Risk::Cosmetic`. Name→path/theme resolution lives in a small curated map
(reuse for `app_install` aliases in C-M2). Unknown alias → `BadArgs`, fed back
to the model so it can correct.

---

## 3. Permission gate changes (`permissions.rs`)

Extend the request so the prompt can show the concrete, resolved action and the
approver can be risk-aware:

```rust
pub struct PermissionRequest {
    pub tool: String,
    pub action: String,
    pub resource: String,
    pub description: Option<String>, // from Action::describe(); preferred in the prompt
    pub risk: Risk,                  // defaults to System for legacy callers
}
```

- `InteractiveApprover`: if `description` is `Some`, prompt with it verbatim
  ("DeamOS wants to switch to the **dark** theme. Allow?"), else fall back to the
  current "{action} {resource}" text. **`fs_read` behaviour is unchanged.**
- **Risk → behaviour** (per spec §4), enforced in `agent.rs` before calling the
  approver, configurable via `config`:
  - `Cosmetic` → auto-allow when `auto_allow_cosmetic = true` (default), else confirm.
  - `Connectivity` → confirm once (C-M2).
  - `Install` / `System` → always confirm.

`Risk` lives in `permissions.rs` (or a tiny shared `risk.rs`) and is re-exported
from `actions` to avoid a cycle.

---

## 4. Agent wiring (`agent.rs`)

- Add `actions: Arc<Registry>` to `Agent`; thread it through `Agent::new`.
- `tools()` → append `self.actions.tool_defs()` to the advertised list.
- `execute()` → after the `fs_read`/`terminal_exec` arms, fall through to:
  ```rust
  name => match self.actions.get(name) {
      Some(action) => self.exec_action(action, call),
      None => /* existing unknown-tool error */,
  }
  ```
- `exec_action(action, call)`:
  1. `let args = &call.function.arguments;`
  2. risk policy: if `Cosmetic` && `auto_allow_cosmetic` → skip prompt; else
     `gate_action(action.name(), action.risk(), &action.describe(args))`.
  3. `action.run(args)` → map `Ok`/`Err(ActionError)` to `(feedback, ToolStep)`
     reusing the existing `tool_ok/tool_denied/tool_error` helpers.
- **System prompt** (the `Message::system(...)` in `run`): add a short line that
  the agent can change appearance/volume/brightness via the typed tools, must
  use them (never describe shell), and 2-3 few-shot examples from the spec's
  landing script. Keep it tight — few, well-named tools + crisp schema is what
  makes a small local model pick the right tool reliably (spec §7).

---

## 5. Config (`config.rs`) & wiring (`server.rs`)

Add (all `#[serde(default)]`, documented in `agent.example.toml`):

```rust
pub auto_allow_cosmetic: bool, // default true  — cosmetic actions don't prompt
pub enable_actions: bool,      // default true  — master switch for the action set
```

`server.rs`: build `Arc::new(Registry::cosmetic())` into `AppState`, pass to
`Agent`. The local API stays loopback-only; no new routes required for C-M1
(actions are reached through `/agent`). Optionally add `/actions` (GET) to list
advertised capabilities for debugging.

Mirror the runtime config in `recipes/modules/deamos-agent/agent.toml` so the
shipped image enables the action set.

---

## 6. Tests (guardrails — `cargo test`)

Pure, host-independent (assert argv, never execute the desktop):

- `command()` builders: `set_color_scheme("dark")` → `["plasma-apply-lookandfeel","-a","org.kde.breezedark.desktop"]`; light variant; `set_volume(30)` → `["wpctl","set-volume","@DEFAULT_AUDIO_SINK@","30%"]`; `set_brightness(+10)`; `set_wallpaper("deamos")` → resolved path.
- Arg validation: out-of-range percent / unknown mode / unknown alias → `BadArgs`.
- `Registry`: `tool_defs().len() == 4`, names unique, `get()` round-trips.
- Gate: `Cosmetic` + `auto_allow_cosmetic` skips the approver;
  `RequireInteractiveApproval` still denies when a prompt is required.
- Agent loop (extend the existing `ScriptedBackend` test): scripted call to
  `set_color_scheme` with `AutoApprover` → step `ok`, argv as expected via an
  injectable runner (add a `CommandRunner` seam so the test asserts argv without
  spawning a process).

---

## 7. Landing-demo acceptance (manual, on a Plasma 6 install)

From the spec §5, the subset C-M1 must satisfy:
1. «Сделай тёмную тему» → `set_color_scheme(dark)`
2. «Поставь громкость на 30%» → `set_volume(30)`
3. «Прибавь яркость» → `set_brightness(+10)`
4. «Поменяй обои на тёмные» → `set_wallpaper(Deamos)`

Record these as the C-M1 demo GIFs once green on hardware.

---

## 8. Commit sequence (small, reviewable; trunk-based + CI gate)

1. `feat(agent): action framework — Risk, Action trait, Registry, vetted run_cmd`
2. `feat(agent): cosmetic actions — color scheme, wallpaper, volume, brightness`
3. `feat(agent): risk-aware gate + describe()-based approval prompt`
4. `feat(agent): advertise actions in the loop + system-prompt examples`
5. `feat(image): enable action set in shipped agent.toml`
6. `test(agent): action command-builder + gate + loop coverage`

Each builds and passes `cargo test`/`fmt`/`clippy`. After merge, the standard
edition image carries the action set; verify on the next ISO.

---

## 9. Risks / open questions

- **`plasma-apply-lookandfeel` vs `-colorscheme`**: look-and-feel switches more
  (icons, splash) but may override the user's accent; confirm on a boot test
  which gives the cleanest dark/light toggle. Same Plasma-6 CLI-presence caveat
  as the OOBE bug — prefer `command -v` checks and a `gdbus`/`zbus` path later.
- **`wpctl` vs `pactl`**: Aurora ships PipeWire → `wpctl` should exist; keep
  `pactl` fallback.
- **Auto-allow vs always-show**: auto-allowing cosmetics is the spec default and
  best demo UX, but for the very first alpha we may want the prompt visible once
  to build trust. Decide at wire-up; it is one config flag.
- **zbus migration**: C-M1 uses vetted CLIs for speed; appearance/volume can move
  to `zbus` (D-Bus) later for robustness without changing the `Action` contract.
