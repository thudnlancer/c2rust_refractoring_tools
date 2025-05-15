use gsl::{Vector, Matrix, BLAS, Linalg};
use gsl::error::{Error, Result};

pub fn hessenberg_decomp(a: &mut Matrix, tau: &mut Vector) -> Result<()> {
    let n = a.size1();
    if n != a.size2() {
        return Err(Error::NotSquare);
    }
    if n != tau.size() {
        return Err(Error::BadLength);
    }
    if n < 3 {
        return Ok(());
    }

    for i in 0..n - 2 {
        let mut c = a.subcolumn(i, i + 1, n - i - 1)?;
        let mut hv = tau.subvector(i + 1, n - i - 1)?;
        hv.copy(&c)?;
        
        let tau_i = Linalg::householder_transform(&mut hv)?;
        
        let mut m = a.submatrix(i + 1, i, n - i - 1, n - i)?;
        Linalg::householder_hm(tau_i, &hv, &mut m)?;
        
        let mut m = a.submatrix(0, i + 1, n, n - i - 1)?;
        Linalg::householder_mh(tau_i, &hv, &mut m)?;
        
        tau.set(i, tau_i)?;
        
        let mut c = c.subvector(1, c.size() - 1)?;
        let mut hv = hv.subvector(1, hv.size() - 1)?;
        c.copy(&hv)?;
    }

    Ok(())
}

pub fn hessenberg_unpack(h: &Matrix, tau: &Vector, u: &mut Matrix) -> Result<()> {
    u.set_identity();
    hessenberg_unpack_accum(h, tau, u)
}

pub fn hessenberg_unpack_accum(h: &Matrix, tau: &Vector, v: &mut Matrix) -> Result<()> {
    let n = h.size1();
    if n != h.size2() {
        return Err(Error::NotSquare);
    }
    if n != tau.size() {
        return Err(Error::BadLength);
    }
    if n != v.size2() {
        return Err(Error::BadLength);
    }
    if n < 3 {
        return Ok(());
    }

    for j in 0..n - 2 {
        let c = h.column(j)?;
        let tau_j = tau.get(j)?;
        let hv = c.subvector(j + 1, n - j - 1)?;
        
        let mut m = v.submatrix(0, j + 1, v.size1(), n - j - 1)?;
        Linalg::householder_mh(tau_j, &hv, &mut m)?;
    }

    Ok(())
}

pub fn hessenberg_set_zero(h: &mut Matrix) -> Result<()> {
    let n = h.size1();
    if n < 3 {
        return Ok(());
    }

    for j in 0..n - 2 {
        for i in j + 2..n {
            h.set(i, j, 0.0)?;
        }
    }

    Ok(())
}

pub fn hessenberg_submatrix(
    m: &mut Matrix,
    a: &mut Matrix,
    top: usize,
    tau: &mut Vector,
) -> Result<()> {
    let n = a.size1();
    let n_m = m.size1();
    
    if n != a.size2() {
        return Err(Error::NotSquare);
    }
    if n != tau.size() {
        return Err(Error::BadLength);
    }
    if n < 3 {
        return Ok(());
    }

    for i in 0..n - 2 {
        let mut c = a.subcolumn(i, i + 1, n - i - 1)?;
        let mut hv = tau.subvector(i + 1, n - i - 1)?;
        hv.copy(&c)?;
        
        let tau_i = Linalg::householder_transform(&mut hv)?;
        
        let mut m_sub = m.submatrix(
            top + i + 1,
            top + i,
            n - i - 1,
            n_m - top - i,
        )?;
        Linalg::householder_hm(tau_i, &hv, &mut m_sub)?;
        
        let mut m_sub = m.submatrix(
            0,
            top + i + 1,
            top + n,
            n - i - 1,
        )?;
        Linalg::householder_mh(tau_i, &hv, &mut m_sub)?;
        
        tau.set(i, tau_i)?;
        
        let mut c = c.subvector(1, c.size() - 1)?;
        let mut hv = hv.subvector(1, hv.size() - 1)?;
        c.copy(&hv)?;
    }

    Ok(())
}