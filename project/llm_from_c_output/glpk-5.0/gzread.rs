use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::Path;
use libc::{c_int, c_uint, c_ulong};
use zlib::{Z_OK, Z_ERRNO, Z_STREAM_ERROR, Z_DATA_ERROR, Z_MEM_ERROR, Z_BUF_ERROR};
use zlib::{inflateInit2, inflateReset, inflateEnd, inflate, Z_NO_FLUSH, Z_STREAM_END};
use zlib::{crc32, Z_NULL, Z_NEED_DICT, Z_STREAM_ERROR as Z_STREAM_ERROR_CODE};
use std::ptr;
use std::mem;
use std::slice;

struct GzState {
    fd: File,
    path: String,
    mode: c_int,
    size: c_uint,
    want: c_uint,
    direct: c_int,
    eof: c_int,
    err: c_int,
    msg: String,
    pos: u64,
    in_buf: Vec<u8>,
    out_buf: Vec<u8>,
    have: c_uint,
    next: *mut u8,
    raw: u64,
    seek: c_int,
    skip: u64,
    how: c_int,
    strm: z_stream,
}

const GZ_READ: c_int = 1;
const LOOK: c_int = 0;
const COPY: c_int = 1;
const GZIP: c_int = 2;

impl GzState {
    fn new(fd: File, path: String, mode: c_int) -> Self {
        GzState {
            fd,
            path,
            mode,
            size: 0,
            want: 8192,
            direct: 0,
            eof: 0,
            err: Z_OK,
            msg: String::new(),
            pos: 0,
            in_buf: Vec::new(),
            out_buf: Vec::new(),
            have: 0,
            next: ptr::null_mut(),
            raw: 0,
            seek: 0,
            skip: 0,
            how: LOOK,
            strm: z_stream {
                next_in: Z_NULL,
                avail_in: 0,
                total_in: 0,
                next_out: Z_NULL,
                avail_out: 0,
                total_out: 0,
                msg: Z_NULL,
                state: Z_NULL,
                zalloc: Z_NULL,
                zfree: Z_NULL,
                opaque: Z_NULL,
                data_type: 0,
                adler: 0,
                reserved: 0,
            },
        }
    }

    fn gz_error(&mut self, err: c_int, msg: &str) {
        self.err = err;
        self.msg = msg.to_string();
    }

    fn gz_load(&mut self, buf: &mut [u8], len: c_uint, have: &mut c_uint) -> c_int {
        *have = 0;
        loop {
            let to_read = (len - *have) as usize;
            match self.fd.read(&mut buf[*have as usize..(*have as usize + to_read)]) {
                Ok(n) if n > 0 => {
                    *have += n as c_uint;
                    if *have >= len {
                        break;
                    }
                }
                Ok(_) => break,
                Err(e) => {
                    self.gz_error(Z_ERRNO, &e.to_string());
                    return -1;
                }
            }
        }
        if *have < len {
            self.eof = 1;
        }
        0
    }

    fn gz_avail(&mut self) -> c_int {
        if self.err != Z_OK {
            return -1;
        }
        if self.eof == 0 {
            let mut avail = 0;
            if self.gz_load(&mut self.in_buf, self.size, &mut avail) == -1 {
                return -1;
            }
            self.strm.avail_in = avail;
            self.strm.next_in = self.in_buf.as_mut_ptr();
        }
        0
    }

    fn next_byte(&mut self) -> c_int {
        if self.strm.avail_in == 0 && self.gz_avail() == -1 {
            -1
        } else if self.strm.avail_in == 0 {
            -1
        } else {
            self.strm.avail_in -= 1;
            let byte = unsafe { *self.strm.next_in };
            self.strm.next_in = unsafe { self.strm.next_in.add(1) };
            byte as c_int
        }
    }

    fn gz_next4(&mut self, ret: &mut c_ulong) -> c_int {
        let mut val = 0;
        let ch = self.next_byte();
        if ch == -1 {
            return -1;
        }
        val += ch as c_ulong;
        let ch = self.next_byte();
        if ch == -1 {
            return -1;
        }
        val += (ch as c_ulong) << 8;
        let ch = self.next_byte();
        if ch == -1 {
            return -1;
        }
        val += (ch as c_ulong) << 16;
        let ch = self.next_byte();
        if ch == -1 {
            return -1;
        }
        val += (ch as c_ulong) << 24;
        *ret = val;
        0
    }

