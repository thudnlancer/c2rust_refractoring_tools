use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::cmp;
use std::fmt;
use std::cell::RefCell;

const INIT_BUF_SIZE: usize = 200;
const PRINTF_SIZE_GUESS: usize = 150;

pub struct ArgpFmtStream {
    stream: RefCell<Box<dyn Write>>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_offs: usize,
    point_col: isize,
    buf: Vec<u8>,
    p: usize,
}

impl ArgpFmtStream {
    pub fn new(
        stream: Box<dyn Write>,
        lmargin: usize,
        rmargin: usize,
        wmargin: isize,
    ) -> Result<Self, io::Error> {
        let buf = vec![0; INIT_BUF_SIZE];
        Ok(Self {
            stream: RefCell::new(stream),
            lmargin,
            rmargin,
            wmargin,
            point_col: 0,
            point_offs: 0,
            buf,
            p: 0,
        })
    }

    pub fn free(self) -> io::Result<()> {
        self.update()?;
        if self.p > 0 {
            self.stream.borrow_mut().write_all(&self.buf[..self.p])?;
        }
        Ok(())
    }

    fn update(&mut self) -> io::Result<()> {
        let mut buf = &mut self.buf[self.point_offs..self.p];
        while !buf.is_empty() {
            if self.point_col == 0 && self.lmargin != 0 {
                let pad = self.lmargin;
                if self.p + pad <= self.buf.len() {
                    let start = self.point_offs;
                    let end = self.p;
                    self.buf.copy_within(start..end, start + pad);
                    for b in &mut self.buf[start..start + pad] {
                        *b = b' ';
                    }
                    self.p += pad;
                    buf = &mut self.buf[self.point_offs + pad..self.p];
                } else {
                    for _ in 0..pad {
                        self.stream.borrow_mut().write_all(&[b' '])?;
                    }
                }
                self.point_col = pad as isize;
            }

            let len = buf.len();
            let nl_pos = buf.iter().position(|&b| b == b'\n');

            if self.point_col < 0 {
                self.point_col = 0;
            }

            let (nl, next_buf) = if let Some(pos) = nl_pos {
                if (self.point_col as usize + pos) < self.rmargin {
                    self.point_col = 0;
                    buf = &mut buf[pos + 1..];
                    continue;
                }
                (&buf[pos..], &mut buf[pos + 1..])
            } else {
                if (self.point_col as usize + len) < self.rmargin {
                    self.point_col += len as isize;
                    break;
                }
                (&buf[len..], &mut buf[len..])
            };

            let r = self.rmargin - 1;
            if self.wmargin < 0 {
                if !nl.is_empty() {
                    let move_len = self.p - (self.point_offs + (r - self.point_col as usize));
                    if move_len > 0 {
                        let src = self.point_offs + (r - self.point_col as usize);
                        let dst = self.point_offs;
                        self.buf.copy_within(src..src + move_len, dst);
                        self.p -= src - dst;
                    }
                    self.point_col = 0;
                    buf = next_buf;
                } else {
                    self.point_col += len as isize;
                    self.p -= (self.point_col as usize).saturating_sub(r);
                    break;
                }
            } else {
                let mut p = self.point_offs + (r + 1 - self.point_col as usize);
                while p > self.point_offs && !self.buf[p - 1].is_ascii_whitespace() {
                    p -= 1;
                }

                let nextline = if p > self.point_offs {
                    let mut p = p;
                    while p > self.point_offs && self.buf[p - 1].is_ascii_whitespace() {
                        p -= 1;
                    }
                    p + 1
                } else {
                    let mut p = self.point_offs + (r + 1 - self.point_col as usize);
                    while p < self.p && !self.buf[p].is_ascii_whitespace() {
                        p += 1;
                    }
                    if p >= self.p {
                        self.point_col = 0;
                        buf = next_buf;
                        continue;
                    }
                    let nl_pos = p;
                    while p < self.p && self.buf[p].is_ascii_whitespace() {
                        p += 1;
                    }
                    p
                };

                let wmargin = self.wmargin as usize;
                if (nextline == self.p + 1 && self.buf.len() - nl_pos < wmargin + 1)
                    || (nextline - (nl_pos + 1) < wmargin && self.p > nextline
                {
                    if self.buf.len() - self.p > wmargin + 1 {
                        let mv = self.p - nextline;
                        self.buf
                            .copy_within(nextline..nextline + mv, nl_pos + 1 + wmargin);
                        let new_nextline = nl_pos + 1 + wmargin;
                        self.buf[nl_pos] = b'\n';
                        self.p = new_nextline + mv;
                        buf = &mut self.buf[nl_pos + 1..self.p];
                    } else {
                        self.stream
                            .borrow_mut()
                            .write_all(&self.buf[..nl_pos])?;
                        self.stream.borrow_mut().write_all(&[b'\n'])?;
                        buf = &mut self.buf[..0];
                        self.p = 0;
                    }
                } else {
                    self.buf[nl_pos] = b'\n';
                }

                if nextline - (nl_pos + 1) >= wmargin
                    || (nextline == self.p + 1 && self.buf.len() - nextline >= wmargin)
                {
                    for i in 0..wmargin {
                        self.buf[nl_pos + 1 + i] = b' ';
                    }
                } else {
                    for _ in 0..wmargin {
                        self.stream.borrow_mut().write_all(&[b' '])?;
                    }
                }

                if nl_pos + 1 + wmargin < nextline {
                    let move_len = self.p - nextline;
                    self.buf.copy_within(
                        nextline..nextline + move_len,
                        nl_pos + 1 + wmargin,
                    );
                }
                self.p -= nextline - (nl_pos + 1 + wmargin);
                buf = &mut self.buf[nl_pos + 1 + wmargin..self.p];
                self.point_col = if wmargin > 0 { wmargin as isize } else { -1 };
            }
        }

        self.point_offs = self.p;
        Ok(())
    }

    fn ensure(&mut self, amount: usize) -> io::Result<()> {
        if self.buf.len() - self.p < amount {
            self.update()?;
            let wrote = self.stream.borrow_mut().write(&self.buf[..self.p])?;
            if wrote == self.p {
                self.p = 0;
                self.point_offs = 0;
            } else {
                self.p -= wrote;
                self.point_offs -= wrote;
                self.buf.copy_within(wrote..wrote + self.p, 0);
                return Err(io::Error::new(io::ErrorKind::Other, "short write"));
            }

            if self.buf.len() < amount {
                let new_size = self.buf.len() + amount;
                self.buf.resize(new_size, 0);
            }
        }
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.ensure(data.len())?;
        let len = data.len();
        self.buf[self.p..self.p + len].copy_from_slice(data);
        self.p += len;
        Ok(len)
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
        self.ensure(1)?;
        self.buf[self.p] = ch;
        self.p += 1;
        Ok(())
    }

    pub fn set_lmargin(&mut self, lmargin: usize) -> usize {
        if self.p > self.point_offs {
            self.update().unwrap();
        }
        let old = self.lmargin;
        self.lmargin = lmargin;
        old
    }

    pub fn set_rmargin(&mut self, rmargin: usize) -> usize {
        if self.p > self.point_offs {
            self.update().unwrap();
        }
        let old = self.rmargin;
        self.rmargin = rmargin;
        old
    }

    pub fn set_wmargin(&mut self, wmargin: isize) -> isize {
        if self.p > self.point_offs {
            self.update().unwrap();
        }
        let old = self.wmargin;
        self.wmargin = wmargin;
        old
    }

    pub fn point(&self) -> usize {
        if self.p > self.point_offs {
            self.update().unwrap();
        }
        if self.point_col >= 0 {
            self.point_col as usize
        } else {
            0
        }
    }

    pub fn printf(&mut self, args: fmt::Arguments) -> io::Result<()> {
        let mut size_guess = PRINTF_SIZE_GUESS;
        loop {
            self.ensure(size_guess)?;
            let avail = self.buf.len() - self.p;
            let s = format_args!();
            let s = s.as_bytes();
            if s.len() >= avail {
                size_guess = s.len() + 1;
                continue;
            }
            self.buf[self.p..self.p + s.len()].copy_from_slice(s);
            self.p += s.len();
            break;
        }
        Ok(())
    }
}

impl Drop for ArgpFmtStream {
    fn drop(&mut self) {
        let _ = self.free();
    }
}