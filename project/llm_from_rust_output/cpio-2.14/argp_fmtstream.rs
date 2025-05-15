use std::fmt::{self, Write};
use std::io::{self, Write as IoWrite};
use std::ops::Range;

pub struct ArgpFmtStream {
    stream: Box<dyn IoWrite>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_col: isize,
    buf: Vec<u8>,
    point_offs: usize,
}

impl ArgpFmtStream {
    pub fn new(
        stream: Box<dyn IoWrite>,
        lmargin: usize,
        rmargin: usize,
        wmargin: isize,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            stream,
            lmargin,
            rmargin,
            wmargin,
            point_col: 0,
            buf: vec![0; 200],
            point_offs: 0,
        })
    }

    pub fn update(&mut self) -> io::Result<()> {
        let mut buf_start = self.point_offs;
        
        while buf_start < self.buf.len() {
            if self.point_col == 0 && self.lmargin != 0 {
                let pad = self.lmargin;
                if self.buf.len() + pad <= self.buf.capacity() {
                    self.buf.splice(buf_start..buf_start, vec![b' '; pad]);
                    buf_start += pad;
                    self.point_col = pad as isize;
                } else {
                    self.stream.write_all(&vec![b' '; pad])?;
                }
            }

            let len = self.buf.len() - buf_start;
            let nl_pos = self.buf[buf_start..].iter().position(|&c| c == b'\n');

            if self.point_col < 0 {
                self.point_col = 0;
            }

            match nl_pos {
                Some(pos) if (self.point_col + pos as isize) < self.rmargin as isize => {
                    self.point_col = 0;
                    buf_start += pos + 1;
                    continue;
                }
                None if (self.point_col as usize + len) < self.rmargin => {
                    self.point_col += len as isize;
                    break;
                }
                _ => {
                    let r = self.rmargin - 1;
                    if self.wmargin < 0 {
                        self.handle_negative_wmargin(buf_start, r, len)?;
                    } else {
                        self.handle_positive_wmargin(buf_start, r, len)?;
                    }
                }
            }
        }

        self.point_offs = self.buf.len();
        Ok(())
    }

    fn handle_negative_wmargin(&mut self, buf_start: usize, r: usize, len: usize) -> io::Result<()> {
        let nl_pos = self.buf[buf_start..].iter().position(|&c| c == b'\n');
        
        match nl_pos {
            Some(pos) => {
                let nl = buf_start + pos;
                let move_len = self.buf.len() - nl;
                let new_pos = buf_start + r - self.point_col as usize;
                
                if new_pos + move_len <= self.buf.capacity() {
                    self.buf.copy_within(nl..nl + move_len, new_pos);
                    self.buf.truncate(new_pos + move_len);
                    self.point_col = 0;
                } else {
                    self.stream.write_all(&self.buf[buf_start..nl])?;
                    self.stream.write_all(b"\n")?;
                    self.buf.drain(..nl + 1);
                }
            }
            None => {
                self.point_col += len as isize;
                let overflow = (self.point_col as usize).saturating_sub(r);
                if overflow > 0 {
                    self.buf.truncate(self.buf.len().saturating_sub(overflow));
                }
            }
        }
        Ok(())
    }

    fn handle_positive_wmargin(&mut self, buf_start: usize, r: usize, len: usize) -> io::Result<()> {
        let p = buf_start + r - self.point_col as usize;
        let nextline = self.find_next_line(p, buf_start, len);
        
        if nextline > buf_start {
            self.insert_newline_and_spaces(buf_start, nextline, len)?;
        } else {
            self.handle_no_break_positions(buf_start, r, len)?;
        }
        
        Ok(())
    }

    fn find_next_line(&self, start: usize, buf_start: usize, len: usize) -> usize {
        let mut p = start;
        while p > buf_start && !self.buf[p].is_ascii_whitespace() {
            p -= 1;
        }
        
        if p > buf_start {
            p + 1
        } else {
            buf_start + len
        }
    }

    fn insert_newline_and_spaces(
        &mut self,
        buf_start: usize,
        nextline: usize,
        len: usize,
    ) -> io::Result<()> {
        let nl_pos = buf_start + self.buf[buf_start..].iter().position(|&c| c == b'\n').unwrap_or(len);
        
        if self.wmargin > 0 {
            if nextline - nl_pos > 1 || (self.buf.capacity() - nextline) < self.wmargin as usize {
                self.stream.write_all(&self.buf[..nl_pos])?;
                self.stream.write_all(b"\n")?;
                self.stream.write_all(&vec![b' '; self.wmargin as usize])?;
                self.buf.drain(..nextline);
            } else {
                self.buf.insert(nl_pos, b'\n');
                self.buf.splice(nl_pos + 1..nl_pos + 1, vec![b' '; self.wmargin as usize]);
            }
        }
        
        Ok(())
    }

    fn handle_no_break_positions(
        &mut self,
        buf_start: usize,
        r: usize,
        len: usize,
    ) -> io::Result<()> {
        let p = buf_start + r - self.point_col as usize;
        let nl_pos = buf_start + self.buf[buf_start..].iter().position(|&c| c == b'\n').unwrap_or(len);
        
        if p < nl_pos {
            self.buf.insert(p, b'\n');
            if self.wmargin > 0 {
                self.buf.splice(p + 1..p + 1, vec![b' '; self.wmargin as usize]);
            }
        }
        
        Ok(())
    }

    pub fn ensure(&mut self, amount: usize) -> io::Result<bool> {
        if self.buf.capacity() - self.buf.len() < amount {
            self.update()?;
            let wrote = self.stream.write(&self.buf)?;
            
            if wrote == self.buf.len() {
                self.buf.clear();
                self.point_offs = 0;
            } else {
                self.buf.drain(..wrote);
                self.point_offs = self.point_offs.saturating_sub(wrote);
                return Ok(false);
            }
            
            if self.buf.capacity() < amount {
                let new_size = self.buf.capacity() + amount;
                self.buf.reserve(new_size - self.buf.capacity());
            }
        }
        Ok(true)
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<usize> {
        let mut s = String::new();
        s.write_fmt(args)?;
        self.write_all(s.as_bytes())?;
        Ok(s.len())
    }
}

impl IoWrite for ArgpFmtStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.ensure(buf.len())? {
            let start = self.buf.len();
            self.buf.extend_from_slice(buf);
            Ok(self.buf.len() - start)
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.update()?;
        self.stream.flush()
    }
}

impl Drop for ArgpFmtStream {
    fn drop(&mut self) {
        let _ = self.update();
        if !self.buf.is_empty() {
            let _ = self.stream.write_all(&self.buf);
        }
    }
}