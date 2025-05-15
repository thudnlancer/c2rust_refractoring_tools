use libc::{c_char, c_int, c_uint, c_ulong, c_float, c_double};
use gsl_sys::{gsl_matrix, gsl_matrix_float, gsl_matrix_complex, gsl_matrix_complex_float};
use gsl_sys::{gsl_vector, gsl_vector_float, gsl_vector_complex, gsl_vector_complex_float};
use gsl_sys::{gsl_blas_dcopy, gsl_blas_scopy, gsl_blas_zcopy, gsl_blas_ccopy};
use gsl_sys::{gsl_blas_dswap, gsl_blas_sswap, gsl_blas_zswap, gsl_blas_cswap};
use gsl_sys::{gsl_matrix_const_diagonal, gsl_matrix_diagonal};
use gsl_sys::{gsl_matrix_const_subrow, gsl_matrix_subrow};
use gsl_sys::{gsl_matrix_complex_const_diagonal, gsl_matrix_complex_diagonal};
use gsl_sys::{gsl_matrix_complex_const_subrow, gsl_matrix_complex_subrow};
use gsl_sys::{gsl_matrix_float_const_diagonal, gsl_matrix_float_diagonal};
use gsl_sys::{gsl_matrix_float_const_subrow, gsl_matrix_float_subrow};
use gsl_sys::{gsl_matrix_complex_float_const_diagonal, gsl_matrix_complex_float_diagonal};
use gsl_sys::{gsl_matrix_complex_float_const_subrow, gsl_matrix_complex_float_subrow};
use gsl_sys::{gsl_matrix_const_row, gsl_matrix_row};
use gsl_sys::{gsl_matrix_complex_const_row, gsl_matrix_complex_row};
use gsl_sys::{gsl_matrix_float_const_row, gsl_matrix_float_row};
use gsl_sys::{gsl_matrix_complex_float_const_row, gsl_matrix_complex_float_row};
use gsl_sys::{GSL_SUCCESS, GSL_EBADLEN, GSL_EINVAL};

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

pub type size_t = c_ulong;

