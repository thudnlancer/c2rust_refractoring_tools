use gsl::{
    blas::{CblasDiag, CblasSide, CblasTranspose, CblasUplo},
    error::{GslError, GslResult},
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
    VectorUint,
};

pub fn lu_decomp(
    a: &mut Matrix,
    p: &mut Permutation,
    signum: &mut i32,
) -> GslResult<()> {
    let m = a.size1();
    if p.size() != m {
        return Err(GslError::BadLen);
    }

    let n = a.size2();
    let min_mn = m.min(n);
    let mut ipiv = VectorUint::new(min_mn)?;

    let mut al = a.submatrix(0, 0, m, min_mn)?;
    lu_decomp_l3(&mut al, &mut ipiv)?;

    if m < n {
        let mut ar = a.submatrix(0, m, m, n - m)?;
        apply_pivots(&mut ar, &ipiv)?;
        ar.trsm(
            CblasSide::Left,
            CblasUplo::Lower,
            CblasTranspose::NoTrans,
            CblasDiag::Unit,
            1.0,
            &al,
        )?;
    }

    p.init();
    *signum = 1;

    for i in 0..min_mn {
        let pivi = ipiv.get(i)? as usize;
        if p.get(pivi) != p.get(i) {
            p.swap(pivi, i);
            *signum = -*signum;
        }
    }

    Ok(())
}

fn lu_decomp_l2(a: &mut Matrix, ipiv: &mut VectorUint) -> GslResult<()> {
    let m = a.size1();
    let n = a.size2();
    let min_mn = m.min(n);

    if ipiv.size() != min_mn {
        return Err(GslError::BadLen);
    }

    for j in 0..min_mn {
        let mut v = a.subcolumn(j, j, m - j)?;
        let j_pivot = j + v.idamax()?;

        ipiv.set(j, j_pivot as u32)?;

        if j_pivot != j {
            let mut v1 = a.row(j)?;
            let mut v2 = a.row(j_pivot)?;
            v1.swap(&mut v2)?;
        }

        if j < m - 1 {
            let ajj = a.get(j, j)?;
            if ajj.abs() >= f64::MIN_POSITIVE {
                let mut v1 = a.subcolumn(j, j + 1, m - j - 1)?;
                v1.scale(1.0 / ajj)?;
            } else {
                for i in 1..m - j {
                    let ptr = a.get_mut(j + i, j)?;
                    *ptr /= ajj;
                }
            }
        }

        if j < min_mn - 1 {
            let mut a22 = a.submatrix(j + 1, j + 1, m - j - 1, n - j - 1)?;
            let mut v1 = a.subcolumn(j, j + 1, m - j - 1)?;
            let mut v2 = a.subrow(j, j + 1, n - j - 1)?;
            a22.ger(-1.0, &v1, &v2)?;
        }
    }

    Ok(())
}

fn lu_decomp_l3(a: &mut Matrix, ipiv: &mut VectorUint) -> GslResult<()> {
    let m = a.size1();
    let n = a.size2();

    if m < n {
        return Err(GslError::BadLen);
    }

    if ipiv.size() != m.min(n) {
        return Err(GslError::BadLen);
    }

    if n <= 24 {
        return lu_decomp_l2(a, ipiv);
    }

    let n1 = if n >= 16 {
        ((n + 8) / 16) * 8
    } else {
        n / 2
    };

    let n2 = n - n1;
    let m2 = m - n1;

    let mut a11 = a.submatrix(0, 0, n1, n1)?;
    let mut a12 = a.submatrix(0, n1, n1, n2)?;
    let mut a21 = a.submatrix(n1, 0, m2, n1)?;
    let mut a22 = a.submatrix(n1, n1, m2, n2)?;

    let mut al = a.submatrix(0, 0, m, n1)?;
    let mut ar = a.submatrix(0, n1, m, n2)?;

    let mut ipiv1 = ipiv.subvector(0, n1)?;
    let mut ipiv2 = ipiv.subvector(n1, n2)?;

    lu_decomp_l3(&mut al, &mut ipiv1)?;
    apply_pivots(&mut ar, &ipiv1)?;

    a12.trsm(
        CblasSide::Left,
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::Unit,
        1.0,
        &a11,
    )?;

    a22.gemm(
        CblasTranspose::NoTrans,
        CblasTranspose::NoTrans,
        -1.0,
        &a21,
        &a12,
        1.0,
    )?;

    lu_decomp_l3(&mut a22, &mut ipiv2)?;
    apply_pivots(&mut a21, &ipiv2)?;

    for i in 0..n2 {
        let ptr = ipiv2.get_mut(i)?;
        *ptr = (*ptr as usize + n1) as u32;
    }

    Ok(())
}

pub fn lu_solve(
    lu: &Matrix,
    p: &Permutation,
    b: &Vector,
    x: &mut Vector,
) -> GslResult<()> {
    if lu.size1() != lu.size2() {
        return Err(GslError::NotSquare);
    }

    if lu.size1() != p.size() {
        return Err(GslError::BadLen);
    }

    if lu.size1() != b.size() {
        return Err(GslError::BadLen);
    }

    if lu.size2() != x.size() {
        return Err(GslError::BadLen);
    }

    if is_singular(lu) {
        return Err(GslError::Dom);
    }

    x.copy_from(b)?;
    lu_svx(lu, p, x)
}

pub fn lu_svx(lu: &Matrix, p: &Permutation, x: &mut Vector) -> GslResult<()> {
    if lu.size1() != lu.size2() {
        return Err(GslError::NotSquare);
    }

    if lu.size1() != p.size() {
        return Err(GslError::BadLen);
    }

    if lu.size1() != x.size() {
        return Err(GslError::BadLen);
    }

    if is_singular(lu) {
        return Err(GslError::Dom);
    }

    p.permute(x)?;
    x.trsv(
        CblasUplo::Lower,
        CblasTranspose::NoTrans,
        CblasDiag::Unit,
        lu,
    )?;
    x.trsv(
        CblasUplo::Upper,
        CblasTranspose::NoTrans,
        CblasDiag::NonUnit,
        lu,
    )?;

    Ok(())
}

fn is_singular(lu: &Matrix) -> bool {
    let n = lu.size1();
    for i in 0..n {
        if lu.get(i, i).unwrap_or(1.0) == 0.0 {
            return true;
        }
    }
    false
}

fn apply_pivots(a: &mut Matrix, ipiv: &VectorUint) -> GslResult<()> {
    if a.size1() < ipiv.size() {
        return Err(GslError::BadLen);
    }

    for i in 0..ipiv.size() {
        let pi = ipiv.get(i)? as usize;
        if i != pi {
            let mut v1 = a.row(i)?;
            let mut v2 = a.row(pi)?;
            v1.swap(&mut v2)?;
        }
    }

    Ok(())
}