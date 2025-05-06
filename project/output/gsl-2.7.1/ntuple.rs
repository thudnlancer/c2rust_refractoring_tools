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
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
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
    fn feof(__stream: *mut FILE) -> i32;
    fn gsl_histogram_increment(h: *mut gsl_histogram, x: libc::c_double) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type C2RustUnnamed = i32;
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
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple {
    pub file: *mut FILE,
    pub ntuple_data: *mut libc::c_void,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple_select_fn {
    pub function: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> i32,
    >,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ntuple_value_fn {
    pub function: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_create(
    mut filename: *mut i8,
    mut ntuple_data: *mut libc::c_void,
    mut size: size_t,
) -> *mut gsl_ntuple {
    let mut ntuple: *mut gsl_ntuple = malloc(::core::mem::size_of::<gsl_ntuple>() as u64)
        as *mut gsl_ntuple;
    if ntuple.is_null() {
        gsl_error(
            b"failed to allocate space for ntuple struct\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            40 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_ntuple;
    }
    (*ntuple).ntuple_data = ntuple_data;
    (*ntuple).size = size;
    (*ntuple).file = fopen(filename, b"wb\0" as *const u8 as *const i8);
    if ((*ntuple).file).is_null() {
        free(ntuple as *mut libc::c_void);
        gsl_error(
            b"unable to create ntuple file\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            51 as i32,
            GSL_EFAILED as i32,
        );
        return 0 as *mut gsl_ntuple;
    }
    return ntuple;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_open(
    mut filename: *mut i8,
    mut ntuple_data: *mut libc::c_void,
    mut size: size_t,
) -> *mut gsl_ntuple {
    let mut ntuple: *mut gsl_ntuple = malloc(::core::mem::size_of::<gsl_ntuple>() as u64)
        as *mut gsl_ntuple;
    if ntuple.is_null() {
        gsl_error(
            b"failed to allocate space for ntuple struct\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            70 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_ntuple;
    }
    (*ntuple).ntuple_data = ntuple_data;
    (*ntuple).size = size;
    (*ntuple).file = fopen(filename, b"rb\0" as *const u8 as *const i8);
    if ((*ntuple).file).is_null() {
        free(ntuple as *mut libc::c_void);
        gsl_error(
            b"unable to open ntuple file for reading\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            82 as i32,
            GSL_EFAILED as i32,
        );
        return 0 as *mut gsl_ntuple;
    }
    return ntuple;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_write(mut ntuple: *mut gsl_ntuple) -> i32 {
    let mut nwrite: size_t = 0;
    nwrite = fwrite(
        (*ntuple).ntuple_data,
        (*ntuple).size,
        1 as i32 as size_t,
        (*ntuple).file,
    );
    if nwrite != 1 as i32 as u64 {
        gsl_error(
            b"failed to write ntuple entry to file\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            103 as i32,
            GSL_EFAILED as i32,
        );
        return GSL_EFAILED as i32;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_bookdata(mut ntuple: *mut gsl_ntuple) -> i32 {
    return gsl_ntuple_write(ntuple);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_read(mut ntuple: *mut gsl_ntuple) -> i32 {
    let mut nread: size_t = 0;
    nread = fread(
        (*ntuple).ntuple_data,
        (*ntuple).size,
        1 as i32 as size_t,
        (*ntuple).file,
    );
    if nread == 0 as i32 as u64 && feof((*ntuple).file) != 0 {
        return GSL_EOF as i32;
    }
    if nread != 1 as i32 as u64 {
        gsl_error(
            b"failed to read ntuple entry from file\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            136 as i32,
            GSL_EFAILED as i32,
        );
        return GSL_EFAILED as i32;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_project(
    mut h: *mut gsl_histogram,
    mut ntuple: *mut gsl_ntuple,
    mut value_func: *mut gsl_ntuple_value_fn,
    mut select_func: *mut gsl_ntuple_select_fn,
) -> i32 {
    let mut nread: size_t = 0;
    loop {
        nread = fread(
            (*ntuple).ntuple_data,
            (*ntuple).size,
            1 as i32 as size_t,
            (*ntuple).file,
        );
        if nread == 0 as i32 as u64 && feof((*ntuple).file) != 0 {
            break;
        }
        if nread != 1 as i32 as u64 {
            gsl_error(
                b"failed to read ntuple for projection\0" as *const u8 as *const i8,
                b"ntuple.c\0" as *const u8 as *const i8,
                170 as i32,
                GSL_EFAILED as i32,
            );
            return GSL_EFAILED as i32;
        }
        if (Some(((*select_func).function).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*ntuple).ntuple_data, (*select_func).params) != 0
        {
            gsl_histogram_increment(
                h,
                (Some(((*value_func).function).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )((*ntuple).ntuple_data, (*value_func).params),
            );
        }
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ntuple_close(mut ntuple: *mut gsl_ntuple) -> i32 {
    let mut status: i32 = fclose((*ntuple).file);
    if status != 0 {
        gsl_error(
            b"failed to close ntuple file\0" as *const u8 as *const i8,
            b"ntuple.c\0" as *const u8 as *const i8,
            196 as i32,
            GSL_EFAILED as i32,
        );
        return GSL_EFAILED as i32;
    }
    free(ntuple as *mut libc::c_void);
    return GSL_SUCCESS as i32;
}