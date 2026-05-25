# DeamOS task runner.  Install `just`:  https://github.com/casey/just
# Image builds require Linux + podman.  On Windows use the Fedora dev VM
# (see docs/build-plan.md §1) or just let GitHub Actions do it.

image    := "deamos-base"
registry := "ghcr.io/agent4bnk-spec"
tag      := "latest"

# List available recipes
default:
    @just --list

# Build the base OCI image locally (Linux + podman)
build:
    podman build -t {{image}}:{{tag}} -f recipes/base/Containerfile .

# Sanity check: print the rebranded os-release from the built image
check:
    podman run --rm {{image}}:{{tag}} cat /usr/lib/os-release

# Lint the Containerfile
lint:
    podman run --rm -i docker.io/hadolint/hadolint < recipes/base/Containerfile

# Build a bootable installer ISO from the image (Linux only, needs root)
# Output lands in ./output/.  Pull the image first or build it locally.
iso:
    mkdir -p output
    sudo podman run --rm -it --privileged \
        --security-opt label=type:unconfined_t \
        -v ./output:/output \
        -v /var/lib/containers/storage:/var/lib/containers/storage \
        quay.io/centos-bootc/bootc-image-builder:latest \
        --type iso --use-librepo=True \
        {{registry}}/{{image}}:{{tag}}

# Boot the produced qcow2 in QEMU for a real smoke test (Linux + kvm)
run-vm:
    qemu-system-x86_64 -enable-kvm -m 4096 -smp 4 \
        -cdrom output/bootiso/install.iso \
        -drive file=output/test-disk.qcow2,if=virtio,format=qcow2 \
        -bios /usr/share/OVMF/OVMF_CODE.fd
