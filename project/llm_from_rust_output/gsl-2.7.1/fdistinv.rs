use thiserror::Error;

#[derive(Error, Debug)]
pub enum GslError {
    #[error("Domain error (GSL_EDOM)")]
    DomainError,
    #[error("Range error (GSL_ERANGE)")]
    RangeError,
    // Add other GSL error variants as needed
}

pub fn gsl_cdf_fdist_pinv(p: f64, nu1: f64, nu2: f64) -> Result<f64, GslError> {
    if p < 0.0 {
        return Err(GslError::DomainError);
    }
    if p > 1.0 {
        return Err(GslError::DomainError);
    }
    if nu1 < 1.0 {
        return Err(GslError::DomainError);
    }
    if nu2 < 1.0 {
        return Err(GslError::DomainError);
    }

    let y = if p < 0.5 {
        // Assume safe wrapper exists for gsl_cdf_beta_Pinv
        safe_gsl_cdf_beta_pinv(p, nu1 / 2.0, nu2 / 2.0)?
    } else {
        // Assume safe wrapper exists for gsl_cdf_beta_Qinv
        1.0 - safe_gsl_cdf_beta_qinv(p, nu2 / 2.0, nu1 / 2.0)?
    };

    Ok(nu2 * y / (nu1 * (1.0 - y)))
}

pub fn gsl_cdf_fdist_qinv(q: f64, nu1: f64, nu2: f64) -> Result<f64, GslError> {
    if q < 0.0 {
        return Err(GslError::DomainError);
    }
    if q > 1.0 {
        return Err(GslError::DomainError);
    }
    if nu1 < 1.0 {
        return Err(GslError::DomainError);
    }
    if nu2 < 1.0 {
        return Err(GslError::DomainError);
    }

    let y = if q > 0.5 {
        // Assume safe wrapper exists for gsl_cdf_beta_Qinv
        safe_gsl_cdf_beta_qinv(q, nu1 / 2.0, nu2 / 2.0)?
    } else {
        // Assume safe wrapper exists for gsl_cdf_beta_Pinv
        1.0 - safe_gsl_cdf_beta_pinv(q, nu2 / 2.0, nu1 / 2.0)?
    };

    Ok(nu2 * y / (nu1 * (1.0 - y)))
}

// These would be safe wrappers around the FFI functions
fn safe_gsl_cdf_beta_pinv(p: f64, a: f64, b: f64) -> Result<f64, GslError> {
    // Implementation would call the unsafe FFI function with proper error handling
    unimplemented!()
}

fn safe_gsl_cdf_beta_qinv(q: f64, a: f64, b: f64) -> Result<f64, GslError> {
    // Implementation would call the unsafe FFI function with proper error handling
    unimplemented!()
}