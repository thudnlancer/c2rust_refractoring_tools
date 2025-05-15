use std::fs::File;
use std::io::{Seek, SeekFrom, Error, ErrorKind};
use std::os::unix::io::AsRawFd;

pub fn fseeko(file: &mut File, offset: i64, whence: i32) -> Result<(), Error> {
    let seek_from = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return Err(Error::new(ErrorKind::InvalidInput, "invalid whence")),
    };

    // Check if the file is seekable (lseek equivalent)
    let fd = file.as_raw_fd();
    let current_pos = unsafe { libc::lseek(fd, 0, libc::SEEK_CUR) };
    if current_pos == -1 {
        return Err(Error::last_os_error());
    }

    // Perform the seek operation
    match file.seek(seek_from) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Clear error flags similar to original C code
            // Note: In Rust, the error state is handled through Result
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Write, Read};
    use tempfile::tempfile;

    #[test]
    fn test_fseeko() {
        let mut file = tempfile().unwrap();
        file.write_all(b"test").unwrap();
        
        // Test SEEK_SET
        assert!(fseeko(&mut file, 0, 0).is_ok());
        let mut buf = [0; 4];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"test");
        
        // Test SEEK_CUR
        assert!(fseeko(&mut file, -2, 1).is_ok());
        let mut buf = [0; 2];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"st");
        
        // Test SEEK_END
        assert!(fseeko(&mut file, -1, 2).is_ok());
        let mut buf = [0; 1];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"t");
        
        // Test invalid whence
        assert!(fseeko(&mut file, 0, 3).is_err());
    }
}