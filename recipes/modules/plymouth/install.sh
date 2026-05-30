#!/usr/bin/env bash
# Install the DeamOS Plymouth boot theme and make it the default.
#
# RISK: this regenerates the initramfs so the new splash is used at boot. A
# broken splash script can block the LUKS unlock prompt -> boot lockout. The
# theme implements the password/question/message callbacks for exactly this
# reason. ALWAYS smoke-test boot (and disk unlock) in a VM before hardware.
set -euo pipefail

THEME_DIR=/usr/share/plymouth/themes/deamos

# The Containerfile COPYs deamos.plymouth, deamos.script, background.png and
# d.png into $THEME_DIR before this script runs. Verify they are present.
for f in deamos.plymouth deamos.script background.png d.png; do
    test -f "$THEME_DIR/$f" || { echo "plymouth: missing $THEME_DIR/$f" >&2; exit 1; }
done

# The "script" theme type needs Plymouth's script renderer plugin
# (/usr/lib64/plymouth/script.so), which the Aurora base does not ship. Install
# it, or `plymouth-set-default-theme` fails with "script.so does not exist".
if [ ! -e /usr/lib64/plymouth/script.so ]; then
    dnf install -y plymouth-plugin-script
fi
test -e /usr/lib64/plymouth/script.so \
    || { echo "plymouth: script.so still missing after install" >&2; exit 1; }

# Make DeamOS the default Plymouth theme.
plymouth-set-default-theme deamos

# Regenerate the initramfs for every installed kernel so the theme is embedded.
# (bootc/Atomic image build: do it explicitly per kernel; -R on a live system
# is not available during the container build.)
regenerated=0
for kver in /usr/lib/modules/*/; do
    kver=$(basename "$kver")
    if [ -f "/usr/lib/modules/$kver/vmlinuz" ]; then
        echo "plymouth: regenerating initramfs for $kver"
        dracut --force --no-hostonly \
            "/usr/lib/modules/$kver/initramfs.img" "$kver"
        regenerated=1
    fi
done

if [ "$regenerated" -eq 0 ]; then
    echo "plymouth: WARNING — no kernel found to regenerate initramfs for" >&2
fi

# Sanity: default theme is now deamos.
plymouth-set-default-theme | grep -qx deamos \
    || { echo "plymouth: default theme is not 'deamos'" >&2; exit 1; }
