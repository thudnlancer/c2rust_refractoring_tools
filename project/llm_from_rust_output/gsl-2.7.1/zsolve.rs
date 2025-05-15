use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct GslPolyComplexWorkspace {
    nc: usize,
    matrix: Vec<f64>,
}

pub type GslComplexPackedPtr = *mut f64;

fn set_companion_matrix(a: &[f64], nc: usize, m: &mut [f64]) {
    let n = nc;
    for i in 0..n {
        for j in 0..n {
            m[i * n + j] = 0.0;
        }
    }
    
    for i in 1..n {
        m[i * n + (i - 1)] = 1.0;
    }
    
    for i in 0..n {
        m[i * n + (n - 1)] = -a[i] / a[n];
    }
}

fn balance_companion_matrix(m: &mut [f64], nc: usize) {
    let n = nc;
    let mut not_converged = true;
    
    while not_converged {
        not_converged = false;
        
        for i in 0..n {
            let col_norm = if i != n - 1 {
                f64::abs(m[(i + 1) * n + i])
            } else {
                (0..n - 1).map(|j| f64::abs(m[j * n + (n - 1)])).sum()
            };
            
            let row_norm = if i == 0 {
                f64::abs(m[0 * n + (n - 1)])
            } else if i == n - 1 {
                f64::abs(m[i * n + (i - 1)])
            } else {
                f64::abs(m[i * n + (i - 1)]) + f64::abs(m[i * n + (n - 1)])
            };
            
            if col_norm != 0.0 && row_norm != 0.0 {
                let mut g = row_norm / 2.0;
                let mut f = 1.0;
                let s = col_norm + row_norm;
                
                while col_norm < g {
                    f *= 2.0;
                    g /= 2.0;
                }
                
                g = row_norm * 2.0;
                
                while col_norm > g {
                    f /= 2.0;
                    g *= 2.0;
                }
                
                if row_norm + col_norm < 0.95 * s * f {
                    not_converged = true;
                    g = 1.0 / f;
                    
                    if i == 0 {
                        m[0 * n + (n - 1)] *= g;
                    } else {
                        m[i * n + (i - 1)] *= g;
                        m[i * n + (n - 1)] *= g;
                    }
                    
                    if i == n - 1 {
                        for j in 0..n {
                            m[j * n + i] *= f;
                        }
                    } else {
                        m[(i + 1) * n + i] *= f;
                    }
                }
            }
        }
    }
}

