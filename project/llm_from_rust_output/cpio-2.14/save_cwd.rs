use std::ffi::CString;
use std::os::unix::io::{RawFd, AsRawFd};
use std::path::Path;
use std::{fs, ptr};

#[derive(Debug)]
pub struct SavedCwd {
    desc: Option<fs::File>,
    name: Option<CString>,
}

impl SavedCwd {
    pub fn save() -> Result<Self, std::io::Error> {
        let desc = match fs::File::open(".") {
            Ok(file) => Some(file),
            Err(_) => None,
        };

        if let Some(file) = &desc {
            let name = None;
            Ok(Self { desc: Some(file.try_clone()?), name })
        } else {
            let current_dir = std::env::current_dir()?;
            let c_string = CString::new(current_dir.to_string_lossy().into_owned())?;
            Ok(Self { desc: None, name: Some(c_string) })
        }
    }

    pub fn restore(&self) -> Result<(), std::io::Error> {
        if let Some(file) = &self.desc {
            let dir = fs::File::open(file.as_raw_fd())?;
            std::env::set_current_dir(dir)?;
        } else if let Some(path) = &self.name {
            std::env::set_current_dir(Path::new(&path.to_string_lossy()))?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(file) = self.desc.take() {
            let _ = std::fs::File::into_raw_fd(file);
        }
        self.name.take();
    }
}