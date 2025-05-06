#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
pub type medacc_type_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medacc_state_t {
    pub n: i32,
    pub idx: i32,
    pub ct: i32,
    pub data: *mut medacc_type_t,
    pub pos: *mut i32,
    pub heap: *mut i32,
}
unsafe extern "C" fn medacc_size(n: size_t) -> size_t {
    let mut size: size_t = 0 as i32 as size_t;
    size = (size as u64).wrapping_add(::core::mem::size_of::<medacc_state_t>() as u64)
        as size_t as size_t;
    size = (size as u64)
        .wrapping_add(n.wrapping_mul(::core::mem::size_of::<medacc_type_t>() as u64))
        as size_t as size_t;
    size = (size as u64)
        .wrapping_add(
            (2 as i32 as u64)
                .wrapping_mul(n)
                .wrapping_mul(::core::mem::size_of::<i32>() as u64),
        ) as size_t as size_t;
    return size;
}
unsafe extern "C" fn medacc_init(n: size_t, mut vstate: *mut libc::c_void) -> i32 {
    let mut state: *mut medacc_state_t = vstate as *mut medacc_state_t;
    let mut k: i32 = n as i32;
    (*state).n = n as i32;
    (*state).ct = 0 as i32;
    (*state).idx = 0 as i32;
    (*state).data = (vstate as *mut u8)
        .offset(::core::mem::size_of::<medacc_state_t>() as u64 as isize)
        as *mut medacc_type_t;
    (*state).pos = ((*state).data as *mut u8)
        .offset(n.wrapping_mul(::core::mem::size_of::<medacc_type_t>() as u64) as isize)
        as *mut i32;
    (*state).heap = ((*state).pos)
        .offset(n as isize)
        .offset(n.wrapping_div(2 as i32 as u64) as isize);
    loop {
        let fresh0 = k;
        k = k - 1;
        if !(fresh0 != 0) {
            break;
        }
        *((*state).pos).offset(k as isize) = (k + 1 as i32) / 2 as i32
            * (if k & 1 as i32 != 0 { -(1 as i32) } else { 1 as i32 });
        *((*state).heap).offset(*((*state).pos).offset(k as isize) as isize) = k;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn medacc_insert(
    x: medacc_type_t,
    mut vstate: *mut libc::c_void,
) -> i32 {
    let mut state: *mut medacc_state_t = vstate as *mut medacc_state_t;
    let mut isNew: i32 = ((*state).ct < (*state).n) as i32;
    let mut p: i32 = *((*state).pos).offset((*state).idx as isize);
    let mut old: medacc_type_t = *((*state).data).offset((*state).idx as isize);
    *((*state).data).offset((*state).idx as isize) = x;
    (*state).idx = ((*state).idx + 1 as i32) % (*state).n;
    (*state).ct += isNew;
    if p > 0 as i32 {
        if isNew == 0 && old < x {
            minSortDown(state, p * 2 as i32);
        } else if minSortUp(state, p) != 0 {
            maxSortDown(state, -(1 as i32));
        }
    } else if p < 0 as i32 {
        if isNew == 0 && x < old {
            maxSortDown(state, p * 2 as i32);
        } else if maxSortUp(state, p) != 0 {
            minSortDown(state, 1 as i32);
        }
    } else {
        if (*state).ct / 2 as i32 != 0 {
            maxSortDown(state, -(1 as i32));
        }
        if ((*state).ct - 1 as i32) / 2 as i32 != 0 {
            minSortDown(state, 1 as i32);
        }
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn medacc_get(
    mut params: *mut libc::c_void,
    mut result: *mut medacc_type_t,
    mut vstate: *const libc::c_void,
) -> i32 {
    let mut state: *const medacc_state_t = vstate as *const medacc_state_t;
    let mut median: medacc_type_t = *((*state).data)
        .offset(*((*state).heap).offset(0 as i32 as isize) as isize);
    if (*state).ct & 1 as i32 == 0 as i32 {
        median = (median
            + *((*state).data)
                .offset(*((*state).heap).offset(-(1 as i32) as isize) as isize))
            / 2 as i32 as libc::c_double;
    }
    *result = median;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn mmless(mut state: *const medacc_state_t, i: i32, j: i32) -> i32 {
    return (*((*state).data).offset(*((*state).heap).offset(i as isize) as isize)
        < *((*state).data).offset(*((*state).heap).offset(j as isize) as isize)) as i32;
}
unsafe extern "C" fn mmexchange(mut state: *mut medacc_state_t, i: i32, j: i32) -> i32 {
    let mut t: i32 = *((*state).heap).offset(i as isize);
    *((*state).heap).offset(i as isize) = *((*state).heap).offset(j as isize);
    *((*state).heap).offset(j as isize) = t;
    *((*state).pos).offset(*((*state).heap).offset(i as isize) as isize) = i;
    *((*state).pos).offset(*((*state).heap).offset(j as isize) as isize) = j;
    return 1 as i32;
}
unsafe extern "C" fn mmCmpExch(mut state: *mut medacc_state_t, i: i32, j: i32) -> i32 {
    return (mmless(state, i, j) != 0 && mmexchange(state, i, j) != 0) as i32;
}
unsafe extern "C" fn minSortDown(mut state: *mut medacc_state_t, mut i: i32) {
    while i <= ((*state).ct - 1 as i32) / 2 as i32 {
        if i > 1 as i32 && i < ((*state).ct - 1 as i32) / 2 as i32
            && mmless(state, i + 1 as i32, i) != 0
        {
            i += 1;
            i;
        }
        if mmCmpExch(state, i, i / 2 as i32) == 0 {
            break;
        }
        i *= 2 as i32;
    }
}
unsafe extern "C" fn maxSortDown(mut state: *mut medacc_state_t, mut i: i32) {
    while i >= -((*state).ct / 2 as i32) {
        if i < -(1 as i32) && i > -((*state).ct / 2 as i32)
            && mmless(state, i, i - 1 as i32) != 0
        {
            i -= 1;
            i;
        }
        if mmCmpExch(state, i / 2 as i32, i) == 0 {
            break;
        }
        i *= 2 as i32;
    }
}
unsafe extern "C" fn minSortUp(mut state: *mut medacc_state_t, mut i: i32) -> i32 {
    while i > 0 as i32 && mmCmpExch(state, i, i / 2 as i32) != 0 {
        i /= 2 as i32;
    }
    return (i == 0 as i32) as i32;
}
unsafe extern "C" fn maxSortUp(mut state: *mut medacc_state_t, mut i: i32) -> i32 {
    while i < 0 as i32 && mmCmpExch(state, i / 2 as i32, i) != 0 {
        i /= 2 as i32;
    }
    return (i == 0 as i32) as i32;
}
static mut median_accum_type: gsl_movstat_accum = unsafe {
    {
        let mut init = gsl_movstat_accum {
            size: Some(medacc_size as unsafe extern "C" fn(size_t) -> size_t),
            init: Some(
                medacc_init as unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32,
            ),
            insert: Some(
                medacc_insert
                    as unsafe extern "C" fn(medacc_type_t, *mut libc::c_void) -> i32,
            ),
            delete_oldest: None,
            get: Some(
                medacc_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut medacc_type_t,
                        *const libc::c_void,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_movstat_accum_median: *const gsl_movstat_accum = unsafe {
    &median_accum_type as *const gsl_movstat_accum
};