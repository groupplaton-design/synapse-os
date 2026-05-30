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

# Apply the DeamOS look for this user, live, now — brand identity, regardless of
# the layout choice. The /etc/skel containment-ID hack (appletsrc ->
# Containments[1]) is unreliable: Plasma's real desktop containment often isn't
# ID 1, so the skel value is ignored and Aurora's "человечки" wallpaper wins.
# Setting it per-user at first login is ID-independent and robust.
# Desktop wallpaper = the dark "D + waveform" art (replaces the Aurora cartoon).
if command -v plasma-apply-wallpaperimage >/dev/null 2>&1; then
    plasma-apply-wallpaperimage "/usr/share/wallpapers/DeamosWave/" \
        || plasma-apply-wallpaperimage \
            "/usr/share/wallpapers/DeamosWave/contents/images/1672x941.png" \
        || true
fi
# Default colour theme: DeamOS Dark.
if command -v plasma-apply-colorscheme >/dev/null 2>&1; then
    plasma-apply-colorscheme DeamOSDark || plasma-apply-colorscheme "DeamOS Dark" || true
fi

mkdir -p "$(dirname "$flag")"
date > "$flag"

kdialog --title "DeamOS" --passivepopup \
    "Рабочий стол настроен под: $choice. Изменить можно позже. / Desktop set to: $choice." 6 \
    || true
