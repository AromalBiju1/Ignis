# Ignis

A minimal UEFI bootloader, written in Rust from scratch.

Ignis boots Linux kernels directly via the EFI stub protocol and can
chainload other UEFI applications (Windows Boot Manager, other OS
loaders). No legacy BIOS support, no bespoke filesystem drivers, no
config sprawl — just enough to boot the machines people actually run
today, in code small enough that one person can read the whole thing.

Built out of dissatisfaction with GRUB's complexity. See
[SCOPE.md](./SCOPE.md) for what Ignis is and isn't trying to be.

## Status

Early development. Currently boots to a hello-world screen on real
UEFI firmware (tested via QEMU + OVMF). Config-driven boot menus and
native Linux kernel loading are in progress — see open issues.

## Building

Requires Rust nightly with the `x86_64-unknown-uefi` target:

\`\`\`bash
rustup toolchain install nightly --component rust-src
rustup target add x86_64-unknown-uefi --toolchain nightly
cargo build --target x86_64-unknown-uefi
\`\`\`

## Running (QEMU)

Requires `qemu-system-x86` and OVMF firmware installed.

\`\`\`bash
./run.sh
\`\`\`

This builds Ignis, drops it into a FAT-formatted ESP layout, and boots
it in QEMU with OVMF. Note: `run.sh` has hardcoded OVMF firmware paths
that vary by distro — check `find /usr/share -iname "OVMF_CODE*"` on
your system and update the script if needed.

## Contributing

Contributions welcome — see [CONTRIBUTING.md](./CONTRIBUTING.md).
This is early-stage, so expect the architecture to shift. Open an
issue before a large PR to avoid wasted work.

## License

MIT — see [LICENSE](./LICENSE).