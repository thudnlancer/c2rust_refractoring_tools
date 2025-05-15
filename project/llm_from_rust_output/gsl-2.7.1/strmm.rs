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

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    panic!("CBLAS error {}: {} {}", p, rout, form);
}

pub fn cblas_strmm(
    order: CBLAS_ORDER,
    side: CBLAS_SIDE,
    uplo: CBLAS_UPLO,
    trans_a: CBLAS_TRANSPOSE,
    diag: CBLAS_DIAG,
    m: i32,
    n: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    b: &mut [f32],
    ldb: i32,
) {
    let mut pos = 0;
    let dim = if side == CBLAS_SIDE::Left { m } else { n };

    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        pos = 1;
    }
    if side != CBLAS_SIDE::Left && side != CBLAS_SIDE::Right {
        pos = 2;
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        pos = 3;
    }
    if trans_a != CBLAS_TRANSPOSE::NoTrans
        && trans_a != CBLAS_TRANSPOSE::Trans
        && trans_a != CBLAS_TRANSPOSE::ConjTrans
    {
        pos = 4;
    }
    if diag != CBLAS_DIAG::NonUnit && diag != CBLAS_DIAG::Unit {
        pos = 5;
    }
    if m < 0 {
        pos = 6;
    }
    if n < 0 {
        pos = 7;
    }
    if lda < 1.max(dim) {
        pos = 10;
    }
    if (order == CBLAS_ORDER::RowMajor && ldb < 1.max(n))
        || (order == CBLAS_ORDER::ColMajor && ldb < 1.max(m))
    {
        pos = 12;
    }

    if pos != 0 {
        cblas_xerbla(pos, "./source_trmm_r.h", "");
        return;
    }

    let (n1, n2, side, uplo, trans) = if order == CBLAS_ORDER::RowMajor {
        (
            m,
            n,
            side as i32,
            uplo as i32,
            if trans_a == CBLAS_TRANSPOSE::ConjTrans {
                CBLAS_TRANSPOSE::Trans as i32
            } else {
                trans_a as i32
            },
        )
    } else {
        (
            n,
            m,
            if side == CBLAS_SIDE::Left {
                CBLAS_SIDE::Right as i32
            } else {
                CBLAS_SIDE::Left as i32
            },
            if uplo == CBLAS_UPLO::Upper {
                CBLAS_UPLO::Lower as i32
            } else {
                CBLAS_UPLO::Upper as i32
            },
            if trans_a == CBLAS_TRANSPOSE::ConjTrans {
                CBLAS_TRANSPOSE::Trans as i32
            } else {
                trans_a as i32
            },
        )
    };

    let nonunit = (diag == CBLAS_DIAG::NonUnit) as i32;

    macro_rules! get_a {
        ($i:expr, $j:expr) => {
            a[($i * lda + $j) as usize]
        };
    }

    macro_rules! get_b {
        ($i:expr, $j:expr) => {
            b[($i * ldb + $j) as usize]
        };
    }

    macro_rules! set_b {
        ($i:expr, $j:expr, $val:expr) => {
            b[($i * ldb + $j) as usize] = $val;
        };
    }

    if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::NoTrans as i32 {
        for i in 0..n1 {
            for j in 0..n2 {
                let mut temp = if nonunit != 0 {
                    get_a!(i, i) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                for k in (i + 1)..n1 {
                    temp += get_a!(i, k) * get_b!(k, j);
                }
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::Trans as i32 {
        for i in (0..n1).rev() {
            for j in 0..n2 {
                let mut temp = 0.0;
                for k in 0..i {
                    temp += get_a!(k, i) * get_b!(k, j);
                }
                temp += if nonunit != 0 {
                    get_a!(i, i) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Lower as i32 && trans == CBLAS_TRANSPOSE::NoTrans as i32 {
        for i in (0..n1).rev() {
            for j in 0..n2 {
                let mut temp = 0.0;
                for k in 0..i {
                    temp += get_a!(i, k) * get_b!(k, j);
                }
                temp += if nonunit != 0 {
                    get_a!(i, i) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Left as i32 && uplo == CBLAS_UPLO::Lower as i32 && trans == CBLAS_TRANSPOSE::Trans as i32 {
        for i in 0..n1 {
            for j in 0..n2 {
                let mut temp = if nonunit != 0 {
                    get_a!(i, i) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                for k in (i + 1)..n1 {
                    temp += get_a!(k, i) * get_b!(k, j);
                }
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Right as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::NoTrans as i32 {
        for i in 0..n1 {
            for j in (0..n2).rev() {
                let mut temp = 0.0;
                for k in 0..j {
                    temp += get_a!(k, j) * get_b!(i, k);
                }
                temp += if nonunit != 0 {
                    get_a!(j, j) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Right as i32 && uplo == CBLAS_UPLO::Upper as i32 && trans == CBLAS_TRANSPOSE::Trans as i32 {
        for i in 0..n1 {
            for j in 0..n2 {
                let mut temp = if nonunit != 0 {
                    get_a!(j, j) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                for k in (j + 1)..n2 {
                    temp += get_a!(j, k) * get_b!(i, k);
                }
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Right as i32 && uplo == CBLAS_UPLO::Lower as i32 && trans == CBLAS_TRANSPOSE::NoTrans as i32 {
        for i in 0..n1 {
            for j in 0..n2 {
                let mut temp = if nonunit != 0 {
                    get_a!(j, j) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                for k in (j + 1)..n2 {
                    temp += get_a!(k, j) * get_b!(i, k);
                }
                set_b!(i, j, alpha * temp);
            }
        }
    } else if side == CBLAS_SIDE::Right as i32 && uplo == CBLAS_UPLO::Lower as i32 && trans == CBLAS_TRANSPOSE::Trans as i32 {
        for i in 0..n1 {
            for j in (0..n2).rev() {
                let mut temp = 0.0;
                for k in 0..j {
                    temp += get_a!(j, k) * get_b!(i, k);
                }
                temp += if nonunit != 0 {
                    get_a!(j, j) * get_b!(i, j)
                } else {
                    get_b!(i, j)
                };
                set_b!(i, j, alpha * temp);
            }
        }
    } else {
        cblas_xerbla(0, "./source_trmm_r.h", "unrecognized operation");
    }
}