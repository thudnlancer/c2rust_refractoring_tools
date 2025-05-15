use std::env;
use std::ffi::CString;
use std::io::{self, Read};
use std::process;
use std::str::FromStr;

struct Histogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl Histogram {
    fn new(n: usize) -> Self {
        Histogram {
            n,
            range: vec![0.0; n + 1],
            bin: vec![0.0; n],
        }
    }

    fn set_ranges_uniform(&mut self, xmin: f64, xmax: f64) {
        let step = (xmax - xmin) / self.n as f64;
        for i in 0..=self.n {
            self.range[i] = xmin + i as f64 * step;
        }
    }

    fn increment(&mut self, x: f64) {
        for i in 0..self.n {
            if x >= self.range[i] && x < self.range[i + 1] {
                self.bin[i] += 1.0;
                break;
            }
        }
    }

    fn print(&self) {
        for i in 0..self.n {
            println!("{} {}", self.range[i], self.bin[i]);
        }
        println!("{} {}", self.range[self.n], 0.0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage: gsl-histogram xmin xmax [n]");
        eprintln!("Computes a histogram of the data on stdin using n bins from xmin to xmax.");
        eprintln!("If n is unspecified then bins of integer width are used.");
        process::exit(0);
    }

    let a = f64::from_str(&args[1]).unwrap_or_else(|_| {
        eprintln!("Invalid xmin value");
        process::exit(1);
    });

    let b = f64::from_str(&args[2]).unwrap_or_else(|_| {
        eprintln!("Invalid xmax value");
        process::exit(1);
    });

    let n = if args.len() == 4 {
        usize::from_str(&args[3]).unwrap_or_else(|_| {
            eprintln!("Invalid n value");
            process::exit(1);
        })
    } else {
        (b - a).floor() as usize
    };

    let mut h = Histogram::new(n);
    h.set_ranges_uniform(a, b);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        if let Ok(x) = f64::from_str(line.trim()) {
            h.increment(x);
        }
    }

    h.print();
}