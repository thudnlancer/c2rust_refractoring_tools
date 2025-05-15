use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

const SYS_BUFSIZE_MAX: usize = 2146435072;

pub fn safe_write<W: Write>(fd: &mut W, buf: &[u8]) -> io::Result<usize> {
    let mut count = buf.len();
    let mut written = 0;
    let raw_fd = fd.as_raw_fd();

    while written < count {
        let chunk = if count - written > SYS_BUFSIZE_MAX {
            &buf[written..written + SYS_BUFSIZE_MAX]
        } else {
            &buf[written..]
        };

        match fd.write(chunk) {
            Ok(n) => written += n,
            Err(e) => {
                if e.kind() == io::ErrorKind::Interrupted {
                    continue;
                } else if e.kind() == io::ErrorKind::InvalidInput && count > SYS_BUFSIZE_MAX {
                    count = SYS_BUFSIZE_MAX;
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }

    Ok(written)
}