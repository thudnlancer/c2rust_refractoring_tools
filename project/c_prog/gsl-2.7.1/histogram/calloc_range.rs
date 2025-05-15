use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
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
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram_calloc_range(
    mut n: size_t,
    mut range: *mut libc::c_double,
) -> *mut gsl_histogram {
    let mut i: size_t = 0;
    let mut h: *mut gsl_histogram = 0 as *mut gsl_histogram;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"calloc_range.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        if *range.offset(i as isize)
            >= *range.offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        {
            gsl_error(
                b"histogram bin extremes  must be in increasing order\0" as *const u8
                    as *const libc::c_char,
                b"calloc_range.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return 0 as *mut gsl_histogram;
        }
        i = i.wrapping_add(1);
        i;
    }
    h = malloc(::core::mem::size_of::<gsl_histogram>() as libc::c_ulong)
        as *mut gsl_histogram;
    if h.is_null() {
        gsl_error(
            b"failed to allocate space for histogram struct\0" as *const u8
                as *const libc::c_char,
            b"calloc_range.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram;
    }
    (*h)
        .range = malloc(
        n
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).range).is_null() {
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram ranges\0" as *const u8
                as *const libc::c_char,
            b"calloc_range.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram;
    }
    (*h)
        .bin = malloc(
        n.wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*h).bin).is_null() {
        free((*h).range as *mut libc::c_void);
        free(h as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram bins\0" as *const u8
                as *const libc::c_char,
            b"calloc_range.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram;
    }
    i = 0 as libc::c_int as size_t;
    while i <= n {
        *((*h).range).offset(i as isize) = *range.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        *((*h).bin).offset(i as isize) = 0 as libc::c_int as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    (*h).n = n;
    return h;
}
