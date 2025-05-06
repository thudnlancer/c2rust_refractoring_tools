#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn _glp_zlib_open(path: *const i8, oflag: i32, _: ...) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn _glp_zlib_lseek(fd: i32, offset: i64, whence: i32) -> i64;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type voidp = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type gzFile = voidp;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub mode: i32,
    pub fd: i32,
    pub path: *mut i8,
    pub pos: i64,
    pub size: u32,
    pub want: u32,
    pub in_0: *mut u8,
    pub out: *mut u8,
    pub next: *mut u8,
    pub have: u32,
    pub eof: i32,
    pub start: i64,
    pub raw: i64,
    pub how: i32,
    pub direct: i32,
    pub level: i32,
    pub strategy: i32,
    pub skip: i64,
    pub seek: i32,
    pub err: i32,
    pub msg: *mut i8,
    pub strm: z_stream,
}
unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    if (*state).mode == 7247 as i32 {
        (*state).have = 0 as i32 as u32;
        (*state).eof = 0 as i32;
        (*state).how = 0 as i32;
        (*state).direct = 1 as i32;
    }
    (*state).seek = 0 as i32;
    _glp_zlib_gz_error(state, 0 as i32, 0 as *const i8);
    (*state).pos = 0 as i32 as i64;
    (*state).strm.avail_in = 0 as i32 as uInt;
}
unsafe extern "C" fn gz_open(
    mut path: *const i8,
    mut fd: i32,
    mut mode: *const i8,
) -> gzFile {
    let mut state: gz_statep = 0 as *mut gz_state;
    state = malloc(::core::mem::size_of::<gz_state>() as u64) as gz_statep;
    if state.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*state).size = 0 as i32 as u32;
    (*state).want = 8192 as i32 as u32;
    (*state).msg = 0 as *mut i8;
    (*state).mode = 0 as i32;
    (*state).level = -(1 as i32);
    (*state).strategy = 0 as i32;
    while *mode != 0 {
        if *mode as i32 >= '0' as i32 && *mode as i32 <= '9' as i32 {
            (*state).level = *mode as i32 - '0' as i32;
        } else {
            match *mode as i32 {
                114 => {
                    (*state).mode = 7247 as i32;
                }
                119 => {
                    (*state).mode = 31153 as i32;
                }
                97 => {
                    (*state).mode = 1 as i32;
                }
                43 => {
                    free(state as *mut libc::c_void);
                    return 0 as *mut libc::c_void;
                }
                98 => {}
                102 => {
                    (*state).strategy = 1 as i32;
                }
                104 => {
                    (*state).strategy = 2 as i32;
                }
                82 => {
                    (*state).strategy = 3 as i32;
                }
                70 => {
                    (*state).strategy = 4 as i32;
                }
                _ => {}
            }
        }
        mode = mode.offset(1);
        mode;
    }
    if (*state).mode == 0 as i32 {
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    (*state).path = malloc((strlen(path)).wrapping_add(1 as i32 as u64)) as *mut i8;
    if ((*state).path).is_null() {
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    strcpy((*state).path, path);
    (*state).fd = if fd != -(1 as i32) {
        fd
    } else {
        _glp_zlib_open(
            path,
            if (*state).mode == 7247 as i32 {
                0 as i32
            } else {
                0x1 as i32 | 0x10 as i32
                    | (if (*state).mode == 31153 as i32 {
                        0x20 as i32
                    } else {
                        0x30 as i32
                    })
            },
            0o666 as i32,
        )
    };
    if (*state).fd == -(1 as i32) {
        free((*state).path as *mut libc::c_void);
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    if (*state).mode == 1 as i32 {
        (*state).mode = 31153 as i32;
    }
    if (*state).mode == 7247 as i32 {
        (*state).start = _glp_zlib_lseek((*state).fd, 0 as i32 as i64, 1 as i32);
        if (*state).start == -(1 as i32) as i64 {
            (*state).start = 0 as i32 as i64;
        }
    }
    gz_reset(state);
    return state as gzFile;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzopen(
    mut path: *const i8,
    mut mode: *const i8,
) -> gzFile {
    return gz_open(path, -(1 as i32), mode);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzopen64(
    mut path: *const i8,
    mut mode: *const i8,
) -> gzFile {
    return gz_open(path, -(1 as i32), mode);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzdopen(mut fd: i32, mut mode: *const i8) -> gzFile {
    let mut path: *mut i8 = 0 as *mut i8;
    let mut gz: gzFile = 0 as *mut libc::c_void;
    if fd == -(1 as i32)
        || {
            path = malloc(
                (7 as i32 as u64)
                    .wrapping_add(
                        (3 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<i32>() as u64),
                    ),
            ) as *mut i8;
            path.is_null()
        }
    {
        return 0 as *mut libc::c_void;
    }
    sprintf(path, b"<fd:%d>\0" as *const u8 as *const i8, fd);
    gz = gz_open(path, fd, mode);
    free(path as *mut libc::c_void);
    return gz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzbuffer(mut file: gzFile, mut size: u32) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return -(1 as i32);
    }
    if (*state).size != 0 as i32 as u32 {
        return -(1 as i32);
    }
    if size == 0 as i32 as u32 {
        return -(1 as i32);
    }
    (*state).want = size;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzrewind(mut file: gzFile) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 || (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if _glp_zlib_lseek((*state).fd, (*state).start, 0 as i32) == -(1 as i32) as i64 {
        return -(1 as i32);
    }
    gz_reset(state);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzseek64(
    mut file: gzFile,
    mut offset: i64,
    mut whence: i32,
) -> i64 {
    let mut n: u32 = 0;
    let mut ret: i64 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32) as i64;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return -(1 as i32) as i64;
    }
    if (*state).err != 0 as i32 {
        return -(1 as i32) as i64;
    }
    if whence != 0 as i32 && whence != 1 as i32 {
        return -(1 as i32) as i64;
    }
    if whence == 0 as i32 {
        offset -= (*state).pos;
    } else if (*state).seek != 0 {
        offset += (*state).skip;
    }
    (*state).seek = 0 as i32;
    if (*state).mode == 7247 as i32 && (*state).how == 1 as i32
        && (*state).pos + offset >= (*state).raw
    {
        ret = _glp_zlib_lseek((*state).fd, offset - (*state).have as i64, 1 as i32);
        if ret == -(1 as i32) as i64 {
            return -(1 as i32) as i64;
        }
        (*state).have = 0 as i32 as u32;
        (*state).eof = 0 as i32;
        (*state).seek = 0 as i32;
        _glp_zlib_gz_error(state, 0 as i32, 0 as *const i8);
        (*state).strm.avail_in = 0 as i32 as uInt;
        (*state).pos += offset;
        return (*state).pos;
    }
    if offset < 0 as i32 as i64 {
        if (*state).mode != 7247 as i32 {
            return -(1 as i32) as i64;
        }
        offset += (*state).pos;
        if offset < 0 as i32 as i64 {
            return -(1 as i32) as i64;
        }
        if _glp_zlib_gzrewind(file) == -(1 as i32) {
            return -(1 as i32) as i64;
        }
    }
    if (*state).mode == 7247 as i32 {
        n = if ::core::mem::size_of::<i32>() as u64
            == ::core::mem::size_of::<i64>() as u64
            && (*state).have > 2147483647 as i32 as u32 || (*state).have as i64 > offset
        {
            offset as u32
        } else {
            (*state).have
        };
        (*state).have = ((*state).have).wrapping_sub(n);
        (*state).next = ((*state).next).offset(n as isize);
        (*state).pos += n as i64;
        offset -= n as i64;
    }
    if offset != 0 {
        (*state).seek = 1 as i32;
        (*state).skip = offset;
    }
    return (*state).pos + offset;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzseek(
    mut file: gzFile,
    mut offset: i64,
    mut whence: i32,
) -> i64 {
    let mut ret: i64 = 0;
    ret = _glp_zlib_gzseek64(file, offset, whence);
    return if ret == ret { ret } else { -(1 as i32) as i64 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gztell64(mut file: gzFile) -> i64 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32) as i64;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return -(1 as i32) as i64;
    }
    return (*state).pos
        + (if (*state).seek != 0 { (*state).skip } else { 0 as i32 as i64 });
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gztell(mut file: gzFile) -> i64 {
    let mut ret: i64 = 0;
    ret = _glp_zlib_gztell64(file);
    return if ret == ret { ret } else { -(1 as i32) as i64 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzoffset64(mut file: gzFile) -> i64 {
    let mut offset: i64 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32) as i64;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return -(1 as i32) as i64;
    }
    offset = _glp_zlib_lseek((*state).fd, 0 as i32 as i64, 1 as i32);
    if offset == -(1 as i32) as i64 {
        return -(1 as i32) as i64;
    }
    if (*state).mode == 7247 as i32 {
        offset -= (*state).strm.avail_in as i64;
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzoffset(mut file: gzFile) -> i64 {
    let mut ret: i64 = 0;
    ret = _glp_zlib_gzoffset64(file);
    return if ret == ret { ret } else { -(1 as i32) as i64 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzeof(mut file: gzFile) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as i32;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return 0 as i32;
    }
    return if (*state).mode == 7247 as i32 {
        ((*state).eof != 0 && (*state).strm.avail_in == 0 as i32 as u32
            && (*state).have == 0 as i32 as u32) as i32
    } else {
        0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzerror(
    mut file: gzFile,
    mut errnum: *mut i32,
) -> *const i8 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as *const i8;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return 0 as *const i8;
    }
    if !errnum.is_null() {
        *errnum = (*state).err;
    }
    return if ((*state).msg).is_null() {
        b"\0" as *const u8 as *const i8
    } else {
        (*state).msg
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 && (*state).mode != 31153 as i32 {
        return;
    }
    if (*state).mode == 7247 as i32 {
        (*state).eof = 0 as i32;
    }
    _glp_zlib_gz_error(state, 0 as i32, 0 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gz_error(
    mut state: gz_statep,
    mut err: i32,
    mut msg: *const i8,
) {
    if !((*state).msg).is_null() {
        if (*state).err != -(4 as i32) {
            free((*state).msg as *mut libc::c_void);
        }
        (*state).msg = 0 as *mut i8;
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == -(4 as i32) {
        (*state).msg = msg as *mut i8;
        return;
    }
    (*state).msg = malloc(
        (strlen((*state).path)).wrapping_add(strlen(msg)).wrapping_add(3 as i32 as u64),
    ) as *mut i8;
    if ((*state).msg).is_null() {
        (*state).err = -(4 as i32);
        (*state).msg = b"out of memory\0" as *const u8 as *const i8 as *mut i8;
        return;
    }
    strcpy((*state).msg, (*state).path);
    strcat((*state).msg, b": \0" as *const u8 as *const i8);
    strcat((*state).msg, msg);
}