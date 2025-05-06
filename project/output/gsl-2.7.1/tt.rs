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
pub struct tt_state_t {
    pub n: i32,
    pub x: [u64; 25],
}
#[inline]
unsafe extern "C" fn tt_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut tt_state_t = vstate as *mut tt_state_t;
    let mag01: [u64; 2] = [0 as i32 as u64, 0x8ebfd028 as u64];
    let mut y: u64 = 0;
    let x: *mut u64 = ((*state).x).as_mut_ptr();
    let mut n: i32 = (*state).n;
    if n >= 25 as i32 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 25 as i32 - 7 as i32 {
            *x.offset(i as isize) = *x.offset((i + 7 as i32) as isize)
                ^ *x.offset(i as isize) >> 1 as i32
                ^ mag01[(*x.offset(i as isize)).wrapping_rem(2 as i32 as u64) as usize];
            i += 1;
            i;
        }
        while i < 25 as i32 {
            *x.offset(i as isize) = *x.offset((i + (7 as i32 - 25 as i32)) as isize)
                ^ *x.offset(i as isize) >> 1 as i32
                ^ mag01[(*x.offset(i as isize)).wrapping_rem(2 as i32 as u64) as usize];
            i += 1;
            i;
        }
        n = 0 as i32;
    }
    y = *x.offset(n as isize);
    y ^= y << 7 as i32 & 0x2b5b2500 as u64;
    y ^= y << 15 as i32 & 0xdb8b0000 as u64;
    y &= 0xffffffff as u64;
    y ^= y >> 16 as i32;
    (*state).n = n + 1 as i32;
    return y;
}
unsafe extern "C" fn tt_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return tt_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn tt_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut tt_state_t = vstate as *mut tt_state_t;
    let init_state: tt_state_t = {
        let mut init = tt_state_t {
            n: 0 as i32,
            x: [
                0x95f24dab as u64,
                0xb685215 as u64,
                0xe76ccae7 as u64,
                0xaf3ec239 as u64,
                0x715fad23 as u64,
                0x24a590ad as u64,
                0x69e4b5ef as u64,
                0xbf456141 as u64,
                0x96bc1b7b as u64,
                0xa7bdf825 as u64,
                0xc1de75b7 as u64,
                0x8858a9c9 as u64,
                0x2da87693 as u64,
                0xb657f9dd as u64,
                0xffdc8a9f as u64,
                0x8121da71 as u64,
                0x8b823ecb as u64,
                0x885d05f5 as u64,
                0x4e20cd47 as u64,
                0x5a9ad5d9 as u64,
                0x512c0c03 as u64,
                0xea857ccd as u64,
                0x4cc1d30f as u64,
                0x8891a8a1 as u64,
                0xa6b7aadb as u64,
            ],
        };
        init
    };
    if s == 0 as i32 as u64 {
        *state = init_state;
    } else {
        let mut i: i32 = 0;
        (*state).n = 0 as i32;
        (*state).x[0 as i32 as usize] = s & 0xffffffff as u64;
        i = 1 as i32;
        while i < 25 as i32 {
            (*state).x[i as usize] = (69069 as i32 as u64)
                .wrapping_mul((*state).x[(i - 1 as i32) as usize]) & 0xffffffff as u64;
            i += 1;
            i;
        }
    };
}
static mut tt_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"tt800\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<tt_state_t>() as u64,
        set: Some(tt_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(tt_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            tt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_tt800: *const gsl_rng_type = unsafe {
    &tt_type as *const gsl_rng_type
};