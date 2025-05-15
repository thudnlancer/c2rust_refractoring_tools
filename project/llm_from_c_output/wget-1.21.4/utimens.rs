use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::{OpenOptionsExt, MetadataExt};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, timespec, AT_FDCWD, AT_SYMLINK_NOFOLLOW};
use nix::sys::stat::{futimens, utimensat, UTIME_NOW, UTIME_OMIT};
use nix::sys::time::TimeSpec;
use nix::fcntl::OFlag;

const TIMESPEC_HZ: i64 = 1_000_000_000;

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

fn validate_timespec(times: &[timespec; 2]) -> Result<i32, UtimensError> {
    let mut result = 0;
    let mut utime_omit_count = 0;

    for time in times {
        if time.tv_nsec != UTIME_NOW as i64 && 
           time.tv_nsec != UTIME_OMIT as i64 &&
           !(0 <= time.tv_nsec && time.tv_nsec < TIMESPEC_HZ) {
            return Err(UtimensError::InvalidTimespec);
        }
    }

    if times[0].tv_nsec == UTIME_NOW as i64 || times[0].tv_nsec == UTIME_OMIT as i64 {
        if times[0].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
        result = 1;
    }

    if times[1].tv_nsec == UTIME_NOW as i64 || times[1].tv_nsec == UTIME_OMIT as i64 {
        if times[1].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
        result = 1;
    }

    Ok(result + if utime_omit_count == 1 { 1 } else { 0 })
}

fn update_timespec(stat: &std::fs::Metadata, times: &mut [timespec; 2]) -> bool {
    if times[0].tv_nsec == UTIME_OMIT as i64 && times[1].tv_nsec == UTIME_OMIT as i64 {
        return true;
    }

    if times[0].tv_nsec == UTIME_NOW as i64 && times[1].tv_nsec == UTIME_NOW as i64 {
        return false;
    }

    if times[0].tv_nsec == UTIME_OMIT as i64 {
        times[0].tv_sec = stat.atime();
        times[0].tv_nsec = stat.atime_nsec() as i64;
    } else if times[0].tv_nsec == UTIME_NOW as i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        times[0].tv_sec = now.as_secs() as i64;
        times[0].tv_nsec = now.subsec_nanos() as i64;
    }

    if times[1].tv_nsec == UTIME_OMIT as i64 {
        times[1].tv_sec = stat.mtime();
        times[1].tv_nsec = stat.mtime_nsec() as i64;
    } else if times[1].tv_nsec == UTIME_NOW as i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        times[1].tv_sec = now.as_secs() as i64;
        times[1].tv_nsec = now.subsec_nanos() as i64;
    }

    false
}

pub fn fdutimens(fd: Option<&File>, file: Option<&Path>, times: Option<&[timespec; 2]>) -> Result<(), UtimensError> {
    let mut adjusted_times = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut adjustment_needed = 0;

    if let Some(ts) = times {
        adjusted_times = *ts;
        adjustment_needed = validate_timespec(&adjusted_times)?;
    }

    if fd.is_none() && file.is_none() {
        return Err(UtimensError::UnsupportedOperation);
    }

    // Try modern syscalls first
    if let Some(fd) = fd {
        futimens(fd.as_raw_fd(), &adjusted_times)?;
        return Ok(());
    }

    if let Some(path) = file {
        utimensat(Some(AT_FDCWD), path, &adjusted_times, 0)?;
        return Ok(());
    }

    // Fallback implementations would go here
    Err(UtimensError::UnsupportedOperation)
}

pub fn utimens(file: &Path, times: Option<&[timespec; 2]>) -> Result<(), UtimensError> {
    fdutimens(None, Some(file), times)
}

pub fn lutimens(file: &Path, times: Option<&[timespec; 2]>) -> Result<(), UtimensError> {
    let mut adjusted_times = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut adjustment_needed = 0;

    if let Some(ts) = times {
        adjusted_times = *ts;
        adjustment_needed = validate_timespec(&adjusted_times)?;
    }

    // Try modern syscall first
    match utimensat(Some(AT_FDCWD), file, &adjusted_times, AT_SYMLINK_NOFOLLOW) {
        Ok(_) => Ok(()),
        Err(nix::Error::Sys(errno)) if errno == nix::errno::Errno::ENOSYS => {
            // Fallback implementation would go here
            Err(UtimensError::UnsupportedOperation)
        },
        Err(e) => Err(UtimensError::NixError(e)),
    }
}