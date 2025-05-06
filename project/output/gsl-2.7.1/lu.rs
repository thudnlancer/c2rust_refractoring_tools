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
    fn log(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_vector_memcpy(dest: *mut gsl_vector, src: *const gsl_vector) -> i32;
    fn gsl_vector_uint_alloc(n: size_t) -> *mut gsl_vector_uint;
    fn gsl_vector_uint_free(v: *mut gsl_vector_uint);
    fn gsl_vector_uint_subvector(
        v: *mut gsl_vector_uint,
        i: size_t,
        n: size_t,
    ) -> _gsl_vector_uint_view;
    fn gsl_matrix_submatrix(
        m: *mut gsl_matrix,
        i: size_t,
        j: size_t,
        n1: size_t,
        n2: size_t,
    ) -> _gsl_matrix_view;
    fn gsl_matrix_row(m: *mut gsl_matrix, i: size_t) -> _gsl_vector_view;
    fn gsl_matrix_subrow(
        m: *mut gsl_matrix,
        i: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_subcolumn(
        m: *mut gsl_matrix,
        j: size_t,
        offset: size_t,
        n: size_t,
    ) -> _gsl_vector_view;
    fn gsl_matrix_memcpy(dest: *mut gsl_matrix, src: *const gsl_matrix) -> i32;
    fn gsl_permutation_init(p: *mut gsl_permutation);
    fn gsl_permute_vector(p: *const gsl_permutation, v: *mut gsl_vector) -> i32;
    fn gsl_permute_vector_inverse(p: *const gsl_permutation, v: *mut gsl_vector) -> i32;
    fn gsl_blas_idamax(X: *const gsl_vector) -> CBLAS_INDEX_t;
    fn gsl_blas_dswap(X: *mut gsl_vector, Y: *mut gsl_vector) -> i32;
    fn gsl_blas_daxpy(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *mut gsl_vector,
    ) -> i32;
    fn gsl_blas_dscal(alpha: libc::c_double, X: *mut gsl_vector);
    fn gsl_blas_dgemv(
        TransA: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        X: *const gsl_vector,
        beta: libc::c_double,
        Y: *mut gsl_vector,
    ) -> i32;
    fn gsl_blas_dtrsv(
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        A: *const gsl_matrix,
        X: *mut gsl_vector,
    ) -> i32;
    fn gsl_blas_dger(
        alpha: libc::c_double,
        X: *const gsl_vector,
        Y: *const gsl_vector,
        A: *mut gsl_matrix,
    ) -> i32;
    fn gsl_blas_dgemm(
        TransA: CBLAS_TRANSPOSE_t,
        TransB: CBLAS_TRANSPOSE_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *const gsl_matrix,
        beta: libc::c_double,
        C: *mut gsl_matrix,
    ) -> i32;
    fn gsl_blas_dtrsm(
        Side: CBLAS_SIDE_t,
        Uplo: CBLAS_UPLO_t,
        TransA: CBLAS_TRANSPOSE_t,
        Diag: CBLAS_DIAG_t,
        alpha: libc::c_double,
        A: *const gsl_matrix,
        B: *mut gsl_matrix,
    ) -> i32;
    fn gsl_linalg_tri_invert(
        Uplo: CBLAS_UPLO_t,
        Diag: CBLAS_DIAG_t,
        T: *mut gsl_matrix,
    ) -> i32;
    fn gsl_linalg_tri_UL(LU: *mut gsl_matrix) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut u32,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u32,
    pub block: *mut gsl_block_uint,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _gsl_vector_uint_view {
    pub vector: gsl_vector_uint,
}
pub type gsl_vector_uint_view = _gsl_vector_uint_view;
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
pub type CBLAS_INDEX_t = size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
#[inline]
unsafe extern "C" fn gsl_vector_uint_get(
    mut v: *const gsl_vector_uint,
    i: size_t,
) -> u32 {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_set(
    mut v: *mut gsl_vector_uint,
    i: size_t,
    mut x: u32,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[inline]
unsafe extern "C" fn gsl_vector_uint_ptr(
    mut v: *mut gsl_vector_uint,
    i: size_t,
) -> *mut u32 {
    return ((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_get(
    mut m: *const gsl_matrix,
    i: size_t,
    j: size_t,
) -> libc::c_double {
    return *((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[inline]
unsafe extern "C" fn gsl_matrix_ptr(
    mut m: *mut gsl_matrix,
    i: size_t,
    j: size_t,
) -> *mut libc::c_double {
    return ((*m).data).offset(i.wrapping_mul((*m).tda).wrapping_add(j) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_decomp(
    mut A: *mut gsl_matrix,
    mut p: *mut gsl_permutation,
    mut signum: *mut i32,
) -> i32 {
    let M: size_t = (*A).size1;
    if (*p).size != M {
        gsl_error(
            b"permutation length must match matrix size1\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            68 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut status: i32 = 0;
        let N: size_t = (*A).size2;
        let minMN: size_t = if M < N { M } else { N };
        let mut ipiv: *mut gsl_vector_uint = gsl_vector_uint_alloc(minMN);
        let mut AL: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as i32 as size_t,
            0 as i32 as size_t,
            M,
            minMN,
        );
        let mut i: size_t = 0;
        status = LU_decomp_L3(&mut AL.matrix, ipiv);
        if M < N {
            let mut AR: gsl_matrix_view = gsl_matrix_submatrix(
                A,
                0 as i32 as size_t,
                M,
                M,
                N.wrapping_sub(M),
            );
            apply_pivots(&mut AR.matrix, ipiv);
            gsl_blas_dtrsm(
                CblasLeft,
                CblasLower,
                CblasNoTrans,
                CblasUnit,
                1.0f64,
                &mut AL.matrix,
                &mut AR.matrix,
            );
        }
        gsl_permutation_init(p);
        *signum = 1 as i32;
        i = 0 as i32 as size_t;
        while i < minMN {
            let mut pivi: u32 = gsl_vector_uint_get(ipiv, i);
            if *((*p).data).offset(pivi as isize) != *((*p).data).offset(i as isize) {
                let mut tmp: size_t = *((*p).data).offset(pivi as isize);
                *((*p).data).offset(pivi as isize) = *((*p).data).offset(i as isize);
                *((*p).data).offset(i as isize) = tmp;
                *signum = -*signum;
            }
            i = i.wrapping_add(1);
            i;
        }
        gsl_vector_uint_free(ipiv);
        return status;
    };
}
unsafe extern "C" fn LU_decomp_L2(
    mut A: *mut gsl_matrix,
    mut ipiv: *mut gsl_vector_uint,
) -> i32 {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    let minMN: size_t = if M < N { M } else { N };
    if (*ipiv).size != minMN {
        gsl_error(
            b"ipiv length must equal MIN(M,N)\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            137 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        j = 0 as i32 as size_t;
        while j < minMN {
            let mut v: gsl_vector_view = gsl_matrix_subcolumn(
                A,
                j,
                j,
                M.wrapping_sub(j),
            );
            let mut j_pivot: size_t = j.wrapping_add(gsl_blas_idamax(&mut v.vector));
            let mut v1: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            let mut v2: gsl_vector_view = gsl_vector_view {
                vector: gsl_vector {
                    size: 0,
                    stride: 0,
                    data: 0 as *mut libc::c_double,
                    block: 0 as *mut gsl_block,
                    owner: 0,
                },
            };
            gsl_vector_uint_set(ipiv, j, j_pivot as u32);
            if j_pivot != j {
                v1 = gsl_matrix_row(A, j);
                v2 = gsl_matrix_row(A, j_pivot);
                gsl_blas_dswap(&mut v1.vector, &mut v2.vector);
            }
            if j < M.wrapping_sub(1 as i32 as u64) {
                let mut Ajj: libc::c_double = gsl_matrix_get(A, j, j);
                if fabs(Ajj) >= 2.2250738585072014e-308f64 {
                    v1 = gsl_matrix_subcolumn(
                        A,
                        j,
                        j.wrapping_add(1 as i32 as u64),
                        M.wrapping_sub(j).wrapping_sub(1 as i32 as u64),
                    );
                    gsl_blas_dscal(1.0f64 / Ajj, &mut v1.vector);
                } else {
                    i = 1 as i32 as size_t;
                    while i < M.wrapping_sub(j) {
                        let mut ptr: *mut libc::c_double = gsl_matrix_ptr(
                            A,
                            j.wrapping_add(i),
                            j,
                        );
                        *ptr /= Ajj;
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
            if j < minMN.wrapping_sub(1 as i32 as u64) {
                let mut A22: gsl_matrix_view = gsl_matrix_submatrix(
                    A,
                    j.wrapping_add(1 as i32 as u64),
                    j.wrapping_add(1 as i32 as u64),
                    M.wrapping_sub(j).wrapping_sub(1 as i32 as u64),
                    N.wrapping_sub(j).wrapping_sub(1 as i32 as u64),
                );
                v1 = gsl_matrix_subcolumn(
                    A,
                    j,
                    j.wrapping_add(1 as i32 as u64),
                    M.wrapping_sub(j).wrapping_sub(1 as i32 as u64),
                );
                v2 = gsl_matrix_subrow(
                    A,
                    j,
                    j.wrapping_add(1 as i32 as u64),
                    N.wrapping_sub(j).wrapping_sub(1 as i32 as u64),
                );
                gsl_blas_dger(-1.0f64, &mut v1.vector, &mut v2.vector, &mut A22.matrix);
            }
            j = j.wrapping_add(1);
            j;
        }
        return GSL_SUCCESS as i32;
    };
}
unsafe extern "C" fn LU_decomp_L3(
    mut A: *mut gsl_matrix,
    mut ipiv: *mut gsl_vector_uint,
) -> i32 {
    let M: size_t = (*A).size1;
    let N: size_t = (*A).size2;
    if M < N {
        gsl_error(
            b"matrix must have M >= N\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            212 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*ipiv).size != (if M < N { M } else { N }) {
        gsl_error(
            b"ipiv length must equal MIN(M,N)\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            216 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if N <= 24 as i32 as u64 {
        return LU_decomp_L2(A, ipiv)
    } else {
        let mut status: i32 = 0;
        let N1: size_t = if N >= 16 as i32 as u64 {
            N.wrapping_add(8 as i32 as u64)
                .wrapping_div(16 as i32 as u64)
                .wrapping_mul(8 as i32 as u64)
        } else {
            N.wrapping_div(2 as i32 as u64)
        };
        let N2: size_t = N.wrapping_sub(N1);
        let M2: size_t = M.wrapping_sub(N1);
        let mut A11: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as i32 as size_t,
            0 as i32 as size_t,
            N1,
            N1,
        );
        let mut A12: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as i32 as size_t,
            N1,
            N1,
            N2,
        );
        let mut A21: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            N1,
            0 as i32 as size_t,
            M2,
            N1,
        );
        let mut A22: gsl_matrix_view = gsl_matrix_submatrix(A, N1, N1, M2, N2);
        let mut AL: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as i32 as size_t,
            0 as i32 as size_t,
            M,
            N1,
        );
        let mut AR: gsl_matrix_view = gsl_matrix_submatrix(
            A,
            0 as i32 as size_t,
            N1,
            M,
            N2,
        );
        let mut ipiv1: gsl_vector_uint_view = gsl_vector_uint_subvector(
            ipiv,
            0 as i32 as size_t,
            N1,
        );
        let mut ipiv2: gsl_vector_uint_view = gsl_vector_uint_subvector(ipiv, N1, N2);
        let mut i: size_t = 0;
        status = LU_decomp_L3(&mut AL.matrix, &mut ipiv1.vector);
        if status != 0 {
            return status;
        }
        apply_pivots(&mut AR.matrix, &mut ipiv1.vector);
        gsl_blas_dtrsm(
            CblasLeft,
            CblasLower,
            CblasNoTrans,
            CblasUnit,
            1.0f64,
            &mut A11.matrix,
            &mut A12.matrix,
        );
        gsl_blas_dgemm(
            CblasNoTrans,
            CblasNoTrans,
            -1.0f64,
            &mut A21.matrix,
            &mut A12.matrix,
            1.0f64,
            &mut A22.matrix,
        );
        status = LU_decomp_L3(&mut A22.matrix, &mut ipiv2.vector);
        if status != 0 {
            return status;
        }
        apply_pivots(&mut A21.matrix, &mut ipiv2.vector);
        i = 0 as i32 as size_t;
        while i < N2 {
            let mut ptr: *mut u32 = gsl_vector_uint_ptr(&mut ipiv2.vector, i);
            *ptr = (*ptr as u64).wrapping_add(N1) as u32 as u32;
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_solve(
    mut LU: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
) -> i32 {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            295 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            299 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            303 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size2 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            307 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            311 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut status: i32 = 0;
        gsl_vector_memcpy(x, b);
        status = gsl_linalg_LU_svx(LU, p, x);
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_svx(
    mut LU: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut x: *mut gsl_vector,
) -> i32 {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            333 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            337 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution/rhs size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            341 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            345 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        gsl_permute_vector(p, x);
        gsl_blas_dtrsv(CblasLower, CblasNoTrans, CblasUnit, LU, x);
        gsl_blas_dtrsv(CblasUpper, CblasNoTrans, CblasNonUnit, LU, x);
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_refine(
    mut A: *const gsl_matrix,
    mut LU: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut b: *const gsl_vector,
    mut x: *mut gsl_vector,
    mut work: *mut gsl_vector,
) -> i32 {
    if (*A).size1 != (*A).size2 {
        gsl_error(
            b"matrix a must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            368 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            372 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*A).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be decomposition of a\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            376 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            380 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size1 != (*b).size {
        gsl_error(
            b"matrix size must match b size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            384 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size1 != (*x).size {
        gsl_error(
            b"matrix size must match solution size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            388 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*LU).size1 != (*work).size {
        gsl_error(
            b"matrix size must match workspace size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            392 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            396 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut status: i32 = 0;
        gsl_vector_memcpy(work, b);
        gsl_blas_dgemv(CblasNoTrans, 1.0f64, A, x, -1.0f64, work);
        status = gsl_linalg_LU_svx(LU, p, work);
        gsl_blas_daxpy(-1.0f64, work, x);
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_invert(
    mut LU: *const gsl_matrix,
    mut p: *const gsl_permutation,
    mut inverse: *mut gsl_matrix,
) -> i32 {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            419 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            423 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*inverse).size1 != (*LU).size1 || (*inverse).size2 != (*LU).size2 {
        gsl_error(
            b"inverse matrix must match LU matrix dimensions\0" as *const u8
                as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            427 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        gsl_matrix_memcpy(inverse, LU);
        return gsl_linalg_LU_invx(inverse, p);
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_invx(
    mut LU: *mut gsl_matrix,
    mut p: *const gsl_permutation,
) -> i32 {
    if (*LU).size1 != (*LU).size2 {
        gsl_error(
            b"LU matrix must be square\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            441 as i32,
            GSL_ENOTSQR as i32,
        );
        return GSL_ENOTSQR as i32;
    } else if (*LU).size1 != (*p).size {
        gsl_error(
            b"permutation length must match matrix size\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            445 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if singular(LU) != 0 {
        gsl_error(
            b"matrix is singular\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            449 as i32,
            GSL_EDOM as i32,
        );
        return GSL_EDOM as i32;
    } else {
        let mut status: i32 = 0;
        let N: size_t = (*LU).size1;
        let mut i: size_t = 0;
        status = gsl_linalg_tri_invert(CblasUpper, CblasNonUnit, LU);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_tri_invert(CblasLower, CblasUnit, LU);
        if status != 0 {
            return status;
        }
        status = gsl_linalg_tri_UL(LU);
        if status != 0 {
            return status;
        }
        i = 0 as i32 as size_t;
        while i < N {
            let mut v: gsl_vector_view = gsl_matrix_row(LU, i);
            gsl_permute_vector_inverse(p, &mut v.vector);
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_det(
    mut LU: *mut gsl_matrix,
    mut signum: i32,
) -> libc::c_double {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut det: libc::c_double = signum as libc::c_double;
    i = 0 as i32 as size_t;
    while i < n {
        det *= gsl_matrix_get(LU, i, i);
        i = i.wrapping_add(1);
        i;
    }
    return det;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_lndet(mut LU: *mut gsl_matrix) -> libc::c_double {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut lndet: libc::c_double = 0.0f64;
    i = 0 as i32 as size_t;
    while i < n {
        lndet += log(fabs(gsl_matrix_get(LU, i, i)));
        i = i.wrapping_add(1);
        i;
    }
    return lndet;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_linalg_LU_sgndet(
    mut LU: *mut gsl_matrix,
    mut signum: i32,
) -> i32 {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    let mut s: i32 = signum;
    i = 0 as i32 as size_t;
    while i < n {
        let mut u: libc::c_double = gsl_matrix_get(LU, i, i);
        if u < 0 as i32 as libc::c_double {
            s *= -(1 as i32);
        } else if u == 0 as i32 as libc::c_double {
            s = 0 as i32;
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    return s;
}
unsafe extern "C" fn singular(mut LU: *const gsl_matrix) -> i32 {
    let mut i: size_t = 0;
    let mut n: size_t = (*LU).size1;
    i = 0 as i32 as size_t;
    while i < n {
        let mut u: libc::c_double = gsl_matrix_get(LU, i, i);
        if u == 0 as i32 as libc::c_double {
            return 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
unsafe extern "C" fn apply_pivots(
    mut A: *mut gsl_matrix,
    mut ipiv: *const gsl_vector_uint,
) -> i32 {
    if (*A).size1 < (*ipiv).size {
        gsl_error(
            b"matrix does not match pivot vector\0" as *const u8 as *const i8,
            b"lu.c\0" as *const u8 as *const i8,
            558 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while i < (*ipiv).size {
            let mut pi: size_t = gsl_vector_uint_get(ipiv, i) as size_t;
            if i != pi {
                let mut v1: gsl_vector_view = gsl_matrix_row(A, i);
                let mut v2: gsl_vector_view = gsl_matrix_row(A, pi);
                gsl_blas_dswap(&mut v1.vector, &mut v2.vector);
            }
            i = i.wrapping_add(1);
            i;
        }
        return GSL_SUCCESS as i32;
    };
}