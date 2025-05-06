#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_vector_subvector(
        v: *mut gsl_vector,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> i32;
    fn gsl_vector_add(a: *mut gsl_vector, b: *const gsl_vector) -> i32;
    fn gsl_vector_sub(a: *mut gsl_vector, b: *const gsl_vector) -> i32;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> i32;
    fn gsl_matrix_add(a: *mut gsl_matrix, b: *const gsl_matrix) -> i32;
    fn gsl_matrix_sub(a: *mut gsl_matrix, b: *const gsl_matrix) -> i32;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_linalg_QR_UZ_decomp(
        S: *mut gsl_matrix,
        A: *mut gsl_matrix,
        T: *mut gsl_matrix,
    ) -> i32;
    fn gsl_blas_dtrmm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
    ) -> i32;
    fn gsl_blas_dtrmv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> i32;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> i32;
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_view {
    pub vector: gsl_vector,
}
pub type gsl_vector_view = _gsl_vector_view;
pub type CBLAS_TRANSPOSE = u32;
pub const CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub const CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub type CBLAS_UPLO = u32;
pub const CblasLower: CBLAS_UPLO = 122;
pub const CblasUpper: CBLAS_UPLO = 121;
pub type CBLAS_DIAG = u32;
pub const CblasUnit: CBLAS_DIAG = 132;
pub const CblasNonUnit: CBLAS_DIAG = 131;
pub type CBLAS_SIDE = u32;
pub const CblasRight: CBLAS_SIDE = 142;
pub const CblasLeft: CBLAS_SIDE = 141;
pub type CBLAS_TRANSPOSE_t = CBLAS_TRANSPOSE;
pub type CBLAS_UPLO_t = CBLAS_UPLO;
pub type CBLAS_DIAG_t = CBLAS_DIAG;
pub type CBLAS_SIDE_t = CBLAS_SIDE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_matrix_view {
    pub matrix: gsl_matrix,
}
pub type gsl_matrix_view = _gsl_matrix_view;
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_QR_UU_decomp(
    mut U: *mut gsl_matrix,
    mut S: *mut gsl_matrix,
    mut T: *mut gsl_matrix,
) -> i32 {
    let N: size_t = (*U).size1;
    if N != (*U).size2 {
        gsl_error(
            b"U matrix must be square\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            77 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*S).size1 != (*S).size2 {
        gsl_error(
            b"S matrix must be square\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            81 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if N != (*S).size1 {
        gsl_error(
            b"S and U must have same dimensions\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            85 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix has wrong dimensions\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            89 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if N == 1 as i32 as u64 {
        let mut T00: *mut libc::c_double = gsl_matrix_ptr(
            T,
            0 as i32 as size_t,
            0 as i32 as size_t,
        );
        let mut U00: *mut libc::c_double = gsl_matrix_ptr(
            U,
            0 as i32 as size_t,
            0 as i32 as size_t,
        );
        let mut S00: *mut libc::c_double = gsl_matrix_ptr(
            S,
            0 as i32 as size_t,
            0 as i32 as size_t,
        );
        *T00 = qrtt_householder_transform(U00, S00);
        return GSL_SUCCESS as i32;
    } else {
        let mut status: i32 = 0;
        let N1: size_t = N.wrapping_div(2 as i32 as u64);
        let N2: size_t = N.wrapping_sub(N1);
        let mut U11: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as i32 as size_t,
            0 as i32 as size_t,
            N1,
            N1,
        );
        let mut U12: gsl_matrix_view = gsl_matrix_submatrix(
            U,
            0 as i32 as size_t,
            N1,
            N1,
            N2,
        );
        let mut U22: gsl_matrix_view = gsl_matrix_submatrix(U, N1, N1, N2, N2);
        let mut S11: gsl_matrix_view = gsl_matrix_submatrix(
            S,
            0 as i32 as size_t,
            0 as i32 as size_t,
            N1,
            N1,
        );
        let mut S12: gsl_matrix_view = gsl_matrix_submatrix(
            S,
            0 as i32 as size_t,
            N1,
            N1,
            N2,
        );
        let mut T11: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as i32 as size_t,
            0 as i32 as size_t,
            N1,
            N1,
        );
        let mut T12: gsl_matrix_view = gsl_matrix_submatrix(
            T,
            0 as i32 as size_t,
            N1,
            N1,
            N2,
        );
        let mut T22: gsl_matrix_view = gsl_matrix_submatrix(T, N1, N1, N2, N2);
        let mut m: gsl_matrix_view = gsl_matrix_view {
            matrix: gsl_matrix {
                size1: 0,
                size2: 0,
                tda: 0,
                data: 0 as *mut libc::c_double,
                block: 0 as *mut gsl_block,
                owner: 0,
            },
        };
        status = gsl_linalg_QR_UU_decomp(
            &mut U11.matrix,
            &mut S11.matrix,
            &mut T11.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut S12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut S11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_add(&mut T12.matrix, &mut U12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_sub(&mut U12.matrix, &mut T12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            &mut S11.matrix,
            &mut T12.matrix,
        );
        gsl_matrix_sub(&mut S12.matrix, &mut T12.matrix);
        m = gsl_matrix_submatrix(S, 0 as i32 as size_t, N1, N, N2);
        status = gsl_linalg_QR_UZ_decomp(
            &mut U22.matrix,
            &mut m.matrix,
            &mut T22.matrix,
        );
        if status != 0 {
            return status;
        }
        gsl_matrix_memcpy(&mut T12.matrix, &mut S12.matrix);
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasTrans,
            CblasNonUnit,
            1.0f64,
            &mut S11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            -1.0f64,
            &mut T11.matrix,
            &mut T12.matrix,
        );
        gsl_blas_dtrmm(
            CblasRight,
            CblasUpper,
            CblasNoTrans,
            CblasNonUnit,
            1.0f64,
            &mut T22.matrix,
            &mut T12.matrix,
        );
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_QR_UU_lssolve(
    mut R: *const gsl_matrix,
    mut Y: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> i32 {
    let N: size_t = (*R).size1;
    let M: size_t = (2 as i32 as u64).wrapping_mul(N);
    if (*R).size2 != N {
        gsl_error(
            b"R matrix must be square\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            227 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*Y).size1 != (*Y).size2 {
        gsl_error(
            b"Y matrix must be square\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            231 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*Y).size1 != N {
        gsl_error(
            b"Y and R must have same dimensions\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            235 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            239 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if M != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            243 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if M != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            247 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if N != (*work).size {
        gsl_error(
            b"workspace must be length N\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            251 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut x1: gsl_vector_view = gsl_vector_subvector(x, 0 as i32 as size_t, N);
        gsl_vector_memcpy(x, b);
        gsl_linalg_QR_UU_QTvec(Y, T, x, work);
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, R, &mut x1.vector);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_QR_UU_QTvec(
    mut Y: *const gsl_matrix,
    mut T: *const gsl_matrix,
    mut b: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> i32 {
    let N: size_t = (*Y).size1;
    let M: size_t = (2 as i32 as u64).wrapping_mul(N);
    if (*Y).size2 != N {
        gsl_error(
            b"Y matrix must be square\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            296 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*T).size1 != N || (*T).size2 != N {
        gsl_error(
            b"T matrix must be N-by-N\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            300 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*b).size != M {
        gsl_error(
            b"b vector must have length M\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            304 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*work).size != N {
        gsl_error(
            b"workspace must be length N\0" as *const u8 as *const i8,
            b"qr_uu.c\0" as *const u8 as *const i8,
            308 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut b1: gsl_vector_view = gsl_vector_subvector(b, 0 as i32 as size_t, N);
        let mut b2: gsl_vector_view = gsl_vector_subvector(b, N, N);
        gsl_vector_memcpy(work, &mut b2.vector);
        gsl_blas_dtrmv(CblasUpper, CblasTrans, CblasNonUnit, Y, work);
        gsl_vector_add(work, &mut b1.vector);
        gsl_blas_dtrmv(CblasUpper, CblasTrans, CblasNonUnit, T, work);
        gsl_vector_sub(&mut b1.vector, work);
        gsl_blas_dtrmv(CblasUpper, CblasNoTrans, CblasNonUnit, Y, work);
        gsl_vector_sub(&mut b2.vector, work);
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn qrtt_householder_transform(
    mut v0: *mut libc::c_double,
    mut v1: *mut libc::c_double,
) -> libc::c_double {
    let mut alpha: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut tau: libc::c_double = 0.;
    let mut xnorm: libc::c_double = fabs(*v1);
    if xnorm == 0 as i32 as libc::c_double {
        return 0.0f64;
    }
    alpha = *v0;
    beta = -(if alpha >= 0.0f64 { 1 as i32 } else { -(1 as i32) }) as libc::c_double
        * hypot(alpha, xnorm);
    tau = (beta - alpha) / beta;
    let mut s: libc::c_double = alpha - beta;
    if fabs(s) > 2.2250738585072014e-308f64 {
        *v1 /= s;
        *v0 = beta;
    } else {
        *v1 *= 2.2204460492503131e-16f64 / s;
        *v1 /= 2.2204460492503131e-16f64;
        *v0 = beta;
    }
    return tau;
}