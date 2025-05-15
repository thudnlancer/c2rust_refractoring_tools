use gsl::{
    blas::{CBLAS_TRANSPOSE, CblasConjTrans},
    cblas::hypot,
    complex::{Complex, Complex64},
    error::{Error, Result},
    matrix_complex::MatrixComplex,
    vector_complex::{VectorComplex, VectorComplexView, VectorComplexConstView},
};

pub fn complex_householder_transform(v: &mut VectorComplex) -> Complex64 {
    let n = v.size();
    if n == 1 {
        let alpha = v.get(0);
        let absa = alpha.abs();
        let beta_r = -alpha.re().signum() * absa;
        
        if beta_r == 0.0 {
            Complex64::new(0.0, 0.0)
        } else {
            let tau = Complex64::new(
                (beta_r - alpha.re()) / beta_r,
                -alpha.im() / beta_r,
            );
            v.set(0, Complex64::new(beta_r, 0.0));
            tau
        }
    } else {
        let mut tau = Complex64::new(0.0, 0.0);
        let mut beta_r = 0.0;
        
        let mut x = v.subvector(1, n - 1);
        let alpha = v.get(0);
        let absa = alpha.abs();
        let xnorm = x.nrm2();
        
        if xnorm == 0.0 && alpha.im() == 0.0 {
            return Complex64::new(0.0, 0.0);
        }
        
        beta_r = -alpha.re().signum() * hypot(absa, xnorm);
        tau = Complex64::new(
            (beta_r - alpha.re()) / beta_r,
            -alpha.im() / beta_r,
        );
        
        let amb = alpha - beta_r;
        let s = amb.inverse();
        x.scale(&s);
        
        v.set(0, Complex64::new(beta_r, 0.0));
        tau
    }
}

pub fn complex_householder_hv(
    tau: Complex64,
    v: &VectorComplex,
    w: &mut VectorComplex,
) -> Result<()> {
    let n = v.size();
    
    if tau.is_zero() {
        return Ok(());
    }
    
    if n == 1 {
        let w0 = w.get(0);
        let a = Complex64::new(1.0 - tau.re(), -tau.im());
        w.set(0, a * w0);
    } else {
        let z0 = w.get(0);
        let v1 = v.subvector_const(1, n - 1);
        let mut w1 = w.subvector(1, n - 1);
        
        let z1 = v1.dotc(&w1)?;
        let z = z0 + z1;
        let tz = tau * z;
        let ntz = -tz;
        
        let w0 = w.get(0);
        w.set(0, w0 + ntz);
        w1.axpy(&ntz, &v1)?;
    }
    
    Ok(())
}

pub fn complex_householder_left(
    tau: Complex64,
    v: &VectorComplex,
    a: &mut MatrixComplex,
    work: &mut VectorComplex,
) -> Result<()> {
    let m = a.size1();
    let n = a.size2();
    
    if v.size() != m {
        return Err(Error::EBADLEN);
    }
    if work.size() != n {
        return Err(Error::EBADLEN);
    }
    
    if tau.is_zero() {
        return Ok(());
    }
    
    let v0 = v.get(0);
    unsafe {
        *v.ptr_mut(0) = Complex64::new(1.0, 0.0);
    }
    
    a.gemv(
        CblasConjTrans,
        Complex64::new(1.0, 0.0),
        v,
        Complex64::new(0.0, 0.0),
        work,
    )?;
    
    let mtau = -tau;
    a.gerc(&mtau, v, work)?;
    
    unsafe {
        *v.ptr_mut(0) = v0;
    }
    
    Ok(())
}

pub fn complex_householder_hm(
    tau: Complex64,
    v: &VectorComplex,
    a: &mut MatrixComplex,
) -> Result<()> {
    if tau.is_zero() {
        return Ok(());
    }
    
    for j in 0..a.size2() {
        let mut wj = a.get(0, j);
        
        for i in 1..a.size1() {
            let aij = a.get(i, j);
            let vi = v.get(i);
            wj += aij * vi.conj();
        }
        
        let tauwj = tau * wj;
        let a0j = a.get(0, j);
        a.set(0, j, a0j - tauwj);
        
        for i in 1..a.size1() {
            let vi = v.get(i);
            let tauvw = vi * tauwj;
            let aij = a.get(i, j);
            a.set(i, j, aij - tauvw);
        }
    }
    
    Ok(())
}

pub fn complex_householder_mh(
    tau: Complex64,
    v: &VectorComplex,
    a: &mut MatrixComplex,
) -> Result<()> {
    if tau.is_zero() {
        return Ok(());
    }
    
    for i in 0..a.size1() {
        let ai0 = a.get(i, 0);
        let mut wi = ai0;
        
        for j in 1..a.size2() {
            let aij = a.get(i, j);
            let vj = v.get(j);
            wi += aij * vj;
        }
        
        let tauwi = tau * wi;
        a.set(i, 0, ai0 - tauwi);
        
        for j in 1..a.size2() {
            let vj = v.get(j);
            let tauwv = vj.conj() * tauwi;
            let aij = a.get(i, j);
            a.set(i, j, aij - tauwv);
        }
    }
    
    Ok(())
}