use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use sha2::{Sha256, Sha224, Digest};

const BUFFER_SIZE: usize = 32768;

pub fn sha256_stream<P: AsRef<Path>>(path: P, resblock: &mut [u8; 32]) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    resblock.copy_from_slice(&hasher.finalize());
    Ok(())
}

pub fn sha224_stream<P: AsRef<Path>>(path: P, resblock: &mut [u8; 28]) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut hasher = Sha224::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    resblock.copy_from_slice(&hasher.finalize());
    Ok(())
}