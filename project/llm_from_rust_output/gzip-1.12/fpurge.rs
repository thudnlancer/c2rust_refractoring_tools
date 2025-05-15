use std::io::{self, Write};

pub struct File {
    inner: std::fs::File,
}

impl File {
    pub fn fpurge(&mut self) -> io::Result<()> {
        self.inner.flush()?;
        Ok(())
    }
}

impl From<std::fs::File> for File {
    fn from(file: std::fs::File) -> Self {
        File { inner: file }
    }
}