use libc::{c_int, c_void, size_t, off64_t};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::mem;
use std::os::unix::io::RawFd;
use std::ptr;
use zlib::{Deflate, Z_BUF_ERROR, Z_ERRNO, Z_FINISH, Z_MEM_ERROR, Z_NO_FLUSH, Z_OK, Z_PARTIAL_FLUSH, Z_STREAM_END, Z_STREAM_ERROR};

struct GzState {
    fd: RawFd,
    path: Option<CString>,
    mode: c_int,
    size: size_t,
    want: size_t,
    level: c_int,
    strategy: c_int,
    err: c_int,
    err_msg: Option<CString>,
    seek: bool,
    skip: off64_t,
    in_buf: Option<Vec<u8>>,
    out_buf: Option<Vec<u8>>,
    next: *mut u8,
    pos: off64_t,
    strm: Deflate,
}

impl GzState {
    fn new(fd: RawFd, path: Option<CString>, mode: c_int) -> Self {
        GzState {
            fd,
            path,
            mode,
            size: 0,
            want: 1 << 15, // Default buffer size
            level: Z_DEFAULT_COMPRESSION,
            strategy: Z_DEFAULT_STRATEGY,
            err: Z_OK,
            err_msg: None,
            seek: false,
            skip: 0,
            in_buf: None,
            out_buf: None,
            next: ptr::null_mut(),
            pos: 0,
            strm: Deflate::new(),
        }
    }

