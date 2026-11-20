#ifndef SM_FPKG_H
#define SM_FPKG_H
#include <stdbool.h>
#include <stddef.h>
#include "sm_limits.h"

/* PS5 FIH debug packages only; normal PS4 CNT packages remain installer input. */
bool sm_fpkg_file(const char *path);
bool sm_fpkg_name(const char *name);
bool sm_fpkg_legacy_mount(const char *fstype, const char *from, const char *on);
bool sm_fpkg_release_legacy_mounts(void);
bool sm_fpkg_prepare(const char *path, char cache_path[MAX_PATH], bool *unstable);
bool sm_fpkg_cache_path(const char *source, char cache_path[MAX_PATH]);
bool sm_fpkg_source_path(const char *cache_path, char source[MAX_PATH]);
bool sm_fpkg_manages_title(const char *title_id);
bool sm_fpkg_validate_cache(const char *cache_path);
void sm_fpkg_track_title(const char *title_id, const char *cache_path);
void sm_fpkg_start(void);
void sm_fpkg_stop(void);
bool sm_fpkg_wait_cleanup(void);
void sm_fpkg_bridge_log(const char *message);
void sm_fpkg_bridge_snd0(const char *title_id);
#endif
