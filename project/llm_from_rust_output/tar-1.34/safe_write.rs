use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

const SYS_BUFSIZE_MAX: usize = 2146435072;

pub fn safe_write<W: Write>(w: &mut W, buf: &[u8]) -> io::Result<usize> {
    let mut remaining = buf.len();
    let mut written = 0;
    
    while remaining > 0 {
        let chunk_size = if remaining > SYS_BUFSIZE_MAX {
            SYS_BUFSIZE_MAX
        } else {
            remaining
        };
        
        match w.write(&buf[written..written + chunk_size]) {
            Ok(0) => return Ok(written),
            Ok(n) => {
                written += n;
                remaining -= n;
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(written)
}

pub fn safe_write_fd(fd: i32, buf: &[u8]) -> io::Result<usize> {
    use std::os::unix::io::FromRawFd;
    unsafe {
        let mut file = std::fs::File::from_raw_fd(fd);
        let result = safe_write(&mut file, buf);
        std::mem::forget(file); // Prevent closing the file descriptor
        result
    }
}