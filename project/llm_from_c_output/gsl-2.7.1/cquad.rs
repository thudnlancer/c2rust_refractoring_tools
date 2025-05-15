use std::f64;
use std::mem;
use std::ptr;
use std::cmp;

const GSL_DBL_EPSILON: f64 = 2.2204460492503131e-16;
const GSL_POSINF: f64 = f64::INFINITY;
const GSL_NEGINF: f64 = f64::NEG_INFINITY;
const M_SQRT2: f64 = std::f64::consts::SQRT_2;

#[derive(Debug)]
pub enum GslError {
    EDOM,
    ENOMEM,
    EINVAL,
    EBADTOL,
    EDIVERGE,
    Success,
}

pub struct GslIntegrationCquadWorkspace {
    ivals: Vec<GslIntegrationCquadIval>,
    heap: Vec<usize>,
    size: usize,
}

pub struct GslIntegrationCquadIval {
    fx: [f64; 33],
    c: [f64; 65],
    a: f64,
    b: f64,
    depth: i32,
    rdepth: i32,
    ndiv: i32,
    igral: f64,
    err: f64,
}

pub struct GslFunction {
    function: Box<dyn Fn(f64) -> f64>,
}

impl GslFunction {
    pub fn eval(&self, x: f64) -> f64 {
        (self.function)(x)
    }
}

// Constants from cquad_const.c
static V1inv: [f64; 25] = [/* ... */];
static V2inv: [f64; 81] = [/* ... */];
static V3inv: [f64; 289] = [/* ... */];
static V4inv: [f64; 1089] = [/* ... */];
static bee: [f64; 68] = [/* ... */];
static Lalpha: [f64; 32] = [/* ... */];
static Lgamma: [f64; 33] = [/* ... */];
static xi: [f64; 33] = [/* ... */];
static Tleft: [f64; 1089] = [/* ... */];
static Tright: [f64; 1089] = [/* ... */];

pub fn gsl_integration_cquad_workspace_alloc(n: usize) -> Result<GslIntegrationCquadWorkspace, GslError> {
    if n < 3 {
        return Err(GslError::EDOM);
    }

    let ivals = vec![
        GslIntegrationCquadIval {
            fx: [0.0; 33],
            c: [0.0; 65],
            a: 0.0,
            b: 0.0,
            depth: 0,
            rdepth: 0,
            ndiv: 0,
            igral: 0.0,
            err: 0.0,
        };
        n
    ];

    let heap = vec![0; n];

    Ok(GslIntegrationCquadWorkspace { ivals, heap, size: n })
}

pub fn gsl_integration_cquad_workspace_free(_w: Option<GslIntegrationCquadWorkspace>) {
    // Rust's ownership system handles memory deallocation automatically
}

fn vinvfx(fx: &[f64; 33], c: &mut [f64], d: i32) {
    match d {
        0 => {
            for i in 0..=4 {
                c[i] = 0.0;
                for j in 0..=4 {
                    c[i] += V1inv[i * 5 + j] * fx[j * 8];
                }
            }
        }
        1 => {
            for i in 0..=8 {
                c[i] = 0.0;
                for j in 0..=8 {
                    c[i] += V2inv[i * 9 + j] * fx[j * 4];
                }
            }
        }
        2 => {
            for i in 0..=16 {
                c[i] = 0.0;
                for j in 0..=16 {
                    c[i] += V3inv[i * 17 + j] * fx[j * 2];
                }
            }
        }
        3 => {
            for i in 0..=32 {
                c[i] = 0.0;
                for j in 0..=32 {
                    c[i] += V4inv[i * 33 + j] * fx[j];
                }
            }
        }
        _ => {}
    }
}

