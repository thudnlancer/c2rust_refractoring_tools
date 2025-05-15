use std::io::{self, Read, Write, ErrorKind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);

pub type MtOffT = i64;

pub trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;
}

pub fn copyfile(
    source: &mut dyn Stream,
    target: &mut dyn Stream,
) -> io::Result<MtOffT> {
    const BUFFER_SIZE: usize = 131072;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut pos: MtOffT = 0;

    loop {
        let bytes_read = match source.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("file read: {}", e);
                return Err(e);
            }
        };

        if GOT_SIGNAL.load(Ordering::Relaxed) {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "Signal received"));
        }

        let mut bytes_written = 0;
        while bytes_written < bytes_read {
            match target.write(&buffer[bytes_written..bytes_read]) {
                Ok(n) => bytes_written += n,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => {
                    if e.kind() == ErrorKind::StorageFull {
                        GOT_SIGNAL.store(true, Ordering::Relaxed);
                    }
                    return Err(e);
                }
            }
        }

        if bytes_written != bytes_read {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                format!("Short write {} instead of {}", bytes_written, bytes_read),
            ));
        }

        pos += bytes_read as MtOffT;
    }

    Ok(pos)
}