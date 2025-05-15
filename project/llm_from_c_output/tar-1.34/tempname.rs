use std::env;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use rand::Rng;
use libc::{O_RDWR, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, S_IXUSR};

const GT_FILE: i32 = 0;
const GT_DIR: i32 = 1;
const GT_NOCREATE: i32 = 2;

const BASE_62_DIGITS: usize = 10;
const BASE_62_POWER: u64 = 62u64.pow(10);
const ATTEMPTS_MIN: u32 = 62 * 62 * 62;
const TMP_MAX: u32 = 238328;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn random_bits(seed: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn try_file(tmpl: &str, flags: &i32) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode((S_IRUSR | S_IWUSR) as u32)
        .open(tmpl)
}

fn try_dir(tmpl: &str, _flags: &i32) -> io::Result<()> {
    fs::create_dir(tmpl)?;
    fs::set_permissions(tmpl, fs::Permissions::from_mode(S_IRUSR | S_IWUSR | S_IXUSR))?;
    Ok(())
}

fn try_nocreate(tmpl: &str, _flags: &i32) -> io::Result<()> {
    if Path::new(tmpl).exists() {
        Err(io::Error::new(io::ErrorKind::AlreadyExists, "File exists"))
    } else {
        Ok(())
    }
}

fn try_tempname_len(
    tmpl: &mut String,
    suffixlen: usize,
    args: &i32,
    tryfunc: fn(&str, &i32) -> io::Result<File>,
    x_suffix_len: usize,
) -> io::Result<File> {
    let len = tmpl.len();
    if len < x_suffix_len + suffixlen || tmpl[len - x_suffix_len - suffixlen..].chars().filter(|&c| c == 'X').count() < x_suffix_len {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid template"));
    }

    let mut v = random_bits(0);
    let mut vdigits = 0;
    let unfair_min = u64::MAX - u64::MAX % BASE_62_POWER;
    let attempts = std::cmp::max(ATTEMPTS_MIN, TMP_MAX);

    for _ in 0..attempts {
        let xxxxxx_start = len - x_suffix_len - suffixlen;
        let mut xxxxxx = tmpl[xxxxxx_start..xxxxxx_start + x_suffix_len].to_string();

        for i in 0..x_suffix_len {
            if vdigits == 0 {
                loop {
                    v = random_bits(v);
                    if v < unfair_min {
                        break;
                    }
                }
                vdigits = BASE_62_DIGITS;
            }

            let idx = (v % 62) as usize;
            xxxxxx.replace_range(i..i+1, &LETTERS[idx..idx+1]);
            v /= 62;
            vdigits -= 1;
        }

        tmpl.replace_range(xxxxxx_start..xxxxxx_start + x_suffix_len, &xxxxxx);
        
        match tryfunc(tmpl, args) {
            Ok(file) => return Ok(file),
            Err(e) if e.kind() != io::ErrorKind::AlreadyExists => return Err(e),
            _ => continue,
        }
    }

    Err(io::Error::new(io::ErrorKind::AlreadyExists, "Could not create temporary file"))
}

fn gen_tempname_len(
    tmpl: &mut String,
    suffixlen: usize,
    flags: i32,
    kind: i32,
    x_suffix_len: usize,
) -> io::Result<File> {
    let tryfunc = match kind {
        GT_FILE => try_file,
        GT_DIR => |tmpl, flags| {
            try_dir(tmpl, flags)?;
            OpenOptions::new().read(true).write(true).open(tmpl)
        },
        GT_NOCREATE => |tmpl, flags| {
            try_nocreate(tmpl, flags)?;
            OpenOptions::new().read(true).write(true).open(tmpl)
        },
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid kind")),
    };

    try_tempname_len(tmpl, suffixlen, &flags, tryfunc, x_suffix_len)
}

pub fn gen_tempname(tmpl: &mut String, suffixlen: usize, flags: i32, kind: i32) -> io::Result<File> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, 6)
}

pub fn try_tempname(
    tmpl: &mut String,
    suffixlen: usize,
    args: &i32,
    tryfunc: fn(&str, &i32) -> io::Result<File>,
) -> io::Result<File> {
    try_tempname_len(tmpl, suffixlen, args, tryfunc, 6)
}

fn path_search(tmpl: &mut String, dir: Option<&str>, prefix: Option<&str>) -> io::Result<()> {
    let prefix = prefix.unwrap_or("file");
    let prefix = if prefix.len() > 5 { &prefix[..5] } else { prefix };

    let tmpdir = env::temp_dir();
    let dir = match dir {
        Some(d) if Path::new(d).is_dir() => d,
        _ => {
            if let Ok(d) = env::var("TMPDIR") {
                if Path::new(&d).is_dir() {
                    d.as_str()
                } else {
                    tmpdir.to_str().unwrap()
                }
            } else {
                tmpdir.to_str().unwrap()
            }
        }
    };

    let dir = Path::new(dir);
    let dir = dir.to_str().unwrap().trim_end_matches('/');

    if tmpl.capacity() < dir.len() + 1 + prefix.len() + 6 + 1 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Template too short"));
    }

    *tmpl = format!("{}/{}.XXXXXX", dir, prefix);
    Ok(())
}