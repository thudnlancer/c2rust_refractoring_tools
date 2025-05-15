use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io;

fn stdopen() -> io::Result<()> {
    for fd in 0..=2 {
        if let Err(_) = nix::fcntl::fcntl(fd, nix::fcntl::F_GETFD) {
            let mode = if fd == 0 {
                OpenOptions::new().write(true)
            } else {
                OpenOptions::new()
            };
            
            let file = match File::open("/dev/full") {
                Ok(f) => f,
                Err(_) => File::open("/dev/null")?,
            };
            
            let new_fd = file.as_raw_fd();
            if new_fd > 2 {
                drop(file);
                return Ok(());
            }
            
            if new_fd != fd {
                nix::unistd::dup2(new_fd, fd)?;
                drop(file);
            }
        }
    }
    Ok(())
}