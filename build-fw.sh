#!/bin/sh

set -eu

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <project> [additional \`cargo rustc\` args...]"
  exit 1
fi
if ! [ -d ./target ] && [ -f Cargo.lock ]; then
  echo "Must run $0 in the project root (next to target/)"
fi

proj="$1"
shift 1

cargo_target=./target/spec/release
fw_target=./target/firmware
fw_target_bld="$fw_target"/intermediate
prefix="$(jq -r '.hpp["platform-prefix"] | select(. != null)' ./"$proj"/compile-target/spec.json)"
if [ -z "$prefix" ]; then
  AS="as"
  OBJCOPY="objcopy"
else
  AS="$prefix-as"
  OBJCOPY="$prefix-objcopy"
fi
as_flags="$(jq -r '.hpp["as-flags"] | select(. != null)' ./"$proj"/compile-target/spec.json)"

echo 'Cleaning project...'
cargo clean
mkdir -p ./target/ ./target/compile-target "$fw_target" "$fw_target_bld"

echo 'Building tools...'
gcc -O3 ./mksunxiboot/src.c -o ./target/mksunxiboot
rm -rf ./target/compile-target/*
cp ./"$proj"/compile-target/* ./target/compile-target/
"$AS" $as_flags ./target/compile-target/hpprt.s -o ./target/compile-target/hpprt.o

echo 'Rebuilding project...'
cargo +nightly rustc -Z build-std=core,alloc --release --target=./target/compile-target/spec.json -p "$proj" "$@" || exit $?

echo 'Extracting blob...'
"$OBJCOPY" -O binary -j .text "$cargo_target"/"$proj" "$fw_target_bld"/"$proj".bin || exit $?

echo 'Packaging for boot...'
./target/mksunxiboot "$fw_target_bld"/"$proj".bin "$fw_target_bld"/"$proj".hdr || exit $?
dd bs=1024 if="$fw_target_bld"/"$proj".hdr seek=8 of="$fw_target"/"$proj".img || exit $?

echo "Firmware image output to $fw_target/$proj.img"
