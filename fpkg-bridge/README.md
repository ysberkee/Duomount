# DuoMount FPKG bridge

Embedded fork of [Kraken Mount](https://github.com/quer3q/kraken-mount) at `e1e784560fe3a56e3b5ffd579f47ed5b2cf194f7`. This directory builds `libduomount_fpkg.a`, linked into the full ShadowMountPlus C host. It does not start another payload or webserver.

See [DESIGN.md](DESIGN.md) for integration ownership, paths, C ABI, lifecycle and preservation rules, and the [root README](../README.md) for the combined build and current console test status.

The original standalone documentation is preserved in [README.upstream.md](README.upstream.md) and [DESIGN.upstream.md](DESIGN.upstream.md). Those standalone build/deployment instructions do not produce DuoMount. Original credits, GPL license and [third-party notices](THIRD_PARTY_NOTICES) apply.
