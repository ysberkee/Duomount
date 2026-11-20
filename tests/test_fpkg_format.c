#include "sm_fpkg.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void sample(const char *path, const char *data, size_t length) {
  FILE *file = fopen(path, "wb");
  assert(file);
  assert(fwrite(data, 1, length, file) == length);
  assert(fclose(file) == 0);
}
int main(void) {
  char root[] = "/tmp/duomount-format-XXXXXX";
  assert(mkdtemp(root));
  char path[512];
  snprintf(path, sizeof(path), "%s/game.pkg", root);
  sample(path, "\177FIH", 4); assert(sm_fpkg_file(path));
  sample(path, "\177CNT", 4); assert(!sm_fpkg_file(path));
  sample(path, "\177FI", 3); assert(!sm_fpkg_file(path));
  sample(path, "", 0); assert(!sm_fpkg_file(path));
  assert(unlink(path) == 0);
  assert(!sm_fpkg_file(path));
  snprintf(path, sizeof(path), "%s/game.pkg.1-130.part", root);
  sample(path, "\177FIH", 4); assert(!sm_fpkg_file(path));
  assert(unlink(path) == 0);
  assert(rmdir(root) == 0);
  assert(sm_fpkg_name("UFC5.PKG"));
  assert(!sm_fpkg_name("PPSA03977.exfat"));
  assert(!sm_fpkg_name("PPSA11386.ffpfsc"));
  assert(!sm_fpkg_name("file.pkg.part"));
  assert(!sm_fpkg_name(NULL));
  assert(sm_fpkg_legacy_mount("nullfs", "/data/krakenmount/caches/PPSA03541", "/system_ex/app/PPSA03541"));
  assert(!sm_fpkg_legacy_mount("unionfs", "/data/krakenmount/caches/PPSA03541", "/system_ex/app/PPSA03541"));
  assert(!sm_fpkg_legacy_mount("nullfs", "/mnt/ext1/homebrew/PPSA03541", "/system_ex/app/PPSA03541"));
  assert(!sm_fpkg_legacy_mount("nullfs", "/data/krakenmount/caches/PPSA03541", "/system_ex/app/PPSA99999"));
  assert(!sm_fpkg_legacy_mount("nullfs", "/data/krakenmount/caches/PPSA03541/extra", "/system_ex/app/PPSA03541"));
  assert(!sm_fpkg_legacy_mount("nullfs", "/data/krakenmount/caches/CUSA03541", "/system_ex/app/CUSA03541"));
  puts("FPKG routing: PS5 FIH accepted, PS4 CNT, partial transfers and image formats excluded: PASS");
}
