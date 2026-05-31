#!/usr/bin/env bash
# DeamOS first-run setup: the single up-front wizard, shown ONCE at first login
# (autostart phase 1) and never again. Two questions — language, then which OS
# you came from — then it applies the language, the matching layout, and the
# DeamOS wallpaper + theme.
#
# NOTE: the done-flag is written BEFORE applying anything, so a failure (or a
# crash) while applying can never make the wizard reappear on the next login.
set -uo pipefail   # not -e: a failing apply step must not abort before we finish

flag="${XDG_STATE_HOME:-$HOME/.local/state}/deamos/setup-done"
# Migrate the old flag name so existing installs don't see the wizard again.
old_flag="${XDG_STATE_HOME:-$HOME/.local/state}/deamos/welcome-done"
{ [ -f "$flag" ] || [ -f "$old_flag" ]; } && exit 0

# Wait for the Plasma session to be ready enough to script.
sleep 4

# ---- Step 1: language -------------------------------------------------------
lang=$(kdialog --title "DeamOS — язык / language" --radiolist \
"Выберите язык системы / Choose your system language:" \
    ru "Русский" on \
    en "English" off) || lang="ru"
[ -n "$lang" ] || lang="ru"

case "$lang" in
    ru) locale="ru_RU.UTF-8"; tr="ru" ;;
    *)  locale="en_US.UTF-8"; tr="en_US" ;;
esac

# ---- Step 2: which OS did you come from? ------------------------------------
if [ "$lang" = "ru" ]; then
    prompt="Откуда вы перешли? Подберём привычный вид рабочего стола."
else
    prompt="Which OS are you coming from? We'll match the desktop layout."
fi
choice=$(kdialog --title "DeamOS — первичная настройка / first-time setup" --radiolist \
"$prompt" \
    windows "Windows" on \
    macos   "macOS" off \
    gnome   "Ubuntu / GNOME" off \
    kde     "Linux / KDE" off) || choice="kde"
[ -n "$choice" ] || choice="kde"

# ---- Write the done-flag NOW (before applying) so it never repeats ----------
mkdir -p "$(dirname "$flag")"
date > "$flag"

# ---- Apply language (per-user; takes full effect after re-login) ------------
mkdir -p "$HOME/.config"
cat > "$HOME/.config/plasma-localerc" <<EOF
[Formats]
LANG=${locale}

[Translations]
LANGUAGE=${tr}
EOF

# ---- Apply the familiar layout preset ---------------------------------------
/usr/libexec/deamos/apply-preset.sh "$choice" || true

# ---- DeamOS look: wallpaper (dark "D + waveform") + Dark theme --------------
if command -v plasma-apply-wallpaperimage >/dev/null 2>&1; then
    plasma-apply-wallpaperimage "/usr/share/wallpapers/DeamosWave/" \
        || plasma-apply-wallpaperimage \
            "/usr/share/wallpapers/DeamosWave/contents/images/1672x941.png" \
        || true
fi
if command -v plasma-apply-colorscheme >/dev/null 2>&1; then
    plasma-apply-colorscheme DeamOSDark || plasma-apply-colorscheme "DeamOS Dark" || true
fi

# ---- Tell the user; offer to re-login so the language applies fully ---------
if [ "$lang" = "ru" ]; then
    msg="Настройка готова (вид: $choice). Чтобы русский применился полностью, выйдите и войдите снова."
else
    msg="Setup done (layout: $choice). Log out and back in for the language to fully apply."
fi
kdialog --title "DeamOS" --passivepopup "$msg" 8 || true
