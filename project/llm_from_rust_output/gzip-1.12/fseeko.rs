use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

pub type off_t = i64;

pub fn rpl_fseeko(file: &mut File, offset: off_t, whence: i32) -> std::io::Result<()> {
    let seek_from = match whence {
        libc::SEEK_SET => SeekFrom::Start(offset as u64),
        libc::SEEK_CUR => SeekFrom::Current(offset),
        libc::SEEK_END => SeekFrom::End(offset),
        _ => return Err(std::io::Error::from_raw_os_error(libc::EINVAL)),
    };

    file.seek(seek_from)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use tempfile::tempfile;

    #[test]
    fn test_rpl_fseeko() {
        let mut file = tempfile().unwrap();
        file.write_all(b"test").unwrap();
        
        assert!(rpl_fseeko(&mut file, 0, libc::SEEK_SET).is_ok());
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"test");

        assert!(rpl_fseeko(&mut file, -2, libc::SEEK_END).is_ok());
        let mut buf = [0u8; 2];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"st");

        assert!(rpl_fseeko(&mut file, -1, libc::SEEK_CUR).is_ok());
        let mut buf = [0u8; 1];
        file.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, b"t");
    }
}