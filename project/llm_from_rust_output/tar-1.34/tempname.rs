use std::fs::{File, OpenOptions, DirBuilder};
use std::io;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
use rand::Rng;
use libc::{self, c_int, mode_t};
use std::time::{SystemTime, UNIX_EPOCH};

const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const TEMPLATE_SUFFIX_LEN: usize = 6;

#[derive(Debug, Clone, Copy)]
pub enum TempFileKind {
    File,
    Dir,
    NoCreate,
}

fn random_bits(seed: u64) -> u64 {
    let mut rng = rand::thread_rng();
    if let Ok(bytes) = getrandom::getrandom(&mut [0u8; 8]) {
        if bytes == 8 {
            return u64::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]);
        }
    }
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    (2862933555777941757u64.wrapping_mul(seed ^ now)).wrapping_add(3037000493u64)
}

fn try_file(tmpl: &Path, flags: i32) -> io::Result<File> {
    let open_flags = flags & !0o3 | libc::O_CLOEXEC | libc::O_CREAT | libc::O_EXCL;
    OpenOptions::new()
        .mode(0o600)
        .custom_flags(open_flags)
        .open(tmpl)
}

fn try_dir(tmpl: &Path, _flags: i32) -> io::Result<()> {
    DirBuilder::new()
        .mode(0o700)
        .create(tmpl)
}

fn try_nocreate(tmpl: &Path, _flags: i32) -> io::Result<()> {
    match tmpl.metadata() {
        Ok(_) => Err(io::Error::from_raw_os_error(libc::EEXIST)),
        Err(e) if e.raw_os_error() == Some(libc::ENOENT) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn gen_tempname(tmpl: &mut [u8], suffixlen: usize, flags: i32, kind: TempFileKind) -> io::Result<()> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, TEMPLATE_SUFFIX_LEN)
}

pub fn gen_tempname_len(
    tmpl: &mut [u8],
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
    x_suffix_len: usize,
) -> io::Result<()> {
    let try_func = match kind {
        TempFileKind::File => try_file,
        TempFileKind::Dir => |path, flags| try_dir(path, flags).map(|_| ()),
        TempFileKind::NoCreate => |path, flags| try_nocreate(path, flags),
    };
    
    try_tempname_len(tmpl, suffixlen, flags, try_func, x_suffix_len)
}

pub fn try_tempname(
    tmpl: &mut [u8],
    suffixlen: usize,
    flags: i32,
    try_func: fn(&Path, i32) -> io::Result<()>,
) -> io::Result<()> {
    try_tempname_len(tmpl, suffixlen, flags, try_func, TEMPLATE_SUFFIX_LEN)
}

pub fn try_tempname_len(
    tmpl: &mut [u8],
    suffixlen: usize,
    flags: i32,
    try_func: fn(&Path, i32) -> io::Result<()>,
    x_suffix_len: usize,
) -> io::Result<()> {
    let len = tmpl.len();
    if len < x_suffix_len + suffixlen {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let x_pos = len - x_suffix_len - suffixlen;
    if !tmpl[x_pos..x_pos + x_suffix_len].iter().all(|&c| c == b'X') {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let attempts = 62 * 62 * 62;
    let unfair_min = u64::MAX - u64::MAX % (62u64.pow(10));

    let mut v = 0u64;
    let mut count = 0;

    while count < attempts {
        let mut rng = random_bits(v);
        v = rng;

        for i in 0..x_suffix_len {
            if rng == 0 {
                loop {
                    rng = random_bits(v);
                    if rng < unfair_min {
                        break;
                    }
                }
                v = rng;
            }

            tmpl[x_pos + i] = LETTERS[(rng % 62) as usize];
            rng /= 62;
        }

        let path = Path::new(unsafe { std::str::from_utf8_unchecked(tmpl) });
        match try_func(path, flags) {
            Ok(_) => return Ok(()),
            Err(e) if e.raw_os_error() == Some(libc::EEXIST) => {
                count += 1;
                continue;
            }
            Err(e) => return Err(e),
        }
    }

    Err(io::Error::from_raw_os_error(libc::EEXIST))
}