#!/usr/bin/env bash
# Rebrand os-release to Veil OS. Shared by every edition that starts from an
# upstream (non-Veil) base. Conservative: keeps ID/ID_LIKE so Fedora/ublue
# tooling and package managers keep working.
set -euo pipefail

sed -i \
    -e 's/^NAME=.*/NAME="Veil OS"/' \
    -e 's/^PRETTY_NAME=.*/PRETTY_NAME="Veil OS (alpha)"/' \
    -e 's/^VARIANT=.*/VARIANT="Veil OS Base"/' \
    -e 's/^VARIANT_ID=.*/VARIANT_ID=veil-base/' \
    /usr/lib/os-release

printf '%s\n' \
    'VEIL_EDITION="base"' \
    'VEIL_VERSION="2026.05.0-alpha"' \
    >> /usr/lib/os-release

grep -q 'Veil OS' /usr/lib/os-release
