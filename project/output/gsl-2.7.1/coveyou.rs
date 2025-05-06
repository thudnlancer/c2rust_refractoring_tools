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
pub struct ran_state_t {
    pub x: u64,
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    (*state).x = ((*state).x).wrapping_mul(((*state).x).wrapping_add(1 as i32 as u64))
        & 0xffffffff as u64;
    return (*state).x;
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 4294967296.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut diff: u64 = s
        .wrapping_rem(4 as u64)
        .wrapping_sub(2 as u64)
        .wrapping_rem(0xffffffff as u64);
    if diff != 0 {
        (*state).x = s.wrapping_sub(diff) & 0xffffffff as u64;
    } else {
        (*state).x = s & 0xffffffff as u64;
    };
}
static mut ran_type: gsl_rng_type = gsl_rng_type {
    name: 0 as *const i8,
    max: 0,
    min: 0,
    size: 0,
    set: None,
    get: None,
    get_double: None,
};
#[no_mangle]
pub static mut gsl_rng_coveyou: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
unsafe extern "C" fn run_static_initializers() {
    ran_type = {
        let mut init = gsl_rng_type {
            name: b"coveyou\0" as *const u8 as *const i8,
            max: (0xffffffff as u64).wrapping_sub(1 as i32 as u64),
            min: 2 as i32 as u64,
            size: ::core::mem::size_of::<ran_state_t>() as u64,
            set: Some(ran_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
            get: Some(ran_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
            get_double: Some(
                ran_get_double
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
            ),
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];