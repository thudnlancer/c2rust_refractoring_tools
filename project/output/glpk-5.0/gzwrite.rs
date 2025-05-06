#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn _glp_zlib_deflateInit2_(
        strm: z_streamp,
        level: i32,
        method: i32,
        windowBits: i32,
        memLevel: i32,
        strategy: i32,
        version: *const i8,
        stream_size: i32,
    ) -> i32;
    fn _glp_zlib_gz_error(_: gz_statep, _: i32, _: *const i8);
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn _glp_zlib_write(fd: i32, buf: *const libc::c_void, nbyte: u64) -> i64;
    fn _glp_zlib_close(fd: i32) -> i32;
    fn _glp_zlib_deflate(strm: z_streamp, flush: i32) -> i32;
    fn _glp_zlib_deflateEnd(strm: z_streamp) -> i32;
    fn _glp_zlib_deflateReset(strm: z_streamp) -> i32;
    fn _glp_zlib_deflateParams(strm: z_streamp, level: i32, strategy: i32) -> i32;
    fn vsprintf(_: *mut i8, _: *const i8, _: ::core::ffi::VaList) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpc = *const libc::c_void;
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
pub type z_streamp = *mut z_stream;
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
unsafe extern "C" fn gz_init(mut state: gz_statep) -> i32 {
    let mut ret: i32 = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    (*state).in_0 = malloc((*state).want as u64) as *mut u8;
    (*state).out = malloc((*state).want as u64) as *mut u8;
    if ((*state).in_0).is_null() || ((*state).out).is_null() {
        if !((*state).out).is_null() {
            free((*state).out as *mut libc::c_void);
        }
        if !((*state).in_0).is_null() {
            free((*state).in_0 as *mut libc::c_void);
        }
        _glp_zlib_gz_error(
            state,
            -(4 as i32),
            b"out of memory\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    (*strm).zalloc = None;
    (*strm).zfree = None;
    (*strm).opaque = 0 as voidpf;
    ret = _glp_zlib_deflateInit2_(
        strm,
        (*state).level,
        8 as i32,
        15 as i32 + 16 as i32,
        8 as i32,
        (*state).strategy,
        b"1.2.5\0" as *const u8 as *const i8,
        ::core::mem::size_of::<z_stream>() as u64 as i32,
    );
    if ret != 0 as i32 {
        free((*state).in_0 as *mut libc::c_void);
        _glp_zlib_gz_error(
            state,
            -(4 as i32),
            b"out of memory\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    (*state).size = (*state).want;
    (*strm).avail_out = (*state).size;
    (*strm).next_out = (*state).out;
    (*state).next = (*strm).next_out;
    return 0 as i32;
}
unsafe extern "C" fn gz_comp(mut state: gz_statep, mut flush: i32) -> i32 {
    let mut ret: i32 = 0;
    let mut got: i32 = 0;
    let mut have: u32 = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).size == 0 as i32 as u32 && gz_init(state) == -(1 as i32) {
        return -(1 as i32);
    }
    ret = 0 as i32;
    loop {
        if (*strm).avail_out == 0 as i32 as u32
            || flush != 0 as i32 && (flush != 4 as i32 || ret == 1 as i32)
        {
            have = ((*strm).next_out).offset_from((*state).next) as i64 as u32;
            if have != 0
                && {
                    got = _glp_zlib_write(
                        (*state).fd,
                        (*state).next as *const libc::c_void,
                        have as u64,
                    ) as i32;
                    got < 0 as i32 || got as u32 != have
                }
            {
                _glp_zlib_gz_error(state, -(1 as i32), strerror(*__errno_location()));
                return -(1 as i32);
            }
            if (*strm).avail_out == 0 as i32 as u32 {
                (*strm).avail_out = (*state).size;
                (*strm).next_out = (*state).out;
            }
            (*state).next = (*strm).next_out;
        }
        have = (*strm).avail_out;
        ret = _glp_zlib_deflate(strm, flush);
        if ret == -(2 as i32) {
            _glp_zlib_gz_error(
                state,
                -(2 as i32),
                b"internal error: deflate stream corrupt\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        have = have.wrapping_sub((*strm).avail_out);
        if !(have != 0) {
            break;
        }
    }
    if flush == 4 as i32 {
        _glp_zlib_deflateReset(strm);
    }
    return 0 as i32;
}
unsafe extern "C" fn gz_zero(mut state: gz_statep, mut len: i64) -> i32 {
    let mut first: i32 = 0;
    let mut n: u32 = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    if (*strm).avail_in != 0 && gz_comp(state, 0 as i32) == -(1 as i32) {
        return -(1 as i32);
    }
    first = 1 as i32;
    while len != 0 {
        n = if ::core::mem::size_of::<i32>() as u64
            == ::core::mem::size_of::<i64>() as u64
            && (*state).size > 2147483647 as i32 as u32 || (*state).size as i64 > len
        {
            len as u32
        } else {
            (*state).size
        };
        if first != 0 {
            memset((*state).in_0 as *mut libc::c_void, 0 as i32, n as u64);
            first = 0 as i32;
        }
        (*strm).avail_in = n;
        (*strm).next_in = (*state).in_0;
        (*state).pos += n as i64;
        if gz_comp(state, 0 as i32) == -(1 as i32) {
            return -(1 as i32);
        }
        len -= n as i64;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzwrite(
    mut file: gzFile,
    mut buf: voidpc,
    mut len: u32,
) -> i32 {
    let mut put: u32 = len;
    let mut n: u32 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return 0 as i32;
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as i32 || (*state).err != 0 as i32 {
        return 0 as i32;
    }
    if (len as i32) < 0 as i32 {
        _glp_zlib_gz_error(
            state,
            -(5 as i32),
            b"requested length does not fit in int\0" as *const u8 as *const i8,
        );
        return 0 as i32;
    }
    if len == 0 as i32 as u32 {
        return 0 as i32;
    }
    if (*state).size == 0 as i32 as u32 && gz_init(state) == -(1 as i32) {
        return 0 as i32;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_zero(state, (*state).skip) == -(1 as i32) {
            return 0 as i32;
        }
    }
    if len < (*state).size {
        loop {
            if (*strm).avail_in == 0 as i32 as u32 {
                (*strm).next_in = (*state).in_0;
            }
            n = ((*state).size).wrapping_sub((*strm).avail_in);
            if n > len {
                n = len;
            }
            memcpy(
                ((*strm).next_in).offset((*strm).avail_in as isize) as *mut libc::c_void,
                buf,
                n as u64,
            );
            (*strm).avail_in = ((*strm).avail_in as u32).wrapping_add(n) as uInt as uInt;
            (*state).pos += n as i64;
            buf = (buf as *mut i8).offset(n as isize) as voidpc;
            len = len.wrapping_sub(n);
            if len != 0 && gz_comp(state, 0 as i32) == -(1 as i32) {
                return 0 as i32;
            }
            if !(len != 0) {
                break;
            }
        }
    } else {
        if (*strm).avail_in != 0 && gz_comp(state, 0 as i32) == -(1 as i32) {
            return 0 as i32;
        }
        (*strm).avail_in = len;
        (*strm).next_in = buf as voidp as *mut Bytef;
        (*state).pos += len as i64;
        if gz_comp(state, 0 as i32) == -(1 as i32) {
            return 0 as i32;
        }
    }
    return put as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzputc(mut file: gzFile, mut c: i32) -> i32 {
    let mut buf: [u8; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as i32 || (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_zero(state, (*state).skip) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    if (*strm).avail_in < (*state).size {
        if (*strm).avail_in == 0 as i32 as u32 {
            (*strm).next_in = (*state).in_0;
        }
        let fresh0 = (*strm).avail_in;
        (*strm).avail_in = ((*strm).avail_in).wrapping_add(1);
        *((*strm).next_in).offset(fresh0 as isize) = c as Bytef;
        (*state).pos += 1;
        (*state).pos;
        return c;
    }
    buf[0 as i32 as usize] = c as u8;
    if _glp_zlib_gzwrite(file, buf.as_mut_ptr() as voidpc, 1 as i32 as u32) != 1 as i32 {
        return -(1 as i32);
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzputs(mut file: gzFile, mut str: *const i8) -> i32 {
    let mut ret: i32 = 0;
    let mut len: u32 = 0;
    len = strlen(str) as u32;
    ret = _glp_zlib_gzwrite(file, str as voidpc, len);
    return if ret == 0 as i32 && len != 0 as i32 as u32 { -(1 as i32) } else { ret };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzprintf(
    mut file: gzFile,
    mut format: *const i8,
    mut args: ...
) -> i32 {
    let mut size: i32 = 0;
    let mut len: i32 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    let mut va: ::core::ffi::VaListImpl;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as i32 || (*state).err != 0 as i32 {
        return 0 as i32;
    }
    if (*state).size == 0 as i32 as u32 && gz_init(state) == -(1 as i32) {
        return 0 as i32;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_zero(state, (*state).skip) == -(1 as i32) {
            return 0 as i32;
        }
    }
    if (*strm).avail_in != 0 && gz_comp(state, 0 as i32) == -(1 as i32) {
        return 0 as i32;
    }
    size = (*state).size as i32;
    *((*state).in_0).offset((size - 1 as i32) as isize) = 0 as i32 as u8;
    va = args.clone();
    len = vsprintf((*state).in_0 as *mut i8, format, va.as_va_list());
    if len <= 0 as i32 || len >= size
        || *((*state).in_0).offset((size - 1 as i32) as isize) as i32 != 0 as i32
    {
        return 0 as i32;
    }
    (*strm).avail_in = len as u32;
    (*strm).next_in = (*state).in_0;
    (*state).pos += len as i64;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzflush(mut file: gzFile, mut flush: i32) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 31153 as i32 || (*state).err != 0 as i32 {
        return -(2 as i32);
    }
    if flush < 0 as i32 || flush > 4 as i32 {
        return -(2 as i32);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_zero(state, (*state).skip) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    gz_comp(state, flush);
    return (*state).err;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzsetparams(
    mut file: gzFile,
    mut level: i32,
    mut strategy: i32,
) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(2 as i32);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as i32 || (*state).err != 0 as i32 {
        return -(2 as i32);
    }
    if level == (*state).level && strategy == (*state).strategy {
        return 0 as i32;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_zero(state, (*state).skip) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0 && gz_comp(state, 1 as i32) == -(1 as i32) {
            return (*state).err;
        }
        _glp_zlib_deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzclose_w(mut file: gzFile) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 31153 as i32 {
        return -(2 as i32);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        ret += gz_zero(state, (*state).skip);
    }
    ret += gz_comp(state, 4 as i32);
    _glp_zlib_deflateEnd(&mut (*state).strm);
    free((*state).out as *mut libc::c_void);
    free((*state).in_0 as *mut libc::c_void);
    _glp_zlib_gz_error(state, 0 as i32, 0 as *const i8);
    free((*state).path as *mut libc::c_void);
    ret += _glp_zlib_close((*state).fd);
    free(state as *mut libc::c_void);
    return if ret != 0 { -(1 as i32) } else { 0 as i32 };
}