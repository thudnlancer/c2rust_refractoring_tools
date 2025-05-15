/*  Copyright 1996,1997,1999,2001-2003,2008,2009,2021 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::io::{self, Read};
use std::marker::PhantomData;

const U2D_BUFSIZE: usize = 4096;

struct Filter<R: Read> {
    inner: R,
    buffer: [u8; U2D_BUFSIZE],
    read_bytes: usize,
    buf_pos: usize,
    pending_nl: bool,
    eof: bool,
}

impl<R: Read> Filter<R> {
    fn new(inner: R) -> Self {
        Filter {
            inner,
            buffer: [0; U2D_BUFSIZE],
            read_bytes: 0,
            buf_pos: 0,
            pending_nl: false,
            eof: false,
        }
    }
}

impl<R: Read> Read for Filter<R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if self.eof {
            return Ok(0);
        }

        let mut i = 0;
        while i < output.len() && !self.eof {
            let c = if self.pending_nl {
                self.pending_nl = false;
                b'\n'
            } else {
                if self.buf_pos == self.read_bytes {
                    match self.inner.read(&mut self.buffer) {
                        Ok(0) => {
                            self.eof = true;
                            b'\x1a'
                        }
                        Ok(n) => {
                            self.read_bytes = n;
                            self.buf_pos = 0;
                            continue;
                        }
                        Err(e) => {
                            if i == 0 {
                                return Err(e);
                            } else {
                                break;
                            }
                        }
                    }
                } else if self.buf_pos == self.read_bytes {
                    self.eof = true;
                    b'\x1a'
                } else {
                    let c = self.buffer[self.buf_pos];
                    self.buf_pos += 1;
                    if c == b'\n' {
                        self.pending_nl = true;
                        b'\r'
                    } else {
                        c
                    }
                }
            };
            output[i] = c;
            i += 1;
        }

        Ok(i)
    }
}

pub fn open_unix2dos<R: Read>(inner: R, _convert_charset: bool) -> impl Read {
    Filter::new(inner)
}