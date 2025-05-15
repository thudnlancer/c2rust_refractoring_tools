use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

const SYS_BUFSIZE_MAX: usize = 2146435072;

pub fn safe_read(fd: &impl AsRawFd, buf: &mut [u8]) -> io::Result<usize> {
    let mut count = buf.len();
    let fd = fd.as_raw_fd();
    
    loop {
        let result = unsafe {
            libc::read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                count.min(SYS_BUFSIZE_MAX)
            )
        };

        if result >= 0 {
            return Ok(result as usize);
        }

        let errno = unsafe { *libc::__errno_location() };
        if errno == libc::EINTR {
            continue;
        } else if errno == libc::EINVAL && count > SYS_BUFSIZE_MAX {
            count = SYS_BUFSIZE_MAX;
        } else {
            return Err(io::Error::from_raw_os_error(errno));
        }
    }
}