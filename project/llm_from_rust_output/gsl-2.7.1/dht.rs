use std::ptr;
use std::mem;
use std::ffi::CString;
use libc::{c_double, c_uint, c_int, c_ulong, c_void};

pub type size_t = c_ulong;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_result {
    pub val: c_double,
    pub err: c_double,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct gsl_dht {
    pub size: size_t,
    pub nu: c_double,
    pub xmax: c_double,
    pub kmax: c_double,
    pub j: *mut c_double,
    pub Jjj: *mut c_double,
    pub J2: *mut c_double,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

extern "C" {
    fn gsl_sf_bessel_Jnu_e(nu: c_double, x: c_double, result: *mut gsl_sf_result) -> c_int;
    fn gsl_sf_bessel_zero_Jnu_e(nu: c_double, s: c_uint, result: *mut gsl_sf_result) -> c_int;
    fn gsl_error(reason: *const libc::c_char, file: *const libc::c_char, line: c_int, gsl_errno: c_int);
}

pub struct Dht {
    inner: Box<gsl_dht>,
}

impl Dht {
    pub fn new(size: size_t, nu: c_double, xmax: c_double) -> Option<Self> {
        if size == 0 {
            unsafe {
                gsl_error(
                    CString::new("size == 0").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    37,
                    GslError::Dom as c_int,
                );
            }
            return None;
        }

        if xmax <= 0.0 {
            unsafe {
                gsl_error(
                    CString::new("xmax is not positive").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    120,
                    GslError::Dom as c_int,
                );
            }
            return None;
        }

        if nu < 0.0 {
            unsafe {
                gsl_error(
                    CString::new("nu is negative").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    122,
                    GslError::Dom as c_int,
                );
            }
            return None;
        }

        let mut dht = Box::new(gsl_dht {
            size,
            nu,
            xmax,
            kmax: 0.0,
            j: ptr::null_mut(),
            Jjj: ptr::null_mut(),
            J2: ptr::null_mut(),
        });

        unsafe {
            dht.j = libc::malloc((size + 2) * mem::size_of::<c_double>() as c_ulong) as *mut c_double;
            if dht.j.is_null() {
                libc::free(dht as *mut _ as *mut c_void);
                gsl_error(
                    CString::new("could not allocate memory for j").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    55,
                    GslError::Nomem as c_int,
                );
                return None;
            }

            dht.Jjj = libc::malloc((size * (size + 1) / 2) * mem::size_of::<c_double>() as c_ulong) as *mut c_double;
            if dht.Jjj.is_null() {
                libc::free(dht.j as *mut c_void);
                libc::free(dht as *mut _ as *mut c_void);
                gsl_error(
                    CString::new("could not allocate memory for Jjj").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    63,
                    GslError::Nomem as c_int,
                );
                return None;
            }

            dht.J2 = libc::malloc((size + 1) * mem::size_of::<c_double>() as c_ulong) as *mut c_double;
            if dht.J2.is_null() {
                libc::free(dht.Jjj as *mut c_void);
                libc::free(dht.j as *mut c_void);
                libc::free(dht as *mut _ as *mut c_void);
                gsl_error(
                    CString::new("could not allocate memory for J2").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    72,
                    GslError::Nomem as c_int,
                );
                return None;
            }
        }

        if let Err(_) = dht.init(nu, xmax) {
            return None;
        }

        Some(Dht { inner: dht })
    }

    fn init(&mut self, nu: c_double, xmax: c_double) -> Result<(), GslError> {
        self.inner.nu = nu;
        self.inner.xmax = xmax;

        if self.compute_bessel_zeros() != GslError::Success {
            return Err(GslError::Failed);
        }

        let jN = unsafe { *self.inner.j.offset((self.inner.size + 1) as isize) };
        self.inner.kmax = jN / xmax;

        if self.compute_bessel_functions() != GslError::Success {
            return Err(GslError::Failed);
        }

        Ok(())
    }

    fn compute_bessel_zeros(&mut self) -> GslError {
        unsafe {
            *self.inner.j.offset(0) = 0.0;
            let mut stat_z = GslError::Success as c_int;

            for s in 1..(self.inner.size + 2) {
                let mut z = gsl_sf_result { val: 0.0, err: 0.0 };
                stat_z += gsl_sf_bessel_zero_Jnu_e(self.inner.nu, s as c_uint, &mut z);
                *self.inner.j.offset(s as isize) = z.val;
            }

            if stat_z != 0 {
                gsl_error(
                    CString::new("could not compute bessel zeroes").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    91,
                    GslError::Failed as c_int,
                );
                GslError::Failed
            } else {
                GslError::Success
            }
        }
    }

    fn compute_bessel_functions(&mut self) -> GslError {
        unsafe {
            let jN = *self.inner.j.offset((self.inner.size + 1) as isize);
            let mut stat_J = 0;

            *self.inner.J2.offset(0) = 0.0;

            for m in 1..(self.inner.size + 1) {
                let mut J = gsl_sf_result { val: 0.0, err: 0.0 };
                stat_J += gsl_sf_bessel_Jnu_e(
                    self.inner.nu + 1.0,
                    *self.inner.j.offset(m as isize),
                    &mut J,
                );
                *self.inner.J2.offset(m as isize) = J.val * J.val;
            }

            for n in 1..(self.inner.size + 1) {
                for m in 1..=n {
                    let arg = *self.inner.j.offset(n as isize) * *self.inner.j.offset(m as isize) / jN;
                    let mut J_0 = gsl_sf_result { val: 0.0, err: 0.0 };
                    stat_J += gsl_sf_bessel_Jnu_e(self.inner.nu, arg, &mut J_0);
                    let idx = n * (n - 1) / 2 + m - 1;
                    *self.inner.Jjj.offset(idx as isize) = J_0.val;
                }
            }

            if stat_J != 0 {
                gsl_error(
                    CString::new("error computing bessel function").unwrap().as_ptr(),
                    CString::new("dht.c").unwrap().as_ptr(),
                    160,
                    GslError::Failed as c_int,
                );
                GslError::Failed
            } else {
                GslError::Success
            }
        }
    }

    pub fn x_sample(&self, n: c_int) -> c_double {
        unsafe {
            *self.inner.j.offset((n + 1) as isize) / 
            *self.inner.j.offset((self.inner.size + 1) as isize) * self.inner.xmax
        }
    }

    pub fn k_sample(&self, n: c_int) -> c_double {
        unsafe {
            *self.inner.j.offset((n + 1) as isize) / self.inner.xmax
        }
    }

    pub fn apply(&self, f_in: &[c_double], f_out: &mut [c_double]) -> Result<(), GslError> {
        if f_in.len() != self.inner.size as usize || f_out.len() != self.inner.size as usize {
            return Err(GslError::Badlen);
        }

        unsafe {
            let jN = *self.inner.j.offset((self.inner.size + 1) as isize);
            let r = self.inner.xmax / jN;

            for m in 0..self.inner.size {
                let mut sum = 0.0;
                for i in 0..self.inner.size {
                    let (m_local, n_local) = if i < m { (i, m) } else { (m, i) };
                    let idx = n_local * (n_local + 1) / 2 + m_local;
                    let Y = *self.inner.Jjj.offset(idx as isize) / 
                           *self.inner.J2.offset((i + 1) as isize);
                    sum += Y * f_in[i as usize];
                }
                f_out[m as usize] = sum * 2.0 * r * r;
            }
        }

        Ok(())
    }
}

impl Drop for Dht {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.inner.J2 as *mut c_void);
            libc::free(self.inner.Jjj as *mut c_void);
            libc::free(self.inner.j as *mut c_void);
        }
    }
}