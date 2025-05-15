use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum GslError {
    BadLen,
    Invalid,
    Other(String),
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GslError::BadLen => write!(f, "matrix dimensions do not match"),
            GslError::Invalid => write!(f, "matrix storage formats do not match"),
            GslError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for GslError {}

struct SpMatrix {
    size1: usize,
    size2: usize,
    sptype: usize,
    nzmax: usize,
    nz: usize,
    p: Vec<i32>,
    i: Vec<i32>,
    data: Vec<f64>,
    work_int: Vec<i32>,
    work_atomic: Vec<f64>,
}

impl SpMatrix {
    fn is_ccs(&self) -> bool {
        self.sptype == 1 // Assuming 1 represents CCS format
    }

    fn realloc(&mut self, new_nzmax: usize) -> Result<(), GslError> {
        if new_nzmax <= self.nzmax {
            return Ok(());
        }

        self.nzmax = new_nzmax;
        self.i.resize(self.nzmax, 0);
        self.data.resize(self.nzmax, 0.0);
        Ok(())
    }

    fn scale(&mut self, alpha: f64) {
        for val in self.data.iter_mut() {
            *val *= alpha;
        }
    }
}

fn gsl_spblas_dgemm(
    alpha: f64,
    a: &SpMatrix,
    b: &SpMatrix,
    c: &mut SpMatrix,
) -> Result<(), GslError> {
    if a.size2 != b.size1 || a.size1 != c.size1 || b.size2 != c.size2 {
        return Err(GslError::BadLen);
    } else if a.sptype != b.sptype || a.sptype != c.sptype {
        return Err(GslError::Invalid);
    } else if !a.is_ccs() {
        return Err(GslError::Other("compressed column format required".to_string()));
    }

    let m = a.size1;
    let n = b.size2;
    let bi = &b.i;
    let bp = &b.p;
    let bd = &b.data;
    let w = &mut a.work_int; // workspace of length M
    let x = &mut c.work_atomic; // workspace of length M
    let mut nz = 0;

    if c.nzmax < a.nz + b.nz {
        c.realloc(a.nz + b.nz)?;
    }

    // initialize workspace to 0
    for j in 0..m {
        w[j] = 0;
    }

    let cp = &mut c.p;
    let ci = &mut c.i;
    let cd = &mut c.data;

    for j in 0..n {
        if nz + m > c.nzmax {
            c.realloc(2 * c.nzmax + m)?;
        }

        cp[j] = nz as i32; // column j of C starts here

        for p in bp[j]..bp[j + 1] {
            nz = gsl_spblas_scatter(a, bi[p as usize] as usize, bd[p as usize], w, x, (j + 1) as i32, c, nz)?;
        }

        for p in cp[j]..nz as i32 {
            cd[p as usize] = x[ci[p as usize] as usize];
        }
    }

    cp[n] = nz as i32;
    c.nz = nz;

    // scale by alpha
    c.scale(alpha);

    Ok(())
}

fn gsl_spblas_scatter(
    a: &SpMatrix,
    j: usize,
    alpha: f64,
    w: &mut [i32],
    x: &mut [f64],
    mark: i32,
    c: &mut SpMatrix,
    mut nz: usize,
) -> Result<usize, GslError> {
    let ai = &a.i;
    let ap = &a.p;
    let ad = &a.data;
    let ci = &mut c.i;

    for p in ap[j]..ap[j + 1] {
        let i = ai[p as usize] as usize; // A(i,j) is nonzero

        if w[i] < mark {
            // check if row i has been stored in column j yet
            w[i] = mark; // i is new entry in column j
            ci[nz] = i as i32; // add i to pattern of C(:,j)
            x[i] = alpha * ad[p as usize]; // x(i) = alpha * A(i,j)
            nz += 1;
        } else {
            // this (i,j) exists in C from a previous call
            x[i] += alpha * ad[p as usize]; // add alpha*A(i,j) to C(i,j)
        }
    }

    Ok(nz)
}