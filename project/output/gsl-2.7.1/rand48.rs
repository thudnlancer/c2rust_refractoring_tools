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
pub struct rand48_state_t {
    pub x0: libc::c_ushort,
    pub x1: libc::c_ushort,
    pub x2: libc::c_ushort,
}
static mut a0: libc::c_ushort = 0xe66d as i32 as libc::c_ushort;
static mut a1: libc::c_ushort = 0xdeec as i32 as libc::c_ushort;
static mut a2: libc::c_ushort = 0x5 as i32 as libc::c_ushort;
static mut c0: libc::c_ushort = 0xb as i32 as libc::c_ushort;
#[inline]
unsafe extern "C" fn rand48_advance(mut vstate: *mut libc::c_void) {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    let x0: u64 = (*state).x0 as u64;
    let x1: u64 = (*state).x1 as u64;
    let x2: u64 = (*state).x2 as u64;
    let mut a: u64 = 0;
    a = (a0 as u64).wrapping_mul(x0).wrapping_add(c0 as u64);
    (*state).x0 = (a & 0xffff as i32 as u64) as libc::c_ushort;
    a >>= 16 as i32;
    a = a
        .wrapping_add(
            (a0 as u64).wrapping_mul(x1).wrapping_add((a1 as u64).wrapping_mul(x0)),
        );
    (*state).x1 = (a & 0xffff as i32 as u64) as libc::c_ushort;
    a >>= 16 as i32;
    a = a
        .wrapping_add(
            (a0 as u64)
                .wrapping_mul(x2)
                .wrapping_add((a1 as u64).wrapping_mul(x1))
                .wrapping_add((a2 as u64).wrapping_mul(x0)),
        );
    (*state).x2 = (a & 0xffff as i32 as u64) as libc::c_ushort;
}
unsafe extern "C" fn rand48_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut x1: u64 = 0;
    let mut x2: u64 = 0;
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    rand48_advance(state as *mut libc::c_void);
    x2 = (*state).x2 as u64;
    x1 = (*state).x1 as u64;
    return (x2 << 16 as i32).wrapping_add(x1);
}
unsafe extern "C" fn rand48_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    rand48_advance(state as *mut libc::c_void);
    return ldexp((*state).x2 as libc::c_double, -(16 as i32))
        + ldexp((*state).x1 as libc::c_double, -(32 as i32))
        + ldexp((*state).x0 as libc::c_double, -(48 as i32));
}
unsafe extern "C" fn rand48_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut rand48_state_t = vstate as *mut rand48_state_t;
    if s == 0 as i32 as u64 {
        (*state).x0 = 0x330e as i32 as libc::c_ushort;
        (*state).x1 = 0xabcd as i32 as libc::c_ushort;
        (*state).x2 = 0x1234 as i32 as libc::c_ushort;
    } else {
        (*state).x0 = 0x330e as i32 as libc::c_ushort;
        (*state).x1 = (s & 0xffff as i32 as u64) as libc::c_ushort;
        (*state).x2 = (s >> 16 as i32 & 0xffff as i32 as u64) as libc::c_ushort;
    };
}
static mut rand48_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"rand48\0" as *const u8 as *const i8,
        max: 0xffffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<rand48_state_t>() as u64,
        set: Some(rand48_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(rand48_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            rand48_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_rand48: *const gsl_rng_type = unsafe {
    &rand48_type as *const gsl_rng_type
};