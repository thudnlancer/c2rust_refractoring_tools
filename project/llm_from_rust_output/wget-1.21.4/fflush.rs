use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

pub fn clear_ungetc_buffer_preserving_position(file: &mut File) -> std::io::Result<()> {
    let pos = file.stream_position()?;
    file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(pos))?;
    Ok(())
}

pub fn rpl_fflush(stream: Option<&mut File>) -> std::io::Result<()> {
    let stream = match stream {
        Some(f) => f,
        None => return Ok(()),
    };

    if !is_reading(stream) {
        return stream.flush();
    }

    clear_ungetc_buffer_preserving_position(stream)?;
    stream.flush()
}

fn is_reading(file: &File) -> bool {
    use libc::{c_int, c_void, fcntl, F_GETFL, O_ACCMODE, O_RDONLY, O_RDWR, O_WRONLY};
    
    unsafe {
        let flags = fcntl(file.as_raw_fd(), F_GETFL);
        if flags == -1 {
            return false;
        }
        match flags & O_ACCMODE {
            O_RDONLY | O_RDWR => true,
            O_WRONLY => false,
            _ => false,
        }
    }
}