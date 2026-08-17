# Ignis — Scope

## What Ignis is
A minimal UEFI bootloader written in Rust. Boots Linux kernels
directly via the EFI stub protocol, and can chainload other
UEFI applications (Windows Boot Manager, other loaders).

## What Ignis is not (yet, maybe ever)
- Not a GRUB replacement for legacy BIOS systems.
- Not filesystem-agnostic — targets FAT32 ESP only, matching
  the UEFI spec's own assumptions.
- Not trying to support every architecture on day one — x86_64
  first, arm64 later if there's contributor interest.

## Design principles
1. Readable over clever — anyone should be able to read
   src/main.rs and understand the whole boot flow.
2. Fail loud — no silent fallback to firmware menus without
   a clear on-screen reason.
3. Config over convention — boot entries are explicit, not
   auto-discovered magic.