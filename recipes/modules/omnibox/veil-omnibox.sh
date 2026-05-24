#!/usr/bin/env bash
# Veil Omnibox: ask the local agent and show the answer, via KDE dialogs.
# Bound to a global hotkey and launchable from the app menu / KRunner.
set -euo pipefail

ENDPOINT="http://127.0.0.1:7421/agent"

PROMPT=$(kdialog --title "Veil Agent" --inputbox "Ask Veil Agent:" 2>/dev/null) || exit 0
[ -z "${PROMPT}" ] && exit 0

PAYLOAD=$(python3 -c 'import json,sys; print(json.dumps({"prompt": sys.argv[1]}))' "$PROMPT")

if ! RESP=$(curl -fsS "$ENDPOINT" -H 'content-type: application/json' -d "$PAYLOAD" 2>/dev/null); then
    kdialog --title "Veil Agent" --error "Veil Agent is not running (start the veil-agentd user service)." 2>/dev/null || true
    exit 1
fi

ANSWER=$(printf '%s' "$RESP" | python3 -c \
    'import json,sys; d=json.load(sys.stdin); print(d.get("answer") or d.get("reason") or "(no answer)")')

kdialog --title "Veil Agent" --msgbox "$ANSWER" 2>/dev/null || true
