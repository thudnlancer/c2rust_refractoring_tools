use std::io::{self, Seek, SeekFrom, Write};
use std::fs::File;

fn clear_ungetc_buffer_preserving_position(file: &mut File) -> io::Result<()> {
    let current_pos = file.stream_position()?;
    file.seek(SeekFrom::Current(0))?;
    file.seek(SeekFrom::Start(current_pos))?;
    Ok(())
}

pub fn rpl_fflush(stream: Option<&mut File>) -> io::Result<()> {
    let Some(stream) = stream else {
        return Ok(());
    };

    if stream.metadata()?.permissions().readonly() {
        return Ok(());
    }

    clear_ungetc_buffer_preserving_position(stream)?;
    stream.flush()?;
    Ok(())
}