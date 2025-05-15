use std::io::{self, Write};

pub struct File {
    inner: std::fs::File,
    error_flag: bool,
}

impl File {
    pub fn set_error(&mut self) {
        self.error_flag = true;
    }

    pub fn check_error(&self) -> bool {
        self.error_flag
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = self.inner.write(buf);
        if result.is_err() {
            self.set_error();
        }
        result
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = self.inner.flush();
        if result.is_err() {
            self.set_error();
        }
        result
    }
}