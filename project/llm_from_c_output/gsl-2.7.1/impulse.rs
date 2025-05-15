use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use statrs::function::erf::erf_inv;
use std::f64::consts::PI;

const N: usize = 1000; // length of time series
const K: usize = 25;   // window size
const T: f64 = 4.0;    // number of scale factors for outlier detection

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate input signal
    let mut rng = StdRng::from_entropy();
    let uniform = Uniform::new(0.0, 1.0);
    
    let mut x = vec![0.0; N];
    let mut y = vec![0.0; N];
    let mut xmedian = vec![0.0; N];
    let mut xsigma = vec![0.0; N];
    let mut ioutlier = vec![0; N];
    
    for i in 0..N {
        let xi = 10.0 * (2.0 * PI * i as f64 / N as f64).sin();
        let ei = rng.sample(rand_distr::Normal::new(0.0, 2.0)?);
        let u = uniform.sample(&mut rng);
        let outlier = if u < 0.01 { 15.0 * ei.signum() } else { 0.0 };
        
        x[i] = xi + ei + outlier;
    }

    // Apply impulse detection filter
    let noutlier = impulse_filter(
        &x, 
        &mut y, 
        &mut xmedian, 
        &mut xsigma, 
        &mut ioutlier, 
        K, 
        T
    )?;

    // Print results
    for i in 0..N {
        println!(
            "{} {} {} {} {} {}",
            i,
            x[i],
            y[i],
            xmedian[i] + T * xsigma[i],
            xmedian[i] - T * xsigma[i],
            ioutlier[i]
        );
    }

    Ok(())
}

fn impulse_filter(
    x: &[f64],
    y: &mut [f64],
    xmedian: &mut [f64],
    xsigma: &mut [f64],
    ioutlier: &mut [i32],
    k: usize,
    t: f64,
) -> Result<usize, Box<dyn std::error::Error>> {
    let n = x.len();
    let mut noutlier = 0;

    for i in 0..n {
        let start = if i >= k / 2 { i - k / 2 } else { 0 };
        let end = if i + k / 2 < n { i + k / 2 } else { n - 1 };
        
        // Calculate median
        let mut window: Vec<f64> = x[start..=end].to_vec();
        window.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = window[window.len() / 2];
        xmedian[i] = median;

        // Calculate Qn scale estimator
        let mut deviations: Vec<f64> = window.iter().map(|&v| (v - median).abs()).collect();
        deviations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let qn = if deviations.len() % 2 == 1 {
            deviations[deviations.len() / 2]
        } else {
            (deviations[deviations.len() / 2 - 1] + deviations[deviations.len() / 2]) / 2.0
        };
        
        xsigma[i] = qn * 1.1926; // Consistency factor for Qn

        // Detect outliers
        let threshold = t * xsigma[i];
        if (x[i] - xmedian[i]).abs() > threshold {
            ioutlier[i] = 1;
            noutlier += 1;
            y[i] = xmedian[i];
        } else {
            ioutlier[i] = 0;
            y[i] = x[i];
        }
    }

    Ok(noutlier)
}