fn downdate(c: &mut [f64], n: i32, d: i32, nans: &[i32], nnans: i32) {
    let bidx = [0, 6, 16, 34];
    let mut b_new = [0.0; 34];
    let mut alpha;

    for i in 0..=(n + 1) as usize {
        b_new[i] = bee[bidx[d as usize] + i];
    }

    for i in 0..nnans as usize {
        b_new[(n + 1) as usize] /= Lalpha[n as usize];
        b_new[n as usize] = (b_new[n as usize] + xi[nans[i] as f64 * b_new[(n + 1) as usize]) / Lalpha[(n - 1) as usize];

        for j in (1..n).rev() {
            b_new[j as usize] = (b_new[j as usize] 
                + xi[nans[i] as usize] * b_new[(j + 1) as usize] 
                - Lgamma[(j + 1) as usize] * b_new[(j + 2) as usize]) 
                / Lalpha[(j - 1) as usize];
        }

        for j in 0..=n as usize {
            b_new[j] = b_new[j + 1];
        }

        alpha = c[n as usize] / b_new[n as usize];
        for j in 0..n as usize {
            c[j] -= alpha * b_new[j];
        }
        c[n as usize] = 0.0;
    }
}

pub fn gsl_integration_cquad(
    f: &GslFunction,
    a: f64,
    b: f64,
    epsabs: f64,
    epsrel: f64,
    ws: &mut GslIntegrationCquadWorkspace,
    result: &mut f64,
    abserr: Option<&mut f64>,
    nevals: Option<&mut usize>,
) -> Result<(), GslError> {
    let n = [4, 8, 16, 32];
    let skip = [8, 4, 2, 1];
    let idx = [0, 5, 14, 31];
    let w = M_SQRT2 / 2.0;
    let ndiv_max = 20;

    let mut m;
    let mut h;
    let mut temp;
    let mut igral = 0.0;
    let mut err = 0.0;
    let mut igral_final = 0.0;
    let mut err_final = 0.0;
    let mut err_excess = 0.0;
    let mut nivals = 0;
    let mut neval = 0;
    let mut nnans;
    let mut nans = [0; 32];
    let mut nc;
    let mut ncdiff;

    if epsabs < 0.0 || epsrel < 0.0 {
        return Err(GslError::EBADTOL);
    }
    if epsabs <= 0.0 && epsrel < GSL_DBL_EPSILON {
        return Err(GslError::EBADTOL);
    }

    // Initialize first interval
    let mut iv = &mut ws.ivals[0];
    m = (a + b) / 2.0;
    h = (b - a) / 2.0;
    nnans = 0;

    for i in 0..=n[3] {
        iv.fx[i] = f.eval(m + xi[i] * h);
        neval += 1;
        if !iv.fx[i].is_finite() {
            nans[nnans] = i as i32;
            nnans += 1;
            iv.fx[i] = 0.0;
        }
    }

    vinvfx(&iv.fx, &mut iv.c[idx[0]..], 0);
    vinvfx(&iv.fx, &mut iv.c[idx[3]..], 3);
    vinvfx(&iv.fx, &mut iv.c[idx[2]..], 2);

    for i in 0..nnans {
        iv.fx[nans[i] as usize] = f64::NAN;
    }

    iv.a = a;
    iv.b = b;
    iv.depth = 3;
    iv.rdepth = 1;
    iv.ndiv = 0;
    iv.igral = 2.0 * h * iv.c[idx[3]] * w;

    nc = 0.0;
    for i in (n[2] + 1)..=n[3] {
        temp = iv.c[idx[3] + i];
        nc += temp * temp;
    }
    ncdiff = nc;
    for i in 0..=n[2] {
        temp = iv.c[idx[2] + i] - iv.c[idx[3] + i];
        ncdiff += temp * temp;
        nc += iv.c[idx[3] + i] * iv.c[idx[3] + i];
    }
    ncdiff = ncdiff.sqrt();
    nc = nc.sqrt();
    iv.err = ncdiff * 2.0 * h;
    if ncdiff / nc > 0.1 && iv.err < 2.0 * h * nc {
        iv.err = 2.0 * h * nc;
    }

    // Initialize heap
    for i in 0..ws.size {
        ws.heap[i] = i;
    }

    // Initialize global values
    igral = iv.igral;
    err = iv.err;
    nivals = 1;

    // Main loop
    while nivals > 0 && err > 0.0 
        && !(err <= igral.abs() * epsrel || err <= epsabs)
        && !(err_final > igral.abs() * epsrel && err - err_final < igral.abs() * epsrel)
        && !(err_final > epsabs && err - err_final < epsabs)
    {
        iv = &mut ws.ivals[ws.heap[0]];
        m = (iv.a + iv.b) / 2.0;
        h = (iv.b - iv.a) / 2.0;

        // Try to increase degree
        if iv.depth < 3 {
            let d = iv.depth + 1;
            iv.depth = d;

            // Get new function values
            for i in (skip[d]..=32).step_by(2 * skip[d]) {
                iv.fx[i] = f.eval(m + xi[i] * h);
                neval += 1;
            }

            nnans = 0;
            for i in (0..=32).step_by(skip[d]) {
                if !iv.fx[i].is_finite() {
                    nans[nnans] = i as i32;
                    nnans += 1;
                    iv.fx[i] = 0.0;
                }
            }

            // Compute new coefficients
            vinvfx(&iv.fx, &mut iv.c[idx[d]..], d);

            // Downdate NaNs
            if nnans > 0 {
                downdate(&mut iv.c[idx[d]..], n[d] as i32, d as i32, &nans, nnans as i32);
                for i in 0..nnans {
                    iv.fx[nans[i] as usize] = f64::NAN;
                }
            }

            // Compute error estimate
            nc = 0.0;
            for i in (n[d - 1] + 1)..=n[d] {
                temp = iv.c[idx[d] + i];
                nc += temp * temp;
            }
            ncdiff = nc;
            for i in 0..=n[d - 1] {
                temp = iv.c[idx[d - 1] + i] - iv.c[idx[d] + i];
                ncdiff += temp * temp;
                nc += iv.c[idx[d] + i] * iv.c[idx[d] + i];
            }
            ncdiff = ncdiff.sqrt();
            nc = nc.sqrt();
            iv.err = ncdiff * 2.0 * h;
            iv.igral = 2.0 * h * w * iv.c[idx[d]];

            // Check if we should split
            if nc > 0.0 && ncdiff / nc > 0.1 {
                // Split interval
                // ... (rest of the splitting logic)
            }
        } else {
            // Split interval
            // ... (rest of the splitting logic)
        }

        // ... (rest of the main loop logic)
    }

    // Finalize results
    *result = igral;
    if let Some(abserr) = abserr {
        *abserr = err;
    }
    if let Some(nevals) = nevals {
        *nevals = neval;
    }

    Ok(())
}