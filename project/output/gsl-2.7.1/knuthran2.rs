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
    pub x0: u64,
    pub x1: u64,
}
#[inline]
unsafe extern "C" fn schrage(mut a: u64, mut b: u64, mut m: u64) -> u64 {
    let mut q: u64 = 0;
    let mut t: u64 = 0;
    if a == 0 as u64 {
        return 0 as u64;
    }
    q = m.wrapping_div(a);
    t = (2 as i32 as u64)
        .wrapping_mul(m)
        .wrapping_sub(m.wrapping_rem(a).wrapping_mul(b.wrapping_div(q)));
    if t >= m {
        t = t.wrapping_sub(m);
    }
    t = t.wrapping_add(a.wrapping_mul(b.wrapping_rem(q)));
    return if t >= m { t.wrapping_sub(m) } else { t };
}
#[inline]
unsafe extern "C" fn schrage_mult(
    mut a: u64,
    mut b: u64,
    mut m: u64,
    mut sqrtm: u64,
) -> u64 {
    let mut t0: u64 = schrage(sqrtm, b, m);
    let mut t1: u64 = schrage(a.wrapping_div(sqrtm), t0, m);
    let mut t2: u64 = schrage(a.wrapping_rem(sqrtm), b, m);
    let mut t: u64 = t1.wrapping_add(t2);
    return if t >= m { t.wrapping_sub(m) } else { t };
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let xtmp: u64 = (*state).x1;
    (*state).x1 = (schrage_mult(
        271828183 as u64,
        (*state).x1,
        0x7fffffff as u64,
        46341 as u64,
    ))
        .wrapping_add(
            schrage_mult(1833324378 as u64, (*state).x0, 0x7fffffff as u64, 46341 as u64),
        );
    if (*state).x1 >= 0x7fffffff as u64 {
        (*state).x1 = ((*state).x1).wrapping_sub(0x7fffffff as u64);
    }
    (*state).x0 = xtmp;
    return (*state).x1;
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 2147483647.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    if s.wrapping_rem(0x7fffffff as u64) == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    (*state).x0 = s.wrapping_rem(0x7fffffff as u64);
    (*state).x1 = s.wrapping_rem(0x7fffffff as u64);
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
pub static mut gsl_rng_knuthran2: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};
unsafe extern "C" fn run_static_initializers() {
    ran_type = {
        let mut init = gsl_rng_type {
            name: b"knuthran2\0" as *const u8 as *const i8,
            max: (0x7fffffff as u64).wrapping_sub(1 as i64 as u64),
            min: 0 as i32 as u64,
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