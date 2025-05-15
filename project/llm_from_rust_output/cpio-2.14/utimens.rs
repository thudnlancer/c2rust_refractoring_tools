use std::os::unix::fs::{FileTimeExt, MetadataExt};
use std::fs::{File, OpenOptions, metadata, symlink_metadata};
use std::os::unix::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, timespec, timeval, stat};
use std::ptr;

const TIMESPEC_HZ: u64 = 1_000_000_000;
const UTIME_NOW: i64 = ((1 << 30) - 1);
const UTIME_OMIT: i64 = ((1 << 30) - 2);

fn validate_timespec(ts: &[timespec; 2]) -> Result<i32, i32> {
    let mut result = 0;
    let mut utime_omit_count = 0;

    for t in ts {
        if t.tv_nsec != UTIME_NOW && t.tv_nsec != UTIME_OMIT &&
           !(0 <= t.tv_nsec && (t.tv_nsec as u64) < TIMESPEC_HZ) {
            return Err(libc::EINVAL);
        }
    }

    for i in 0..2 {
        if ts[i].tv_nsec == UTIME_NOW || ts[i].tv_nsec == UTIME_OMIT {
            result = 1;
            if ts[i].tv_nsec == UTIME_OMIT {
                utime_omit_count += 1;
            }
        }
    }

    Ok(result + if utime_omit_count == 1 { 1 } else { 0 })
}

fn update_timespec(metadata: &std::fs::Metadata, ts: &mut [timespec; 2]) -> bool {
    if ts[0].tv_nsec == UTIME_OMIT && ts[1].tv_nsec == UTIME_OMIT {
        return true;
    }

    if ts[0].tv_nsec == UTIME_NOW && ts[1].tv_nsec == UTIME_NOW {
        return false;
    }

    if ts[0].tv_nsec == UTIME_OMIT {
        ts[0].tv_sec = metadata.atime() as i64;
        ts[0].tv_nsec = metadata.atime_nsec() as i64;
    } else if ts[0].tv_nsec == UTIME_NOW {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[0].tv_sec = now.as_secs() as i64;
        ts[0].tv_nsec = now.subsec_nanos() as i64;
    }

    if ts[1].tv_nsec == UTIME_OMIT {
        ts[1].tv_sec = metadata.mtime() as i64;
        ts[1].tv_nsec = metadata.mtime_nsec() as i64;
    } else if ts[1].tv_nsec == UTIME_NOW {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[1].tv_sec = now.as_secs() as i64;
        ts[1].tv_nsec = now.subsec_nanos() as i64;
    }

    false
}

pub fn fdutimens(fd: Option<RawFd>, path: Option<&Path>, times: Option<&[timespec; 2]>) -> Result<(), std::io::Error> {
    let mut adjusted_times = match times {
        Some(t) => *t,
        None => [timespec { tv_sec: 0, tv_nsec: UTIME_OMIT }; 2],
    };

    let adjustment_needed = if times.is_some() {
        validate_timespec(&adjusted_times)?
    } else {
        0
    };

    if fd.is_none() && path.is_none() {
        return Err(std::io::Error::from_raw_os_error(libc::EBADF));
    }

    // Try utimensat first if available
    unsafe {
        if let Some(p) = path {
            let c_path = std::ffi::CString::new(p.as_os_str().as_bytes()).unwrap();
            if libc::utimensat(
                libc::AT_FDCWD,
                c_path.as_ptr(),
                &adjusted_times as *const _,
                0,
            ) == 0
            {
                return Ok(());
            }
        }

        if let Some(fd) = fd {
            if libc::futimens(fd, &adjusted_times as *const _) == 0 {
                return Ok(());
            }
        }
    }

    // Fallback to futimes/utimes
    let mut timevals = [timeval { tv_sec: 0, tv_usec: 0 }; 2];
    if times.is_some() {
        for i in 0..2 {
            timevals[i].tv_sec = adjusted_times[i].tv_sec;
            timevals[i].tv_usec = adjusted_times[i].tv_nsec / 1000;
        }
    }

    unsafe {
        if let Some(fd) = fd {
            if libc::futimes(fd, &timevals as *const _) == 0 {
                return Ok(());
            }
        }

        if let Some(p) = path {
            let c_path = std::ffi::CString::new(p.as_os_str().as_bytes()).unwrap();
            if libc::utimes(c_path.as_ptr(), &timevals as *const _) == 0 {
                return Ok(());
            }
        }
    }

    Err(std::io::Error::last_os_error())
}

pub fn utimens(path: &Path, times: Option<&[timespec; 2]>) -> Result<(), std::io::Error> {
    fdutimens(None, Some(path), times)
}

pub fn lutimens(path: &Path, times: Option<&[timespec; 2]>) -> Result<(), std::io::Error> {
    let metadata = symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(std::io::Error::from_raw_os_error(libc::ENOSYS));
    }
    utimens(path, times)
}