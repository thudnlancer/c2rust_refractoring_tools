use gsl::{
    blas::CBLAS_TRANSPOSE_t,
    complex::{gsl_complex, gsl_complex_conjugate, gsl_complex_rect},
    error::{gsl_error, GSL_EBADLEN, GSL_ENOTSQR, GSL_SUCCESS},
    linalg::{
        gsl_linalg_complex_householder_hv, gsl_linalg_complex_householder_left,
        gsl_linalg_complex_householder_transform,
    },
    matrix_complex::{
        gsl_matrix_complex, gsl_matrix_complex_const_column,
        gsl_matrix_complex_const_subcolumn, gsl_matrix_complex_const_submatrix,
        gsl_matrix_complex_get, gsl_matrix_complex_set, gsl_matrix_complex_set_identity,
        gsl_matrix_complex_subcolumn, gsl_matrix_complex_submatrix,
    },
    vector_complex::{
        gsl_vector_complex, gsl_vector_complex_const_subvector, gsl_vector_complex_get,
        gsl_vector_complex_memcpy, gsl_vector_complex_set, gsl_vector_complex_set_zero,
        gsl_vector_complex_subvector,
    },
    CBLAS_DIAG_t, CBLAS_UPLO_t, CblasLower, CblasNoTrans, CblasNonUnit, CblasUpper,
};

