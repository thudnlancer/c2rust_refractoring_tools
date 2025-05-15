use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use libc::{iconv_t, iconv, iconv_open, iconv_close, size_t};
use std::error::Error;
use std::fmt;
use std::str;

#[derive(Debug)]
pub enum CharsetError {
    ConversionFailed,
    InvalidInput,
    OutOfMemory,
    Unsupported,
    Unknown,
}

impl fmt::Display for CharsetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharsetError::ConversionFailed => write!(f, "Conversion failed"),
            CharsetError::InvalidInput => write!(f, "Invalid input"),
            CharsetError::OutOfMemory => write!(f, "Out of memory"),
            CharsetError::Unsupported => write!(f, "Unsupported operation"),
            CharsetError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Error for CharsetError {}

const TMP_BUF_SIZE: usize = 4096;

pub fn mem_cd_iconv(
    src: &[u8],
    cd: iconv_t,
) -> Result<Vec<u8>, CharsetError> {
    unsafe {
        // Reset iconv state
        iconv(cd, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());

        // First pass: determine output size
        let mut tmp_buf = [0u8; TMP_BUF_SIZE];
        let mut in_ptr = src.as_ptr() as *const c_char;
        let mut in_bytes = src.len();
        let mut count = 0;

        while in_bytes > 0 {
            let mut out_ptr = tmp_buf.as_mut_ptr() as *mut c_char;
            let mut out_bytes = TMP_BUF_SIZE;

            let res = iconv(
                cd,
                &mut in_ptr,
                &mut in_bytes,
                &mut out_ptr,
                &mut out_bytes,
            );

            if res == (size_t::MAX) {
                match std::io::Error::last_os_error().raw_os_error() {
                    Some(libc::E2BIG) => (),
                    Some(libc::EINVAL) => break,
                    _ => return Err(CharsetError::ConversionFailed),
                }
            }
            count += (out_ptr as usize - tmp_buf.as_ptr() as usize);
        }

        // Finalize conversion
        let mut out_ptr = tmp_buf.as_mut_ptr() as *mut c_char;
        let mut out_bytes = TMP_BUF_SIZE;
        let res = iconv(cd, ptr::null_mut(), ptr::null_mut(), &mut out_ptr, &mut out_bytes);
        if res == (size_t::MAX) {
            return Err(CharsetError::ConversionFailed);
        }
        count += (out_ptr as usize - tmp_buf.as_ptr() as usize);

        if count == 0 {
            return Ok(Vec::new());
        }

        // Second pass: actual conversion
        let mut result = Vec::with_capacity(count);
        let mut in_ptr = src.as_ptr() as *const c_char;
        let mut in_bytes = src.len();
        let mut out_ptr = result.as_mut_ptr() as *mut c_char;
        let mut out_bytes = result.capacity();

        // Reset iconv state again
        iconv(cd, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());

        while in_bytes > 0 {
            let res = iconv(
                cd,
                &mut in_ptr,
                &mut in_bytes,
                &mut out_ptr,
                &mut out_bytes,
            );

            if res == (size_t::MAX) {
                match std::io::Error::last_os_error().raw_os_error() {
                    Some(libc::EINVAL) => break,
                    _ => return Err(CharsetError::ConversionFailed),
                }
            }
        }

        // Finalize conversion
        let res = iconv(cd, ptr::null_mut(), ptr::null_mut(), &mut out_ptr, &mut out_bytes);
        if res == (size_t::MAX) {
            return Err(CharsetError::ConversionFailed);
        }

        unsafe {
            result.set_len(count);
        }

        Ok(result)
    }
}

pub fn str_cd_iconv(src: &str, cd: iconv_t) -> Result<String, CharsetError> {
    let src_bytes = src.as_bytes();
    let mut result = mem_cd_iconv(src_bytes, cd)?;
    result.push(b'\0'); // Add null terminator
    CStr::from_bytes_with_nul(&result)
        .map_err(|_| CharsetError::InvalidInput)?
        .to_str()
        .map_err(|_| CharsetError::InvalidInput)
        .map(|s| s.to_owned())
}

pub fn str_iconv(
    src: &str,
    from_codeset: &str,
    to_codeset: &str,
) -> Result<String, CharsetError> {
    if src.is_empty() || from_codeset.eq_ignore_ascii_case(to_codeset) {
        return Ok(src.to_owned());
    }

    unsafe {
        let to_codeset_c = CString::new(to_codeset).map_err(|_| CharsetError::InvalidInput)?;
        let from_codeset_c = CString::new(from_codeset).map_err(|_| CharsetError::InvalidInput)?;

        let cd = iconv_open(to_codeset_c.as_ptr(), from_codeset_c.as_ptr());
        if cd == (iconv_t::MAX as iconv_t) {
            return Err(CharsetError::Unsupported);
        }

        let result = str_cd_iconv(src, cd);
        iconv_close(cd);
        result
    }
}