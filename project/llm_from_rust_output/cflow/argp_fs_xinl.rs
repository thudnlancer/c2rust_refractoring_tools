use std::io::{self, Write};
use std::ops::Range;

pub struct ArgpFmtStream {
    stream: Box<dyn Write>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_offs: usize,
    point_col: isize,
    buf: Vec<u8>,
}

impl ArgpFmtStream {
    pub fn new(stream: Box<dyn Write>) -> Self {
        Self {
            stream,
            lmargin: 0,
            rmargin: 0,
            wmargin: 0,
            point_offs: 0,
            point_col: 0,
            buf: Vec::new(),
        }
    }

    pub fn point(&mut self) -> usize {
        if self.buf.len() > self.point_offs {
            self.update();
        }
        self.point_col.max(0) as usize
    }

    pub fn set_wmargin(&mut self, wmargin: usize) -> usize {
        if self.buf.len() > self.point_offs {
            self.update();
        }
        let old = self.wmargin as usize;
        self.wmargin = wmargin as isize;
        old
    }

    pub fn set_rmargin(&mut self, rmargin: usize) -> usize {
        if self.buf.len() > self.point_offs {
            self.update();
        }
        let old = self.rmargin;
        self.rmargin = rmargin;
        old
    }

    pub fn set_lmargin(&mut self, lmargin: usize) -> usize {
        if self.buf.len() > self.point_offs {
            self.update();
        }
        let old = self.lmargin;
        self.lmargin = lmargin;
        old
    }

    pub fn putc(&mut self, ch: u8) -> io::Result<()> {
        self.ensure(1)?;
        self.buf.push(ch);
        Ok(())
    }

    pub fn puts(&mut self, s: &str) -> io::Result<()> {
        if !s.is_empty() {
            self.write(s.as_bytes())?;
        }
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if self.buf.len() + data.len() <= self.buf.capacity() {
            self.ensure(data.len())?;
        }
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }

    fn ensure(&mut self, amount: usize) -> io::Result<()> {
        if self.buf.len() + amount > self.buf.capacity() {
            self.update()?;
            self.buf.reserve(amount);
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        if !self.buf.is_empty() {
            self.stream.write_all(&self.buf)?;
            self.buf.clear();
        }
        Ok(())
    }
}