pub fn gsl_linalg_complex_QR_decomp(
    A: &mut gsl_matrix_complex,
    tau: &mut gsl_vector_complex,
) -> i32 {
    let M = A.size1;
    let N = A.size2;

    if tau.size != N {
        gsl_error(
            "size of tau must be N",
            "qrc.c",
            68,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    for i in 0..std::cmp::min(M, N) {
        let mut c = gsl_matrix_complex_subcolumn(A, i, i, M - i);
        let tau_i = gsl_linalg_complex_householder_transform(&mut c.vector);
        gsl_vector_complex_set(tau, i, tau_i);

        if i + 1 < N {
            let mut m = gsl_matrix_complex_submatrix(A, i, i + 1, M - i, N - i - 1);
            let tau_i_conj = gsl_complex_conjugate(tau_i);
            let mut work = gsl_vector_complex_subvector(tau, i + 1, N - i - 1);
            gsl_linalg_complex_householder_left(
                tau_i_conj,
                &c.vector,
                &mut m.matrix,
                &mut work.vector,
            );
        }
    }

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_solve(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    b: &gsl_vector_complex,
    x: &mut gsl_vector_complex,
) -> i32 {
    if QR.size1 != QR.size2 {
        gsl_error(
            "QR matrix must be square",
            "qrc.c",
            113,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if QR.size1 != b.size {
        gsl_error(
            "matrix size must match b size",
            "qrc.c",
            117,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if QR.size2 != x.size {
        gsl_error(
            "matrix size must match solution size",
            "qrc.c",
            121,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    gsl_vector_complex_memcpy(x, b);
    gsl_linalg_complex_QR_svx(QR, tau, x);

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_svx(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    x: &mut gsl_vector_complex,
) -> i32 {
    if QR.size1 != QR.size2 {
        gsl_error(
            "QR matrix must be square",
            "qrc.c",
            148,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if QR.size1 != x.size {
        gsl_error(
            "matrix size must match x/rhs size",
            "qrc.c",
            152,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    gsl_linalg_complex_QR_QHvec(QR, tau, x);
    gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, QR, x);

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_lssolve(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    b: &gsl_vector_complex,
    x: &mut gsl_vector_complex,
    residual: &mut gsl_vector_complex,
) -> i32 {
    let M = QR.size1;
    let N = QR.size2;

    if M < N {
        gsl_error(
            "QR matrix must have M>=N",
            "qrc.c",
            184,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if M != b.size {
        gsl_error(
            "matrix size must match b size",
            "qrc.c",
            188,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if N != x.size {
        gsl_error(
            "matrix size must match solution size",
            "qrc.c",
            192,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if M != residual.size {
        gsl_error(
            "matrix size must match residual size",
            "qrc.c",
            196,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    let R = gsl_matrix_complex_const_submatrix(QR, 0, 0, N, N);
    let mut c = gsl_vector_complex_subvector(residual, 0, N);

    gsl_vector_complex_memcpy(residual, b);
    gsl_linalg_complex_QR_QHvec(QR, tau, residual);
    gsl_vector_complex_memcpy(x, &mut c.vector);
    gsl_blas_ztrsv(CblasUpper, CblasNoTrans, CblasNonUnit, &R.matrix, x);
    gsl_vector_complex_set_zero(&mut c.vector);
    gsl_linalg_complex_QR_Qvec(QR, tau, residual);

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_QHvec(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    v: &mut gsl_vector_complex,
) -> i32 {
    let M = QR.size1;
    let N = QR.size2;

    if tau.size != N {
        gsl_error(
            "size of tau must be N",
            "qrc.c",
            230,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if v.size != M {
        gsl_error(
            "vector size must be M",
            "qrc.c",
            234,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    for i in 0..std::cmp::min(M, N) {
        let h = gsl_matrix_complex_const_subcolumn(QR, i, i, M - i);
        let mut w = gsl_vector_complex_subvector(v, i, M - i);
        let ti = gsl_vector_complex_get(tau, i);
        let ti_conj = gsl_complex_conjugate(ti);
        gsl_linalg_complex_householder_hv(ti_conj, &h.vector, &mut w.vector);
    }

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_Qvec(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    v: &mut gsl_vector_complex,
) -> i32 {
    let M = QR.size1;
    let N = QR.size2;
    let min_mn = std::cmp::min(M, N);

    if tau.size != min_mn {
        gsl_error(
            "size of tau must be MIN(M,N)",
            "qrc.c",
            264,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if v.size != M {
        gsl_error(
            "vector size must be M",
            "qrc.c",
            268,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    for i in (1..=min_mn).rev() {
        let c = gsl_matrix_complex_const_column(QR, i - 1);
        let h = gsl_vector_complex_const_subvector(&c.vector, i - 1, M - (i - 1));
        let mut w = gsl_vector_complex_subvector(v, i - 1, M - (i - 1));
        let ti = gsl_vector_complex_get(tau, i - 1);
        gsl_linalg_complex_householder_hv(ti, &h.vector, &mut w.vector);
    }

    GSL_SUCCESS as i32
}

pub fn gsl_linalg_complex_QR_unpack(
    QR: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    Q: &mut gsl_matrix_complex,
    R: &mut gsl_matrix_complex,
) -> i32 {
    let M = QR.size1;
    let N = QR.size2;

    if Q.size1 != M || Q.size2 != M {
        gsl_error(
            "Q matrix must be M x M",
            "qrc.c",
            300,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if R.size1 != M || R.size2 != N {
        gsl_error(
            "R matrix must be M x N",
            "qrc.c",
            304,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if tau.size != N {
        gsl_error(
            "size of tau must be N",
            "qrc.c",
            308,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }

    gsl_matrix_complex_set_identity(Q);

    for i in (1..=std::cmp::min(M, N)).rev() {
        let c = gsl_matrix_complex_const_column(QR, i - 1);
        let h = gsl_vector_complex_const_subvector(&c.vector, i - 1, M - (i - 1));
        let mut m = gsl_matrix_complex_submatrix(Q, i - 1, i - 1, M - (i - 1), M - (i - 1));
        let ti = gsl_vector_complex_get(tau, i - 1);
        let mut work = gsl_matrix_complex_subcolumn(R, 0, 0, M - (i - 1));
        gsl_linalg_complex_householder_left(ti, &h.vector, &mut m.matrix, &mut work.vector);
    }

    for i in 0..M {
        for j in 0..std::cmp::min(i, N) {
            gsl_matrix_complex_set(R, i, j, gsl_complex_rect(0.0, 0.0));
        }
        for j in i..N {
            gsl_matrix_complex_set(R, i, j, gsl_matrix_complex_get(QR, i, j));
        }
    }

    GSL_SUCCESS as i32
}