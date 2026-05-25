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

grep -q 'DeamOS' /usr/lib/os-release
