#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
base=${ONEMOUNT_DEPS_BUILD_ROOT:-/opt/duomount-build}
cp "$base/openlibm/src/openlibm-0.8.6/LICENSE.md" licenses/openlibm-LICENSE.md
cp "$base/zlib/src/zlib-1.3.2/LICENSE" licenses/zlib-LICENSE.txt
cp "$base/libpng/src/libpng-1.6.43/LICENSE" licenses/libpng-LICENSE.txt
cp "$base/libmicrohttpd/src/libmicrohttpd-1.0.1/COPYING" licenses/libmicrohttpd-COPYING.txt
# Public-domain notice from the official SQLite source release of the same version.
if [[ -f "$base/sqlite/src/sqlite-src-3460100/LICENSE.md" ]]; then
  cp "$base/sqlite/src/sqlite-src-3460100/LICENSE.md" licenses/sqlite-LICENSE.md
fi
