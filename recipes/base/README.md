# recipes/base — base image decision (ADR)

**Status:** accepted · **Milestone:** A-M0 · **Date:** 2026-06

## Decision

Veil OS is built **on top of** Universal Blue **Aurora**
(`ghcr.io/ublue-os/aurora:stable`), not from raw Fedora and not by rebuilding
Fedora.

## Why this base

Aurora is a KDE Plasma desktop built `FROM` Fedora Kinoite using bootc, part of
the Universal Blue project. Picking it over the alternatives:

| Candidate | Verdict |
|---|---|
| **Rebuild Fedora from sources** | Rejected. Hundreds of person-years; a solo/small team cannot maintain a kernel + package set. |
| **Raw Fedora Kinoite (`quay.io/fedora/...`)** | Workable, but we'd re-solve NVIDIA drivers, codecs, and hardware enablement ourselves. |
| **Universal Blue Aurora (KDE)** | **Chosen.** Fedora Atomic + KDE with drivers/codecs already integrated and continuously tested. Saves months. |
| Bluefin (GNOME) / Bazzite (gaming) | Same family, wrong desktop/focus. Aurora is the KDE one. |

## What this means

- We maintain a **layer**, not an OS: `Containerfile` + our apps. Kernel,
  glibc, systemd, dnf — all upstream, updated for free.
- We inherit Universal Blue's driver and codec work and their bootc plumbing.
- Differentiation lives **above** the base: Veil Agent, the AI stack, privacy
  layers, branding, first-boot UX.

## Trade-offs accepted

- Coupled to the Fedora/ublue release cadence and their decisions.
- Deep low-level kernel customization is harder (deferred to roadmap Phase 5
  "NPU-first / WebGPU", where we may selectively rebuild individual packages on
  the Fedora base).

## Revisit if

- Aurora's direction diverges from ours, or
- we hit a hard ceiling that only a custom base can solve (post Phase 4).

In that case the migration target is **raw Fedora Kinoite bootc**, keeping the
same `Containerfile`/CI shape — a contained change, not a rewrite.
