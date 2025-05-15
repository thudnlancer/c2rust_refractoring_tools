use std::io::{self, Write};
use std::ptr;
use std::mem;
use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

struct ArgpFmtStream {
    stream: Rc<RefCell<dyn Write>>,
    lmargin: usize,
    rmargin: usize,
    wmargin: isize,
    point_offs: usize,
    point_col: isize,
    buf: Vec<u8>,
    p: usize,
}

impl ArgpFmtStream {
    fn new(
        stream: Rc<RefCell<dyn Write>>,
        lmargin: usize,
        rmargin: usize,
        wmargin: isize,
    ) -> Result<Self, io::Error> {
        let initial_buf_size = 200;
        let mut buf = Vec::with_capacity(initial_buf_size);
        buf.resize(initial_buf_size, 0);

        Ok(Self {
            stream,
            lmargin,
            rmargin,
            wmargin,
            point_col: 0,
            point_offs: 0,
            buf,
            p: 0,
        })
    }

    fn update(&mut self) -> io::Result<()> {
        let mut buf = &mut self.buf[self.point_offs..];
        let mut len = self.p - self.point_offs;

        while !buf.is_empty() {
            if self.point_col == 0 && self.lmargin != 0 {
                let pad = self.lmargin;
                if self.p + pad <= self.buf.len() {
                    unsafe {
                        ptr::copy(
                            buf.as_ptr(),
                            buf.as_mut_ptr().add(pad),
                            len,
                        );
                        ptr::write_bytes(buf.as_mut_ptr(), b' ', pad);
                    }
                    self.p += pad;
                    buf = &mut buf[pad..];
                } else {
                    for _ in 0..pad {
                        self.stream.borrow_mut().write_all(&[b' '])?;
                    }
                }
                self.point_col = pad as isize;
            }

            let nl_pos = buf.iter().position(|&c| c == b'\n');

            if self.point_col < 0 {
                self.point_col = 0;
            }

            let (line_end, next_line_start) = match nl_pos {
                Some(pos) if (self.point_col as usize + pos) < self.rmargin => {
                    self.point_col = 0;
                    (pos, pos + 1)
                }
                Some(pos) => (pos, pos + 1),
                None if (self.point_col as usize + len) < self.rmargin => {
                    self.point_col += len as isize;
                    break;
                }
                None => (len, len),
            };

            if line_end >= self.rmargin {
                if self.wmargin < 0 {
                    // Truncate line
                    if line_end < len {
                        unsafe {
                            ptr::copy(
                                buf.as_ptr().add(line_end),
                                buf.as_mut_ptr().add(self.rmargin - 1),
                                len - line_end,
                            );
                        }
                        self.p -= line_end - (self.rmargin - 1);
                        self.point_col = 0;
                        buf = &mut buf[self.rmargin..];
                    } else {
                        self.point_col += len as isize;
                        self.p -= (self.point_col as usize - self.rmargin + 1);
                        break;
                    }
                } else {
                    // Word wrap
                    let mut p = self.rmargin - 1;
                    while p > 0 && !buf[p].is_ascii_whitespace() {
                        p -= 1;
                    }

                    let nextline = if p > 0 {
                        p + 1
                    } else {
                        self.rmargin
                    };

                    let nl = if nextline > 0 {
                        let mut nl = p;
                        while nl > 0 && buf[nl].is_ascii_whitespace() {
                            nl -= 1;
                        }
                        nl + 1
                    } else {
                        let mut p = self.rmargin;
                        while p < len && !buf[p].is_ascii_whitespace() {
                            p += 1;
                        }
                        if p == len {
                            self.point_col = 0;
                            buf = &mut buf[p + 1..];
                            continue;
                        }
                        p
                    };

                    if (nextline == len + 1 && self.buf.len() - nl < (self.wmargin as usize + 1))
                        || (nextline - (nl + 1) < self.wmargin as usize
                            && self.p > nextline + self.point_offs)
                    {
                        if self.buf.len() - self.p > self.wmargin as usize + 1 {
                            unsafe {
                                ptr::copy(
                                    buf.as_ptr().add(nextline),
                                    buf.as_mut_ptr().add(nl + 1 + self.wmargin as usize),
                                    len - nextline,
                                );
                            }
                            buf[nl] = b'\n';
                            nl += 1;
                            for i in 0..self.wmargin {
                                buf[nl + i as usize] = b' ';
                            }
                        } else {
                            self.stream.borrow_mut().write_all(&buf[..nl])?;
                            self.stream.borrow_mut().write_all(&[b'\n'])?;
                            len += self.point_offs;
                            buf = &mut self.buf[..];
                        }
                    } else {
                        buf[nl] = b'\n';
                        for i in 0..self.wmargin {
                            buf[nl + 1 + i as usize] = b' ';
                        }
                    }

                    if nl + 1 + self.wmargin as usize < len {
                        unsafe {
                            ptr::copy(
                                buf.as_ptr().add(nextline),
                                buf.as_mut_ptr().add(nl + 1 + self.wmargin as usize),
                                len - nextline,
                            );
                        }
                    }

                    self.point_col = if self.wmargin > 0 {
                        self.wmargin
                    } else {
                        -1
                    };
                }
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
                unsafe {
                    ptr::copy(
                        self.buf.as_ptr().add(wrote),
                        self.buf.as_mut_ptr(),
                        self.p,
                    );
                }
                return Err(io::Error::new(io::ErrorKind::Other, "short write"));
            }

            if self.buf.len() < amount {
                let new_size = self.buf.len() + amount;
                self.buf.resize(new_size, 0);
            }
        }
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.ensure(buf.len())?;
        let to_copy = std::cmp::min(buf.len(), self.buf.len() - self.p);
        unsafe {
            ptr::copy_nonoverlapping(
                buf.as_ptr(),
                self.buf.as_mut_ptr().add(self.p),
                to_copy,
            );
        }
        self.p += to_copy;
        Ok(to_copy)
    }

    fn putc(&mut self, ch: u8) -> io::Result<()> {
        self.ensure(1)?;
        self.buf[self.p] = ch;
        self.p += 1;
        Ok(())
    }

    fn puts(&mut self, s: &str) -> io::Result<()> {
        self.write_all(s.as_bytes())?;
        Ok(())
    }

    fn printf(&mut self, args: fmt::Arguments) -> io::Result<()> {
        let mut s = String::new();
        fmt::write(&mut s, args).map_err(|_| io::Error::new(io::ErrorKind::Other, "format error"))?;
        self.write_all(s.as_bytes())?;
        Ok(())
    }

    fn free(mut self) -> io::Result<()> {
        self.update()?;
        if self.p > 0 {
            self.stream.borrow_mut().write_all(&self.buf[..self.p])?;
        }
        Ok(())
    }
}

impl Drop for ArgpFmtStream {
    fn drop(&mut self) {
        let _ = self.update();
        if self.p > 0 {
            let _ = self.stream.borrow_mut().write_all(&self.buf[..self.p]);
        }
    }
}