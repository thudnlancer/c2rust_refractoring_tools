use libc::{c_double, c_int, c_ulong, c_void};
use std::ffi::CString;
use std::ptr;

type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: *mut c_double,
    block: *mut GslBlock,
    owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub struct GslBlock {
    size: size_t,
    data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct GslFunctionFdf {
    f: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_double>,
    df: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_double>,
    fdf: Option<unsafe extern "C" fn(c_double, *mut c_void, *mut c_double, *mut c_double)>,
    params: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
pub struct GslMultiminFunctionFdf {
    f: Option<unsafe extern "C" fn(*const GslVector, *mut c_void) -> c_double>,
    df: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslVector)>,
    fdf: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut c_double, *mut GslVector)>,
    n: size_t,
    params: *mut c_void,
}

#[derive(Debug, Clone, Copy)]
pub struct GslMultiminFdfminimizerType {
    name: *const libc::c_char,
    size: size_t,
    alloc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> c_int>,
    set: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut GslMultiminFunctionFdf,
            *const GslVector,
            *mut c_double,
            *mut GslVector,
            c_double,
            c_double,
        ) -> c_int,
    >,
    iterate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut GslMultiminFunctionFdf,
            *mut GslVector,
            *mut c_double,
            *mut GslVector,
            *mut GslVector,
        ) -> c_int,
    >,
    restart: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
