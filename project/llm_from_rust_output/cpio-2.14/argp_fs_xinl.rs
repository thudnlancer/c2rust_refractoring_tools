use std::io::{self, Write};
use std::ptr;

pub struct ArgpFmtStream {
    stream: Box<dyn Write>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_offs: usize,
    point_col: isize,
    buf: Vec<u8>,
    pos: usize,
}

impl ArgpFmtStream {
    pub fn point(&mut self) -> usize {
        if self.pos > self.point_offs {
            self.update();
        }
        self.point_col.max(0) as usize
    }

    pub fn set_wmargin(&mut self, wmargin: usize) -> usize {
        if self.pos > self.point_offs {
            self.update();
        }
        let old = self.wmargin as usize;
        self.wmargin = wmargin as isize;
        old
    }

    pub fn set_rmargin(&mut self, rmargin: usize) -> usize {
        if self.pos > self.point_offs {
            self.update();
        }
        let old = self.rmargin;
        self.rmargin = rmargin;
        old
    }

    pub fn set_lmargin(&mut self, lmargin: usize) -> usize {
        if self.pos > self.point_offs {
            self.update();
        }
        let old = self.lmargin;
        self.lmargin = lmargin;
        old
    }

    pub fn putc(&mut self, ch: u8) -> io::Result<()> {
        self.ensure(1)?;
        self.buf[self.pos] = ch;
        self.pos += 1;
        Ok(())
    }

    pub fn puts(&mut self, s: &str) -> io::Result<()> {
        if !s.is_empty() {
            let len = s.len();
            let wrote = self.write(s.as_bytes())?;
            if wrote != len {
                return Err(io::Error::new(io::ErrorKind::Other, "failed to write full string"));
            }
        }
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let len = data.len();
        self.ensure(len)?;
        self.buf[self.pos..self.pos + len].copy_from_slice(data);
        self.pos += len;
        Ok(len)
    }

    fn ensure(&mut self, amount: usize) -> io::Result<()> {
        if self.pos + amount > self.buf.len() {
            self.update()?;
            if self.pos + amount > self.buf.len() {
                let new_size = self.pos + amount;
                self.buf.resize(new_size, 0);
            }
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        if self.pos > 0 {
            self.stream.write_all(&self.buf[..self.pos])?;
            self.pos = 0;
        }
        Ok(())
    }
}