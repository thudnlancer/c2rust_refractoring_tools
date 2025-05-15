use gsl::{
    blas::{CBLAS_UPLO, CblasLower},
    cblas::CBLAS_UPLO_t,
    complex::{gsl_complex, gsl_complex_mul, gsl_complex_mul_real, gsl_complex_rect},
    error::{gsl_error, GSL_EBADLEN, GSL_ENOTSQR, GSL_SUCCESS},
    linalg::{
        gsl_linalg_complex_householder_left, gsl_linalg_complex_householder_transform,
    },
    matrix_complex::{
        gsl_matrix_complex, gsl_matrix_complex_column, gsl_matrix_complex_const_diagonal,
        gsl_matrix_complex_const_subcolumn, gsl_matrix_complex_const_subdiagonal,
        gsl_matrix_complex_set_identity, gsl_matrix_complex_submatrix,
    },
    vector::gsl_vector,
    vector_complex::{
        gsl_vector_complex, gsl_vector_complex_alloc, gsl_vector_complex_const_real,
        gsl_vector_complex_free, gsl_vector_complex_get, gsl_vector_complex_set,
        gsl_vector_complex_subvector,
    },
};

pub fn gsl_linalg_hermtd_decomp(
    A: &mut gsl_matrix_complex,
    tau: &mut gsl_vector_complex,
) -> i32 {
    if A.size1 != A.size2 {
        gsl_error(
            "hermitian tridiagonal decomposition requires square matrix",
            "hermtd.c",
            66,
            GSL_ENOTSQR,
        );
        return GSL_ENOTSQR;
    } else if (tau.size + 1) != A.size1 {
        gsl_error(
            "size of tau must be (matrix size - 1)",
            "hermtd.c",
            70,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    let N = A.size1;
    let zero = gsl_complex_rect(0.0, 0.0);
    let one = gsl_complex_rect(1.0, 0.0);
    let neg_one = gsl_complex_rect(-1.0, 0.0);

    for i in 0..N - 1 {
        let c = gsl_matrix_complex_column(A, i);
        let mut v = gsl_vector_complex_subvector(&c.vector, i + 1, N - (i + 1));
        let mut tau_i = gsl_linalg_complex_householder_transform(&mut v.vector);

        if i + 1 < N - 1 && !(tau_i.dat[0] == 0.0 && tau_i.dat[1] == 0.0) {
            let mut m = gsl_matrix_complex_submatrix(A, i + 1, i + 1, N - (i + 1), N - (i + 1));
            let ei = gsl_vector_complex_get(&v.vector, 0);
            let mut x = gsl_vector_complex_subvector(tau, i, N - (i + 1));

            gsl_vector_complex_set(&mut v.vector, 0, one);
            gsl_blas_zhemv(
                CblasLower,
                tau_i,
                &m.matrix,
                &v.vector,
                zero,
                &mut x.vector,
            );

            let mut xv = gsl_complex { dat: [0.0, 0.0] };
            let mut txv = gsl_complex { dat: [0.0, 0.0] };
            let mut alpha = gsl_complex { dat: [0.0, 0.0] };

            gsl_blas_zdotc(&x.vector, &v.vector, &mut xv);
            txv = gsl_complex_mul(tau_i, xv);
            alpha = gsl_complex_mul_real(txv, -0.5);
            gsl_blas_zaxpy(alpha, &v.vector, &mut x.vector);
            gsl_blas_zher2(CblasLower, neg_one, &v.vector, &x.vector, &mut m.matrix);
            gsl_vector_complex_set(&mut v.vector, 0, ei);
        }

        gsl_vector_complex_set(tau, i, tau_i);
    }

    GSL_SUCCESS
}

pub fn gsl_linalg_hermtd_unpack(
    A: &gsl_matrix_complex,
    tau: &gsl_vector_complex,
    U: &mut gsl_matrix_complex,
    diag: &mut gsl_vector,
    sdiag: &mut gsl_vector,
) -> i32 {
    if A.size1 != A.size2 {
        gsl_error("matrix A must be square", "hermtd.c", 136, GSL_ENOTSQR);
        return GSL_ENOTSQR;
    } else if (tau.size + 1) != A.size1 {
        gsl_error(
            "size of tau must be (matrix size - 1)",
            "hermtd.c",
            140,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if U.size1 != A.size1 || U.size2 != A.size1 {
        gsl_error(
            "size of U must match size of A",
            "hermtd.c",
            144,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if diag.size != A.size1 {
        gsl_error(
            "size of diagonal must match size of A",
            "hermtd.c",
            148,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (sdiag.size + 1) != A.size1 {
        gsl_error(
            "size of subdiagonal must be (matrix size - 1)",
            "hermtd.c",
            152,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    let N = A.size1;
    let zd = gsl_matrix_complex_const_diagonal(A);
    let zsd = gsl_matrix_complex_const_subdiagonal(A, 1);
    let d = gsl_vector_complex_const_real(&zd.vector);
    let sd = gsl_vector_complex_const_real(&zsd.vector);
    let mut work = gsl_vector_complex_alloc(N).unwrap();

    gsl_matrix_complex_set_identity(U);

    for i in (0..N - 1).rev() {
        let ti = gsl_vector_complex_get(tau, i);
        let h = gsl_matrix_complex_const_subcolumn(A, i, i + 1, N - (i + 1));
        let mut m = gsl_matrix_complex_submatrix(U, i + 1, i + 1, N - (i + 1), N - (i + 1));
        let mut w = gsl_vector_complex_subvector(&mut work, 0, N - (i + 1));

        gsl_linalg_complex_householder_left(ti, &h.vector, &mut m.matrix, &mut w.vector);
    }

    gsl_vector_memcpy(diag, &d.vector);
    gsl_vector_memcpy(sdiag, &sd.vector);
    gsl_vector_complex_free(&mut work);

    GSL_SUCCESS
}

pub fn gsl_linalg_hermtd_unpack_T(
    A: &gsl_matrix_complex,
    diag: &mut gsl_vector,
    sdiag: &mut gsl_vector,
) -> i32 {
    if A.size1 != A.size2 {
        gsl_error("matrix A must be square", "hermtd.c", 198, GSL_ENOTSQR);
        return GSL_ENOTSQR;
    } else if diag.size != A.size1 {
        gsl_error(
            "size of diagonal must match size of A",
            "hermtd.c",
            202,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    } else if (sdiag.size + 1) != A.size1 {
        gsl_error(
            "size of subdiagonal must be (matrix size - 1)",
            "hermtd.c",
            206,
            GSL_EBADLEN,
        );
        return GSL_EBADLEN;
    }

    let zd = gsl_matrix_complex_const_diagonal(A);
    let zsd = gsl_matrix_complex_const_subdiagonal(A, 1);
    let d = gsl_vector_complex_const_real(&zd.vector);
    let sd = gsl_vector_complex_const_real(&zsd.vector);

    gsl_vector_memcpy(diag, &d.vector);
    gsl_vector_memcpy(sdiag, &sd.vector);

    GSL_SUCCESS
}