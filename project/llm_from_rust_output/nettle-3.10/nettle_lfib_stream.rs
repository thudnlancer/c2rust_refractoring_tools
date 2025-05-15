use std::env;
use std::io::{self, Write};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use std::num::ParseIntError;

#[derive(Clone, Copy)]
struct KnuthLfibCtx {
    x: [u32; 100],
    index: u32,
}

impl KnuthLfibCtx {
    fn new() -> Self {
        KnuthLfibCtx {
            x: [0; 100],
            index: 0,
        }
    }

    fn init(&mut self, seed: u32) {
        // This would normally call nettle_knuth_lfib_init
        // For this example, we'll simulate initialization
        self.x[0] = seed;
        self.index = 0;
    }

    fn random(&mut self, n: usize, dst: &mut [u8]) {
        // This would normally call nettle_knuth_lfib_random
        // For this example, we'll simulate random data generation
        for i in 0..n {
            dst[i] = (self.x[0] as u8).wrapping_add(i as u8);
        }
        self.x[0] = self.x[0].wrapping_add(1);
    }
}

fn usage() {
    eprintln!("Usage: lfib-stream [SEED]");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ctx = KnuthLfibCtx::new();
    let seed = match args.len() {
        1 => {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32
        }
        2 => {
            match args[1].parse::<u32>() {
                Ok(seed) if seed != 0 => seed,
                _ => {
                    usage();
                    process::exit(1);
                }
            }
        }
        _ => {
            usage();
            process::exit(1);
        }
    };

    ctx.init(seed);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut buffer = [0u8; 500];

    loop {
        ctx.random(buffer.len(), &mut buffer);
        handle.write_all(&buffer)?;
        handle.flush()?;
    }
}