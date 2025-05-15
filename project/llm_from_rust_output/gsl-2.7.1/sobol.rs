use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

#[derive(Debug, Clone)]
pub struct GslQrngType {
    pub name: &'static str,
    pub max_dimension: u32,
    pub state_size: fn(u32) -> usize,
    pub init_state: fn(&mut SobolState, u32) -> Result<(), GslError>,
    pub get: fn(&mut SobolState, u32, &mut [f64]) -> Result<(), GslError>,
}

#[derive(Debug, Clone)]
pub struct SobolState {
    pub sequence_count: u32,
    pub last_denominator_inv: f64,
    pub last_numerator_vec: [i32; 40],
    pub v_direction: [[i32; 40]; 30],
}

const PRIMITIVE_POLYNOMIALS: [i32; 40] = [
    1, 3, 7, 11, 13, 19, 25, 37, 59, 47, 61, 55, 41, 67, 97, 91, 109, 103, 115, 131, 193, 137,
    145, 143, 241, 157, 185, 167, 229, 171, 213, 191, 253, 203, 211, 239, 247, 285, 369, 299,
];

const DEGREE_TABLE: [i32; 40] = [
    0, 1, 2, 3, 3, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    7, 7, 7, 7, 7, 7, 8, 8, 8,
];

const V_INIT: [[i32; 40]; 8] = [
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ],
    [
        0, 1, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 1, 3, 1, 1, 3, 1, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 1, 3,
        1, 1, 3, 1, 3, 1, 3, 1, 3, 1,
    ],
    [
        0, 0, 7, 5, 1, 3, 3, 7, 5, 5, 7, 7, 1, 3, 3, 7, 5, 1, 1, 5, 3, 3, 1, 7, 5, 1, 3, 3, 7, 5,
        1, 1, 5, 7, 7, 5, 1, 3, 3, 1,
    ],
    [
        0, 0, 0, 1, 7, 9, 13, 11, 1, 3, 7, 9, 5, 13, 13, 11, 3, 15, 5, 3, 15, 7, 9, 13, 9, 1, 11,
        7, 5, 15, 1, 15, 11, 5, 3, 1, 7, 9, 13, 9,
    ],
    [
        0, 0, 0, 0, 9, 3, 27, 15, 29, 21, 23, 19, 11, 25, 7, 13, 17, 1, 25, 29, 3, 31, 11, 5, 23,
        27, 19, 21, 5, 1, 17, 13, 7, 15, 9, 31, 9, 31, 9, 9,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 37, 33, 7, 5, 11, 39, 63, 27, 17, 15, 23, 29, 3, 21, 13, 31, 25, 9,
        49, 33, 19, 29, 11, 19, 27, 15, 25, 25, 15, 25, 15, 25, 25,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 33, 115, 41, 79, 17, 29, 119, 75, 73, 105, 7, 59,
        65, 21, 3, 113, 61, 89, 45, 107, 107, 45, 107, 45, 107, 45, 107,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 7, 23, 39,
    ],
];

fn sobol_state_size(_dimension: u32) -> usize {
    std::mem::size_of::<SobolState>()
}

fn sobol_init(state: &mut SobolState, dimension: u32) -> Result<(), GslError> {
    if dimension < 1 || dimension > 40 {
        return Err(GslError::EINVAL);
    }

    for k in 0..30 {
        state.v_direction[k][0] = 1;
    }

    for i_dim in 1..dimension {
        let poly_index = i_dim as usize;
        let degree_i = DEGREE_TABLE[poly_index];
        let mut includ = [0; 8];
        let mut p_i = PRIMITIVE_POLYNOMIALS[poly_index];

        for k in (0..degree_i).rev() {
            includ[k as usize] = (p_i % 2 == 1) as i32;
            p_i /= 2;
        }

        for j in 0..degree_i {
            state.v_direction[j as usize][i_dim as usize] = V_INIT[j as usize][i_dim as usize];
        }

        for j in degree_i..30 {
            let mut newv = state.v_direction[(j - degree_i) as usize][i_dim as usize];
            let mut ell = 1;

            for k in 0..degree_i {
                ell *= 2;
                if includ[k as usize] != 0 {
                    newv ^= ell * state.v_direction[(j - k - 1) as usize][i_dim as usize];
                }
            }

            state.v_direction[j as usize][i_dim as usize] = newv;
        }
    }

    let mut ell = 1;
    for j in (0..30 - 1 - 1).rev() {
        ell *= 2;
        for i_dim in 0..dimension {
            state.v_direction[j as usize][i_dim as usize] *= ell;
        }
    }

    state.last_denominator_inv = 1.0 / (2.0 * ell as f64);
    state.sequence_count = 0;
    for i_dim in 0..dimension {
        state.last_numerator_vec[i_dim as usize] = 0;
    }

    Ok(())
}

fn sobol_get(state: &mut SobolState, dimension: u32, v: &mut [f64]) -> Result<(), GslError> {
    let mut ell = 0;
    let mut c = state.sequence_count as i32;

    loop {
        ell += 1;
        if c % 2 != 1 {
            break;
        }
        c /= 2;
    }

    if ell > 30 {
        return Err(GslError::EFAILED);
    }

    for i_dim in 0..dimension {
        let direction_i = state.v_direction[(ell - 1) as usize][i_dim as usize];
        let old_numerator_i = state.last_numerator_vec[i_dim as usize];
        let new_numerator_i = old_numerator_i ^ direction_i;
        state.last_numerator_vec[i_dim as usize] = new_numerator_i;
        v[i_dim as usize] = new_numerator_i as f64 * state.last_denominator_inv;
    }

    state.sequence_count += 1;
    Ok(())
}

pub static GSL_QRNG_SOBOL: GslQrngType = GslQrngType {
    name: "sobol",
    max_dimension: 40,
    state_size: sobol_state_size,
    init_state: sobol_init,
    get: sobol_get,
};