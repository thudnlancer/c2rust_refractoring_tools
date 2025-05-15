use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const TEMPLATE_SUFFIX_LEN: usize = 6;
const LETTERS: &[u8; 62] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Debug, Clone, Copy)]
pub enum TempFileKind {
    File,
    Dir,
    NoCreate,
}

fn random_bits(seed: u64) -> u64 {
    if let Ok(mut buf) = getrandom::getrandom(&mut [0u8; 8]) {
        let mut r = 0u64;
        r = u64::from_le_bytes(buf);
        return r;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    (2862933555777941757u64)
        .wrapping_mul(seed ^ now)
        .wrapping_add(3037000493u64)
}

pub fn gen_tempname(
    template: &mut CString,
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
) -> io::Result<File> {
    gen_tempname_len(template, suffixlen, flags, kind, TEMPLATE_SUFFIX_LEN)
}

pub fn gen_tempname_len(
    template: &mut CString,
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
    x_suffix_len: usize,
) -> io::Result<File> {
    try_tempname_len(template, suffixlen, flags, kind, x_suffix_len)
}

fn try_tempname_len(
    template: &mut CString,
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
    x_suffix_len: usize,
) -> io::Result<File> {
    let template_bytes = template.as_bytes_with_nul();
    let len = template_bytes.len() - 1; // exclude null terminator

    if len < x_suffix_len + suffixlen {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let x_pos = len - x_suffix_len - suffixlen;
    let x_slice = &template_bytes[x_pos..x_pos + x_suffix_len];
    if x_slice.iter().any(|&c| c != b'X') {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let mut x_part = Vec::from(x_slice);
    let attempts = 62u32.pow(x_suffix_len as u32);
    let mut seed = random_bits(0);

    for _ in 0..attempts {
        let mut v = seed;
        for i in 0..x_suffix_len {
            x_part[i] = LETTERS[(v % 62) as usize];
            v /= 62;
        }

        let new_path = unsafe {
            CString::from_vec_with_nul_unchecked(
                [&template_bytes[..x_pos], &x_part, &template_bytes[x_pos + x_suffix_len..]
                    .concat(),
            )
        };

        match kind {
            TempFileKind::File => {
                let file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode((0o600 | (flags & !0o3)) as u32)
                    .open(Path::new(new_path.to_str().unwrap()))?;
                return Ok(file);
            }
            TempFileKind::Dir => {
                std::fs::create_dir(Path::new(new_path.to_str().unwrap()))?;
                return Ok(File::open(new_path.to_str().unwrap())?);
            }
            TempFileKind::NoCreate => {
                if !Path::new(new_path.to_str().unwrap()).exists() {
                    return Ok(File::open("/dev/null")?); // Return a dummy file
                }
            }
        }

        seed = random_bits(seed);
    }

    Err(io::Error::from_raw_os_error(libc::EEXIST))
}

// Helper function to create CString from parts
unsafe fn cstring_from_parts(parts: &[&[u8]]) -> CString {
    let mut v = Vec::new();
    for part in parts {
        v.extend_from_slice(part);
    }
    CString::from_vec_with_nul_unchecked(v)
}