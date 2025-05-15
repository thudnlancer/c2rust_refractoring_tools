use std::env;
use std::ffi::{CString, OsStr};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use libc::{O_CREAT, O_EXCL, O_RDWR, S_IRUSR, S_IWUSR, S_IXUSR};

const GT_FILE: i32 = 0;
const GT_DIR: i32 = 1;
const GT_NOCREATE: i32 = 2;

const TMP_MAX: u32 = 238328;
const BASE_62_DIGITS: usize = 10;
const BASE_62_POWER: u64 = 62u64.pow(10);
const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Debug)]
enum TempError {
    InvalidTemplate,
    Exists,
    Io(io::Error),
}

impl From<io::Error> for TempError {
    fn from(err: io::Error) -> Self {
        TempError::Io(err)
    }
}

fn random_bits(v: u64, use_getrandom: bool) -> u64 {
    if use_getrandom {
        let mut buf = [0u8; 8];
        if let Ok(n) = getrandom::getrandom(&mut buf) {
            if n == 8 {
                return u64::from_ne_bytes(buf);
            }
        }
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    2862933555777941757u64.wrapping_mul(v ^ now).wrapping_add(3037000493)
}

fn try_file(tmpl: &Path, flags: i32) -> Result<File, TempError> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(flags | O_CREAT | O_EXCL)
        .mode((S_IRUSR | S_IWUSR) as u32)
        .open(tmpl)
        .map_err(TempError::Io)
}

fn try_dir(tmpl: &Path, _flags: i32) -> Result<(), TempError> {
    fs::DirBuilder::new()
        .mode((S_IRUSR | S_IWUSR | S_IXUSR) as u32)
        .create(tmpl)
        .map_err(TempError::Io)
}

fn try_nocreate(tmpl: &Path, _flags: i32) -> Result<(), TempError> {
    if tmpl.exists() {
        Err(TempError::Exists)
    } else {
        Ok(())
    }
}

fn try_tempname_len(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    flags: i32,
    kind: i32,
    x_suffix_len: usize,
) -> Result<Option<File>, TempError> {
    let tryfunc = match kind {
        GT_FILE => try_file,
        GT_DIR => |path, flags| try_dir(path, flags).map(|_| None),
        GT_NOCREATE => |path, flags| try_nocreate(path, flags).map(|_| None),
        _ => return Err(TempError::InvalidTemplate),
    };

    let tmpl_str = tmpl.to_str().ok_or(TempError::InvalidTemplate)?;
    if !tmpl_str.ends_with("XXXXXX") {
        return Err(TempError::InvalidTemplate);
    }

    let x_pos = tmpl_str.len() - x_suffix_len - suffixlen;
    let x_part = &tmpl_str[x_pos..x_pos + x_suffix_len];
    if x_part.chars().any(|c| c != 'X') {
        return Err(TempError::InvalidTemplate);
    }

    let mut rng = rand::thread_rng();
    let mut v = rng.gen::<u64>();
    let mut vdigits = 0;
    let mut use_getrandom = kind == GT_NOCREATE;
    let unfair_min = u64::MAX - u64::MAX % BASE_62_POWER;

    for _ in 0..TMP_MAX {
        let mut x_chars = Vec::with_capacity(x_suffix_len);
        for _ in 0..x_suffix_len {
            if vdigits == 0 {
                loop {
                    v = random_bits(v, use_getrandom);
                    use_getrandom = true;
                    if v < unfair_min {
                        break;
                    }
                }
                vdigits = BASE_62_DIGITS;
            }
            x_chars.push(LETTERS[(v % 62) as usize] as char);
            v /= 62;
            vdigits -= 1;
        }

        let new_tmpl = format!(
            "{}{}{}",
            &tmpl_str[..x_pos],
            x_chars.iter().collect::<String>(),
            &tmpl_str[x_pos + x_suffix_len..]
        );

        let path = Path::new(&new_tmpl);
        match tryfunc(path, flags) {
            Ok(file) => return Ok(file),
            Err(TempError::Exists) => continue,
            Err(e) => return Err(e),
        }
    }

    Err(TempError::Exists)
}

pub fn gen_tempname(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    flags: i32,
    kind: i32,
) -> Result<Option<File>, TempError> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, 6)
}

pub fn gen_tempname_len(
    tmpl: &mut PathBuf,
    suffixlen: usize,
    flags: i32,
    kind: i32,
    x_suffix_len: usize,
) -> Result<Option<File>, TempError> {
    try_tempname_len(tmpl, suffixlen, flags, kind, x_suffix_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_temp_file() {
        let dir = tempdir().unwrap();
        let mut path = dir.path().join("tempXXXXXX");
        let file = gen_tempname(&mut path, 0, 0, GT_FILE).unwrap().unwrap();
        assert!(file.metadata().unwrap().is_file());
    }

    #[test]
    fn test_temp_dir() {
        let dir = tempdir().unwrap();
        let mut path = dir.path().join("tempXXXXXX");
        gen_tempname(&mut path, 0, 0, GT_DIR).unwrap();
        assert!(path.is_dir());
    }
}