pub struct Wrapper {
    fdf_linear: GslFunctionFdf,
    fdf: *mut GslMultiminFunctionFdf,
    x: *const GslVector,
    g: *const GslVector,
    p: *const GslVector,
    f_alpha: c_double,
    df_alpha: c_double,
    x_alpha: *mut GslVector,
    g_alpha: *mut GslVector,
    f_cache_key: c_double,
    df_cache_key: c_double,
    x_cache_key: c_double,
    g_cache_key: c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct VectorBfgs2State {
    iter: c_int,
    step: c_double,
    g0norm: c_double,
    pnorm: c_double,
    delta_f: c_double,
    fp0: c_double,
    x0: *mut GslVector,
    g0: *mut GslVector,
    p: *mut GslVector,
    dx0: *mut GslVector,
    dg0: *mut GslVector,
    x_alpha: *mut GslVector,
    g_alpha: *mut GslVector,
    wrap: Wrapper,
    rho: c_double,
    sigma: c_double,
    tau1: c_double,
    tau2: c_double,
    tau3: c_double,
    order: c_int,
}

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Edom = 1,
    Erange = 2,
    Edefault = 3,
    Einval = 4,
    Efailed = 5,
    Efactor = 6,
    Esanity = 7,
    Enomem = 8,
    Ebadfunc = 9,
    Erunaway = 10,
    Emaxiter = 11,
    Ezerodiv = 12,
    Ebadtol = 13,
    Etol = 14,
    Eundrflw = 15,
    Eovrflw = 16,
    Eloss = 17,
    Eround = 18,
    Ebadlen = 19,
    Enotsqr = 20,
    Esing = 21,
    Ediverge = 22,
    Eunsup = 23,
    Eunimpl = 24,
    Ecache = 25,
    Etable = 26,
    Enoprog = 27,
    Enoprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

fn gsl_finite(x: c_double) -> c_int {
    unsafe { libc::finite(x) }
}

fn gsl_error(reason: &str, file: &str, line: c_int, gsl_errno: GslError) {
    let reason = CString::new(reason).unwrap();
    let file = CString::new(file).unwrap();
    unsafe {
        libc::printf(
            b"GSL error: %s at %s:%d\n\0" as *const u8 as *const libc::c_char,
            reason.as_ptr(),
            file.as_ptr(),
            line,
        );
    }
}

fn gsl_vector_calloc(n: size_t) -> *mut GslVector {
    unsafe {
        let v = libc::malloc(std::mem::size_of::<GslVector>()) as *mut GslVector;
        if v.is_null() {
            return ptr::null_mut();
        }
        (*v).size = n;
        (*v).stride = 1;
        (*v).data = libc::malloc((n * std::mem::size_of::<c_double>()) as usize) as *mut c_double;
        if (*v).data.is_null() {
            libc::free(v as *mut c_void);
            return ptr::null_mut();
        }
        (*v).block = ptr::null_mut();
        (*v).owner = 1;
        v
    }
}

fn gsl_vector_free(v: *mut GslVector) {
    if !v.is_null() {
        unsafe {
            if (*v).owner != 0 && !(*v).data.is_null() {
                libc::free((*v).data as *mut c_void);
            }
            libc::free(v as *mut c_void);
        }
    }
}

fn gsl_vector_set_zero(v: *mut GslVector) {
    if !v.is_null() {
        unsafe {
            libc::memset(
                (*v).data as *mut c_void,
                0,
                (*v).size * std::mem::size_of::<c_double>(),
            );
        }
    }
}

fn gsl_vector_memcpy(dest: *mut GslVector, src: *const GslVector) -> c_int {
    if dest.is_null() || src.is_null() {
        return GslError::Edefault as c_int;
    }
    unsafe {
        if (*dest).size != (*src).size {
            return GslError::Ebadlen as c_int;
        }
        libc::memcpy(
            (*dest).data as *mut c_void,
            (*src).data as *const c_void,
            (*dest).size * std::mem::size_of::<c_double>(),
        );
    }
    GslError::Success as c_int
}

fn gsl_blas_ddot(x: *const GslVector, y: *const GslVector, result: *mut c_double) -> c_int {
    if x.is_null() || y.is_null() || result.is_null() {
        return GslError::Edefault as c_int;
    }
    unsafe {
        if (*x).size != (*y).size {
            return GslError::Ebadlen as c_int;
        }
        let mut sum = 0.0;
        for i in 0..(*x).size {
            sum += *(*x).data.add(i) * *(*y).data.add(i);
        }
        *result = sum;
    }
    GslError::Success as c_int
}

fn gsl_blas_dnrm2(x: *const GslVector) -> c_double {
    if x.is_null() {
        return 0.0;
    }
    unsafe {
        let mut sum = 0.0;
        for i in 0..(*x).size {
            sum += *(*x).data.add(i) * *(*x).data.add(i);
        }
        libc::sqrt(sum)
    }
}

fn gsl_blas_daxpy(alpha: c_double, x: *const GslVector, y: *mut GslVector) -> c_int {
    if x.is_null() || y.is_null() {
        return GslError::Edefault as c_int;
    }
    unsafe {
        if (*x).size != (*y).size {
            return GslError::Ebadlen as c_int;
        }
        for i in 0..(*x).size {
            *(*y).data.add(i) += alpha * *(*x).data.add(i);
        }
    }
    GslError::Success as c_int
}

fn gsl_blas_dscal(alpha: c_double, x: *mut GslVector) {
    if !x.is_null() {
        unsafe {
            for i in 0..(*x).size {
                *(*x).data.add(i) *= alpha;
            }
        }
    }
}

fn gsl_poly_solve_quadratic(
    a: c_double,
    b: c_double,
    c: c_double,
    x0: *mut c_double,
    x1: *mut c_double,
) -> c_int {
    if x0.is_null() || x1.is_null() {
        return 0;
    }
    unsafe {
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            0
        } else if disc == 0.0 {
            *x0 = -b / (2.0 * a);
            1
        } else {
            let sqrt_disc = libc::sqrt(disc);
            *x0 = (-b - sqrt_disc) / (2.0 * a);
            *x1 = (-b + sqrt_disc) / (2.0 * a);
            2
        }
    }
}

fn interp_quad(
    f0: c_double,
    fp0: c_double,
    f1: c_double,
    zl: c_double,
    zh: c_double,
) -> c_double {
    let fl = f0 + zl * (fp0 + zl * (f1 - f0 - fp0));
    let fh = f0 + zh * (fp0 + zh * (f1 - f0 - fp0));
    let c = 2.0 * (f1 - f0 - fp0);
    let mut zmin = zl;
    let mut fmin = fl;
    if fh < fmin {
        zmin = zh;
        fmin = fh;
    }
    if c > 0.0 {
        let z = -fp0 / c;
        if z > zl && z < zh {
            let f = f0 + z * (fp0 + z * (f1 - f0 - fp0));
            if f < fmin {
                zmin = z;
                fmin = f;
            }
        }
    }
    zmin
}

fn cubic(c0: c_double, c1: c_double, c2: c_double, c3: c_double, z: c_double) -> c_double {
    c0 + z * (c1 + z * (c2 + z * c3))
}

