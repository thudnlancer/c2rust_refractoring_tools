use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::io;

fn careadlinkatcwd(filename: &Path, buffer: &mut [u8]) -> io::Result<usize> {
    std::fs::read_link(filename)
        .and_then(|target| {
            let target_bytes = target.as_os_str().as_bytes();
            if target_bytes.len() > buffer.len() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Buffer too small",
                ))
            } else {
                buffer[..target_bytes.len()].copy_from_slice(target_bytes);
                Ok(target_bytes.len())
            }
        })
}

pub fn areadlink(filename: &Path) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut size = 128;

    loop {
        buffer.resize(size, 0);
        match careadlinkatcwd(filename, &mut buffer) {
            Ok(len) => {
                buffer.truncate(len);
                return Ok(buffer);
            }
            Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
                size *= 2;
                if size > 65536 {
                    return Err(e);
                }
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_areadlink() {
        let dir = tempdir().unwrap();
        let target_path = dir.path().join("target");
        let link_path = dir.path().join("link");

        fs::write(&target_path, "test").unwrap();
        std::os::unix::fs::symlink(&target_path, &link_path).unwrap();

        let result = areadlink(&link_path).unwrap();
        assert_eq!(result, target_path.as_os_str().as_bytes());
    }
}