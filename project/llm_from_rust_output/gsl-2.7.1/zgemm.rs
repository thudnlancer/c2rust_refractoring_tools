use num_complex::Complex64;

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

pub fn cblas_zgemm(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    trans_b: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    k: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    b: &[Complex64],
    ldb: i32,
    beta: Complex64,
    c: &mut [Complex64],
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
        panic!("Invalid matrix dimensions");
    }

    // Early return if alpha is zero and beta is one
    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    // Determine matrix dimensions and access patterns based on order
    let (n1, n2, f, ldf, conj_f, trans_f, g, ldg, conj_g, trans_g) = if order == CBLAS_ORDER::RowMajor {
        (
            m,
            n,
            a,
            lda,
            if trans_a == CBLAS_TRANSPOSE::ConjTrans { -1 } else { 1 },
            if trans_a == CBLAS_TRANSPOSE::NoTrans { CBLAS_TRANSPOSE::NoTrans } else { CBLAS_TRANSPOSE::Trans },
            b,
            ldb,
            if trans_b == CBLAS_TRANSPOSE::ConjTrans { -1 } else { 1 },
            if trans_b == CBLAS_TRANSPOSE::NoTrans { CBLAS_TRANSPOSE::NoTrans } else { CBLAS_TRANSPOSE::Trans },
        )
    } else {
        (
            n,
            m,
            b,
            ldb,
            if trans_b == CBLAS_TRANSPOSE::ConjTrans { -1 } else { 1 },
            if trans_b == CBLAS_TRANSPOSE::NoTrans { CBLAS_TRANSPOSE::NoTrans } else { CBLAS_TRANSPOSE::Trans },
            a,
            lda,
            if trans_a == CBLAS_TRANSPOSE::ConjTrans { -1 } else { 1 },
            if trans_a == CBLAS_TRANSPOSE::NoTrans { CBLAS_TRANSPOSE::NoTrans } else { CBLAS_TRANSPOSE::Trans },
        )
    };

    // Scale C by beta if beta is not zero or one
    if beta != Complex64::new(0.0, 0.0) && beta != Complex64::new(1.0, 0.0) {
        for i in 0..n1 as usize {
            for j in 0..n2 as usize {
                let idx = (i * ldc as usize + j) * 2;
                c[idx] = beta.re * c[idx] - beta.im * c[idx + 1];
                c[idx + 1] = beta.re * c[idx + 1] + beta.im * c[idx];
            }
        }
    } else if beta == Complex64::new(0.0, 0.0) {
        for i in 0..n1 as usize {
            for j in 0..n2 as usize {
                let idx = (i * ldc as usize + j) * 2;
                c[idx] = 0.0;
                c[idx + 1] = 0.0;
            }
        }
    }

    // Early return if alpha is zero
    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    // Perform matrix multiplication based on transpose combinations
    match (trans_f, trans_g) {
        (CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::NoTrans) => {
            for kk in 0..k as usize {
                for i in 0..n1 as usize {
                    let f_idx = (i * ldf as usize + kk) * 2;
                    let fik = Complex64::new(
                        f[f_idx],
                        conj_f as f64 * f[f_idx + 1]
                    );
                    let temp = alpha * fik;
                    
                    if temp != Complex64::new(0.0, 0.0) {
                        for j in 0..n2 as usize {
                            let g_idx = (kk * ldg as usize + j) * 2;
                            let gkj = Complex64::new(
                                g[g_idx],
                                conj_g as f64 * g[g_idx + 1]
                            );
                            let c_idx = (i * ldc as usize + j) * 2;
                            let product = temp * gkj;
                            c[c_idx] += product.re;
                            c[c_idx + 1] += product.im;
                        }
                    }
                }
            }
        }
        (CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 as usize {
                for j in 0..n2 as usize {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for kk in 0..k as usize {
                        let f_idx = (i * ldf as usize + kk) * 2;
                        let fik = Complex64::new(
                            f[f_idx],
                            conj_f as f64 * f[f_idx + 1]
                        );
                        let g_idx = (j * ldg as usize + kk) * 2;
                        let gjk = Complex64::new(
                            g[g_idx],
                            conj_g as f64 * g[g_idx + 1]
                        );
                        temp += fik * gjk;
                    }
                    let c_idx = (i * ldc as usize + j) * 2;
                    let product = alpha * temp;
                    c[c_idx] += product.re;
                    c[c_idx + 1] += product.im;
                }
            }
        }
        (CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::NoTrans) => {
            for kk in 0..k as usize {
                for i in 0..n1 as usize {
                    let f_idx = (kk * ldf as usize + i) * 2;
                    let fki = Complex64::new(
                        f[f_idx],
                        conj_f as f64 * f[f_idx + 1]
                    );
                    let temp = alpha * fki;
                    
                    if temp != Complex64::new(0.0, 0.0) {
                        for j in 0..n2 as usize {
                            let g_idx = (kk * ldg as usize + j) * 2;
                            let gkj = Complex64::new(
                                g[g_idx],
                                conj_g as f64 * g[g_idx + 1]
                            );
                            let c_idx = (i * ldc as usize + j) * 2;
                            let product = temp * gkj;
                            c[c_idx] += product.re;
                            c[c_idx + 1] += product.im;
                        }
                    }
                }
            }
        }
        (CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..n1 as usize {
                for j in 0..n2 as usize {
                    let mut temp = Complex64::new(0.0, 0.0);
                    for kk in 0..k as usize {
                        let f_idx = (kk * ldf as usize + i) * 2;
                        let fki = Complex64::new(
                            f[f_idx],
                            conj_f as f64 * f[f_idx + 1]
                        );
                        let g_idx = (j * ldg as usize + kk) * 2;
                        let gjk = Complex64::new(
                            g[g_idx],
                            conj_g as f64 * g[g_idx + 1]
                        );
                        temp += fki * gjk;
                    }
                    let c_idx = (i * ldc as usize + j) * 2;
                    let product = alpha * temp;
                    c[c_idx] += product.re;
                    c[c_idx + 1] += product.im;
                }
            }
        }
        _ => panic!("Unrecognized operation"),
    }
}