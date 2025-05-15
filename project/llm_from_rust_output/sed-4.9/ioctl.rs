use std::os::fd::AsRawFd;
use std::io;

pub fn rpl_ioctl<F: AsRawFd>(
    fd: &F,
    request: i32,
    buf: Option<&mut [u8]>,
) -> io::Result<i32> {
    let fd_raw = fd.as_raw_fd();
    let request_ulong = request as u32 as libc::c_ulong;
    let buf_ptr = buf.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr() as *mut libc::c_void);

    let result = unsafe { libc::ioctl(fd_raw, request_ulong, buf_ptr) };
    
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}