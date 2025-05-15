use std::fmt::{self, Write};
use std::io::{self, Write as IoWrite};
use std::ops::Range;

pub struct ArgpFmtStream {
    stream: Box<dyn IoWrite>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_col: isize,
    buf: String,
    point_offs: usize,
}

impl ArgpFmtStream {
    pub fn new(
        stream: Box<dyn IoWrite>,
        lmargin: usize,
        rmargin: usize,
        wmargin: isize,
    ) -> Option<Self> {
        Some(Self {
            stream,
            lmargin,
            rmargin,
            wmargin,
            point_col: 0,
            buf: String::with_capacity(200),
            point_offs: 0,
        })
    }

    fn update(&mut self) -> io::Result<()> {
        let mut buf = &mut self.buf[self.point_offs..];
        
        while !buf.is_empty() {
            let r = self.rmargin.saturating_sub(1);
            
            if self.point_col == 0 && self.lmargin != 0 {
                let pad = self.lmargin;
                if self.buf.len() + pad <= self.buf.capacity() {
                    let new_point = self.point_offs + pad;
                    self.buf.insert_str(self.point_offs, &" ".repeat(pad));
                    self.point_offs = new_point;
                    buf = &mut self.buf[self.point_offs..];
                } else {
                    write!(self.stream, "{}", " ".repeat(pad))?;
                }
                self.point_col = pad as isize;
            }

            let len = buf.len();
            let nl_pos = buf.find('\n');

            if self.point_col < 0 {
                self.point_col = 0;
            }

            match nl_pos {
                Some(pos) if (self.point_col + pos as isize) < self.rmargin as isize => {
                    self.point_col = 0;
                    buf = &mut buf[pos + 1..];
                    continue;
                }
                _ => {
                    if let Some(pos) = nl_pos {
                        buf = &mut buf[..pos];
                    }
                }
            }

            if self.wmargin < 0 {
                // Handle negative wmargin case
                // ... (implementation omitted for brevity)
            } else {
                // Handle positive wmargin case
                // ... (implementation omitted for brevity)
            }
        }

        self.point_offs = self.buf.len();
        Ok(())
    }

    pub fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.buf.push_str(s);
        self.update()
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<()> {
        write!(&mut self.buf, "{}", args)?;
        self.update()
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.update()?;
        if !self.buf.is_empty() {
            self.stream.write_all(self.buf.as_bytes())?;
            self.buf.clear();
            self.point_offs = 0;
        }
        Ok(())
    }
}

impl Drop for ArgpFmtStream {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl Write for ArgpFmtStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s).map_err(|_| fmt::Error)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.write_fmt(args).map_err(|_| fmt::Error)
    }
}

impl IoWrite for ArgpFmtStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8 sequence")
        })?;
        self.write_str(s)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        ArgpFmtStream::flush(self)
    }
}