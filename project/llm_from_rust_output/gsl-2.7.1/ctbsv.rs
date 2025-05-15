use std::f64;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_DIAG {
    NonUnit = 131,
    Unit = 132,
}

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Complex {
    fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
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

fn cblas_ctbsv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    trans: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    n: i32,
    k: i32,
    a: &[Complex],
    lda: i32,
    x: &mut [Complex],
    incx: i32,
) {
    let conj = if trans == CBLAS_TRANSPOSE::ConjTrans { -1.0 } else { 1.0 };
    let trans = if trans != CBLAS_TRANSPOSE::ConjTrans {
        trans
    } else {
        CBLAS_TRANSPOSE::Trans
    };
    let nonunit = diag == CBLAS_DIAG::NonUnit;

    // Parameter validation
    if !(order == CBLAS_ORDER::RowMajor || order == CBLAS_ORDER::ColMajor) {
        panic!("Invalid order");
    }
    if !(uplo == CBLAS_UPLO::Upper || uplo == CBLAS_UPLO::Lower) {
        panic!("Invalid uplo");
    }
    if !(trans == CBLAS_TRANSPOSE::NoTrans || trans == CBLAS_TRANSPOSE::Trans || trans == CBLAS_TRANSPOSE::ConjTrans) {
        panic!("Invalid trans");
    }
    if !(diag == CBLAS_DIAG::NonUnit || diag == CBLAS_DIAG::Unit) {
        panic!("Invalid diag");
    }
    if n < 0 {
        panic!("Invalid n");
    }
    if k < 0 {
        panic!("Invalid k");
    }
    if lda < std::cmp::max(1, k + 1) {
        panic!("Invalid lda");
    }
    if incx == 0 {
        panic!("Invalid incx");
    }
    if n == 0 {
        return;
    }

    // Main logic
    if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx } + incx * (n - 1);
        for i in (0..n).rev() {
            let mut tmp = x[(ix / 2) as usize];
            let j_min = i + 1;
            let j_max = std::cmp::min(n, i + k + 1);
            let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;
            for j in j_min..j_max {
                let a_idx = (lda * i + (j - i)) as usize;
                let a_ij = Complex::new(a[a_idx].real, conj * a[a_idx].imag);
                let x_j = x[(jx / 2) as usize];
                tmp = tmp - a_ij * x_j;
                jx += incx;
            }
            if nonunit {
                let a_idx = (lda * i) as usize;
                let a_ii = Complex::new(a[a_idx].real, conj * a[a_idx].imag);
                let s = xhypot(a_ii.real as f64, a_ii.imag as f64) as f32;
                let b = Complex::new(a_ii.real / s, a_ii.imag / s);
                let result = tmp * b;
                x[(ix / 2) as usize] = Complex::new(result.real / s, result.imag / s);
            } else {
                x[(ix / 2) as usize] = tmp;
            }
            ix -= incx;
        }
    } else if (order == CBLAS_ORDER::RowMajor && trans == CBLAS_TRANSPOSE::NoTrans && uplo == CBLAS_UPLO::Lower)
        || (order == CBLAS_ORDER::ColMajor && trans == CBLAS_TRANSPOSE::Trans && uplo == CBLAS_UPLO::Upper)
    {
        let mut ix = if incx > 0 { 0 } else { (n - 1) * -incx };
        for i in 0..n {
            let mut tmp = x[(ix / 2) as usize];
            let j_min = if k > i { 0 } else { i - k };
            let j_max = i;
            let mut jx = if incx > 0 { 0 } else { (n - 1) * -incx } + j_min * incx;
            for j in j_min..j_max {
                let a_idx = (lda * i + (k + j - i)) as usize;
                let a_ij = Complex::new(a[a_idx].real, conj * a[a_idx].imag);
                let x_j = x[(jx / 2) as usize];
                tmp = tmp - a_ij * x_j;
                jx += incx;
            }
            if nonunit {
                let a_idx = (lda * i + k) as usize;
                let a_ii = Complex::new(a[a_idx].real, conj * a[a_idx].imag);
                let s = xhypot(a_ii.real as f64, a_ii.imag as f64) as f32;
                let b = Complex::new(a_ii.real / s, a_ii.imag / s);
                let result = tmp * b;
                x[(ix / 2) as usize] = Complex::new(result.real / s, result.imag / s);
            } else {
                x[(ix / 2) as usize] = tmp;
            }
            ix += incx;
        }
    } else {
        panic!("Unrecognized operation");
    }
}