    fn gz_head(&mut self) -> c_int {
        if self.size == 0 {
            self.in_buf = vec![0; self.want as usize];
            self.out_buf = vec![0; (self.want << 1) as usize];
            if self.in_buf.is_empty() || self.out_buf.is_empty() {
                self.gz_error(Z_MEM_ERROR, "out of memory");
                return -1;
            }
            self.size = self.want;

            self.strm.zalloc = Z_NULL;
            self.strm.zfree = Z_NULL;
            self.strm.opaque = Z_NULL;
            self.strm.avail_in = 0;
            self.strm.next_in = Z_NULL;
            if unsafe { inflateInit2(&mut self.strm, -15) } != Z_OK {
                self.gz_error(Z_MEM_ERROR, "out of memory");
                return -1;
            }
        }

        if self.strm.avail_in == 0 {
            if self.gz_avail() == -1 {
                return -1;
            }
            if self.strm.avail_in == 0 {
                return 0;
            }
        }

        if unsafe { *self.strm.next_in } == 31 {
            self.strm.avail_in -= 1;
            self.strm.next_in = unsafe { self.strm.next_in.add(1) };
            if self.strm.avail_in == 0 && self.gz_avail() == -1 {
                return -1;
            }
            if self.strm.avail_in > 0 && unsafe { *self.strm.next_in } == 139 {
                self.strm.avail_in -= 1;
                self.strm.next_in = unsafe { self.strm.next_in.add(1) };

                if self.next_byte() != 8 {
                    self.gz_error(Z_DATA_ERROR, "unknown compression method");
                    return -1;
                }
                let flags = self.next_byte();
                if flags & 0xe0 != 0 {
                    self.gz_error(Z_DATA_ERROR, "unknown header flags set");
                    return -1;
                }
                for _ in 0..5 {
                    self.next_byte();
                }
                if flags & 4 != 0 {
                    let mut len = self.next_byte() as c_uint;
                    len += (self.next_byte() as c_uint) << 8;
                    for _ in 0..len {
                        if self.next_byte() < 0 {
                            break;
                        }
                    }
                }
                if flags & 8 != 0 {
                    while self.next_byte() > 0 {}
                }
                if flags & 16 != 0 {
                    while self.next_byte() > 0 {}
                }
                if flags & 2 != 0 {
                    self.next_byte();
                    self.next_byte();
                }

                unsafe { inflateReset(&mut self.strm) };
                self.strm.adler = crc32(0, Z_NULL, 0);
                self.how = GZIP;
                self.direct = 0;
                return 0;
            } else {
                self.out_buf[0] = 31;
                self.have = 1;
            }
        }

        self.raw = self.pos;
        self.next = self.out_buf.as_mut_ptr();
        if self.strm.avail_in > 0 {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.strm.next_in,
                    self.next.add(self.have as usize),
                    self.strm.avail_in as usize,
                );
            }
            self.have += self.strm.avail_in;
            self.strm.avail_in = 0;
        }
        self.how = COPY;
        self.direct = 1;
        0
    }

    fn gz_decomp(&mut self) -> c_int {
        let had = self.strm.avail_out;
        loop {
            if self.strm.avail_in == 0 && self.gz_avail() == -1 {
                return -1;
            }
            if self.strm.avail_in == 0 {
                self.gz_error(Z_DATA_ERROR, "unexpected end of file");
                return -1;
            }

            let ret = unsafe { inflate(&mut self.strm, Z_NO_FLUSH) };
            if ret == Z_STREAM_ERROR_CODE || ret == Z_NEED_DICT {
                self.gz_error(Z_STREAM_ERROR, "internal error: inflate stream corrupt");
                return -1;
            }
            if ret == Z_MEM_ERROR {
                self.gz_error(Z_MEM_ERROR, "out of memory");
                return -1;
            }
            if ret == Z_DATA_ERROR {
                self.gz_error(
                    Z_DATA_ERROR,
                    if self.strm.msg.is_null() {
                        "compressed data error"
                    } else {
                        unsafe { std::ffi::CStr::from_ptr(self.strm.msg) }
                            .to_str()
                            .unwrap_or("compressed data error")
                    },
                );
                return -1;
            }
            if self.strm.avail_out == 0 || ret == Z_STREAM_END {
                break;
            }
        }

        self.have = had - self.strm.avail_out;
        self.next = unsafe { self.strm.next_out.sub(self.have as usize) };
        self.strm.adler = crc32(
            self.strm.adler,
            unsafe { slice::from_raw_parts(self.next, self.have as usize) }.as_ptr(),
            self.have as u32,
        );

        if self.strm.avail_out == 0 {
            let mut crc = 0;
            let mut len = 0;
            if self.gz_next4(&mut crc) == -1 || self.gz_next4(&mut len) == -1 {
                self.gz_error(Z_DATA_ERROR, "unexpected end of file");
                return -1;
            }
            if crc != self.strm.adler {
                self.gz_error(Z_DATA_ERROR, "incorrect data check");
                return -1;
            }
            if len != (self.strm.total_out & 0xffffffff) {
                self.gz_error(Z_DATA_ERROR, "incorrect length check");
                return -1;
            }
            self.how = LOOK;
        }

        0
    }

    fn gz_make(&mut self) -> c_int {
        if self.how == LOOK {
            if self.gz_head() == -1 {
                return -1;
            }
            if self.have > 0 {
                return 0;
            }
        }
        if self.how == COPY {
            if self.gz_load(&mut self.out_buf, self.size << 1, &mut self.have) == -1 {
                return -1;
            }
            self.next = self.out_buf.as_mut_ptr();
        } else if self.how == GZIP {
            self.strm.avail_out = self.size << 1;
            self.strm.next_out = self.out_buf.as_mut_ptr();
            if self.gz_decomp() == -1 {
                return -1;
            }
        }
        0
    }

    fn gz_skip(&mut self, len: u64) -> c_int {
        let mut remaining = len;
        while remaining > 0 {
            if self.have > 0 {
                let n = if self.have as u64 > remaining {
                    remaining as c_uint
                } else {
                    self.have
                };
                self.have -= n;
                self.next = unsafe { self.next.add(n as usize) };
                self.pos += n as u64;
                remaining -= n as u64;
            } else if self.eof != 0 && self.strm.avail_in == 0 {
                break;
            } else {
                if self.gz_make() == -1 {
                    return -1;
                }
            }
        }
        0
    }
}

