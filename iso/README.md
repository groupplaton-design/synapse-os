# iso/

ISO build is driven by `.github/workflows/build-iso.yml`, which wraps
`bootc-image-builder` to turn the `veil-base` OCI image into a bootable Anaconda
installer ISO.

## How to get an ISO

1. GitHub → **Actions** → **build-iso** → **Run workflow** (or push a `v*` tag).
2. When it finishes (~20–40 min), download the **`veil-os-iso`** artifact from
   that run. It contains `install.iso` + `SHA256SUMS`. Retention: 90 days.

## Why not GitHub Releases

The ISO is ~4.5 GB. GitHub **release assets are capped at 2 GiB**, so the ISO
cannot be attached to a Release. Permanent, public hosting will move to object
storage (Cloudflare R2 / S3) when there is a public alpha; until then the
workflow artifact is the download channel.

## Installing (reminder)

The installer **wipes the target disk** — it is a full OS, not an app. Test in a
VM first, then install on a spare laptop / second disk / external USB SSD. Never
on your only working machine.

## Later

- Per-edition ISOs (standard with the AI stack, not just base).
- Kickstart/install-config presets.
- Object-storage upload + a downloads page.
