use std::env;
use std::ffi::{CString, CStr};
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use std::process;
use std::str;
use std::ptr;
use std::mem;

use libc::{c_int, c_uint, c_char, c_uchar, c_long, c_ulong, c_void};
use nettle_sys::{
    nettle_pbkdf2_hmac_sha256,
    nettle_base16_encode_update,
    nettle_base16_decode_init,
    nettle_base16_decode_update,
    nettle_base16_decode_final,
    base16_decode_ctx,
};

const DEFAULT_ITERATIONS: c_uint = 10000;
const DEFAULT_OUTPUT_LENGTH: c_uint = 16;
const PASSWORD_BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
enum Error {
    InvalidArgument(String),
    Io(io::Error),
    ParseInt(ParseIntError),
    NettleError(String),
    MemoryAllocation,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}

struct Options {
    iterations: c_uint,
    output_length: c_uint,
    raw_output: bool,
    hex_salt: bool,
    salt: String,
}

fn parse_args() -> Result<Options, Error> {
    let mut args = env::args().skip(1);
    let mut iterations = DEFAULT_ITERATIONS;
    let mut output_length = DEFAULT_OUTPUT_LENGTH;
    let mut raw_output = false;
    let mut hex_salt = false;
    let mut salt = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => {
                print_usage();
                process::exit(0);
            }
            "--version" | "-V" => {
                println!("nettle-pbkdf2 (nettle 3.10)");
                process::exit(0);
            }
            "--length" | "-l" => {
                let len = args.next().ok_or_else(|| Error::InvalidArgument("Missing length argument".to_string()))?;
                output_length = len.parse::<c_uint>()?;
                if output_length == 0 {
                    return Err(Error::InvalidArgument("Length must be positive".to_string()));
                }
            }
            "--iterations" | "-i" => {
                let iter = args.next().ok_or_else(|| Error::InvalidArgument("Missing iterations argument".to_string()))?;
                iterations = iter.parse::<c_uint>()?;
                if iterations == 0 {
                    return Err(Error::InvalidArgument("Iterations must be positive".to_string()));
                }
            }
            "--raw" => raw_output = true,
            "--hex-salt" => hex_salt = true,
            _ => {
                if salt.is_none() {
                    salt = Some(arg);
                } else {
                    return Err(Error::InvalidArgument("Unexpected argument".to_string()));
                }
            }
        }
    }

    let salt = salt.ok_or_else(|| Error::InvalidArgument("Missing salt argument".to_string()))?;

    Ok(Options {
        iterations,
        output_length,
        raw_output,
        hex_salt,
        salt,
    })
}

fn print_usage() {
    println!("Usage: nettle-pbkdf2 [OPTIONS] SALT");
    println!("Options:");
    println!("  --help                 Show this help.");
    println!("  -V, --version          Show version information.");
    println!("  -i, --iterations=COUNT Desired iteration count (default {}).", DEFAULT_ITERATIONS);
    println!("  -l, --length=LENGTH    Desired output length (octets, default {})", DEFAULT_OUTPUT_LENGTH);
    println!("  --raw                  Raw binary output.");
    println!("  --hex-salt             Use hex encoding for the salt.");
}

fn read_password() -> Result<Vec<u8>, Error> {
    let mut password = vec![0u8; PASSWORD_BUFFER_SIZE];
    let bytes_read = io::stdin().read(&mut password)?;
    
    if bytes_read == PASSWORD_BUFFER_SIZE {
        return Err(Error::InvalidArgument(format!(
            "Password input too long. Current limit is {} characters.",
            PASSWORD_BUFFER_SIZE - 1
        )));
    }

    password.truncate(bytes_read);
    Ok(password)
}

fn decode_hex_salt(salt: &str) -> Result<Vec<u8>, Error> {
    let mut ctx = base16_decode_ctx { word: 0, bits: 0 };
    let mut decoded = vec![0u8; salt.len() / 2];
    let mut decoded_len = 0;

    unsafe {
        nettle_base16_decode_init(&mut ctx);
        if nettle_base16_decode_update(
            &mut ctx,
            &mut decoded_len,
            decoded.as_mut_ptr(),
            salt.len(),
            salt.as_ptr() as *const c_char,
        ) == 0 || nettle_base16_decode_final(&mut ctx) == 0
        {
            return Err(Error::NettleError("Invalid hex salt".to_string()));
        }
    }

    decoded.truncate(decoded_len);
    Ok(decoded)
}

fn main() -> Result<(), Error> {
    let options = parse_args()?;
    let password = read_password()?;
    let salt = if options.hex_salt {
        decode_hex_salt(&options.salt)?
    } else {
        options.salt.as_bytes().to_vec()
    };

    let mut output = vec![0u8; options.output_length as usize];
    
    unsafe {
        nettle_pbkdf2_hmac_sha256(
            password.len(),
            password.as_ptr(),
            options.iterations,
            salt.len(),
            salt.as_ptr(),
            options.output_length as usize,
            output.as_mut_ptr(),
        );
    }

    if options.raw_output {
        io::stdout().write_all(&output)?;
    } else {
        let mut hex_output = vec![0u8; options.output_length as usize * 2 + 1];
        unsafe {
            nettle_base16_encode_update(
                hex_output.as_mut_ptr() as *mut c_char,
                options.output_length as usize,
                output.as_ptr(),
            );
        }
        let hex_str = unsafe { CStr::from_ptr(hex_output.as_ptr() as *const c_char) };
        println!("{}", hex_str.to_str().unwrap());
    }

    Ok(())
}