pub fn gzread(file: &mut GzState, buf: &mut [u8], len: c_uint) -> c_int {
    if file.mode != GZ_READ || file.err != Z_OK {
        return -1;
    }
    if len == 0 {
        return 0;
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_skip(file.skip) == -1 {
            return -1;
        }
    }

    let mut got = 0;
    let mut remaining = len;
    let mut dest = buf.as_mut_ptr();
    while remaining > 0 {
        if file.have > 0 {
            let n = if file.have > remaining {
                remaining
            } else {
                file.have
            };
            unsafe {
                ptr::copy_nonoverlapping(file.next, dest, n as usize);
                file.next = file.next.add(n as usize);
                dest = dest.add(n as usize);
            }
            file.have -= n;
            remaining -= n;
            got += n;
            file.pos += n as u64;
        } else if file.eof != 0 && file.strm.avail_in == 0 {
            break;
        } else if file.how == LOOK || remaining < (file.size << 1) {
            if file.gz_make() == -1 {
                return -1;
            }
        } else if file.how == COPY {
            let mut n = 0;
            if file.gz_load(unsafe { slice::from_raw_parts_mut(dest, remaining as usize) }, remaining, &mut n) == -1 {
                return -1;
            }
            remaining -= n;
            got += n;
            file.pos += n as u64;
            dest = unsafe { dest.add(n as usize) };
        } else {
            file.strm.avail_out = remaining;
            file.strm.next_out = dest;
            if file.gz_decomp() == -1 {
                return -1;
            }
            let n = file.have;
            file.have = 0;
            remaining -= n;
            got += n;
            file.pos += n as u64;
            dest = unsafe { dest.add(n as usize) };
        }
    }
    got as c_int
}

pub fn gzgetc(file: &mut GzState) -> c_int {
    if file.mode != GZ_READ || file.err != Z_OK {
        return -1;
    }

    if file.have > 0 {
        file.have -= 1;
        file.pos += 1;
        let byte = unsafe { *file.next };
        file.next = unsafe { file.next.add(1) };
        return byte as c_int;
    }

    let mut buf = [0];
    if gzread(file, &mut buf, 1) < 1 {
        -1
    } else {
        buf[0] as c_int
    }
}

pub fn gzungetc(c: c_int, file: &mut GzState) -> c_int {
    if file.mode != GZ_READ || file.err != Z_OK {
        return -1;
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_skip(file.skip) == -1 {
            return -1;
        }
    }

    if c < 0 {
        return -1;
    }

    if file.have == 0 {
        file.have = 1;
        file.next = unsafe { file.out_buf.as_mut_ptr().add((file.size << 1) as usize - 1) };
        unsafe { *file.next = c as u8 };
        file.pos -= 1;
        return c;
    }

    if file.have == (file.size << 1) {
        file.gz_error(Z_BUF_ERROR, "out of room to push characters");
        return -1;
    }

    if file.next == file.out_buf.as_mut_ptr() {
        let mut src = unsafe { file.out_buf.as_mut_ptr().add(file.have as usize) };
        let mut dest = unsafe { file.out_buf.as_mut_ptr().add((file.size << 1) as usize) };
        while src > file.out_buf.as_mut_ptr() {
            src = unsafe { src.sub(1) };
            dest = unsafe { dest.sub(1) };
            unsafe { *dest = *src };
        }
        file.next = dest;
    }
    file.have += 1;
    file.next = unsafe { file.next.sub(1) };
    unsafe { *file.next = c as u8 };
    file.pos -= 1;
    c
}

pub fn gzgets(file: &mut GzState, buf: &mut [u8], len: c_int) -> *mut c_char {
    if len < 1 {
        return ptr::null_mut();
    }

    if file.mode != GZ_READ || file.err != Z_OK {
        return ptr::null_mut();
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_skip(file.skip) == -1 {
            return ptr::null_mut();
        }
    }

    let mut left = (len - 1) as usize;
