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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn vsnprintf(_: *mut i8, _: u64, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
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
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type va_list = __builtin_va_list;
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_fmtstream {
    pub stream: *mut FILE,
    pub lmargin: size_t,
    pub rmargin: size_t,
    pub wmargin: ssize_t,
    pub point_offs: size_t,
    pub point_col: ssize_t,
    pub buf: *mut i8,
    pub p: *mut i8,
    pub end: *mut i8,
}
pub type argp_fmtstream_t = *mut argp_fmtstream;
#[inline]
unsafe extern "C" fn putc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn argp_make_fmtstream(
    mut stream: *mut FILE,
    mut lmargin: size_t,
    mut rmargin: size_t,
    mut wmargin: ssize_t,
) -> argp_fmtstream_t {
    let mut fs: argp_fmtstream_t = 0 as *mut argp_fmtstream;
    fs = malloc(::core::mem::size_of::<argp_fmtstream>() as u64) as *mut argp_fmtstream;
    if !fs.is_null() {
        (*fs).stream = stream;
        (*fs).lmargin = lmargin;
        (*fs).rmargin = rmargin;
        (*fs).wmargin = wmargin;
        (*fs).point_col = 0 as i32 as ssize_t;
        (*fs).point_offs = 0 as i32 as size_t;
        (*fs).buf = malloc(200 as i32 as u64) as *mut i8;
        if ((*fs).buf).is_null() {
            free(fs as *mut libc::c_void);
            fs = 0 as argp_fmtstream_t;
        } else {
            (*fs).p = (*fs).buf;
            (*fs).end = ((*fs).buf).offset(200 as i32 as isize);
        }
    }
    return fs;
}
#[no_mangle]
pub unsafe extern "C" fn argp_fmtstream_free(mut fs: argp_fmtstream_t) {
    _argp_fmtstream_update(fs);
    if (*fs).p > (*fs).buf {
        if 0 != 0 && 0 != 0
            && (1 as i32 as size_t)
                .wrapping_mul(((*fs).p).offset_from((*fs).buf) as i64 as size_t)
                <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
        {
            ({
                let mut __ptr: *const i8 = (*fs).buf as *const i8;
                let mut __stream: *mut FILE = (*fs).stream;
                let mut __cnt: size_t = 0;
                __cnt = (1 as i32 as size_t)
                    .wrapping_mul(((*fs).p).offset_from((*fs).buf) as i64 as size_t);
                while __cnt > 0 as i32 as u64 {
                    if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                        as i32 as i64 != 0
                    {
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        __overflow(__stream, *fresh1 as u8 as i32)
                    } else {
                        let fresh2 = __ptr;
                        __ptr = __ptr.offset(1);
                        let fresh3 = (*__stream)._IO_write_ptr;
                        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                            .offset(1);
                        *fresh3 = *fresh2;
                        *fresh3 as u8 as i32
                    }) == -(1 as i32)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                compile_error!("Binary expression is not supposed to be used")
            });
        } else {
            if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                || 0 != 0
                    && ((*fs).p).offset_from((*fs).buf) as i64 as size_t
                        == 0 as i32 as u64
            {} else {
                fwrite_unlocked(
                    (*fs).buf as *const libc::c_void,
                    1 as i32 as size_t,
                    ((*fs).p).offset_from((*fs).buf) as i64 as size_t,
                    (*fs).stream,
                );
            };
        };
        compile_error!("Conditional expression is not supposed to be used");
    }
    free((*fs).buf as *mut libc::c_void);
    free(fs as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _argp_fmtstream_update(mut fs: argp_fmtstream_t) {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut nl: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    buf = ((*fs).buf).offset((*fs).point_offs as isize);
    while buf < (*fs).p {
        let mut r: size_t = 0;
        if (*fs).point_col == 0 as i32 as i64 && (*fs).lmargin != 0 as i32 as u64 {
            let pad: size_t = (*fs).lmargin;
            if ((*fs).p).offset(pad as isize) < (*fs).end {
                memmove(
                    buf.offset(pad as isize) as *mut libc::c_void,
                    buf as *const libc::c_void,
                    ((*fs).p).offset_from(buf) as i64 as u64,
                );
                (*fs).p = ((*fs).p).offset(pad as isize);
                memset(buf as *mut libc::c_void, ' ' as i32, pad);
                buf = buf.offset(pad as isize);
            } else {
                let mut i: size_t = 0;
                i = 0 as i32 as size_t;
                while i < pad {
                    putc_unlocked(' ' as i32, (*fs).stream);
                    i = i.wrapping_add(1);
                    i;
                }
            }
            (*fs).point_col = pad as ssize_t;
        }
        len = ((*fs).p).offset_from(buf) as i64 as size_t;
        nl = memchr(buf as *const libc::c_void, '\n' as i32, len) as *mut i8;
        if (*fs).point_col < 0 as i32 as i64 {
            (*fs).point_col = 0 as i32 as ssize_t;
        }
        if nl.is_null() {
            if ((*fs).point_col as u64).wrapping_add(len) < (*fs).rmargin {
                (*fs).point_col = ((*fs).point_col as u64).wrapping_add(len) as ssize_t
                    as ssize_t;
                break;
            } else {
                nl = (*fs).p;
            }
        } else if ((*fs).point_col + nl.offset_from(buf) as i64)
            < (*fs).rmargin as ssize_t
        {
            (*fs).point_col = 0 as i32 as ssize_t;
            buf = nl.offset(1 as i32 as isize);
            continue;
        }
        r = ((*fs).rmargin).wrapping_sub(1 as i32 as u64);
        if (*fs).wmargin < 0 as i32 as i64 {
            if nl < (*fs).p {
                memmove(
                    buf.offset(r.wrapping_sub((*fs).point_col as u64) as isize)
                        as *mut libc::c_void,
                    nl as *const libc::c_void,
                    ((*fs).p).offset_from(nl) as i64 as u64,
                );
                (*fs).p = ((*fs).p)
                    .offset(
                        -(buf
                            .offset(r.wrapping_sub((*fs).point_col as u64) as isize)
                            .offset_from(nl) as i64 as isize),
                    );
                (*fs).point_col = 0 as i32 as ssize_t;
                buf = buf.offset(r.wrapping_add(1 as i32 as u64) as isize);
            } else {
                (*fs).point_col = ((*fs).point_col as u64).wrapping_add(len) as ssize_t
                    as ssize_t;
                (*fs).p = ((*fs).p)
                    .offset(-(((*fs).point_col as u64).wrapping_sub(r) as isize));
                break;
            }
        } else {
            let mut p: *mut i8 = 0 as *mut i8;
            let mut nextline: *mut i8 = 0 as *mut i8;
            let mut i_0: i32 = 0;
            p = buf
                .offset(
                    r.wrapping_add(1 as i32 as u64).wrapping_sub((*fs).point_col as u64)
                        as isize,
                );
            while p >= buf
                && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                    & _ISblank as i32 as libc::c_ushort as i32 == 0
            {
                p = p.offset(-1);
                p;
            }
            nextline = p.offset(1 as i32 as isize);
            if nextline > buf {
                if p >= buf {
                    loop {
                        p = p.offset(-1);
                        p;
                        if !(p >= buf
                            && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize)
                                as i32 & _ISblank as i32 as libc::c_ushort as i32 != 0)
                        {
                            break;
                        }
                    }
                }
                nl = p.offset(1 as i32 as isize);
            } else {
                p = buf
                    .offset(
                        r
                            .wrapping_add(1 as i32 as u64)
                            .wrapping_sub((*fs).point_col as u64) as isize,
                    );
                if p < nl {
                    loop {
                        p = p.offset(1);
                        p;
                        if !(p < nl
                            && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize)
                                as i32 & _ISblank as i32 as libc::c_ushort as i32 == 0)
                        {
                            break;
                        }
                    }
                }
                if p == nl {
                    (*fs).point_col = 0 as i32 as ssize_t;
                    buf = nl.offset(1 as i32 as isize);
                    continue;
                } else {
                    nl = p;
                    loop {
                        p = p.offset(1);
                        p;
                        if !(*(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                            & _ISblank as i32 as libc::c_ushort as i32 != 0)
                        {
                            break;
                        }
                    }
                    nextline = p;
                }
            }
            if (if nextline == buf.offset(len as isize).offset(1 as i32 as isize) {
                ((((*fs).end).offset_from(nl) as i64) < (*fs).wmargin + 1 as i32 as i64)
                    as i32
            } else {
                ((nextline.offset_from(nl.offset(1 as i32 as isize)) as i64)
                    < (*fs).wmargin) as i32
            }) != 0 && (*fs).p > nextline
            {
                if ((*fs).end).offset_from((*fs).p) as i64
                    > (*fs).wmargin + 1 as i32 as i64
                {
                    let mut mv: size_t = ((*fs).p).offset_from(nextline) as i64
                        as size_t;
                    memmove(
                        nl.offset(1 as i32 as isize).offset((*fs).wmargin as isize)
                            as *mut libc::c_void,
                        nextline as *const libc::c_void,
                        mv,
                    );
                    nextline = nl
                        .offset(1 as i32 as isize)
                        .offset((*fs).wmargin as isize);
                    len = nextline.offset(mv as isize).offset_from(buf) as i64 as size_t;
                    let fresh4 = nl;
                    nl = nl.offset(1);
                    *fresh4 = '\n' as i32 as i8;
                } else {
                    if nl > (*fs).buf {
                        if 0 != 0 && 0 != 0
                            && (1 as i32 as size_t)
                                .wrapping_mul(nl.offset_from((*fs).buf) as i64 as size_t)
                                <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
                        {
                            ({
                                let mut __ptr: *const i8 = (*fs).buf as *const i8;
                                let mut __stream: *mut FILE = (*fs).stream;
                                let mut __cnt: size_t = 0;
                                __cnt = (1 as i32 as size_t)
                                    .wrapping_mul(nl.offset_from((*fs).buf) as i64 as size_t);
                                while __cnt > 0 as i32 as u64 {
                                    if (if ((*__stream)._IO_write_ptr
                                        >= (*__stream)._IO_write_end) as i32 as i64 != 0
                                    {
                                        let fresh5 = __ptr;
                                        __ptr = __ptr.offset(1);
                                        __overflow(__stream, *fresh5 as u8 as i32)
                                    } else {
                                        let fresh6 = __ptr;
                                        __ptr = __ptr.offset(1);
                                        let fresh7 = (*__stream)._IO_write_ptr;
                                        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                            .offset(1);
                                        *fresh7 = *fresh6;
                                        *fresh7 as u8 as i32
                                    }) == -(1 as i32)
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                    __cnt;
                                }
                                compile_error!(
                                    "Binary expression is not supposed to be used"
                                )
                            });
                        } else {
                            if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                                || 0 != 0
                                    && nl.offset_from((*fs).buf) as i64 as size_t
                                        == 0 as i32 as u64
                            {} else {
                                fwrite_unlocked(
                                    (*fs).buf as *const libc::c_void,
                                    1 as i32 as size_t,
                                    nl.offset_from((*fs).buf) as i64 as size_t,
                                    (*fs).stream,
                                );
                            };
                        };
                        compile_error!(
                            "Conditional expression is not supposed to be used"
                        );
                    }
                    putc_unlocked('\n' as i32, (*fs).stream);
                    len = (len as u64)
                        .wrapping_add(buf.offset_from((*fs).buf) as i64 as u64) as size_t
                        as size_t;
                    buf = (*fs).buf;
                    nl = buf;
                }
            } else {
                let fresh8 = nl;
                nl = nl.offset(1);
                *fresh8 = '\n' as i32 as i8;
            }
            if nextline.offset_from(nl) as i64 >= (*fs).wmargin
                || nextline == buf.offset(len as isize).offset(1 as i32 as isize)
                    && ((*fs).end).offset_from(nextline) as i64 >= (*fs).wmargin
            {
                i_0 = 0 as i32;
                while (i_0 as i64) < (*fs).wmargin {
                    let fresh9 = nl;
                    nl = nl.offset(1);
                    *fresh9 = ' ' as i32 as i8;
                    i_0 += 1;
                    i_0;
                }
            } else {
                i_0 = 0 as i32;
                while (i_0 as i64) < (*fs).wmargin {
                    putc_unlocked(' ' as i32, (*fs).stream);
                    i_0 += 1;
                    i_0;
                }
            }
            if nl < nextline {
                memmove(
                    nl as *mut libc::c_void,
                    nextline as *const libc::c_void,
                    buf.offset(len as isize).offset_from(nextline) as i64 as u64,
                );
            }
            len = (len as u64).wrapping_sub(nextline.offset_from(buf) as i64 as u64)
                as size_t as size_t;
            buf = nl;
            (*fs).p = nl.offset(len as isize);
            (*fs).point_col = if (*fs).wmargin != 0 {
                (*fs).wmargin
            } else {
                -(1 as i32) as i64
            };
        }
    }
    (*fs).point_offs = ((*fs).p).offset_from((*fs).buf) as i64 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn _argp_fmtstream_ensure(
    mut fs: *mut argp_fmtstream,
    mut amount: size_t,
) -> i32 {
    if (((*fs).end).offset_from((*fs).p) as i64 as size_t) < amount {
        let mut wrote: ssize_t = 0;
        _argp_fmtstream_update(fs);
        wrote = (if 0 != 0 && 0 != 0
            && (1 as i32 as size_t)
                .wrapping_mul(((*fs).p).offset_from((*fs).buf) as i64 as size_t)
                <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
        {
            ({
                let mut __ptr: *const i8 = (*fs).buf as *const i8;
                let mut __stream: *mut FILE = (*fs).stream;
                let mut __cnt: size_t = 0;
                __cnt = (1 as i32 as size_t)
                    .wrapping_mul(((*fs).p).offset_from((*fs).buf) as i64 as size_t);
                while __cnt > 0 as i32 as u64 {
                    if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                        as i32 as i64 != 0
                    {
                        let fresh10 = __ptr;
                        __ptr = __ptr.offset(1);
                        __overflow(__stream, *fresh10 as u8 as i32)
                    } else {
                        let fresh11 = __ptr;
                        __ptr = __ptr.offset(1);
                        let fresh12 = (*__stream)._IO_write_ptr;
                        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                            .offset(1);
                        *fresh12 = *fresh11;
                        *fresh12 as u8 as i32
                    }) == -(1 as i32)
                    {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                    __cnt;
                }
                (1 as i32 as size_t)
                    .wrapping_mul(((*fs).p).offset_from((*fs).buf) as i64 as size_t)
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as i32 as size_t)
            })
        } else if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
            || 0 != 0
                && ((*fs).p).offset_from((*fs).buf) as i64 as size_t == 0 as i32 as u64
        {
            0 as i32 as size_t
        } else {
            fwrite_unlocked(
                (*fs).buf as *const libc::c_void,
                1 as i32 as size_t,
                ((*fs).p).offset_from((*fs).buf) as i64 as size_t,
                (*fs).stream,
            )
        }) as ssize_t;
        if wrote == ((*fs).p).offset_from((*fs).buf) as i64 {
            (*fs).p = (*fs).buf;
            (*fs).point_offs = 0 as i32 as size_t;
        } else {
            (*fs).p = ((*fs).p).offset(-(wrote as isize));
            (*fs).point_offs = ((*fs).point_offs as u64).wrapping_sub(wrote as u64)
                as size_t as size_t;
            memmove(
                (*fs).buf as *mut libc::c_void,
                ((*fs).buf).offset(wrote as isize) as *const libc::c_void,
                ((*fs).p).offset_from((*fs).buf) as i64 as u64,
            );
            return 0 as i32;
        }
        if (((*fs).end).offset_from((*fs).buf) as i64 as size_t) < amount {
            let mut old_size: size_t = ((*fs).end).offset_from((*fs).buf) as i64
                as size_t;
            let mut new_size: size_t = old_size.wrapping_add(amount);
            let mut new_buf: *mut i8 = 0 as *mut i8;
            if new_size < old_size
                || {
                    new_buf = realloc((*fs).buf as *mut libc::c_void, new_size)
                        as *mut i8;
                    new_buf.is_null()
                }
            {
                *__errno_location() = 12 as i32;
                return 0 as i32;
            }
            (*fs).buf = new_buf;
            (*fs).end = new_buf.offset(new_size as isize);
            (*fs).p = (*fs).buf;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn argp_fmtstream_printf(
    mut fs: *mut argp_fmtstream,
    mut fmt: *const i8,
    mut args: ...
) -> ssize_t {
    let mut out: i32 = 0;
    let mut avail: size_t = 0;
    let mut size_guess: size_t = 150 as i32 as size_t;
    loop {
        let mut args_0: ::core::ffi::VaListImpl;
        if _argp_fmtstream_ensure(fs, size_guess) == 0 {
            return -(1 as i32) as ssize_t;
        }
        args_0 = args.clone();
        avail = ((*fs).end).offset_from((*fs).p) as i64 as size_t;
        out = vsnprintf((*fs).p, avail, fmt, args_0.as_va_list());
        if out as size_t >= avail {
            size_guess = (out + 1 as i32) as size_t;
        }
        if !(out as size_t >= avail) {
            break;
        }
    }
    (*fs).p = ((*fs).p).offset(out as isize);
    return out as ssize_t;
}