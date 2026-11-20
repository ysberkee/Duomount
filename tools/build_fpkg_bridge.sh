#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../fpkg-bridge"
export PS5_PAYLOAD_SDK=/opt/ps5-payload-sdk
export RUSTUP_HOME=${RUSTUP_HOME:-/opt/kraken-unified-build/rustup} CARGO_HOME=${CARGO_HOME:-/opt/kraken-unified-build/cargo}
export RUSTUP_TOOLCHAIN=1.99.0 RUSTUP_AUTO_INSTALL=0 RUSTC_BOOTSTRAP=1
export PATH="$CARGO_HOME/bin:/usr/lib/llvm-19/bin:$PATH"
export CARGO_TARGET_X86_64_PS5_FREEBSD_RUSTFLAGS='--cfg libc_unstable_freebsd_version="11"'
export CC_x86_64_ps5_freebsd="$PS5_PAYLOAD_SDK/bin/prospero-clang"
export AR_x86_64_ps5_freebsd="$PS5_PAYLOAD_SDK/bin/prospero-ar"
(cd vendor/ampr-emu-ng && sha256sum -c --ignore-missing --strict SHA256SUMS)
(cd vendor/dlc_emu && sha256sum -c --strict SHA256SUMS)
(cd vendor/pgo_stub && sha256sum -c --strict SHA256SUMS)
cargo check --offline --locked --lib --target x86_64-unknown-freebsd
cargo build --release --locked --lib --target ps5/x86_64-ps5-freebsd.json -Z json-target-spec \
  -Z build-std=std,panic_abort -Z build-std-features=panic-unwind
ls -lh target/x86_64-ps5-freebsd/release/libduomount_fpkg.a
