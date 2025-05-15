use std::ffi::CString;
use std::os::raw::{c_double, c_int, c_void, c_char, c_ulong};

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[repr(C)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[repr(C)]
pub struct GslMatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

pub type GslMultiminFunction = extern "C" fn(*const GslVector, *mut c_void) -> c_double;

#[repr(C)]
pub struct GslMultiminFunctionStruct {
    pub f: Option<GslMultiminFunction>,
    pub n: size_t,
    pub params: *mut c_void,
}

#[repr(C)]
pub struct NmsimplexState {
    pub x1: *mut GslMatrix,
    pub y1: *mut GslVector,
    pub ws1: *mut GslVector,
    pub ws2: *mut GslVector,
}

#[repr(C)]
pub struct GslMultiminFminimizerType {
    pub name: *const c_char,
    pub size: size_t,
    pub alloc: Option<extern "C" fn(*mut c_void, size_t) -> c_int>,
    pub set: Option<extern "C" fn(*mut c_void, *mut GslMultiminFunctionStruct, *const GslVector, *mut c_double, *const GslVector) -> c_int>,
    pub iterate: Option<extern "C" fn(*mut c_void, *mut GslMultiminFunctionStruct, *mut GslVector, *mut c_double, *mut c_double) -> c_int>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    fn gsl_vector_alloc(n: size_t) -> *mut GslVector;
    fn gsl_vector_free(v: *mut GslVector);
    fn gsl_vector_memcpy(dest: *mut GslVector, src: *const GslVector) -> c_int;
    fn gsl_vector_min_index(v: *const GslVector) -> size_t;
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut GslMatrix;
    fn gsl_matrix_free(m: *mut GslMatrix);
    fn gsl_matrix_get_row(v: *mut GslVector, m: *const GslMatrix, i: size_t) -> c_int;
    fn gsl_matrix_set_row(m: *mut GslMatrix, i: size_t, v: *const GslVector) -> c_int;
    fn gsl_blas_dnrm2(X: *const GslVector) -> c_double;
    fn gsl_blas_daxpy(alpha: c_double, X: *const GslVector, Y: *mut GslVector) -> c_int;
    fn gsl_finite(x: c_double) -> c_int;
}

pub struct Simplex {
    state: Box<NmsimplexState>,
}

impl Simplex {
    pub fn new(n: size_t) -> Result<Self, GslError> {
        if n == 0 {
            unsafe {
                gsl_error(
                    CString::new("invalid number of parameters specified").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    196,
                    GslError::Invalid as c_int,
                );
            }
            return Err(GslError::Invalid);
        }

        let x1 = unsafe { gsl_matrix_alloc(n + 1, n) };
        if x1.is_null() {
            unsafe {
                gsl_error(
                    CString::new("failed to allocate space for x1").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    203,
                    GslError::NoMem as c_int,
                );
            }
            return Err(GslError::NoMem);
        }

        let y1 = unsafe { gsl_vector_alloc(n + 1) };
        if y1.is_null() {
            unsafe { gsl_matrix_free(x1) };
            unsafe {
                gsl_error(
                    CString::new("failed to allocate space for y").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    211,
                    GslError::NoMem as c_int,
                );
            }
            return Err(GslError::NoMem);
        }

        let ws1 = unsafe { gsl_vector_alloc(n) };
        if ws1.is_null() {
            unsafe {
                gsl_matrix_free(x1);
                gsl_vector_free(y1);
                gsl_error(
                    CString::new("failed to allocate space for ws1").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    220,
                    GslError::NoMem as c_int,
                );
            }
            return Err(GslError::NoMem);
        }

        let ws2 = unsafe { gsl_vector_alloc(n) };
        if ws2.is_null() {
            unsafe {
                gsl_matrix_free(x1);
                gsl_vector_free(y1);
                gsl_vector_free(ws1);
                gsl_error(
                    CString::new("failed to allocate space for ws2").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    230,
                    GslError::NoMem as c_int,
                );
            }
            return Err(GslError::NoMem);
        }

        Ok(Simplex {
            state: Box::new(NmsimplexState { x1, y1, ws1, ws2 }),
        })
    }

    pub fn set(
        &mut self,
        f: &mut GslMultiminFunctionStruct,
        x: &GslVector,
        size: &mut c_double,
        step_size: &GslVector,
    ) -> Result<(), GslError> {
        if unsafe { (*self.state.ws1).size != x.size } {
            unsafe {
                gsl_error(
                    CString::new("incompatible size of x").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    251,
                    GslError::Invalid as c_int,
                );
            }
            return Err(GslError::Invalid);
        }

        if unsafe { (*self.state.ws1).size != step_size.size } {
            unsafe {
                gsl_error(
                    CString::new("incompatible size of step_size").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    256,
                    GslError::Invalid as c_int,
                );
            }
            return Err(GslError::Invalid);
        }

        let val = unsafe { (f.f.unwrap())(x, f.params) };
        if unsafe { gsl_finite(val) == 0 } {
            unsafe {
                gsl_error(
                    CString::new("non-finite function value encountered").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    265,
                    GslError::BadFunc as c_int,
                );
            }
            return Err(GslError::BadFunc);
        }

        unsafe {
            gsl_matrix_set_row(self.state.x1, 0, x);
            gsl_vector_set(self.state.y1, 0, val);
        }

        for i in 0..x.size {
            let status = unsafe { gsl_vector_memcpy(self.state.ws1, x) };
            if status != GslError::Success as c_int {
                unsafe {
                    gsl_error(
                        CString::new("vector memcopy failed").unwrap().as_ptr(),
                        CString::new("simplex.c").unwrap().as_ptr(),
                        279,
                        GslError::Failed as c_int,
                    );
                }
                return Err(GslError::Failed);
            }

            let val = unsafe {
                let current = gsl_vector_get(self.state.ws1, i);
                let step = gsl_vector_get(step_size, i);
                current + step
            };
            unsafe { gsl_vector_set(self.state.ws1, i, val) };

            let val = unsafe { (f.f.unwrap())(self.state.ws1, f.params) };
            if unsafe { gsl_finite(val) == 0 } {
                unsafe {
                    gsl_error(
                        CString::new("non-finite function value encountered").unwrap().as_ptr(),
                        CString::new("simplex.c").unwrap().as_ptr(),
                        288,
                        GslError::BadFunc as c_int,
                    );
                }
                return Err(GslError::BadFunc);
            }

            unsafe {
                gsl_matrix_set_row(self.state.x1, i + 1, self.state.ws1);
                gsl_vector_set(self.state.y1, i + 1, val);
            }
        }

        *size = self.size();
        Ok(())
    }

    fn move_corner(
        &self,
        coeff: c_double,
        corner: size_t,
        xc: &mut GslVector,
        f: &GslMultiminFunctionStruct,
    ) -> c_double {
        let x1 = self.state.x1;
        let n = unsafe { (*x1).size2 };

        for j in 0..n {
            let mut mp = 0.0;
            for i in 0..unsafe { (*x1).size1 } {
                if i != corner {
                    mp += unsafe { gsl_matrix_get(x1, i, j) };
                }
            }
            mp /= (unsafe { (*x1).size1 } - 1) as c_double;
            let newval = mp - coeff * (mp - unsafe { gsl_matrix_get(x1, corner, j) });
            unsafe { gsl_vector_set(xc, j, newval) };
        }

        unsafe { (f.f.unwrap())(xc, f.params) }
    }

    fn contract_by_best(
        &mut self,
        best: size_t,
        xc: &mut GslVector,
        f: &mut GslMultiminFunctionStruct,
    ) -> Result<(), GslError> {
        let x1 = self.state.x1;
        let y1 = self.state.y1;
        let mut status = GslError::Success;

        for i in 0..unsafe { (*x1).size1 } {
            if i != best {
                for j in 0..unsafe { (*x1).size2 } {
                    let newval = 0.5 * (unsafe { gsl_matrix_get(x1, i, j) } + unsafe {
                        gsl_matrix_get(x1, best, j)
                    });
                    unsafe { gsl_matrix_set(x1, i, j, newval) };
                }
                unsafe { gsl_matrix_get_row(xc, x1, i) };
                let newval = unsafe { (f.f.unwrap())(xc, f.params) };
                unsafe { gsl_vector_set(y1, i, newval) };
                if unsafe { gsl_finite(newval) == 0 } {
                    status = GslError::BadFunc;
                }
            }
        }

        if status == GslError::Success {
            Ok(())
        } else {
            Err(status)
        }
    }

    fn calc_center(&self, mp: &mut GslVector) -> Result<(), GslError> {
        let x1 = self.state.x1;
        let n = unsafe { (*x1).size2 };

        for j in 0..n {
            let mut val = 0.0;
            for i in 0..unsafe { (*x1).size1 } {
                val += unsafe { gsl_matrix_get(x1, i, j) };
            }
            val /= unsafe { (*x1).size1 } as c_double;
            unsafe { gsl_vector_set(mp, j, val) };
        }

        Ok(())
    }

    fn size(&self) -> c_double {
        let s = self.state.ws1;
        let mp = self.state.ws2;
        let x1 = self.state.x1;
        let mut ss = 0.0;

        self.calc_center(mp).unwrap();

        for i in 0..unsafe { (*x1).size1 } {
            unsafe {
                gsl_matrix_get_row(s, x1, i);
                gsl_blas_daxpy(-1.0, mp, s);
                ss += gsl_blas_dnrm2(s);
            }
        }

        ss / unsafe { (*x1).size1 } as c_double
    }

    pub fn iterate(
        &mut self,
        f: &mut GslMultiminFunctionStruct,
        x: &mut GslVector,
        size: &mut c_double,
        fval: &mut c_double,
    ) -> Result<(), GslError> {
        if unsafe { (*self.state.ws1).size != x.size } {
            unsafe {
                gsl_error(
                    CString::new("incompatible size of x").unwrap().as_ptr(),
                    CString::new("simplex.c").unwrap().as_ptr(),
                    340,
                    GslError::Invalid as c_int,
                );
            }
            return Err(GslError::Invalid);
        }

        let y1 = self.state.y1;
        let n = unsafe { (*y1).size };

        let mut dlo = unsafe { gsl_vector_get(y1, 0) };
        let mut dhi = dlo;
        let mut hi = 0;
        let mut lo = 0;
        let mut ds_hi = unsafe { gsl_vector_get(y1, 1) };
        let mut s_hi = 1;

        for i in 1..n {
            let val = unsafe { gsl_vector_get(y1, i) };
            if val < dlo {
                dlo = val;
                lo = i;
            } else if val > dhi {
                ds_hi = dhi;
                s_hi = hi;
                dhi = val;
                hi = i;
            } else if val > ds_hi {
                ds_hi = val;
                s_hi = i;
            }
        }

        let mut xc = unsafe { &mut *self.state.ws1 };
        let mut xc2 = unsafe { &mut *self.state.ws2 };

        let val = self.move_corner(-1.0, hi, xc, f);
        if unsafe { gsl_finite(val) != 0 } && val < unsafe { gsl_vector_get(y1, lo) } {
            let val2 = self.move_corner(-2.0, hi, xc2, f);
            if unsafe { gsl_finite(val2) != 0 } && val2 < unsafe { gsl_vector_get(y1, lo) } {
                unsafe {
                    gsl_matrix_set_row(self.state.x1, hi, xc2);
                    gsl_vector_set(y1, hi, val2);
                }
            } else {
                unsafe {
                    gsl_matrix_set_row(self.state.x1, hi, xc);
                    gsl_vector_set(y1, hi, val);
                }
            }
        } else if unsafe { gsl_finite(val) == 0 } || val > unsafe { gsl_vector_get(y1, s_hi) } {
            if unsafe { gsl_finite(val) != 0 } && val <= unsafe { gsl_vector_get(y1, hi) } {
                unsafe {
                    gsl_matrix_set_row(self.state.x1, hi, xc);
                    gsl_vector_set(y1, hi, val);
                }
            }
            let val2 = self.move_corner(0.5, hi, xc2, f);
            if unsafe { gsl_finite(val2) != 0 } && val2 <= unsafe { gsl_vector_get(y1, hi) } {
                unsafe {
                    gsl_matrix_set_row(self.state.x1, hi, xc2);
                    gsl_vector_set(y1, hi, val2);
                }
            } else {
                self.contract_by_best(lo, xc, f)?;
            }
        } else {
            unsafe {
                gsl_matrix_set_row(self.state.x1, hi, xc);
                gsl_vector_set(y1, hi, val);
            }
        }

        lo = unsafe { gsl_vector_min_index(y1) };
        unsafe {
            gsl_matrix_get_row(x, self.state.x1, lo);
            *fval = gsl_vector_get(y1, lo);
        }
        *size = self.size();
        Ok(())
    }
}

impl Drop for Simplex {
    fn drop(&mut self) {
        unsafe {
            gsl_matrix_free(self.state.x1);
            gsl_vector_free(self.state.y1);
            gsl_vector_free(self.state.ws1);
            gsl_vector_free