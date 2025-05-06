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
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_block_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> i32;
    fn gsl_block_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> i32;
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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_fread(
    mut stream: *mut FILE,
    mut h: *mut gsl_histogram2d,
) -> i32 {
    let mut status: i32 = gsl_block_raw_fread(
        stream,
        (*h).xrange,
        ((*h).nx).wrapping_add(1 as i32 as u64),
        1 as i32 as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fread(
        stream,
        (*h).yrange,
        ((*h).ny).wrapping_add(1 as i32 as u64),
        1 as i32 as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fread(
        stream,
        (*h).bin,
        ((*h).nx).wrapping_mul((*h).ny),
        1 as i32 as size_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_fwrite(
    mut stream: *mut FILE,
    mut h: *const gsl_histogram2d,
) -> i32 {
    let mut status: i32 = gsl_block_raw_fwrite(
        stream,
        (*h).xrange,
        ((*h).nx).wrapping_add(1 as i32 as u64),
        1 as i32 as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fwrite(
        stream,
        (*h).yrange,
        ((*h).ny).wrapping_add(1 as i32 as u64),
        1 as i32 as size_t,
    );
    if status != 0 {
        return status;
    }
    status = gsl_block_raw_fwrite(
        stream,
        (*h).bin,
        ((*h).nx).wrapping_mul((*h).ny),
        1 as i32 as size_t,
    );
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_fprintf(
    mut stream: *mut FILE,
    mut h: *const gsl_histogram2d,
    mut range_format: *const i8,
    mut bin_format: *const i8,
) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut status: i32 = 0;
    i = 0 as i32 as size_t;
    while i < nx {
        j = 0 as i32 as size_t;
        while j < ny {
            status = fprintf(stream, range_format, *((*h).xrange).offset(i as isize));
            if status < 0 as i32 {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    78 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = _IO_putc(' ' as i32, stream);
            if status == -(1 as i32) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    85 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = fprintf(
                stream,
                range_format,
                *((*h).xrange).offset(i.wrapping_add(1 as i32 as u64) as isize),
            );
            if status < 0 as i32 {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    92 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = _IO_putc(' ' as i32, stream);
            if status == -(1 as i32) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    99 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = fprintf(stream, range_format, *((*h).yrange).offset(j as isize));
            if status < 0 as i32 {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    106 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = _IO_putc(' ' as i32, stream);
            if status == -(1 as i32) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    113 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = fprintf(
                stream,
                range_format,
                *((*h).yrange).offset(j.wrapping_add(1 as i32 as u64) as isize),
            );
            if status < 0 as i32 {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    120 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = _IO_putc(' ' as i32, stream);
            if status == -(1 as i32) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    127 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = fprintf(
                stream,
                bin_format,
                *((*h).bin).offset(i.wrapping_mul(ny).wrapping_add(j) as isize),
            );
            if status < 0 as i32 {
                gsl_error(
                    b"fprintf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    134 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            status = _IO_putc('\n' as i32, stream);
            if status == -(1 as i32) {
                gsl_error(
                    b"putc failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    141 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            j = j.wrapping_add(1);
            j;
        }
        status = _IO_putc('\n' as i32, stream);
        if status == -(1 as i32) {
            gsl_error(
                b"putc failed\0" as *const u8 as *const i8,
                b"file2d.c\0" as *const u8 as *const i8,
                148 as i32,
                GSL_EFAILED as i32,
            );
            return GSL_EFAILED as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_fscanf(
    mut stream: *mut FILE,
    mut h: *mut gsl_histogram2d,
) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    let mut xupper: libc::c_double = 0.;
    let mut yupper: libc::c_double = 0.;
    i = 0 as i32 as size_t;
    while i < nx {
        j = 0 as i32 as size_t;
        while j < ny {
            let mut status: i32 = fscanf(
                stream,
                b"%lg %lg %lg %lg %lg\0" as *const u8 as *const i8,
                ((*h).xrange).offset(i as isize),
                &mut xupper as *mut libc::c_double,
                ((*h).yrange).offset(j as isize),
                &mut yupper as *mut libc::c_double,
                ((*h).bin).offset(i.wrapping_mul(ny) as isize).offset(j as isize),
            );
            if status != 5 as i32 {
                gsl_error(
                    b"fscanf failed\0" as *const u8 as *const i8,
                    b"file2d.c\0" as *const u8 as *const i8,
                    175 as i32,
                    GSL_EFAILED as i32,
                );
                return GSL_EFAILED as i32;
            }
            j = j.wrapping_add(1);
            j;
        }
        *((*h).yrange).offset(ny as isize) = yupper;
        i = i.wrapping_add(1);
        i;
    }
    *((*h).xrange).offset(nx as isize) = xupper;
    return GSL_SUCCESS as i32;
}