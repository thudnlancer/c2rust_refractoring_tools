use std::fs::File;
use std::io::{self, Read};
use sha1::{Sha1, Digest};

pub fn sha1_stream(mut stream: File, resblock: &mut [u8; 20]) -> io::Result<()> {
    let mut buffer = vec![0u8; 32768 + 72];
    let mut hasher = Sha1::new();

    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    resblock.copy_from_slice(&result);
    Ok(())
}