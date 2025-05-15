use std::ffi::CString;
use std::os::unix::io::{RawFd, AsRawFd};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io;

#[derive(Debug)]
pub struct SavedCwd {
    desc: Option<File>,
    name: Option<CString>,
}

impl SavedCwd {
    pub fn save() -> io::Result<Self> {
        let desc = OpenOptions::new()
            .read(true)
            .open(".")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(SavedCwd {
            desc: Some(desc),
            name: None,
        })
    }

    pub fn restore(&self) -> io::Result<()> {
        if let Some(ref file) = self.desc {
            let dir = std::fs::File::open(file.as_raw_fd())?;
            std::env::set_current_dir(dir)?;
        } else if let Some(ref path) = self.name {
            std::env::set_current_dir(Path::new(path))?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(file) = self.desc.take() {
            let _ = std::fs::File::from(file);
        }
        self.name.take();
    }
}

pub fn save_cwd() -> io::Result<SavedCwd> {
    SavedCwd::save()
}

pub fn restore_cwd(cwd: &SavedCwd) -> io::Result<()> {
    cwd.restore()
}