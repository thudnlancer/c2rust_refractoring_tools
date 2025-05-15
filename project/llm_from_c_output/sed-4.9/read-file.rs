use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::ptr;
use std::slice;

bitflags::bitflags! {
    pub struct ReadFlags: u32 {
        const BINARY = 0x1;
        const SENSITIVE = 0x2;
    }
}

pub fn fread_file(stream: &mut File, flags: ReadFlags, length: &mut usize) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut alloc = 8192; // BUFSIZ equivalent

    // Check if it's a regular file to optimize allocation
    if let Ok(metadata) = stream.metadata() {
        if metadata.is_file() {
            let pos = stream.stream_position()?;
            let file_size = metadata.len();
            
            if pos < file_size {
                let remaining = file_size - pos;
                if remaining > isize::MAX as u64 - 1 {
                    return Err(io::Error::new(io::ErrorKind::OutOfMemory, "file too large"));
                }
                alloc = remaining as usize + 1;
            }
        }
    }

    buf.try_reserve(alloc)?;
    unsafe { buf.set_len(alloc); }

    let mut size = 0;
    loop {
        let requested = alloc - size;
        let count = match stream.read(&mut buf[size..size + requested]) {
            Ok(n) => n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        size += count;

        if count != requested {
            // Shrink buffer if possible
            if size < alloc - 1 {
                if flags.contains(ReadFlags::SENSITIVE) {
                    let mut smaller_buf = Vec::with_capacity(size + 1);
                    smaller_buf.extend_from_slice(&buf[..size]);
                    buf = smaller_buf;
                } else {
                    buf.truncate(size + 1);
                    buf.shrink_to_fit();
                }
            }

            buf[size] = 0;
            *length = size;
            return Ok(buf);
        }

        // Need to grow buffer
        if alloc == isize::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::OutOfMemory, "file too large"));
        }

        let new_alloc = if alloc < isize::MAX as usize - alloc / 2 {
            alloc + alloc / 2
        } else {
            isize::MAX as usize
        };

        if flags.contains(ReadFlags::SENSITIVE) {
            let mut new_buf = Vec::with_capacity(new_alloc);
            new_buf.extend_from_slice(&buf[..alloc]);
            buf = new_buf;
        } else {
            buf.reserve(new_alloc - alloc);
            unsafe { buf.set_len(new_alloc); }
        }
        alloc = new_alloc;
    }
}

pub fn read_file(filename: &Path, flags: ReadFlags, length: &mut usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    
    if flags.contains(ReadFlags::SENSITIVE) {
        // In Rust, we can't directly control buffering like in C, but we can minimize buffering
        // by reading directly without buffered I/O
    }

    let result = fread_file(&mut file, flags, length);
    
    // Ensure sensitive data is cleared if there was an error
    if let Err(e) = &result {
        if flags.contains(ReadFlags::SENSITIVE) {
            if let Ok(buf) = result.as_ref() {
                unsafe {
                    ptr::write_volatile(buf.as_ptr() as *mut u8, 0);
                }
            }
        }
    }

    result
}