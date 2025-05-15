use libc::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use zlib::{deflate, deflateEnd, deflateInit2, deflateParams, deflateReset, ZStream};

const Z_NO_FLUSH: c_int = 0;
const Z_PARTIAL_FLUSH: c_int = 1;
const Z_SYNC_FLUSH: c_int = 2;
const Z_FULL_FLUSH: c_int = 3;
const Z_FINISH: c_int = 4;
const Z_OK: c_int = 0;
const Z_STREAM_END: c_int = 1;
const Z_ERRNO: c_int = -1;
const Z_STREAM_ERROR: c_int = -2;
const Z_DATA_ERROR: c_int = -3;
const Z_MEM_ERROR: c_int = -4;
const Z_BUF_ERROR: c_int = -5;
const Z_VERSION_ERROR: c_int = -6;

const GZ_WRITE: c_int = 31153;

struct GzState {
    mode: c_int,
    fd: c_int,
    path: CString,
    pos: c_long,
    size: c_uint,
    want: c_uint,
    in_buf: Vec<c_uchar>,
    out_buf: Vec<c_uchar>,
    next_out: usize,
    have: c_uint,
    eof: c_int,
    start: c_long,
    raw: c_long,
    how: c_int,
    direct: c_int,
    level: c_int,
    strategy: c_int,
    skip: c_long,
    seek: c_int,
    err: c_int,
    msg: CString,
    strm: ZStream,
}

impl GzState {
    fn new(fd: c_int, path: &str, mode: c_int) -> Result<Self, io::Error> {
        let path = CString::new(path).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?;
        let want = 8192; // Default buffer size
        Ok(Self {
            mode,
            fd,
            path,
            pos: 0,
            size: 0,
            want,
            in_buf: Vec::with_capacity(want as usize),
            out_buf: Vec::with_capacity(want as usize),
            next_out: 0,
            have: 0,
            eof: 0,
            start: 0,
            raw: 0,
            how: 0,
            direct: 0,
            level: 6, // Default compression level
            strategy: 0,
            skip: 0,
            seek: 0,
            err: 0,
            msg: CString::default(),
            strm: ZStream::default(),
        })
    }

    fn init(&mut self) -> Result<(), io::Error> {
        self.in_buf.resize(self.want as usize, 0);
        self.out_buf.resize(self.want as usize, 0);

        unsafe {
            let ret = deflateInit2(
                &mut self.strm,
                self.level,
                8,  // DEFLATED
                15 + 16, // windowBits + 16 for gzip
                8,  // memLevel
                self.strategy,
                b"1.2.5\0".as_ptr() as *const c_char,
                mem::size_of::<ZStream>() as c_int,
            );

            if ret != Z_OK {
                self.free_buffers();
                return Err(io::Error::new(io::ErrorKind::Other, "zlib initialization failed"));
            }
        }

        self.size = self.want;
        self.strm.avail_out = self.size;
        self.strm.next_out = self.out_buf.as_mut_ptr();
        self.next_out = 0;

        Ok(())
    }

    fn free_buffers(&mut self) {
        self.in_buf.clear();
        self.out_buf.clear();
    }

    fn gz_error(&mut self, err: c_int, msg: &str) {
        self.err = err;
        self.msg = CString::new(msg).unwrap_or_default();
    }

