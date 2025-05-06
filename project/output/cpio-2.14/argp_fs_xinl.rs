#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
extern "C" {
    fn _argp_fmtstream_ensure(__fs: argp_fmtstream_t, __amount: size_t) -> i32;
    fn _argp_fmtstream_update(__fs: argp_fmtstream_t);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_point(mut __fs: argp_fmtstream_t) -> size_t {
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    return (if (*__fs).point_col >= 0 as i32 as i64 {
        (*__fs).point_col
    } else {
        0 as i32 as i64
    }) as size_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_wmargin(
    mut __fs: argp_fmtstream_t,
    mut __wmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).wmargin as size_t;
    (*__fs).wmargin = __wmargin as ssize_t;
    return __old;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_rmargin(
    mut __fs: argp_fmtstream_t,
    mut __rmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).rmargin;
    (*__fs).rmargin = __rmargin;
    return __old;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_set_lmargin(
    mut __fs: argp_fmtstream_t,
    mut __lmargin: size_t,
) -> size_t {
    let mut __old: size_t = 0;
    if ((*__fs).p).offset_from((*__fs).buf) as i64 as size_t > (*__fs).point_offs {
        _argp_fmtstream_update(__fs);
    }
    __old = (*__fs).lmargin;
    (*__fs).lmargin = __lmargin;
    return __old;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_putc(
    mut __fs: argp_fmtstream_t,
    mut __ch: i32,
) -> i32 {
    if (*__fs).p < (*__fs).end || _argp_fmtstream_ensure(__fs, 1 as i32 as size_t) != 0 {
        let fresh0 = (*__fs).p;
        (*__fs).p = ((*__fs).p).offset(1);
        *fresh0 = __ch as i8;
        return *fresh0 as i32;
    } else {
        return -(1 as i32)
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_puts(
    mut __fs: argp_fmtstream_t,
    mut __str: *const i8,
) -> i32 {
    let mut __len: size_t = strlen(__str);
    if __len != 0 {
        let mut __wrote: size_t = argp_fmtstream_write(__fs, __str, __len);
        return if __wrote == __len { 0 as i32 } else { -(1 as i32) };
    } else {
        return 0 as i32
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn argp_fmtstream_write(
    mut __fs: argp_fmtstream_t,
    mut __str: *const i8,
    mut __len: size_t,
) -> size_t {
    if ((*__fs).p).offset(__len as isize) <= (*__fs).end
        || _argp_fmtstream_ensure(__fs, __len) != 0
    {
        memcpy((*__fs).p as *mut libc::c_void, __str as *const libc::c_void, __len);
        (*__fs).p = ((*__fs).p).offset(__len as isize);
        return __len;
    } else {
        return 0 as i32 as size_t
    };
}