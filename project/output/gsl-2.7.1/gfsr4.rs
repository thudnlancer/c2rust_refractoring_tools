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
pub struct gfsr4_state_t {
    pub nd: i32,
    pub ra: [u64; 16384],
}
#[inline]
unsafe extern "C" fn gfsr4_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut gfsr4_state_t = vstate as *mut gfsr4_state_t;
    (*state).nd = (*state).nd + 1 as i32 & 16383 as i32;
    (*state).ra[(*state).nd as usize] = (*state)
        .ra[((*state).nd + (16383 as i32 + 1 as i32 - 471 as i32) & 16383 as i32)
        as usize]
        ^ (*state)
            .ra[((*state).nd + (16383 as i32 + 1 as i32 - 1586 as i32) & 16383 as i32)
            as usize]
        ^ (*state)
            .ra[((*state).nd + (16383 as i32 + 1 as i32 - 6988 as i32) & 16383 as i32)
            as usize]
        ^ (*state)
            .ra[((*state).nd + (16383 as i32 + 1 as i32 - 9689 as i32) & 16383 as i32)
            as usize];
    return (*state).ra[(*state).nd as usize];
}
unsafe extern "C" fn gfsr4_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return gfsr4_get(vstate) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn gfsr4_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut gfsr4_state_t = vstate as *mut gfsr4_state_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut msb: u64 = 0x80000000 as u64;
    let mut mask: u64 = 0xffffffff as u64;
    if s == 0 as i32 as u64 {
        s = 4357 as i32 as u64;
    }
    i = 0 as i32;
    while i <= 16383 as i32 {
        let mut t: u64 = 0 as i32 as u64;
        let mut bit: u64 = msb;
        j = 0 as i32;
        while j < 32 as i32 {
            s = (69069 as i32 as u64).wrapping_mul(s) & 0xffffffff as u64;
            if s & msb != 0 {
                t |= bit;
            }
            bit >>= 1 as i32;
            j += 1;
            j;
        }
        (*state).ra[i as usize] = t;
        i += 1;
        i;
    }
    i = 0 as i32;
    while i < 32 as i32 {
        let mut k: i32 = 7 as i32 + i * 3 as i32;
        (*state).ra[k as usize] &= mask;
        (*state).ra[k as usize] |= msb;
        mask >>= 1 as i32;
        msb >>= 1 as i32;
        i += 1;
        i;
    }
    (*state).nd = i;
}
static mut gfsr4_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"gfsr4\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<gfsr4_state_t>() as u64,
        set: Some(gfsr4_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(gfsr4_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            gfsr4_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_gfsr4: *const gsl_rng_type = unsafe {
    &gfsr4_type as *const gsl_rng_type
};