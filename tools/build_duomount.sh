#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export PS5_PAYLOAD_SDK=/opt/ps5-payload-sdk
export PATH=/usr/lib/llvm-19/bin:$PATH
mkdir -p .build
bash tools/build_fpkg_bridge.sh
cc="$PS5_PAYLOAD_SDK/bin/prospero-clang"
for source in fpkg-bridge/ps5/shim/freebsd11.c fpkg-bridge/ps5/shim/syscalls.S fpkg-bridge/ps5/compat.c; do
  name=$(basename "${source%.*}")
  "$cc" -O2 -Wall -Wextra -Werror -c "$source" -o ".build/$name.o"
done
mapfile -t wraps < <(tr -d '\r' < fpkg-bridge/ps5/shim/syscalls.link)
printf '%s\n' "${wraps[@]}" > .build/syscalls.link
make -j6 duomount.elf PS5_SCE_STUBS_DIR="$PWD/tools/sce_stubs" VERSION_TAG=DuoMount-beta1
llvm-readelf -h -d duomount.elf > .build/duomount.elf.txt
sha256sum duomount.elf
