use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::FromRawFd;
use std::io;
use std::ffi::CStr;
use std::os::unix::io::IntoRawFd;

#[derive(Debug)]
pub enum OpenMode {
    Read,
    Write,
    Append,
    ReadWrite,
}

#[derive(Debug)]
pub struct OpenOptionsBuilder {
    mode: OpenMode,
    create: bool,
    truncate: bool,
    append: bool,
    create_new: bool,
    cloexec: bool,
}

impl OpenOptionsBuilder {
    pub fn new() -> Self {
        Self {
            mode: OpenMode::Read,
            create: false,
            truncate: false,
            append: false,
            create_new: false,
            cloexec: false,
        }
    }

    pub fn mode(mut self, mode: OpenMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn create(mut self, create: bool) -> Self {
        self.create = create;
        self
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }

    pub fn append(mut self, append: bool) -> Self {
        self.append = append;
        self
    }

    pub fn create_new(mut self, create_new: bool) -> Self {
        self.create_new = create_new;
        self
    }

    pub fn cloexec(mut self, cloexec: bool) -> Self {
        self.cloexec = cloexec;
        self
    }

    pub fn build(self) -> OpenOptions {
        let mut options = OpenOptions::new();
        match self.mode {
            OpenMode::Read => options.read(true),
            OpenMode::Write => options.write(true),
            OpenMode::Append => {
                options.write(true).append(true)
            }
            OpenMode::ReadWrite => {
                options.read(true).write(true)
            }
        }
        .create(self.create)
        .truncate(self.truncate)
        .create_new(self.create_new)
        .custom_flags(if self.cloexec { libc::O_CLOEXEC } else { 0 })
    }
}

pub fn rpl_fopen(filename: &CStr, mode: &CStr) -> io::Result<File> {
    let mode_str = mode.to_str().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode string"))?;
    
    let mut builder = OpenOptionsBuilder::new();
    let mut needs_gnu = false;

    for c in mode_str.chars() {
        match c {
            'r' => builder = builder.mode(OpenMode::Read),
            'w' => {
                builder = builder.mode(OpenMode::Write)
                    .create(true)
                    .truncate(true);
            }
            'a' => {
                builder = builder.mode(OpenMode::Append)
                    .create(true);
            }
            'b' => (), // binary mode is default in Rust
            '+' => builder = builder.mode(OpenMode::ReadWrite),
            'x' => {
                builder = builder.create_new(true);
                needs_gnu = true;
            }
            'e' => {
                builder = builder.cloexec(true);
                needs_gnu = true;
            }
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid mode character")),
        }
    }

    if needs_gnu {
        let options = builder.build();
        let file = options.open(filename.to_str()?)?;
        Ok(file)
    } else {
        let file = File::open(filename.to_str()?)?;
        Ok(file)
    }
}