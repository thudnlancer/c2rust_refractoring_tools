use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, MetadataExt};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, timespec, UTIME_NOW, UTIME_OMIT, AT_FDCWD, AT_SYMLINK_NOFOLLOW};
use nix::sys::stat::{futimens, utimensat};
use nix::sys::time::TimeSpec;

#[derive(Debug)]
pub enum UtimensError {
    InvalidTimespec,
    UnsupportedOperation,
    IoError(io::Error),
    NixError(nix::Error),
}

impl From<io::Error> for UtimensError {
    fn from(err: io::Error) -> Self {
        UtimensError::IoError(err)
    }
}

impl From<nix::Error> for UtimensError {
    fn from(err: nix::Error) -> Self {
        UtimensError::NixError(err)
    }
}

fn validate_timespec(timespec: &[TimeSpec; 2]) -> Result<i32, UtimensError> {
    let mut result = 0;
    let mut utime_omit_count = 0;

    for ts in timespec.iter() {
        let tv_nsec = ts.nanoseconds();
        if tv_nsec != UTIME_NOW as i64 && 
           tv_nsec != UTIME_OMIT as i64 && 
           !(0 <= tv_nsec && tv_nsec < 1_000_000_000) {
            return Err(UtimensError::InvalidTimespec);
        }
    }

    if timespec[0].nanoseconds() == UTIME_NOW as i64 || 
       timespec[0].nanoseconds() == UTIME_OMIT as i64 {
        result = 1;
        if timespec[0].nanoseconds() == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    if timespec[1].nanoseconds() == UTIME_NOW as i64 || 
       timespec[1].nanoseconds() == UTIME_OMIT as i64 {
        result = 1;
        if timespec[1].nanoseconds() == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    Ok(result + if utime_omit_count == 1 { 1 } else { 0 })
}

fn update_timespec(stat: &std::fs::Metadata, ts: &mut [TimeSpec; 2]) -> bool {
    if ts[0].nanoseconds() == UTIME_OMIT as i64 && ts[1].nanoseconds() == UTIME_OMIT as i64 {
        return true;
    }

    if ts[0].nanoseconds() == UTIME_NOW as i64 && ts[1].nanoseconds() == UTIME_NOW as i64 {
        return false;
    }

    if ts[0].nanoseconds() == UTIME_OMIT as i64 {
        ts[0] = TimeSpec::from(stat.atime(), stat.atime_nsec() as i64);
    } else if ts[0].nanoseconds() == UTIME_NOW as i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[0] = TimeSpec::new(now.as_secs() as i64, now.subsec_nanos() as i64);
    }

    if ts[1].nanoseconds() == UTIME_OMIT as i64 {
        ts[1] = TimeSpec::from(stat.mtime(), stat.mtime_nsec() as i64);
    } else if ts[1].nanoseconds() == UTIME_NOW as i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[1] = TimeSpec::new(now.as_secs() as i64, now.subsec_nanos() as i64);
    }

    false
}

pub fn fdutimens(fd: Option<&File>, path: Option<&Path>, times: Option<&[TimeSpec; 2]>) -> Result<(), UtimensError> {
    let mut adjusted_times = match times {
        Some(ts) => {
            let mut adjusted = [ts[0], ts[1]];
            let adj_needed = validate_timespec(&adjusted)?;
            if adj_needed < 0 {
                return Err(UtimensError::InvalidTimespec);
            }
            Some(adjusted)
        }
        None => None,
    };

    if fd.is_none() && path.is_none() {
        return Err(UtimensError::IoError(io::Error::from_raw_os_error(libc::EBADF)));
    }

    if let Some(ts) = &mut adjusted_times {
        let stat = if let Some(f) = fd {
            f.metadata()?
        } else if let Some(p) = path {
            p.metadata()?
        } else {
            return Err(UtimensError::IoError(io::Error::from_raw_os_error(libc::EBADF)));
        };

        if update_timespec(&stat, ts) {
            return Ok(());
        }
    }

    if let Some(f) = fd {
        futimens(f.as_raw_fd(), adjusted_times.as_ref().map(|t| &t[..]))?;
    } else if let Some(p) = path {
        utimensat(Some(AT_FDCWD), p, adjusted_times.as_ref().map(|t| &t[..]), 0)?;
    }

    Ok(())
}

pub fn utimens(path: &Path, times: Option<&[TimeSpec; 2]>) -> Result<(), UtimensError> {
    fdutimens(None, Some(path), times)
}

pub fn lutimens(path: &Path, times: Option<&[TimeSpec; 2]>) -> Result<(), UtimensError> {
    let mut adjusted_times = match times {
        Some(ts) => {
            let mut adjusted = [ts[0], ts[1]];
            let adj_needed = validate_timespec(&adjusted)?;
            if adj_needed < 0 {
                return Err(UtimensError::InvalidTimespec);
            }
            Some(adjusted)
        }
        None => None,
    };

    let stat = std::fs::symlink_metadata(path)?;
    if let Some(ts) = &mut adjusted_times {
        if update_timespec(&stat, ts) {
            return Ok(());
        }
    }

    if stat.file_type().is_symlink() {
        utimensat(Some(AT_FDCWD), path, adjusted_times.as_ref().map(|t| &t[..]), AT_SYMLINK_NOFOLLOW)?;
        Ok(())
    } else {
        fdutimens(None, Some(path), times)
    }
}