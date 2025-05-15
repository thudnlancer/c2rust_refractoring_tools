use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::os::unix::io::RawFd;

pub fn areadlinkat(fd: RawFd, filename: &Path) -> Result<CString, std::io::Error> {
    let mut buf = Vec::with_capacity(1024);
    
    loop {
        let res = unsafe {
            libc::readlinkat(
                fd,
                CString::new(filename.as_os_str().as_bytes())?.as_ptr(),
                buf.as_mut_ptr() as *mut libc::c_char,
                buf.capacity(),
            )
        };

        if res < 0 {
            return Err(std::io::Error::last_os_error());
        }

        let len = res as usize;
        if len < buf.capacity() {
            unsafe {
                buf.set_len(len);
            }
            return Ok(unsafe { CString::from_vec_unchecked(buf) });
        }

        buf.reserve(buf.capacity() * 2);
    }
}