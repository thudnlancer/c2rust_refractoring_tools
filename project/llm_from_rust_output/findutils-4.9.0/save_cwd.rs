use std::ffi::CString;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::{fs, io, ptr};

pub struct SavedCwd {
    desc: Option<fs::File>,
    name: Option<CString>,
}

impl SavedCwd {
    pub fn save() -> io::Result<Self> {
        let desc = match fs::File::open(".") {
            Ok(file) => Some(file),
            Err(_) => None,
        };

        if let Some(file) = &desc {
            Ok(Self {
                desc: Some(file.try_clone()?),
                name: None,
            })
        } else {
            let current_dir = std::env::current_dir()?;
            let c_string = CString::new(current_dir.to_string_lossy().into_owned())?;
            Ok(Self {
                desc: None,
                name: Some(c_string),
            })
        }
    }

    pub fn restore(&self) -> io::Result<()> {
        if let Some(file) = &self.desc {
            let fd = file.as_raw_fd();
            unsafe {
                if libc::fchdir(fd) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
        } else if let Some(path) = &self.name {
            let path_str = path.to_str().ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid path encoding",
            ))?;
            std::env::set_current_dir(Path::new(path_str))?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(file) = self.desc.take() {
            unsafe {
                libc::close(file.as_raw_fd());
            }
        }
        self.name.take();
    }
}