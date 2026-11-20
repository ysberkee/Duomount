#!/usr/bin/env bash
set -euo pipefail
export PS5_PAYLOAD_SDK=/opt/ps5-payload-sdk
export PATH=/usr/lib/llvm-19/bin:$PATH
export MAKEFLAGS=-j6
export CPPFLAGS=-I/opt/ps5-payload-sdk/target/user/homebrew/include
export LDFLAGS=-L/opt/ps5-payload-sdk/target/user/homebrew/lib
repo_root=$(cd "$(dirname "$0")/.." && pwd)
recipes=${ONEMOUNT_RECIPE_DIR:-$repo_root/.deps/pacbrew-repo}
pin=722187f0ee246c0d7d57401c8a97de1c7f4e2ff0
if [[ ! -d "$recipes/.git" ]]; then
  mkdir -p "$(dirname "$recipes")"
  git clone --no-checkout https://github.com/ps5-payload-dev/pacbrew-repo.git "$recipes"
  git -C "$recipes" checkout --detach "$pin"
fi
[[ $(git -C "$recipes" rev-parse HEAD) == "$pin" ]] || { echo "Recipe revision mismatch" >&2; exit 1; }
base=${ONEMOUNT_DEPS_BUILD_ROOT:-/opt/duomount-build}
mkdir -p "$base"
for recipe in openlibm zlib libpng libmicrohttpd libjpeg-turbo sqlite libsmb2; do
  [[ ! -f "$base/$recipe/installed.ok" ]] || continue
  (
    echo "Building $recipe"
    mkdir -p "$base/$recipe/src" "$base/$recipe/pkg"
    export srcdir="$base/$recipe/src" pkgdir="$base/$recipe/pkg"
    if [[ "$recipe" == sqlite ]]; then
      # Official 3.46.1 amalgamation, same library version as the upstream recipe.
      # This avoids needing Tcl just to regenerate an already released file.
      cd "$srcdir"
      curl -fsSL --retry 3 --proto '=https' https://www.sqlite.org/2024/sqlite-amalgamation-3460100.zip -o sqlite-amalgamation.zip
      printf '%s  %s\n' 77823cb110929c2bcb0f5d48e4833b5c59a8a6e40cdea3936b99e199dbbe5784 sqlite-amalgamation.zip | sha256sum -c -
      unzip -qo sqlite-amalgamation.zip
      "$PS5_PAYLOAD_SDK/bin/prospero-clang" -O2 -DSQLITE_THREADSAFE=1 -DSQLITE_OMIT_LOAD_EXTENSION \
        -c sqlite-amalgamation-3460100/sqlite3.c -o sqlite3.o
      "$PS5_PAYLOAD_SDK/bin/prospero-ar" rcs "$PS5_PAYLOAD_SDK/target/user/homebrew/lib/libsqlite3.a" sqlite3.o
      cp sqlite-amalgamation-3460100/sqlite3.h sqlite-amalgamation-3460100/sqlite3ext.h "$PS5_PAYLOAD_SDK/target/user/homebrew/include/"
      touch "$base/$recipe/installed.ok"
      exit 0
    fi
    source <(tr -d '\r' < "$recipes/$recipe/PKGBUILD")
    cd "$srcdir"
    for i in "${!source[@]}"; do
      url="${source[$i]}"; file="${url##*/}"
      [[ -f "$file" ]] || curl -fL --retry 3 --proto '=https' "$url" -o "$file"
      sum="${sha256sums[$i]}"
      if [[ "$sum" != SKIP ]]; then printf '%s  %s\n' "$sum" "$file" | sha256sum -c -; fi
      case "$file" in
        *.zip) unzip -qo "$file" ;;
        *) tar -xf "$file" ;;
      esac
    done
    (cd "$srcdir"; prepare; build)
    (cd "$srcdir"; package)
    cp -a "$pkgdir/opt/ps5-payload-sdk/target/." "$PS5_PAYLOAD_SDK/target/"
    echo "$recipe: installed"
    touch "$base/$recipe/installed.ok"
  )
done
