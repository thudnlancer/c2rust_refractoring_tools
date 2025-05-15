use std::fs::File;
use std::io::{self, Read, Write, Error, ErrorKind};
use std::path::Path;
use std::slice;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};

static QUIET_FLAG: AtomicBool = AtomicBool::new(false);

pub fn set_quiet(quiet: bool) {
    QUIET_FLAG.store(quiet, Ordering::Relaxed);
}

pub fn xalloc(size: usize) -> Vec<u8> {
    vec![0; size]
}

pub fn werror(format: &str, args: fmt::Arguments) {
    if !QUIET_FLAG.load(Ordering::Relaxed) {
        eprint!("{}", format);
    }
}

pub fn read_file(name: &str, max_size: Option<usize>) -> io::Result<Vec<u8>> {
    let mut file = File::open(name)?;
    let mut buffer = Vec::new();
    let mut size = 100;

    loop {
        if let Some(max) = max_size {
            if size > max {
                size = max;
            }
        }

        buffer.resize(size, 0);
        let read_bytes = file.read(&mut buffer)?;

        if read_bytes < size {
            buffer.truncate(read_bytes);
            if buffer.is_empty() {
                return Err(Error::new(ErrorKind::InvalidData, "Empty file"));
            }
            break;
        }

        if max_size == Some(size) {
            break;
        }
        size *= 2;
    }

    Ok(buffer)
}

pub fn write_data(f: &mut File, data: &[u8]) -> io::Result<()> {
    f.write_all(data)
}

pub fn write_file(name: &str, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(name)?;
    write_data(&mut file, data)
}

pub fn simple_random(ctx: &mut yarrow256_ctx, name: Option<&str>) -> io::Result<()> {
    const RANDOM_DEVICE: &str = "/dev/urandom";
    let buffer = if let Some(name) = name {
        read_file(name, None)?
    } else {
        read_file(RANDOM_DEVICE, Some(20))?
    };

    yarrow256_seed(ctx, &buffer);
    Ok(())
}

pub fn hash_file(hash: &nettle_hash, ctx: &mut impl HashContext, mut f: File) -> io::Result<()> {
    const BUFSIZE: usize = 1000;
    let mut buffer = [0u8; BUFSIZE];

    loop {
        let bytes_read = f.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hash.update(ctx, &buffer[..bytes_read]);
    }

    Ok(())
}

#[cfg(feature = "hogweed")]
pub fn read_rsa_key(
    name: &str,
    pub_key: &mut rsa_public_key,
    priv_key: &mut rsa_private_key,
) -> io::Result<()> {
    let data = read_file(name, None)?;
    // RSA key parsing implementation would go here
    Ok(())
}