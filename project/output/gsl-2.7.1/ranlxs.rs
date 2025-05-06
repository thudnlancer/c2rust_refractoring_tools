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
pub struct ranlxs_state_t {
    pub xdbl: [libc::c_double; 12],
    pub ydbl: [libc::c_double; 12],
    pub carry: libc::c_double,
    pub xflt: [libc::c_float; 24],
    pub ir: u32,
    pub jr: u32,
    pub is: u32,
    pub is_old: u32,
    pub pr: u32,
}
static mut next: [i32; 12] = [
    1 as i32,
    2 as i32,
    3 as i32,
    4 as i32,
    5 as i32,
    6 as i32,
    7 as i32,
    8 as i32,
    9 as i32,
    10 as i32,
    11 as i32,
    0 as i32,
];
static mut snext: [i32; 24] = [
    1 as i32,
    2 as i32,
    3 as i32,
    4 as i32,
    5 as i32,
    6 as i32,
    7 as i32,
    8 as i32,
    9 as i32,
    10 as i32,
    11 as i32,
    12 as i32,
    13 as i32,
    14 as i32,
    15 as i32,
    16 as i32,
    17 as i32,
    18 as i32,
    19 as i32,
    20 as i32,
    21 as i32,
    22 as i32,
    23 as i32,
    0 as i32,
];
static mut sbase: libc::c_double = 16777216.0f64;
static mut sone_bit: libc::c_double = 1.0f64 / 16777216.0f64;
static mut one_bit: libc::c_double = 1.0f64 / 281474976710656.0f64;
static mut shift: libc::c_double = 268435456.0f64;
unsafe extern "C" fn increment_state(mut state: *mut ranlxs_state_t) {
    let mut k: i32 = 0;
    let mut kmax: i32 = 0;
    let mut m: i32 = 0;
    let mut x: libc::c_double = 0.;
    let mut y1: libc::c_double = 0.;
    let mut y2: libc::c_double = 0.;
    let mut y3: libc::c_double = 0.;
    let mut xflt: *mut libc::c_float = ((*state).xflt).as_mut_ptr();
    let mut xdbl: *mut libc::c_double = ((*state).xdbl).as_mut_ptr();
    let mut ydbl: *mut libc::c_double = ((*state).ydbl).as_mut_ptr();
    let mut carry: libc::c_double = (*state).carry;
    let mut ir: u32 = (*state).ir;
    let mut jr: u32 = (*state).jr;
    k = 0 as i32;
    while ir > 0 as i32 as u32 {
        y1 = *xdbl.offset(jr as isize) - *xdbl.offset(ir as isize);
        y2 = y1 - carry;
        if y2 < 0 as i32 as libc::c_double {
            carry = one_bit;
            y2 += 1 as i32 as libc::c_double;
        } else {
            carry = 0 as i32 as libc::c_double;
        }
        *xdbl.offset(ir as isize) = y2;
        ir = next[ir as usize] as u32;
        jr = next[jr as usize] as u32;
        k += 1;
        k;
    }
    kmax = ((*state).pr).wrapping_sub(12 as i32 as u32) as i32;
    while k <= kmax {
        y1 = *xdbl.offset(7 as i32 as isize) - *xdbl.offset(0 as i32 as isize);
        y1 -= carry;
        y2 = *xdbl.offset(8 as i32 as isize) - *xdbl.offset(1 as i32 as isize);
        if y1 < 0 as i32 as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(0 as i32 as isize) = y1;
        y3 = *xdbl.offset(9 as i32 as isize) - *xdbl.offset(2 as i32 as isize);
        if y2 < 0 as i32 as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(1 as i32 as isize) = y2;
        y1 = *xdbl.offset(10 as i32 as isize) - *xdbl.offset(3 as i32 as isize);
        if y3 < 0 as i32 as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(2 as i32 as isize) = y3;
        y2 = *xdbl.offset(11 as i32 as isize) - *xdbl.offset(4 as i32 as isize);
        if y1 < 0 as i32 as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(3 as i32 as isize) = y1;
        y3 = *xdbl.offset(0 as i32 as isize) - *xdbl.offset(5 as i32 as isize);
        if y2 < 0 as i32 as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(4 as i32 as isize) = y2;
        y1 = *xdbl.offset(1 as i32 as isize) - *xdbl.offset(6 as i32 as isize);
        if y3 < 0 as i32 as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(5 as i32 as isize) = y3;
        y2 = *xdbl.offset(2 as i32 as isize) - *xdbl.offset(7 as i32 as isize);
        if y1 < 0 as i32 as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(6 as i32 as isize) = y1;
        y3 = *xdbl.offset(3 as i32 as isize) - *xdbl.offset(8 as i32 as isize);
        if y2 < 0 as i32 as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(7 as i32 as isize) = y2;
        y1 = *xdbl.offset(4 as i32 as isize) - *xdbl.offset(9 as i32 as isize);
        if y3 < 0 as i32 as libc::c_double {
            y1 -= one_bit;
            y3 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(8 as i32 as isize) = y3;
        y2 = *xdbl.offset(5 as i32 as isize) - *xdbl.offset(10 as i32 as isize);
        if y1 < 0 as i32 as libc::c_double {
            y2 -= one_bit;
            y1 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(9 as i32 as isize) = y1;
        y3 = *xdbl.offset(6 as i32 as isize) - *xdbl.offset(11 as i32 as isize);
        if y2 < 0 as i32 as libc::c_double {
            y3 -= one_bit;
            y2 += 1 as i32 as libc::c_double;
        }
        *xdbl.offset(10 as i32 as isize) = y2;
        if y3 < 0 as i32 as libc::c_double {
            carry = one_bit;
            y3 += 1 as i32 as libc::c_double;
        } else {
            carry = 0 as i32 as libc::c_double;
        }
        *xdbl.offset(11 as i32 as isize) = y3;
        k += 12 as i32;
    }
    kmax = (*state).pr as i32;
    while k < kmax {
        y1 = *xdbl.offset(jr as isize) - *xdbl.offset(ir as isize);
        y2 = y1 - carry;
        if y2 < 0 as i32 as libc::c_double {
            carry = one_bit;
            y2 += 1 as i32 as libc::c_double;
        } else {
            carry = 0 as i32 as libc::c_double;
        }
        *xdbl.offset(ir as isize) = y2;
        *ydbl.offset(ir as isize) = y2 + shift;
        ir = next[ir as usize] as u32;
        jr = next[jr as usize] as u32;
        k += 1;
        k;
    }
    *ydbl.offset(ir as isize) = *xdbl.offset(ir as isize) + shift;
    k = next[ir as usize];
    while k > 0 as i32 {
        *ydbl.offset(k as isize) = *xdbl.offset(k as isize) + shift;
        k = next[k as usize];
    }
    k = 0 as i32;
    m = 0 as i32;
    while k < 12 as i32 {
        x = *xdbl.offset(k as isize);
        y2 = *ydbl.offset(k as isize) - shift;
        if y2 > x {
            y2 -= sone_bit;
        }
        y1 = (x - y2) * sbase;
        let fresh0 = m;
        m = m + 1;
        *xflt.offset(fresh0 as isize) = y1 as libc::c_float;
        let fresh1 = m;
        m = m + 1;
        *xflt.offset(fresh1 as isize) = y2 as libc::c_float;
        k += 1;
        k;
    }
    (*state).ir = ir;
    (*state).is = (2 as i32 as u32).wrapping_mul(ir);
    (*state).is_old = (2 as i32 as u32).wrapping_mul(ir);
    (*state).jr = jr;
    (*state).carry = carry;
}
#[inline]
unsafe extern "C" fn ranlxs_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ranlxs_state_t = vstate as *mut ranlxs_state_t;
    let is: u32 = snext[(*state).is as usize] as u32;
    (*state).is = is;
    if is == (*state).is_old {
        increment_state(state);
    }
    return (*state).xflt[(*state).is as usize] as libc::c_double;
}
unsafe extern "C" fn ranlxs_get(mut vstate: *mut libc::c_void) -> u64 {
    return (ranlxs_get_double(vstate) * 16777216.0f64) as u64;
}
unsafe extern "C" fn ranlxs_set_lux(
    mut vstate: *mut libc::c_void,
    mut s: u64,
    mut luxury: u32,
) {
    let mut state: *mut ranlxs_state_t = vstate as *mut ranlxs_state_t;
    let mut ibit: i32 = 0;
    let mut jbit: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut xbit: [i32; 31] = [0; 31];
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut seed: i64 = 0;
    if s == 0 as i32 as u64 {
        s = 1 as i32 as u64;
    }
    seed = s as i64;
    i = (seed as u64 & 0x7fffffff as u64) as i32;
    k = 0 as i32;
    while k < 31 as i32 {
        xbit[k as usize] = i % 2 as i32;
        i /= 2 as i32;
        k += 1;
        k;
    }
    ibit = 0 as i32;
    jbit = 18 as i32;
    k = 0 as i32;
    while k < 12 as i32 {
        x = 0 as i32 as libc::c_double;
        m = 1 as i32;
        while m <= 48 as i32 {
            y = xbit[ibit as usize] as libc::c_double;
            x += x + y;
            xbit[ibit as usize] = (xbit[ibit as usize] + xbit[jbit as usize]) % 2 as i32;
            ibit = (ibit + 1 as i32) % 31 as i32;
            jbit = (jbit + 1 as i32) % 31 as i32;
            m += 1;
            m;
        }
        (*state).xdbl[k as usize] = one_bit * x;
        k += 1;
        k;
    }
    (*state).carry = 0 as i32 as libc::c_double;
    (*state).ir = 0 as i32 as u32;
    (*state).jr = 7 as i32 as u32;
    (*state).is = 23 as i32 as u32;
    (*state).is_old = 0 as i32 as u32;
    (*state).pr = luxury;
}
unsafe extern "C" fn ranlxs0_set(mut vstate: *mut libc::c_void, mut s: u64) {
    ranlxs_set_lux(vstate, s, 109 as i32 as u32);
}
unsafe extern "C" fn ranlxs1_set(mut vstate: *mut libc::c_void, mut s: u64) {
    ranlxs_set_lux(vstate, s, 202 as i32 as u32);
}
unsafe extern "C" fn ranlxs2_set(mut vstate: *mut libc::c_void, mut s: u64) {
    ranlxs_set_lux(vstate, s, 397 as i32 as u32);
}
static mut ranlxs0_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlxs0\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranlxs_state_t>() as u64,
        set: Some(ranlxs0_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranlxs_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranlxs_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut ranlxs1_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlxs1\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranlxs_state_t>() as u64,
        set: Some(ranlxs1_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranlxs_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranlxs_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
static mut ranlxs2_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"ranlxs2\0" as *const u8 as *const i8,
        max: 0xffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ranlxs_state_t>() as u64,
        set: Some(ranlxs2_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ranlxs_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ranlxs_get_double
                as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_ranlxs0: *const gsl_rng_type = unsafe {
    &ranlxs0_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_ranlxs1: *const gsl_rng_type = unsafe {
    &ranlxs1_type as *const gsl_rng_type
};
#[no_mangle]
pub static mut gsl_rng_ranlxs2: *const gsl_rng_type = unsafe {
    &ranlxs2_type as *const gsl_rng_type
};