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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ran3_state_t {
    pub x: u32,
    pub y: u32,
    pub buffer: [u64; 56],
}
#[inline]
unsafe extern "C" fn ran3_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran3_state_t = vstate as *mut ran3_state_t;
    let mut j: i64 = 0;
    (*state).x = ((*state).x).wrapping_add(1);
    (*state).x;
    if (*state).x == 56 as i32 as u32 {
        (*state).x = 1 as i32 as u32;
    }
    (*state).y = ((*state).y).wrapping_add(1);
    (*state).y;
    if (*state).y == 56 as i32 as u32 {
        (*state).y = 1 as i32 as u32;
    }
    j = ((*state).buffer[(*state).x as usize])
        .wrapping_sub((*state).buffer[(*state).y as usize]) as i64;
    if j < 0 as i32 as i64 {
        j += 1000000000 as i32 as i64;
    }
    (*state).buffer[(*state).x as usize] = j as u64;
    return j as u64;
}
unsafe extern "C" fn ran3_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ran3_get(vstate) as libc::c_double / 1000000000 as i32 as libc::c_double;
}
unsafe extern "C" fn ran3_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran3_state_t = vstate as *mut ran3_state_t;
    let mut i: i32 = 0;
    let mut i1: i32 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    j = (161803398 as i32 as u64).wrapping_sub(s).wrapping_rem(1000000000 as i32 as u64)
        as i64;
    if j < 0 as i32 as i64 {
        j += 1000000000 as i32 as i64;
    }
    (*state).buffer[0 as i32 as usize] = 0 as i32 as u64;
    (*state).buffer[55 as i32 as usize] = j as u64;
    k = 1 as i32 as i64;
    i = 1 as i32;
    while i < 55 as i32 {
        let mut n: i32 = 21 as i32 * i % 55 as i32;
        (*state).buffer[n as usize] = k as u64;
        k = j - k;
        if k < 0 as i32 as i64 {
            k += 1000000000 as i32 as i64;
        }
        j = (*state).buffer[n as usize] as i64;
        i += 1;
        i;
    }
    i1 = 0 as i32;
    while i1 < 4 as i32 {
        i = 1 as i32;
        while i < 56 as i32 {
            let mut t: i64 = ((*state).buffer[i as usize])
                .wrapping_sub(
                    (*state).buffer[(1 as i32 + (i + 30 as i32) % 55 as i32) as usize],
                ) as i64;
            if t < 0 as i32 as i64 {
                t += 1000000000 as i32 as i64;
            }
            (*state).buffer[i as usize] = t as u64;
            i += 1;
            i;
        }
        i1 += 1;
        i1;
    }
    (*state).x = 0 as i32 as u32;
    (*state).y = 31 as i32 as u32;
}
static mut ran3_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ran3\0" as *const u8 as *const i8,
        max: 1000000000 as i32 as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ran3_state_t>() as u64,
        set: Some(ran3_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ran3_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ran3_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ran3: *const gsl_rng_type = unsafe {
    &ran3_type as *const gsl_rng_type
};