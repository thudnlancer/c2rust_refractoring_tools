use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::fs;
use std::os::unix::fs::DirBuilderExt;

const TMP_MAX: u32 = 238328;
const BASE_62_DIGITS: usize = 10;
const BASE_62_POWER: u64 = 62u64.pow(10);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempKind {
    File,
    Dir,
    NoCreate,
}

fn mix_random_values(r: u64, s: u64) -> u64 {
    (2862933555777941757u64.wrapping_mul(r).wrapping_add(3037000493)) ^ s
}

fn random_bits(r: &mut u64, s: u64) -> bool {
    let mut buf = [0u8; 8];
    if let Ok(()) = getrandom::getrandom(&mut buf) {
        *r = u64::from_ne_bytes(buf);
        true
    } else {
        let mut v = s;
        
        if let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) {
            v = mix_random_values(v, duration.as_secs());
            v = mix_random_values(v, duration.subsec_nanos() as u64);
        }
        
        *r = mix_random_values(v, SystemTime::now().elapsed().unwrap().as_nanos() as u64);
        false
    }
}

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn try_tempname_len(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    args: &mut OpenOptions,
    kind: TempKind,
    x_suffix_len: usize,
) -> io::Result<File> {
    let attempts = TMP_MAX.max(62 * 62 * 62);
    let mut v = 0u64;
    let mut vdigbuf;
    let mut vdigits = 0;

    let biased_min = u64::MAX - u64::MAX % BASE_62_POWER;

    let tmpl_str = tmpl.to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Invalid template",
    ))?;

    if tmpl_str.len() < x_suffix_len + suffixlen
        || tmpl_str
            .chars()
            .rev()
            .take(x_suffix_len + suffixlen)
            .filter(|&c| c == 'X')
            .count()
            < x_suffix_len
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Template must end with at least X_SUFFIX_LEN 'X's",
        ));
    }

    let x_pos = tmpl_str.len() - x_suffix_len - suffixlen;
    let mut rng = rand::thread_rng();

    for _ in 0..attempts {
        let mut new_tmpl = tmpl_str.to_string();
        let x_part: String = (0..x_suffix_len)
            .map(|_| {
                if vdigits == 0 {
                    while random_bits(&mut v, v) && biased_min <= v {}
                    vdigbuf = v;
                    vdigits = BASE_62_DIGITS;
                }
                let idx = (vdigbuf % 62) as usize;
                vdigbuf /= 62;
                vdigits -= 1;
                LETTERS[idx] as char
            })
            .collect();

        new_tmpl.replace_range(x_pos..x_pos + x_suffix_len, &x_part);
        let path = Path::new(&new_tmpl);

        match kind {
            TempKind::File => {
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(path)
                {
                    Ok(file) => return Ok(file),
                    Err(e) if e.kind() != io::ErrorKind::AlreadyExists => return Err(e),
                    _ => continue,
                }
            }
            TempKind::Dir => {
                match fs::DirBuilder::new()
                    .mode(0o700)
                    .create(path)
                {
                    Ok(_) => return Ok(File::open(path)?),
                    Err(e) if e.kind() != io::ErrorKind::AlreadyExists => return Err(e),
                    _ => continue,
                }
            }
            TempKind::NoCreate => {
                if !path.exists() {
                    return Ok(File::open(path)?);
                } else {
                    continue;
                }
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "Failed to create temporary file after maximum attempts",
    ))
}

pub fn gen_tempname(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    flags: &mut OpenOptions,
    kind: TempKind,
) -> io::Result<File> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, 6)
}

pub fn gen_tempname_len(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    flags: &mut OpenOptions,
    kind: TempKind,
    x_suffix_len: usize,
) -> io::Result<File> {
    try_tempname_len(tmpl, suffixlen, flags, kind, x_suffix_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    fn test_temp_file() {
        let mut tmpl = env::temp_dir();
        tmpl.push("testXXXXXX");
        let mut opts = OpenOptions::new();
        let _file = gen_tempname(&mut tmpl, 0, &mut opts, TempKind::File).unwrap();
        assert!(tmpl.exists());
        fs::remove_file(&tmpl).unwrap();
    }

    #[test]
    fn test_temp_dir() {
        let mut tmpl = env::temp_dir();
        tmpl.push("testXXXXXX");
        let mut opts = OpenOptions::new();
        let _file = gen_tempname(&mut tmpl, 0, &mut opts, TempKind::Dir).unwrap();
        assert!(tmpl.exists());
        fs::remove_dir(&tmpl).unwrap();
    }
}