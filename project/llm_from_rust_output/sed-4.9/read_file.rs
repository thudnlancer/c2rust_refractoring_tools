use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::fs::FileTypeExt;
use std::path::Path;
use std::ptr;

const READ_CHUNK_SIZE: usize = 8192;
const MODE_REGULAR_FILE: u32 = 0o100000;

pub fn read_file(
    filename: &Path,
    flags: i32,
) -> io::Result<(Vec<u8>, usize)> {
    let mut file = File::open(filename)?;
    
    if flags & 0x2 != 0 {
        // TODO: Implement buffer control equivalent to setvbuf
    }

    let metadata = file.metadata()?;
    let mut alloc_size = READ_CHUNK_SIZE;

    if metadata.file_type().is_file() {
        let current_pos = file.stream_position()?;
        let remaining_size = metadata.len() - current_pos;
        
        if remaining_size > 0 {
            alloc_size = remaining_size as usize + 1;
        }
    }

    let mut buffer = Vec::with_capacity(alloc_size);
    let mut total_read = 0;

    loop {
        let remaining_space = buffer.capacity() - buffer.len();
        if remaining_space == 0 {
            let new_capacity = buffer
                .capacity()
                .checked_add(buffer.capacity() / 2)
                .unwrap_or(usize::MAX);
            
            if new_capacity == buffer.capacity() {
                return Err(io::Error::from_raw_os_error(libc::ENOMEM));
            }
            
            buffer.reserve(new_capacity - buffer.capacity());
            continue;
        }

        let mut chunk = vec![0; remaining_space];
        match file.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&chunk[..n]);
                total_read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }

    if flags & 0x2 != 0 && total_read < buffer.capacity() - 1 {
        buffer.truncate(total_read + 1);
        buffer.shrink_to_fit();
    }

    buffer.push(b'\0');
    Ok((buffer, total_read))
}

pub fn read_file_with_flags(
    filename: &Path,
    flags: i32,
) -> io::Result<(Vec<u8>, usize)> {
    let (mut data, length) = read_file(filename, flags)?;

    if flags & 0x2 != 0 {
        // Secure erase if requested
        for byte in &mut data {
            *byte = 0;
        }
    }

    Ok((data, length))
}