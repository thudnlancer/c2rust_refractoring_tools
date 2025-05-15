use std::os::unix::io::AsRawFd;

pub fn rpl_ioctl<T: AsRawFd>(
    fd: &T,
    request: u32,
    buf: Option<&mut [u8]>,
) -> Result<(), std::io::Error> {
    let fd = fd.as_raw_fd();
    let request = request as libc::c_ulong;
    let buf_ptr = buf.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr() as *mut libc::c_void);

    let result = unsafe { libc::ioctl(fd, request, buf_ptr) };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}