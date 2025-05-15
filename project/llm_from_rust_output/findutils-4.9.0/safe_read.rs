use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

const SYS_BUFSIZE_MAX: usize = 2146435072;

pub fn safe_read(fd: &impl AsRawFd, buf: &mut [u8]) -> io::Result<usize> {
    let mut count = buf.len();
    let fd = fd.as_raw_fd();
    
    loop {
        let result = unsafe {
            libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, count)
        };

        if result >= 0 {
            return Ok(result as usize);
        } else {
            let errno = io::Error::last_os_error();
            if errno.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            if errno.raw_os_error() == Some(libc::EINVAL) && SYS_BUFSIZE_MAX < count {
                count = SYS_BUFSIZE_MAX;
                continue;
            }
            return Err(errno);
        }
    }
}