use gsl::{
    blas::{CblasConjTrans, CblasLower, CblasNoTrans, CblasTrans, CblasUpper},
    complex::{Complex, ComplexF64},
    matrix_complex::MatrixComplexF64,
    vector_complex::VectorComplexF64,
    error::{Error, Result},
};

pub fn gsl_linalg_complex_tri_LHL(L: &mut MatrixComplexF64) -> Result<()> {
    triangular_multherm_L3(CblasLower, L)
}

pub fn gsl_linalg_complex_tri_UL(LU: &mut MatrixComplexF64) -> Result<()> {
    triangular_mult_L3(CblasUpper, LU)
}

fn triangular_multherm_L2(uplo: i32, T: &mut MatrixComplexF64) -> Result<()> {
    let N = T.size1();
    if N != T.size2() {
        return Err(Error::new(
            "matrix must be square",
            "trimult_complex.c",
            75,
            gsl::error::ENOTSQR,
        ));
    }

    if uplo != CblasUpper {
        for i in 0..N {
            let Tii = T.get_mut(i, i)?;
            let z0 = *Tii;
            
            if i < N - 1 {
                let mut v = T.subcolumn(i, i + 1, N - i - 1)?;
                let norm = v.norm();
                
                Tii.dat[0] = z0.norm_squared() + norm * norm;
                
                if i > 0 {
                    let mut w = T.subrow(i, 0, i)?;
                    let m = T.submatrix(i + 1, 0, N - i - 1, i)?;
                    
                    complex_conj_vector(&mut w);
                    
                    w.gemv(
                        CblasConjTrans,
                        ComplexF64::new(1.0, 0.0),
                        &m,
                        &v,
                        z0,
                    )?;
                    
                    complex_conj_vector(&mut w);
                }
            } else {
                let mut w = T.row(i)?;
                w.scale(z0.real());
            }
            
            Tii.dat[1] = 0.0;
        }
    }
    
    Ok(())
}

fn triangular_multherm_L3(uplo: i32, T: &mut MatrixComplexF64) -> Result<()> {
    let N = T.size1();
    if N != T.size2() {
        return Err(Error::new(
            "matrix must be square",
            "trimult_complex.c",
            143,
            gsl::error::ENOTSQR,
        ));
    }
    
    if N <= 24 {
        return triangular_multherm_L2(uplo, T);
    }
    
    let N1 = if N >= 8 {
        (N + 4) / 8 * 4
    } else {
        N / 2
    };
    let N2 = N - N1;
    
    let mut T11 = T.submatrix(0, 0, N1, N1)?;
    let mut T12 = T.submatrix(0, N1, N1, N2)?;
    let mut T21 = T.submatrix(N1, 0, N2, N1)?;
    let mut T22 = T.submatrix(N1, N1, N2, N2)?;
    
    triangular_multherm_L3(uplo, &mut T11)?;
    
    if uplo == CblasLower {
        T11.herk(
            uplo,
            CblasConjTrans,
            1.0,
            &T21,
            1.0,
        )?;
        
        T21.trmm(
            CblasLeft,
            uplo,
            CblasConjTrans,
            gsl::blas::CblasNonUnit,
            ComplexF64::new(1.0, 0.0),
            &T22,
        )?;
    } else {
        T11.herk(
            uplo,
            CblasNoTrans,
            1.0,
            &T12,
            1.0,
        )?;
        
        T12.trmm(
            CblasRight,
            uplo,
            CblasConjTrans,
            gsl::blas::CblasNonUnit,
            ComplexF64::new(1.0, 0.0),
            &T22,
        )?;
    }
    
    triangular_multherm_L3(uplo, &mut T22)?;
    
    Ok(())
}

fn triangular_mult_L2(uplo: i32, LU: &mut MatrixComplexF64) -> Result<()> {
    let N = LU.size1();
    if N != LU.size2() {
        return Err(Error::new(
            "matrix must be square",
            "trimult_complex.c",
            221,
            gsl::error::ENOTSQR,
        ));
    }
    
    if N == 1 {
        return Ok(());
    }
    
    if uplo == CblasUpper {
        for i in 0..N {
            let Aii = LU.get_mut(i, i)?;
            let Uii = *Aii;
            
            if i < N - 1 {
                let mut lb = LU.subcolumn(i, i + 1, N - i - 1)?;
                let mut ur = LU.subrow(i, i + 1, N - i - 1)?;
                
                let dot = lb.dot(&ur)?;
                *Aii = *Aii + dot;
                
                if i > 0 {
                    let U_TR = LU.submatrix(0, i + 1, i, N - i - 1)?;
                    let L_BL = LU.submatrix(i + 1, 0, N - i - 1, i)?;
                    let mut ut = LU.subcolumn(i, 0, i)?;
                    let mut ll = LU.subrow(i, 0, i)?;
                    
                    ll.gemv(
                        CblasTrans,
                        ComplexF64::new(1.0, 0.0),
                        &L_BL,
                        &ur,
                        Uii,
                    )?;
                    
                    ut.gemv(
                        CblasNoTrans,
                        ComplexF64::new(1.0, 0.0),
                        &U_TR,
                        &lb,
                        ComplexF64::new(1.0, 0.0),
                    )?;
                }
            } else {
                let mut v = LU.subrow(i, 0, i)?;
                v.scale(Uii);
            }
        }
    }
    
    Ok(())
}

fn triangular_mult_L3(uplo: i32, A: &mut MatrixComplexF64) -> Result<()> {
    let N = A.size1();
    if N != A.size2() {
        return Err(Error::new(
            "matrix must be square",
            "trimult_complex.c",
            293,
            gsl::error::ENOTSQR,
        ));
    }
    
    if N <= 24 {
        return triangular_mult_L2(uplo, A);
    }
    
    let N1 = if N >= 8 {
        (N + 4) / 8 * 4
    } else {
        N / 2
    };
    let N2 = N - N1;
    
    let mut A11 = A.submatrix(0, 0, N1, N1)?;
    let mut A12 = A.submatrix(0, N1, N1, N2)?;
    let mut A21 = A.submatrix(N1, 0, N2, N1)?;
    let mut A22 = A.submatrix(N1, N1, N2, N2)?;
    
    triangular_mult_L3(uplo, &mut A11)?;
    
    if uplo != CblasLower {
        A11.gemm(
            CblasNoTrans,
            CblasNoTrans,
            ComplexF64::new(1.0, 0.0),
            &A12,
            &A21,
            ComplexF64::new(1.0, 0.0),
        )?;
        
        A12.trmm(
            CblasRight,
            CblasLower,
            CblasNoTrans,
            gsl::blas::CblasUnit,
            ComplexF64::new(1.0, 0.0),
            &A22,
        )?;
        
        A21.trmm(
            CblasLeft,
            CblasUpper,
            CblasNoTrans,
            gsl::blas::CblasNonUnit,
            ComplexF64::new(1.0, 0.0),
            &A22,
        )?;
    }
    
    triangular_mult_L3(uplo, &mut A22)?;
    
    Ok(())
}

fn complex_conj_vector(v: &mut VectorComplexF64) {
    for i in 0..v.len() {
        let vi = v.get_mut(i).unwrap();
        vi.dat[1] = -vi.dat[1];
    }
}