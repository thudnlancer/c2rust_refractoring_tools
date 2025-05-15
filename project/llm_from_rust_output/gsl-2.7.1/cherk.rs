use num_traits::Zero;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    fn zero() -> Self {
        Self {
            real: 0.0,
            imag: 0.0,
        }
    }

    fn scale(&self, alpha: f32) -> Self {
        Self {
            real: self.real * alpha,
            imag: self.imag * alpha,
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imag == 0.0
    }
}

pub fn cblas_cherk(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    n: i32,
    k: i32,
    alpha: f32,
    a: &[Complex],
    lda: i32,
    beta: f32,
    c: &mut [Complex],
    ldc: i32,
) {
    assert!(n >= 0, "n must be non-negative");
    assert!(k >= 0, "k must be non-negative");
    assert!(lda >= 1.max(if trans == CBLAS_TRANSPOSE::NoTrans { n } else { k }),
        "lda must be >= max(1, dimA)");
    assert!(ldc >= 1.max(n), "ldc must be >= max(1, n)");

    if beta == 1.0 && (alpha == 0.0 || k == 0) {
        return;
    }

    let (uplo, trans) = if order == CBLAS_ORDER::RowMajor {
        (uplo, trans)
    } else {
        (
            match uplo {
                CBLAS_UPLO::Upper => CBLAS_UPLO::Lower,
                CBLAS_UPLO::Lower => CBLAS_UPLO::Upper,
            },
            match trans {
                CBLAS_TRANSPOSE::NoTrans => CBLAS_TRANSPOSE::ConjTrans,
                _ => CBLAS_TRANSPOSE::NoTrans,
            },
        )
    };

    // Initialize C matrix
    if beta == 0.0 {
        for i in 0..n {
            let start = if uplo == CBLAS_UPLO::Upper { i } else { 0 };
            let end = if uplo == CBLAS_UPLO::Upper { n } else { i + 1 };
            for j in start..end {
                let idx = (i * ldc + j) as usize;
                c[idx] = Complex::zero();
            }
        }
    } else if beta != 1.0 {
        for i in 0..n {
            let start = if uplo == CBLAS_UPLO::Upper { i } else { 0 };
            let end = if uplo == CBLAS_UPLO::Upper { n } else { i + 1 };
            for j in start..end {
                let idx = (i * ldc + j) as usize;
                c[idx] = c[idx].scale(beta);
                if i == j {
                    c[idx].imag = 0.0;
                }
            }
        }
    } else {
        for i in 0..n {
            let idx = (i * ldc + i) as usize;
            c[idx].imag = 0.0;
        }
    }

    if alpha == 0.0 {
        return;
    }

    match (uplo, trans) {
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex::zero();
                    for k in 0..k {
                        let aik_idx = (i * lda + k) as usize;
                        let ajk_idx = (j * lda + k) as usize;
                        let aik = a[aik_idx];
                        let ajk = Complex::new(a[ajk_idx].real, -a[ajk_idx].imag);
                        temp = temp.add(&aik.mul(&ajk));
                    }
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] = c[c_idx].add(&temp.scale(alpha));
                }
            }
        }
        (CBLAS_UPLO::Upper, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..n {
                for j in i..n {
                    let mut temp = Complex::zero();
                    for k in 0..k {
                        let aki_idx = (k * lda + i) as usize;
                        let akj_idx = (k * lda + j) as usize;
                        let aki = Complex::new(a[aki_idx].real, -a[aki_idx].imag);
                        let akj = a[akj_idx];
                        temp = temp.add(&aki.mul(&akj));
                    }
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] = c[c_idx].add(&temp.scale(alpha));
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::NoTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex::zero();
                    for k in 0..k {
                        let aik_idx = (i * lda + k) as usize;
                        let ajk_idx = (j * lda + k) as usize;
                        let aik = a[aik_idx];
                        let ajk = Complex::new(a[ajk_idx].real, -a[ajk_idx].imag);
                        temp = temp.add(&aik.mul(&ajk));
                    }
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] = c[c_idx].add(&temp.scale(alpha));
                }
            }
        }
        (CBLAS_UPLO::Lower, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..n {
                for j in 0..=i {
                    let mut temp = Complex::zero();
                    for k in 0..k {
                        let aki_idx = (k * lda + i) as usize;
                        let akj_idx = (k * lda + j) as usize;
                        let aki = Complex::new(a[aki_idx].real, -a[aki_idx].imag);
                        let akj = a[akj_idx];
                        temp = temp.add(&aki.mul(&akj));
                    }
                    let c_idx = (i * ldc + j) as usize;
                    c[c_idx] = c[c_idx].add(&temp.scale(alpha));
                }
            }
        }
        _ => panic!("unrecognized operation"),
    }
}