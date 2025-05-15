use std::os::raw::{c_double, c_int, c_void};
use std::ptr;

#[repr(C)]
pub struct GslBlock {
    pub size: usize,
    pub data: *mut c_double,
}

#[repr(C)]
pub struct GslVector {
    pub size: usize,
    pub stride: usize,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[repr(C)]
pub struct GslVectorConstView {
    pub vector: GslVector,
}

#[repr(C)]
pub struct GslMatrix {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[repr(C)]
pub struct GslMultifitNlinearFdf {
    pub f: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslVector) -> c_int>,
    pub df: Option<unsafe extern "C" fn(*const GslVector, *mut c_void, *mut GslMatrix) -> c_int>,
    pub fvv: Option<unsafe extern "C" fn(*const GslVector, *const GslVector, *mut c_void, *mut GslVector) -> c_int>,
    pub n: usize,
    pub p: usize,
    pub params: *mut c_void,
    pub nevalf: usize,
    pub nevaldf: usize,
    pub nevalfvv: usize,
}

pub fn gsl_multifit_nlinear_fdfvv(
    h: c_double,
    x: &GslVector,
    v: &GslVector,
    f: &GslVector,
    J: &GslMatrix,
    swts: &GslVector,
    fdf: &mut GslMultifitNlinearFdf,
    fvv: &mut GslVector,
    work: &mut GslVector,
) -> c_int {
    let n = fdf.n;
    let p = fdf.p;
    let hinv = 1.0 / h;
    let mut status = 0;

    unsafe {
        for i in 0..p {
            let xi = *x.data.add(i * x.stride);
            let vi = *v.data.add(i * v.stride);
            *work.data.add(i * work.stride) = xi + h * vi;
        }

        status = gsl_multifit_nlinear_eval_f(fdf, work, swts, fvv);
        if status != 0 {
            return status;
        }

        for i in 0..n {
            let fi = *f.data.add(i * f.stride);
            let fip = *fvv.data.add(i * fvv.stride);
            let row = gsl_matrix_const_row(J, i);
            let mut u = 0.0;
            gsl_blas_ddot(&row.vector, v, &mut u);
            let fvvi = 2.0 * hinv * ((fip - fi) * hinv - u);
            *fvv.data.add(i * fvv.stride) = fvvi;
        }
    }

    status
}

unsafe fn gsl_matrix_const_row(m: *const GslMatrix, i: usize) -> GslVectorConstView {
    unimplemented!("Safe wrapper for gsl_matrix_const_row needed")
}

unsafe fn gsl_blas_ddot(X: *const GslVector, Y: *const GslVector, result: *mut c_double) -> c_int {
    unimplemented!("Safe wrapper for gsl_blas_ddot needed")
}

unsafe fn gsl_multifit_nlinear_eval_f(
    fdf: *mut GslMultifitNlinearFdf,
    x: *const GslVector,
    swts: *const GslVector,
    y: *mut GslVector,
) -> c_int {
    unimplemented!("Safe wrapper for gsl_multifit_nlinear_eval_f needed")
}