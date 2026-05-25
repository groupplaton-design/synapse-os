#!/usr/bin/env bash
# Rebrand os-release to DeamOS. Shared by every edition that starts from an
# upstream (non-Deamos) base. Conservative: keeps ID/ID_LIKE so Fedora/ublue
# tooling and package managers keep working.
set -euo pipefail

sed -i \
    -e 's/^NAME=.*/NAME="DeamOS"/' \
    -e 's/^PRETTY_NAME=.*/PRETTY_NAME="DeamOS (alpha)"/' \
    -e 's/^VARIANT=.*/VARIANT="DeamOS Base"/' \
    -e 's/^VARIANT_ID=.*/VARIANT_ID=deamos-base/' \
    /usr/lib/os-release

printf '%s\n' \
    'DEAMOS_EDITION="base"' \
    'DEAMOS_VERSION="2026.05.0-alpha"' \
    >> /usr/lib/os-release

# --- Boot / login identity --------------------------------------------------
# The upstream base ships hostname "aurora", which is what the text console
# shows ("aurora login:") and what SDDM displays. Default it to "deamos"; the
# installer / initial-setup can still let the user override.
echo 'deamos' > /etc/hostname

# Login banner printed above the console prompt.
printf 'DeamOS \\r — a product of free people\n\n' > /etc/issue
printf 'DeamOS \\r — a product of free people\n\n' > /etc/issue.net

# GRUB title for any non-ostree-BLS path (ostree menu entries derive from the
# os-release PRETTY_NAME set above).
if [ -f /etc/default/grub ]; then
    sed -i 's/^GRUB_DISTRIBUTOR=.*/GRUB_DISTRIBUTOR="DeamOS"/' /etc/default/grub || true
fi

grep -q 'DeamOS' /usr/lib/os-release
# NOTE: the Plymouth boot-splash theme + the SDDM login theme still reference the
# upstream brand visually; retheming those needs a boot test (see assets/README.md)
# and is tracked separately so a bad theme can't break boot.
