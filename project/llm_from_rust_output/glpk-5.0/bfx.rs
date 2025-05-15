use std::ptr;
use std::ffi::CString;
use std::os::raw::{c_int, c_void, c_char, c_ushort};

#[derive(Debug, Clone, Copy)]
pub struct mpz_seg {
    pub d: [c_ushort; 6],
    pub next: *mut mpz_seg,
}

#[derive(Debug, Clone, Copy)]
pub struct mpz {
    pub val: c_int,
    pub ptr: *mut mpz_seg,
}

#[derive(Debug, Clone, Copy)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}

pub type mpq_t = *mut mpq;

#[derive(Debug)]
pub struct LUXELM {
    pub i: c_int,
    pub j: c_int,
    pub val: mpq_t,
    pub r_prev: *mut LUXELM,
    pub r_next: *mut LUXELM,
    pub c_prev: *mut LUXELM,
    pub c_next: *mut LUXELM,
}

#[derive(Debug)]
pub struct LUX {
    pub n: c_int,
    pub pool: *mut c_void,
    pub F_row: *mut *mut LUXELM,
    pub F_col: *mut *mut LUXELM,
    pub V_piv: *mut mpq_t,
    pub V_row: *mut *mut LUXELM,
    pub V_col: *mut *mut LUXELM,
    pub P_row: *mut c_int,
    pub P_col: *mut c_int,
    pub Q_row: *mut c_int,
    pub Q_col: *mut c_int,
    pub rank: c_int,
}

#[derive(Debug)]
pub struct BFX {
    pub valid: c_int,
    pub lux: Option<Box<LUX>>,
}

impl BFX {
    pub fn new() -> Self {
        BFX {
            valid: 0,
            lux: None,
        }
    }

    pub fn factorize(
        &mut self,
        m: c_int,
        col: Option<extern "C" fn(*mut c_void, c_int, *mut c_int, *mut mpq_t) -> c_int>,
        info: *mut c_void,
    ) -> c_int {
        assert!(m > 0, "m must be greater than 0");

        if let Some(lux) = &self.lux {
            if lux.n != m {
                self.lux = None;
            }
        }

        if self.lux.is_none() {
            self.lux = Some(Box::new(LUX::new(m)));
        }

        let ret = unsafe { _glp_lux_decomp(self.lux.as_mut().unwrap().as_mut(), col, info) };
        self.valid = if ret == 0 { 1 } else { 0 };
        ret
    }

    pub fn ftran(&self, x: *mut mpq_t, save: c_int) {
        assert!(self.valid != 0, "binv must be valid");
        unsafe { _glp_lux_solve(self.lux.as_ref().unwrap().as_ref(), 0, x) };
        assert!(save == save, "save must equal save");
    }

    pub fn btran(&self, x: *mut mpq_t) {
        assert!(self.valid != 0, "binv must be valid");
        unsafe { _glp_lux_solve(self.lux.as_ref().unwrap().as_ref(), 1, x) };
    }

    pub fn update(&self, j: c_int) -> c_int {
        assert!(self.valid != 0, "binv must be valid");
        assert!(1 <= j && j <= self.lux.as_ref().unwrap().n, "j out of range");
        1
    }
}

impl Drop for BFX {
    fn drop(&mut self) {
        self.lux = None;
    }
}

impl LUX {
    pub fn new(n: c_int) -> Self {
        unsafe {
            let lux = _glp_lux_create(n);
            Box::from_raw(lux).as_ref().clone()
        }
    }
}

impl Drop for LUX {
    fn drop(&mut self) {
        unsafe {
            _glp_lux_delete(Box::into_raw(Box::new(self.clone())));
        }
    }
}

extern "C" {
    fn glp_free(ptr: *mut c_void);
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn _glp_lux_solve(lux: *mut LUX, tr: c_int, x: *mut mpq_t);
    fn _glp_lux_decomp(
        lux: *mut LUX,
        col: Option<extern "C" fn(*mut c_void, c_int, *mut c_int, *mut mpq_t) -> c_int>,
        info: *mut c_void,
    ) -> c_int;
    fn _glp_lux_create(n: c_int) -> *mut LUX;
    fn _glp_lux_delete(lux: *mut LUX);
}

pub fn create_binv() -> Box<BFX> {
    Box::new(BFX::new())
}

pub fn delete_binv(binv: Box<BFX>) {
    drop(binv);
}