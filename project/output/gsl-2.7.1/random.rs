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
pub struct random128_state_t {
    pub i: i32,
    pub j: i32,
    pub x: [i64; 31],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random256_state_t {
    pub i: i32,
    pub j: i32,
    pub x: [i64; 63],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random32_state_t {
    pub i: i32,
    pub j: i32,
    pub x: [i64; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random64_state_t {
    pub i: i32,
    pub j: i32,
    pub x: [i64; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random8_state_t {
    pub x: i64,
}
#[inline]
unsafe extern "C" fn random8_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    (*state).x = ((1103515245 as i32 as i64 * (*state).x + 12345 as i32 as i64) as u64
        & 0x7fffffff as u64) as i64;
    return (*state).x as u64;
}
#[inline]
unsafe extern "C" fn random_get(
    mut i: *mut i32,
    mut j: *mut i32,
    mut n: i32,
    mut x: *mut i64,
) -> i64 {
    let mut k: i64 = 0;
    *x.offset(*i as isize) += *x.offset(*j as isize);
    k = *x.offset(*i as isize) >> 1 as i32 & 0x7fffffff as i32 as i64;
    *i += 1;
    *i;
    if *i == n {
        *i = 0 as i32;
    }
    *j += 1;
    *j;
    if *j == n {
        *j = 0 as i32;
    }
    return k;
}
#[inline]
unsafe extern "C" fn random32_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut k: u64 = random_get(
        &mut (*state).i,
        &mut (*state).j,
        7 as i32,
        ((*state).x).as_mut_ptr(),
    ) as u64;
    return k;
}
#[inline]
unsafe extern "C" fn random64_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut k: i64 = random_get(
        &mut (*state).i,
        &mut (*state).j,
        15 as i32,
        ((*state).x).as_mut_ptr(),
    );
    return k as u64;
}
#[inline]
unsafe extern "C" fn random128_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut k: u64 = random_get(
        &mut (*state).i,
        &mut (*state).j,
        31 as i32,
        ((*state).x).as_mut_ptr(),
    ) as u64;
    return k;
}
#[inline]
unsafe extern "C" fn random256_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut k: i64 = random_get(
        &mut (*state).i,
        &mut (*state).j,
        63 as i32,
        ((*state).x).as_mut_ptr(),
    );
    return k as u64;
}
unsafe extern "C" fn random8_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random8_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random32_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random32_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random64_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random64_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random128_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random128_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random256_get_double(
    mut vstate: *mut libc::c_void,
) -> libc::c_double {
    return random256_get(vstate) as libc::c_double / 2147483648.0f64;
}
unsafe extern "C" fn random8_bsd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x = s as i64;
}
unsafe extern "C" fn random32_bsd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: i32 = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 7 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 7 as i32 {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_bsd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: i32 = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 15 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 15 as i32 {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_bsd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: i32 = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 31 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 31 as i32 {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_bsd_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: i32 = 0;
    bsd_initialize(((*state).x).as_mut_ptr(), 63 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 63 as i32 {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn bsd_initialize(mut x: *mut i64, mut n: i32, mut s: u64) {
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    *x.offset(0 as i32 as isize) = s as i64;
    i = 1 as i32;
    while i < n {
        *x.offset(i as isize) = 1103515245 as i32 as i64
            * *x.offset((i - 1 as i32) as isize) + 12345 as i32 as i64;
        i += 1;
        i;
    }
}
unsafe extern "C" fn libc5_initialize(mut x: *mut i64, mut n: i32, mut s: u64) {
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    *x.offset(0 as i32 as isize) = s as i64;
    i = 1 as i32;
    while i < n {
        *x.offset(i as isize) = 1103515145 as i32 as i64
            * *x.offset((i - 1 as i32) as isize) + 12345 as i32 as i64;
        i += 1;
        i;
    }
}
unsafe extern "C" fn glibc2_initialize(mut x: *mut i64, mut n: i32, mut s: u64) {
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    *x.offset(0 as i32 as isize) = s as i64;
    i = 1 as i32;
    while i < n {
        let h: i64 = s.wrapping_div(127773 as i32 as u64) as i64;
        let t: i64 = (16807 as i32 as u64)
            .wrapping_mul(s.wrapping_sub((h * 127773 as i32 as i64) as u64))
            .wrapping_sub((h * 2836 as i32 as i64) as u64) as i64;
        if t < 0 as i32 as i64 {
            s = (t + 2147483647 as i32 as i64) as u64;
        } else {
            s = t as u64;
        }
        *x.offset(i as isize) = s as i64;
        i += 1;
        i;
    }
}
unsafe extern "C" fn random8_glibc2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x = s as i64;
}
unsafe extern "C" fn random32_glibc2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: i32 = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 7 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 7 as i32 {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_glibc2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: i32 = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 15 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 15 as i32 {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_glibc2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: i32 = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 31 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 31 as i32 {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_glibc2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: i32 = 0;
    glibc2_initialize(((*state).x).as_mut_ptr(), 63 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 63 as i32 {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random8_libc5_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random8_state_t = vstate as *mut random8_state_t;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x = s as i64;
}
unsafe extern "C" fn random32_libc5_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random32_state_t = vstate as *mut random32_state_t;
    let mut i: i32 = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 7 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 7 as i32 {
        random32_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random64_libc5_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random64_state_t = vstate as *mut random64_state_t;
    let mut i: i32 = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 15 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 15 as i32 {
        random64_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random128_libc5_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random128_state_t = vstate as *mut random128_state_t;
    let mut i: i32 = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 31 as i32, s);
    (*state).i = 3 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 31 as i32 {
        random128_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
unsafe extern "C" fn random256_libc5_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut random256_state_t = vstate as *mut random256_state_t;
    let mut i: i32 = 0;
    libc5_initialize(((*state).x).as_mut_ptr(), 63 as i32, s);
    (*state).i = 1 as i32;
    (*state).j = 0 as i32;
    i = 0 as i32;
    while i < 10 as i32 * 63 as i32 {
        random256_get(state as *mut libc::c_void);
        i += 1;
        i;
    }
}
static mut random_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random8_state_t>() as u64,
        set: Some(
            random8_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random8_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random32_state_t>() as u64,
        set: Some(
            random32_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random32_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random64_state_t>() as u64,
        set: Some(
            random64_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random64_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_glibc2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-glibc2\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random256_state_t>() as u64,
        set: Some(
            random256_glibc2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random256_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random8_state_t>() as u64,
        set: Some(
            random8_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random8_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random32_state_t>() as u64,
        set: Some(
            random32_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random32_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random64_state_t>() as u64,
        set: Some(
            random64_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random64_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_libc5_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-libc5\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random256_state_t>() as u64,
        set: Some(
            random256_libc5_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random256_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random8_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random8-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random8_state_t>() as u64,
        set: Some(random8_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(random8_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random8_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random32_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random32-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random32_state_t>() as u64,
        set: Some(
            random32_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random32_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random32_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random64_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random64-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random64_state_t>() as u64,
        set: Some(
            random64_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random64_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random64_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random128_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random128-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random128_state_t>() as u64,
        set: Some(
            random128_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random128_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random128_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut random256_bsd_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"random256-bsd\0" as *const u8 as *const i8,
        max: 0x7fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<random256_state_t>() as u64,
        set: Some(
            random256_bsd_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> (),
        ),
        get: Some(random256_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            random256_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_random_libc5: *const gsl_rng_type = unsafe {
    &random_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random8_libc5: *const gsl_rng_type = unsafe {
    &random8_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random32_libc5: *const gsl_rng_type = unsafe {
    &random32_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random64_libc5: *const gsl_rng_type = unsafe {
    &random64_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random128_libc5: *const gsl_rng_type = unsafe {
    &random128_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random256_libc5: *const gsl_rng_type = unsafe {
    &random256_libc5_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random_glibc2: *const gsl_rng_type = unsafe {
    &random_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random8_glibc2: *const gsl_rng_type = unsafe {
    &random8_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random32_glibc2: *const gsl_rng_type = unsafe {
    &random32_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random64_glibc2: *const gsl_rng_type = unsafe {
    &random64_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random128_glibc2: *const gsl_rng_type = unsafe {
    &random128_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random256_glibc2: *const gsl_rng_type = unsafe {
    &random256_glibc2_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random_bsd: *const gsl_rng_type = unsafe {
    &random_bsd_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random8_bsd: *const gsl_rng_type = unsafe {
    &random8_bsd_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random32_bsd: *const gsl_rng_type = unsafe {
    &random32_bsd_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random64_bsd: *const gsl_rng_type = unsafe {
    &random64_bsd_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random128_bsd: *const gsl_rng_type = unsafe {
    &random128_bsd_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_random256_bsd: *const gsl_rng_type = unsafe {
    &random256_bsd_type as *const gsl_rng_type
};