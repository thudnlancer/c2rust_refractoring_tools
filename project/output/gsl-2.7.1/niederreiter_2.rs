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
pub struct nied2_state_t {
    pub sequence_count: u32,
    pub cj: [[i32; 12]; 31],
    pub nextq: [i32; 12],
}
static mut nied2_type: gsl_qrng_type = unsafe {
    {
        let mut init = gsl_qrng_type {
            name: b"niederreiter-base-2\0" as *const u8 as *const i8,
            max_dimension: 12 as i32 as u32,
            state_size: Some(nied2_state_size as unsafe extern "C" fn(u32) -> size_t),
            init_state: Some(
                nied2_init as unsafe extern "C" fn(*mut libc::c_void, u32) -> i32,
            ),
            get: Some(
                nied2_get
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
pub static mut gsl_qrng_niederreiter_2: *const gsl_qrng_type = unsafe {
    &nied2_type as *const gsl_qrng_type
};
static mut primitive_poly: [[i32; 6]; 13] = [
    [1 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32],
    [0 as i32, 1 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32],
    [1 as i32, 1 as i32, 0 as i32, 0 as i32, 0 as i32, 0 as i32],
    [1 as i32, 1 as i32, 1 as i32, 0 as i32, 0 as i32, 0 as i32],
    [1 as i32, 1 as i32, 0 as i32, 1 as i32, 0 as i32, 0 as i32],
    [1 as i32, 0 as i32, 1 as i32, 1 as i32, 0 as i32, 0 as i32],
    [1 as i32, 1 as i32, 0 as i32, 0 as i32, 1 as i32, 0 as i32],
    [1 as i32, 0 as i32, 0 as i32, 1 as i32, 1 as i32, 0 as i32],
    [1 as i32, 1 as i32, 1 as i32, 1 as i32, 1 as i32, 0 as i32],
    [1 as i32, 0 as i32, 1 as i32, 0 as i32, 0 as i32, 1 as i32],
    [1 as i32, 0 as i32, 0 as i32, 1 as i32, 0 as i32, 1 as i32],
    [1 as i32, 1 as i32, 1 as i32, 1 as i32, 0 as i32, 1 as i32],
    [1 as i32, 1 as i32, 1 as i32, 0 as i32, 1 as i32, 1 as i32],
];
static mut poly_degree: [i32; 13] = [
    0 as i32,
    1 as i32,
    1 as i32,
    2 as i32,
    3 as i32,
    3 as i32,
    4 as i32,
    4 as i32,
    4 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
    5 as i32,
];
unsafe extern "C" fn nied2_state_size(mut dimension: u32) -> size_t {
    return ::core::mem::size_of::<nied2_state_t>() as u64;
}
unsafe extern "C" fn poly_multiply(
    mut pa: *const i32,
    mut pa_degree: i32,
    mut pb: *const i32,
    mut pb_degree: i32,
    mut pc: *mut i32,
    mut pc_degree: *mut i32,
) {
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut pt: [i32; 51] = [0; 51];
    let mut pt_degree: i32 = pa_degree + pb_degree;
    k = 0 as i32;
    while k <= pt_degree {
        let mut term: i32 = 0 as i32;
        j = 0 as i32;
        while j <= k {
            let conv_term: i32 = *pa.offset((k - j) as isize) * *pb.offset(j as isize)
                % 2 as i32;
            term = (term + conv_term) % 2 as i32;
            j += 1;
            j;
        }
        pt[k as usize] = term;
        k += 1;
        k;
    }
    k = 0 as i32;
    while k <= pt_degree {
        *pc.offset(k as isize) = pt[k as usize];
        k += 1;
        k;
    }
    k = pt_degree + 1 as i32;
    while k <= 50 as i32 {
        *pc.offset(k as isize) = 0 as i32;
        k += 1;
        k;
    }
    *pc_degree = pt_degree;
}
unsafe extern "C" fn calculate_v(
    mut px: *const i32,
    mut px_degree: i32,
    mut pb: *mut i32,
    mut pb_degree: *mut i32,
    mut v: *mut i32,
    mut maxv: i32,
) {
    let nonzero_element: i32 = 1 as i32;
    let arbitrary_element: i32 = 1 as i32;
    let mut ph: [i32; 51] = [0; 51];
    let mut bigm: i32 = *pb_degree;
    let mut m: i32 = 0;
    let mut r: i32 = 0;
    let mut k: i32 = 0;
    let mut kj: i32 = 0;
    k = 0 as i32;
    while k <= 50 as i32 {
        ph[k as usize] = *pb.offset(k as isize);
        k += 1;
        k;
    }
    poly_multiply(px, px_degree, pb as *const i32, *pb_degree, pb, pb_degree);
    m = *pb_degree;
    kj = bigm;
    r = 0 as i32;
    while r < kj {
        *v.offset(r as isize) = 0 as i32;
        r += 1;
        r;
    }
    *v.offset(kj as isize) = 1 as i32;
    if kj >= bigm {
        r = kj + 1 as i32;
        while r < m {
            *v.offset(r as isize) = arbitrary_element;
            r += 1;
            r;
        }
    } else {
        let mut term: i32 = (0 as i32 + ph[kj as usize]) % 2 as i32;
        r = kj + 1 as i32;
        while r < bigm {
            *v.offset(r as isize) = arbitrary_element;
            term = (term + ph[r as usize] * *v.offset(r as isize) % 2 as i32) % 2 as i32;
            r += 1;
            r;
        }
        *v.offset(bigm as isize) = (nonzero_element + term) % 2 as i32;
        r = bigm + 1 as i32;
        while r < m {
            *v.offset(r as isize) = arbitrary_element;
            r += 1;
            r;
        }
    }
    r = 0 as i32;
    while r <= maxv - m {
        let mut term_0: i32 = 0 as i32;
        k = 0 as i32;
        while k < m {
            term_0 = (term_0
                + *pb.offset(k as isize) * *v.offset((r + k) as isize) % 2 as i32)
                % 2 as i32;
            k += 1;
            k;
        }
        *v.offset((r + m) as isize) = term_0;
        r += 1;
        r;
    }
}
unsafe extern "C" fn calculate_cj(mut ns: *mut nied2_state_t, mut dimension: u32) {
    let mut ci: [[i32; 31]; 31] = [[0; 31]; 31];
    let mut v: [i32; 82] = [0; 82];
    let mut r: i32 = 0;
    let mut i_dim: u32 = 0;
    i_dim = 0 as i32 as u32;
    while i_dim < dimension {
        let poly_index: i32 = i_dim.wrapping_add(1 as i32 as u32) as i32;
        let mut j: i32 = 0;
        let mut k: i32 = 0;
        let mut u: i32 = 0 as i32;
        let mut pb: [i32; 51] = [0; 51];
        let mut px: [i32; 51] = [0; 51];
        let mut px_degree: i32 = poly_degree[poly_index as usize];
        let mut pb_degree: i32 = 0 as i32;
        k = 0 as i32;
        while k <= px_degree {
            px[k as usize] = primitive_poly[poly_index as usize][k as usize];
            pb[k as usize] = 0 as i32;
            k += 1;
            k;
        }
        while k < 50 as i32 + 1 as i32 {
            px[k as usize] = 0 as i32;
            pb[k as usize] = 0 as i32;
            k += 1;
            k;
        }
        pb[0 as i32 as usize] = 1 as i32;
        j = 0 as i32;
        while j < 30 as i32 + 1 as i32 {
            if u == 0 as i32 {
                calculate_v(
                    px.as_mut_ptr() as *const i32,
                    px_degree,
                    pb.as_mut_ptr(),
                    &mut pb_degree,
                    v.as_mut_ptr(),
                    30 as i32 + 1 as i32 + 50 as i32,
                );
            }
            r = 0 as i32;
            while r < 30 as i32 + 1 as i32 {
                ci[r as usize][j as usize] = v[(r + u) as usize];
                r += 1;
                r;
            }
            u += 1;
            u;
            if u == px_degree {
                u = 0 as i32;
            }
            j += 1;
            j;
        }
        r = 0 as i32;
        while r < 30 as i32 + 1 as i32 {
            let mut term: i32 = 0 as i32;
            j = 0 as i32;
            while j < 30 as i32 + 1 as i32 {
                term = 2 as i32 * term + ci[r as usize][j as usize];
                j += 1;
                j;
            }
            (*ns).cj[r as usize][i_dim as usize] = term;
            r += 1;
            r;
        }
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
}
unsafe extern "C" fn nied2_init(
    mut state: *mut libc::c_void,
    mut dimension: u32,
) -> i32 {
    let mut n_state: *mut nied2_state_t = state as *mut nied2_state_t;
    let mut i_dim: u32 = 0;
    if dimension < 1 as i32 as u32 || dimension > 12 as i32 as u32 {
        return GSL_EINVAL as i32;
    }
    calculate_cj(n_state, dimension);
    i_dim = 0 as i32 as u32;
    while i_dim < dimension {
        (*n_state).nextq[i_dim as usize] = 0 as i32;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    (*n_state).sequence_count = 0 as i32 as u32;
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn nied2_get(
    mut state: *mut libc::c_void,
    mut dimension: u32,
    mut v: *mut libc::c_double,
) -> i32 {
    static mut recip: libc::c_double = 1.0f64
        / ((1 as u32) << 30 as i32 + 1 as i32) as libc::c_double;
    let mut n_state: *mut nied2_state_t = state as *mut nied2_state_t;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut i_dim: u32 = 0;
    i_dim = 0 as i32 as u32;
    while i_dim < dimension {
        *v.offset(i_dim as isize) = (*n_state).nextq[i_dim as usize] as libc::c_double
            * recip;
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    r = 0 as i32;
    c = (*n_state).sequence_count as i32;
    while c % 2 as i32 == 1 as i32 {
        r += 1;
        r;
        c /= 2 as i32;
    }
    if r >= 30 as i32 + 1 as i32 {
        return GSL_EFAILED as i32;
    }
    i_dim = 0 as i32 as u32;
    while i_dim < dimension {
        (*n_state).nextq[i_dim as usize] ^= (*n_state).cj[r as usize][i_dim as usize];
        i_dim = i_dim.wrapping_add(1);
        i_dim;
    }
    (*n_state).sequence_count = ((*n_state).sequence_count).wrapping_add(1);
    (*n_state).sequence_count;
    return GSL_SUCCESS as i32;
}