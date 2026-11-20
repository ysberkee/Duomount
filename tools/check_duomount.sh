#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export RUSTUP_HOME=${RUSTUP_HOME:-/opt/kraken-unified-build/rustup} CARGO_HOME=${CARGO_HOME:-/opt/kraken-unified-build/cargo}
export RUSTUP_TOOLCHAIN=1.99.0 RUSTUP_AUTO_INSTALL=0
export PATH="$CARGO_HOME/bin:/usr/lib/llvm-19/bin:$PATH"
mkdir -p .build/checks
cc -std=c11 -D_GNU_SOURCE -O1 -g -Wall -Wextra -Werror -fsanitize=address,undefined \
  -Iinclude tests/test_fpkg_format.c src/sm_fpkg_format.c -o .build/checks/fpkg-format
.build/checks/fpkg-format
rustc --edition 2021 --test fpkg-bridge/src/ffi.rs -o .build/checks/ffi
.build/checks/ffi
(cd fpkg-bridge && cargo test --offline --locked -p kraken-mount-pkg --target x86_64-unknown-linux-gnu)
python3 tools/test_scan_permissions.py
python3 tools/test_sandbox_cleanup.py
python3 tools/test_storage_paths.py
