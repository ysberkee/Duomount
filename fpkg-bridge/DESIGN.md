# DuoMount embedded FPKG contract

This is an integration fork of Kraken Mount `e1e784560fe3a56e3b5ffd579f47ed5b2cf194f7`, linked as a Rust **static library** into the ShadowMountPlus C host. `autobins = false`: no Rust main, self-copy, second payload, standalone title registrar, updater or HTTP server is built. Historical standalone source files remain for provenance. `DESIGN.upstream.md` describes that original program; this file defines the embedded integration.

## Ownership

The C host alone discovers sources, arbitrates duplicates, registers titles, writes mount.lnk, controls storage operations and provides the UI/API on port 10101. Existing Shadow folder/image behavior remains in C.

Only PS5 FIH debug PKGs with PPSA IDs enter this bridge. The package reader validates declared ranges against the file length; PS4 CNT, retail, LIH and truncated packages fail before cache preparation. The C classifier also excludes `.part` suffixes. The C source-stability gate is reused.

The host calls `om_fp_prepare` to obtain `/data/shadowmount/fpkg/caches/<TID>`. That directory is registered through the ordinary C directory pipeline. A cache is not a complete extracted game: package data stays in the source PKG, accessed by the bundled ng runtime. The host must not pass a cache to image-unpack.

Native fakelib callbacks skip a title only when its current mount.lnk resolves to a valid bridge cache. `om_fp_track` follows each successful host link change. The embedded watcher revalidates the current link before handling a title. Thus a retained old cache cannot take ownership of a newly selected folder/image with the same title ID.

The Rust watcher handles only those selected FPKGs: fakelib overlay, writable app0 and sandbox storage/package aliases use the upstream launch implementation. Native ShellCore hooks prepare the base runtime before launch. The C game lifecycle service still tracks all titles and blocks cache rebuilding during active games.

## Paths and preservation

| Path | Meaning |
| --- | --- |
| `/data/shadowmount/config.ini`, `profiles.ini` | Existing host settings; there is no second active TOML config |
| `/data/shadowmount/fpkg/caches/<TID>` | Separate embedded FPKG cache |
| `/data/shadowmount/fpkg/ampr/libSceAmpr.sprx` | Bundled ng v0.4.1, installed only when missing; a supplied file is preserved |
| `/data/shadowmount/debug.log` | Host log, bridge lines prefixed `[FPKG]` |
| `/user/app/<TID>/mount.lnk` | Sole selected-source authority |

Original `/data/krakenmount` caches are not purged or migrated destructively. Upstream `.built` preservation rules remain: rebuild removes only build-owned files and retains game-written files. No forced unmount is added. The bridge's purge/unmount registrar shim refuses destructive cleanup and delegates it to the controlled C host.

A same-ID PKG on two drives keeps its currently selected live source. When that source disappears, upstream relink logic selects the available copy. This avoids alternately rebuilding one cache for two drives on every scan. Copy/move/delete in the native API operate on the physical PKG, not on its partial cache.

An idle migration helper may release only a nullfs from exact `/data/krakenmount/caches/PPSA#####` onto matching `/system_ex/app/PPSA#####`. Other mounts, nested paths and mismatched IDs are excluded. It never forces a busy mount and never deletes the old cache.

## Settings

FPKGs always require ng injection. The host's emulator-update toggle controls bundled emulator additions; automatic ng network updating is disabled in this bridge. The native fakelib-disable toggle is unavailable for an FPKG. Folder/image settings remain with the original C implementation. Backport selection comes from the host. The package cache builder and ng format behavior otherwise follow upstream.

## C ABI and lifecycle

`src/bridge.rs` exports `om_fp_prepare`, `om_fp_cache`, `om_fp_source`, `om_fp_track`, `om_fp_start`, `om_fp_running` and `om_fp_stop`. Strings are UTF-8 C strings; output capacity includes its NUL. `src/ffi.rs` implements the checked conversion and output contract. Errors return errno-style codes and optional text. Cache builds are serialized.

The host starts one watcher and waits for active sessions to clean up before shutdown. `om_fp_stop` sets a stop flag and joins the thread. Kqueue waits are bounded to 250 ms even with a functioning vnode watch, so an idle thread notices shutdown. No detached bridge thread can outlive the host. Logging and snd0info updates call into the C host, which owns its database locks.

## Build and verification

Use the repository-root `tools/build_duomount.sh`, not the historical standalone `ps5/build.sh` or Dockerfile. It checks bundled runtime hashes, runs `cargo check --offline --locked --lib --target x86_64-unknown-freebsd` (zero errors/warnings), builds the PS5 staticlib, compiles the original FreeBSD 11 ABI shims and links them into the C main. `ps5/entry.c`, standalone appdb and standalone USB helpers are not linked.

`tools/check_duomount.sh` runs portable format/legacy-mount tests under sanitizers, actual FFI buffer tests, package-reader tests and the original Shadow cleanup/storage/permission checks. `tools/check_elf.py` checks executable layout, imports and absence of host TLS/libraries. These local checks do not replace real FPKG and image game-launch testing on the console.

Package decoder/performance code is unchanged. Claims about a performance improvement still require the upstream two-package A/B launch measurements. Original licensing and third-party notices remain applicable.
