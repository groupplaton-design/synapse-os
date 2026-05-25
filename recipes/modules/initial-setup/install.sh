#!/usr/bin/env bash
# First-boot account + language wizard.
#
# WHY: a DeamOS install must never lock the user out. If the installer finishes
# without a human account (e.g. the user skipped the Anaconda user screen), the
# machine boots to a login with no credentials. `initial-setup` is Fedora's
# first-boot tool: it runs ONLY when no human user exists yet and walks the user
# through language, keyboard, time zone and account creation before login.
#
# This is belt-and-suspenders next to the installer's own user screen.
set -euo pipefail

# bootc/Atomic image build: layer the packages with dnf.
dnf install -y initial-setup initial-setup-gui

# Runs on first boot; the service self-skips when a human user already exists.
systemctl enable initial-setup.service

# Sanity: the unit must exist or the build is wrong.
test -f /usr/lib/systemd/system/initial-setup.service \
    || { echo "initial-setup.service missing after install" >&2; exit 1; }

# NOTE: needs a boot test on KDE/Atomic to confirm the GUI path presents before
# SDDM. If the graphical flow misbehaves, the text path still creates an account
# (no lockout), which is the hard requirement.
