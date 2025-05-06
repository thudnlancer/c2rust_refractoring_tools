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
pub struct zuf_state_t {
    pub n: i32,
    pub u: [u64; 607],
}
static mut zuf_randmax: u64 = 16777216 as i32 as u64;
#[inline]
unsafe extern "C" fn zuf_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut zuf_state_t = vstate as *mut zuf_state_t;
    let n: i32 = (*state).n;
    let m: i32 = (n - 273 as i32 + 607 as i32) % 607 as i32;
    let mut t: u64 = ((*state).u[n as usize]).wrapping_add((*state).u[m as usize]);
    while t > zuf_randmax {
        t = t.wrapping_sub(zuf_randmax);
    }
    (*state).u[n as usize] = t;
    if n == 606 as i32 {
        (*state).n = 0 as i32;
    } else {
        (*state).n = n + 1 as i32;
    }
    return t;
}
unsafe extern "C" fn zuf_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return zuf_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn zuf_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut kl: i64 = 9373 as i32 as i64;
    let mut ij: i64 = 1802 as i32 as i64;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut k: i64 = 0;
    let mut l: i64 = 0;
    let mut m: i64 = 0;
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut ii: i64 = 0;
    let mut jj: i64 = 0;
    let mut state: *mut zuf_state_t = vstate as *mut zuf_state_t;
    (*state).n = 0 as i32;
    if s == 0 as i32 as u64 {
        s = 1802 as i32 as u64;
    }
    ij = s as i64;
    i = ij / 177 as i32 as i64 % 177 as i32 as i64 + 2 as i32 as i64;
    j = ij % 177 as i32 as i64 + 2 as i32 as i64;
    k = kl / 169 as i32 as i64 % 178 as i32 as i64 + 1 as i32 as i64;
    l = kl % 169 as i32 as i64;
    ii = 0 as i32 as i64;
    while ii < 607 as i32 as i64 {
        x = 0.0f64;
        y = 0.5f64;
        jj = 1 as i32 as i64;
        while jj <= 24 as i32 as i64 {
            m = i * j % 179 as i32 as i64 * k % 179 as i32 as i64;
            i = j;
            j = k;
            k = m;
            l = (l * 53 as i32 as i64 + 1 as i32 as i64) % 169 as i32 as i64;
            if l * m % 64 as i32 as i64 >= 32 as i32 as i64 {
                x += y;
            }
            y *= 0.5f64;
            jj += 1;
            jj;
        }
        (*state).u[ii as usize] = (x * zuf_randmax as libc::c_double) as u64;
        ii += 1;
        ii;
    }
}
static mut zuf_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"zuf\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<zuf_state_t>() as u64,
        set: Some(zuf_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(zuf_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            zuf_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_zuf: *const gsl_rng_type = unsafe {
    &zuf_type as *const gsl_rng_type
};