use gsl::{
    bspline::{BSpline, BSplineType},
    fit::multifit_linear,
    rng::{Rng, DefaultRng},
    statistics::wtss,
    vector::Vector,
    matrix::Matrix,
};
use rand_distr::Normal;
use std::error::Error;

const N: usize = 200; // number of data points to fit
const NCOEFFS: usize = 12; // number of fit coefficients
const NBREAK: usize = NCOEFFS - 2; // nbreak = ncoeffs - 2 since k = 4

fn main() -> Result<(), Box<dyn Error>> {
    let n = N;
    let ncoeffs = NCOEFFS;
    let nbreak = NBREAK;

    let mut rng = DefaultRng::default()?;

    // allocate a cubic bspline workspace (k = 4)
    let mut bw = BSpline::new(4, nbreak, BSplineType::Uniform)?;
    let mut B = Vector::new(ncoeffs)?;

    let mut x = Vector::new(n)?;
    let mut y = Vector::new(n)?;
    let mut X = Matrix::new(n, ncoeffs)?;
    let mut c = Vector::new(ncoeffs)?;
    let mut w = Vector::new(n)?;
    let mut cov = Matrix::new(ncoeffs, ncoeffs)?;
    let mw = multifit_linear::Workspace::new(n, ncoeffs)?;

    // this is the data to be fitted
    for i in 0..n {
        let xi = (15.0 / (N as f64 - 1.0)) * i as f64;
        let mut yi = xi.cos() * (-0.1 * xi).exp();

        let sigma = 0.1 * yi;
        let dy = rng.sample(&Normal::new(0.0, sigma)?;
        yi += dy;

        x.set(i, xi)?;
        y.set(i, yi)?;
        w.set(i, 1.0 / (sigma * sigma))?;

        println!("{} {}", xi, yi);
    }

    // use uniform breakpoints on [0, 15]
    bw.knots_uniform(0.0, 15.0)?;

    // construct the fit matrix X
    for i in 0..n {
        let xi = x.get(i)?;

        // compute B_j(xi) for all j
        bw.eval(xi, &mut B)?;

        // fill in row i of X
        for j in 0..ncoeffs {
            let Bj = B.get(j)?;
            X.set(i, j, Bj)?;
        }
    }

    // do the fit
    let (chisq, _) = multifit_linear::wlinear(&X, &w, &y, &mut c, &mut cov, &mw)?;

    let dof = n - ncoeffs;
    let tss = wtss(w.data(), 1, y.data(), 1, y.size())?;
    let rsq = 1.0 - chisq / tss;

    eprintln!("chisq/dof = {:.6e}, Rsq = {:.6}", chisq / dof as f64, rsq);
    println!("\n\n");

    // output the smoothed curve
    let mut xi = 0.0;
    while xi < 15.0 {
        bw.eval(xi, &mut B)?;
        let (yi, yerr) = multifit_linear::est(&B, &c, &cov)?;
        println!("{} {}", xi, yi);
        xi += 0.1;
    }

    Ok(())
}