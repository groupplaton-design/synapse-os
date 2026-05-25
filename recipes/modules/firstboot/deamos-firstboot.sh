#!/usr/bin/env bash
# DeamOS first-boot setup (A-M3): pick a local model sized to this machine and
# pull it. Runs once per user; guarded by a flag file so it never repeats once
# it has succeeded.
set -euo pipefail

FLAG="${XDG_CONFIG_HOME:-$HOME/.config}/deamos/firstboot-done"
[ -f "$FLAG" ] && exit 0
mkdir -p "$(dirname "$FLAG")"

notify() {
    command -v notify-send >/dev/null 2>&1 && notify-send "DeamOS" "$1" || true
}

# Wait for the local Ollama service (up to ~60s).
for _ in $(seq 1 30); do
    if curl -fsS http://127.0.0.1:11434/api/version >/dev/null 2>&1; then
        break
    fi
    sleep 2
done

MODEL="$(deamos-agentd recommend --quiet 2>/dev/null || echo llama3.2:3b)"
notify "Setting up local AI: downloading ${MODEL}…"

if ollama pull "$MODEL"; then
    notify "Local AI ready: ${MODEL}"
    touch "$FLAG"
fi
