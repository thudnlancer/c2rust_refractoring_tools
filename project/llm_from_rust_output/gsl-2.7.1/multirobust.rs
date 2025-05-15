use gsl::{
    blas::Level1,
    error::{Error, Result},
    linalg::Matrix,
    matrix::MatrixF64,
    multifit::robust::{Workspace, Stats, Type},
    vector::VectorF64,
    stats::{mean, sd, tss, median_from_sorted_data},
    sort::sort_vector,
};

pub fn robust_alloc(T: &Type, n: usize, p: usize) -> Result<Workspace> {
    if n < p {
        return Err(Error::InvalidArgument("observations n must be >= p"));
    }

    let mut w = Workspace::new(n, p, T)?;

    w.maxiter = 100;
    w.tune = T.tuning_default;

    Ok(w)
}

pub fn robust_free(w: &mut Workspace) {
    w.free();
}

pub fn robust_tune(tune: f64, w: &mut Workspace) -> Result<()> {
    w.tune = tune;
    Ok(())
}

pub fn robust_maxiter(maxiter: usize, w: &mut Workspace) -> Result<()> {
    if maxiter == 0 {
        Err(Error::InvalidArgument("maxiter must be greater than 0"))
    } else {
        w.maxiter = maxiter;
        Ok(())
    }
}

pub fn robust_name(w: &Workspace) -> &str {
    w.type_.name
}

pub fn robust_statistics(w: &Workspace) -> Stats {
    w.stats.clone()
}

pub fn robust_weights(r: &VectorF64, wts: &mut VectorF64, w: &mut Workspace) -> Result<()> {
    if r.len() != wts.len() {
        return Err(Error::InvalidArgument("residual vector does not match weight vector size"));
    }

    let sigma = robust_madsigma(r, w.p, &mut w.workn)?;
    wts.copy_from(r)?;
    
    if sigma > 0.0 {
        wts.scale(1.0 / (sigma * w.tune))?;
    }

    (w.type_.wfun)(wts, wts)
}

pub fn robust(
    X: &MatrixF64,
    y: &VectorF64,
    c: &mut VectorF64,
    cov: &mut MatrixF64,
    w: &mut Workspace,
) -> Result<()> {
    // Input validation
    if X.size1() != y.len() {
        return Err(Error::InvalidArgument("number of observations in y does not match rows of matrix X"));
    }
    if X.size2() != c.len() {
        return Err(Error::InvalidArgument("number of parameters c does not match columns of matrix X"));
    }
    if cov.size1() != cov.size2() {
        return Err(Error::InvalidArgument("covariance matrix is not square"));
    }
    if c.len() != cov.size1() {
        return Err(Error::InvalidArgument("number of parameters does not match size of covariance matrix"));
    }
    if X.size1() != w.n || X.size2() != w.p {
        return Err(Error::InvalidArgument("size of workspace does not match size of observation matrix"));
    }

    let tol = 1.4901161193847656e-08;
    let mut converged = false;
    let mut numit = 0;
    let n = y.len();
    let sigy = sd(y.data(), y.stride(), n)?;
    let sig_lower = 1.0e-6 * sigy.max(1.0);

    // Initial OLS fit
    let mut chisq = 0.0;
    w.multifit_linear(X, y, c, cov, &mut chisq)?;
    
    // Compute leverage factors
    w.compute_leverage()?;
    
    // Scale residuals by leverage factors
    w.scale_residuals()?;

    // Compute initial statistics
    w.stats.sigma_ols = w.r.nrm2()? / (w.stats.dof as f64).sqrt();

    // Iteratively reweighted least squares
    while !converged && numit < w.maxiter {
        numit += 1;
        
        // Compute weights
        let sig = robust_madsigma(&w.r, w.p, &mut w.workn)?;
        w.r.scale(1.0 / (sig.max(sig_lower) * w.tune)?;
        (w.type_.wfun)(&w.r, &mut w.weights)?;
        
        // Save previous coefficients
        w.c_prev.copy_from(c)?;
        
        // Weighted least squares fit
        w.multifit_wlinear(X, &w.weights, y, c, cov, &mut chisq)?;
        
        // Compute new residuals
        w.multifit_linear_residuals(X, y, c, &mut w.r)?;
        
        // Test convergence
        converged = robust_test_convergence(&w.c_prev, c, tol)?;
    }

    // Final statistics
    w.compute_final_stats()?;

    if numit > w.maxiter {
        Err(Error::MaxIteration)
    } else {
        Ok(())
    }
}

// Helper functions
fn robust_test_convergence(c_prev: &VectorF64, c: &VectorF64, tol: f64) -> Result<bool> {
    for i in 0..c.len() {
        let ai = c_prev.get(i)?;
        let bi = c.get(i)?;
        if (bi - ai).abs() > tol * ai.abs().max(bi.abs()) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn robust_madsigma(r: &VectorF64, p: usize, workn: &mut VectorF64) -> Result<f64> {
    let n = r.len();
    for i in 0..n {
        workn.set(i, r.get(i)?.abs())?;
    }
    sort_vector(workn)?;
    
    let v2 = workn.subvector(p-1, n-p+1)?;
    Ok(median_from_sorted_data(v2.data(), v2.stride(), v2.len())? / 0.6745)
}

// Additional helper functions would follow similar patterns