/*
 * base64dec -- a decoder for base64
 *
 * Translated from C to Rust with equivalent functionality
 */

use std::io::{self, Read, Write};
use std::process;
use std::error::Error;
use base64::{Engine as _, engine::general_purpose};

const CHUNK_SIZE: usize = 16392;
const DECODED_SIZE: usize = CHUNK_SIZE * 3 / 4; // Base64 expands to 4/3 ratio

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut result = vec![0u8; DECODED_SIZE];
    let mut decoder = base64::read::DecoderReader::new(io::stdin(), &general_purpose::STANDARD);

    loop {
        let nbytes = decoder.read(&mut buffer)?;
        if nbytes == 0 {
            break;
        }

        let decoded_bytes = general_purpose::STANDARD
            .decode_slice(&buffer[..nbytes], &mut result)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        io::stdout().write_all(&result[..decoded_bytes])?;
    }

    io::stdout().flush()?;
    Ok(())
}