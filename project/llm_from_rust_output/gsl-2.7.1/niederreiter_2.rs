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
pub struct Nied2State {
    sequence_count: u32,
    cj: [[i32; 12]; 31],
    nextq: [i32; 12],
}

#[derive(Debug, Clone)]
pub struct GslQrngType {
    name: &'static str,
    max_dimension: u32,
    state_size: fn(u32) -> usize,
    init_state: fn(&mut Nied2State, u32) -> Result<(), GslError>,
    get: fn(&mut Nied2State, u32, &mut [f64]) -> Result<(), GslError>,
}

const PRIMITIVE_POLY: [[i32; 6]; 13] = [
    [1, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0, 0],
    [1, 1, 1, 0, 0, 0],
    [1, 1, 0, 1, 0, 0],
    [1, 0, 1, 1, 0, 0],
    [1, 1, 0, 0, 1, 0],
    [1, 0, 0, 1, 1, 0],
    [1, 1, 1, 1, 1, 0],
    [1, 0, 1, 0, 0, 1],
    [1, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 0, 1],
    [1, 1, 1, 0, 1, 1],
];

const POLY_DEGREE: [i32; 13] = [0, 1, 1, 2, 3, 3, 4, 4, 4, 5, 5, 5, 5];

fn nied2_state_size(_dimension: u32) -> usize {
    std::mem::size_of::<Nied2State>()
}

fn poly_multiply(
    pa: &[i32],
    pa_degree: i32,
    pb: &[i32],
    pb_degree: i32,
    pc: &mut [i32],
    pc_degree: &mut i32,
) {
    let mut pt = [0; 51];
    let pt_degree = pa_degree + pb_degree;
    
    for k in 0..=pt_degree {
        let mut term = 0;
        for j in 0..=k {
            let conv_term = pa[(k - j) as usize] * pb[j as usize] % 2;
            term = (term + conv_term) % 2;
        }
        pt[k as usize] = term;
    }
    
    for k in 0..=pt_degree {
        pc[k as usize] = pt[k as usize];
    }
    
    for k in (pt_degree + 1)..=50 {
        pc[k as usize] = 0;
    }
    
    *pc_degree = pt_degree;
}

fn calculate_v(
    px: &[i32],
    px_degree: i32,
    pb: &mut [i32],
    pb_degree: &mut i32,
    v: &mut [i32],
    maxv: i32,
) {
    let nonzero_element = 1;
    let arbitrary_element = 1;
    let mut ph = [0; 51];
    let bigm = *pb_degree;
    
    ph[..=50].copy_from_slice(&pb[..=50]);
    poly_multiply(px, px_degree, pb, *pb_degree, pb, pb_degree);
    
    let m = *pb_degree;
    let kj = bigm;
    
    for r in 0..kj {
        v[r as usize] = 0;
    }
    
    v[kj as usize] = 1;
    
    if kj >= bigm {
        for r in (kj + 1)..m {
            v[r as usize] = arbitrary_element;
        }
    } else {
        let mut term = (0 + ph[kj as usize]) % 2;
        for r in (kj + 1)..bigm {
            v[r as usize] = arbitrary_element;
            term = (term + ph[r as usize] * v[r as usize] % 2) % 2;
        }
        v[bigm as usize] = (nonzero_element + term) % 2;
        for r in (bigm + 1)..m {
            v[r as usize] = arbitrary_element;
        }
    }
    
    for r in 0..=(maxv - m) {
        let mut term = 0;
        for k in 0..m {
            term = (term + pb[k as usize] * v[(r + k) as usize] % 2) % 2;
        }
        v[(r + m) as usize] = term;
    }
}

fn calculate_cj(ns: &mut Nied2State, dimension: u32) {
    let mut ci = [[0; 31]; 31];
    let mut v = [0; 82];
    
    for i_dim in 0..dimension {
        let poly_index = (i_dim + 1) as usize;
        let mut pb = [0; 51];
        let mut px = [0; 51];
        let px_degree = POLY_DEGREE[poly_index];
        let mut pb_degree = 0;
        
        for k in 0..=px_degree {
            px[k as usize] = PRIMITIVE_POLY[poly_index][k as usize];
            pb[k as usize] = 0;
        }
        
        for k in (px_degree + 1)..=50 {
            px[k as usize] = 0;
            pb[k as usize] = 0;
        }
        
        pb[0] = 1;
        
        let mut u = 0;
        for j in 0..=30 {
            if u == 0 {
                calculate_v(
                    &px, 
                    px_degree, 
                    &mut pb, 
                    &mut pb_degree, 
                    &mut v, 
                    30 + 1 + 50
                );
            }
            
            for r in 0..=30 {
                ci[r as usize][j as usize] = v[(r + u) as usize];
            }
            
            u += 1;
            if u == px_degree {
                u = 0;
            }
        }
        
        for r in 0..=30 {
            let mut term = 0;
            for j in 0..=30 {
                term = 2 * term + ci[r as usize][j as usize];
            }
            ns.cj[r as usize][i_dim as usize] = term;
        }
    }
}

fn nied2_init(state: &mut Nied2State, dimension: u32) -> Result<(), GslError> {
    if dimension < 1 || dimension > 12 {
        return Err(GslError::EINVAL);
    }
    
    calculate_cj(state, dimension);
    
    for i_dim in 0..dimension {
        state.nextq[i_dim as usize] = 0;
    }
    
    state.sequence_count = 0;
    Ok(())
}

fn nied2_get(state: &mut Nied2State, dimension: u32, v: &mut [f64]) -> Result<(), GslError> {
    const RECIP: f64 = 1.0 / ((1u32 << (30 + 1)) as f64);
    
    for i_dim in 0..dimension {
        v[i_dim as usize] = state.nextq[i_dim as usize] as f64 * RECIP;
    }
    
    let mut r = 0;
    let mut c = state.sequence_count as i32;
    
    while c % 2 == 1 {
        r += 1;
        c /= 2;
    }
    
    if r >= 30 + 1 {
        return Err(GslError::EFAILED);
    }
    
    for i_dim in 0..dimension {
        state.nextq[i_dim as usize] ^= state.cj[r as usize][i_dim as usize];
    }
    
    state.sequence_count += 1;
    Ok(())
}

const NIED2_TYPE: GslQrngType = GslQrngType {
    name: "niederreiter-base-2",
    max_dimension: 12,
    state_size: nied2_state_size,
    init_state: nied2_init,
    get: nied2_get,
};

pub const GSL_QRNG_NIEDERRETTER_2: &GslQrngType = &NIED2_TYPE;