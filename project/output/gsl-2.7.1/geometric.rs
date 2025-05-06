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
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub struct gsl_rng {
    pub type_0: *const gsl_rng_type,
    pub state: *mut libc::c_void,
}
#[inline]
unsafe extern "C" fn gsl_rng_uniform_pos(mut r: *const gsl_rng) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    loop {
        x = ((*(*r).type_0).get_double).expect("non-null function pointer")((*r).state);
        if !(x == 0 as i32 as libc::c_double) {
            break;
        }
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_geometric(
    mut r: *const gsl_rng,
    p: libc::c_double,
) -> u32 {
    let mut u: libc::c_double = gsl_rng_uniform_pos(r);
    let mut k: u32 = 0;
    if p == 1 as i32 as libc::c_double {
        k = 1 as i32 as u32;
    } else {
        k = (log(u) / log(1 as i32 as libc::c_double - p) + 1 as i32 as libc::c_double)
            as u32;
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ran_geometric_pdf(
    k: u32,
    p: libc::c_double,
) -> libc::c_double {
    if k == 0 as i32 as u32 {
        return 0 as i32 as libc::c_double
    } else if k == 1 as i32 as u32 {
        return p
    } else {
        let mut P: libc::c_double = p
            * pow(1 as i32 as libc::c_double - p, k as libc::c_double - 1.0f64);
        return P;
    };
}