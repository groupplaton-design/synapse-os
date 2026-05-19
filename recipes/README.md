# recipes/

bootc / Containerfile recipes for building Veil OS image variants.

This directory will hold the `Containerfile`s that define each Veil OS edition (Lite, Standard, Studio, Server). GitHub Actions builds these into OCI images, which become bootable ISOs via the installer pipeline.

Planned structure:

```
recipes/
  base/              # Common base on top of Fedora Atomic
  lite/              # Lite edition Containerfile
  standard/          # Standard edition (with preinstalled local LLM stack)
  studio/            # Studio edition (full creator AI stack)
  server/            # Server / Hub edition (headless)
  modules/           # Reusable layer pieces (nvidia, amd-rocm, intel-arc, ai-stack, kde-plasma, etc.)
```

Files will arrive in Phase 1 of the roadmap.
