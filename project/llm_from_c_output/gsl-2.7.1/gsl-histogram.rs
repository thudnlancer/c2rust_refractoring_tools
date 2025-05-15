use std::env;
use std::io::{self, BufRead};
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 && args.len() != 4 {
        println!("Usage: gsl-histogram xmin xmax [n]");
        println!(
            "Computes a histogram of the data on stdin using n bins from xmin to xmax.\n\
             If n is unspecified then bins of integer width are used."
        );
        process::exit(0);
    }

    let a = match f64::from_str(&args[1]) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing xmin: {}", e);
            process::exit(1);
        }
    };

    let b = match f64::from_str(&args[2]) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error parsing xmax: {}", e);
            process::exit(1);
        }
    };

    let n = if args.len() == 4 {
        match usize::from_str(&args[3]) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Error parsing n: {}", e);
                process::exit(1);
            }
        }
    } else {
        (b - a) as usize
    };

    let mut histogram = Histogram::new(n, a, b).unwrap_or_else(|e| {
        eprintln!("Error creating histogram: {}", e);
        process::exit(1);
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap_or_else(|e| {
            eprintln!("Error reading line: {}", e);
            process::exit(1);
        });

        if line.trim().is_empty() {
            continue;
        }

        match f64::from_str(&line) {
            Ok(x) => histogram.increment(x).unwrap_or_else(|e| {
                eprintln!("Error incrementing histogram: {}", e);
                process::exit(1);
            }),
            Err(e) => {
                eprintln!("Error parsing input value: {}", e);
                process::exit(1);
            }
        }
    }

    histogram.print();
}

struct Histogram {
    bins: Vec<usize>,
    ranges: Vec<f64>,
}

impl Histogram {
    fn new(n: usize, a: f64, b: f64) -> Result<Self, String> {
        if n == 0 {
            return Err("Number of bins must be greater than 0".to_string());
        }
        if a >= b {
            return Err("xmin must be less than xmax".to_string());
        }

        let mut ranges = Vec::with_capacity(n + 1);
        let bin_width = (b - a) / n as f64;
        for i in 0..=n {
            ranges.push(a + i as f64 * bin_width);
        }

        Ok(Histogram {
            bins: vec![0; n],
            ranges,
        })
    }

    fn increment(&mut self, x: f64) -> Result<(), String> {
        for i in 0..self.bins.len() {
            if x >= self.ranges[i] && x < self.ranges[i + 1] {
                self.bins[i] += 1;
                return Ok(());
            }
        }
        Err("Value out of range".to_string())
    }

    fn print(&self) {
        for i in 0..self.bins.len() {
            println!("{} {}", self.ranges[i], self.bins[i]);
        }
        println!("{}", self.ranges[self.bins.len()]);
    }
}