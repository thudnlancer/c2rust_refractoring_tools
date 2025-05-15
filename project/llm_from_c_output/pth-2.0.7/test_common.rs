/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  test_common.rs: Pth common test program stuff (Rust implementation)
*/

use std::io::{self, Read};
use std::sync::Once;
use std::cell::RefCell;
use std::os::unix::io::RawFd;

const READLINE_MAXLEN: usize = 1024;

thread_local! {
    static READLINE_BUF: RefCell<Option<ReadlineBuf>> = RefCell::new(None);
}

static READLINE_ONCE: Once = Once::new();

struct ReadlineBuf {
    rl_cnt: isize,
    rl_bufptr: usize,
    rl_buf: [u8; READLINE_MAXLEN],
}

impl ReadlineBuf {
    fn new() -> Self {
        ReadlineBuf {
            rl_cnt: 0,
            rl_bufptr: 0,
            rl_buf: [0; READLINE_MAXLEN],
        }
    }
}

fn readline_init() {
    READLINE_ONCE.call_once(|| {
        READLINE_BUF.with(|buf| {
            *buf.borrow_mut() = Some(ReadlineBuf::new());
        });
    });
}

pub fn pth_readline(fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
    pth_readline_ev(fd, buf, None)
}

pub fn pth_readline_ev(fd: RawFd, buf: &mut [u8], ev_extra: Option<()>) -> io::Result<usize> {
    readline_init();
    
    READLINE_BUF.with(|rl| {
        let mut rl = rl.borrow_mut();
        let rl = rl.as_mut().unwrap();
        
        let mut n = 0;
        let mut cp = 0;
        
        while n < buf.len() {
            // Fetch one character (but read more)
            let mut rc = 1;
            if rl.rl_cnt <= 0 {
                let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
                let read_result = file.read(&mut rl.rl_buf);
                unsafe { std::mem::forget(file); } // Prevent double close
                
                match read_result {
                    Ok(count) => {
                        rl.rl_cnt = count as isize;
                        if rl.rl_cnt == 0 {
                            rc = 0;
                        } else {
                            rl.rl_bufptr = 0;
                        }
                    },
                    Err(e) => return Err(e),
                }
            }
            
            let c = if rc == 1 {
                rl.rl_cnt -= 1;
                let c = rl.rl_buf[rl.rl_bufptr];
                rl.rl_bufptr += 1;
                c
            } else if rc == 0 {
                if n == 0 {
                    return Ok(0);
                } else {
                    break;
                }
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "read error"));
            };
            
            // Act on fetched character
            if rc == 1 {
                if c == b'\r' {
                    continue;
                }
                
                if cp < buf.len() {
                    buf[cp] = c;
                    cp += 1;
                    n += 1;
                }
                
                if c == b'\n' {
                    break;
                }
            }
        }
        
        Ok(n)
    })
}