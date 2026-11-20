#include "sm_platform.h"
#include "sm_fpkg.h"
#include "sm_appdb.h"
#include "sm_config_mount.h"
#include "sm_filesystem.h"
#include "sm_game_lifecycle.h"
#include "sm_log.h"
#include "sm_mount_device.h"
#include "sm_mount_diag.h"
#include "sm_runtime.h"
#include "sm_scan.h"
#include "sm_stability.h"

/* Rust static library ABI. There is no independent payload process or server. */
int om_fp_prepare(const char *, const char *, int, char *, size_t, char *, size_t);
int om_fp_cache(const char *, char *, size_t);
int om_fp_source(const char *, char *, size_t);
void om_fp_track(const char *, const char *);
void om_fp_start(void);
void om_fp_stop(void);
int om_fp_running(void);

bool sm_fpkg_cache_path(const char *source, char cache_path[MAX_PATH]) {
  cache_path[0] = 0;
  int result = om_fp_cache(source, cache_path, MAX_PATH);
  if (result != 0) errno = result;
  return result == 0;
}

bool sm_fpkg_source_path(const char *cache_path, char source[MAX_PATH]) {
  source[0] = 0;
  return om_fp_source(cache_path, source, MAX_PATH) == 0;
}

bool sm_fpkg_prepare(const char *path, char cache_path[MAX_PATH], bool *unstable) {
  if (unstable) *unstable = false;
  cache_path[0] = 0;
  /* Cache builds are deferred while any title is active, including an image title. */
  if (sm_game_lifecycle_has_active_game() || should_stop_requested() || runtime_sleep_mode_active()) {
    errno = EBUSY;
    if (unstable) *unstable = true;
    return false;
  }
  const char *filename = strrchr(path, '/');
  filename = filename ? filename + 1 : path;
  if (!is_source_stable_for_mount(path, filename, "FPKG")) {
    if (unstable) *unstable = true;
    return false;
  }
  char preliminary[MAX_PATH], backport[MAX_PATH] = {0};
  if (!sm_fpkg_cache_path(path, preliminary)) return false;
  const char *tid = strrchr(preliminary, '/');
  if (!tid || !tid[1]) { errno = EINVAL; return false; }
  (void)resolve_backport_path_for_title(tid + 1, NULL, backport);
  char error[512] = {0};
  int status = om_fp_prepare(path, backport[0] ? backport : NULL,
      runtime_config().update_emulators_enabled ? 1 : 0,
      cache_path, MAX_PATH, error, sizeof(error));
  if (status) {
    log_debug("  [FPKG] prepare failed: %s: %s", path, error[0] ? error : strerror(status));
    errno = status;
    return false;
  }
  return true;
}

bool sm_fpkg_manages_title(const char *title_id) {
  char cache_path[MAX_PATH], source[MAX_PATH];
  return read_mount_link(title_id, cache_path, sizeof(cache_path)) &&
      sm_fpkg_source_path(cache_path, source);
}

bool sm_fpkg_validate_cache(const char *cache_path) {
  char source[MAX_PATH];
  if (!sm_fpkg_source_path(cache_path, source)) return true;
  struct stat st;
  if (stat(source, &st) == 0 && S_ISREG(st.st_mode)) return true;
  log_debug("  [FPKG] source unavailable for %s: %s", cache_path, source);
  errno = ENOENT;
  return false;
}

void sm_fpkg_track_title(const char *title_id, const char *cache_path) { om_fp_track(title_id, cache_path); }
void sm_fpkg_start(void) { om_fp_start(); }
void sm_fpkg_stop(void) { om_fp_stop(); }
bool sm_fpkg_release_legacy_mounts(void) {
  if (sm_game_lifecycle_has_active_game()) { errno = EBUSY; return false; }
  struct statfs *mounts = NULL;
  int count = sm_mount_table_snapshot(&mounts);
  if (count < 0) return false;
  bool complete = true;
  for (int i = count - 1; i >= 0; --i) {
    if (!sm_fpkg_legacy_mount(mounts[i].f_fstypename, mounts[i].f_mntfromname, mounts[i].f_mntonname)) continue;
    if (sm_game_lifecycle_has_active_game()) { complete = false; break; }
    if (unmount(mounts[i].f_mntonname, 0) != 0) {
      log_debug("  [FPKG] legacy cache mount retained: %s (%s)", mounts[i].f_mntonname, strerror(errno));
      complete = false;
    } else {
      log_debug("  [FPKG] released old Kraken cache mount: %s", mounts[i].f_mntonname);
    }
  }
  free(mounts);
  return complete;
}
bool sm_fpkg_wait_cleanup(void) {
  for (unsigned i = 0; i < 80u && om_fp_running(); ++i) usleep(250000);
  return om_fp_running() == 0;
}
void sm_fpkg_bridge_log(const char *message) { log_debug("  [FPKG] %s", message ? message : ""); }
void sm_fpkg_bridge_snd0(const char *title_id) { (void)update_snd0info(title_id); }
