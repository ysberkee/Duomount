//! Embedded PS5 FPKG reader/runtime for the ShadowMountPlus C host.
//! No second ELF, HTTP server, title registrar or standalone payload is started.
#![allow(dead_code)]
#[macro_use]
mod sys;
mod ampr_index;
mod cache;
mod config;
mod i18n;
mod launch;
mod scan;
mod storage;
mod ffi;
use ffi::{put, string_arg};

use std::collections::{HashMap, HashSet};
use std::ffi::{c_char, CString};
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

pub const DATA_DIR: &str = "/data/shadowmount/fpkg";
const AMPR: &[u8] = include_bytes!("../vendor/ampr-emu-ng/libSceAmpr.sprx");
static WORK: Mutex<()> = Mutex::new(());
static PATHS: OnceLock<Mutex<HashMap<PathBuf, PathBuf>>> = OnceLock::new();
static TITLES: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();
static RUNNING: OnceLock<Arc<Mutex<HashSet<String>>>> = OnceLock::new();
static WATCHER: std::sync::Once = std::sync::Once::new();
static STOPPING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static WATCHER_HANDLE: Mutex<Option<std::thread::JoinHandle<()>>> = Mutex::new(None);

extern "C" {
    fn sm_fpkg_bridge_log(message: *const c_char);
    fn sm_fpkg_bridge_snd0(title_id: *const c_char);
}

mod register {
    use super::*;
    pub(crate) fn fnv1a(bytes: &[u8]) -> String {
        let h = bytes.iter().fold(0xcbf29ce484222325u64, |h, b| (h ^ *b as u64).wrapping_mul(0x100000001b3));
        format!("{h:016x}")
    }
    pub fn unmount_title(tid: &str) -> io::Result<()> {
        // ponytail: cache purge belongs to the C host's controlled mount teardown.
        Err(Error::new(ErrorKind::Unsupported, format!("{tid}: use the host's cache cleanup")))
    }
    pub fn spawn_snd0info(tid: &str, delay: u64) {
        std::thread::sleep(std::time::Duration::from_millis(delay));
        if let Ok(tid) = CString::new(tid) { unsafe { sm_fpkg_bridge_snd0(tid.as_ptr()) }; }
    }
}

pub(crate) fn parse_version(v: &str) -> Option<(u32, u32, u32)> {
    let mut it = v.trim().trim_start_matches('v').split('.').map(|p| p.parse::<u32>().ok());
    let r = (it.next()??, it.next()??, it.next()??);
    it.next().is_none().then_some(r)
}
pub(crate) fn newer(a: Option<&str>, b: Option<&str>) -> bool { a.and_then(parse_version) > b.and_then(parse_version) }

pub(crate) fn host_log(line: &str) {
    if let Ok(message) = CString::new(line.replace('\0', "?")) {
        unsafe { sm_fpkg_bridge_log(message.as_ptr()) };
    }
}

fn paths() -> &'static Mutex<HashMap<PathBuf, PathBuf>> { PATHS.get_or_init(|| Mutex::new(HashMap::new())) }
fn titles() -> Arc<Mutex<HashSet<String>>> { TITLES.get_or_init(|| Arc::new(Mutex::new(HashSet::new()))).clone() }
fn running() -> Arc<Mutex<HashSet<String>>> { RUNNING.get_or_init(|| Arc::new(Mutex::new(HashSet::new()))).clone() }

unsafe fn answer(result: io::Result<String>, out: *mut c_char, cap: usize, error: *mut c_char, error_cap: usize) -> i32 {
    let result = result.and_then(|s| put(&s, out, cap));
    match result {
        Ok(()) => 0,
        Err(e) => {
            if !out.is_null() && cap > 0 { *out = 0; }
            let _ = put(&e.to_string(), error, error_cap);
            e.raw_os_error().unwrap_or(libc::EINVAL)
        }
    }
}

fn initialize() -> io::Result<()> {
    for dir in [DATA_DIR, cache::CACHES, cache::AMPR_DIR] { fs::create_dir_all(dir)?; }
    let target = Path::new(cache::AMPR_DIR).join("libSceAmpr.sprx");
    // A manually supplied runtime is never overwritten. The bundled release is
    // verified by SHA-256 during the build and is installed only when absent.
    if !target.exists() {
        let tmp = target.with_extension("tmp");
        fs::write(&tmp, AMPR)?;
        fs::rename(&tmp, &target)?;
    }
    Ok(())
}

fn cache_for(path: &Path) -> io::Result<PathBuf> {
    if let Some(cache) = paths().lock().unwrap().get(path).cloned() { return Ok(cache); }
    let (tid, _, _) = cache::peek_pkg(path)?;
    if !tid.starts_with("PPSA") { return Err(Error::new(ErrorKind::InvalidData, "not a PPSA PS5 FPKG")); }
    let dir = cache::dir(&tid);
    paths().lock().unwrap().insert(path.to_owned(), dir.clone());
    Ok(dir)
}