fn qr_companion(h: &mut [f64], nc: usize, zroot: &mut [f64]) -> GslError {
    let n = nc;
    let mut t = 0.0;
    
    loop {
        if n == 0 {
            return GslError::Success;
        }
        
        let mut iterations = 0;
        
        loop {
            let mut e = n;
            while e >= 2 {
                let a1 = f64::abs(h[(e - 1) * n + (e - 2)]);
                let a2 = f64::abs(h[(e - 2) * n + (e - 2)]);
                let a3 = f64::abs(h[(e - 1) * n + (e - 1)]);
                
                if a1 <= 2.2204460492503131e-16 * (a2 + a3) {
                    break;
                }
                e -= 1;
            }
            
            let x = h[(n - 1) * n + (n - 1)];
            
            if e == n {
                zroot[2 * (n - 1)] = x + t;
                zroot[2 * (n - 1) + 1] = 0.0;
                return GslError::Success;
            } else {
                let y = h[(n - 2) * n + (n - 2)];
                let w = h[(n - 2) * n + (n - 1)] * h[(n - 1) * n + (n - 2)];
                
                if e == n - 1 {
                    let p = (y - x) / 2.0;
                    let q = p * p + w;
                    let y = f64::sqrt(f64::abs(q));
                    let x = x + t;
                    
                    if q > 0.0 {
                        let y = if p < 0.0 { -y } else { y } + p;
                        zroot[2 * (n - 1)] = x - w / y;
                        zroot[2 * (n - 1) + 1] = 0.0;
                        zroot[2 * (n - 2)] = x + y;
                        zroot[2 * (n - 2) + 1] = 0.0;
                    } else {
                        zroot[2 * (n - 1)] = x + p;
                        zroot[2 * (n - 1) + 1] = -y;
                        zroot[2 * (n - 2)] = x + p;
                        zroot[2 * (n - 2) + 1] = y;
                    }
                    
                    return GslError::Success;
                }
                
                if iterations == 120 {
                    return GslError::Failure;
                }
                
                if iterations % 10 == 0 && iterations > 0 {
                    t += x;
                    for i in 1..=n {
                        h[(i - 1) * n + (i - 1)] -= x;
                    }
                    
                    let s = f64::abs(h[(n - 1) * n + (n - 2)]) + 
                           f64::abs(h[(n - 2) * n + (n - 3)]);
                    let y = 0.75 * s;
                    let x = y;
                    let w = -0.4375 * s * s;
                }
                
                iterations += 1;
                
                let mut m = n - 2;
                while m >= e {
                    let z = h[(m - 1) * n + (m - 1)];
                    let r = x - z;
                    let s = y - z;
                    let mut p = h[(m - 1) * n + m] + (r * s - w) / h[m * n + (m - 1)];
                    let mut q = h[m * n + m] - z - r - s;
                    let r = if m < n - 2 { h[(m + 2) * n + m] } else { 0.0 };
                    
                    let s = f64::abs(p) + f64::abs(q) + f64::abs(r);
                    p /= s;
                    q /= s;
                    r /= s;
                    
                    if m == e {
                        break;
                    }
                    
                    let a1 = f64::abs(h[(m - 1) * n + (m - 2)]);
                    let a2 = f64::abs(h[(m - 2) * n + (m - 2)]);
                    let a3 = f64::abs(h[m * n + m]);
                    
                    if a1 * (f64::abs(q) + f64::abs(r)) <= 2.2204460492503131e-16 * f64::abs(p) * (a2 + a3) {
                        break;
                    }
                    
                    m -= 1;
                }
                
                for i in m + 2..=n {
                    h[(i - 1) * n + (i - 3)] = 0.0;
                }
                
                for i in m + 3..=n {
                    h[(i - 1) * n + (i - 4)] = 0.0;
                }
                
                for k in m..n {
                    let notlast = k != n - 1;
                    let (mut p, mut q, mut r);
                    
                    if k != m {
                        p = h[(k - 1) * n + (k - 2)];
                        q = h[k * n + (k - 2)];
                        r = if notlast { h[(k + 1) * n + (k - 2)] } else { 0.0 };
                    } else {
                        p = h[(k - 1) * n + (k - 1)];
                        q = h[k * n + (k - 1)];
                        r = if notlast { h[(k + 1) * n + (k - 1)] } else { 0.0 };
                    }
                    
                    let x = f64::abs(p) + f64::abs(q) + f64::abs(r);
                    if x == 0.0 {
                        continue;
                    }
                    
                    p /= x;
                    q /= x;
                    r /= x;
                    
                    let mut s = f64::sqrt(p * p + q * q + r * r);
                    if p < 0.0 {
                        s = -s;
                    }
                    
                    if k != m {
                        h[(k - 1) * n + (k - 2)] = -s * x;
                    } else if e != m {
                        h[(k - 1) * n + (k - 2)] *= -1.0;
                    }
                    
                    p += s;
                    let x = p / s;
                    let y = q / s;
                    let z = r / s;
                    let q = q / p;
                    let r = r / p;
                    
                    for j in k..n {
                        let mut p = h[(k - 1) * n + j] + q * h[k * n + j];
                        if notlast {
                            p += r * h[(k + 1) * n + j];
                            h[(k + 1) * n + j] -= p * z;
                        }
                        h[k * n + j] -= p * y;
                        h[(k - 1) * n + j] -= p * x;
                    }
                    
                    let j = std::cmp::min(k + 3, n);
                    for i in e..=j {
                        let mut p = x * h[i * n + (k - 1)] + y * h[i * n + k];
                        if notlast {
                            p += z * h[i * n + (k + 1)];
                            h[i * n + (k + 1)] -= p * r;
                        }
                        h[i * n + k] -= p * q;
                        h[i * n + (k - 1)] -= p;
                    }
                }
            }
        }
    }
}

pub fn gsl_poly_complex_solve(
    a: &[f64],
    n: usize,
    w: &mut GslPolyComplexWorkspace,
    z: &mut [f64],
) -> GslError {
    if n == 0 {
        return GslError::Invalid;
    }
    
    if n == 1 {
        return GslError::Invalid;
    }
    
    if a[n - 1] == 0.0 {
        return GslError::Invalid;
    }
    
    if w.nc != n - 1 {
        return GslError::Invalid;
    }
    
    let m = &mut w.matrix;
    set_companion_matrix(a, n - 1, m);
    balance_companion_matrix(m, n - 1);
    let status = qr_companion(m, n - 1, z);
    
    if status != GslError::Success {
        return GslError::Failed;
    }
    
    GslError::Success
}