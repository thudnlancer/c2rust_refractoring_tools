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
pub struct ranmar_state_t {
    pub i: u32,
    pub j: u32,
    pub carry: i64,
    pub u: [u64; 97],
}
static mut two24: u64 = 16777216 as i32 as u64;
#[inline]
unsafe extern "C" fn ranmar_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ranmar_state_t = vstate as *mut ranmar_state_t;
    let mut i: u32 = (*state).i;
    let mut j: u32 = (*state).j;
    let mut carry: i64 = (*state).carry;
    let mut delta: i64 = ((*state).u[i as usize]).wrapping_sub((*state).u[j as usize])
        as i64;
    if delta < 0 as i32 as i64 {
        delta = (delta as u64).wrapping_add(two24) as i64 as i64;
    }
    (*state).u[i as usize] = delta as u64;
    if i == 0 as i32 as u32 {
        i = 96 as i32 as u32;
    } else {
        i = i.wrapping_sub(1);
        i;
    }
    (*state).i = i;
    if j == 0 as i32 as u32 {
        j = 96 as i32 as u32;
    } else {
        j = j.wrapping_sub(1);
        j;
    }
    (*state).j = j;
    carry += -(7654321 as i32) as i64;
    if carry < 0 as i32 as i64 {
        carry = (carry as u64).wrapping_add(two24.wrapping_sub(3 as i32 as u64)) as i64
            as i64;
    }
    (*state).carry = carry;
    delta += -carry;
    if delta < 0 as i32 as i64 {
        delta = (delta as u64).wrapping_add(two24) as i64 as i64;
    }
    return delta as u64;
}
unsafe extern "C" fn ranmar_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    return ranmar_get(vstate) as libc::c_double / 16777216.0f64;
}
unsafe extern "C" fn ranmar_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ranmar_state_t = vstate as *mut ranmar_state_t;
    let mut ij: u64 = s.wrapping_div(30082 as i32 as u64);
    let mut kl: u64 = s.wrapping_rem(30082 as i32 as u64);
    let mut i: i32 = ij
        .wrapping_div(177 as i32 as u64)
        .wrapping_rem(177 as i32 as u64)
        .wrapping_add(2 as i32 as u64) as i32;
    let mut j: i32 = ij.wrapping_rem(177 as i32 as u64).wrapping_add(2 as i32 as u64)
        as i32;
    let mut k: i32 = kl
        .wrapping_div(169 as i32 as u64)
        .wrapping_rem(178 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as i32;
    let mut l: i32 = kl.wrapping_rem(169 as i32 as u64) as i32;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    a = 0 as i32;
    while a < 97 as i32 {
        let mut sum: u64 = 0 as i32 as u64;
        let mut t: u64 = two24;
        b = 0 as i32;
        while b < 24 as i32 {
            let mut m: u64 = (i * j % 179 as i32 * k % 179 as i32) as u64;
            i = j;
            j = k;
            k = m as i32;
            l = (53 as i32 * l + 1 as i32) % 169 as i32;
            t >>= 1 as i32;
            if (l as u64).wrapping_mul(m).wrapping_rem(64 as i32 as u64)
                >= 32 as i32 as u64
            {
                sum = sum.wrapping_add(t);
            }
            b += 1;
            b;
        }
        (*state).u[a as usize] = sum;
        a += 1;
        a;
    }
    (*state).i = 96 as i32 as u32;
    (*state).j = 32 as i32 as u32;
    (*state).carry = 362436 as i32 as i64;
}
static mut ranmar_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranmar\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranmar_state_t>() as u64,
        set: Some(ranmar_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranmar_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranmar_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ranmar: *const gsl_rng_type = unsafe {
    &ranmar_type as *const gsl_rng_type
};