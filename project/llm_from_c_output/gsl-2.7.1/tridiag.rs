use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub enum TridiagError {
    ZeroDivision,
    MemoryAllocation,
    InvalidLength,
    Other(&'static str),
}

impl fmt::Display for TridiagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TridiagError::ZeroDivision => write!(f, "matrix must be positive definite"),
            TridiagError::MemoryAllocation => write!(f, "failed to allocate working space"),
            TridiagError::InvalidLength => write!(f, "invalid vector lengths"),
            TridiagError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for TridiagError {}

type Result<T> = std::result::Result<T, TridiagError>;

fn solve_tridiag(
    diag: &[f64],
    d_stride: usize,
    offdiag: &[f64],
    o_stride: usize,
    b: &[f64],
    b_stride: usize,
    x: &mut [f64],
    x_stride: usize,
    n: usize,
) -> Result<()> {
    let mut gamma = vec![0.0; n];
    let mut alpha = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut z = vec![0.0; n];

    // Cholesky decomposition
    alpha[0] = diag[0];
    if alpha[0] == 0.0 {
        return Err(TridiagError::ZeroDivision);
    }
    gamma[0] = offdiag[0] / alpha[0];

    for i in 1..n - 1 {
        alpha[i] = diag[d_stride * i] - offdiag[o_stride * (i - 1)] * gamma[i - 1];
        if alpha[i] == 0.0 {
            return Err(TridiagError::ZeroDivision);
        }
        gamma[i] = offdiag[o_stride * i] / alpha[i];
    }

    if n > 1 {
        alpha[n - 1] = diag[d_stride * (n - 1)] - offdiag[o_stride * (n - 2)] * gamma[n - 2];
    }

    // Update RHS
    z[0] = b[0];
    for i in 1..n {
        z[i] = b[b_stride * i] - gamma[i - 1] * z[i - 1];
    }

    for i in 0..n {
        c[i] = z[i] / alpha[i];
    }

    // Backsubstitution
    x[x_stride * (n - 1)] = c[n - 1];
    if n >= 2 {
        for j in 0..=n - 2 {
            let i = n - 2 - j;
            x[x_stride * i] = c[i] - gamma[i] * x[x_stride * (i + 1)];
        }
    }

    Ok(())
}

fn solve_tridiag_nonsym(
    diag: &[f64],
    d_stride: usize,
    abovediag: &[f64],
    a_stride: usize,
    belowdiag: &[f64],
    b_stride: usize,
    rhs: &[f64],
    r_stride: usize,
    x: &mut [f64],
    x_stride: usize,
    n: usize,
) -> Result<()> {
    let mut alpha = vec![0.0; n];
    let mut z = vec![0.0; n];

    alpha[0] = diag[0];
    if alpha[0] == 0.0 {
        return Err(TridiagError::ZeroDivision);
    }
    z[0] = rhs[0];

    for i in 1..n {
        let t = belowdiag[b_stride * (i - 1)] / alpha[i - 1];
        alpha[i] = diag[d_stride * i] - t * abovediag[a_stride * (i - 1)];
        z[i] = rhs[r_stride * i] - t * z[i - 1];
        if alpha[i] == 0.0 {
            return Err(TridiagError::ZeroDivision);
        }
    }

    // Backsubstitution
    x[x_stride * (n - 1)] = z[n - 1] / alpha[n - 1];
    if n >= 2 {
        for j in 0..=n - 2 {
            let i = n - 2 - j;
            x[x_stride * i] = (z[i] - abovediag[a_stride * i] * x[x_stride * (i + 1)]) / alpha[i];
        }
    }

    Ok(())
}

fn solve_cyc_tridiag(
    diag: &[f64],
    d_stride: usize,
    offdiag: &[f64],
    o_stride: usize,
    b: &[f64],
    b_stride: usize,
    x: &mut [f64],
    x_stride: usize,
    n: usize,
) -> Result<()> {
    if n == 1 {
        x[0] = b[0] / diag[0];
        return Ok(());
    }

    let mut delta = vec![0.0; n];
    let mut gamma = vec![0.0; n];
    let mut alpha = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut z = vec![0.0; n];

    alpha[0] = diag[0];
    if alpha[0] == 0.0 {
        return Err(TridiagError::ZeroDivision);
    }
    gamma[0] = offdiag[0] / alpha[0];
    delta[0] = offdiag[o_stride * (n - 1)] / alpha[0];

    let mut sum = 0.0;
    for i in 1..n - 2 {
        alpha[i] = diag[d_stride * i] - offdiag[o_stride * (i - 1)] * gamma[i - 1];
        if alpha[i] == 0.0 {
            return Err(TridiagError::ZeroDivision);
        }
        gamma[i] = offdiag[o_stride * i] / alpha[i];
        delta[i] = -delta[i - 1] * offdiag[o_stride * (i - 1)] / alpha[i];
    }

    for i in 0..n - 2 {
        sum += alpha[i] * delta[i] * delta[i];
    }

    alpha[n - 2] = diag[d_stride * (n - 2)] - offdiag[o_stride * (n - 3)] * gamma[n - 3];
    gamma[n - 2] = (offdiag[o_stride * (n - 2)] - offdiag[o_stride * (n - 3)] * delta[n - 3]) / alpha[n - 2];
    alpha[n - 1] = diag[d_stride * (n - 1)] - sum - alpha[n - 2] * gamma[n - 2] * gamma[n - 2];

    // Update
    z[0] = b[0];
    for i in 1..n - 1 {
        z[i] = b[b_stride * i] - z[i - 1] * gamma[i - 1];
    }

    sum = 0.0;
    for i in 0..n - 2 {
        sum += delta[i] * z[i];
    }
    z[n - 1] = b[b_stride * (n - 1)] - sum - gamma[n - 2] * z[n - 2];

    for i in 0..n {
        c[i] = z[i] / alpha[i];
    }

    // Backsubstitution
    x[x_stride * (n - 1)] = c[n - 1];
    x[x_stride * (n - 2)] = c[n - 2] - gamma[n - 2] * x[x_stride * (n - 1)];
    if n >= 3 {
        for j in 0..=n - 3 {
            let i = n - 3 - j;
            x[x_stride * i] = c[i] - gamma[i] * x[x_stride * (i + 1)] - delta[i] * x[x_stride * (n - 1)];
        }
    }

    Ok(())
}

fn solve_cyc_tridiag_nonsym(
    diag: &[f64],
    d_stride: usize,
    abovediag: &[f64],
    a_stride: usize,
    belowdiag: &[f64],
    b_stride: usize,
    rhs: &[f64],
    r_stride: usize,
    x: &mut [f64],
    x_stride: usize,
    n: usize,
) -> Result<()> {
    let mut alpha = vec![0.0; n];
    let mut zb = vec![0.0; n];
    let mut zu = vec![0.0; n];
    let mut w = vec![0.0; n];

    zb[0] = rhs[0];
    let mut beta = if diag[0] != 0.0 { -diag[0] } else { 1.0 };
    {
        let q = 1.0 - abovediag[0] * belowdiag[0] / (diag[0] * diag[d_stride]);
        let ratio = (q / beta).abs();
        if ratio > 0.5 && ratio < 2.0 {
            beta *= if ratio < 1.0 { 0.5 } else { 2.0 };
        }
    }
    zu[0] = beta;
    alpha[0] = diag[0] - beta;

    if alpha[0] == 0.0 {
        return Err(TridiagError::ZeroDivision);
    }

    for i in 1..n - 1 {
        let t = belowdiag[b_stride * (i - 1)] / alpha[i - 1];
        alpha[i] = diag[d_stride * i] - t * abovediag[a_stride * (i - 1)];
        zb[i] = rhs[r_stride * i] - t * zb[i - 1];
        zu[i] = -t * zu[i - 1];
        if alpha[i] == 0.0 {
            return Err(TridiagError::ZeroDivision);
        }
    }

    {
        let i = n - 1;
        let t = belowdiag[b_stride * (i - 1)] / alpha[i - 1];
        alpha[i] = diag[d_stride * i]
            - abovediag[a_stride * i] * belowdiag[b_stride * i] / beta
            - t * abovediag[a_stride * (i - 1)];
        zb[i] = rhs[r_stride * i] - t * zb[i - 1];
        zu[i] = abovediag[a_stride * i] - t * zu[i - 1];
        if alpha[i] == 0.0 {
            return Err(TridiagError::ZeroDivision);
        }
    }

    // Backsubstitution
    w[n - 1] = zu[n - 1] / alpha[n - 1];
    x[x_stride * (n - 1)] = zb[n - 1] / alpha[n - 1];
    for j in 0..=n - 2 {
        let i = n - 2 - j;
        w[i] = (zu[i] - abovediag[a_stride * i] * w[i + 1]) / alpha[i];
        x[x_stride * i] = (zb[i] - abovediag[a_stride * i] * x[x_stride * (i + 1)]) / alpha[i];
    }

    // Sherman-Morrison
    let vw = w[0] + belowdiag[b_stride * (n - 1)] / beta * w[n - 1];
    let vx = x[0] + belowdiag[b_stride * (n - 1)] / beta * x[x_stride * (n - 1)];
    if vw + 1.0 == 0.0 {
        return Err(TridiagError::ZeroDivision);
    }

    for i in 0..n {
        x[x_stride * i] -= vx / (1.0 + vw) * w[i];
    }

    Ok(())
}

pub fn solve_symm_tridiag(
    diag: &[f64],
    offdiag: &[f64],
    b: &[f64],
    solution: &mut [f64],
) -> Result<()> {
    if diag.len() != b.len() {
        return Err(TridiagError::InvalidLength);
    } else if offdiag.len() != b.len() - 1 {
        return Err(TridiagError::InvalidLength);
    } else if solution.len() != b.len() {
        return Err(TridiagError::InvalidLength);
    }

    solve_tridiag(
        diag,
        1,
        offdiag,
        1,
        b,
        1,
        solution,
        1,
        diag.len(),
    )
}

pub fn solve_tridiagonal(
    diag: &[f64],
    abovediag: &[f64],
    belowdiag: &[f64],
    rhs: &[f64],
    solution: &mut [f64],
) -> Result<()> {
    if diag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if abovediag.len() != rhs.len() - 1 {
        return Err(TridiagError::InvalidLength);
    } else if belowdiag.len() != rhs.len() - 1 {
        return Err(TridiagError::InvalidLength);
    } else if solution.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    }

    solve_tridiag_nonsym(
        diag,
        1,
        abovediag,
        1,
        belowdiag,
        1,
        rhs,
        1,
        solution,
        1,
        diag.len(),
    )
}

pub fn solve_symm_cyc_tridiag(
    diag: &[f64],
    offdiag: &[f64],
    rhs: &[f64],
    solution: &mut [f64],
) -> Result<()> {
    if diag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if offdiag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if solution.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if diag.len() < 3 {
        return Err(TridiagError::InvalidLength);
    }

    solve_cyc_tridiag(
        diag,
        1,
        offdiag,
        1,
        rhs,
        1,
        solution,
        1,
        diag.len(),
    )
}

pub fn solve_cyc_tridiagonal(
    diag: &[f64],
    abovediag: &[f64],
    belowdiag: &[f64],
    rhs: &[f64],
    solution: &mut [f64],
) -> Result<()> {
    if diag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if abovediag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if belowdiag.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if solution.len() != rhs.len() {
        return Err(TridiagError::InvalidLength);
    } else if diag.len() < 3 {
        return Err(TridiagError::InvalidLength);
    }

    solve_cyc_tridiag_nonsym(
        diag,
        1,
        abovediag,
        1,
        belowdiag,
        1,
        rhs,
        1,
        solution,
        1,
        diag.len(),
    )
}