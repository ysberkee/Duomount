# Native components in DuoMount

Original ShadowMountPlus copyright notices and the root GPL-3.0 license remain in place. The embedded Kraken fork has its own original notices in `fpkg-bridge/THIRD_PARTY_NOTICES`.

| Component | Version | Source / license |
| --- | --- | --- |
| openlibm | 0.8.6 | https://github.com/JuliaMath/openlibm, see `openlibm-LICENSE.md` for the component notices |
| zlib | 1.3.2 | https://zlib.net/, `zlib-LICENSE.txt` |
| libpng | 1.6.43 | https://libpng.org/, `libpng-LICENSE.txt` |
| libmicrohttpd | 1.0.1 | https://www.gnu.org/software/libmicrohttpd/, `libmicrohttpd-COPYING.txt` |
| libjpeg-turbo | 3.0.2 | https://libjpeg-turbo.org/, `libjpeg-turbo-LICENSE.md`, `libjpeg-turbo-README.ijg` |
| SQLite | 3.46.1 | https://sqlite.org/2024/sqlite-amalgamation-3460100.zip, public domain; `sqlite-LICENSE.md` |
| libsmb2 | 6.2 | https://github.com/sahlberg/libsmb2, `libsmb2-COPYING.txt`, `libsmb2-LGPL-2.1.txt` |
| json-c | 0.19 | https://github.com/json-c/json-c, `json-c-COPYING.txt`; upstream Shadow ownership patch retained |
| SDK syscall import stub | commit `4122b714c7e7a9449d403812b9d640bf7a55af3f` | https://github.com/ps5-payload-dev/sdk, GPL-3.0; root `LICENSE` |

Pinned source recipes and checksums for native libraries are fetched by `tools/build_native_deps.sh`. JSON-C is fetched by `tools/build_ps5_json_c.py`. Combined C/Rust sources and build scripts are supplied so the payload can be rebuilt and relinked with modified library versions. No SDK package or Sony SPRX binary is included in this source archive.
