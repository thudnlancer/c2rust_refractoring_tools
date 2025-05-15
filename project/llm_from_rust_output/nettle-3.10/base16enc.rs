use std::io::{self, Read, Write};
use std::ffi::CString;
use std::process;

fn main() {
    let mut buffer = [0u8; 36];
    let mut stdout = io::stdout();
    
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(nbytes) => {
                let encoded = base16_encode(&buffer[..nbytes]);
                if let Err(e) = writeln!(stdout, "{}", encoded) {
                    eprintln!("Error writing to stdout: {}", e);
                    process::exit(1);
                }
                
                if nbytes < buffer.len() {
                    if let Err(e) = stdout.flush() {
                        eprintln!("Error flushing stdout: {}", e);
                        process::exit(1);
                    }
                    process::exit(0);
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                process::exit(1);
            }
        }
    }
}

fn base16_encode(input: &[u8]) -> String {
    const HEX_CHARS: &[u8] = b"0123456789ABCDEF";
    let mut output = Vec::with_capacity(input.len() * 2);
    
    for &byte in input {
        output.push(HEX_CHARS[(byte >> 4) as usize]);
        output.push(HEX_CHARS[(byte & 0xF) as usize]);
    }
    
    unsafe { String::from_utf8_unchecked(output) }
}