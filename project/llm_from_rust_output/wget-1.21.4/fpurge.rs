use std::io::{self, Write};

pub struct FileStream {
    inner: std::fs::File,
}

impl FileStream {
    pub fn new(file: std::fs::File) -> Self {
        Self { inner: file }
    }

    pub fn purge(&mut self) -> io::Result<()> {
        self.inner.flush()?;
        Ok(())
    }
}