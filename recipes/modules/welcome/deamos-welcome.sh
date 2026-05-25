#!/usr/bin/env bash
# First-login welcome: ask which OS the user came from and shape the desktop to
# match (minimise relearning). Runs once per user via /etc/xdg/autostart.
set -euo pipefail

flag="${XDG_STATE_HOME:-$HOME/.local/state}/deamos/welcome-done"
[ -f "$flag" ] && exit 0

# Wait for the Plasma session to be ready enough to script.
sleep 4

choice=$(kdialog --title "Welcome to DeamOS" --radiolist \
"Откуда вы перешли? Подберём привычный вид рабочего стола.

Which OS are you coming from? We'll match the desktop layout." \
    windows "Windows" on \
    macos   "macOS" off \
    gnome   "Ubuntu / GNOME" off \
    kde     "Linux / KDE — как есть / keep as-is" off) || choice="kde"

[ -n "$choice" ] || choice="kde"

/usr/libexec/deamos/apply-preset.sh "$choice" || true

mkdir -p "$(dirname "$flag")"
date > "$flag"

kdialog --title "DeamOS" --passivepopup \
    "Рабочий стол настроен под: $choice. Изменить можно позже. / Desktop set to: $choice." 6 \
    || true
