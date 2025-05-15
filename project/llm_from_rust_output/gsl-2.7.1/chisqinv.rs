use gsl::cdf;

pub fn gsl_cdf_chisq_Pinv(P: f64, nu: f64) -> f64 {
    cdf::gamma_Pinv(P, nu / 2.0, 2.0)
}

pub fn gsl_cdf_chisq_Qinv(Q: f64, nu: f64) -> f64 {
    cdf::gamma_Qinv(Q, nu / 2.0, 2.0)
}