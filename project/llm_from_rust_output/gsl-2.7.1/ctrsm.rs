use std::f64;
use std::f32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_SIDE {
    Left = 141,
    Right = 142,
}

fn xhypot(x: f64, y: f64) -> f64 {
    let xabs = x.abs();
    let yabs = y.abs();
    let (min, max) = if xabs < yabs {
        (xabs, yabs)
    } else {
        (yabs, xabs)
    };

    if min == 0.0 {
        return max;
    }

    let u = min / max;
    max * (1.0 + u * u).sqrt()
}

#[derive(Debug, Clone, Copy)]
struct Complex32 {
    real: f32,
    imag: f32,
}

impl Complex32 {
    fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    fn scale(&self, alpha: Complex32) -> Self {
        Self {
            real: alpha.real * self.real - alpha.imag * self.imag,
            imag: alpha.real * self.imag + alpha.imag * self.real,
        }
    }

    fn conj(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    fn norm(&self) -> f32 {
        xhypot(self.real as f64, self.imag as f64) as f32
    }

    fn normalize(&self) -> Self {
        let s = self.norm();
        Self {
            real: self.real / s,
            imag: self.imag / s,
        }
    }
}

impl std::ops::Add for Complex32 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl std::ops::Sub for Complex32 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl std::ops::Mul for Complex32 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl std::ops::Div<f32> for Complex32 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            real: self.real / rhs,
            imag: self.imag / rhs,
        }
    }
}

impl std::ops::Neg for Complex32 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

struct Matrix<'a> {
    data: &'a [Complex32],
    rows: usize,
    cols: usize,
    stride: usize,
    order: CBLAS_ORDER,
}

impl<'a> Matrix<'a> {
    fn new(data: &'a [Complex32], rows: usize, cols: usize, stride: usize, order: CBLAS_ORDER) -> Self {
        Self {
            data,
            rows,
            cols,
            stride,
            order,
        }
    }

    fn get(&self, i: usize, j: usize) -> Complex32 {
        let idx = match self.order {
            CBLAS_ORDER::RowMajor => i * self.stride + j,
            CBLAS_ORDER::ColMajor => j * self.stride + i,
        };
        self.data[idx]
    }
}

struct MatrixMut<'a> {
    data: &'a mut [Complex32],
    rows: usize,
    cols: usize,
    stride: usize,
    order: CBLAS_ORDER,
}

impl<'a> MatrixMut<'a> {
    fn new(
        data: &'a mut [Complex32],
        rows: usize,
        cols: usize,
        stride: usize,
        order: CBLAS_ORDER,
    ) -> Self {
        Self {
            data,
            rows,
            cols,
            stride,
            order,
        }
    }

    fn get(&self, i: usize, j: usize) -> Complex32 {
        let idx = match self.order {
            CBLAS_ORDER::RowMajor => i * self.stride + j,
            CBLAS_ORDER::ColMajor => j * self.stride + i,
        };
        self.data[idx]
    }

    fn set(&mut self, i: usize, j: usize, value: Complex32) {
        let idx = match self.order {
            CBLAS_ORDER::RowMajor => i * self.stride + j,
            CBLAS_ORDER::ColMajor => j * self.stride + i,
        };
        self.data[idx] = value;
    }
}

