use libc::{c_double, c_int, c_ulong, c_void, c_char};
use std::ptr;
use std::ffi::CString;

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

pub type GslMultirootFunction = GslMultirootFunctionStruct;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GslMultirootFunctionStruct {
    pub f: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslVector) -> c_int>,
    pub n: size_t,
    pub params: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct GslMultirootFsolverType {
    pub name: *const c_char,
    pub size: size_t,
    pub alloc: Option<unsafe extern "C" fn(*mut c_void, size_t) -> c_int>,
    pub set: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut GslMultirootFunction,
            *mut GslVector,
            *mut GslVector,
            *mut GslVector,
        ) -> c_int,
    >,
    pub iterate: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *mut GslMultirootFunction,
            *mut GslVector,
            *mut GslVector,
            *mut GslVector,
        ) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HybridState {
    pub iter: size_t,
    pub ncfail: size_t,
    pub ncsuc: size_t,
    pub nslow1: size_t,
    pub nslow2: size_t,
    pub fnorm: c_double,
    pub delta: c_double,
    pub j: *mut GslMatrix,
    pub q: *mut GslMatrix,
    pub r: *mut GslMatrix,
    pub tau: *mut GslVector,
    pub diag: *mut GslVector,
    pub qtf: *mut GslVector,
    pub newton: *mut GslVector,
    pub gradient: *mut GslVector,
    pub x_trial: *mut GslVector,
    pub f_trial: *mut GslVector,
    pub df: *mut GslVector,
    pub qtdf: *mut GslVector,
    pub rdx: *mut GslVector,
    pub w: *mut GslVector,
    pub v: *mut GslVector,
}

extern "C" {
    fn sqrt(x: c_double) -> c_double;
    fn fabs(x: c_double) -> c_double;
    fn gsl_error(
        reason: *const c_char,
        file: *const c_char,
        line: c_int,
        gsl_errno: c_int,
    );
    fn gsl_vector_calloc(n: size_t) -> *mut GslVector;
    fn gsl_vector_free(v: *mut GslVector);
    fn gsl_vector_set_all(v: *mut GslVector, x: c_double);
    fn gsl_vector_memcpy(dest: *mut GslVector, src: *const GslVector) -> c_int;
    fn gsl_matrix_calloc(n1: size_t, n2: size_t) -> *mut GslMatrix;
    fn gsl_matrix_free(m: *mut GslMatrix);
    fn gsl_multiroot_fdjacobian(
        F: *mut GslMultirootFunction,
        x: *const GslVector,
        f: *const GslVector,
        epsrel: c_double,
        jacobian: *mut GslMatrix,
    ) -> c_int;
    fn gsl_linalg_QR_decomp(A: *mut GslMatrix, tau: *mut GslVector) -> c_int;
    fn gsl_linalg_QR_update(
        Q: *mut GslMatrix,
        R: *mut GslMatrix,
        w: *mut GslVector,
        v: *const GslVector,
    ) -> c_int;
    fn gsl_linalg_QR_unpack(
        QR: *const GslMatrix,
        tau: *const GslVector,
        Q: *mut GslMatrix,
        R: *mut GslMatrix,
    ) -> c_int;
    fn gsl_linalg_R_solve(
        R: *const GslMatrix,
        b: *const GslVector,
        x: *mut GslVector,
    ) -> c_int;
}

unsafe fn gsl_vector_get(v: *const GslVector, i: size_t) -> c_double {
    *(*v).data.offset((i * (*v).stride) as isize)
}

unsafe fn gsl_vector_set(v: *mut GslVector, i: size_t, x: c_double) {
    *(*v).data.offset((i * (*v).stride) as isize) = x;
}

unsafe fn gsl_matrix_get(m: *const GslMatrix, i: size_t, j: size_t) -> c_double {
    *(*m).data.offset((i * (*m).tda + j) as isize)
}

unsafe fn scaled_enorm(d: *const GslVector, f: *const GslVector) -> c_double {
    let mut e2 = 0.0;
    let n = (*f).size;
    for i in 0..n {
        let fi = gsl_vector_get(f, i);
        let di = gsl_vector_get(d, i);
        let u = di * fi;
        e2 += u * u;
    }
    sqrt(e2)
}

