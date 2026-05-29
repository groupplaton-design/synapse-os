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

# Call a Plasma/KWin D-Bus method, surviving Plasma 6's CLI churn. Plain `qdbus`
# is frequently ABSENT on Plasma 6 (it ships `qdbus6`; many setups have `gdbus`),
# so the old `qdbus`-only calls silently no-op'd and the preset never applied.
# Try gdbus -> qdbus6 -> qdbus. Args: <service> <object-path> <method> [arg]
plasma_dbus() {
    local svc="$1" path="$2" method="$3" arg="${4-}"
    if command -v gdbus >/dev/null 2>&1; then
        if [ $# -ge 4 ]; then
            gdbus call --session --dest "$svc" --object-path "$path" \
                --method "$method" "$arg"
        else
            gdbus call --session --dest "$svc" --object-path "$path" \
                --method "$method"
        fi
    elif command -v qdbus6 >/dev/null 2>&1; then
        if [ $# -ge 4 ]; then qdbus6 "$svc" "$path" "$method" "$arg"; else qdbus6 "$svc" "$path" "$method"; fi
    elif command -v qdbus >/dev/null 2>&1; then
        if [ $# -ge 4 ]; then qdbus "$svc" "$path" "$method" "$arg"; else qdbus "$svc" "$path" "$method"; fi
    else
        echo "apply-preset: no D-Bus CLI (gdbus/qdbus6/qdbus) found" >&2
        return 1
    fi
}

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
    plasma_dbus org.kde.plasmashell /PlasmaShell \
        org.kde.PlasmaShell.evaluateScript "$(cat "$js")"
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

# Apply window-decoration / behaviour changes live (fully-qualified method name
# is required by gdbus; qdbus accepts it too).
plasma_dbus org.kde.KWin /KWin org.kde.KWin.reconfigure 2>/dev/null || true
