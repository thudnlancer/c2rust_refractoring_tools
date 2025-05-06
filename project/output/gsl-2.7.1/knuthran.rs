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
    pub i: u32,
    pub aa: [u64; 2009],
    pub ran_x: [u64; 100],
}
#[inline]
unsafe extern "C" fn ran_array(mut aa: *mut u64, mut n: u32, mut ran_x: *mut u64) {
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    j = 0 as i32 as u32;
    while j < 100 as i32 as u32 {
        *aa.offset(j as isize) = *ran_x.offset(j as isize);
        j = j.wrapping_add(1);
        j;
    }
    while j < n {
        *aa.offset(j as isize) = (*aa.offset(j.wrapping_sub(100 as i32 as u32) as isize))
            .wrapping_sub(*aa.offset(j.wrapping_sub(37 as i32 as u32) as isize))
            & (((1 as i64) << 30 as i32) - 1 as i32 as i64) as u64;
        j = j.wrapping_add(1);
        j;
    }
    i = 0 as i32 as u32;
    while i < 37 as i32 as u32 {
        *ran_x.offset(i as isize) = (*aa
            .offset(j.wrapping_sub(100 as i32 as u32) as isize))
            .wrapping_sub(*aa.offset(j.wrapping_sub(37 as i32 as u32) as isize))
            & (((1 as i64) << 30 as i32) - 1 as i32 as i64) as u64;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
    while i < 100 as i32 as u32 {
        *ran_x.offset(i as isize) = (*aa
            .offset(j.wrapping_sub(100 as i32 as u32) as isize))
            .wrapping_sub(*ran_x.offset(i.wrapping_sub(37 as i32 as u32) as isize))
            & (((1 as i64) << 30 as i32) - 1 as i32 as i64) as u64;
        i = i.wrapping_add(1);
        i;
        j = j.wrapping_add(1);
        j;
    }
}
#[inline]
unsafe extern "C" fn ran_get(mut vstate: *mut libc::c_void) -> u64 {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut i: u32 = (*state).i;
    if i == 0 as i32 as u32 {
        ran_array(
            ((*state).aa).as_mut_ptr(),
            2009 as i32 as u32,
            ((*state).ran_x).as_mut_ptr(),
        );
    }
    (*state).i = i.wrapping_add(1 as i32 as u32).wrapping_rem(2009 as i32 as u32);
    return (*state).aa[i as usize];
}
unsafe extern "C" fn ran_get_double(mut vstate: *mut libc::c_void) -> libc::c_double {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    return ran_get(state as *mut libc::c_void) as libc::c_double / 1073741824.0f64;
}
unsafe extern "C" fn ran_set(mut vstate: *mut libc::c_void, mut s: u64) {
    let mut state: *mut ran_state_t = vstate as *mut ran_state_t;
    let mut x: [i64; 199] = [0; 199];
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut ss: i64 = (s.wrapping_add(2 as i32 as u64)
        & (((1 as i64) << 30 as i32) - 2 as i32 as i64) as u64) as i64;
    j = 0 as i32;
    while j < 100 as i32 {
        x[j as usize] = ss;
        ss <<= 1 as i32;
        if ss >= (1 as i64) << 30 as i32 {
            ss -= ((1 as i64) << 30 as i32) - 2 as i32 as i64;
        }
        j += 1;
        j;
    }
    while j < 100 as i32 + 100 as i32 - 1 as i32 {
        x[j as usize] = 0 as i32 as i64;
        j += 1;
        j;
    }
    x[1 as i32 as usize] += 1;
    x[1 as i32 as usize];
    ss = (s & (((1 as i64) << 30 as i32) - 1 as i32 as i64) as u64) as i64;
    t = 70 as i32 - 1 as i32;
    while t != 0 {
        j = 100 as i32 - 1 as i32;
        while j > 0 as i32 {
            x[(j + j) as usize] = x[j as usize];
            j -= 1;
            j;
        }
        j = 100 as i32 + 100 as i32 - 2 as i32;
        while j > 100 as i32 - 37 as i32 {
            x[(100 as i32 + 100 as i32 - 1 as i32 - j) as usize] = x[j as usize]
                & ((1 as i64) << 30 as i32) - 2 as i32 as i64;
            j -= 2 as i32;
        }
        j = 100 as i32 + 100 as i32 - 2 as i32;
        while j >= 100 as i32 {
            if x[j as usize] & 1 as i32 as i64 != 0 {
                x[(j - (100 as i32 - 37 as i32)) as usize] = x[(j
                    - (100 as i32 - 37 as i32)) as usize] - x[j as usize]
                    & ((1 as i64) << 30 as i32) - 1 as i32 as i64;
                x[(j - 100 as i32) as usize] = x[(j - 100 as i32) as usize]
                    - x[j as usize] & ((1 as i64) << 30 as i32) - 1 as i32 as i64;
            }
            j -= 1;
            j;
        }
        if ss & 1 as i32 as i64 != 0 {
            j = 100 as i32;
            while j > 0 as i32 {
                x[j as usize] = x[(j - 1 as i32) as usize];
                j -= 1;
                j;
            }
            x[0 as i32 as usize] = x[100 as i32 as usize];
            if x[100 as i32 as usize] & 1 as i32 as i64 != 0 {
                x[37 as i32 as usize] = x[37 as i32 as usize] - x[100 as i32 as usize]
                    & ((1 as i64) << 30 as i32) - 1 as i32 as i64;
            }
        }
        if ss != 0 {
            ss >>= 1 as i32;
        } else {
            t -= 1;
            t;
        }
    }
    (*state).i = 0 as i32 as u32;
    j = 0 as i32;
    while j < 37 as i32 {
        (*state).ran_x[(j + 100 as i32 - 37 as i32) as usize] = x[j as usize] as u64;
        j += 1;
        j;
    }
    while j < 100 as i32 {
        (*state).ran_x[(j - 37 as i32) as usize] = x[j as usize] as u64;
        j += 1;
        j;
    }
}
static mut ran_type: gsl_rng_type = {
    let mut init = gsl_rng_type {
        name: b"knuthran\0" as *const u8 as *const i8,
        max: 0x3fffffff as u64,
        min: 0 as i32 as u64,
        size: ::core::mem::size_of::<ran_state_t>() as u64,
        set: Some(ran_set as unsafe extern "C" fn(*mut libc::c_void, u64) -> ()),
        get: Some(ran_get as unsafe extern "C" fn(*mut libc::c_void) -> u64),
        get_double: Some(
            ran_get_double as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double,
        ),
    };
    init
};
#[no_mangle]
pub static mut gsl_rng_knuthran: *const gsl_rng_type = unsafe {
    &ran_type as *const gsl_rng_type
};