unsafe fn enorm_sum(a: *const GslVector, b: *const GslVector) -> c_double {
    let mut e2 = 0.0;
    let n = (*a).size;
    for i in 0..n {
        let ai = gsl_vector_get(a, i);
        let bi = gsl_vector_get(b, i);
        let u = ai + bi;
        e2 += u * u;
    }
    sqrt(e2)
}

unsafe fn compute_wv(
    qtdf: *const GslVector,
    rdx: *const GslVector,
    dx: *const GslVector,
    diag: *const GslVector,
    pnorm: c_double,
    w: *mut GslVector,
    v: *mut GslVector,
) {
    let n = (*qtdf).size;
    for i in 0..n {
        let qtdfi = gsl_vector_get(qtdf, i);
        let rdxi = gsl_vector_get(rdx, i);
        let dxi = gsl_vector_get(dx, i);
        let diagi = gsl_vector_get(diag, i);
        gsl_vector_set(w, i, (qtdfi - rdxi) / pnorm);
        gsl_vector_set(v, i, diagi * diagi * dxi / pnorm);
    }
}

unsafe fn compute_df(
    f_trial: *const GslVector,
    f: *const GslVector,
    df: *mut GslVector,
) {
    let n = (*f).size;
    for i in 0..n {
        let dfi = gsl_vector_get(f_trial, i) - gsl_vector_get(f, i);
        gsl_vector_set(df, i, dfi);
    }
}

unsafe fn compute_diag(J: *const GslMatrix, diag: *mut GslVector) {
    let n = (*diag).size;
    for j in 0..n {
        let mut sum = 0.0;
        for i in 0..n {
            let Jij = gsl_matrix_get(J, i, j);
            sum += Jij * Jij;
        }
        if sum == 0.0 {
            sum = 1.0;
        }
        gsl_vector_set(diag, j, sqrt(sum));
    }
}

unsafe fn update_diag(J: *const GslMatrix, diag: *mut GslVector) {
    let n = (*diag).size;
    for j in 0..n {
        let mut sum = 0.0;
        for i in 0..n {
            let Jij = gsl_matrix_get(J, i, j);
            sum += Jij * Jij;
        }
        if sum == 0.0 {
            sum = 1.0;
        }
        let cnorm = sqrt(sum);
        let diagj = gsl_vector_get(diag, j);
        if cnorm > diagj {
            gsl_vector_set(diag, j, cnorm);
        }
    }
}

unsafe fn compute_delta(diag: *mut GslVector, x: *mut GslVector) -> c_double {
    let Dx = scaled_enorm(diag, x);
    let factor = 100.0;
    if Dx > 0.0 { factor * Dx } else { factor }
}

unsafe fn compute_actual_reduction(fnorm: c_double, fnorm1: c_double) -> c_double {
    if fnorm1 < fnorm {
        let u = fnorm1 / fnorm;
        1.0 - u * u
    } else {
        -1.0
    }
}

unsafe fn compute_predicted_reduction(fnorm: c_double, fnorm1: c_double) -> c_double {
    if fnorm1 < fnorm {
        let u = fnorm1 / fnorm;
        1.0 - u * u
    } else {
        0.0
    }
}

unsafe fn compute_qtf(q: *const GslMatrix, f: *const GslVector, qtf: *mut GslVector) {
    let N = (*f).size;
    for j in 0..N {
        let mut sum = 0.0;
        for i in 0..N {
            sum += gsl_matrix_get(q, i, j) * gsl_vector_get(f, i);
        }
        gsl_vector_set(qtf, j, sum);
    }
}

unsafe fn compute_rdx(r: *const GslMatrix, dx: *const GslVector, rdx: *mut GslVector) {
    let N = (*dx).size;
    for i in 0..N {
        let mut sum = 0.0;
        for j in i..N {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(dx, j);
        }
        gsl_vector_set(rdx, i, sum);
    }
}

unsafe fn compute_trial_step(
    x: *mut GslVector,
    dx: *mut GslVector,
    x_trial: *mut GslVector,
) {
    let N = (*x).size;
    for i in 0..N {
        let pi = gsl_vector_get(dx, i);
        let xi = gsl_vector_get(x, i);
        gsl_vector_set(x_trial, i, xi + pi);
    }
}

