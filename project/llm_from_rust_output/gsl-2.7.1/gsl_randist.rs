use gsl::randist::{Bernoulli, Beta, Binomial, Cauchy, ChiSquared, Erlang, Exponential, 
    ExpPow, FDist, Flat, Gamma, Gaussian, GaussianTail, UGaussian, UGaussianTail, 
    BivariateGaussian, Dir2d, Dir3d, DirNd, Geometric, Gumbel1, Gumbel2, 
    Hypergeometric, Laplace, Landau, Levy, LevySkew, Logarithmic, Logistic, 
    Lognormal, NegativeBinomial, Pareto, Pascal, Poisson, Rayleigh, RayleighTail, 
    TDist, Weibull};
use gsl::Rng;
use std::env;
use std::ffi::CString;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: gsl-randist seed n DIST param1 param2 ...\nGenerates n samples from the distribution DIST with parameters param1,\nparam2, etc. Valid distributions are,\n");
        println!("  beta\n  binomial\n  bivariate-gaussian\n  cauchy\n  chisq\n  dir-2d\n  dir-3d\n  dir-nd\n  erlang\n  exponential\n  exppow\n  fdist\n  flat\n  gamma\n  gaussian-tail\n  gaussian\n  geometric\n  gumbel1\n  gumbel2\n  hypergeometric\n  laplace\n  landau\n  levy\n  levy-skew\n  logarithmic\n  logistic\n  lognormal\n  negative-binomial\n  pareto\n  pascal\n  poisson\n  rayleigh-tail\n  rayleigh\n  tdist\n  ugaussian-tail\n  ugaussian\n  weibull");
        exit(0);
    }

    let seed = args[1].parse::<u64>().unwrap();
    let n = args[2].parse::<usize>().unwrap();
    let dist_name = &args[3];
    let params = &args[4..];

    let rng = Rng::new(seed);

    match dist_name.as_str() {
        "bernoulli" => {
            if params.len() != 1 { error("p = probability of success"); }
            let p = params[0].parse::<f64>().unwrap();
            for _ in 0..n {
                println!("{}", Bernoulli::sample(&rng, p));
            }
        }
        "beta" => {
            if params.len() != 2 { error("a,b = shape parameters"); }
            let a = params[0].parse::<f64>().unwrap();
            let b = params[1].parse::<f64>().unwrap();
            for _ in 0..n {
                println!("{}", Beta::sample(&rng, a, b));
            }
        }
        // ... other distributions similarly ...
        _ => {
            eprintln!("Error: unrecognized distribution: {}", dist_name);
            exit(1);
        }
    }
}

fn error(msg: &str) {
    eprintln!("Error: arguments should be {}", msg);
    exit(1);
}