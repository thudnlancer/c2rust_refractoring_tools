use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};
use ndarray_linalg::{cblas::Layout, error::LinalgError, types::c32, types::c64, types::f32, types::f64, types::Scalar, types::UPLO, types::Diag, types::Side, types::Transpose, types::NormType, types::JobSvd, types::Order, types::SVDOutput, types::EigVals, types::EigVecs, types::EigValsVecs, types::QR, types::QRP, types::LU, types::Cholesky, types::Hessenberg, types::Schur, types::SVD, types::Eigen, types::EigenSym, types::EigenHerm, types::Solve, types::Inverse, types::PseudoInverse, types::Det, types::LogDet, types::Norm, types::Trace, types::Triangular, types::TriangularMat, types::TriangularMatMut, types::TriangularView, types::TriangularViewMut, types::TriangularInto, types::TriangularMatInto, types::TriangularMatMutInto, types::TriangularViewInto, types::TriangularViewMutInto, types::TriangularMatSlice, types::TriangularMatSliceMut, types::TriangularViewSlice, types::TriangularViewSliceMut, types::TriangularMatSliceInto, types::TriangularMatSliceMutInto, types::TriangularViewSliceInto, types::TriangularViewSliceMutInto, types::TriangularMatDyn, types::TriangularMatMutDyn, types::TriangularViewDyn, types::TriangularViewMutDyn, types::TriangularMatDynInto, types::TriangularMatMutDynInto, types::TriangularViewDynInto, types::TriangularViewMutDynInto, types::TriangularMatSliceDyn, types::TriangularMatSliceMutDyn, types::TriangularViewSliceDyn, types::TriangularViewSliceMutDyn, types::TriangularMatSliceDynInto, types::TriangularMatSliceMutDynInto, types::TriangularViewSliceDynInto, types::TriangularViewSliceMutDynInto, types::TriangularMat, types::TriangularMatMut, types::TriangularView, types::TriangularViewMut, types::TriangularInto, types::TriangularMatInto, types::TriangularMatMutInto, types::TriangularViewInto, types::TriangularViewMutInto, types::TriangularMatSlice, types::TriangularMatSliceMut, types::TriangularViewSlice, types::TriangularViewSliceMut, types::TriangularMatSliceInto, types::TriangularMatSliceMutInto, types::TriangularViewSliceInto, types::TriangularViewSliceMutInto, types::TriangularMatDyn, types::TriangularMatMutDyn, types::TriangularViewDyn, types::TriangularViewMutDyn, types::TriangularMatDynInto, types::TriangularMatMutDynInto, types::TriangularViewDynInto, types::TriangularViewMutDynInto, types::TriangularMatSliceDyn, types::TriangularMatSliceMutDyn, types::TriangularViewSliceDyn, types::TriangularViewSliceMutDyn, types::TriangularMatSliceDynInto, types::TriangularMatSliceMutDynInto, types::TriangularViewSliceDynInto, types::TriangularViewSliceMutDynInto};
use ndarray_linalg::solve::TriangularSolve;
use ndarray_linalg::norm::Norm;
use ndarray_linalg::Scalar;
use ndarray_linalg::error::Result;

const CROSSOVER_TRIMULT: usize = 64;

pub fn tri_ltl(l: &mut Array2<f64>) -> Result<()> {
    triangular_multsymm_l3(UPLO::Lower, l)
}

pub fn tri_ul(lu: &mut Array2<f64>) -> Result<()> {
    triangular_mult_l3(UPLO::Upper, lu)
}

fn triangular_multsymm_l2(uplo: UPLO, t: &mut Array2<f64>) -> Result<()> {
    let n = t.nrows();
    if n != t.ncols() {
        return Err(LinalgError::NotSquare);
    }

    match uplo {
        UPLO::Upper => {
            // Not implemented in original C code
        }
        UPLO::Lower => {
            for i in 0..n {
                let tii = t[(i, i)];

                if i < n - 1 {
                    let mut v1 = t.slice_mut(s![i.., i]);
                    let tmp = v1.dot(&v1);
                    t[(i, i)] = tmp;

                    if i > 0 {
                        let m = t.slice(s![i+1.., ..i]);
                        let mut v1 = t.slice_mut(s![i+1.., i]);
                        let mut v2 = t.slice_mut(s![i, ..i]);

                        let tii = t[(i, i)];
                        v2.scaled_add(tii, &m.t().dot(&v1));
                    }
                } else {
                    let mut v1 = t.slice_mut(s![n-1, ..]);
                    v1 *= tii;
                }
            }
        }
    }

    Ok(())
}

