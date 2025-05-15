use std::io;
use std::os::unix::io::AsRawFd;

pub trait DirFd {
    fn dirfd(&self) -> io::Result<i32>;
}

impl DirFd for std::fs::File {
    fn dirfd(&self) -> io::Result<i32> {
        let fd = self.as_raw_fd();
        if fd == -1 {
            Err(io::Error::from_raw_os_error(95))
        } else {
            Ok(fd)
        }
    }
}