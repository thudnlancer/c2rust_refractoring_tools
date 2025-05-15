use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

pub type Result<T> = std::result::Result<T, std::io::Error>;

fn clear_ungetc_buffer_preserving_position(file: &mut File) -> Result<()> {
    let current_pos = file.stream_position()?;
    file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(current_pos))?;
    Ok(())
}

pub fn rpl_fflush(stream: Option<&mut File>) -> Result<()> {
    let stream = match stream {
        Some(s) => s,
        None => return Ok(()),
    };

    if !is_reading(stream) {
        return stream.flush();
    }

    clear_ungetc_buffer_preserving_position(stream)?;
    stream.flush()
}

fn is_reading(file: &File) -> bool {
    use libc::{c_int, fcntl, F_GETFL};
    unsafe {
        let flags = fcntl(file.as_raw_fd(), F_GETFL);
        flags & libc::O_ACCMODE == libc::O_RDONLY
    }
}