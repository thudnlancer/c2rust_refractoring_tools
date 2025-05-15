use std::f64;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
    stride: usize,
}

impl Vector {
    pub fn new(size: usize) -> Self {
        Vector {
            data: vec![0.0; size],
            stride: 1,
        }
    }

    pub fn from_vec(data: Vec<f64>) -> Self {
        Vector { data, stride: 1 }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.stride]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.stride]
    }
}

#[derive(Debug, Clone, Copy)]
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

fn solve_tridiag(
    diag: &Vector,
    offdiag: &Vector,
    b: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    let n = diag.size();
    if n != b.size() || n != solution.size() || offdiag.size() != n - 1 {
        return Err(GslError::BadLen);
    }

    let mut gamma = vec![0.0; n];
    let mut alpha = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut z = vec![0.0; n];

    alpha[0] = diag[0];
    gamma[0] = offdiag[0] / alpha[0];
    if alpha[0] == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    for i in 1..n - 1 {
        alpha[i] = diag[i] - offdiag[i - 1] * gamma[i - 1];
        gamma[i] = offdiag[i] / alpha[i];
        if alpha[i] == 0.0 {
            return Err(GslError::ZeroDiv);
        }
    }

    if n > 1 {
        alpha[n - 1] = diag[n - 1] - offdiag[n - 2] * gamma[n - 2];
    }

    z[0] = b[0];
    for i in 1..n {
        z[i] = b[i] - gamma[i - 1] * z[i - 1];
    }

    for i in 0..n {
        c[i] = z[i] / alpha[i];
    }

    solution[n - 1] = c[n - 1];
    if n >= 2 {
        for j in 0..n - 1 {
            let i = n - 2 - j;
            solution[i] = c[i] - gamma[i] * solution[i + 1];
        }
    }

    Ok(())
}

fn solve_tridiag_nonsym(
    diag: &Vector,
    abovediag: &Vector,
    belowdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    let n = diag.size();
    if n != rhs.size()
        || n != solution.size()
        || abovediag.size() != n - 1
        || belowdiag.size() != n - 1
    {
        return Err(GslError::BadLen);
    }

    let mut alpha = vec![0.0; n];
    let mut z = vec![0.0; n];

    alpha[0] = diag[0];
    z[0] = rhs[0];
    if alpha[0] == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    for i in 1..n {
        let t = belowdiag[i - 1] / alpha[i - 1];
        alpha[i] = diag[i] - t * abovediag[i - 1];
        z[i] = rhs[i] - t * z[i - 1];
        if alpha[i] == 0.0 {
            return Err(GslError::ZeroDiv);
        }
    }

    solution[n - 1] = z[n - 1] / alpha[n - 1];
    if n >= 2 {
        for j in 0..n - 1 {
            let i = n - 2 - j;
            solution[i] = (z[i] - abovediag[i] * solution[i + 1]) / alpha[i];
        }
    }

    Ok(())
}

fn solve_cyc_tridiag(
    diag: &Vector,
    offdiag: &Vector,
    b: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    let n = diag.size();
    if n != b.size() || n != solution.size() || offdiag.size() != n {
        return Err(GslError::BadLen);
    }
    if n < 3 {
        return Err(GslError::BadLen);
    }

    let mut delta = vec![0.0; n];
    let mut gamma = vec![0.0; n];
    let mut alpha = vec![0.0; n];
    let mut c = vec![0.0; n];
    let mut z = vec![0.0; n];

    if n == 1 {
        solution[0] = b[0] / diag[0];
        return Ok(());
    }

    alpha[0] = diag[0];
    gamma[0] = offdiag[0] / alpha[0];
    delta[0] = offdiag[n - 1] / alpha[0];
    if alpha[0] == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    for i in 1..n - 2 {
        alpha[i] = diag[i] - offdiag[i - 1] * gamma[i - 1];
        gamma[i] = offdiag[i] / alpha[i];
        delta[i] = -delta[i - 1] * offdiag[i - 1] / alpha[i];
        if alpha[i] == 0.0 {
            return Err(GslError::ZeroDiv);
        }
    }

    let mut sum = 0.0;
    for i in 0..n - 2 {
        sum += alpha[i] * delta[i] * delta[i];
    }

    alpha[n - 2] = diag[n - 2] - offdiag[n - 3] * gamma[n - 3];
    gamma[n - 2] = (offdiag[n - 2] - offdiag[n - 3] * delta[n - 3]) / alpha[n - 2];
    alpha[n - 1] = diag[n - 1] - sum - alpha[n - 2] * gamma[n - 2] * gamma[n - 2];

    z[0] = b[0];
    for i in 1..n - 1 {
        z[i] = b[i] - z[i - 1] * gamma[i - 1];
    }

    sum = 0.0;
    for i in 0..n - 2 {
        sum += delta[i] * z[i];
    }

    z[n - 1] = b[n - 1] - sum - gamma[n - 2] * z[n - 2];

    for i in 0..n {
        c[i] = z[i] / alpha[i];
    }

    solution[n - 1] = c[n - 1];
    solution[n - 2] = c[n - 2] - gamma[n - 2] * solution[n - 1];

    if n >= 3 {
        for j in 0..n - 2 {
            let i = n - 3 - j;
            solution[i] = c[i] - gamma[i] * solution[i + 1] - delta[i] * solution[n - 1];
        }
    }

    Ok(())
}