pub fn gsl_matrix_memcpy(dest: &mut gsl_matrix, src: &gsl_matrix) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let sv = unsafe { gsl_matrix_const_row(src, i) };
        let mut dv = unsafe { gsl_matrix_row(dest, i) };
        unsafe { gsl_blas_dcopy(&sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_float_memcpy(dest: &mut gsl_matrix_float, src: &gsl_matrix_float) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let sv = unsafe { gsl_matrix_float_const_row(src, i) };
        let mut dv = unsafe { gsl_matrix_float_row(dest, i) };
        unsafe { gsl_blas_scopy(&sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_memcpy(dest: &mut gsl_matrix_complex, src: &gsl_matrix_complex) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let sv = unsafe { gsl_matrix_complex_const_row(src, i) };
        let mut dv = unsafe { gsl_matrix_complex_row(dest, i) };
        unsafe { gsl_blas_zcopy(&sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_float_memcpy(
    dest: &mut gsl_matrix_complex_float,
    src: &gsl_matrix_complex_float,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let sv = unsafe { gsl_matrix_complex_float_const_row(src, i) };
        let mut dv = unsafe { gsl_matrix_complex_float_row(dest, i) };
        unsafe { gsl_blas_ccopy(&sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_swap(dest: &mut gsl_matrix, src: &mut gsl_matrix) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let mut sv = unsafe { gsl_matrix_row(src, i) };
        let mut dv = unsafe { gsl_matrix_row(dest, i) };
        unsafe { gsl_blas_dswap(&mut sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_float_swap(dest: &mut gsl_matrix_float, src: &mut gsl_matrix_float) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let mut sv = unsafe { gsl_matrix_float_row(src, i) };
        let mut dv = unsafe { gsl_matrix_float_row(dest, i) };
        unsafe { gsl_blas_sswap(&mut sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_swap(dest: &mut gsl_matrix_complex, src: &mut gsl_matrix_complex) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let mut sv = unsafe { gsl_matrix_complex_row(src, i) };
        let mut dv = unsafe { gsl_matrix_complex_row(dest, i) };
        unsafe { gsl_blas_zswap(&mut sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_float_swap(
    dest: &mut gsl_matrix_complex_float,
    src: &mut gsl_matrix_complex_float,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    for i in 0..src.size1 {
        let mut sv = unsafe { gsl_matrix_complex_float_row(src, i) };
        let mut dv = unsafe { gsl_matrix_complex_float_row(dest, i) };
        unsafe { gsl_blas_cswap(&mut sv.vector, &mut dv.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_tricpy(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    dest: &mut gsl_matrix,
    src: &gsl_matrix,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    match uplo {
        CBLAS_UPLO::CblasLower => {
            for i in 1..src.size1 {
                let k = if i < src.size2 { i } else { src.size2 };
                let a = unsafe { gsl_matrix_const_subrow(src, i, 0, k) };
                let mut b = unsafe { gsl_matrix_subrow(dest, i, 0, k) };
                unsafe { gsl_blas_dcopy(&a.vector, &mut b.vector) };
            }
        }
        CBLAS_UPLO::CblasUpper => {
            let max_i = if src.size1 < src.size2 - 1 {
                src.size1
            } else {
                src.size2 - 1
            };
            for i in 0..max_i {
                let a = unsafe {
                    gsl_matrix_const_subrow(
                        src,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                let mut b = unsafe {
                    gsl_matrix_subrow(
                        dest,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                unsafe { gsl_blas_dcopy(&a.vector, &mut b.vector) };
            }
        }
        _ => return GSL_EINVAL,
    }

    if let CBLAS_DIAG::CblasNonUnit = diag {
        let a = unsafe { gsl_matrix_const_diagonal(src) };
        let mut b = unsafe { gsl_matrix_diagonal(dest) };
        unsafe { gsl_blas_dcopy(&a.vector, &mut b.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_float_tricpy(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    dest: &mut gsl_matrix_float,
    src: &gsl_matrix_float,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    match uplo {
        CBLAS_UPLO::CblasLower => {
            for i in 1..src.size1 {
                let k = if i < src.size2 { i } else { src.size2 };
                let a = unsafe { gsl_matrix_float_const_subrow(src, i, 0, k) };
                let mut b = unsafe { gsl_matrix_float_subrow(dest, i, 0, k) };
                unsafe { gsl_blas_scopy(&a.vector, &mut b.vector) };
            }
        }
        CBLAS_UPLO::CblasUpper => {
            let max_i = if src.size1 < src.size2 - 1 {
                src.size1
            } else {
                src.size2 - 1
            };
            for i in 0..max_i {
                let a = unsafe {
                    gsl_matrix_float_const_subrow(
                        src,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                let mut b = unsafe {
                    gsl_matrix_float_subrow(
                        dest,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                unsafe { gsl_blas_scopy(&a.vector, &mut b.vector) };
            }
        }
        _ => return GSL_EINVAL,
    }

    if let CBLAS_DIAG::CblasNonUnit = diag {
        let a = unsafe { gsl_matrix_float_const_diagonal(src) };
        let mut b = unsafe { gsl_matrix_float_diagonal(dest) };
        unsafe { gsl_blas_scopy(&a.vector, &mut b.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_tricpy(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    dest: &mut gsl_matrix_complex,
    src: &gsl_matrix_complex,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    match uplo {
        CBLAS_UPLO::CblasLower => {
            for i in 1..src.size1 {
                let k = if i < src.size2 { i } else { src.size2 };
                let a = unsafe { gsl_matrix_complex_const_subrow(src, i, 0, k) };
                let mut b = unsafe { gsl_matrix_complex_subrow(dest, i, 0, k) };
                unsafe { gsl_blas_zcopy(&a.vector, &mut b.vector) };
            }
        }
        CBLAS_UPLO::CblasUpper => {
            let max_i = if src.size1 < src.size2 - 1 {
                src.size1
            } else {
                src.size2 - 1
            };
            for i in 0..max_i {
                let a = unsafe {
                    gsl_matrix_complex_const_subrow(
                        src,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                let mut b = unsafe {
                    gsl_matrix_complex_subrow(
                        dest,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                unsafe { gsl_blas_zcopy(&a.vector, &mut b.vector) };
            }
        }
        _ => return GSL_EINVAL,
    }

    if let CBLAS_DIAG::CblasNonUnit = diag {
        let a = unsafe { gsl_matrix_complex_const_diagonal(src) };
        let mut b = unsafe { gsl_matrix_complex_diagonal(dest) };
        unsafe { gsl_blas_zcopy(&a.vector, &mut b.vector) };
    }

    GSL_SUCCESS
}

pub fn gsl_matrix_complex_float_tricpy(
    uplo: CBLAS_UPLO,
    diag: CBLAS_DIAG,
    dest: &mut gsl_matrix_complex_float,
    src: &gsl_matrix_complex_float,
) -> c_int {
    if src.size1 != dest.size1 || src.size2 != dest.size2 {
        return GSL_EBADLEN;
    }

    match uplo {
        CBLAS_UPLO::CblasLower => {
            for i in 1..src.size1 {
                let k = if i < src.size2 { i } else { src.size2 };
                let a = unsafe { gsl_matrix_complex_float_const_subrow(src, i, 0, k) };
                let mut b = unsafe { gsl_matrix_complex_float_subrow(dest, i, 0, k) };
                unsafe { gsl_blas_ccopy(&a.vector, &mut b.vector) };
            }
        }
        CBLAS_UPLO::CblasUpper => {
            let max_i = if src.size1 < src.size2 - 1 {
                src.size1
            } else {
                src.size2 - 1
            };
            for i in 0..max_i {
                let a = unsafe {
                    gsl_matrix_complex_float_const_subrow(
                        src,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                let mut b = unsafe {
                    gsl_matrix_complex_float_subrow(
                        dest,
                        i,
                        i + 1,
                        src.size2 - i - 1,
                    )
                };
                unsafe { gsl_blas_ccopy(&a.vector, &mut b.vector) };
            }
        }
        _ => return GSL_EINVAL,
    }

    if let CBLAS_DIAG::CblasNonUnit = diag {
        let a = unsafe { gsl_matrix_complex_float_const_diagonal(src) };
        let mut b = unsafe { gsl_matrix_complex_float_diagonal(dest) };
        unsafe { gsl_blas_ccopy(&a.vector, &mut b.vector) };
    }

    GSL_SUCCESS
}