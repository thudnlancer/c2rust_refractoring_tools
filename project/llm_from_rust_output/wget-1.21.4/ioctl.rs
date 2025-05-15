use std::os::fd::AsRawFd;
use std::io;

pub fn rpl_ioctl<F: AsRawFd, T>(
    fd: &F,
    request: i32,
    buf: Option<&mut T>,
) -> io::Result<()> {
    let fd_raw = fd.as_raw_fd();
    let buf_ptr = buf.map(|b| b as *mut T as *mut libc::c_void).unwrap_or(std::ptr::null_mut());
    
    unsafe {
        let ret = libc::ioctl(fd_raw, request as libc::c_ulong, buf_ptr);
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}