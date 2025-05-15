use std::ptr::null_mut;
use std::mem;
use std::os::raw::c_int;
use libc::{free, calloc};
use gsl_sys::{gsl_eigen_francis_workspace, gsl_vector, gsl_matrix, gsl_vector_complex, gsl_eigen_nonsymm_workspace, gsl_linalg_balance_matrix, gsl_linalg_hessenberg_decomp, gsl_linalg_hessenberg_unpack, gsl_eigen_francis_Z, gsl_eigen_francis, gsl_linalg_balance_accum, gsl_vector_alloc, gsl_vector_free, gsl_eigen_francis_free, gsl_eigen_francis_alloc, gsl_eigen_francis_T, GSL_EINVAL, GSL_ENOMEM, GSL_ENOTSQR, GSL_EBADLEN};

pub struct EigenNonsymmWorkspace {
    size: usize,
    Z: *mut gsl_matrix,
    do_balance: c_int,
    diag: *mut gsl_vector,
    tau: *mut gsl_vector,
    francis_workspace_p: *mut gsl_eigen_francis_workspace,
    n_evals: usize,
}

impl EigenNonsymmWorkspace {
    pub fn new(n: usize) -> Result<Self, i32> {
        if n == 0 {
            return Err(GSL_EINVAL);
        }

        let w = unsafe {
            calloc(1, mem::size_of::<gsl_eigen_nonsymm_workspace>()) as *mut gsl_eigen_nonsymm_workspace
        };
        if w.is_null() {
            return Err(GSL_ENOMEM);
        }

        let diag = unsafe { gsl_vector_alloc(n) };
        if diag.is_null() {
            unsafe { free(w as *mut _); }
            return Err(GSL_ENOMEM);
        }

        let tau = unsafe { gsl_vector_alloc(n) };
        if tau.is_null() {
            unsafe {
                gsl_vector_free(diag);
                free(w as *mut _);
            }
            return Err(GSL_ENOMEM);
        }

        let francis_workspace_p = unsafe { gsl_eigen_francis_alloc() };
        if francis_workspace_p.is_null() {
            unsafe {
                gsl_vector_free(tau);
                gsl_vector_free(diag);
                free(w as *mut _);
            }
            return Err(GSL_ENOMEM);
        }

        Ok(Self {
            size: n,
            Z: null_mut(),
            do_balance: 0,
            diag,
            tau,
            francis_workspace_p,
            n_evals: 0,
        })
    }

    pub fn free(&mut self) {
        unsafe {
            if !self.tau.is_null() {
                gsl_vector_free(self.tau);
                self.tau = null_mut();
            }
            if !self.diag.is_null() {
                gsl_vector_free(self.diag);
                self.diag = null_mut();
            }
            if !self.francis_workspace_p.is_null() {
                gsl_eigen_francis_free(self.francis_workspace_p);
                self.francis_workspace_p = null_mut();
            }
        }
    }

    pub fn params(&mut self, compute_t: c_int, balance: c_int) {
        unsafe {
            gsl_eigen_francis_T(compute_t, self.francis_workspace_p);
        }
        self.do_balance = balance;
    }

    pub fn nonsymm(&mut self, A: *mut gsl_matrix, eval: *mut gsl_vector_complex) -> Result<(), i32> {
        let N = unsafe { (*A).size1 };
        
        if N != unsafe { (*A).size2 } {
            return Err(GSL_ENOTSQR);
        } else if unsafe { (*eval).size } != N {
            return Err(GSL_EBADLEN);
        } else {
            let s;

            if self.do_balance != 0 {
                unsafe {
                    gsl_linalg_balance_matrix(A, self.diag);
                }
            }

            unsafe {
                gsl_linalg_hessenberg_decomp(A, self.tau);
            }

            if !self.Z.is_null() {
                unsafe {
                    gsl_linalg_hessenberg_unpack(A, self.tau, self.Z);
                    s = gsl_eigen_francis_Z(A, eval, self.Z, self.francis_workspace_p);
                    if self.do_balance != 0 {
                        gsl_linalg_balance_accum(self.Z, self.diag);
                    }
                }
            } else {
                unsafe {
                    s = gsl_eigen_francis(A, eval, self.francis_workspace_p);
                }
            }

            self.n_evals = unsafe { (*self.francis_workspace_p).n_evals };
            
            if s == 0 {
                Ok(())
            } else {
                Err(s)
            }
        }
    }

    pub fn nonsymm_Z(&mut self, A: *mut gsl_matrix, eval: *mut gsl_vector_complex, Z: *mut gsl_matrix) -> Result<(), i32> {
        if unsafe { (*A).size1 != (*A).size2 } {
            return Err(GSL_ENOTSQR);
        } else if unsafe { (*eval).size != (*A).size1 } {
            return Err(GSL_EBADLEN);
        } else if unsafe { (*Z).size1 != (*Z).size2 } || unsafe { (*Z).size1 != (*A).size1 } {
            return Err(GSL_EBADLEN);
        } else {
            self.Z = Z;
            let result = self.nonsymm(A, eval);
            self.Z = null_mut();
            result
        }
    }
}

impl Drop for EigenNonsymmWorkspace {
    fn drop(&mut self) {
        self.free();
    }
}