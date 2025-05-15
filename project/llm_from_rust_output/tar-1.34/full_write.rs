use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

pub fn full_write<W: Write + AsRawFd>(mut writer: W, mut buf: &[u8]) -> io::Result<usize> {
    let mut total = 0;
    while !buf.is_empty() {
        match writer.write(buf) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "failed to write whole buffer",
                ));
            }
            Ok(n) => {
                total += n;
                buf = &buf[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(total)
}