fn cblas_ctrsm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: Complex32,
    a: &[Complex32],
    lda: i32,
    b: &mut [Complex32],
    ldb: i32,
) {
    let m = m as usize;
    let n = n as usize;
    let lda = lda as usize;
    let ldb = ldb as usize;

    let nonunit = diag == CBLAS_DIAG::NonUnit;
    let conj = if trans_a == CBLAS_TRANSPOSE::ConjTrans {
        -1.0
    } else {
        1.0
    };

    let (n1, n2, side, uplo, trans) = if order == CBLAS_ORDER::RowMajor {
        (
            m,
            n,
            side,
            uplo,
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        )
    } else {
        (
            n,
            m,
            if side == CBLAS_SIDE::Left {
                CBLAS_SIDE::Right
            } else {
                CBLAS_SIDE::Left
            },
            if uplo == CBLAS_UPLO::Upper {
                CBLAS_UPLO::Lower
            } else {
                CBLAS_UPLO::Upper
            },
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        )
    };

    let a_matrix = Matrix::new(a, n1, n1, lda, order);
    let mut b_matrix = MatrixMut::new(b, n1, n2, ldb, order);

    if side == CBLAS_SIDE::Left && uplo == CBLAS_UPLO::Upper && trans == CBLAS_TRANSPOSE::NoTrans {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in (0..n1).rev() {
            if nonunit {
                let aii = a_matrix.get(i, i);
                let s = aii.norm();
                let a_norm = aii.normalize();
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }
            }

            for k in 0..i {
                let aki = a_matrix.get(k, i).scale(Complex32::new(1.0, conj));
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let bkj = b_matrix.get(k, j);
                    b_matrix.set(k, j, bkj - aki * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Left && uplo == CBLAS_UPLO::Upper && trans == CBLAS_TRANSPOSE::Trans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            if nonunit {
                let aii = a_matrix.get(i, i);
                let s = aii.norm();
                let a_norm = aii.normalize();
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }
            }

            for k in i + 1..n1 {
                let aik = a_matrix.get(i, k).scale(Complex32::new(1.0, conj));
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let bkj = b_matrix.get(k, j);
                    b_matrix.set(k, j, bkj - aik * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Left && uplo == CBLAS_UPLO::Lower && trans == CBLAS_TRANSPOSE::NoTrans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            if nonunit {
                let aii = a_matrix.get(i, i);
                let s = aii.norm();
                let a_norm = aii.normalize();
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }
            }

            for k in i + 1..n1 {
                let aki = a_matrix.get(k, i).scale(Complex32::new(1.0, conj));
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let bkj = b_matrix.get(k, j);
                    b_matrix.set(k, j, bkj - aki * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Left && uplo == CBLAS_UPLO::Lower && trans == CBLAS_TRANSPOSE::Trans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in (0..n1).rev() {
            if nonunit {
                let aii = a_matrix.get(i, i);
                let s = aii.norm();
                let a_norm = aii.normalize();
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }
            }

            for k in 0..i {
                let aik = a_matrix.get(i, k).scale(Complex32::new(1.0, conj));
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    let bkj = b_matrix.get(k, j);
                    b_matrix.set(k, j, bkj - aik * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Right && uplo == CBLAS_UPLO::Upper && trans == CBLAS_TRANSPOSE::NoTrans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            for j in 0..n2 {
                if nonunit {
                    let ajj = a_matrix.get(j, j);
                    let s = ajj.norm();
                    let a_norm = ajj.normalize();
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }

                let bij = b_matrix.get(i, j);
                for k in j + 1..n2 {
                    let ajk = a_matrix.get(j, k).scale(Complex32::new(1.0, conj));
                    let bik = b_matrix.get(i, k);
                    b_matrix.set(i, k, bik - ajk * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Right && uplo == CBLAS_UPLO::Upper && trans == CBLAS_TRANSPOSE::Trans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            for j in (0..n2).rev() {
                if nonunit {
                    let ajj = a_matrix.get(j, j);
                    let s = ajj.norm();
                    let a_norm = ajj.normalize();
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }

                let bij = b_matrix.get(i, j);
                for k in 0..j {
                    let akj = a_matrix.get(k, j).scale(Complex32::new(1.0, conj));
                    let bik = b_matrix.get(i, k);
                    b_matrix.set(i, k, bik - akj * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Right && uplo == CBLAS_UPLO::Lower && trans == CBLAS_TRANSPOSE::NoTrans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            for j in (0..n2).rev() {
                if nonunit {
                    let ajj = a_matrix.get(j, j);
                    let s = ajj.norm();
                    let a_norm = ajj.normalize();
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }

                let bij = b_matrix.get(i, j);
                for k in 0..j {
                    let ajk = a_matrix.get(j, k).scale(Complex32::new(1.0, conj));
                    let bik = b_matrix.get(i, k);
                    b_matrix.set(i, k, bik - ajk * bij);
                }
            }
        }
    } else if side == CBLAS_SIDE::Right && uplo == CBLAS_UPLO::Lower && trans == CBLAS_TRANSPOSE::Trans
    {
        if alpha.real != 1.0 || alpha.imag != 0.0 {
            for i in 0..n1 {
                for j in 0..n2 {
                    let bij = b_matrix.get(i, j);
                    b_matrix.set(i, j, bij.scale(alpha));
                }
            }
        }

        for i in 0..n1 {
            for j in 0..n2 {
                if nonunit {
                    let ajj = a_matrix.get(j, j);
                    let s = ajj.norm();
                    let a_norm = ajj.normalize();
                    let bij = b_matrix.get(i, j);
                    let new_bij = (bij * a_norm) / s;
                    b_matrix.set(i, j, new_bij);
                }

                let bij = b_matrix.get(i, j);
                for k in j + 1..n2 {
