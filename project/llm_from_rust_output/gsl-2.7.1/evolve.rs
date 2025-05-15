use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_double, c_int, c_ulong, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_system {
    pub function: Option<
        extern "C" fn(
            c_double,
            *const c_double,
            *mut c_double,
            *mut c_void,
        ) -> c_int,
    >,
    pub jacobian: Option<
        extern "C" fn(
            c_double,
            *const c_double,
            *mut c_double,
            *mut c_double,
            *mut c_void,
        ) -> c_int,
    >,
    pub dimension: usize,
    pub params: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_step_type {
    pub name: *const c_char,
    pub can_use_dydt_in: c_int,
    pub gives_exact_dydt_out: c_int,
    pub alloc: Option<extern "C" fn(usize) -> *mut c_void>,
    pub apply: Option<
        extern "C" fn(
            *mut c_void,
            usize,
            c_double,
            c_double,
            *mut c_double,
            *mut c_double,
            *const c_double,
            *mut c_double,
            *const gsl_odeiv_system,
        ) -> c_int,
    >,
    pub reset: Option<extern "C" fn(*mut c_void, usize) -> c_int>,
    pub order: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_step {
    pub type_: *const gsl_odeiv_step_type,
    pub dimension: usize,
    pub state: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_control_type {
    pub name: *const c_char,
    pub alloc: Option<extern "C" fn() -> *mut c_void>,
    pub init: Option<
        extern "C" fn(
            *mut c_void,
            c_double,
            c_double,
            c_double,
            c_double,
        ) -> c_int,
    >,
    pub hadjust: Option<
        extern "C" fn(
            *mut c_void,
            usize,
            c_uint,
            *const c_double,
            *const c_double,
            *const c_double,
            *mut c_double,
        ) -> c_int,
    >,
    pub free: Option<extern "C" fn(*mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_odeiv_control {
    pub type_: *const gsl_odeiv_control_type,
    pub state: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct gsl_odeiv_evolve {
    pub dimension: usize,
    pub y0: *mut c_double,
    pub yerr: *mut c_double,
    pub dydt_in: *mut c_double,
    pub dydt_out: *mut c_double,
    pub last_step: c_double,
    pub count: c_ulong,
    pub failed_steps: c_ulong,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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

impl gsl_odeiv_evolve {
    pub fn new(dim: usize) -> Result<Box<Self>, GslError> {
        let size = mem::size_of::<Self>();
        let e = unsafe { libc::malloc(size) as *mut Self };
        if e.is_null() {
            return Err(GslError::Enomem);
        }

        let e = unsafe { &mut *e };
        e.dimension = dim;
        e.count = 0;
        e.failed_steps = 0;
        e.last_step = 0.0;

        e.y0 = unsafe { libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double };
        if e.y0.is_null() {
            unsafe { libc::free(e as *mut _ as *mut c_void) };
            return Err(GslError::Enomem);
        }

        e.yerr = unsafe { libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double };
        if e.yerr.is_null() {
            unsafe { libc::free(e.y0 as *mut c_void) };
            unsafe { libc::free(e as *mut _ as *mut c_void) };
            return Err(GslError::Enomem);
        }

        e.dydt_in = unsafe { libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double };
        if e.dydt_in.is_null() {
            unsafe { libc::free(e.yerr as *mut c_void) };
            unsafe { libc::free(e.y0 as *mut c_void) };
            unsafe { libc::free(e as *mut _ as *mut c_void) };
            return Err(GslError::Enomem);
        }

        e.dydt_out = unsafe { libc::malloc(dim * mem::size_of::<c_double>()) as *mut c_double };
        if e.dydt_out.is_null() {
            unsafe { libc::free(e.dydt_in as *mut c_void) };
            unsafe { libc::free(e.yerr as *mut c_void) };
            unsafe { libc::free(e.y0 as *mut c_void) };
            unsafe { libc::free(e as *mut _ as *mut c_void) };
            return Err(GslError::Enomem);
        }

        Ok(unsafe { Box::from_raw(e) })
    }

    pub fn reset(&mut self) -> GslError {
        self.count = 0;
        self.failed_steps = 0;
        self.last_step = 0.0;
        GslError::Success
    }

    pub fn apply(
        &mut self,
        con: &mut gsl_odeiv_control,
        step: &mut gsl_odeiv_step,
        dydt: &gsl_odeiv_system,
        t: &mut c_double,
        t1: c_double,
        h: &mut c_double,
        y: &mut [c_double],
    ) -> Result<(), GslError> {
        let t0 = *t;
        let mut h0 = *h;
        let dt = t1 - t0;

        if self.dimension != step.dimension {
            return Err(GslError::Einval);
        }

        if (dt < 0.0 && h0 > 0.0) || (dt > 0.0 && h0 < 0.0) {
            return Err(GslError::Einval);
        }

        unsafe {
            ptr::copy_nonoverlapping(
                y.as_ptr(),
                self.y0,
                self.dimension,
            );
        }

        if unsafe { (*step.type_).can_use_dydt_in != 0 } {
            let status = unsafe {
                (dydt.function.unwrap())(
                    t0,
                    y.as_ptr(),
                    self.dydt_in,
                    dydt.params,
                )
            };
            if status != GslError::Success as c_int {
                return Err(GslError::from(status));
            }
        }

        loop {
            let final_step = if (dt >= 0.0 && h0 > dt) || (dt < 0.0 && h0 < dt) {
                h0 = dt;
                true
            } else {
                false
            };

            let step_status = if unsafe { (*step.type_).can_use_dydt_in != 0 } {
                unsafe {
                    ((*step.type_).apply.unwrap())(
                        step.state,
                        step.dimension,
                        t0,
                        h0,
                        y.as_mut_ptr(),
                        self.yerr,
                        self.dydt_in,
                        self.dydt_out,
                        dydt,
                    )
                }
            } else {
                unsafe {
                    ((*step.type_).apply.unwrap())(
                        step.state,
                        step.dimension,
                        t0,
                        h0,
                        y.as_mut_ptr(),
                        self.yerr,
                        ptr::null(),
                        self.dydt_out,
                        dydt,
                    )
                }
            };

            if step_status != GslError::Success as c_int {
                *h = h0;
                *t = t0;
                return Err(GslError::from(step_status));
            }

            self.count += 1;
            self.last_step = h0;

            *t = if final_step { t1 } else { t0 + h0 };

            if ptr::eq(con, ptr::null_mut()) {
                break;
            }

            let h_old = h0;
            let hadjust_status = unsafe {
                ((*con.type_).hadjust.unwrap())(
                    con.state,
                    step.dimension,
                    0,
                    y.as_ptr(),
                    self.yerr,
                    self.dydt_out,
                    &mut h0,
                )
            };

            if hadjust_status != -1 {
                break;
            }

            let t_curr = unsafe { libc::fabs(*t) };
            let t_next = unsafe { libc::fabs(*t + h0) };
            if unsafe { libc::fabs(h0) } < unsafe { libc::fabs(h_old) } && t_next != t_curr {
                unsafe {
                    ptr::copy_nonoverlapping(
                        self.y0,
                        y.as_mut_ptr(),
                        dydt.dimension,
                    );
                }
                self.failed_steps += 1;
            } else {
                h0 = h_old;
                break;
            }
        }

        *h = h0;
        Ok(())
    }
}

impl Drop for gsl_odeiv_evolve {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.dydt_out as *mut c_void);
            libc::free(self.dydt_in as *mut c_void);
            libc::free(self.yerr as *mut c_void);
            libc::free(self.y0 as *mut c_void);
        }
    }
}

impl From<c_int> for GslError {
    fn from(value: c_int) -> Self {
        unsafe { mem::transmute(value) }
    }
}