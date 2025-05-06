#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32>,
    pub insert: Option<unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub get: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvacc_state_t {
    pub n: size_t,
    pub k: size_t,
    pub mean: libc::c_double,
    pub M2: libc::c_double,
    pub rbuf: *mut ringbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuf {
    pub array: *mut ringbuf_type_t,
    pub head: i32,
    pub tail: i32,
    pub size: i32,
}
pub type ringbuf_type_t = libc::c_double;
unsafe extern "C" fn ringbuf_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<ringbuf>() as u64) as size_t
        as size_t;
    size = (size as u64)
        .wrapping_add(n.wrapping_mul(::core::mem::size_of::<ringbuf_type_t>() as u64))
        as size_t as size_t;
    return size;
}
unsafe extern "C" fn ringbuf_init(n: size_t, mut b: *mut ringbuf) -> i32 {
    (*b).array = (b as *mut i8).offset(::core::mem::size_of::<ringbuf>() as u64 as isize)
        as *mut ringbuf_type_t;
    (*b).head = -(1 as i32);
    (*b).tail = 0 as i32;
    (*b).size = n as i32;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn ringbuf_is_empty(mut b: *const ringbuf) -> i32 {
    return ((*b).head == -(1 as i32)) as i32;
}
unsafe extern "C" fn ringbuf_is_full(mut b: *const ringbuf) -> i32 {
    return ((*b).head == 0 as i32 && (*b).tail == (*b).size - 1 as i32
        || (*b).head == (*b).tail + 1 as i32) as i32;
}
unsafe extern "C" fn ringbuf_insert(x: ringbuf_type_t, mut b: *mut ringbuf) -> i32 {
    if (*b).head == -(1 as i32) {
        (*b).head = 0 as i32;
        (*b).tail = 0 as i32;
    } else if (*b).head == 0 as i32 {
        (*b).head = (*b).size - 1 as i32;
        if (*b).tail == (*b).head && (*b).size > 1 as i32 {
            (*b).tail -= 1;
            (*b).tail;
        }
    } else {
        (*b).head -= 1;
        (*b).head;
        if (*b).tail == (*b).head {
            if (*b).tail == 0 as i32 {
                (*b).tail = (*b).size - 1 as i32;
            } else {
                (*b).tail -= 1;
                (*b).tail;
            }
        }
    }
    *((*b).array).offset((*b).head as isize) = x;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn ringbuf_pop_back(mut b: *mut ringbuf) -> i32 {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as i32 {
        gsl_error(
            b"buffer is empty\0" as *const u8 as *const i8,
            b"./ringbuf.c\0" as *const u8 as *const i8,
            133 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        if (*b).head == (*b).tail {
            (*b).head = -(1 as i32);
            (*b).tail = -(1 as i32);
        } else if (*b).tail == 0 as i32 {
            (*b).tail = (*b).size - 1 as i32;
        } else {
            (*b).tail -= 1;
            (*b).tail;
        }
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn ringbuf_peek_back(mut b: *const ringbuf) -> ringbuf_type_t {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as i32 {
        gsl_error(
            b"buffer is empty\0" as *const u8 as *const i8,
            b"./ringbuf.c\0" as *const u8 as *const i8,
            186 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32 as ringbuf_type_t;
    } else {
        return *((*b).array).offset((*b).tail as isize)
    };
}
unsafe extern "C" fn mvacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<mvacc_state_t>() as u64)
        as size_t as size_t;
    size = (size as u64).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    return size;
}
unsafe extern "C" fn mvacc_init(n: size_t, mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut mvacc_state_t = vstate as *mut mvacc_state_t;
    (*state).n = n;
    (*state).k = 0 as i32 as size_t;
    (*state).mean = 0.0f64;
    (*state).M2 = 0.0f64;
    (*state).rbuf = (vstate as *mut u8)
        .offset(::core::mem::size_of::<mvacc_state_t>() as u64 as isize) as *mut ringbuf;
    ringbuf_init(n, (*state).rbuf);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mvacc_insert(
    x: libc::c_double,
    mut vstate: *mut libc::c_void,
) -> i32 {
    let mut state: *mut mvacc_state_t = vstate as *mut mvacc_state_t;
    if ringbuf_is_full((*state).rbuf) != 0 {
        let mut old: libc::c_double = ringbuf_peek_back((*state).rbuf);
        let mut prev_mean: libc::c_double = (*state).mean;
        (*state).mean += (x - old) / (*state).n as libc::c_double;
        (*state).M2 += (old - prev_mean + (x - (*state).mean)) * (x - old);
    } else {
        let mut delta: libc::c_double = x - (*state).mean;
        (*state).k = ((*state).k).wrapping_add(1);
        (*state).k;
        (*state).mean += delta / (*state).k as libc::c_double;
        (*state).M2 += delta * (x - (*state).mean);
    }
    ringbuf_insert(x, (*state).rbuf);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mvacc_delete(mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut mvacc_state_t = vstate as *mut mvacc_state_t;
    if ringbuf_is_empty((*state).rbuf) == 0 {
        if (*state).k > 1 as i32 as u64 {
            let mut old: libc::c_double = ringbuf_peek_back((*state).rbuf);
            let mut prev_mean: libc::c_double = (*state).mean;
            let mut delta: libc::c_double = prev_mean - old;
            (*state).mean += delta / ((*state).k as libc::c_double - 1.0f64);
            (*state).M2 -= delta * ((*state).mean - old);
        } else if (*state).k == 1 as i32 as u64 {
            (*state).mean = 0.0f64;
            (*state).M2 = 0.0f64;
        }
        ringbuf_pop_back((*state).rbuf);
        (*state).k = ((*state).k).wrapping_sub(1);
        (*state).k;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mvacc_mean(
    mut params: *mut libc::c_void,
    mut result: *mut libc::c_double,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const mvacc_state_t = vstate as *const mvacc_state_t;
    *result = (*state).mean;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mvacc_variance(
    mut params: *mut libc::c_void,
    mut result: *mut libc::c_double,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const mvacc_state_t = vstate as *const mvacc_state_t;
    if (*state).k < 2 as i32 as u64 {
        *result = 0.0f64;
    } else {
        *result = (*state).M2 / ((*state).k as libc::c_double - 1.0f64);
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mvacc_sd(
    mut params: *mut libc::c_void,
    mut result: *mut libc::c_double,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut variance: libc::c_double = 0.;
    let mut status: i32 = mvacc_variance(params, &mut variance, vstate);
    *result = sqrt(variance);
    return status;
}
static mut mean_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mvacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mvacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mvacc_insert
                    as unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mvacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mvacc_mean
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_double,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_mean: *const gsl_movstat_accum = unsafe {
    &mean_accum_type as *const gsl_movstat_accum
};
static mut variance_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mvacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mvacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mvacc_insert
                    as unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mvacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mvacc_variance
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_double,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_variance: *const gsl_movstat_accum = unsafe {
    &variance_accum_type as *const gsl_movstat_accum
};
static mut sd_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mvacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mvacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mvacc_insert
                    as unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mvacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mvacc_sd
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_double,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_sd: *const gsl_movstat_accum = unsafe {
    &sd_accum_type as *const gsl_movstat_accum
};