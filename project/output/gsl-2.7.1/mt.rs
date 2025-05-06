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
pub struct mt_state_t {
    pub mt: [u64; 624],
    pub mti: i32,
}
static mut UPPER_MASK: u64 = 0x80000000 as u64;
static mut LOWER_MASK: u64 = 0x7fffffff as u64;
#[inline]
unsafe extern "C" fn mt_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut k: u64 = 0;
    let mt: *mut u64 = ((*state).mt).as_mut_ptr();
    if (*state).mti >= 624 as i32 {
        let mut kk: i32 = 0;
        kk = 0 as i32;
        while kk < 624 as i32 - 397 as i32 {
            let mut y: u64 = *mt.offset(kk as isize) & UPPER_MASK
                | *mt.offset((kk + 1 as i32) as isize) & LOWER_MASK;
            *mt.offset(kk as isize) = *mt.offset((kk + 397 as i32) as isize)
                ^ y >> 1 as i32
                ^ (if y & 0x1 as i32 as u64 != 0 {
                    0x9908b0df as u64
                } else {
                    0 as i32 as u64
                });
            kk += 1;
            kk;
        }
        while kk < 624 as i32 - 1 as i32 {
            let mut y_0: u64 = *mt.offset(kk as isize) & UPPER_MASK
                | *mt.offset((kk + 1 as i32) as isize) & LOWER_MASK;
            *mt.offset(kk as isize) = *mt
                .offset((kk + (397 as i32 - 624 as i32)) as isize) ^ y_0 >> 1 as i32
                ^ (if y_0 & 0x1 as i32 as u64 != 0 {
                    0x9908b0df as u64
                } else {
                    0 as i32 as u64
                });
            kk += 1;
            kk;
        }
        let mut y_1: u64 = *mt.offset((624 as i32 - 1 as i32) as isize) & UPPER_MASK
            | *mt.offset(0 as i32 as isize) & LOWER_MASK;
        *mt.offset((624 as i32 - 1 as i32) as isize) = *mt
            .offset((397 as i32 - 1 as i32) as isize) ^ y_1 >> 1 as i32
            ^ (if y_1 & 0x1 as i32 as u64 != 0 {
                0x9908b0df as u64
            } else {
                0 as i32 as u64
            });
        (*state).mti = 0 as i32;
    }
    k = *mt.offset((*state).mti as isize);
    k ^= k >> 11 as i32;
    k ^= k << 7 as i32 & 0x9d2c5680 as u64;
    k ^= k << 15 as i32 & 0xefc60000 as u64;
    k ^= k >> 18 as i32;
    (*state).mti += 1;
    (*state).mti;
    return k;
}
unsafe extern "C" fn mt_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return mt_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn mt_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 4357 as i32 as u64;
    }
    (*state).mt[0 as i32 as usize] = s & 0xffffffff as u64;
    i = 1 as i32;
    while i < 624 as i32 {
        (*state).mt[i as usize] = (1812433253 as u64)
            .wrapping_mul(
                (*state).mt[(i - 1 as i32) as usize]
                    ^ (*state).mt[(i - 1 as i32) as usize] >> 30 as i32,
            )
            .wrapping_add(i as u64);
        (*state).mt[i as usize] &= 0xffffffff as u64;
        i += 1;
        i;
    }
    (*state).mti = i;
}
unsafe extern "C" fn mt_1999_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 4357 as i32 as u64;
    }
    i = 0 as i32;
    while i < 624 as i32 {
        (*state).mt[i as usize] = s & 0xffff0000 as u64;
        s = (69069 as i32 as u64).wrapping_mul(s).wrapping_add(1 as i32 as u64)
            & 0xffffffff as u64;
        (*state).mt[i as usize] |= (s & 0xffff0000 as u64) >> 16 as i32;
        s = (69069 as i32 as u64).wrapping_mul(s).wrapping_add(1 as i32 as u64)
            & 0xffffffff as u64;
        i += 1;
        i;
    }
    (*state).mti = i;
}
unsafe extern "C" fn mt_1998_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut mt_state_t = vstate as *mut mt_state_t;
    let mut i: i32 = 0;
    if s == 0 as i32 as u64 {
        s = 4357 as i32 as u64;
    }
    (*state).mt[0 as i32 as usize] = s & 0xffffffff as u64;
    i = 1 as i32;
    while i < 624 as i32 {
        (*state).mt[i as usize] = (69069 as i32 as u64)
            .wrapping_mul((*state).mt[(i - 1 as i32) as usize]) & 0xffffffff as u64;
        i += 1;
        i;
    }
    (*state).mti = i;
}
static mut mt_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<mt_state_t>() as u64,
        set: Some(mt_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut mt_1999_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937_1999\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<mt_state_t>() as u64,
        set: Some(mt_1999_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut mt_1998_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"mt19937_1998\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<mt_state_t>() as u64,
        set: Some(mt_1998_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(mt_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            mt_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_mt19937: *const gsl_rng_type = unsafe {
    &mt_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_mt19937_1999: *const gsl_rng_type = unsafe {
    &mt_1999_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_mt19937_1998: *const gsl_rng_type = unsafe {
    &mt_1998_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_default: *const gsl_rng_type = unsafe {
    &mt_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_default_seed: u64 = 0 as i32 as u64;