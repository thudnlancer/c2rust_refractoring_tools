use std::ffi::CString;
use std::os::unix::io::{RawFd, AsRawFd};
use std::path::{Path, PathBuf};
use std::io::{self, Error, ErrorKind};

pub struct SavedCwd {
    desc: Option<RawFd>,
    name: Option<CString>,
}

impl SavedCwd {
    pub fn save() -> io::Result<Self> {
        let desc = unsafe {
            let fd = libc::open(
                b".\0".as_ptr() as *const libc::c_char,
                libc::O_RDONLY | libc::O_DIRECTORY,
            );
            if fd < 0 {
                None
            } else {
                Some(fd)
            }
        };

        if let Some(fd) = desc {
            Ok(Self { desc: Some(fd), name: None })
        } else {
            let name = std::env::current_dir()
                .and_then(|p| CString::new(p.into_os_string().into_vec()).map_err(|_| Error::new(ErrorKind::Other, "Invalid path")))?;
            Ok(Self { desc: None, name: Some(name) })
        }
    }

    pub fn restore(&self) -> io::Result<()> {
        if let Some(fd) = self.desc {
            let result = unsafe { libc::fchdir(fd) };
            if result < 0 {
                return Err(Error::last_os_error());
            }
        } else if let Some(name) = &self.name {
            let path = Path::new(name.to_str().map_err(|_| Error::new(ErrorKind::Other, "Invalid path"))?);
            std::env::set_current_dir(path)?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(fd) = self.desc {
            unsafe { libc::close(fd) };
        }
    }
}