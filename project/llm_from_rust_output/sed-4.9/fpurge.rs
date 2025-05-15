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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempfile;

    #[test]
    fn test_purge() {
        let file = tempfile().unwrap();
        let mut stream = FileStream::new(file);
        assert!(stream.purge().is_ok());
    }
}