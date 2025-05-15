use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use libc::{O_RDWR, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, S_IXUSR};

const GT_FILE: i32 = 0;
const GT_DIR: i32 = 1;
const GT_NOCREATE: i32 = 2;
const TMP_MAX: u32 = 238328;
const ATTEMPTS_MIN: u32 = 62 * 62 * 62;
const BASE_62_DIGITS: usize = 10;
const BASE_62_POWER: u64 = 62u64.pow(10);

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

type RandomValue = u64;

fn mix_random_values(r: RandomValue, s: RandomValue) -> RandomValue {
    (2862933555777941757u64 * r + 3037000493) ^ s
}

fn random_bits(r: &mut RandomValue, s: RandomValue) -> bool {
    let mut buf = [0u8; 8];
    if let Ok(n) = getrandom::getrandom(&mut buf) {
        if n == 8 {
            *r = RandomValue::from_le_bytes(buf);
            return true;
        }
    }

    let mut v = s;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    v = mix_random_values(v, now.as_secs() as RandomValue);
    v = mix_random_values(v, now.subsec_nanos() as RandomValue);
    *r = mix_random_values(v, now.as_secs() as RandomValue);
    false
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
    std::fs::create_dir(tmpl)?;
    std::fs::set_permissions(tmpl, std::fs::Permissions::from_mode(S_IRUSR | S_IWUSR | S_IXUSR))?;
    Ok(())
}

fn try_nocreate(tmpl: &str, _flags: &i32) -> io::Result<()> {
    if Path::new(tmpl).exists() {
        Err(io::Error::from_raw_os_error(libc::EEXIST))
    } else {
        Ok(())
    }
}

pub fn gen_tempname_len(
    tmpl: &mut String,
    suffixlen: usize,
    flags: i32,
    kind: i32,
    x_suffix_len: usize,
) -> io::Result<File> {
    let tryfunc = match kind {
        GT_FILE => |tmpl: &str, flags: &i32| try_file(tmpl, flags).map(|f| (Some(f), ())),
        GT_DIR => |tmpl: &str, flags: &i32| try_dir(tmpl, flags).map(|()| (None, ()))),
        GT_NOCREATE => |tmpl: &str, flags: &i32| try_nocreate(tmpl, flags).map(|()| (None, ()))),
        _ => return Err(io::Error::from_raw_os_error(libc::EINVAL)),
    };

    try_tempname_len(tmpl, suffixlen, &flags, tryfunc, x_suffix_len)
        .and_then(|(file, _)| file.ok_or_else(|| io::Error::from_raw_os_error(libc::EEXIST)))
}

pub fn try_tempname_len<F, A>(
    tmpl: &mut String,
    suffixlen: usize,
    args: &A,
    tryfunc: F,
    x_suffix_len: usize,
) -> io::Result<(Option<File>, A)>
where
    F: Fn(&str, &A) -> io::Result<(Option<File>, A)>>,
{
    let len = tmpl.len();
    if len < x_suffix_len + suffixlen
        || tmpl[len - x_suffix_len - suffixlen..]
            .chars()
            .take(x_suffix_len)
            .any(|c| c != 'X')
    {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let attempts = std::cmp::max(ATTEMPTS_MIN, TMP_MAX);
    let mut rng = rand::thread_rng();
    let x_start = len - x_suffix_len - suffixlen;

    for _ in 0..attempts {
        let random_part: String = (0..x_suffix_len)
            .map(|_| {
                let idx = rng.gen_range(0..LETTERS.len());
                LETTERS[idx] as char
            })
            .collect();

        tmpl.replace_range(x_start..x_start + x_suffix_len, &random_part);

        match tryfunc(tmpl, args) {
            Ok((file, args)) => return Ok((file, args)),
            Err(e) if e.raw_os_error() != Some(libc::EEXIST) => return Err(e),
            _ => continue,
        }
    }

    Err(io::Error::from_raw_os_error(libc::EEXIST))
}

pub fn gen_tempname(tmpl: &mut String, suffixlen: usize, flags: i32, kind: i32) -> io::Result<File> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, 6)
}

pub fn try_tempname<F, A>(
    tmpl: &mut String,
    suffixlen: usize,
    args: &A,
    tryfunc: F,
) -> io::Result<(Option<File>, A)>
where
    F: Fn(&str, &A) -> io::Result<(Option<File>, A)>>,
{
    try_tempname_len(tmpl, suffixlen, args, tryfunc, 6)
}