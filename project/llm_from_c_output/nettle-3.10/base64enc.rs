use std::io::{self, Read, Write};
use std::process;

const CHUNK_SIZE: usize = 54;
const ENCODED_SIZE: usize = BASE64_ENCODE_LENGTH!(CHUNK_SIZE);

macro_rules! BASE64_ENCODE_LENGTH {
    ($n:expr) => {
        (($n + 2) / 3) * 4
    };
}

struct Base64EncodeContext {
    // Context fields would go here
}

impl Base64EncodeContext {
    fn new() -> Self {
        Base64EncodeContext {
            // Initialize context fields
        }
    }

    fn update(&mut self, output: &mut [u8], input: &[u8]) -> usize {
        // Base64 encode update implementation
        0 // Return encoded bytes count
    }

    fn finalize(&mut self, output: &mut [u8]) -> usize {
        // Base64 encode final implementation
        0 // Return final encoded bytes count
    }
}

fn write_data(mut writer: impl Write, data: &[u8]) -> io::Result<()> {
    writer.write_all(data)
}

fn main() -> io::Result<()> {
    let mut b64_ctx = Base64EncodeContext::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut buffer = [0u8; CHUNK_SIZE];
    let mut result = vec![0u8; ENCODED_SIZE + BASE64_ENCODE_LENGTH!(0) + 1];

    loop {
        let nbytes = stdin.read(&mut buffer)?;

        let encoded_bytes = b64_ctx.update(&mut result, &buffer[..nbytes]);

        if nbytes < CHUNK_SIZE {
            if nbytes == 0 {
                break;
            }

            let final_bytes = b64_ctx.finalize(&mut result[encoded_bytes..]);
            let total_bytes = encoded_bytes + final_bytes;
            
            result[total_bytes] = b'\n';
            write_data(&mut stdout, &result[..=total_bytes])?;
            stdout.flush()?;
            process::exit(0);
        }

        result[encoded_bytes] = b'\n';
        write_data(&mut stdout, &result[..=encoded_bytes])?;
    }

    Ok(())
}

fn werror(msg: &str) {
    eprintln!("{}", msg);
}