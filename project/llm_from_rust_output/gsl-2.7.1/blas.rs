use libc::{c_char, c_double, c_float, c_int, c_ulong, c_void};
use std::ptr;

pub type size_t = c_ulong;

#[repr(C)]
pub enum CBLAS_ORDER {
    CblasRowMajor = 101,
    CblasColMajor = 102,
}

#[repr(C)]
pub enum CBLAS_TRANSPOSE {
    CblasNoTrans = 111,
    CblasTrans = 112,
    CblasConjTrans = 113,
}

#[repr(C)]
pub enum CBLAS_UPLO {
    CblasUpper = 121,
    CblasLower = 122,
}

#[repr(C)]
pub enum CBLAS_DIAG {
    CblasNonUnit = 131,
    CblasUnit = 132,
}

#[repr(C)]
pub enum CBLAS_SIDE {
    CblasLeft = 141,
    CblasRight = 142,
}

pub type CBLAS_INDEX_t = size_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_complex {
    pub dat: [c_double; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_complex_float {
    pub dat: [c_float; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_block {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_block_complex {
    pub size: size_t,
    pub data: *mut c_double,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_block_float {
    pub size: size_t,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_float,
    pub block: *mut gsl_block_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_block_complex_float {
    pub size: size_t,
    pub data: *mut c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block_complex,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_double,
    pub block: *mut gsl_block,
    pub owner: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut c_float,
    pub block: *mut gsl_block_float,
    pub owner: c_int,
}

#[repr(C)]
pub enum GSL_ERROR {
    GSL_SUCCESS = 0,
    GSL_FAILURE = -1,
    GSL_CONTINUE = -2,
    GSL_EDOM = 1,
    GSL_ERANGE = 2,
    GSL_EFAULT = 3,
    GSL_EINVAL = 4,
    GSL_EFAILED = 5,
    GSL_EFACTOR = 6,
    GSL_ESANITY = 7,
    GSL_ENOMEM = 8,
    GSL_EBADFUNC = 9,
    GSL_ERUNAWAY = 10,
    GSL_EMAXITER = 11,
    GSL_EZERODIV = 12,
    GSL_EBADTOL = 13,
    GSL_ETOL = 14,
    GSL_EUNDRFLW = 15,
    GSL_EOVRFLW = 16,
    GSL_ELOSS = 17,
    GSL_EROUND = 18,
    GSL_EBADLEN = 19,
    GSL_ENOTSQR = 20,
    GSL_ESING = 21,
    GSL_EDIVERGE = 22,
    GSL_EUNSUP = 23,
    GSL_EUNIMPL = 24,
    GSL_ECACHE = 25,
    GSL_ETABLE = 26,
    GSL_ENOPROG = 27,
    GSL_ENOPROGJ = 28,
    GSL_ETOLF = 29,
    GSL_ETOLX = 30,
    GSL_ETOLG = 31,
    GSL_EOF = 32,
}

extern "C" {
    fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);
    fn cblas_sdsdot(
        N: c_int,
        alpha: c_float,
        X: *const c_float,
        incX: c_int,
        Y: *const c_float,
        incY: c_int,
    ) -> c_float;
    fn cblas_dsdot(
        N: c_int,
        X: *const c_float,
        incX: c_int,
        Y: *const c_float,
        incY: c_int,
    ) -> c_double;
    fn cblas_sdot(
        N: c_int,
        X: *const c_float,
        incX: c_int,
        Y: *const c_float,
        incY: c_int,
    ) -> c_float;
    fn cblas_ddot(
        N: c_int,
        X: *const c_double,
        incX: c_int,
        Y: *const c_double,
        incY: c_int,
    ) -> c_double;
    fn cblas_cdotu_sub(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *const c_void,
        incY: c_int,
        dotu: *mut c_void,
    );
    fn cblas_cdotc_sub(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *const c_void,
        incY: c_int,
        dotc: *mut c_void,
    );
    fn cblas_zdotu_sub(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *const c_void,
        incY: c_int,
        dotu: *mut c_void,
    );
    fn cblas_zdotc_sub(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *const c_void,
        incY: c_int,
        dotc: *mut c_void,
    );
    fn cblas_snrm2(N: c_int, X: *const c_float, incX: c_int) -> c_float;
    fn cblas_sasum(N: c_int, X: *const c_float, incX: c_int) -> c_float;
    fn cblas_dnrm2(N: c_int, X: *const c_double, incX: c_int) -> c_double;
    fn cblas_dasum(N: c_int, X: *const c_double, incX: c_int) -> c_double;
    fn cblas_scnrm2(N: c_int, X: *const c_void, incX: c_int) -> c_float;
    fn cblas_scasum(N: c_int, X: *const c_void, incX: c_int) -> c_float;
    fn cblas_dznrm2(N: c_int, X: *const c_void, incX: c_int) -> c_double;
    fn cblas_dzasum(N: c_int, X: *const c_void, incX: c_int) -> c_double;
    fn cblas_isamax(N: c_int, X: *const c_float, incX: c_int) -> size_t;
    fn cblas_idamax(N: c_int, X: *const c_double, incX: c_int) -> size_t;
    fn cblas_icamax(N: c_int, X: *const c_void, incX: c_int) -> size_t;
    fn cblas_izamax(N: c_int, X: *const c_void, incX: c_int) -> size_t;
    fn cblas_sswap(
        N: c_int,
        X: *mut c_float,
        incX: c_int,
        Y: *mut c_float,
        incY: c_int,
    );
    fn cblas_scopy(
        N: c_int,
        X: *const c_float,
        incX: c_int,
        Y: *mut c_float,
        incY: c_int,
    );
    fn cblas_saxpy(
        N: c_int,
        alpha: c_float,
        X: *const c_float,
        incX: c_int,
        Y: *mut c_float,
        incY: c_int,
    );
    fn cblas_dswap(
        N: c_int,
        X: *mut c_double,
        incX: c_int,
        Y: *mut c_double,
        incY: c_int,
    );
    fn cblas_dcopy(
        N: c_int,
        X: *const c_double,
        incX: c_int,
        Y: *mut c_double,
        incY: c_int,
    );
    fn cblas_daxpy(
        N: c_int,
        alpha: c_double,
        X: *const c_double,
        incX: c_int,
        Y: *mut c_double,
        incY: c_int,
    );
    fn cblas_cswap(
        N: c_int,
        X: *mut c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_ccopy(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_caxpy(
        N: c_int,
        alpha: *const c_void,
        X: *const c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_zswap(
        N: c_int,
        X: *mut c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_zcopy(
        N: c_int,
        X: *const c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_zaxpy(
        N: c_int,
        alpha: *const c_void,
        X: *const c_void,
        incX: c_int,
        Y: *mut c_void,
        incY: c_int,
    );
    fn cblas_srotg(
        a: *mut c_float,
        b: *mut c_float,
        c: *mut c_float,
        s: *mut c_float,
    );
    fn cblas_srotmg(
        d1: *mut c_float,
        d2: *mut c_float,
        b1: *mut c_float,
        b2: c_float,
        P: *mut c_float,
    );
    fn cblas_srot(
        N: c_int,
        X: *mut c_float,
        incX: c_int,
        Y: *mut c_float,
        incY: c_int,
        c: c_float,
        s: c_float,
    );
    fn cblas_srotm(
        N: c_int,
        X: *mut c_float,
        incX: c_int,
        Y: *mut c_float,
        incY: c_int,
        P: *const c_float,
    );
    fn cblas_drotg(
        a: *mut c_double,
        b: *mut c_double,
        c: *mut c_double,
        s: *mut c_double,
    );
    fn cblas_drotmg(
        d1: *mut c_double,
        d2: *mut c_double,
        b1: *mut c_double,
        b2: c_double,
        P: *mut c_double,
    );
    fn cblas_drot(
        N: c_int,
        X: *mut c_double,
        incX: c_int,
        Y: *mut c_double,
        incY: c_int,
        c: c_double,
        s: c_double,
    );
    fn cblas_drotm(
        N: c_int,
        X: *mut c_double,
        incX: c_int,
        Y: *mut c_double,
        incY: c_int,
        P: *const c_double,
    );
    fn cblas_sscal(N: c_int, alpha: c_float, X: *mut c_float, incX: c_int);
    fn cblas_dscal(N: c_int, alpha: c_double, X: *mut c_double, incX: c_int);
    fn cblas_cscal(N: c_int, alpha: *const c_void, X: *mut c_void, incX: c_int);
    fn cblas_zscal(N: c_int, alpha: *const c_void, X: *mut c_void, incX: c_int);
    fn cblas_csscal(N: c_int, alpha: c_float, X: *mut c_void, incX: c_int);
    fn cblas_zdscal(N: c_int, alpha: c_double, X: *mut c_void, incX: c_int);
    fn cblas_sgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: c_int,
        N: c_int,
        alpha: c_float,
        A: *const c_float,
        lda: c_int,
        X: *const c_float,
        incX: c_int,
        beta: c_float,
        Y: *mut c_float,
        incY: c_int,
    );
    fn cblas_strmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: c_int,
        A: *const c_float,
        lda: c_int,
        X: *mut c_float,
        incX: c_int,
    );
    fn cblas_strsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: c_int,
        A: *const c_float,
        lda: c_int,
        X: *mut c_float,
        incX: c_int,
    );
    fn cblas_dgemv(
        order: CBLAS_ORDER,
        TransA: CBLAS_TRANSPOSE,
        M: c_int,
        N: c_int,
        alpha: c_double,
        A: *const c_double,
        lda: c_int,
        X: *const c_double,
        incX: c_int,
        beta: c_double,
        Y: *mut c_double,
        incY: c_int,
    );
    fn cblas_dtrmv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: c_int,
        A: *const c_double,
        lda: c_int,
        X: *mut c_double,
        incX: c_int,
    );
    fn cblas_dtrsv(
        order: CBLAS_ORDER,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: