use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::mem;
use libc::{iconv_t, size_t, iconv, iconv_open, iconv_close, strdup, strlen, c_strcasecmp};
use std::io::Error;

const EINVAL: c_int = 22;
const E2BIG: c_int = 7;
const ENOMEM: c_int = 12;

struct IconvHandle {
    handle: iconv_t,
}

impl IconvHandle {
    fn new(to_code: &CStr, from_code: &CStr) -> Result<Self, Error> {
        let handle = unsafe { iconv_open(to_code.as_ptr(), from_code.as_ptr()) };
        if handle == (-1isize) as iconv_t {
            Err(Error::last_os_error())
        } else {
            Ok(Self { handle })
        }
    }

    fn convert(&self, src: &[u8]) -> Result<Vec<u8>, Error> {
        let mut in_buf = src.as_ptr() as *mut c_char;
        let mut in_bytes_left = src.len() as size_t;
        
        // First pass to calculate required output size
        let mut out_buf = ptr::null_mut();
        let mut out_bytes_left = 0;
        let mut count = 0;
        
        unsafe {
            iconv(self.handle, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
            
            let mut tmp_buf = [0u8; 4096];
            while in_bytes_left > 0 {
                let mut tmp_ptr = tmp_buf.as_mut_ptr() as *mut c_char;
                let mut tmp_size = tmp_buf.len() as size_t;
                
                let res = iconv(
                    self.handle,
                    &mut in_buf,
                    &mut in_bytes_left,
                    &mut tmp_ptr,
                    &mut tmp_size,
                );
                
                if res == (-1isize) as size_t {
                    let err = Error::last_os_error();
                    if err.raw_os_error() != Some(E2BIG) {
                        if err.raw_os_error() == Some(EINVAL) {
                            break;
                        }
                        return Err(err);
                    }
                }
                count += (tmp_ptr as usize - tmp_buf.as_ptr() as usize) as size_t;
            }
            
            let mut tmp_ptr = tmp_buf.as_mut_ptr() as *mut c_char;
            let mut tmp_size = tmp_buf.len() as size_t;
            let res = iconv(
                self.handle,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut tmp_ptr,
                &mut tmp_size,
            );
            
            if res == (-1isize) as size_t {
                return Err(Error::last_os_error());
            }
            count += (tmp_ptr as usize - tmp_buf.as_ptr() as usize) as size_t;
        }
        
        if count == 0 {
            return Ok(Vec::new());
        }
        
        // Second pass to actually perform conversion
        let mut output = Vec::with_capacity(count as usize);
        let mut out_ptr = output.as_mut_ptr() as *mut c_char;
        let mut out_bytes_left = output.capacity() as size_t;
        
        unsafe {
            iconv(self.handle, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
            
            let mut in_buf = src.as_ptr() as *mut c_char;
            let mut in_bytes_left = src.len() as size_t;
            
            loop {
                if in_bytes_left == 0 {
                    break;
                }
                
                let res = iconv(
                    self.handle,
                    &mut in_buf,
                    &mut in_bytes_left,
                    &mut out_ptr,
                    &mut out_bytes_left,
                );
                
                if res == (-1isize) as size_t {
                    let err = Error::last_os_error();
                    if err.raw_os_error() == Some(EINVAL) {
                        break;
                    }
                    return Err(err);
                }
            }
            
            let res = iconv(
                self.handle,
                ptr::null_mut(),
                ptr::null_mut(),
                &mut out_ptr,
                &mut out_bytes_left,
            );
            
            if res == (-1isize) as size_t {
                return Err(Error::last_os_error());
            }
            
            if out_bytes_left != 0 {
                panic!("Output buffer not fully used");
            }
            
            output.set_len(count as usize);
        }
        
        Ok(output)
    }
}

impl Drop for IconvHandle {
    fn drop(&mut self) {
        unsafe {
            iconv_close(self.handle);
        }
    }
}

pub fn str_iconv(
    src: &CStr,
    from_codeset: &CStr,
    to_codeset: &CStr,
) -> Result<CString, Error> {
    if src.to_bytes().is_empty() || unsafe { c_strcasecmp(from_codeset.as_ptr(), to_codeset.as_ptr()) } == 0 {
        let result = unsafe { strdup(src.as_ptr()) };
        if result.is_null() {
            Err(Error::from_raw_os_error(ENOMEM))
        } else {
            Ok(unsafe { CString::from_raw(result) })
        }
    } else {
        let cd = IconvHandle::new(to_codeset, from_codeset)?;
        let result = cd.convert(src.to_bytes())?;
        Ok(CString::new(result).unwrap())
    }
}

pub fn str_cd_iconv(src: &CStr, cd: &IconvHandle) -> Result<CString, Error> {
    let result = cd.convert(src.to_bytes())?;
    Ok(CString::new(result).unwrap())
}

pub fn mem_cd_iconv(
    src: &[u8],
    cd: &IconvHandle,
) -> Result<Vec<u8>, Error> {
    cd.convert(src)
}