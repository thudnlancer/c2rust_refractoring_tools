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
pub type mmacc_type_t = libc::c_double;
pub type ringbuf_type_t = mmacc_type_t;
pub type deque_type_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deque {
    pub head: i32,
    pub tail: i32,
    pub size: i32,
    pub array: *mut deque_type_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmacc_state_t {
    pub n: size_t,
    pub k: size_t,
    pub xprev: mmacc_type_t,
    pub rbuf: *mut ringbuf,
    pub minque: *mut deque,
    pub maxque: *mut deque,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ringbuf {
    pub array: *mut ringbuf_type_t,
    pub head: i32,
    pub tail: i32,
    pub size: i32,
}
unsafe extern "C" fn deque_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<deque>() as u64) as size_t
        as size_t;
    size = (size as u64)
        .wrapping_add(n.wrapping_mul(::core::mem::size_of::<deque_type_t>() as u64))
        as size_t as size_t;
    return size;
}
unsafe extern "C" fn deque_init(n: size_t, mut d: *mut deque) -> i32 {
    (*d).head = -(1 as i32);
    (*d).tail = 0 as i32;
    (*d).size = n as i32;
    (*d).array = (d as *mut u8).offset(::core::mem::size_of::<deque>() as u64 as isize)
        as *mut deque_type_t;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn deque_is_empty(mut d: *const deque) -> i32 {
    return ((*d).head == -(1 as i32)) as i32;
}
unsafe extern "C" fn deque_is_full(mut d: *const deque) -> i32 {
    return ((*d).head == 0 as i32 && (*d).tail == (*d).size - 1 as i32
        || (*d).head == (*d).tail + 1 as i32) as i32;
}
unsafe extern "C" fn deque_push_back(x: deque_type_t, mut d: *mut deque) -> i32 {
    if deque_is_full(d) != 0 {
        gsl_error(
            b"deque is full\0" as *const u8 as *const i8,
            b"./deque.c\0" as *const u8 as *const i8,
            125 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        if (*d).head == -(1 as i32) {
            (*d).head = 0 as i32;
            (*d).tail = 0 as i32;
        } else if (*d).tail == (*d).size - 1 as i32 {
            (*d).tail = 0 as i32;
        } else {
            (*d).tail += 1;
            (*d).tail;
        }
        *((*d).array).offset((*d).tail as isize) = x;
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn deque_pop_front(mut d: *mut deque) -> i32 {
    if deque_is_empty(d) != 0 {
        gsl_error(
            b"cannot pop element from empty queue\0" as *const u8 as *const i8,
            b"./deque.c\0" as *const u8 as *const i8,
            155 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        if (*d).head == (*d).tail {
            (*d).head = -(1 as i32);
            (*d).tail = -(1 as i32);
        } else if (*d).head == (*d).size - 1 as i32 {
            (*d).head = 0 as i32;
        } else {
            (*d).head += 1;
            (*d).head;
        }
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn deque_pop_back(mut d: *mut deque) -> i32 {
    if deque_is_empty(d) != 0 {
        gsl_error(
            b"cannot pop element from empty queue\0" as *const u8 as *const i8,
            b"./deque.c\0" as *const u8 as *const i8,
            182 as i32,
            GSL_EOVRFLW as i32,
        );
        return GSL_EOVRFLW as i32;
    } else {
        if (*d).head == (*d).tail {
            (*d).head = -(1 as i32);
            (*d).tail = -(1 as i32);
        } else if (*d).tail == 0 as i32 {
            (*d).tail = (*d).size - 1 as i32;
        } else {
            (*d).tail -= 1;
            (*d).tail;
        }
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn deque_peek_front(mut d: *const deque) -> deque_type_t {
    if deque_is_empty(d) != 0 {
        gsl_error(
            b"queue is empty\0" as *const u8 as *const i8,
            b"./deque.c\0" as *const u8 as *const i8,
            209 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        return *((*d).array).offset((*d).head as isize)
    };
}
unsafe extern "C" fn deque_peek_back(mut d: *const deque) -> deque_type_t {
    if deque_is_empty(d) != 0 || (*d).tail < 0 as i32 {
        gsl_error(
            b"queue is empty\0" as *const u8 as *const i8,
            b"./deque.c\0" as *const u8 as *const i8,
            222 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        return *((*d).array).offset((*d).tail as isize)
    };
}
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
unsafe extern "C" fn mmacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<mmacc_state_t>() as u64)
        as size_t as size_t;
    size = (size as u64).wrapping_add(ringbuf_size(n)) as size_t as size_t;
    size = (size as u64)
        .wrapping_add(
            (2 as i32 as u64).wrapping_mul(deque_size(n.wrapping_add(1 as i32 as u64))),
        ) as size_t as size_t;
    return size;
}
unsafe extern "C" fn mmacc_init(n: size_t, mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut mmacc_state_t = vstate as *mut mmacc_state_t;
    (*state).n = n;
    (*state).k = 0 as i32 as size_t;
    (*state).xprev = 0.0f64;
    (*state).rbuf = (vstate as *mut u8)
        .offset(::core::mem::size_of::<mmacc_state_t>() as u64 as isize) as *mut ringbuf;
    (*state).minque = ((*state).rbuf as *mut u8).offset(ringbuf_size(n) as isize)
        as *mut deque;
    (*state).maxque = ((*state).minque as *mut u8)
        .offset(deque_size(n.wrapping_add(1 as i32 as u64)) as isize) as *mut deque;
    ringbuf_init(n, (*state).rbuf);
    deque_init(n.wrapping_add(1 as i32 as u64), (*state).minque);
    deque_init(n.wrapping_add(1 as i32 as u64), (*state).maxque);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mmacc_insert(
    x: mmacc_type_t,
    mut vstate: *mut libc::c_void,
) -> i32 {
    let mut state: *mut mmacc_state_t = vstate as *mut mmacc_state_t;
    let mut head: i32 = 0;
    let mut tail: i32 = 0;
    if (*state).k == 0 as i32 as u64 {
        ringbuf_insert(x, (*state).rbuf);
        head = (*(*state).rbuf).head;
        deque_push_back(head, (*state).maxque);
        deque_push_back(head, (*state).minque);
    } else {
        if x > (*state).xprev {
            deque_pop_back((*state).maxque);
            while deque_is_empty((*state).maxque) == 0 {
                if x
                    <= *((*(*state).rbuf).array)
                        .offset(deque_peek_back((*state).maxque) as isize)
                {
                    break;
                }
                deque_pop_back((*state).maxque);
            }
        } else {
            deque_pop_back((*state).minque);
            while deque_is_empty((*state).minque) == 0 {
                if x
                    >= *((*(*state).rbuf).array)
                        .offset(deque_peek_back((*state).minque) as isize)
                {
                    break;
                }
                deque_pop_back((*state).minque);
            }
        }
        tail = (*(*state).rbuf).tail;
        ringbuf_insert(x, (*state).rbuf);
        head = (*(*state).rbuf).head;
        deque_push_back(head, (*state).maxque);
        deque_push_back(head, (*state).minque);
        if (*state).k == (*state).n {
            if (*(*state).maxque).head != (*(*state).maxque).tail
                && tail == deque_peek_front((*state).maxque)
            {
                deque_pop_front((*state).maxque);
            } else if (*(*state).minque).head != (*(*state).minque).tail
                && tail == deque_peek_front((*state).minque)
            {
                deque_pop_front((*state).minque);
            }
        }
    }
    if (*state).k < (*state).n {
        (*state).k = ((*state).k).wrapping_add(1);
        (*state).k;
    }
    (*state).xprev = x;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mmacc_delete(mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut mmacc_state_t = vstate as *mut mmacc_state_t;
    if (*state).k > 0 as i32 as u64 {
        if (*(*state).rbuf).tail == deque_peek_front((*state).maxque) {
            deque_pop_front((*state).maxque);
        } else if (*(*state).rbuf).tail == deque_peek_front((*state).minque) {
            deque_pop_front((*state).minque);
        }
        ringbuf_pop_back((*state).rbuf);
        (*state).k = ((*state).k).wrapping_sub(1);
        (*state).k;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mmacc_min(
    mut params: *mut libc::c_void,
    mut result: *mut mmacc_type_t,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const mmacc_state_t = vstate as *const mmacc_state_t;
    if (*state).k == 0 as i32 as u64 {
        gsl_error(
            b"no samples yet added to workspace\0" as *const u8 as *const i8,
            b"mmacc.c\0" as *const u8 as *const i8,
            200 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else {
        *result = *((*(*state).rbuf).array)
            .offset(deque_peek_front((*state).minque) as isize);
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn mmacc_max(
    mut params: *mut libc::c_void,
    mut result: *mut mmacc_type_t,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const mmacc_state_t = vstate as *const mmacc_state_t;
    if (*state).k == 0 as i32 as u64 {
        gsl_error(
            b"no samples yet added to workspace\0" as *const u8 as *const i8,
            b"mmacc.c\0" as *const u8 as *const i8,
            218 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else {
        *result = *((*(*state).rbuf).array)
            .offset(deque_peek_front((*state).maxque) as isize);
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn mmacc_minmax(
    mut params: *mut libc::c_void,
    mut result: *mut mmacc_type_t,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const mmacc_state_t = vstate as *const mmacc_state_t;
    if (*state).k == 0 as i32 as u64 {
        gsl_error(
            b"no samples yet added to workspace\0" as *const u8 as *const i8,
            b"mmacc.c\0" as *const u8 as *const i8,
            236 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else {
        *result.offset(0 as i32 as isize) = *((*(*state).rbuf).array)
            .offset(deque_peek_front((*state).minque) as isize);
        *result.offset(1 as i32 as isize) = *((*(*state).rbuf).array)
            .offset(deque_peek_front((*state).maxque) as isize);
        return GSL_SUCCESS as i32;
    };
}
static mut min_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mmacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mmacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mmacc_insert
                    as unsafe extern "C" fn(mmacc_type_t, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mmacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mmacc_min
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut mmacc_type_t,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_min: *const gsl_movstat_accum = unsafe {
    &min_accum_type as *const gsl_movstat_accum
};
static mut max_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mmacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mmacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mmacc_insert
                    as unsafe extern "C" fn(mmacc_type_t, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mmacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mmacc_max
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut mmacc_type_t,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_max: *const gsl_movstat_accum = unsafe {
    &max_accum_type as *const gsl_movstat_accum
};
static mut minmax_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(mmacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                mmacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                mmacc_insert
                    as unsafe extern "C" fn(mmacc_type_t, *mut libc::c_void) -> i32,
            ),
            delete_oldest: Some(
                mmacc_delete as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            get: Some(
                mmacc_minmax
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut mmacc_type_t,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_minmax: *const gsl_movstat_accum = unsafe {
    &minmax_accum_type as *const gsl_movstat_accum
};