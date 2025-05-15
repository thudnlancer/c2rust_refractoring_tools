use std::ffi::CString;

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

#[derive(Debug, Clone, Copy)]
pub struct ComplexFloat {
    pub real: f32,
    pub imag: f32,
}

impl ComplexFloat {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imag == 0.0
    }

    pub fn scale(&self, alpha: ComplexFloat) -> Self {
        Self {
            real: alpha.real * self.real - alpha.imag * self.imag,
            imag: alpha.real * self.imag + alpha.imag * self.real,
        }
    }

    pub fn add(&self, other: ComplexFloat) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn mul(&self, other: ComplexFloat) -> Self {
        Self {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

pub fn cblas_cgemm(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    trans_b: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    k: i32,
    alpha: ComplexFloat,
    a: &[ComplexFloat],
    lda: i32,
    b: &[ComplexFloat],
    ldb: i32,
    beta: ComplexFloat,
    c: &mut [ComplexFloat],
    ldc: i32,
) {
    // Parameter validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        panic!("Invalid order parameter");
    }
    if trans_a != CBLAS_TRANSPOSE::NoTrans
        && trans_a != CBLAS_TRANSPOSE::Trans
        && trans_a != CBLAS_TRANSPOSE::ConjTrans
    {
        panic!("Invalid trans_a parameter");
    }
    if trans_b != CBLAS_TRANSPOSE::NoTrans
        && trans_b != CBLAS_TRANSPOSE::Trans
        && trans_b != CBLAS_TRANSPOSE::ConjTrans
    {
        panic!("Invalid trans_b parameter");
    }
    if m < 0 || n < 0 || k < 0 {
        panic!("Negative matrix dimension");
    }

    // Early return if alpha is zero and beta is one
    if alpha.is_zero() && beta.real == 1.0 && beta.imag == 0.0 {
        return;
    }

    // Determine matrix dimensions and strides
    let (n1, n2, f, g, ldf, ldg, conj_f, conj_g, trans_f, trans_g) = match order {
        CBLAS_ORDER::RowMajor => (
            m,
            n,
            a,
            b,
            lda,
            ldb,
            if trans_a == CBLAS_TRANSPOSE::ConjTrans {
                -1
            } else {
                1
            },
            if trans_b == CBLAS_TRANSPOSE::ConjTrans {
                -1
            } else {
                1
            },
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
            if trans_b == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        ),
        CBLAS_ORDER::ColMajor => (
            n,
            m,
            b,
            a,
            ldb,
            lda,
            if trans_b == CBLAS_TRANSPOSE::ConjTrans {
                -1
            } else {
                1
            },
            if trans_a == CBLAS_TRANSPOSE::ConjTrans {
                -1
            } else {
                1
            },
            if trans_b == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
            if trans_a == CBLAS_TRANSPOSE::NoTrans {
                CBLAS_TRANSPOSE::NoTrans
            } else {
                CBLAS_TRANSPOSE::Trans
            },
        ),
    };

    // Scale C by beta if needed
    if beta.is_zero() {
        for i in 0..n1 as usize {
            for j in 0..n2 as usize {
                let idx = (i * ldc as usize + j) as usize;
                c[idx] = ComplexFloat::zero();
            }
        }
    } else if !(beta.real == 1.0 && beta.imag == 0.0) {
        for i in 0..n1 as usize {
            for j in 0..n2 as usize {
                let idx = (i * ldc as usize + j) as usize;
                c[idx] = c[idx].scale(beta);
            }
        }
    }

    // Early return if alpha is zero
    if alpha.is_zero() {
        return;
    }

    // Perform matrix multiplication
    match (trans_f, trans_g) {
        (CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::NoTrans) => {
            for k_idx in 0..k as usize {
                for i in 0..n1 as usize {
                    let f_idx = i * ldf as usize + k_idx;
                    let fik = ComplexFloat::new(
                        f[f_idx].real,
                        conj_f as f32 * f[f_idx].imag,
                    );
                    let temp = fik.scale(alpha);
                    if !temp.is_zero() {
                        for j in 0..n2 as usize {
                            let g_idx = k_idx * ldg as usize + j;
                            let gkj = ComplexFloat::new(
                                g[g_idx].real,
                                conj_g as f32 * g[g_idx].imag,
                            );
                            let c_idx = i * ldc as usize + j;
                            c[c_idx] = c[c_idx].add(temp.mul(gkj));
                        }
                    }
                }
            }
        }
        (CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 as usize {
                for j in 0..n2 as usize {
                    let mut temp = ComplexFloat::zero();
                    for k_idx in 0..k as usize {
                        let f_idx = i * ldf as usize + k_idx;
                        let g_idx = j * ldg as usize + k_idx;
                        let fik = ComplexFloat::new(
                            f[f_idx].real,
                            conj_f as f32 * f[f_idx].imag,
                        );
                        let gjk = ComplexFloat::new(
                            g[g_idx].real,
                            conj_g as f32 * g[g_idx].imag,
                        );
                        temp = temp.add(fik.mul(gjk));
                    }
                    let c_idx = i * ldc as usize + j;
                    c[c_idx] = c[c_idx].add(temp.scale(alpha));
                }
            }
        }
        (CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::NoTrans) => {
            for k_idx in 0..k as usize {
                for i in 0..n1 as usize {
                    let f_idx = k_idx * ldf as usize + i;
                    let fki = ComplexFloat::new(
                        f[f_idx].real,
                        conj_f as f32 * f[f_idx].imag,
                    );
                    let temp = fki.scale(alpha);
                    if !temp.is_zero() {
                        for j in 0..n2 as usize {
                            let g_idx = k_idx * ldg as usize + j;
                            let gkj = ComplexFloat::new(
                                g[g_idx].real,
                                conj_g as f32 * g[g_idx].imag,
                            );
                            let c_idx = i * ldc as usize + j;
                            c[c_idx] = c[c_idx].add(temp.mul(gkj));
                        }
                    }
                }
            }
        }
        (CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 as usize {
                for j in 0..n2 as usize {
                    let mut temp = ComplexFloat::zero();
                    for k_idx in 0..k as usize {
                        let f_idx = k_idx * ldf as usize + i;
                        let g_idx = j * ldg as usize + k_idx;
                        let fki = ComplexFloat::new(
                            f[f_idx].real,
                            conj_f as f32 * f[f_idx].imag,
                        );
                        let gjk = ComplexFloat::new(
                            g[g_idx].real,
                            conj_g as f32 * g[g_idx].imag,
                        );
                        temp = temp.add(fki.mul(gjk));
                    }
                    let c_idx = i * ldc as usize + j;
                    c[c_idx] = c[c_idx].add(temp.scale(alpha));
                }
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}