unsafe fn newton_direction(
    r: *const GslMatrix,
    qtf: *const GslVector,
    p: *mut GslVector,
) -> c_int {
    let N = (*r).size2;
    let status = gsl_linalg_R_solve(r, qtf, p);
    for i in 0..N {
        let pi = gsl_vector_get(p, i);
        gsl_vector_set(p, i, -pi);
    }
    status
}

unsafe fn gradient_direction(
    r: *const GslMatrix,
    qtf: *const GslVector,
    diag: *const GslVector,
    g: *mut GslVector,
) {
    let M = (*r).size1;
    let N = (*r).size2;
    for j in 0..M {
        let mut sum = 0.0;
        for i in 0..N {
            sum += gsl_matrix_get(r, i, j) * gsl_vector_get(qtf, i);
        }
        let dj = gsl_vector_get(diag, j);
        gsl_vector_set(g, j, -sum / dj);
    }
}

unsafe fn compute_Rg(
    r: *const GslMatrix,
    gradient: *const GslVector,
    Rg: *mut GslVector,
) {
    let N = (*r).size2;
    for i in 0..N {
        let mut sum = 0.0;
        for j in i..N {
            let gj = gsl_vector_get(gradient, j);
            let rij = gsl_matrix_get(r, i, j);
            sum += rij * gj;
        }
        gsl_vector_set(Rg, i, sum);
    }
}

unsafe fn enorm(f: *const GslVector) -> c_double {
    let mut e2 = 0.0;
    let n = (*f).size;
    for i in 0..n {
        let fi = gsl_vector_get(f, i);
        e2 += fi * fi;
    }
    sqrt(e2)
}

unsafe fn dogleg(
    r: *const GslMatrix,
    qtf: *const GslVector,
    diag: *const GslVector,
    delta: c_double,
    newton: *mut GslVector,
    gradient: *mut GslVector,
    p: *mut GslVector,
) -> c_int {
    let qnorm = scaled_enorm(diag, newton);
    if qnorm <= delta {
        gsl_vector_memcpy(p, newton);
        return GslError::Success as c_int;
    }

    gradient_direction(r, qtf, diag, gradient);
    let gnorm = enorm(gradient);
    if gnorm == 0.0 {
        let alpha = delta / qnorm;
        let beta = 0.0;
        scaled_addition(alpha, newton, beta, gradient, p);
        return GslError::Success as c_int;
    }

    minimum_step(gnorm, diag, gradient);
    compute_Rg(r, gradient, p);
    let temp = enorm(p);
    let sgnorm = gnorm / temp / temp;

    if sgnorm > delta {
        let alpha_0 = 0.0;
        let beta_0 = delta;
        scaled_addition(alpha_0, newton, beta_0, gradient, p);
        return GslError::Success as c_int;
    }

    let bnorm = enorm(qtf);
    let bg = bnorm / gnorm;
    let bq = bnorm / qnorm;
    let dq = delta / qnorm;
    let dq2 = dq * dq;
    let sd = sgnorm / delta;
    let sd2 = sd * sd;
    let t1 = bg * bq * sd;
    let u = t1 - dq;
    let t2 = t1 - dq * sd2 + sqrt(u * u + (1.0 - dq2) * (1.0 - sd2));
    let alpha_1 = dq * (1.0 - sd2) / t2;
    let beta_1 = (1.0 - alpha_1) * sgnorm;

    scaled_addition(alpha_1, newton, beta_1, gradient, p);
    GslError::Success as c_int
}

unsafe fn minimum_step(
    gnorm: c_double,
    diag: *const GslVector,
    g: *mut GslVector,
) {
    let N = (*g).size;
    for i in 0..N {
        let gi = gsl_vector_get(g, i);
        let di = gsl_vector_get(diag, i);
        gsl_vector_set(g, i, gi / gnorm / di);
    }
}

unsafe fn scaled_addition(
    alpha: c_double,
    newton: *mut GslVector,
    beta: c_double,
    gradient: *mut GslVector,
    p: *mut GslVector,
) {
    let N = (*p).size;
    for i in 0..N {
        let ni = gsl_vector_get(newton, i);
        let gi = gsl_vector_get(gradient, i);
        gsl_vector_set(p, i, alpha * ni + beta * gi);
    }
}