    fn gz_comp(&mut self, flush: c_int) -> Result<(), io::Error> {
        if self.size == 0 && self.init().is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "initialization failed"));
        }

        loop {
            if self.strm.avail_out == 0 || (flush != Z_NO_FLUSH && (flush != Z_FINISH || self.err == Z_STREAM_END)) {
                let have = (self.strm.next_out as usize - self.next_out) as c_uint;
                if have > 0 {
                    let written = unsafe {
                        libc::write(
                            self.fd,
                            self.out_buf.as_ptr().add(self.next_out) as *const c_void,
                            have as c_ulong,
                        )
                    };

                    if written < 0 || written as c_uint != have {
                        let err = io::Error::last_os_error();
                        self.gz_error(Z_ERRNO, &err.to_string());
                        return Err(err);
                    }
                }

                if self.strm.avail_out == 0 {
                    self.strm.avail_out = self.size;
                    self.strm.next_out = self.out_buf.as_mut_ptr();
                }
                self.next_out = (self.strm.next_out as usize - self.out_buf.as_ptr() as usize);
            }

            let have = self.strm.avail_out;
            let ret = unsafe { deflate(&mut self.strm, flush) };

            if ret == Z_STREAM_ERROR {
                self.gz_error(Z_STREAM_ERROR, "internal error: deflate stream corrupt");
                return Err(io::Error::new(io::ErrorKind::Other, "deflate stream error"));
            }

            if have - self.strm.avail_out == 0 {
                break;
            }
        }

        if flush == Z_FINISH {
            unsafe { deflateReset(&mut self.strm) };
        }

        Ok(())
    }

    fn gz_zero(&mut self, len: c_long) -> Result<(), io::Error> {
        if self.strm.avail_in != 0 && self.gz_comp(Z_NO_FLUSH).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "compression failed"));
        }

        let mut first = true;
        let mut remaining = len;

        while remaining > 0 {
            let n = if self.size as c_long > remaining {
                remaining as c_uint
            } else {
                self.size
            };

            if first {
                self.in_buf[..n as usize].fill(0);
                first = false;
            }

            self.strm.avail_in = n;
            self.strm.next_in = self.in_buf.as_mut_ptr();
            self.pos += n as c_long;

            if self.gz_comp(Z_NO_FLUSH).is_err() {
                return Err(io::Error::new(io::ErrorKind::Other, "compression failed"));
            }

            remaining -= n as c_long;
        }

        Ok(())
    }
}

pub fn gzwrite(file: &mut GzState, buf: &[u8]) -> Result<usize, io::Error> {
    if file.mode != GZ_WRITE || file.err != 0 {
        return Ok(0);
    }

    let len = buf.len() as c_uint;
    if len == 0 {
        return Ok(0);
    }

    if file.size == 0 && file.init().is_err() {
        return Ok(0);
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_zero(file.skip).is_err() {
            return Ok(0);
        }
    }

    if len < file.size {
        let mut remaining = buf;
        while !remaining.is_empty() {
            if file.strm.avail_in == 0 {
                file.strm.next_in = file.in_buf.as_mut_ptr();
            }

            let n = (file.size - file.strm.avail_in).min(remaining.len() as c_uint);
            unsafe {
                ptr::copy_nonoverlapping(
                    remaining.as_ptr(),
                    file.strm.next_in.add(file.strm.avail_in as usize),
                    n as usize,
                );
            }

            file.strm.avail_in += n;
            file.pos += n as c_long;
            remaining = &remaining[n as usize..];

            if !remaining.is_empty() && file.gz_comp(Z_NO_FLUSH).is_err() {
                return Ok(0);
            }
        }
    } else {
        if file.strm.avail_in != 0 && file.gz_comp(Z_NO_FLUSH).is_err() {
            return Ok(0);
        }

        file.strm.avail_in = len;
        file.strm.next_in = buf.as_ptr() as *mut _;
        file.pos += len as c_long;

        if file.gz_comp(Z_NO_FLUSH).is_err() {
            return Ok(0);
        }
    }

    Ok(len as usize)
}

pub fn gzputc(file: &mut GzState, c: u8) -> Result<u8, io::Error> {
    if file.mode != GZ_WRITE || file.err != 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid file state"));
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_zero(file.skip).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "seek failed"));
        }
    }

    if file.strm.avail_in < file.size {
        if file.strm.avail_in == 0 {
            file.strm.next_in = file.in_buf.as_mut_ptr();
        }
        unsafe {
            *file.strm.next_in.add(file.strm.avail_in as usize) = c;
        }
        file.strm.avail_in += 1;
        file.pos += 1;
        Ok(c)
    } else {
        gzwrite(file, &[c]).map(|_| c)
    }
}

pub fn gzputs(file: &mut GzState, s: &str) -> Result<usize, io::Error> {
    let len = s.len();
    let ret = gzwrite(file, s.as_bytes())?;
    Ok(if ret == 0 && len != 0 { 0 } else { ret })
}

