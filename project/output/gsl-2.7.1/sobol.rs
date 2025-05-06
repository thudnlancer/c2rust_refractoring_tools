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
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_qrng_type {
    pub name: *const i8,
    pub max_dimension: u32,
    pub state_size: Option<unsafe extern "C" fn(u32) -> size_t>,
    pub init_state: Option<unsafe extern "C" fn(*mut libc::c_void, u32) -> i32>,
    pub get: Option<
        unsafe extern "C" fn(*mut libc::c_void, u32, *mut libc::c_double) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sobol_state_t {
    pub sequence_count: u32,
    pub last_denominator_inv: libc::c_double,
    pub last_numerator_vec: [i32; 40],
    pub v_direction: [[i32; 40]; 30],
}
static mut sobol_type: gsl_qrng_type = unsafe {
    {
        let mut init = gsl_qrng_type {
            name: b"sobol\0" as *const u8 as *const i8,
            max_dimension: 40 as i32 as u32,
            state_size: Some(sobol_state_size as unsafe extern "C" fn(u32) -> size_t),
            init_state: Some(
                sobol_init as unsafe extern "C" fn(*mut libc::c_void, u32) -> i32,
            ),
            get: Some(
                sobol_get
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        u32,
                        *mut libc::c_double,
                    ) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut gsl_qrng_sobol: *const gsl_qrng_type = unsafe {
    &sobol_type as *const gsl_qrng_type
};
static mut primitive_polynomials: [i32; 40] = [
    1 as i32,
    3 as i32,
    7 as i32,
    11 as i32,
    13 as i32,
    19 as i32,
    25 as i32,
    37 as i32,
    59 as i32,
    47 as i32,
    61 as i32,
    55 as i32,
    41 as i32,
    67 as i32,
    97 as i32,
    91 as i32,
    109 as i32,
    103 as i32,
    115 as i32,
    131 as i32,
    193 as i32,
    137 as i32,
    145 as i32,
    143 as i32,
    241 as i32,
    157 as i32,
    185 as i32,
    167 as i32,
    229 as i32,
    171 as i32,
    213 as i32,
    191 as i32,
    253 as i32,
    203 as i32,
    211 as i32,
    239 as i32,
    247 as i32,
    285 as i32,
    369 as i32,
    299 as i32,
];
static mut degree_table: [i32; 40] = [
    0 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    3 as i32,
    4 as i32,
    4 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    6 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    7 as i32,
    8 as i32,
    8 as i32,
    8 as i32,
];
static mut v_init: [[i32; 40]; 8] = [
    [
        0 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
        1 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
        1 as i32,
        3 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        7 as i32,
        5 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
        7 as i32,
        5 as i32,
        5 as i32,
        7 as i32,
        7 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
        7 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        5 as i32,
        3 as i32,
        3 as i32,
        1 as i32,
        7 as i32,
        5 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
        7 as i32,
        5 as i32,
        1 as i32,
        1 as i32,
        5 as i32,
        7 as i32,
        7 as i32,
        5 as i32,
        1 as i32,
        3 as i32,
        3 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        1 as i32,
        7 as i32,
        9 as i32,
        13 as i32,
        11 as i32,
        1 as i32,
        3 as i32,
        7 as i32,
        9 as i32,
        5 as i32,
        13 as i32,
        13 as i32,
        11 as i32,
        3 as i32,
        15 as i32,
        5 as i32,
        3 as i32,
        15 as i32,
        7 as i32,
        9 as i32,
        13 as i32,
        9 as i32,
        1 as i32,
        11 as i32,
        7 as i32,
        5 as i32,
        15 as i32,
        1 as i32,
        15 as i32,
        11 as i32,
        5 as i32,
        3 as i32,
        1 as i32,
        7 as i32,
        9 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        9 as i32,
        3 as i32,
        27 as i32,
        15 as i32,
        29 as i32,
        21 as i32,
        23 as i32,
        19 as i32,
        11 as i32,
        25 as i32,
        7 as i32,
        13 as i32,
        17 as i32,
        1 as i32,
        25 as i32,
        29 as i32,
        3 as i32,
        31 as i32,
        11 as i32,
        5 as i32,
        23 as i32,
        27 as i32,
        19 as i32,
        21 as i32,
        5 as i32,
        1 as i32,
        17 as i32,
        13 as i32,
        7 as i32,
        15 as i32,
        9 as i32,
        31 as i32,
        9 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        37 as i32,
        33 as i32,
        7 as i32,
        5 as i32,
        11 as i32,
        39 as i32,
        63 as i32,
        27 as i32,
        17 as i32,
        15 as i32,
        23 as i32,
        29 as i32,
        3 as i32,
        21 as i32,
        13 as i32,
        31 as i32,
        25 as i32,
        9 as i32,
        49 as i32,
        33 as i32,
        19 as i32,
        29 as i32,
        11 as i32,
        19 as i32,
        27 as i32,
        15 as i32,
        25 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        13 as i32,
        33 as i32,
        115 as i32,
        41 as i32,
        79 as i32,
        17 as i32,
        29 as i32,
        119 as i32,
        75 as i32,
        73 as i32,
        105 as i32,
        7 as i32,
        59 as i32,
        65 as i32,
        21 as i32,
        3 as i32,
        113 as i32,
        61 as i32,
        89 as i32,
        45 as i32,
        107 as i32,
    ],
    [
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        7 as i32,
        23 as i32,
        39 as i32,
    ],
];
unsafe extern "C" fn sobol_state_size(mut dimension: u32) -> size_t {
    return ::core::mem::size_of::<sobol_state_t>() as u64;
}
unsafe extern "C" fn sobol_init(
    mut state: *mut libc::c_void,
    mut dimension: u32,
) -> i32 {
    let mut s_state: *mut sobol_state_t = state as *mut sobol_state_t;
    let mut i_dim: u32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ell: i32 = 0;
    if dimension < 1 as i32 as u32 || dimension > 40 as i32 as u32 {
        return GSL_EINVAL as i32;
    }
    k = 0 as i32;
    while k < 30 as i32 {
        (*s_state).v_direction[k as usize][0 as i32 as usize] = 1 as i32;
        k += 1;
        k;
    }
    i_dim = 1 as i32 as u32;
    while i_dim < dimension {
        let poly_index: i32 = i_dim as i32;
        let degree_i: i32 = degree_table[poly_index as usize];
        let mut includ: [i32; 8] = [0; 8];
        let mut p_i: i32 = primitive_polynomials[poly_index as usize];
        k = degree_i - 1 as i32;
        while k >= 0 as i32 {
            includ[k as usize] = (p_i % 2 as i32 == 1 as i32) as i32;
            p_i /= 2 as i32;
            k -= 1;
            k;
        }
        j = 0 as i32;
        while j < degree_i {
            (*s_state).v_direction[j as usize][i_dim as usize] = v_init[j
                as usize][i_dim as usize];
            j += 1;
            j;
        }
        j = degree_i;
        while j < 30 as i32 {
            let mut newv: i32 = (*s_state)
                .v_direction[(j - degree_i) as usize][i_dim as usize];
            ell = 1 as i32;
            k = 0 as i32;
            while k < degree_i {
                ell *= 2 as i32;
                if includ[k as usize] != 0 {
                    newv
                        ^= ell
                            * (*s_state)
                                .v_direction[(j - k - 1 as i32) as usize][i_dim as usize];
                }
                k += 1;
                k;
            }
            (*s_state).v_direction[j as usize][i_dim as usize] = newv;
            j += 1;
            j;
        }
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    ell = 1 as i32;
    j = 30 as i32 - 1 as i32 - 1 as i32;
    while j >= 0 as i32 {
        ell *= 2 as i32;
        i_dim = 0 as i32 as u32;
        while i_dim < dimension {
            (*s_state).v_direction[j as usize][i_dim as usize] *= ell;
            i_dim = i_dim.wrapping_add(1);
            i_dim;
        }
        j -= 1;
        j;
    }
    (*s_state).last_denominator_inv = 1.0f64 / (2.0f64 * ell as libc::c_double);
    (*s_state).sequence_count = 0 as i32 as u32;
    i_dim = 0 as i32 as u32;
    while i_dim < dimension {
        (*s_state).last_numerator_vec[i_dim as usize] = 0 as i32;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn sobol_get(
    mut state: *mut libc::c_void,
    mut dimension: u32,
    mut v: *mut libc::c_double,
) -> i32 {
    let mut s_state: *mut sobol_state_t = state as *mut sobol_state_t;
    let mut i_dimension: u32 = 0;
    let mut ell: i32 = 0 as i32;
    let mut c: i32 = (*s_state).sequence_count as i32;
    loop {
        ell += 1;
        ell;
        if !(c % 2 as i32 == 1 as i32) {
            break;
        }
        c /= 2 as i32;
    }
    if ell > 30 as i32 {
        return GSL_EFAILED as i32;
    }
    i_dimension = 0 as i32 as u32;
    while i_dimension < dimension {
        let direction_i: i32 = (*s_state)
            .v_direction[(ell - 1 as i32) as usize][i_dimension as usize];
        let old_numerator_i: i32 = (*s_state).last_numerator_vec[i_dimension as usize];
        let new_numerator_i: i32 = old_numerator_i ^ direction_i;
        (*s_state).last_numerator_vec[i_dimension as usize] = new_numerator_i;
        *v.offset(i_dimension as isize) = new_numerator_i as libc::c_double
            * (*s_state).last_denominator_inv;
        i_dimension = i_dimension.wrapping_add(1);
        i_dimension;
    }
    (*s_state).sequence_count = ((*s_state).sequence_count).wrapping_add(1);
    (*s_state).sequence_count;
    return GSL_SUCCESS as i32;
}