fn prepare(path: &Path, backport: Option<PathBuf>, emulators: bool) -> io::Result<String> {
    let _work = WORK.lock().unwrap();
    initialize()?;
    let (title_id, title_name, version) = cache::peek_pkg(path)?;
    if !title_id.starts_with("PPSA") { return Err(Error::new(ErrorKind::InvalidData, "only PPSA PS5 FPKGs are supported")); }
    if let Some(previous) = cache::pkg_source(&title_id) {
        if Path::new(&previous) != path && Path::new(&previous).is_file() {
            // Same title copied onto two drives: keep the selected live source.
            // Once the old drive is removed, the normal Moved path relinks the
            // cache to the internal copy. Never rebuild it twice every scan.
            let dir = cache::dir(&title_id);
            if dir.join("sce_sys/param.json").is_file() && dir.join("eboot.bin").is_file() {
                paths().lock().unwrap().insert(path.to_owned(), dir.clone());
                return Ok(dir.to_string_lossy().into_owned());
            }
        }
    }
    let src = scan::Source { path:path.to_owned(), kind:scan::Kind::Pkg, title_id, title_name, version,
        drive:storage::drive_of(path), size:fs::metadata(path)?.len(), backport };
    let settings = config::Settings { inject_ampr:true, add_emulators:emulators, ampr_auto_update:false,
        auto_remove_missing:false, ..config::Settings::default() };
    match cache::check(&src, &settings) {
        cache::Fresh::Fresh => {},
        cache::Fresh::Fakelib(_) => cache::refresh_fakelib(&src, &settings)?,
        cache::Fresh::Moved(from) => cache::relink(&src, &settings, &from)?,
        cache::Fresh::Missing | cache::Fresh::Stale(_) => cache::build(&src, &settings)?,
    }
    cache::fix_access(&src);
    let dir = cache::dir(&src.title_id);
    paths().lock().unwrap().insert(path.to_owned(), dir.clone());
    Ok(dir.to_string_lossy().into_owned())
}

#[no_mangle]
pub unsafe extern "C" fn om_fp_prepare(path:*const c_char, backport:*const c_char, emulators:i32,
    out:*mut c_char, cap:usize, error:*mut c_char, error_cap:usize) -> i32 {
    let result = string_arg(path).and_then(|p| {
        let backport = if backport.is_null() { None } else { Some(PathBuf::from(string_arg(backport)?)) };
        prepare(Path::new(p), backport, emulators != 0)
    });
    answer(result, out, cap, error, error_cap)
}

#[no_mangle]
pub unsafe extern "C" fn om_fp_cache(path:*const c_char, out:*mut c_char, cap:usize) -> i32 {
    answer(string_arg(path).and_then(|p| cache_for(Path::new(p))).map(|p| p.to_string_lossy().into_owned()), out, cap, std::ptr::null_mut(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn om_fp_source(cache_dir:*const c_char, out:*mut c_char, cap:usize) -> i32 {
    let result = string_arg(cache_dir).and_then(|p| {
        let dir = Path::new(p);
        let tid = dir.file_name().and_then(|s|s.to_str()).unwrap_or_default();
        if !dir.starts_with(cache::CACHES) || dir.parent() != Some(Path::new(cache::CACHES)) || !scan::valid_tid(tid) {
            return Err(Error::new(ErrorKind::InvalidInput, "not an FPKG cache"));
        }
        let done:serde_json::Value = serde_json::from_slice(&fs::read(dir.join(cache::DONE))?)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let source = done["source"].as_str().ok_or_else(||Error::new(ErrorKind::InvalidData,"cache has no source"))?;
        if done["kind"] != "pkg" || !source.starts_with('/') || !source.to_ascii_lowercase().ends_with(".pkg") {
            return Err(Error::new(ErrorKind::InvalidData,"not a package source"));
        }
        Ok(source.to_owned())
    });
    answer(result, out, cap, std::ptr::null_mut(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn om_fp_track(title:*const c_char, cache_dir:*const c_char) {
    let (Ok(tid), Ok(dir)) = (string_arg(title), string_arg(cache_dir)) else {return};
    if !scan::valid_tid(tid) || !tid.starts_with("PPSA") { return; }
    let expected = cache::dir(tid);
    let shared = titles();
    let mut set = shared.lock().unwrap();
    if Path::new(dir) == expected { set.insert(tid.to_owned()); }
    else { set.remove(tid); }
}

#[no_mangle]
pub extern "C" fn om_fp_start() {
    WATCHER.call_once(|| {
        let shared = titles();
        if let Ok(entries) = fs::read_dir(cache::CACHES) {
            for entry in entries.flatten() {
                let tid = entry.file_name().to_string_lossy().into_owned();
                if !scan::valid_tid(&tid) { continue; }
                if fs::read_to_string(format!("/user/app/{tid}/mount.lnk")).is_ok_and(|p|p.trim() == entry.path().to_string_lossy()) {
                    shared.lock().unwrap().insert(tid);
                }
            }
        }
        let live = running();
        *WATCHER_HANDLE.lock().unwrap() = Some(std::thread::spawn(move || launch::watch(shared, live)));
    });
}

#[no_mangle]
pub extern "C" fn om_fp_running() -> i32 { i32::from(!running().lock().unwrap().is_empty()) }

pub(crate) fn runtime_stopping() -> bool { STOPPING.load(std::sync::atomic::Ordering::SeqCst) }

#[no_mangle]
pub extern "C" fn om_fp_stop() {
    STOPPING.store(true, std::sync::atomic::Ordering::SeqCst);
    if let Some(handle) = WATCHER_HANDLE.lock().unwrap().take() { let _ = handle.join(); }
}
