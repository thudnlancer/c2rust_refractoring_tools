use gsl::{
    blas::{CblasDiag, CblasOrder, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::Matrix,
    vector::Vector,
    vector_uint::VectorUInt,
};

pub fn gsl_linalg_LU_band_decomp(
    M: usize,
    lb: usize,
    ub: usize,
    AB: &mut Matrix,
    piv: &mut VectorUInt,
) -> GslResult<()> {
    let N = AB.size1();
    let minMN = M.min(N);

    if lb >= M {
        return Err(GslError::Domain("lower bandwidth must be less than M"));
    } else if ub >= N {
        return Err(GslError::Domain("upper bandwidth must be less than N"));
    } else if AB.size2() != 2 * lb + ub + 1 {
        return Err(GslError::BadLength(
            "matrix size inconsistent with bandwidths",
        ));
    } else if piv.size() != minMN {
        return Err(GslError::BadLength(
            "pivot vector must have length MIN(M,N)",
        ));
    }

    LU_band_decomp_L2(M, lb, ub, AB, piv)
}

pub fn gsl_linalg_LU_band_solve(
    lb: usize,
    ub: usize,
    LUB: &Matrix,
    piv: &VectorUInt,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    let N = LUB.size1();

    if N != x.size() {
        return Err(GslError::BadLength(
            "matrix size must match solution size",
        ));
    } else if N != b.size() {
        return Err(GslError::BadLength("matrix size must match rhs size"));
    } else if lb >= N {
        return Err(GslError::Domain("lower bandwidth must be less than N"));
    } else if ub >= N {
        return Err(GslError::Domain("upper bandwidth must be less than N"));
    } else if LUB.size2() != 2 * lb + ub + 1 {
        return Err(GslError::BadLength(
            "matrix size inconsistent with bandwidths",
        ));
    } else if piv.size() != N {
        return Err(GslError::BadLength("pivot vector must have length N"));
    }

    x.copy_from(b)?;
    gsl_linalg_LU_band_svx(lb, ub, LUB, piv, x)
}

pub fn gsl_linalg_LU_band_svx(
    lb: usize,
    ub: usize,
    LUB: &Matrix,
    piv: &VectorUInt,
    x: &mut Vector,
) -> GslResult<()> {
    let N = LUB.size1();

    if N != x.size() {
        return Err(GslError::BadLength(
            "matrix size must match solution/rhs size",
        ));
    } else if lb >= N {
        return Err(GslError::Domain("lower bandwidth must be less than N"));
    } else if ub >= N {
        return Err(GslError::Domain("upper bandwidth must be less than N"));
    } else if LUB.size2() != 2 * lb + ub + 1 {
        return Err(GslError::BadLength(
            "matrix size inconsistent with bandwidths",
        ));
    } else if piv.size() != N {
        return Err(GslError::BadLength("pivot vector must have length N"));
    }

    if lb > 0 {
        for j in 0..N - 1 {
            let pj = piv.get(j) as usize;
            let xj = x.get_mut(j);
            let lm = lb.min(N - j - 1);

            let mut xv = x.subvector(j + 1, lm);
            let yv = LUB.subrow(j, lb + ub + 1, lm);

            if j != pj {
                let xl = x.get(pj);
                x.set(pj, *xj);
                *xj = xl;
            }

            xv.axpy(-*xj, &yv)?;
        }
    }

    x.tbsv(
        CblasOrder::ColMajor,
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        lb + ub,
        LUB.data(),
        LUB.tda() as i32,
    );

    Ok(())
}

pub fn gsl_linalg_LU_band_unpack(
    M: usize,
    lb: usize,
    ub: usize,
    LUB: &Matrix,
    piv: &VectorUInt,
    L: &mut Matrix,
    U: &mut Matrix,
) -> GslResult<()> {
    let N = LUB.size1();
    let minMN = M.min(N);

    if ub >= N {
        return Err(GslError::Domain("upper bandwidth must be < N"));
    } else if lb >= M {
        return Err(GslError::Domain("lower bandwidth must be < M"));
    } else if LUB.size2() != 2 * lb + ub + 1 {
        return Err(GslError::BadLength(
            "matrix size inconsistent with bandwidths",
        ));
    } else if piv.size() != minMN {
        return Err(GslError::BadLength(
            "pivot vector must have length MIN(M,N)",
        ));
    } else if L.size1() != M || L.size2() != minMN {
        return Err(GslError::BadLength("L matrix has wrong dimensions"));
    } else if U.size1() != minMN || U.size2() != N {
        return Err(GslError::BadLength("U matrix has wrong dimensions"));
    }

    let ub_U = lb + ub;

    L.set_identity();
    U.set_zero();

    if lb > 0 {
        let jstart = if M > N {
            minMN
        } else {
            minMN - 1
        };

        for j in (0..jstart).rev() {
            let pj = piv.get(j) as usize;
            let lm = lb.min(M - j - 1);

            let xv = LUB.subrow(j, lb + ub + 1, lm);
            let yv = L.subrow(j, 0, minMN);
            let mut m = L.submatrix(j + 1, 0, lm, minMN);

            m.ger(1.0, &xv, &yv)?;

            if j != pj {
                let mut Lj = L.row(j);
                let mut Lpj = L.row(pj);
                Lj.swap(&mut Lpj)?;
            }
        }
    }

    for j in 0..=ub_U.min(N - 1) {
        let src = LUB.subcolumn(ub_U - j, j, M.min(N - j));
        let mut dest = U.superdiagonal(j);
        dest.copy_from(&src)?;
    }

    Ok(())
}

fn LU_band_decomp_L2(
    M: usize,
    lb: usize,
    ub: usize,
    AB: &mut Matrix,
    ipiv: &mut VectorUInt,
) -> GslResult<()> {
    let N = AB.size1();
    let minMN = M.min(N);

    if ipiv.size() != minMN {
        return Err(GslError::BadLength("ipiv length must equal MIN(M,N)"));
    } else if lb >= M {
        return Err(GslError::Domain("lower bandwidth must be less than M"));
    } else if ub >= N {
        return Err(GslError::Domain("upper bandwidth must be less than N"));
    } else if AB.size2() != 2 * lb + ub + 1 {
        return Err(GslError::BadLength(
            "matrix size inconsistent with bandwidths",
        ));
    }

    let ub_U = lb + ub;
    let ldab = AB.size2();
    let mut ju = 0;

    if lb > 0 {
        let mut m = AB.submatrix(0, 0, N, lb);
        m.set_zero();
    }

    for j in 0..minMN {
        let lbj = lb.min(M - j - 1);
        let mut x = AB.subrow(j, ub_U, lbj + 1);
        let j_pivot = x.idamax();

        ipiv.set(j, (j + j_pivot) as u32);

        let pivot_val = AB.get(j, ub_U + j_pivot);
        if pivot_val != 0.0 {
            ju = ju.max(j + ub + j_pivot).min(N - 1);
        }

        if j_pivot != 0 {
            let mut x_view = AB.view_vector_with_stride(
                j,
                ub_U + j_pivot,
                ldab - 1,
                ju - j + 1,
            );
            let mut y_view = AB.view_vector_with_stride(
                j,
                ub_U,
                ldab - 1,
                ju - j + 1,
            );
            x_view.swap(&mut y_view)?;
        }

        if lbj > 0 {
            let tmp = AB.get(j, ub_U);
            let mut x = AB.subrow(j, ub_U + 1, lbj);
            x.scale(1.0 / tmp);

            if ju > j {
                let mut m = AB.submatrix(j + 1, ub_U, ju - j, lbj);
                m.tda = ldab - 1;
                let y = AB.view_vector_with_stride(j + 1, ub_U - 1, ldab - 1, ju - j);
                m.ger(-1.0, &y, &x)?;
            }
        }
    }

    Ok(())
}