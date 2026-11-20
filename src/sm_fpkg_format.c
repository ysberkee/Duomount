#include "sm_fpkg.h"
#include <fcntl.h>
#include <string.h>
#include <strings.h>
#include <unistd.h>

bool sm_fpkg_name(const char *name) {
  if (!name) return false;
  size_t size = strlen(name);
  return size > 4u && strcasecmp(name + size - 4u, ".pkg") == 0;
}
bool sm_fpkg_file(const char *path) {
  if (!sm_fpkg_name(path)) return false;
  int fd = open(path, O_RDONLY);
  if (fd < 0) return false;
  unsigned char magic[4] = {0};
  ssize_t count = pread(fd, magic, sizeof(magic), 0);
  close(fd);
  static const unsigned char fih[4] = {0x7f, 'F', 'I', 'H'};
  return count == 4 && memcmp(magic, fih, sizeof(fih)) == 0;
}

bool sm_fpkg_legacy_mount(const char *fstype, const char *from, const char *on) {
  static const char old[] = "/data/krakenmount/caches/";
  static const char target[] = "/system_ex/app/";
  if (!fstype || !from || !on || strcmp(fstype, "nullfs") ||
      strncmp(from, old, sizeof(old) - 1u) || strncmp(on, target, sizeof(target) - 1u)) return false;
  const char *tid = from + sizeof(old) - 1u;
  if (strlen(tid) != 9u || strncmp(tid, "PPSA", 4u) || strcmp(on + sizeof(target) - 1u, tid)) return false;
  for (size_t i = 4u; i < 9u; ++i) if (tid[i] < '0' || tid[i] > '9') return false;
  return true;
}
