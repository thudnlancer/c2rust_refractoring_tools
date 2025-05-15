use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::cmp;

const INIT_BUF_SIZE: usize = 200;
const PRINTF_SIZE_GUESS: usize = 150;

pub struct ArgpFmtStream {
    stream: Box<dyn Write>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_offs: usize,
    point_col: isize,
    buf: Vec<u8>,
    p: usize,
}

impl ArgpFmtStream {
    pub fn new(stream: Box<dyn Write>, lmargin: usize, rmargin: usize, wmargin: isize) -> Option<Self> {
        Some(ArgpFmtStream {
            stream,
            lmargin,
            rmargin,
            wmargin,
            point_col: 0,
            point_offs: 0,
            buf: vec![0; INIT_BUF_SIZE],
            p: 0,
        })
    }

    pub fn free(self) -> io::Result<()> {
        self.update()?;
        if self.p > 0 {
            self.stream.write_all(&self.buf[..self.p])?;
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        let mut buf = &self.buf[self.point_offs..self.p];
        
        while !buf.is_empty() {
            if self.point_col == 0 && self.lmargin != 0 {
                let pad = self.lmargin;
                if self.p + pad <= self.buf.len() {
                    let start = self.point_offs;
                    let end = self.p;
                    let len = end - start;
                    
                    unsafe {
                        ptr::copy(
                            self.buf.as_ptr().add(start),
                            self.buf.as_mut_ptr().add(start + pad),
                            len
                        );
                    }
                    
                    for i in start..start+pad {
                        self.buf[i] = b' ';
                    }
                    
                    self.p += pad;
                    buf = &self.buf[start+pad..self.p];
                    self.point_col = pad as isize;
                } else {
                    for _ in 0..pad {
                        self.stream.write_all(b" ")?;
                    }
                    self.point_col = pad as isize;
                }
            }

            let nl_pos = buf.iter().position(|&c| c == b'\n');

            if self.point_col < 0 {
                self.point_col = 0;
            }

            let (line, rest) = match nl_pos {
                Some(pos) if (self.point_col as usize + pos) < self.rmargin => {
                    self.point_col = 0;
                    (&buf[..pos+1], &buf[pos+1..])
                },
                _ => {
                    let max_len = self.rmargin.saturating_sub(self.point_col as usize);
                    if buf.len() <= max_len {
                        self.point_col += buf.len() as isize;
                        break;
                    } else {
                        if self.wmargin < 0 {
                            if let Some(pos) = nl_pos {
                                let keep = max_len;
                                unsafe {
                                    ptr::copy(
                                        buf.as_ptr().add(pos),
                                        self.buf.as_mut_ptr().add(self.point_offs + keep),
                                        buf.len() - pos
                                    );
                                }
                                self.p = self.point_offs + keep + (buf.len() - pos);
                                self.point_col = 0;
                                buf = &self.buf[self.point_offs + keep + 1..self.p];
                            } else {
                                self.point_col += buf.len() as isize;
                                self.p = self.point_offs + max_len;
                                break;
                            }
                        } else {
                            self.handle_wrap(&mut buf)?;
                        }
                        (buf, &[])
                    }
                }
            };

            buf = rest;
        }

        self.point_offs = self.p;
        Ok(())
    }

    fn handle_wrap(&mut self, buf: &mut &[u8]) -> io::Result<()> {
        // Simplified wrap handling - actual implementation would mirror C logic
        let r = self.rmargin - 1;
        let mut p = r.saturating_sub(self.point_col as usize);
        
        // Find word boundary
        while p > 0 && !buf[p].is_ascii_whitespace() {
            p -= 1;
        }
        
        let nextline = if p > 0 {
            // Skip whitespace
            while p > 0 && buf[p-1].is_ascii_whitespace() {
                p -= 1;
            }
            p + 1
        } else {
            // Long word case
            r.saturating_sub(self.point_col as usize)
        };

        // Insert newline and padding
        if self.ensure(self.wmargin as usize + 1 + (buf.len() - nextline)) {
            unsafe {
                ptr::copy(
                    buf.as_ptr().add(nextline),
                    self.buf.as_mut_ptr().add(self.p + self.wmargin as usize + 1),
                    buf.len() - nextline
                );
            }
            
            self.buf[self.p] = b'\n';
            self.p += 1;
            
            for i in 0..self.wmargin {
                self.buf[self.p + i as usize] = b' ';
            }
            
            self.p += self.wmargin as usize;
            self.p += buf.len() - nextline;
            self.point_col = if self.wmargin > 0 { self.wmargin } else { -1 };
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "ensure failed"));
        }
        
        Ok(())
    }

    fn ensure(&mut self, amount: usize) -> bool {
        if self.buf.len() - self.p < amount {
            if let Err(_) = self.update() {
                return false;
            }
            
            if self.buf.len() - self.p < amount {
                let new_size = self.buf.len() + amount;
                self.buf.resize(new_size, 0);
            }
        }
        true
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if self.ensure(data.len()) {
            unsafe {
                ptr::copy_nonoverlapping(
                    data.as_ptr(),
                    self.buf.as_mut_ptr().add(self.p),
                    data.len()
                );
            }
            self.p += data.len();
            Ok(data.len())
        } else {
            Ok(0)
        }
    }

    pub fn puts(&mut self, s: &str) -> io::Result<()> {
        let len = s.len();
        if len > 0 {
            let wrote = self.write(s.as_bytes())?;
            if wrote != len {
                return Err(io::Error::new(io::ErrorKind::Other, "short write"));
            }
        }
        Ok(())
    }

    pub fn putc(&mut self, ch: u8) -> io::Result<()> {
        if self.ensure(1) {
            self.buf[self.p] = ch;
            self.p += 1;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "ensure failed"))
        }
    }

    pub fn printf(&mut self, fmt: &str, args: fmt::Arguments) -> io::Result<usize> {
        let mut size_guess = PRINTF_SIZE_GUESS;
        loop {
            if !self.ensure(size_guess) {
                return Err(io::Error::new(io::ErrorKind::Other, "ensure failed"));
            }
            
            let avail = self.buf.len() - self.p;
            let s = fmt::write(&mut self.buf[self.p..], args)?;
            
            if s >= avail {
                size_guess = s + 1;
            } else {
                self.p += s;
                return Ok(s);
            }
        }
    }

    // Accessors and setters
    pub fn lmargin(&self) -> usize { self.lmargin }
    pub fn rmargin(&self) -> usize { self.rmargin }
    pub fn wmargin(&self) -> isize { self.wmargin }
    
    pub fn set_lmargin(&mut self, lmargin: usize) -> usize {
        if self.p > self.point_offs {
            let _ = self.update();
        }
        mem::replace(&mut self.lmargin, lmargin)
    }
    
    pub fn set_rmargin(&mut self, rmargin: usize) -> usize {
        if self.p > self.point_offs {
            let _ = self.update();
        }
        mem::replace(&mut self.rmargin, rmargin)
    }
    
    pub fn set_wmargin(&mut self, wmargin: isize) -> isize {
        if self.p > self.point_offs {
            let _ = self.update();
        }
        mem::replace(&mut self.wmargin, wmargin)
    }
    
    pub fn point(&self) -> usize {
        if self.p > self.point_offs {
            let _ = self.update();
        }
        cmp::max(self.point_col, 0) as usize
    }
}

impl Drop for ArgpFmtStream {
    fn drop(&mut self) {
        let _ = self.free();
    }
}

impl Write for ArgpFmtStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.update()?;
        self.stream.flush()
    }
}