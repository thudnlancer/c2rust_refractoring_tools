use std::ptr;
use std::mem;
use std::ops::{Deref, DerefMut};
use libc::{c_int, c_double};
use gsl_sys::{gsl_interp_accel, gsl_interp_type, gsl_linalg_solve_symm_tridiag, gsl_linalg_solve_symm_cyc_tridiag, gsl_vector, gsl_error, GSL_ENOMEM, GSL_SUCCESS, GSL_EINVAL};

struct CsplineState {
    c: Vec<f64>,
    g: Vec<f64>,
    diag: Vec<f64>,
    offdiag: Vec<f64>,
}

impl CsplineState {
    fn new(size: usize) -> Result<Self, i32> {
        let c = vec![0.0; size];
        let g = vec![0.0; size];
        let diag = vec![0.0; size];
        let offdiag = vec![0.0; size];
        
        Ok(Self { c, g, diag, offdiag })
    }
}

fn cspline_alloc(size: usize) -> *mut CsplineState {
    match CsplineState::new(size) {
        Ok(state) => Box::into_raw(Box::new(state)),
        Err(_) => {
            unsafe { gsl_error("failed to allocate space for state", "", 0, GSL_ENOMEM); }
            ptr::null_mut()
        }
    }
}

fn cspline_init(vstate: *mut CsplineState, xa: &[f64], ya: &[f64], size: usize) -> i32 {
    let state = unsafe { &mut *vstate };
    let max_index = size - 1;
    let sys_size = max_index - 1;

    state.c[0] = 0.0;
    state.c[max_index] = 0.0;

    for i in 0..sys_size {
        let h_i = xa[i + 1] - xa[i];
        let h_ip1 = xa[i + 2] - xa[i + 1];
        let ydiff_i = ya[i + 1] - ya[i];
        let ydiff_ip1 = ya[i + 2] - ya[i + 1];
        let g_i = if h_i != 0.0 { 1.0 / h_i } else { 0.0 };
        let g_ip1 = if h_ip1 != 0.0 { 1.0 / h_ip1 } else { 0.0 };
        
        state.offdiag[i] = h_ip1;
        state.diag[i] = 2.0 * (h_ip1 + h_i);
        state.g[i] = 3.0 * (ydiff_ip1 * g_ip1 - ydiff_i * g_i);
    }

    if sys_size == 1 {
        state.c[1] = state.g[0] / state.diag[0];
        GSL_SUCCESS
    } else {
        let mut g_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.g.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut diag_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.diag.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut offdiag_vec = gsl_vector {
            size: sys_size - 1,
            stride: 1,
            data: state.offdiag.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut solution_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.c[1..].as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        unsafe {
            gsl_linalg_solve_symm_tridiag(
                &mut diag_vec,
                &mut offdiag_vec,
                &mut g_vec,
                &mut solution_vec,
            )
        }
    }
}

fn cspline_init_periodic(vstate: *mut CsplineState, xa: &[f64], ya: &[f64], size: usize) -> i32 {
    let state = unsafe { &mut *vstate };
    let max_index = size - 1;
    let sys_size = max_index;

    if sys_size == 2 {
        let h0 = xa[1] - xa[0];
        let h1 = xa[2] - xa[1];
        let a = 2.0 * (h0 + h1);
        let b = h0 + h1;
        
        let mut g = [0.0; 2];
        g[0] = 3.0 * ((ya[2] - ya[1]) / h1 - (ya[1] - ya[0]) / h0);
        g[1] = 3.0 * ((ya[1] - ya[2]) / h0 - (ya[2] - ya[1]) / h1);
        
        let det = 3.0 * (h0 + h1) * (h0 + h1);
        state.c[1] = (a * g[0] - b * g[1]) / det;
        state.c[2] = (-b * g[0] + a * g[1]) / det;
        state.c[0] = state.c[2];
        
        GSL_SUCCESS
    } else {
        for i in 0..sys_size - 1 {
            let h_i = xa[i + 1] - xa[i];
            let h_ip1 = xa[i + 2] - xa[i + 1];
            let ydiff_i = ya[i + 1] - ya[i];
            let ydiff_ip1 = ya[i + 2] - ya[i + 1];
            let g_i = if h_i != 0.0 { 1.0 / h_i } else { 0.0 };
            let g_ip1 = if h_ip1 != 0.0 { 1.0 / h_ip1 } else { 0.0 };
            
            state.offdiag[i] = h_ip1;
            state.diag[i] = 2.0 * (h_ip1 + h_i);
            state.g[i] = 3.0 * (ydiff_ip1 * g_ip1 - ydiff_i * g_i);
        }

        let i = sys_size - 1;
        let h_i = xa[i + 1] - xa[i];
        let h_ip1 = xa[1] - xa[0];
        let ydiff_i = ya[i + 1] - ya[i];
        let ydiff_ip1 = ya[1] - ya[0];
        let g_i = if h_i != 0.0 { 1.0 / h_i } else { 0.0 };
        let g_ip1 = if h_ip1 != 0.0 { 1.0 / h_ip1 } else { 0.0 };
        
        state.offdiag[i] = h_ip1;
        state.diag[i] = 2.0 * (h_ip1 + h_i);
        state.g[i] = 3.0 * (ydiff_ip1 * g_ip1 - ydiff_i * g_i);

        let mut g_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.g.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut diag_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.diag.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut offdiag_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.offdiag.as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let mut solution_vec = gsl_vector {
            size: sys_size,
            stride: 1,
            data: state.c[1..].as_mut_ptr(),
            block: ptr::null_mut(),
            owner: 0,
        };
        
        let status = unsafe {
            gsl_linalg_solve_symm_cyc_tridiag(
                &mut diag_vec,
                &mut offdiag_vec,
                &mut g_vec,
                &mut solution_vec,
            )
        };
        
        state.c[0] = state.c[max_index];
        status
    }
}

fn cspline_free(vstate: *mut CsplineState) {
    if !vstate.is_null() {
        unsafe { Box::from_raw(vstate); }
    }
}

fn coeff_calc(c_array: &[f64], dy: f64, dx: f64, index: usize, b: &mut f64, c: &mut f64, d: &mut f64) {
    let c_i = c_array[index];
    let c_ip1 = c_array[index + 1];
    *b = (dy / dx) - dx * (c_ip1 + 2.0 * c_i) / 3.0;
    *c = c_i;
    *d = (c_ip1 - c_i) / (3.0 * dx);
}

fn cspline_eval(vstate: *const CsplineState, x_array: &[f64], y_array: &[f64], size: usize,
                x: f64, a: *mut gsl_interp_accel, y: &mut f64) -> i32 {
    let state = unsafe { &*vstate };
    let index = if !a.is_null() {
        unsafe { gsl_interp_accel_find(a, x_array.as_ptr(), size as _, x) }
    } else {
        unsafe { gsl_interp_bsearch(x_array.as_ptr(), x, 0, (size - 1) as _) }
    } as usize;

    let x_hi = x_array[index + 1];
    let x_lo = x_array[index];
    let dx = x_hi - x_lo;
    
    if dx > 0.0 {
        let y_lo = y_array[index];
        let y_hi = y_array[index + 1];
        let dy = y_hi - y_lo;
        let delx = x - x_lo;
        
        let mut b_i = 0.0;
        let mut c_i = 0.0;
        let mut d_i = 0.0;
        coeff_calc(&state.c, dy, dx, index, &mut b_i, &mut c_i, &mut d_i);
        
        *y = y_lo + delx * (b_i + delx * (c_i + delx * d_i));
        GSL_SUCCESS
    } else {
        *y = 0.0;
        GSL_EINVAL
    }
}

// Similar implementations for cspline_eval_deriv, cspline_eval_deriv2, and cspline_eval_integ
// would follow the same pattern as cspline_eval...

static CSplineType: gsl_interp_type = gsl_interp_type {
    name: b"cspline\0".as_ptr() as *const _,
    min_size: 3,
    alloc: Some(cspline_alloc),
    init: Some(cspline_init),
    eval: Some(cspline_eval),
    eval_deriv: None,
    eval_deriv2: None,
    eval_integ: None,
    free: Some(cspline_free),
};

static CSplinePeriodicType: gsl_interp_type = gsl_interp_type {
    name: b"cspline-periodic\0".as_ptr() as *const _,
    min_size: 2,
    alloc: Some(cspline_alloc),
    init: Some(cspline_init_periodic),
    eval: Some(cspline_eval),
    eval_deriv: None,
    eval_deriv2: None,
    eval_integ: None,
    free: Some(cspline_free),
};

#[no_mangle]
pub static gsl_interp_cspline: *const gsl_interp_type = &CSplineType;

#[no_mangle]
pub static gsl_interp_cspline_periodic: *const gsl_interp_type = &CSplinePeriodicType;