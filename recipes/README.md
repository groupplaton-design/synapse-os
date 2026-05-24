# recipes/

bootc / Containerfile recipes for building Veil OS image editions. GitHub
Actions (`build-image.yml`) builds these into OCI images on GHCR; the installer
pipeline (`build-iso.yml`) turns them into bootable ISOs.

## Current structure

```
recipes/
  base/              # veil-base: Aurora + rebrand + branding
  standard/          # veil-standard: base + AI stack (Ollama)
  standard-nvidia/   # veil-standard-nvidia: aurora-nvidia + rebrand + branding + AI stack
  modules/
    rebrand.sh       # os-release -> Veil OS (shared)
    ai-stack/        # Ollama systemd service install
```

## Build graph

```
aurora:stable             ─► base ─► standard
aurora-nvidia-open:stable ─────────► standard-nvidia
```

NVIDIA uses the **open** kernel-module image (`aurora-nvidia-open`, Turing/RTX
20-series+) — freely redistributable, unlike the access-gated proprietary one.

`standard` builds `FROM` the pushed `veil-base` (dependent CI job).
`standard-nvidia` builds `FROM` upstream `aurora-nvidia` directly (it can't use
the non-NVIDIA `veil-base`), reusing the same rebrand/branding/AI-stack pieces.
NVIDIA is built only on `workflow_dispatch` or a release tag (large base).

## Planned editions (later)

`lite` (minimal), `studio` (full creator AI stack), `server` (headless Hub).
The AI stack will also grow whisper.cpp / piper / embeddings beyond Ollama.