fn triangular_multsymm_l3(uplo: UPLO, t: &mut Array2<f64>) -> Result<()> {
    let n = t.nrows();
    if n != t.ncols() {
        return Err(LinalgError::NotSquare);
    } else if n <= CROSSOVER_TRIMULT {
        return triangular_multsymm_l2(uplo, t);
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let (mut t11, rest) = t.view_mut().split_at(Axis(0), n1);
    let (mut t12, mut t22) = rest.split_at(Axis(1), n1);
    let (mut t21, mut t22) = t.slice_mut(s![n1.., ..]).split_at(Axis(1), n1);

    triangular_multsymm_l3(uplo, &mut t11)?;

    match uplo {
        UPLO::Lower => {
            // T11 += T21^T T21
            t11.syrk(1.0, &t21.t(), 1.0)?;
            // T21 = T22^T * T21
            t21.triangular_solve(UPLO::Lower, Transpose::Trans, Diag::NonUnit, 1.0, &t22)?;
        }
        UPLO::Upper => {
            // T11 += T12 T12^T
            t11.syrk(1.0, &t12, 1.0)?;
            // T12 = T12 * T22^T
            t12.triangular_solve(UPLO::Upper, Transpose::Trans, Diag::NonUnit, 1.0, &t22)?;
        }
    }

    triangular_multsymm_l3(uplo, &mut t22)?;

    Ok(())
}

fn triangular_mult_l2(uplo: UPLO, a: &mut Array2<f64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    }

    if n == 1 {
        return Ok(());
    }

    match uplo {
        UPLO::Upper => {
            for i in 0..n {
                let uii = a[(i, i)];

                if i < n - 1 {
                    let mut lb = a.slice_mut(s![i+1.., i]);
                    let mut ur = a.slice_mut(s![i, i+1..]);
                    let tmp = lb.dot(&ur);
                    a[(i, i)] += tmp;

                    if i > 0 {
                        let u_tr = a.slice(s![..i, i+1..]);
                        let l_bl = a.slice(s![i+1.., ..i]);
                        let mut ut = a.slice_mut(s![..i, i]);
                        let mut ll = a.slice_mut(s![i, ..i]);

                        ll.scaled_add(uii, &l_bl.t().dot(&ur));
                        ut.scaled_add(1.0, &u_tr.dot(&lb));
                    }
                } else {
                    let mut v = a.slice_mut(s![n-1, ..n-1]);
                    v *= uii;
                }
            }
        }
        UPLO::Lower => {
            // Not implemented in original C code
        }
    }

    Ok(())
}

fn triangular_mult_l3(uplo: UPLO, a: &mut Array2<f64>) -> Result<()> {
    let n = a.nrows();
    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    } else if n <= CROSSOVER_TRIMULT {
        return triangular_mult_l2(uplo, a);
    }

    let n1 = n / 2;
    let n2 = n - n1;

    let (mut a11, rest) = a.view_mut().split_at(Axis(0), n1);
    let (mut a12, mut a22) = rest.split_at(Axis(1), n1);
    let (mut a21, mut a22) = a.slice_mut(s![n1.., ..]).split_at(Axis(1), n1);

    triangular_mult_l3(uplo, &mut a11)?;

    match uplo {
        UPLO::Lower => {
            // Not implemented in original C code
        }
        UPLO::Upper => {
            // A11 += A12 A21
            a11.gemm(1.0, &a12, &a21, 1.0)?;
            // A12 = A12 * L22
            a12.triangular_solve(UPLO::Lower, Transpose::NoTrans, Diag::Unit, 1.0, &a22)?;
            // A21 = U22 * A21
            a21.triangular_solve(UPLO::Upper, Transpose::NoTrans, Diag::NonUnit, 1.0, &a22)?;
        }
    }

    triangular_mult_l3(uplo, &mut a22)?;

    Ok(())
}