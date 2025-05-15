use std::os::unix::io::AsRawFd;

pub fn rpl_ioctl<T: AsRawFd>(
    fd: &T,
    request: u32,
    buf: Option<*mut std::ffi::c_void>,
) -> Result<(), std::io::Error> {
    let res = unsafe {
        libc::ioctl(
            fd.as_raw_fd(),
            request as libc::c_ulong,
            buf.unwrap_or(std::ptr::null_mut()),
        )
    };

    if res == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}