fn check_extremum(
    c0: c_double,
    c1: c_double,
    c2: c_double,
    c3: c_double,
    z: c_double,
    zmin: &mut c_double,
    fmin: &mut c_double,
) {
    let y = cubic(c0, c1, c2, c3, z);
    if y < *fmin {
        *zmin = z;
        *fmin = y;
    }
}

fn interp_cubic(
    f0: c_double,
    fp0: c_double,
    f1: c_double,
    fp1: c_double,
    zl: c_double,
    zh: c_double,
) -> c_double {
    let eta = 3.0 * (f1 - f0) - 2.0 * fp0 - fp1;
    let xi = fp0 + fp1 - 2.0 * (f1 - f0);
    let c0 = f0;
    let c1 = fp0;
    let c2 = eta;
    let c3 = xi;
    let mut zmin = zl;
    let mut fmin = cubic(c0, c1, c2, c3, zl);
    check_extremum(c0, c1, c2, c3, zh, &mut zmin, &mut fmin);
    let mut z0 = 0.0;
    let mut z1 = 0.0;
    let n = gsl_poly_solve_quadratic(3.0 * c3, 2.0 * c2, c1, &mut z0, &mut z1);
    if n == 2 {
        if z0 > zl && z0 < zh {
            check_extremum(c0, c1, c2, c3, z0, &mut zmin, &mut fmin);
        }
        if z1 > zl && z1 < zh {
            check_extremum(c0, c1, c2, c3, z1, &mut zmin, &mut fmin);
        }
    } else if n == 1 {
        if z0 > zl && z0 < zh {
            check_extremum(c0, c1, c2, c3, z0, &mut zmin, &mut fmin);
        }
    }
    zmin
}

fn interpolate(
    a: c_double,
    fa: c_double,
    fpa: c_double,
    b: c_double,
    fb: c_double,
    fpb: c_double,
    xmin: c_double,
    xmax: c_double,
    order: c_int,
) -> c_double {
    let mut zmin = (xmin - a) / (b - a);
    let mut zmax = (xmax - a) / (b - a);
    if zmin > zmax {
        std::mem::swap(&mut zmin, &mut zmax);
    }
    let z = if order > 2 && gsl_finite(fpb) != 0 {
        interp_cubic(fa, fpa * (b - a), fb, fpb * (b - a), zmin, zmax)
    } else {
        interp_quad(fa, fpa * (b - a), fb, zmin, zmax)
    };
    a + z * (b - a)
}

fn minimize(
    fn_: &mut GslFunctionFdf,
    rho: c_double,
    sigma: c_double,
    tau1: c_double,
    tau2: c_double,
    tau3: c_double,
    order: c_int,
    alpha1: c_double,
    alpha_new: &mut c_double,
) -> c_int {
    let mut f0 = 0.0;
    let mut fp0 = 0.0;
    let mut falpha = 0.0;
    let mut falpha_prev = 0.0;
    let mut fpalpha = 0.0;
    let mut fpalpha_prev = 0.0;
    let mut delta = 0.0;
    let mut alpha_next = 0.0;
    let mut alpha = alpha1;
    let mut alpha_prev = 0.0;
    let mut a = 0.0;
    let mut b = 0.0;
    let mut fa = 0.0;
    let mut fb = 0.0;
    let mut fpa = 0.0;
    let mut fpb = 0.0;
    let bracket_iters = 100;
    let section_iters = 100;
    let mut i = 0;

    unsafe {
        if let Some(fdf) = fn_.fdf {
            fdf(0.0, fn_.params, &mut f0, &mut fp0);
        }
    }

    falpha_prev = f0;
    fpalpha_prev = fp0;
    a = 0.0;
    b = alpha;
    fa = f0;
    fb = 0.0;
    fpa = fp0;
    fpb = 0.0;

    while i < bracket_iters {
        i += 1;
        unsafe {
            if let Some(f) = fn_.f {
                falpha = f(alpha, fn_.params);
            }
        }

        if falpha > f0 + alpha * rho * fp0 || falpha >= falpha_prev {
            a = alpha_prev;
            fa = falpha_prev;
            fpa = fpalpha_prev;
            b = alpha;
            fb = falpha;
            fpb = std::f64::NAN;
           