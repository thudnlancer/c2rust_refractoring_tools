use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const TEMPLATE_SUFFIX_LEN: usize = 6;
const LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(Debug, Clone, Copy)]
pub enum TempFileKind {
    File,
    Dir,
    NoCreate,
}

pub fn gen_tempname(
    tmpl: &mut CString,
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
) -> io::Result<File> {
    gen_tempname_len(tmpl, suffixlen, flags, kind, TEMPLATE_SUFFIX_LEN)
}

pub fn gen_tempname_len(
    tmpl: &mut CString,
    suffixlen: usize,
    flags: i32,
    kind: TempFileKind,
    x_suffix_len: usize,
) -> io::Result<File> {
    let tryfunc = match kind {
        TempFileKind::File => try_file,
        TempFileKind::Dir => try_dir,
        TempFileKind::NoCreate => try_nocreate,
    };
    try_tempname_len(tmpl, suffixlen, flags, tryfunc, x_suffix_len)
}

fn try_tempname_len(
    tmpl: &mut CString,
    suffixlen: usize,
    flags: i32,
    tryfunc: fn(&mut CString, i32) -> io::Result<File>,
    x_suffix_len: usize,
) -> io::Result<File> {
    let len = tmpl.as_bytes().len();
    if len < x_suffix_len + suffixlen {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let x_pos = len - x_suffix_len - suffixlen;
    let x_part = &mut tmpl.as_bytes_mut()[x_pos..x_pos + x_suffix_len];
    if !x_part.iter().all(|&c| c == b'X') {
        return Err(io::Error::from_raw_os_error(libc::EINVAL));
    }

    let attempts = 62 * 62 * 62;
    let save_errno = io::Error::last_os_error();

    for _ in 0..attempts {
        let mut rng = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        for i in 0..x_suffix_len {
            let idx = (rng % 62) as usize;
            x_part[i] = LETTERS[idx];
            rng /= 62;
        }

        match tryfunc(tmpl, flags) {
            Ok(fd) => return Ok(fd),
            Err(e) if e.raw_os_error() != Some(libc::EEXIST) => return Err(e),
            _ => continue,
        }
    }

    Err(io::Error::from_raw_os_error(libc::EEXIST))
}

fn try_file(tmpl: &mut CString, flags: i32) -> io::Result<File> {
    let path = Path::new(OsStr::from_bytes(tmpl.as_bytes()));
    OpenOptions::new()
        .mode(0o600)
        .flags(flags & !0o3 | libc::O_CLOEXEC | libc::O_CREAT | libc::O_EXCL)
        .read(true)
        .write(true)
        .open(path)
}

fn try_dir(tmpl: &mut CString, _flags: i32) -> io::Result<File> {
    let path = Path::new(OsStr::from_bytes(tmpl.as_bytes()));
    std::fs::DirBuilder::new()
        .mode(0o700)
        .create(path)
        .map(|_| File::open(path).unwrap())
}

fn try_nocreate(tmpl: &mut CString, _flags: i32) -> io::Result<File> {
    let path = Path::new(OsStr::from_bytes(tmpl.as_bytes()));
    match std::fs::metadata(path) {
        Ok(_) => Err(io::Error::from_raw_os_error(libc::EEXIST)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(File::open("/dev/null").unwrap()),
        Err(e) => Err(e),
    }
}