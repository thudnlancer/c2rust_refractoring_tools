use ::libc;
extern "C" {
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type C2RustUnnamed = libc::c_int;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_multiset_struct {
    pub n: size_t,
    pub k: size_t,
    pub data: *mut size_t,
}
pub type gsl_multiset = gsl_multiset_struct;
#[no_mangle]
pub unsafe extern "C" fn gsl_multiset_fread(
    mut stream: *mut FILE,
    mut c: *mut gsl_multiset,
) -> libc::c_int {
    let mut k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut items: size_t = fread(
        data as *mut libc::c_void,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        k,
        stream,
    );
    if items != k {
        gsl_error(
            b"fread failed\0" as *const u8 as *const libc::c_char,
            b"file.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multiset_fwrite(
    mut stream: *mut FILE,
    mut c: *const gsl_multiset,
) -> libc::c_int {
    let mut k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut items: size_t = fwrite(
        data as *const libc::c_void,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
        k,
        stream,
    );
    if items != k {
        gsl_error(
            b"fwrite failed\0" as *const u8 as *const libc::c_char,
            b"file.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            GSL_EFAILED as libc::c_int,
        );
        return GSL_EFAILED as libc::c_int;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multiset_fprintf(
    mut stream: *mut FILE,
    mut c: *const gsl_multiset,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < k {
        let mut status: libc::c_int = fprintf(stream, format, *data.offset(i as isize));
        if status < 0 as libc::c_int {
            gsl_error(
                b"fprintf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_multiset_fscanf(
    mut stream: *mut FILE,
    mut c: *mut gsl_multiset,
) -> libc::c_int {
    let mut k: size_t = (*c).k;
    let mut data: *mut size_t = (*c).data;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < k {
        let mut j: libc::c_ulong = 0;
        let mut status: libc::c_int = fscanf(
            stream,
            b"%lu\0" as *const u8 as *const libc::c_char,
            &mut j as *mut libc::c_ulong,
        );
        if status != 1 as libc::c_int {
            gsl_error(
                b"fscanf failed\0" as *const u8 as *const libc::c_char,
                b"file.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int,
                GSL_EFAILED as libc::c_int,
            );
            return GSL_EFAILED as libc::c_int;
        }
        *data.offset(i as isize) = j;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
