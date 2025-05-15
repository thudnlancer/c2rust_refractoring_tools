use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::os::unix::io::RawFd;

pub fn areadlinkat(fd: RawFd, filename: &Path) -> Result<CString, std::io::Error> {
    let filename_c = CString::new(filename.as_os_str().as_bytes())?;
    
    let mut buffer = Vec::with_capacity(1024);
    loop {
        let res = unsafe {
            libc::readlinkat(
                fd,
                filename_c.as_ptr(),
                buffer.as_mut_ptr() as *mut libc::c_char,
                buffer.capacity(),
            )
        };
        
        if res < 0 {
            return Err(std::io::Error::last_os_error());
        }
        
        let res = res as usize;
        if res < buffer.capacity() {
            unsafe {
                buffer.set_len(res);
            }
            return Ok(CString::new(buffer).unwrap());
        }
        
        buffer.reserve(buffer.capacity() * 2);
    }
}