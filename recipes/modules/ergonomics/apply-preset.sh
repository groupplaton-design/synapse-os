#!/usr/bin/env bash
# Apply a desktop ergonomics preset matching the user's previous OS, so they
# don't have to relearn the desktop. Run inside the user's Plasma session
# (it talks to the running plasmashell + kwin over the session bus).
#
# Usage:  apply-preset.sh <windows|macos|gnome|kde>
#
# See docs/spec-familiar-ergonomics.md. NOTE: panel scripting is Plasma-version
# sensitive (Plasma 6 here); these are sane starting points that need a boot
# test to fine-tune floating/auto-hide/centering on the target.
set -euo pipefail

preset="${1:-kde}"
here="$(dirname "$(readlink -f "$0")")"

kw() { kwriteconfig6 "$@"; }

apply_decorations_right() {
    # Window buttons on the right (Windows / GNOME): menu left, min/max/close right.
    kw --file kwinrc --group org.kde.kdecoration2 --key ButtonsOnLeft "M"
    kw --file kwinrc --group org.kde.kdecoration2 --key ButtonsOnRight "IAX"
}
apply_decorations_left() {
    # Window buttons on the left (macOS): close/min/max left, nothing right.
    kw --file kwinrc --group org.kde.kdecoration2 --key ButtonsOnLeft "XIA"
    kw --file kwinrc --group org.kde.kdecoration2 --key ButtonsOnRight ""
}

load_layout() {
    local js="$here/layout-$1.js"
    [ -f "$js" ] || { echo "no layout script for '$1'"; return 0; }
    # Replace the current panel layout with the preset's.
    qdbus org.kde.plasmashell /PlasmaShell org.kde.PlasmaShell.evaluateScript \
        "$(cat "$js")"
}

case "$preset" in
    windows)
        kw --file kdeglobals --group KDE --key SingleClick false
        apply_decorations_right
        load_layout windows
        ;;
    macos)
        kw --file kdeglobals --group KDE --key SingleClick false
        apply_decorations_left
        load_layout macos
        ;;
    gnome)
        kw --file kdeglobals --group KDE --key SingleClick false
        apply_decorations_right
        load_layout gnome
        ;;
    kde|classic|*)
        # Native Aurora/Plasma look — change nothing.
        ;;
esac

# Apply window-decoration / behaviour changes live.
qdbus org.kde.KWin /KWin reconfigure 2>/dev/null || true
