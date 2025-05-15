use std::ffi::{CString, OsStr};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use std::os::unix::prelude::*;
use std::path::Path;
use std::ptr;
use libc::{c_int, c_void, off64_t};
use zlib_sys::*;

struct GzState {
    mode: c_int,
    fd: c_int,
    path: CString,
    size: usize,
    want: usize,
    msg: Option<CString>,
    err: c_int,
    seek: bool,
    skip: off64_t,
    pos: off64_t,
    start: off64_t,
    eof: bool,
    direct: bool,
    have: usize,
    next: *mut u8,
    raw: off64_t,
    how: c_int,
    strm: z_stream,
}

impl GzState {
    fn reset(&mut self) {
        if self.mode == GZ_READ {
            self.have = 0;
            self.eof = false;
            self.how = LOOK;
            self.direct = true;
        }
        self.seek = false;
        self.err = Z_OK;
        self.msg = None;
        self.pos = 0;
        self.strm.avail_in = 0;
    }

    fn open(path: &Path, fd: c_int, mode: &str) -> Option<Box<GzState>> {
        let mut state = Box::new(GzState {
            mode: GZ_NONE,
            fd: -1,
            path: CString::new(path.as_os_str().as_bytes()).ok()?,
            size: 0,
            want: GZBUFSIZE as usize,
            msg: None,
            err: Z_OK,
            seek: false,
            skip: 0,
            pos: 0,
            start: 0,
            eof: false,
            direct: false,
            have: 0,
            next: ptr::null_mut(),
            raw: 0,
            how: 0,
            strm: z_stream {
                next_in: ptr::null_mut(),
                avail_in: 0,
                total_in: 0,
                next_out: ptr::null_mut(),
                avail_out: 0,
                total_out: 0,
                msg: ptr::null_mut(),
                state: ptr::null_mut(),
                zalloc,
                zfree,
                opaque: ptr::null_mut(),
                data_type: 0,
                adler: 0,
                reserved: 0,
            },
        });

        for c in mode.chars() {
            match c {
                '0'..='9' => state.level = c as c_int - '0' as c_int,
                'r' => state.mode = GZ_READ,
                'w' => state.mode = GZ_WRITE,
                'a' => state.mode = GZ_APPEND,
                '+' => return None,
                'b' => (),
                'f' => state.strategy = Z_FILTERED,
                'h' => state.strategy = Z_HUFFMAN_ONLY,
                'R' => state.strategy = Z_RLE,
                'F' => state.strategy = Z_FIXED,
                _ => (),
            }
        }

        if state.mode == GZ_NONE {
            return None;
        }

        state.fd = if fd != -1 {
            fd
        } else {
            let file = OpenOptions::new()
                .read(state.mode == GZ_READ)
                .write(state.mode != GZ_READ)
                .create(state.mode != GZ_READ)
                .truncate(state.mode == GZ_WRITE)
                .append(state.mode == GZ_APPEND)
                .open(path)
                .ok()?
                .into_raw_fd();
            file
        };

        if state.fd == -1 {
            return None;
        }

        if state.mode == GZ_APPEND {
            state.mode = GZ_WRITE;
        }

        if state.mode == GZ_READ {
            state.start = unsafe { libc::lseek(state.fd, 0, SEEK_CUR) };
            if state.start == -1 {
                state.start = 0;
            }
        }

        state.reset();
        Some(state)
    }

    fn error(&mut self, err: c_int, msg: Option<&str>) {
        if self.err != Z_MEM_ERROR {
            self.msg = None;
        }

        self.err = err;
        if msg.is_none() {
            return;
        }

        if err == Z_MEM_ERROR {
            self.msg = Some(CString::new(msg.unwrap()).unwrap());
            return;
        }

        let combined = format!(
            "{}: {}",
            self.path.to_str().unwrap(),
            msg.unwrap()
        );
        self.msg = Some(CString::new(combined).unwrap());
    }
}

pub fn gzopen(path: &Path, mode: &str) -> Option<Box<GzState>> {
    GzState::open(path, -1, mode)
}

pub fn gzopen64(path: &Path, mode: &str) -> Option<Box<GzState>> {
    GzState::open(path, -1, mode)
}

pub fn gzdopen(fd: c_int, mode: &str) -> Option<Box<GzState>> {
    let path = Path::new(&format!("<fd:{}>", fd));
    GzState::open(path, fd, mode)
}

pub fn gzbuffer(state: &mut GzState, size: usize) -> c_int {
    if state.size != 0 {
        return -1;
    }
    if size == 0 {
        return -1;
    }
    state.want = size;
    0
}

pub fn gzrewind(state: &mut GzState) -> c_int {
    if state.mode != GZ_READ || state.err != Z_OK {
        return -1;
    }
    if unsafe { libc::lseek(state.fd, state.start, SEEK_SET) } == -1 {
        return -1;
    }
    state.reset();
    0
}

pub fn gzseek64(state: &mut GzState, offset: off64_t, whence: c_int) -> off64_t {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return -1;
    }
    if state.err != Z_OK {
        return -1;
    }
    if whence != SEEK_SET && whence != SEEK_CUR {
        return -1;
    }

    let mut offset = offset;
    if whence == SEEK_SET {
        offset -= state.pos;
    } else if state.seek {
        offset += state.skip;
    }
    state.seek = false;

    if state.mode == GZ_READ && state.how == COPY && state.pos + offset >= state.raw {
        let ret = unsafe { libc::lseek(state.fd, offset - state.have as off64_t, SEEK_CUR) };
        if ret == -1 {
            return -1;
        }
        state.have = 0;
        state.eof = false;
        state.seek = false;
        state.error(Z_OK, None);
        state.strm.avail_in = 0;
        state.pos += offset;
        return state.pos;
    }

    if offset < 0 {
        if state.mode != GZ_READ {
            return -1;
        }
        offset += state.pos;
        if offset < 0 {
            return -1;
        }
        if gzrewind(state) == -1 {
            return -1;
        }
    }

    if state.mode == GZ_READ {
        let n = if state.have as off64_t > offset {
            offset as usize
        } else {
            state.have
        };
        state.have -= n;
        unsafe {
            state.next = state.next.add(n);
        }
        state.pos += n as off64_t;
        offset -= n as off64_t;
    }

    if offset != 0 {
        state.seek = true;
        state.skip = offset;
    }
    state.pos + offset
}

pub fn gztell64(state: &GzState) -> off64_t {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return -1;
    }
    state.pos + if state.seek { state.skip } else { 0 }
}

pub fn gzoffset64(state: &GzState) -> off64_t {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return -1;
    }
    let offset = unsafe { libc::lseek(state.fd, 0, SEEK_CUR) };
    if offset == -1 {
        return -1;
    }
    if state.mode == GZ_READ {
        offset - state.strm.avail_in as off64_t
    } else {
        offset
    }
}

pub fn gzeof(state: &GzState) -> c_int {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return 0;
    }
    if state.mode == GZ_READ {
        (state.eof && state.strm.avail_in == 0 && state.have == 0) as c_int
    } else {
        0
    }
}

pub fn gzerror(state: &GzState, errnum: Option<&mut c_int>) -> Option<&str> {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return None;
    }
    if let Some(err) = errnum {
        *err = state.err;
    }
    state.msg.as_ref().map(|m| m.to_str().unwrap_or(""))
}

pub fn gzclearerr(state: &mut GzState) {
    if state.mode != GZ_READ && state.mode != GZ_WRITE {
        return;
    }
    if state.mode == GZ_READ {
        state.eof = false;
    }
    state.error(Z_OK, None);
}