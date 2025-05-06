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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
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
pub unsafe extern "C" fn gsl_histogram2d_alloc(
    nx: size_t,
    ny: size_t,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if nx == 0 as i32 as u64 {
        gsl_error(
            b"histogram2d length nx must be positive integer\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            34 as i32,
            GSL_EDOM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ny == 0 as i32 as u64 {
        gsl_error(
            b"histogram2d length ny must be positive integer\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            40 as i32,
            GSL_EDOM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = malloc(::core::mem::size_of::<gsl_histogram2d>() as u64) as *mut gsl_histogram2d;
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram2d struct\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            48 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).xrange = malloc(
        nx
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).xrange).is_null() {
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d x ranges\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            58 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).yrange = malloc(
        ny
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).yrange).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d y ranges\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            69 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).bin = malloc(
        nx.wrapping_mul(ny).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).bin).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free((*h).yrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram bins\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).nx = nx;
    (*h).ny = ny;
    return h;
}
unsafe extern "C" fn make_uniform(
    mut range: *mut libc::c_double,
    mut n: size_t,
    mut xmin: libc::c_double,
    mut xmax: libc::c_double,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i <= n {
        let mut f1: libc::c_double = n.wrapping_sub(i) as libc::c_double
            / n as libc::c_double;
        let mut f2: libc::c_double = i as libc::c_double / n as libc::c_double;
        *range.offset(i as isize) = f1 * xmin + f2 * xmax;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_calloc_uniform(
    nx: size_t,
    ny: size_t,
    xmin: libc::c_double,
    xmax: libc::c_double,
    ymin: libc::c_double,
    ymax: libc::c_double,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if xmin >= xmax {
        gsl_error(
            b"xmin must be less than xmax\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            112 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ymin >= ymax {
        gsl_error(
            b"ymin must be less than ymax\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            117 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = gsl_histogram2d_calloc(nx, ny);
    if h.is_null() {
        return h;
    }
    make_uniform((*h).xrange, nx, xmin, xmax);
    make_uniform((*h).yrange, ny, ymin, ymax);
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_calloc(
    nx: size_t,
    ny: size_t,
) -> *mut gsl_histogram2d {
    let mut h: *mut gsl_histogram2d = 0 as *mut gsl_histogram2d;
    if nx == 0 as i32 as u64 {
        gsl_error(
            b"histogram2d length nx must be positive integer\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            141 as i32,
            GSL_EDOM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    if ny == 0 as i32 as u64 {
        gsl_error(
            b"histogram2d length ny must be positive integer\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            147 as i32,
            GSL_EDOM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    h = malloc(::core::mem::size_of::<gsl_histogram2d>() as u64) as *mut gsl_histogram2d;
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram2d struct\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            155 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).xrange = malloc(
        nx
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).xrange).is_null() {
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d x ranges\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            165 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).yrange = malloc(
        ny
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).yrange).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d y ranges\0" as *const u8
                as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            176 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    (*h).bin = malloc(
        nx.wrapping_mul(ny).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*h).bin).is_null() {
        free((*h).xrange as *mut libc::c_void);
        free((*h).yrange as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram bins\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            188 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_histogram2d;
    }
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < nx.wrapping_add(1 as i32 as u64) {
        *((*h).xrange).offset(i as isize) = i as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i < ny.wrapping_add(1 as i32 as u64) {
        *((*h).yrange).offset(i as isize) = i as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as i32 as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    (*h).nx = nx;
    (*h).ny = ny;
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_free(mut h: *mut gsl_histogram2d) {
    if h.is_null() {
        return;
    }
    free((*h).xrange as *mut libc::c_void);
    free((*h).yrange as *mut libc::c_void);
    free((*h).bin as *mut libc::c_void);
    free(h as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_set_ranges_uniform(
    mut h: *mut gsl_histogram2d,
    mut xmin: libc::c_double,
    mut xmax: libc::c_double,
    mut ymin: libc::c_double,
    mut ymax: libc::c_double,
) -> i32 {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    if xmin >= xmax {
        gsl_error(
            b"xmin must be less than xmax\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            238 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as i32;
    }
    if ymin >= ymax {
        gsl_error(
            b"ymin must be less than ymax\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            243 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as i32;
    }
    make_uniform((*h).xrange, nx, xmin, xmax);
    make_uniform((*h).yrange, ny, ymin, ymax);
    i = 0 as i32 as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as i32 as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_set_ranges(
    mut h: *mut gsl_histogram2d,
    mut xrange: *const libc::c_double,
    mut xsize: size_t,
    mut yrange: *const libc::c_double,
    mut ysize: size_t,
) -> i32 {
    let mut i: size_t = 0;
    let nx: size_t = (*h).nx;
    let ny: size_t = (*h).ny;
    if xsize != nx.wrapping_add(1 as i32 as u64) {
        gsl_error(
            b"size of xrange must match size of histogram\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            272 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as i32;
    }
    if ysize != ny.wrapping_add(1 as i32 as u64) {
        gsl_error(
            b"size of yrange must match size of histogram\0" as *const u8 as *const i8,
            b"init2d.c\0" as *const u8 as *const i8,
            278 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as i32;
    }
    i = 0 as i32 as size_t;
    while i <= nx {
        *((*h).xrange).offset(i as isize) = *xrange.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i <= ny {
        *((*h).yrange).offset(i as isize) = *yrange.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as size_t;
    while i < nx.wrapping_mul(ny) {
        *((*h).bin).offset(i as isize) = 0 as i32 as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}