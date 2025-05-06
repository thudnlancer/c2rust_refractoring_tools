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
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_sort(data: *mut libc::c_double, stride: size_t, n: size_t);
    fn gsl_stats_Sn_from_sorted_data(
        sorted_data: *const libc::c_double,
        stride: size_t,
        n: size_t,
        work: *mut libc::c_double,
    ) -> libc::c_double;
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
pub type snacc_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snacc_state_t {
    pub window: *mut snacc_type_t,
    pub work: *mut snacc_type_t,
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
pub type ringbuf_type_t = snacc_type_t;
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
unsafe extern "C" fn ringbuf_copy(
    mut dest: *mut libc::c_double,
    mut b: *const ringbuf,
) -> size_t {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 as i32 {
        return 0 as i32 as size_t
    } else {
        let n: i32 = ringbuf_n(b);
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < n {
            *dest.offset(i as isize) = *((*b).array)
                .offset((((*b).head + i) % (*b).size) as isize);
            i += 1;
            i;
        }
        return n as size_t;
    };
}
unsafe extern "C" fn ringbuf_n(mut b: *const ringbuf) -> i32 {
    let n: i32 = if (*b).head > (*b).tail {
        (*b).size - (*b).head + (*b).tail + 1 as i32
    } else {
        (*b).tail - (*b).head + 1 as i32
    };
    return n;
}
unsafe extern "C" fn snacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<snacc_state_t>() as u64)
        as size_t as size_t;
    size = (size as u64)
        .wrapping_add(
            (2 as i32 as u64)
                .wrapping_mul(n)
                .wrapping_mul(::core::mem::size_of::<snacc_type_t>() as u64),
        ) as size_t as size_t;
    size = (size as u64).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    return size;
}
unsafe extern "C" fn snacc_init(n: size_t, mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut snacc_state_t = vstate as *mut snacc_state_t;
    (*state).window = (vstate as *mut u8)
        .offset(::core::mem::size_of::<snacc_state_t>() as u64 as isize)
        as *mut snacc_type_t;
    (*state).work = ((*state).window as *mut u8)
        .offset(n.wrapping_mul(::core::mem::size_of::<snacc_type_t>() as u64) as isize)
        as *mut snacc_type_t;
    (*state).rbuf = ((*state).work as *mut u8)
        .offset(n.wrapping_mul(::core::mem::size_of::<snacc_type_t>() as u64) as isize)
        as *mut ringbuf;
    ringbuf_init(n, (*state).rbuf);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn snacc_insert(
    x: snacc_type_t,
    mut vstate: *mut libc::c_void,
) -> i32 {
    let mut state: *mut snacc_state_t = vstate as *mut snacc_state_t;
    ringbuf_insert(x, (*state).rbuf);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn snacc_delete(mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut snacc_state_t = vstate as *mut snacc_state_t;
    if ringbuf_is_empty((*state).rbuf) == 0 {
        ringbuf_pop_back((*state).rbuf);
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn snacc_get(
    mut params: *mut libc::c_void,
    mut result: *mut snacc_type_t,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const snacc_state_t = vstate as *const snacc_state_t;
    let mut n: size_t = ringbuf_copy((*state).window, (*state).rbuf);
    gsl_sort((*state).window, 1 as i32 as size_t, n);
    *result = gsl_stats_Sn_from_sorted_data(
        (*state).window as *const libc::c_double,
        1 as i32 as size_t,
        n,
        (*state).work,
    );
    return GSL_SUCCESS as i32;
}
static mut sn_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(snacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                snacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                snacc_insert
                    as unsafe extern "C" fn(snacc_type_t, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                snacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                snacc_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut snacc_type_t,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_Sn: *const gsl_movstat_accum = unsafe {
    &sn_accum_type as *const gsl_movstat_accum
};