use std::error::Error;
use std::fmt;
use std::f64;

#[derive(Debug)]
enum LinalgError {
    Underdetermined,
    BadLength,
    RankDeficient,
    Singularity,
    NoMemory,
}

impl fmt::Display for LinalgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LinalgError::Underdetermined => write!(f, "System is underdetermined"),
            LinalgError::BadLength => write!(f, "matrix and vector sizes must be equal"),
            LinalgError::RankDeficient => write!(f, "matrix is rank deficient"),
            LinalgError::Singularity => write!(f, "apparent singularity detected"),
            LinalgError::NoMemory => write!(f, "could not allocate memory for workspace"),
        }
    }
}

impl Error for LinalgError {}

type Result<T> = std::result::Result<T, LinalgError>;

struct Matrix {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    fn size1(&self) -> usize {
        self.rows
    }

    fn size2(&self) -> usize {
        self.cols
    }
}

struct Vector {
    data: Vec<f64>,
    size: usize,
}

impl Vector {
    fn new(size: usize) -> Self {
        Vector {
            data: vec![0.0; size],
            size,
        }
    }

    fn get(&self, index: usize) -> f64 {
        self.data[index]
    }

    fn set(&mut self, index: usize, value: f64) {
        self.data[index] = value;
    }

    fn size(&self) -> usize {
        self.size
    }

    fn copy_from(&mut self, other: &Vector) {
        self.data.copy_from_slice(&other.data);
    }
}

fn hh_solve(a: &Matrix, b: &Vector, x: &mut Vector) -> Result<()> {
    if a.size1() > a.size2() {
        Err(LinalgError::Underdetermined)
    } else if a.size2() != x.size() {
        Err(LinalgError::BadLength)
    } else {
        x.copy_from(b);
        hh_svx(a, x)
    }
}

fn hh_svx(a: &Matrix, x: &mut Vector) -> Result<()> {
    if a.size1() > a.size2() {
        return Err(LinalgError::Underdetermined);
    } else if a.size2() != x.size() {
        return Err(LinalgError::BadLength);
    }

    let n = a.size1();
    let m = a.size2();
    let mut d = vec![0.0; n];

    for i in 0..n {
        let aii = a.get(i, i);
        let mut r = 0.0;

        for k in i..m {
            let aki = a.get(k, i);
            r += aki * aki;
        }

        if r == 0.0 {
            return Err(LinalgError::RankDeficient);
        }

        let alpha = r.sqrt() * aii.signum();
        let ak = 1.0 / (r + alpha * aii);
        a.set(i, i, aii + alpha);

        d[i] = -alpha;

        let mut max_norm = 0.0;

        for k in (i + 1)..n {
            let mut norm = 0.0;
            let mut f = 0.0;

            for j in i..m {
                let ajk = a.get(j, k);
                let aji = a.get(j, i);
                norm += ajk * ajk;
                f += ajk * aji;
            }

            max_norm = max_norm.max(norm);
            f *= ak;

            for j in i..m {
                let ajk = a.get(j, k);
                let aji = a.get(j, i);
                a.set(j, k, ajk - f * aji);
            }
        }

        if alpha.abs() < 2.0 * f64::EPSILON * max_norm.sqrt() {
            return Err(LinalgError::Singularity);
        }

        let mut f = 0.0;
        for j in i..m {
            f += x.get(j) * a.get(j, i);
        }
        f *= ak;
        for j in i..m {
            let xj = x.get(j);
            let aji = a.get(j, i);
            x.set(j, xj - f * aji);
        }
    }

    for i in (0..n).rev() {
        let xi = x.get(i);
        let mut sum = 0.0;

        for k in (i + 1)..n {
            sum += a.get(i, k) * x.get(k);
        }

        x.set(i, (xi - sum) / d[i]);
    }

    Ok(())
}