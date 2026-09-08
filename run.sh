#!/usr/bin/env bash
set -euo pipefail

TARGET=x86_64-unknown-uefi
BUILD_DIR=target/$TARGET/debug
IMG_DIR=esp
OVMF_CODE=/usr/share/edk2/x64/OVMF_CODE.4m.fd
OVMF_VARS=./OVMF_VARS.fd

echo "[*] Building Ignis..."

cargo build --target $TARGET

echo "[*] Preparing ESP layout ..."
rm -rf $IMG_DIR

mkdir -p $IMG_DIR/EFI/BOOT

cp $BUILD_DIR/ignis.efi $IMG_DIR/EFI/BOOT/BOOTX64.EFI
cp ignis.conf $IMG_DIR/ignis.conf
echo "[*] Booting in QEMU..."

qemu-system-x86_64 \
  -machine q35 -m 256M \
  -drive if=pflash,format=raw,readonly=on,file=$OVMF_CODE \
  -drive if=pflash,format=raw,file=$OVMF_VARS \
  -drive format=raw,file=fat:rw:$IMG_DIR \
  -net none \
  -serial stdio