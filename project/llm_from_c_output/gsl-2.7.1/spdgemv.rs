use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum SpBlasError {
    InvalidLength,
    UnsupportedMatrixType,
}

impl fmt::Display for SpBlasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpBlasError::InvalidLength => write!(f, "invalid vector length"),
            SpBlasError::UnsupportedMatrixType => write!(f, "unsupported matrix type"),
        }
    }
}

impl Error for SpBlasError {}

#[derive(Debug, PartialEq)]
enum CblasTranspose {
    NoTrans,
    Trans,
}

#[derive(Debug)]
struct SparseMatrix {
    size1: usize,
    size2: usize,
    data: Vec<f64>,
    p: Vec<usize>,
    i: Vec<usize>,
    format: MatrixFormat,
    nz: usize,
}

#[derive(Debug)]
enum MatrixFormat {
    CCS,
    CRS,
    Triplet,
}

impl SparseMatrix {
    fn is_ccs(&self) -> bool {
        matches!(self.format, MatrixFormat::CCS)
    }

    fn is_crs(&self) -> bool {
        matches!(self.format, MatrixFormat::CRS)
    }

    fn is_triplet(&self) -> bool {
        matches!(self.format, MatrixFormat::Triplet)
    }
}

#[derive(Debug)]
struct Vector {
    data: Vec<f64>,
    stride: usize,
}

impl Vector {
    fn size(&self) -> usize {
        self.data.len() / self.stride
    }
}

fn spblas_dgemv(
    trans_a: CblasTranspose,
    alpha: f64,
    a: &SparseMatrix,
    x: &Vector,
    beta: f64,
    y: &mut Vector,
) -> Result<(), SpBlasError> {
    let m = a.size1;
    let n = a.size2;

    if (trans_a == CblasTranspose::NoTrans && n != x.size())
        || (trans_a == CblasTranspose::Trans && m != x.size())
    {
        return Err(SpBlasError::InvalidLength);
    } else if (trans_a == CblasTranspose::NoTrans && m != y.size())
        || (trans_a == CblasTranspose::Trans && n != y.size())
    {
        return Err(SpBlasError::InvalidLength);
    } else {
        let (len_x, len_y) = match trans_a {
            CblasTranspose::NoTrans => (n, m),
            CblasTranspose::Trans => (m, n),
        };

        // y := beta * y
        if beta == 0.0 {
            for j in 0..len_y {
                y.data[j * y.stride] = 0.0;
            }
        } else if beta != 1.0 {
            for j in 0..len_y {
                y.data[j * y.stride] *= beta;
            }
        }

        if alpha == 0.0 {
            return Ok(());
        }

        // y := alpha * op(A) * x + y
        if (a.is_ccs() && trans_a == CblasTranspose::NoTrans)
            || (a.is_crs() && trans_a == CblasTranspose::Trans)
        {
            for j in 0..len_x {
                for p in a.p[j]..a.p[j + 1] {
                    y.data[a.i[p] * y.stride] += alpha * a.data[p] * x.data[j * x.stride];
                }
            }
        } else if (a.is_ccs() && trans_a == CblasTranspose::Trans)
            || (a.is_crs() && trans_a == CblasTranspose::NoTrans)
        {
            for j in 0..len_y {
                for p in a.p[j]..a.p[j + 1] {
                    y.data[j * y.stride] += alpha * a.data[p] * x.data[a.i[p] * x.stride];
                }
            }
        } else if a.is_triplet() {
            let (ai, aj) = match trans_a {
                CblasTranspose::NoTrans => (&a.i, &a.p),
                CblasTranspose::Trans => (&a.p, &a.i),
            };

            for p in 0..a.nz {
                y.data[ai[p] * y.stride] += alpha * a.data[p] * x.data[aj[p] * x.stride];
            }
        } else {
            return Err(SpBlasError::UnsupportedMatrixType);
        }

        Ok(())
    }
}