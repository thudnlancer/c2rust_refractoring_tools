use libc::{c_double, c_int, c_ulong, c_void};
use std::ffi::CStr;
use std::mem;
use std::ptr;

type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_vector_view {
    pub vector: gsl_vector,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Clone, Copy)]
pub enum gsl_multilarge_nlinear_fdtype {
    FwDiff = 0,
    CtDiff = 1,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_fdf {
    pub f: Option<unsafe extern "C" fn(*const gsl_vector, *mut c_void, *mut gsl_vector) -> c_int>,
    pub df: Option<
        unsafe extern "C" fn(
            CBLAS_TRANSPOSE,
            *const gsl_vector,
            *const gsl_vector,
            *mut c_void,
            *mut gsl_vector,
            *mut gsl_matrix,
        ) -> c_int,
    >,
    pub fvv: Option<
        unsafe extern "C" fn(*const gsl_vector, *const gsl_vector, *mut c_void, *mut gsl_vector) -> c_int,
    >,
    pub n: size_t,
    pub p: size_t,
    pub params: *mut c_void,
    pub nevalf: size_t,
    pub nevaldfu: size_t,
    pub nevaldf2: size_t,
    pub nevalfvv: size_t,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_trs {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(*const c_void, size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub preloop: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub step: Option<
        unsafe extern "C" fn(*const c_void, c_double, *mut gsl_vector, *mut c_void) -> c_int,
    >,
    pub preduction: Option<
        unsafe extern "C" fn(*const c_void, *const gsl_vector, *mut c_double, *mut c_void) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_scale {
    pub name: *const libc::c_char,
    pub init: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*const gsl_matrix, *mut gsl_vector) -> c_int>,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_solver {
    pub name: *const libc::c_char,
    pub alloc: Option<unsafe extern "C" fn(size_t, size_t) -> *mut c_void>,
    pub init: Option<unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int>,
    pub presolve: Option<unsafe extern "C" fn(c_double, *const c_void, *mut c_void) -> c_int>,
    pub solve: Option<
        unsafe extern "C" fn(*const gsl_vector, *mut gsl_vector, *const c_void, *mut c_void) -> c_int,
    >,
    pub rcond: Option<
        unsafe extern "C" fn(*mut c_double, *const gsl_matrix, *mut c_void) -> c_int,
    >,
    pub covar: Option<
        unsafe extern "C" fn(*const gsl_matrix, *mut gsl_matrix, *mut c_void) -> c_int,
    >,
    pub free: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_parameters {
    pub trs: *const gsl_multilarge_nlinear_trs,
    pub scale: *const gsl_multilarge_nlinear_scale,
    pub solver: *const gsl_multilarge_nlinear_solver,
    pub fdtype: gsl_multilarge_nlinear_fdtype,
    pub factor_up: c_double,
    pub factor_down: c_double,
    pub avmax: c_double,
    pub h_df: c_double,
    pub h_fvv: c_double,
    pub max_iter: size_t,
    pub tol: c_double,
}

#[derive(Debug, Clone, Copy)]
pub struct gsl_multilarge_nlinear_trust_state {
    pub x: *const gsl_vector,
    pub f: *const gsl_vector,
    pub g: *const gsl_vector,
    pub JTJ: *const gsl_matrix,
    pub diag: *const gsl_vector,
    pub sqrt_wts: *const gsl_vector,
    pub mu: *const c_double,
    pub params: *const gsl_multilarge_nlinear_parameters,
    pub solver_state: *mut c_void,
    pub fdf: *mut gsl_multilarge_nlinear_fdf,
    pub avratio: *mut c_double,
}

#[derive(Debug)]
struct CgstState {
    n: size_t,
    p: size_t,
    z: *mut gsl_vector,
    r: *mut gsl_vector,
    d: *mut gsl_vector,
    workp: *mut gsl_vector,
    workn: *mut gsl_vector,
    norm_g: c_double,
    cgtol: c_double,
    cgmaxit: size_t,
}

impl CgstState {
    unsafe fn new(params: *const c_void, n: size_t, p: size_t) -> Result<*mut c_void, i32> {
        let par = params as *const gsl_multilarge_nlinear_parameters;
        let state = Box::into_raw(Box::new(CgstState {
            n,
            p,
            z: ptr::null_mut(),
            r: ptr::null_mut(),
            d: ptr::null_mut(),
            workp: ptr::null_mut(),
            workn: ptr::null_mut(),
            norm_g: 0.0,
            cgtol: 0.0,
            cgmaxit: 0,
        }));

        unsafe {
            (*state).z = gsl_vector_alloc(p);
            if (*state).z.is_null() {
                return Err(GSL_ENOMEM);
            }

            (*state).r = gsl_vector_alloc(p);
            if (*state).r.is_null() {
                return Err(GSL_ENOMEM);
            }

            (*state).d = gsl_vector_alloc(p);
            if (*state).d.is_null() {
                return Err(GSL_ENOMEM);
            }

            (*state).workp = gsl_vector_alloc(p);
            if (*state).workp.is_null() {
                return Err(GSL_ENOMEM);
            }

            (*state).workn = gsl_vector_alloc(n);
            if (*state).workn.is_null() {
                return Err(GSL_ENOMEM);
            }

            (*state).cgmaxit = (*par).max_iter;
            if (*state).cgmaxit == 0 {
                (*state).cgmaxit = n;
            }
            (*state).cgtol = (*par).tol;
        }

        Ok(state as *mut c_void)
    }

    unsafe fn free(&mut self) {
        if !self.z.is_null() {
            gsl_vector_free(self.z);
        }
        if !self.r.is_null() {
            gsl_vector_free(self.r);
        }
        if !self.d.is_null() {
            gsl_vector_free(self.d);
        }
        if !self.workp.is_null() {
            gsl_vector_free(self.workp);
        }
        if !self.workn.is_null() {
            gsl_vector_free(self.workn);
        }
    }
}

const GSL_SUCCESS: c_int = 0;
const GSL_ENOMEM: c_int = 8;
const GSL_EMAXITER: c_int = 11;

extern "C" {
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> c_int;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_div(a: *mut gsl_vector, b: *const gsl_vector) -> c_int;
    fn gsl_vector_scale(a: *mut gsl_vector, x: c_double) -> c_int;
    fn gsl_blas_dnrm2(X: *const gsl_vector) -> c_double;
    fn gsl_blas_dsymv(
        Uplo: CBLAS_UPLO,
        alpha: c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: c_double,
        Y: *mut gsl_vector,
    ) -> c_int;
    fn gsl_blas_ddot(
        X: *const gsl_vector,
        Y: *const gsl_vector,
        result: *mut c_double,
    ) -> c_int;
    fn gsl_multilarge_nlinear_eval_df(
        TransJ: CBLAS_TRANSPOSE,
        x: *const gsl_vector,
        f: *const gsl_vector,
        u: *const gsl_vector,
        swts: *const gsl_vector,
        h: c_double,
        fdtype: gsl_multilarge_nlinear_fdtype,
        fdf: *mut gsl_multilarge_nlinear_fdf,
        v: *mut gsl_vector,
        JTJ: *mut gsl_matrix,
        work: *mut gsl_vector,
    ) -> c_int;
    static gsl_multilarge_nlinear_solver_cholesky: *const gsl_multilarge_nlinear_solver;
    static gsl_multilarge_nlinear_solver_mcholesky: *const gsl_multilarge_nlinear_solver;
}

fn scaled_addition(
    alpha: c_double,
    x: *const gsl_vector,
    beta: c_double,
    y: *const gsl_vector,
    z: *mut gsl_vector,
) {
    unsafe {
        let n = (*z).size;
        for i in 0..n {
            let xi = *((*x).data.add(i * (*x).stride));
            let yi = *((*y).data.add(i * (*y).stride));
            *((*z).data.add(i * (*z).stride)) = alpha * xi + beta * yi;
        }
    }
}

fn quadratic_preduction(
    trust_state: *const gsl_multilarge_nlinear_trust_state,
    dx: *const gsl_vector,
    work: *mut gsl_vector,
) -> c_double {
    unsafe {
        let f = (*trust_state).f;
        let params = (*trust_state).params;
        let normf = gsl_blas_dnrm2(f);
        let mut gTdx = 0.0;
        let fdf = (*trust_state).fdf;
        let mut pred_reduction = 0.0;
        let mut u = 0.0;

        gsl_blas_ddot((*trust_state).g, dx, &mut gTdx);
        pred_reduction = -2.0 * gTdx / (normf * normf);

        if (*params).solver == gsl_multilarge_nlinear_solver_cholesky
            || (*params).solver == gsl_multilarge_nlinear_solver_mcholesky
        {
            let p = (*fdf).p;
            let mut workp = gsl_vector_subvector(work, 0, p);
            gsl_blas_dsymv(
                CBLAS_UPLO::Lower,
                1.0,
                (*trust_state).JTJ,
                dx,
                0.0,
                &mut workp.vector,
            );
            gsl_blas_ddot(&mut workp.vector, dx, &mut u);
            pred_reduction -= u / (normf * normf);
        } else {
            let status = gsl_multilarge_nlinear_eval_df(
                CBLAS_TRANSPOSE::NoTrans,
                (*trust_state).x,
                f,
                dx,
                (*trust_state).sqrt_wts,
                (*params).h_df,
                (*params).fdtype,
                fdf,
                work,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            if status != GSL_SUCCESS {
                return 0.0;
            }
            u = gsl_blas_dnrm2(work) / normf;
            pred_reduction -= u * u;
        }

        pred_reduction
    }
}

fn cgst_calc_tau(p: *const gsl_vector, d: *const gsl_vector, delta: c_double) -> c_double {
    unsafe {
        let norm_p = gsl_blas_dnrm2(p);
        let norm_d = gsl_blas_dnrm2(d);
        let mut u = 0.0;
        gsl_blas_ddot(p, d, &mut u);
        let t1 = u / (norm_d * norm_d);
        let t2 = t1 * u + (delta + norm_p) * (delta - norm_p);
        -t1 + libm::sqrt(t2) / norm_d
    }
}

static CGST_TYPE: gsl_multilarge_nlinear_trs = gsl_multilarge_nlinear_trs {
    name: b"steihaug-toint\0".as_ptr() as *const libc::c_char,
    alloc: Some(CgstState::new),
    init: Some(|_, _| GSL_SUCCESS),
    preloop: Some(|_, _| GSL_SUCCESS),
    step: Some(cgst_step),
    preduction: Some(cgst_preduction),
    free: Some(|vstate| unsafe {
        let state = Box::from_raw(vstate as *mut CgstState);
        state.free();
    }),
};

#[no_mangle]
pub static gsl_multilarge_nlinear_trs_cgst: *const gsl_multilarge_nlinear_trs = &CGST_TYPE;

fn cgst_step(
    vtrust_state: *const c_void,
    delta: c_double,
    dx: *mut gsl_vector,
    vstate: *mut c_void,
) -> c_int {
    unsafe {
        let trust_state = vtrust_state as *const gsl_multilarge_nlinear_trust_state;
        let state = &mut *(vstate as *mut CgstState);
        let x = (*trust_state).x;
        let f = (*trust_state).f;
        let swts = (*trust_state).sqrt_wts;
        let diag = (*trust_state).diag;
        let params = (*trust_state).params;
        let fdf = (*trust_state).fdf;

        for i in 0..state.p {
            let gi = *((*(*trust_state).g).data.add(i * (*(*trust_state).g).stride));
            let di = *((*diag).data.add(i * (*diag).stride));
            *((*state.z).data.add(i * (*state.z).stride)) = 0.0;
            *((*state.r).data.add(i * (*state.r).stride)) = -gi / di;
            *((*state.d).data.add(i * (*state.d).stride)) = -gi / di;
            *((*state.workp).data.add(i * (*state.workp).stride)) = gi / di;
        }

        state.norm_g = gsl_blas_dnrm2(state.workp);

        for i in 0..state.cgmaxit {
            gsl_vector_memcpy(state.workp, state.d);
            gsl_vector_div(state.workp, (*trust_state).diag);
            let status = gsl_multilarge_nlinear_eval_df(
                CBLAS_TRANSPOSE::NoTrans,
                x,
                f,
                state.workp,
                swts,
                (*params).h_df,
                (*params).fdtype,
                fdf,
                state.workn,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            if status != GSL_SUCCESS {
                return status;
            }

            let norm_Jd = gsl_blas_dnrm2(state.workn);
            if norm_Jd == 0.0 {
                let tau = cgst_calc