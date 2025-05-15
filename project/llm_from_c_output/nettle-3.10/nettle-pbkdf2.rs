use std::env;
use std::error::Error;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;
use std::str;

use getopts::Options;
use hex;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;

const DEFAULT_ITERATIONS: u32 = 10000;
const DEFAULT_LENGTH: usize = 16;
const MAX_PASSWORD: usize = 1024;

fn usage(f: &mut dyn Write) {
    writeln!(f, "Usage: nettle-pbkdf2 [OPTIONS] SALT\n\
                Options:\n\
                \t--help                 Show this help.\n\
                \t-V, --version          Show version information.\n\
                \t-i, --iterations=COUNT Desired iteration count (default {}).\n\
                \t-l, --length=LENGTH    Desired output length (octets, default {})\n\
                \t--raw                  Raw binary output.\n\
                \t--hex-salt             Use hex encoding for the salt.",
                DEFAULT_ITERATIONS, DEFAULT_LENGTH).unwrap();
}

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("", "help", "Show this help.");
    opts.optflag("V", "version", "Show version information.");
    opts.optopt("l", "length", "Desired output length", "LENGTH");
    opts.optopt("i", "iterations", "Desired iteration count", "COUNT");
    opts.optflag("", "raw", "Raw binary output.");
    opts.optflag("", "hex-salt", "Use hex encoding for the salt.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            usage(&mut io::stderr());
            die(&e.to_string());
        }
    };

    if matches.opt_present("help") {
        usage(&mut io::stdout());
        return Ok(());
    }

    if matches.opt_present("V") {
        println!("nettle-pbkdf2 ({} {})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let output_length = matches.opt_get_default("l", DEFAULT_LENGTH)
        .map_err(|e| die(&format!("Invalid length argument: {}", e)))?;

    let iterations = matches.opt_get_default("i", DEFAULT_ITERATIONS)
        .map_err(|e| die(&format!("Invalid iteration count: {}", e)))?;

    let raw = matches.opt_present("raw");
    let hex_salt = matches.opt_present("hex-salt");

    if matches.free.len() != 1 {
        usage(&mut io::stderr());
        die("Expected exactly one SALT argument");
    }

    let salt = &matches.free[0];
    let salt_bytes = if hex_salt {
        hex::decode(salt).map_err(|_| die("Invalid salt (expecting hex encoding)"))?
    } else {
        salt.as_bytes().to_vec()
    };

    let mut password = [0u8; MAX_PASSWORD];
    let password_length = io::stdin().read(&mut password)?;
    if password_length == MAX_PASSWORD {
        die(&format!("Password input too long. Current limit is {} characters.", MAX_PASSWORD - 1));
    }

    let mut output = vec![0u8; output_length];
    pbkdf2::<Hmac<Sha256>>(
        &password[..password_length],
        &salt_bytes,
        iterations,
        &mut output
    );

    if raw {
        io::stdout().write_all(&output)?;
    } else {
        for chunk in output.chunks(8) {
            print!("{} ", hex::encode(chunk));
            if chunk.len() == 8 && output.len() > 8 && output.len() % 64 == 0 {
                println!();
            }
        }
        println!();
    }

    io::stdout().flush()?;
    Ok(())
}