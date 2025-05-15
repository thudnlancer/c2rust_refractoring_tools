use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use gsl::wavelet::{Wavelet, WaveletType, TransformDirection};
use gsl::sort;
use gsl::vector::Vector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err("Usage: program <input_file>".into());
    }

    let n = 256;
    let nc = 20;

    // Read data from file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut orig_data = Vec::with_capacity(n);
    let mut data = Vec::with_capacity(n);

    for line in reader.lines().take(n) {
        let val = line?.trim().parse::<f64>()?;
        orig_data.push(val);
        data.push(val);
    }

    // Ensure we have enough data
    if orig_data.len() < n {
        return Err("Not enough data points in input file".into());
    }

    // Create wavelet and workspace
    let wavelet = Wavelet::new(WaveletType::Daubechies, 4)?;
    let mut workspace = wavelet.workspace(n)?;

    // Forward wavelet transform
    wavelet.transform(
        &mut data,
        1,
        n,
        TransformDirection::Forward,
        &mut workspace,
    )?;

    // Compute absolute coefficients and sort indices
    let abscoeff: Vec<f64> = data.iter().map(|x| x.abs()).collect();
    let mut p: Vec<usize> = (0..n).collect();
    sort::sort_index(&mut p, &abscoeff, 1, n)?;

    // Threshold small coefficients
    for i in 0..(n - nc) {
        data[p[i]] = 0.0;
    }

    // Inverse wavelet transform
    wavelet.transform(
        &mut data,
        1,
        n,
        TransformDirection::Backward,
        &mut workspace,
    )?;

    // Print results
    for i in 0..n {
        println!("{} {}", orig_data[i], data[i]);
    }

    Ok(())
}