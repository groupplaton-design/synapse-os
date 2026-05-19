# iso/

ISO build scripts and release artifacts.

This directory will hold:

- Build scripts that wrap `bootc-image-builder` / `osbuild-composer` to turn an OCI image (from `recipes/`) into a bootable ISO
- Release notes and changelogs per ISO
- Signature / checksum files

ISO outputs are **not committed to git** (see `.gitignore`). They are produced by CI and uploaded to a release server / S3 bucket.

Planned structure:

```
iso/
  build.sh           # Wrapper script
  install-config/    # Anaconda kickstart-style configs
  releases/          # Release notes per version
```

Files will arrive in Phase 1 of the roadmap.
