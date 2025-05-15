use std::fs;
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

const RENAME_NOREPLACE: u32 = 1 << 0;
const RENAME_EXCHANGE: u32 = 1 << 1;
const RENAME_WHITEOUT: u32 = 1 << 2;

#[cfg(target_os = "linux")]
fn renameat2(
    olddirfd: i32,
    oldpath: &Path,
    newdirfd: i32,
    newpath: &Path,
    flags: u32,
) -> Result<(), Error> {
    let oldpath_c = oldpath.to_str().ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
    let newpath_c = newpath.to_str().ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
    
    let res = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            olddirfd,
            oldpath_c.as_ptr() as *const libc::c_char,
            newdirfd,
            newpath_c.as_ptr() as *const libc::c_char,
            flags,
        )
    };
    
    if res == 0 {
        Ok(())
    } else {
        Err(Error::last_os_error())
    }
}

fn renameatu(
    fd1: i32,
    src: &Path,
    fd2: i32,
    dst: &Path,
    flags: u32,
) -> Result<(), Error> {
    #[cfg(target_os = "linux")]
    {
        match renameat2(fd1, src, fd2, dst, flags) {
            Ok(()) => return Ok(()),
            Err(e) => match e.raw_os_error() {
                Some(libc::EINVAL) | Some(libc::ENOSYS) | Some(libc::ENOTSUP) => (),
                _ => return Err(e),
            },
        }
    }

    if flags & !(RENAME_NOREPLACE | RENAME_EXCHANGE) != 0 {
        return Err(Error::from_raw_os_error(libc::ENOTSUP));
    }

    if flags & RENAME_NOREPLACE != 0 {
        match fs::metadata_at(fd2, dst) {
            Ok(_) => return Err(Error::from_raw_os_error(libc::EEXIST)),
            Err(e) if e.kind() != ErrorKind::NotFound => return Err(e),
            _ => (),
        }
    }

    let src_has_slash = src.to_str().map(|s| s.ends_with('/')).unwrap_or(false);
    let dst_has_slash = dst.to_str().map(|s| s.ends_with('/')).unwrap_or(false);

    if !src_has_slash && !dst_has_slash {
        return fs::rename_at(fd1, src, fd2, dst);
    }

    let src_metadata = fs::metadata_at(fd1, src)?;
    if src_has_slash && !src_metadata.is_dir() {
        return Err(Error::from_raw_os_error(libc::ENOTDIR));
    }

    match fs::metadata_at(fd2, dst) {
        Ok(dst_metadata) => {
            if !dst_metadata.is_dir() {
                return Err(Error::from_raw_os_error(libc::ENOTDIR));
            }
            if src_has_slash && !src_metadata.is_dir() {
                return Err(Error::from_raw_os_error(libc::EISDIR));
            }
        }
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if src_has_slash && !src_metadata.is_dir() {
                return Err(Error::from_raw_os_error(libc::ENOENT));
            }
        }
        Err(e) => return Err(e),
    }

    let src_path = if src_has_slash {
        PathBuf::from(src).strip_suffix("/").unwrap_or(src).to_path_buf()
    } else {
        src.to_path_buf()
    };

    let dst_path = if dst_has_slash {
        PathBuf::from(dst).strip_suffix("/").unwrap_or(dst).to_path_buf()
    } else {
        dst.to_path_buf()
    };

    fs::rename_at(fd1, &src_path, fd2, &dst_path)
}

trait FileSystemExt {
    fn metadata_at(fd: i32, path: &Path) -> Result<fs::Metadata, Error>;
    fn rename_at(olddirfd: i32, oldpath: &Path, newdirfd: i32, newpath: &Path) -> Result<(), Error>;
}

impl FileSystemExt for fs {
    fn metadata_at(fd: i32, path: &Path) -> Result<fs::Metadata, Error> {
        let path_c = path.to_str().ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
        let mut stat = unsafe { std::mem::zeroed() };
        let res = unsafe {
            libc::fstatat(
                fd,
                path_c.as_ptr() as *const libc::c_char,
                &mut stat,
                libc::AT_SYMLINK_NOFOLLOW,
            )
        };
        if res == 0 {
            Ok(fs::Metadata::from_raw(stat))
        } else {
            Err(Error::last_os_error())
        }
    }

    fn rename_at(olddirfd: i32, oldpath: &Path, newdirfd: i32, newpath: &Path) -> Result<(), Error> {
        let oldpath_c = oldpath.to_str().ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
        let newpath_c = newpath.to_str().ok_or_else(|| Error::from_raw_os_error(libc::EINVAL))?;
        let res = unsafe {
            libc::renameat(
                olddirfd,
                oldpath_c.as_ptr() as *const libc::c_char,
                newdirfd,
                newpath_c.as_ptr() as *const libc::c_char,
            )
        };
        if res == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}