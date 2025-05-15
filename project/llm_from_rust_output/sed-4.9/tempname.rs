use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const TEMPLATE_SUFFIX_LEN: usize = 6;

type RandomValue = u64;

fn mix_random_values(r: RandomValue, s: RandomValue) -> RandomValue {
    (2862933555777941757u64)
        .wrapping_mul(r)
        .wrapping_add(3037000493u64) ^ s
}

fn random_bits(r: &mut RandomValue, s: RandomValue) -> io::Result<()> {
    let mut buf = [0u8; 8];
    if let Ok(len) = getrandom::getrandom(&mut buf) {
        if len == 8 {
            *r = RandomValue::from_ne_bytes(buf);
            return Ok(());
        }
    }

    let mut v = s;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    v = mix_random_values(v, now.as_secs());
    v = mix_random_values(v, now.subsec_nanos() as u64);
    *r = mix_random_values(v, SystemTime::now().elapsed().unwrap_or_default().as_nanos() as u64);
    Ok(())
}

fn try_file(tmpl: &Path, flags: i32) -> io::Result<File> {
    let open_flags = flags & !0o3 | 0o2 | 0o100 | 0o200;
    OpenOptions::new()
        .mode((0o400 | 0o200) as u32)
        .custom_flags(open_flags)
        .open(tmpl)
}

fn try_dir(tmpl: &Path) -> io::Result<()> {
    std::fs::create_dir(tmpl)?;
    Ok(())
}

fn try_nocreate(tmpl: &Path) -> io::Result<()> {
    match std::fs::metadata(tmpl) {
        Ok(_) => Err(io::Error::from_raw_os_error(libc::EEXIST)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn gen_tempname_len(
    tmpl: &mut CString,
    suffixlen: i32,
    flags: i32,
    kind: i32,
    x_suffix_len: usize,
) -> io::Result<i32> {
    let tryfunc = match kind {
        0 => try_file,
        1 => |path, _| try_dir(path).map(|_| 0),
        2 => |path, _| try_nocreate(path).map(|_| 0),
        _ => return Err(io::Error::from_raw_os_error(libc::EINVAL)),
    };

    try_tempname_len(tmpl, suffixlen, flags, tryfunc, x_suffix_len)
}

pub fn try_tempname_len<F>(
    tmpl: &mut CString,
    suffixlen: i32,
    flags: i32,
    tryfunc: F,
    x_suffix_len: usize,
) -> io::Result<i32>
where
    F: Fn(&Path, i32) -> io::Result<i32>,
{
    let len = tmpl.as_bytes().len();
    if len < x_suffix_len + suffixlen as usize {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let xxxxxx_start = len - x_suffix_len - suffixlen as usize;
    if !tmpl.as_bytes()[xxxxxx_start..xxxxxx_start + x_suffix_len]
        .iter()
        .all(|&c| c == b'X')
    {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let attempts = 62u32.pow(3);
    let biased_min = u64::MAX - u64::MAX % 62u64.pow(10);

    let mut v = 0u64;
    let mut count = 0;
    let mut tmpl_bytes = tmpl.into_bytes();

    while count < attempts {
        let mut vdigbuf = v;
        let mut vdigits = 10;

        for i in 0..x_suffix_len {
            if vdigits == 0 {
                while random_bits(&mut v, v).is_ok() && biased_min <= v {}
                vdigbuf = v;
                vdigits = 10;
            }

            tmpl_bytes[xxxxxx_start + i] = LETTERS[(vdigbuf % 62) as usize];
            vdigbuf /= 62;
            vdigits -= 1;
        }

        let new_tmpl = unsafe { CString::from_vec_unchecked(tmpl_bytes.clone()) };
        match tryfunc(Path::new(new_tmpl.as_c_str()), flags) {
            Ok(fd) => return Ok(fd),
            Err(e) if e.raw_os_error() == Some(libc::EEXIST) => {
                count += 1;
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Err(io::Error::from_raw_os_error(libc::EEXIST))
}

pub fn gen_tempname(
    tmpl: &mut CString,
    suffixlen: i32,
    flags: i32,
    kind: i32,
) -> io::Result<i32> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, TEMPLATE_SUFFIX_LEN)
}

pub fn try_tempname<F>(
    tmpl: &mut CString,
    suffixlen: i32,
    flags: i32,
    tryfunc: F,
) -> io::Result<i32>
where
    F: Fn(&Path, i32) -> io::Result<i32>,
{
    try_tempname_len(tmpl, suffixlen, flags, tryfunc, TEMPLATE_SUFFIX_LEN)
}