pub fn gzflush(file: &mut GzState, flush: c_int) -> Result<(), io::Error> {
    if file.mode != GZ_WRITE || file.err != 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid file state"));
    }

    if flush < Z_NO_FLUSH || flush > Z_FINISH {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid flush value"));
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_zero(file.skip).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "seek failed"));
        }
    }

    file.gz_comp(flush)?;
    Ok(())
}

pub fn gzsetparams(
    file: &mut GzState,
    level: c_int,
    strategy: c_int,
) -> Result<(), io::Error> {
    if file.mode != GZ_WRITE || file.err != 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid file state"));
    }

    if level == file.level && strategy == file.strategy {
        return Ok(());
    }

    if file.seek != 0 {
        file.seek = 0;
        if file.gz_zero(file.skip).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "seek failed"));
        }
    }

    if file.size != 0 {
        if file.strm.avail_in != 0 && file.gz_comp(Z_PARTIAL_FLUSH).is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "compression failed"));
        }

        unsafe {
            deflateParams(&mut file.strm, level, strategy);
        }
    }

    file.level = level;
    file.strategy = strategy;
    Ok(())
}

pub fn gzclose_w(file: &mut GzState) -> Result<(), io::Error> {
    if file.mode != GZ_WRITE {
        return Err(io::Error::new(io::ErrorKind::Other, "invalid file state"));
    }

    let mut ret = Ok(());

    if file.seek != 0 {
        file.seek = 0;
        ret = file.gz_zero(file.skip);
    }

    if file.gz_comp(Z_FINISH).is_err() {
        ret = Err(io::Error::new(io::ErrorKind::Other, "compression failed"));
    }

    unsafe {
        deflateEnd(&mut file.strm);
    }

    file.free_buffers();
    file.gz_error(0, "");

    if unsafe { libc::close(file.fd) } != 0 {
        ret = Err(io::Error::last_os_error());
    }

    ret
}

#[repr(C)]
struct ZStream {
    next_in: *mut c_uchar,
    avail_in: c_uint,
    total_in: c_ulong,
    next_out: *mut c_uchar,
    avail_out: c_uint,
    total_out: c_ulong,
    msg: *mut c_char,
    state: *mut c_void,
    zalloc: Option<extern "C" fn(*mut c_void, c_uint, c_uint) -> *mut c_void>,
    zfree: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    opaque: *mut c_void,
    data_type: c_int,
    adler: c_ulong,
    reserved: c_ulong,
}

impl Default for ZStream {
    fn default() -> Self {
        Self {
            next_in: ptr::null_mut(),
            avail_in: 0,
            total_in: 0,
            next_out: ptr::null_mut(),
            avail_out: 0,
            total_out: 0,
            msg: ptr::null_mut(),
            state: ptr::null_mut(),
            zalloc: None,
            zfree: None,
            opaque: ptr::null_mut(),
            data_type: 0,
            adler: 0,
            reserved: 0,
        }
    }
}

mod zlib {
    use super::*;

    pub unsafe fn deflateInit2(
        strm: *mut ZStream,
        level: c_int,
        method: c_int,
        windowBits: c_int,
        memLevel: c_int,
        strategy: c_int,
        version: *const c_char,
        stream_size: c_int,
    ) -> c_int {
        libc::deflateInit2_(
            strm as *mut _,
            level,
            method,
            windowBits,
            memLevel,
            strategy,
            version,
            stream_size,
        )
    }

    pub unsafe fn deflate(strm: *mut ZStream, flush: c_int) -> c_int {
        libc::deflate(strm as *mut _, flush)
    }

    pub unsafe fn deflateEnd(strm: *mut ZStream) -> c_int {
        libc::deflateEnd(strm as *mut _)
    }

    pub unsafe fn deflateReset(strm: *mut ZStream) -> c_int {
        libc::deflateReset(strm as *mut _)
    }

    pub unsafe fn deflateParams(strm: *mut ZStream, level: c_int, strategy: c_int) -> c_int {
        libc::deflateParams(strm as *mut _, level, strategy)
    }
}