use std::env;
use std::process;
use std::str::FromStr;
use std::error::Error;
use rand::distributions::{Distribution, Bernoulli, Beta, Binomial, Cauchy, ChiSquared, Exponential, Gamma, Normal, Pareto, Poisson, Uniform, Weibull};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn error(s: &str) -> ! {
    eprintln!("Error: arguments should be {}", s);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 4 {
        println!(
"Usage: gsl-randist seed n DIST param1 param2 ...
Generates n samples from the distribution DIST with parameters param1,
param2, etc. Valid distributions are,

  beta
  binomial
  bivariate-gaussian
  cauchy
  chisq
  dir-2d
  dir-3d
  dir-nd
  erlang
  exponential
  exppow
  fdist
  flat
  gamma
  gaussian-tail
  gaussian
  geometric
  gumbel1
  gumbel2
  hypergeometric
  laplace
  landau
  levy
  levy-skew
  logarithmic
  logistic
  lognormal
  negative-binomial
  pareto
  pascal
  poisson
  rayleigh-tail
  rayleigh
  tdist
  ugaussian-tail
  ugaussian
  weibull");
        process::exit(0);
    }

    let mut arg_iter = args.iter().skip(1);
    let seed = arg_iter.next().unwrap().parse::<u64>().unwrap_or_else(|_| error("seed must be a number"));
    let n = arg_iter.next().unwrap().parse::<usize>().unwrap_or_else(|_| error("n must be a positive integer"));
    let name = arg_iter.next().unwrap();
    let mut params: Vec<f64> = vec![];

    for param in arg_iter {
        params.push(param.parse::<f64>().unwrap_or_else(|_| error("parameters must be numbers")));
    }

    let mut rng = StdRng::seed_from_u64(seed);

    match name.as_str() {
        "bernoulli" => {
            if params.len() != 1 { error("p = probability of success"); }
            let p = params[0];
            let dist = Bernoulli::new(p).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng) as u8);
            }
        }
        "beta" => {
            if params.len() != 2 { error("a,b = shape parameters"); }
            let a = params[0];
            let b = params[1];
            let dist = Beta::new(a, b).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "binomial" => {
            if params.len() != 2 { error("p = probability, N = number of trials"); }
            let p = params[0];
            let n_trials = params[1] as u64;
            let dist = Binomial::new(n_trials, p).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "cauchy" => {
            if params.len() != 1 { error("a = scale parameter"); }
            let a = params[0];
            let dist = Cauchy::new(0.0, a).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "chisq" => {
            if params.len() != 1 { error("nu = degrees of freedom"); }
            let nu = params[0];
            let dist = ChiSquared::new(nu).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "exponential" => {
            if params.len() != 1 { error("mu = mean value"); }
            let mu = params[0];
            let dist = Exponential::new(mu).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "gamma" => {
            if params.len() != 2 { error("a = order, b = scale"); }
            let a = params[0];
            let b = params[1];
            let dist = Gamma::new(a, b).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "gaussian" => {
            if params.len() != 1 { error("sigma = standard deviation"); }
            let sigma = params[0];
            let dist = Normal::new(0.0, sigma).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "ugaussian" => {
            if !params.is_empty() { error("unit gaussian, no parameters required"); }
            let dist = Normal::new(0.0, 1.0).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "poisson" => {
            if params.len() != 1 { error("mu = scale parameter"); }
            let mu = params[0];
            let dist = Poisson::new(mu).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "pareto" => {
            if params.len() != 2 { error("a = power, b = scale parameter"); }
            let a = params[0];
            let b = params[1];
            let dist = Pareto::new(b, a).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "weibull" => {
            if params.len() != 2 { error("a = scale parameter, b = exponent"); }
            let a = params[0];
            let b = params[1];
            let dist = Weibull::new(a, b).unwrap();
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        "flat" => {
            if params.len() != 2 { error("a = lower limit, b = upper limit"); }
            let a = params[0];
            let b = params[1];
            let dist = Uniform::new(a, b);
            for _ in 0..n {
                println!("{}", dist.sample(&mut rng));
            }
        }
        _ => {
            eprintln!("Error: unrecognized distribution: {}", name);
            process::exit(1);
        }
    }
}