fn solve_cyc_tridiag_nonsym(
    diag: &Vector,
    abovediag: &Vector,
    belowdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    let n = diag.size();
    if n != rhs.size()
        || n != solution.size()
        || abovediag.size() != n
        || belowdiag.size() != n
    {
        return Err(GslError::BadLen);
    }
    if n < 3 {
        return Err(GslError::BadLen);
    }

    let mut alpha = vec![0.0; n];
    let mut zb = vec![0.0; n];
    let mut zu = vec![0.0; n];
    let mut w = vec![0.0; n];

    zb[0] = rhs[0];
    let mut beta = if diag[0] != 0.0 {
        -diag[0]
    } else {
        1.0
    };

    let q = 1.0 - abovediag[0] * belowdiag[0] / (diag[0] * diag[1]);
    if (q / beta).abs() > 0.5 && (q / beta).abs() < 2.0 {
        beta *= if (q / beta).abs() < 1.0 { 0.5 } else { 2.0 };
    }

    zu[0] = beta;
    alpha[0] = diag[0] - beta;
    if alpha[0] == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    for i in 1..n - 1 {
        let t = belowdiag[i - 1] / alpha[i - 1];
        alpha[i] = diag[i] - t * abovediag[i - 1];
        zb[i] = rhs[i] - t * zb[i - 1];
        zu[i] = -t * zu[i - 1];
        if alpha[i] == 0.0 {
            return Err(GslError::ZeroDiv);
        }
    }

    let i = n - 1;
    let t = belowdiag[i - 1] / alpha[i - 1];
    alpha[i] = diag[i] - abovediag[i] * belowdiag[i] / beta - t * abovediag[i - 1];
    zb[i] = rhs[i] - t * zb[i - 1];
    zu[i] = abovediag[i] - t * zu[i - 1];
    if alpha[i] == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    w[n - 1] = zu[n - 1] / alpha[n - 1];
    solution[n - 1] = zb[n - 1] / alpha[n - 1];

    for j in 0..n - 1 {
        let i = n - 2 - j;
        w[i] = (zu[i] - abovediag[i] * w[i + 1]) / alpha[i];
        solution[i] = (zb[i] - abovediag[i] * solution[i + 1]) / alpha[i];
    }

    let vw = w[0] + belowdiag[n - 1] / beta * w[n - 1];
    let vx = solution[0] + belowdiag[n - 1] / beta * solution[n - 1];

    if vw + 1.0 == 0.0 {
        return Err(GslError::ZeroDiv);
    }

    for i in 0..n {
        solution[i] -= vx / (1.0 + vw) * w[i];
    }

    Ok(())
}

pub fn gsl_linalg_solve_symm_tridiag(
    diag: &Vector,
    offdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    solve_tridiag(diag, offdiag, rhs, solution)
}

pub fn gsl_linalg_solve_tridiag(
    diag: &Vector,
    abovediag: &Vector,
    belowdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    solve_tridiag_nonsym(diag, abovediag, belowdiag, rhs, solution)
}

pub fn gsl_linalg_solve_symm_cyc_tridiag(
    diag: &Vector,
    offdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    solve_cyc_tridiag(diag, offdiag, rhs, solution)
}

pub fn gsl_linalg_solve_cyc_tridiag(
    diag: &Vector,
    abovediag: &Vector,
    belowdiag: &Vector,
    rhs: &Vector,
    solution: &mut Vector,
) -> Result<(), GslError> {
    solve_cyc_tridiag_nonsym(diag, abovediag, belowdiag, rhs, solution)
}