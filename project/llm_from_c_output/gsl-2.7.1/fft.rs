use std::f64::consts::SQRT_2;
use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    // Initialize complex data array with 128 elements
    let mut data: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); 128];

    // Set first element to 1.0 + 0.0i
    data[0] = Complex::new(1.0, 0.0);

    // Set elements 1-10 and their mirror positions to 1.0 + 0.0i
    for i in 1..=10 {
        data[i] = Complex::new(1.0, 0.0);
        data[128 - i] = Complex::new(1.0, 0.0);
    }

    // Print input data
    for (i, item) in data.iter().enumerate() {
        println!("{} {:e} {:e}", i, item.re, item.im);
    }
    println!("\n\n");

    // Perform FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(data.len());
    fft.process(&mut data);

    // Normalize and print output
    let normalization_factor = (data.len() as f64).sqrt();
    for (i, item) in data.iter().enumerate() {
        println!(
            "{} {:e} {:e}",
            i,
            item.re / normalization_factor,
            item.im / normalization_factor
        );
    }
}