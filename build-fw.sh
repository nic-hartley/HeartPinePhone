#!/bin/sh

mkdir -p ./target/

proj="$1"
if [ -z "$proj" ]; then
  echo "Usage: $0 <project> [additional args...]"
  exit 1
fi
shift 1

while [ -f ../Cargo.toml ]; do
  cd ..
done

cd mksunxiboot
gcc -O3 mksunxiboot.c -o t
cd ..

cargo_target=./target/aarch64-unknown-none/release
fw_target=./target/firmware
fw_target_bld="$fw_target"/intermediate

mkdir -p "$fw_target" "$fw_target_bld"

cargo +nightly build --release --target=aarch64-unknown-none -p "$proj" "$@" || exit $?
aarch64-linux-gnu-objcopy -O binary -j .text "$cargo_target"/"$proj" "$fw_target_bld"/"$proj".bin || exit $?
./mksunxiboot/t "$fw_target_bld"/"$proj".bin "$fw_target_bld"/"$proj".hdr || exit $?
dd bs=1024 if="$fw_target_bld"/"$proj".hdr seek=8 of="$fw_target"/"$proj".img || exit $?

echo "Firmware image output to $fw_target/$proj.img"
