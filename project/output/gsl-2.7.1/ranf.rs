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
    fn ldexp(_: libc::c_double, _: i32) -> libc::c_double;
}
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
pub struct ranf_state_t {
    pub x0: libc::c_ushort,
    pub x1: libc::c_ushort,
    pub x2: libc::c_ushort,
}
static mut a0: libc::c_ushort = 0xb175 as i32 as libc::c_ushort;
static mut a1: libc::c_ushort = 0xa2e7 as i32 as libc::c_ushort;
static mut a2: libc::c_ushort = 0x2875 as i32 as libc::c_ushort;
#[inline]
unsafe extern "C" fn ranf_advance(mut vstate: *mut libc::c_void) {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    let x0: u64 = (*state).x0 as u64;
    let x1: u64 = (*state).x1 as u64;
    let x2: u64 = (*state).x2 as u64;
    let mut r: u64 = 0;
    r = (a0 as u64).wrapping_mul(x0);
    (*state).x0 = (r & 0xffff as i32 as u64) as libc::c_ushort;
    r >>= 16 as i32;
    r = r
        .wrapping_add(
            (a0 as u64).wrapping_mul(x1).wrapping_add((a1 as u64).wrapping_mul(x0)),
        );
    (*state).x1 = (r & 0xffff as i32 as u64) as libc::c_ushort;
    r >>= 16 as i32;
    r = r
        .wrapping_add(
            (a0 as u64)
                .wrapping_mul(x2)
                .wrapping_add((a1 as u64).wrapping_mul(x1))
                .wrapping_add((a2 as u64).wrapping_mul(x0)),
        );
    (*state).x2 = (r & 0xffff as i32 as u64) as libc::c_ushort;
}
unsafe extern "C" fn ranf_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut x1: u64 = 0;
    let mut x2: u64 = 0;
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    ranf_advance(state as *mut libc::c_void);
    x1 = (*state).x1 as u64;
    x2 = (*state).x2 as u64;
    return (x2 << 16 as i32).wrapping_add(x1);
}
unsafe extern "C" fn ranf_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    ranf_advance(state as *mut libc::c_void);
    return ldexp((*state).x2 as libc::c_double, -(16 as i32))
        + ldexp((*state).x1 as libc::c_double, -(32 as i32))
        + ldexp((*state).x0 as libc::c_double, -(48 as i32));
}
unsafe extern "C" fn ranf_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ranf_state_t = vstate as *mut ranf_state_t;
    let mut x0: libc::c_ushort = 0;
    let mut x1: libc::c_ushort = 0;
    let mut x2: libc::c_ushort = 0;
    let mut r: u64 = 0;
    let mut b0: u64 = 0xd6dd as i32 as u64;
    let mut b1: u64 = 0xb894 as i32 as u64;
    let mut b2: u64 = 0x5cee as i32 as u64;
    if s == 0 as i32 as u64 {
        x0 = 0x9cd1 as i32 as libc::c_ushort;
        x1 = 0x53fc as i32 as libc::c_ushort;
        x2 = 0x9482 as i32 as libc::c_ushort;
    } else {
        x0 = ((s | 1 as i32 as u64) & 0xffff as i32 as u64) as libc::c_ushort;
        x1 = (s >> 16 as i32 & 0xffff as i32 as u64) as libc::c_ushort;
        x2 = 0 as i32 as libc::c_ushort;
    }
    r = b0.wrapping_mul(x0 as u64);
    (*state).x0 = (r & 0xffff as i32 as u64) as libc::c_ushort;
    r >>= 16 as i32;
    r = r
        .wrapping_add(
            b0.wrapping_mul(x1 as u64).wrapping_add(b1.wrapping_mul(x0 as u64)),
        );
    (*state).x1 = (r & 0xffff as i32 as u64) as libc::c_ushort;
    r >>= 16 as i32;
    r = r
        .wrapping_add(
            b0
                .wrapping_mul(x2 as u64)
                .wrapping_add(b1.wrapping_mul(x1 as u64))
                .wrapping_add(b2.wrapping_mul(x0 as u64)),
        );
    (*state).x2 = (r & 0xffff as i32 as u64) as libc::c_ushort;
}
static mut ranf_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranf\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranf_state_t>() as u64,
        set: Some(ranf_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranf_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranf_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ranf: *const gsl_rng_type = unsafe {
    &ranf_type as *const gsl_rng_type
};