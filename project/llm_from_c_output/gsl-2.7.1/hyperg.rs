use rand::Rng;
use statrs::function::gamma::ln_gamma;

/// The hypergeometric distribution has the form,
///
/// prob(k) =  choose(n1,t) choose(n2, t-k) / choose(n1+n2,t)
///
/// where choose(a,b) = a!/(b!(a-b)!)) 
///
/// n1 + n2 is the total population (tagged plus untagged)
/// n1 is the tagged population
/// t is the number of samples taken (without replacement)
/// k is the number of tagged samples found
pub fn hypergeometric(rng: &mut impl Rng, n1: u32, n2: u32, t: u32) -> u32 {
    let n = n1 + n2;
    let t = if t > n { n } else { t };

    let mut k = 0;
    let mut a = n1;
    let mut b = n;

    if t < n / 2 {
        for _ in 0..t {
            let u = rng.gen::<f64>();
            if (b as f64) * u < a as f64 {
                k += 1;
                if k == n1 {
                    return k;
                }
                a -= 1;
            }
            b -= 1;
        }
        k
    } else {
        for _ in 0..(n - t) {
            let u = rng.gen::<f64>();
            if (b as f64) * u < a as f64 {
                k += 1;
                if k == n1 {
                    return n1 - k;
                }
                a -= 1;
            }
            b -= 1;
        }
        n1 - k
    }
}

pub fn hypergeometric_pdf(k: u32, n1: u32, n2: u32, t: u32) -> f64 {
    let n = n1 + n2;
    let t = if t > n { n } else { t };

    if k > n1 || k > t {
        0.0
    } else if t > n2 && k + n2 < t {
        0.0
    } else {
        let c1 = ln_choose(n1 as f64, k as f64);
        let c2 = ln_choose(n2 as f64, (t - k) as f64);
        let c3 = ln_choose(n as f64, t as f64);
        (c1 + c2 - c3).exp()
    }
}

fn ln_choose(n: f64, k: f64) -> f64 {
    if n == 0.0 || k == 0.0 || n == k {
        0.0
    } else {
        ln_gamma(n + 1.0) - ln_gamma(k + 1.0) - ln_gamma(n - k + 1.0)
    }
}