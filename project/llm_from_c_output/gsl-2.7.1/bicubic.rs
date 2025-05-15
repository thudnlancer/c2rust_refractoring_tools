use std::ptr;
use std::mem;
use std::ops::{Index, IndexMut};
use libc::{c_void, size_t};
use gsl_sys::{gsl_interp_accel, gsl_spline, gsl_vector, gsl_interp2d_type, gsl_interp_accel_alloc, gsl_interp_accel_free, gsl_interp_accel_reset, gsl_interp_accel_find, gsl_interp_bsearch, gsl_spline_alloc, gsl_spline_free, gsl_spline_init, gsl_spline_eval_deriv, gsl_vector_alloc, gsl_vector_free, gsl_vector_set, GSL_SUCCESS, GSL_ENOMEM};

#[derive(Debug)]
struct BicubicState {
    zx: Vec<f64>,
    zy: Vec<f64>,
    zxy: Vec<f64>,
    xsize: usize,
    ysize: usize,
}

impl BicubicState {
    fn new(xsize: usize, ysize: usize) -> Option<Self> {
        let total_size = xsize * ysize;
        Some(BicubicState {
            zx: vec![0.0; total_size],
            zy: vec![0.0; total_size],
            zxy: vec![0.0; total_size],
            xsize,
            ysize,
        })
    }

    fn idx2d(&self, i: usize, j: usize) -> usize {
        j * self.xsize + i
    }
}

fn bicubic_alloc(xsize: usize, ysize: usize) -> Option<Box<BicubicState>> {
    BicubicState::new(xsize, ysize).map(Box::new)
}

fn bicubic_free(state: Option<Box<BicubicState>>) {
    // Rust's drop mechanism will automatically free the memory
}

fn bicubic_init(
    state: &mut BicubicState,
    xa: &[f64],
    ya: &[f64],
    za: &[f64],
    xsize: usize,
    ysize: usize,
) -> i32 {
    unsafe {
        let acc = gsl_interp_accel_alloc();
        if acc.is_null() {
            return GSL_ENOMEM;
        }

        let mut result = GSL_SUCCESS;

        // First pass: compute zx
        let x = gsl_vector_alloc(xsize);
        let y = gsl_vector_alloc(xsize);
        let spline = gsl_spline_alloc(gsl_sys::gsl_interp_cspline, xsize);

        if x.is_null() || y.is_null() || spline.is_null() {
            result = GSL_ENOMEM;
            goto cleanup;
        }

        for j in 0..ysize {
            for i in 0..xsize {
                gsl_vector_set(x, i, xa[i]);
                gsl_vector_set(y, i, za[state.idx2d(i, j)]);
            }
            gsl_spline_init(spline, (*x).data, (*y).data, xsize);
            for i in 0..xsize {
                state.zx[state.idx2d(i, j)] = gsl_spline_eval_deriv(spline, xa[i], acc);
            }
        }

    cleanup:
        if !x.is_null() {
            gsl_vector_free(x);
        }
        if !y.is_null() {
            gsl_vector_free(y);
        }
        if !spline.is_null() {
            gsl_spline_free(spline);
        }
        gsl_interp_accel_reset(acc);

        if result != GSL_SUCCESS {
            gsl_interp_accel_free(acc);
            return result;
        }

        // Second pass: compute zy
        let x = gsl_vector_alloc(ysize);
        let y = gsl_vector_alloc(ysize);
        let spline = gsl_spline_alloc(gsl_sys::gsl_interp_cspline, ysize);

        if x.is_null() || y.is_null() || spline.is_null() {
            result = GSL_ENOMEM;
            goto cleanup2;
        }

        for i in 0..xsize {
            for j in 0..ysize {
                gsl_vector_set(x, j, ya[j]);
                gsl_vector_set(y, j, za[state.idx2d(i, j)]);
            }
            gsl_spline_init(spline, (*x).data, (*y).data, ysize);
            for j in 0..ysize {
                state.zy[state.idx2d(i, j)] = gsl_spline_eval_deriv(spline, ya[j], acc);
            }
        }

    cleanup2:
        if !x.is_null() {
            gsl_vector_free(x);
        }
        if !y.is_null() {
            gsl_vector_free(y);
        }
        if !spline.is_null() {
            gsl_spline_free(spline);
        }
        gsl_interp_accel_reset(acc);

        if result != GSL_SUCCESS {
            gsl_interp_accel_free(acc);
            return result;
        }

        // Third pass: compute zxy
        let x = gsl_vector_alloc(xsize);
        let y = gsl_vector_alloc(xsize);
        let spline = gsl_spline_alloc(gsl_sys::gsl_interp_cspline, xsize);

        if x.is_null() || y.is_null() || spline.is_null() {
            result = GSL_ENOMEM;
            goto cleanup3;
        }

        for j in 0..ysize {
            for i in 0..xsize {
                gsl_vector_set(x, i, xa[i]);
                gsl_vector_set(y, i, state.zy[state.idx2d(i, j)]);
            }
            gsl_spline_init(spline, (*x).data, (*y).data, xsize);
            for i in 0..xsize {
                state.zxy[state.idx2d(i, j)] = gsl_spline_eval_deriv(spline, xa[i], acc);
            }
        }

    cleanup3:
        if !x.is_null() {
            gsl_vector_free(x);
        }
        if !y.is_null() {
            gsl_vector_free(y);
        }
        if !spline.is_null() {
            gsl_spline_free(spline);
        }
        gsl_interp_accel_free(acc);

        result
    }
}

// Other functions (bicubic_eval, bicubic_deriv_x, etc.) would follow similar patterns
// but are omitted for brevity. They would use the same safe Rust patterns with
// proper error handling and memory management.

static BICUBIC_TYPE: gsl_interp2d_type = gsl_interp2d_type {
    name: "bicubic\0".as_ptr() as *const i8,
    min_size: 4,
    alloc: Some(bicubic_alloc_wrapper),
    init: Some(bicubic_init_wrapper),
    eval: Some(bicubic_eval_wrapper),
    eval_deriv_x: Some(bicubic_deriv_x_wrapper),
    eval_deriv_y: Some(bicubic_deriv_y_wrapper),
    eval_deriv_xx: Some(bicubic_deriv_xx_wrapper),
    eval_deriv_xy: Some(bicubic_deriv_xy_wrapper),
    eval_deriv_yy: Some(bicubic_deriv_yy_wrapper),
    free: Some(bicubic_free_wrapper),
};

// Wrapper functions to bridge between C and Rust calling conventions
extern "C" fn bicubic_alloc_wrapper(xsize: size_t, ysize: size_t) -> *mut c_void {
    match bicubic_alloc(xsize, ysize) {
        Some(b) => Box::into_raw(b) as *mut c_void,
        None => ptr::null_mut(),
    }
}

extern "C" fn bicubic_free_wrapper(vstate: *mut c_void) {
    if !vstate.is_null() {
        unsafe { Box::from_raw(vstate as *mut BicubicState) };
    }
}

// Other wrapper functions would follow similar patterns...

#[no_mangle]
pub extern "C" fn gsl_interp2d_bicubic() -> *const gsl_interp2d_type {
    &BICUBIC_TYPE
}