    fn gz_error(&mut self, err: c_int, msg: &str) {
        self.err = err;
        self.err_msg = Some(CString::new(msg).unwrap();
    }

    fn gz_init(&mut self) -> c_int {
        // Allocate input and output buffers
        self.in_buf = Some(vec![0; self.want]);
        self.out_buf = Some(vec![0; self.want]);

        if self.in_buf.is_none() || self.out_buf.is_none() {
            self.gz_error(Z_MEM_ERROR, "out of memory");
            return -1;
        }

        // Set up deflate for gzip compression
        if self.strm.deflate_init2(
            self.level,
            Z_DEFLATED,
            15 + 16,
            8,
            self.strategy,
        ) != Z_OK
        {
            self.gz_error(Z_MEM_ERROR, "out of memory");
            return -1;
        }

        // Mark state as initialized
        self.size = self.want;

        // Initialize write buffer
        self.strm.avail_out = self.size as u32;
        self.strm.next_out = self.out_buf.as_mut().unwrap().as_mut_ptr();
        self.next = self.strm.next_out;
        0
    }

    fn gz_comp(&mut self, flush: c_int) -> c_int {
        if self.size == 0 && self.gz_init() == -1 {
            return -1;
        }

        let mut ret = Z_OK;
        loop {
            if self.strm.avail_out == 0
                || (flush != Z_NO_FLUSH && (flush != Z_FINISH || ret == Z_STREAM_END))
            {
                let have = unsafe { self.strm.next_out.offset_from(self.next) } as usize;
                if have > 0 {
                    let written = unsafe {
                        libc::write(
                            self.fd,
                            self.next as *const c_void,
                            have as size_t,
                        )
                    };
                    if written < 0 || written as usize != have {
                        self.gz_error(Z_ERRNO, "write error");
                        return -1;
                    }
                }

                if self.strm.avail_out == 0 {
                    self.strm.avail_out = self.size as u32;
                    self.strm.next_out = self.out_buf.as_mut().unwrap().as_mut_ptr();
                }
                self.next = self.strm.next_out;
            }

            let have = self.strm.avail_out;
            ret = self.strm.deflate(flush);
            if ret == Z_STREAM_ERROR {
                self.gz_error(Z_STREAM_ERROR, "internal error: deflate stream corrupt");
                return -1;
            }
            if have == self.strm.avail_out {
                break;
            }
        }

        if flush == Z_FINISH {
            self.strm.deflate_reset();
        }

        0
    }

    fn gz_zero(&mut self, len: off64_t) -> c_int {
        if self.strm.avail_in > 0 && self.gz_comp(Z_NO_FLUSH) == -1 {
            return -1;
        }

        let mut remaining = len;
        let mut first = true;
        while remaining > 0 {
            let n = if remaining > self.size as off64_t {
                self.size
            } else {
                remaining as usize
            };

            if first {
                self.in_buf.as_mut().unwrap()[..n].fill(0);
                first = false;
            }

            self.strm.avail_in = n as u32;
            self.strm.next_in = self.in_buf.as_mut().unwrap().as_mut_ptr();
            self.pos += n as off64_t;
            if self.gz_comp(Z_NO_FLUSH) == -1 {
                return -1;
            }
            remaining -= n as off64_t;
        }
        0
    }
}

pub fn gzwrite(file: *mut GzState, buf: *const c_void, len: size_t) -> c_int {
    if file.is_null() {
        return 0;
    }

    let state = unsafe { &mut *file };
    if state.mode != GZ_WRITE || state.err != Z_OK {
        return 0;
    }

    if len == 0 {
        return 0;
    }

    if state.size == 0 && state.gz_init() == -1 {
        return 0;
    }

    if state.seek {
        state.seek = false;
        if state.gz_zero(state.skip) == -1 {
            return 0;
        }
    }

    let buf_slice = unsafe { std::slice::from_raw_parts(buf as *const u8, len) };

    if len < state.size {
        let mut offset = 0;
        while offset < len {
            if state.strm.avail_in == 0 {
                state.strm.next_in = state.in_buf.as_mut().unwrap().as_mut_ptr();
            }

            let available = state.size - state.strm.avail_in as usize;
            let n = std::cmp::min(len - offset, available);
            unsafe {
                ptr::copy_nonoverlapping(
                    buf_slice.as_ptr().add(offset),
                    state.strm.next_in.add(state.strm.avail_in as usize),
                    n,
                );
            }
            state.strm.avail_in += n as u32;
            state.pos += n as off64_t;
            offset += n;

            if offset < len && state.gz_comp(Z_NO_FLUSH) == -1 {
                return 0;
            }
        }
    } else {
        if state.strm.avail_in > 0 && state.gz_comp(Z_NO_FLUSH) == -1 {
            return 0;
        }

        state.strm.avail_in = len as u32;
        state.strm.next_in = buf as *mut u8;
        state.pos += len as off64_t;
        if state.gz_comp(Z_NO_FLUSH) == -1 {
            return 0;
        }
    }

    len as c_int
}

pub fn gzputc(file: *mut GzState, c: c_int) -> c_int {
    if file.is_null() {
        return -1;
    }

    let state = unsafe { &mut *file };
    if state.mode != GZ_WRITE || state.err != Z_OK {
        return -1;
    }

    if state.seek {
        state.seek = false;
        if state.gz_zero(state.skip) == -1 {
            return -1;
        }
    }

    if state.strm.avail_in < state.size as u32 {
        if state.strm.avail_in == 0 {
            state.strm.next_in = state.in_buf.as_mut().unwrap().as_mut_ptr();
        }
        unsafe {
            *state.strm.next_in.add(state.strm.avail_in as usize) = c as u8;
        }
        state.strm.avail_in += 1;
        state.pos += 1;
        return c;
    }

    let buf = [c as u8];
    if gzwrite(file, buf.as_ptr() as *const c_void, 1) != 1 {
        -1
    } else {
        c
    }
}

pub fn gzputs(file: *mut GzState, str: *const c_char) -> c_int {
    if file.is_null() || str.is_null() {
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(str) };
    let len = c_str.to_bytes().len();
    let ret = gzwrite(file, str as *const c_void, len as size_t);
    if ret == 0 && len != 0 {
        -1
    } else {
        ret
    }
}

pub fn gzflush(file: *mut GzState, flush: c_int) -> c_int {
    if file.is_null() {
        return Z_STREAM_ERROR;
    }

    let state = unsafe { &mut *file };
    if state.mode != GZ_WRITE || state.err != Z_OK {
        return Z_STREAM_ERROR;
    }

    if flush < 0 || flush > Z_FINISH {
        return Z_STREAM_ERROR;
    }

    if state.seek {
        state.seek = false;
        if state.gz_zero(state.skip) == -1 {
            return -1;
        }
    }

    state.gz_comp(flush);
    state.err
}

pub fn gzsetparams(file: *mut GzState, level: c_int, strategy: c_int) -> c_int {
    if file.is_null() {
        return Z_STREAM_ERROR;
    }

    let state = unsafe { &mut *file };
    if state.mode != GZ_WRITE || state.err != Z_OK {
        return Z_STREAM_ERROR;
    }

    if level == state.level && strategy == state.strategy {
        return Z_OK;
    }

    if state.seek {
        state.seek = false;
        if state.gz_zero(state.skip) == -1 {
            return -1;
        }
    }

    if state.size != 0 {
        if state.strm.avail_in > 0 && state.gz_comp(Z_PARTIAL_FLUSH) == -1 {
            return state.err;
        }
        state.strm.deflate_params(level, strategy);
    }

    state.level = level;
    state.strategy = strategy;
    Z_OK
}

pub fn gzclose_w(file: *mut GzState) -> c_int {
    if file.is_null() {
        return Z_STREAM_ERROR;
    }

    let state = unsafe { &mut *file };
    if state.mode != GZ_WRITE {
        return Z_STREAM_ERROR;
    }

    let mut ret = 0;
    if state.seek {
        state.seek = false;
        ret += state.gz_zero(state.skip);
    }

    ret += state.gz_comp(Z_FINISH);
    state.strm.deflate_end();
    unsafe {
        libc::close(state.fd);
    }
    Z_OK
}