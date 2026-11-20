//! Small C ABI buffer helpers, also exercised on the host.
use std::ffi::{c_char, CStr};
use std::io::{self, Error, ErrorKind};

pub unsafe fn string_arg<'a>(p: *const c_char) -> io::Result<&'a str> {
    if p.is_null() { return Err(Error::new(ErrorKind::InvalidInput, "null string")); }
    CStr::from_ptr(p).to_str().map_err(|_| Error::new(ErrorKind::InvalidInput, "non-UTF-8 path"))
}
pub unsafe fn put(value: &str, out: *mut c_char, cap: usize) -> io::Result<()> {
    if out.is_null() || cap == 0 || value.len() >= cap || value.as_bytes().contains(&0) {
        return Err(Error::new(ErrorKind::InvalidInput, "output buffer too small"));
    }
    std::ptr::copy_nonoverlapping(value.as_ptr(), out.cast(), value.len());
    *out.add(value.len()) = 0;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn null_invalid_utf8_and_unicode() {
        unsafe {
            assert!(string_arg(std::ptr::null()).is_err());
            assert!(string_arg([255u8, 0].as_ptr().cast()).is_err());
            assert_eq!(string_arg(b"/mnt/ext1/GTA.pkg\0".as_ptr().cast()).unwrap(), "/mnt/ext1/GTA.pkg");
        }
    }
    #[test]
    fn exact_capacity_with_terminator_and_canaries() {
        let mut bytes = [0x55u8; 8];
        unsafe { put("abc", bytes.as_mut_ptr().add(1).cast(), 4).unwrap(); }
        assert_eq!(bytes, [0x55, b'a', b'b', b'c', 0, 0x55, 0x55, 0x55]);
    }
    #[test]
    fn failed_writes_do_not_modify_the_buffer() {
        let mut bytes = [0x55u8; 8];
        unsafe {
            assert!(put("abcd", bytes.as_mut_ptr().cast(), 4).is_err());
            assert!(put("x", bytes.as_mut_ptr().cast(), 0).is_err());
            assert!(put("a\0b", bytes.as_mut_ptr().cast(), 8).is_err());
            assert!(put("x", std::ptr::null_mut(), 8).is_err());
        }
        assert_eq!(bytes, [0x55; 8]);
    }
}
