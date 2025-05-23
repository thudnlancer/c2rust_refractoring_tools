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
pub struct gsl_histogram2d {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram2d_pdf {
    pub nx: size_t,
    pub ny: size_t,
    pub xrange: *mut libc::c_double,
    pub yrange: *mut libc::c_double,
    pub sum: *mut libc::c_double,
}
unsafe extern "C" fn find(
    n: size_t,
    mut range: *const libc::c_double,
    x: libc::c_double,
    mut i: *mut size_t,
) -> libc::c_int {
    let mut i_linear: size_t = 0;
    let mut lower: size_t = 0;
    let mut upper: size_t = 0;
    let mut mid: size_t = 0;
    if x < *range.offset(0 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    if x >= *range.offset(n as isize) {
        return 1 as libc::c_int;
    }
    let mut u: libc::c_double = (x - *range.offset(0 as libc::c_int as isize))
        / (*range.offset(n as isize) - *range.offset(0 as libc::c_int as isize));
    i_linear = (u * n as libc::c_double) as size_t;
    if x >= *range.offset(i_linear as isize)
        && x
            < *range
                .offset(
                    i_linear.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
    {
        *i = i_linear;
        return 0 as libc::c_int;
    }
    upper = n;
    lower = 0 as libc::c_int as size_t;
    while upper.wrapping_sub(lower) > 1 as libc::c_int as libc::c_ulong {
        mid = upper.wrapping_add(lower).wrapping_div(2 as libc::c_int as libc::c_ulong);
        if x >= *range.offset(mid as isize) {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    *i = lower;
    if x < *range.offset(lower as isize)
        || x
            >= *range
                .offset(lower.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
    {
        gsl_error(
            b"x not found in range\0" as *const u8 as *const libc::c_char,
            b"./find.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            GSL_ESANITY as libc::c_int,
        );
        return GSL_ESANITY as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_pdf_sample(
    mut p: *const gsl_histogram2d_pdf,
    mut r1: libc::c_double,
    mut r2: libc::c_double,
    mut x: *mut libc::c_double,
    mut y: *mut libc::c_double,
) -> libc::c_int {
    let mut k: size_t = 0;
    let mut status: libc::c_int = 0;
    if r2 == 1.0f64 {
        r2 = 0.0f64;
    }
    if r1 == 1.0f64 {
        r1 = 0.0f64;
    }
    status = find(
        ((*p).nx).wrapping_mul((*p).ny),
        (*p).sum as *const libc::c_double,
        r1,
        &mut k,
    );
    if status != 0 {
        gsl_error(
            b"cannot find r1 in cumulative pdf\0" as *const u8 as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    } else {
        let mut i: size_t = k.wrapping_div((*p).ny);
        let mut j: size_t = k.wrapping_sub(i.wrapping_mul((*p).ny));
        let mut delta: libc::c_double = (r1 - *((*p).sum).offset(k as isize))
            / (*((*p).sum)
                .offset(k.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                - *((*p).sum).offset(k as isize));
        *x = *((*p).xrange).offset(i as isize)
            + delta
                * (*((*p).xrange)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    - *((*p).xrange).offset(i as isize));
        *y = *((*p).yrange).offset(j as isize)
            + r2
                * (*((*p).yrange)
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    - *((*p).yrange).offset(j as isize));
        return GSL_SUCCESS as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_pdf_alloc(
    nx: size_t,
    ny: size_t,
) -> *mut gsl_histogram2d_pdf {
    let n: size_t = nx.wrapping_mul(ny);
    let mut p: *mut gsl_histogram2d_pdf = 0 as *mut gsl_histogram2d_pdf;
    if n == 0 as libc::c_int as libc::c_ulong {
        gsl_error(
            b"histogram2d pdf length n must be positive integer\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d_pdf;
    }
    p = malloc(::core::mem::size_of::<gsl_histogram2d_pdf>() as libc::c_ulong)
        as *mut gsl_histogram2d_pdf;
    if p.is_null() {
        gsl_error(
            b"failed to allocate space for histogram2d pdf struct\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d_pdf;
    }
    (*p)
        .xrange = malloc(
        nx
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*p).xrange).is_null() {
        free(p as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d pdf xranges\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d_pdf;
    }
    (*p)
        .yrange = malloc(
        ny
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*p).yrange).is_null() {
        free((*p).xrange as *mut libc::c_void);
        free(p as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d pdf yranges\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d_pdf;
    }
    (*p)
        .sum = malloc(
        n
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    if ((*p).sum).is_null() {
        free((*p).yrange as *mut libc::c_void);
        free((*p).xrange as *mut libc::c_void);
        free(p as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for histogram2d pdf sums\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_histogram2d_pdf;
    }
    (*p).nx = nx;
    (*p).ny = ny;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_pdf_init(
    mut p: *mut gsl_histogram2d_pdf,
    mut h: *const gsl_histogram2d,
) -> libc::c_int {
    let mut i: size_t = 0;
    let nx: size_t = (*p).nx;
    let ny: size_t = (*p).ny;
    let n: size_t = nx.wrapping_mul(ny);
    if nx != (*h).nx || ny != (*h).ny {
        gsl_error(
            b"histogram2d size must match pdf size\0" as *const u8
                as *const libc::c_char,
            b"pdf2d.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            GSL_EDOM as libc::c_int,
        );
        return GSL_EDOM as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < n {
        if *((*h).bin).offset(i as isize) < 0 as libc::c_int as libc::c_double {
            gsl_error(
                b"histogram bins must be non-negative to computea probability distribution\0"
                    as *const u8 as *const libc::c_char,
                b"pdf2d.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int,
                GSL_EDOM as libc::c_int,
            );
            return GSL_EDOM as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < nx.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *((*p).xrange).offset(i as isize) = *((*h).xrange).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < ny.wrapping_add(1 as libc::c_int as libc::c_ulong) {
        *((*p).yrange).offset(i as isize) = *((*h).yrange).offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    let mut mean: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut sum: libc::c_double = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        mean
            += (*((*h).bin).offset(i as isize) - mean)
                / i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double;
        i = i.wrapping_add(1);
        i;
    }
    *((*p).sum).offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_double;
    i = 0 as libc::c_int as size_t;
    while i < n {
        sum += *((*h).bin).offset(i as isize) / mean / n as libc::c_double;
        *((*p).sum)
            .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize) = sum;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_histogram2d_pdf_free(mut p: *mut gsl_histogram2d_pdf) {
    if p.is_null() {
        return;
    }
    free((*p).xrange as *mut libc::c_void);
    free((*p).yrange as *mut libc::c_void);
    free((*p).sum as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
