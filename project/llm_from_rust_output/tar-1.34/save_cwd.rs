use std::ffi::CString;
use std::os::unix::io::{RawFd, AsRawFd};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io;

#[derive(Debug)]
pub struct SavedCwd {
    desc: Option<File>,
    name: Option<PathBuf>,
}

impl SavedCwd {
    pub fn save() -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .open(".")?;
        
        Ok(SavedCwd {
            desc: Some(file),
            name: None,
        })
    }

    pub fn restore(&self) -> io::Result<()> {
        if let Some(file) = &self.desc {
            let dir = std::fs::File::open(file.as_raw_fd())?;
            std::env::set_current_dir(dir)?;
        } else if let Some(path) = &self.name {
            std::env::set_current_dir(path)?;
        }
        Ok(())
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(file) = self.desc.take() {
            let _ = file.sync_all();
        }
    }
}

fn save_cwd_fallback() -> io::Result<PathBuf> {
    std::env::current_dir()
}

pub fn save_cwd() -> io::Result<SavedCwd> {
    match SavedCwd::save() {
        Ok(cwd) => Ok(cwd),
        Err(_) => {
            let path = save_cwd_fallback()?;
            Ok(SavedCwd {
                desc: None,
                name: Some(path),
            })
        }
    }
}