use std::os::unix::fs::{FileTimeExt, MetadataExt};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{File, OpenOptions, metadata, symlink_metadata};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::io::{self, Error, ErrorKind};
use libc::{c_int, timespec, UTIME_NOW, UTIME_OMIT};
use std::os::unix::io::AsRawFd;

#[cfg(target_os = "linux")]
use libc::{AT_FDCWD, AT_SYMLINK_NOFOLLOW};

#[derive(Debug)]
struct TimeSpec {
    tv_sec: i64,
    tv_nsec: i64,
}

impl TimeSpec {
    fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        TimeSpec {
            tv_sec: now.as_secs() as i64,
            tv_nsec: now.subsec_nanos() as i64,
        }
    }

    fn omit() -> Self {
        TimeSpec {
            tv_sec: 0,
            tv_nsec: UTIME_OMIT as i64,
        }
    }

    fn from_timespec(ts: ×pec) -> Self {
        TimeSpec {
            tv_sec: ts.tv_sec,
            tv_nsec: ts.tv_nsec,
        }
    }

    fn to_timespec(&self) -> timespec {
        timespec {
            tv_sec: self.tv_sec,
            tv_nsec: self.tv_nsec,
        }
    }
}

fn validate_timespec(times: &[TimeSpec; 2]) -> Result<i32, Error> {
    for ts in times {
        if ts.tv_nsec != UTIME_NOW as i64 
            && ts.tv_nsec != UTIME_OMIT as i64
            && (ts.tv_nsec < 0 || ts.tv_nsec >= 1_000_000_000)
        {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid timespec value"));
        }
    }

    let mut result = 0;
    let mut utime_omit_count = 0;

    if times[0].tv_nsec == UTIME_NOW as i64 || times[0].tv_nsec == UTIME_OMIT as i64 {
        result = 1;
        if times[0].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    if times[1].tv_nsec == UTIME_NOW as i64 || times[1].tv_nsec == UTIME_OMIT as i64 {
        result = 1;
        if times[1].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    Ok(result + if utime_omit_count == 1 { 1 } else { 0 })
}

fn update_timespec(stat: &std::fs::Metadata, times: &mut [TimeSpec; 2]) -> bool {
    if times[0].tv_nsec == UTIME_OMIT as i64 && times[1].tv_nsec == UTIME_OMIT as i64 {
        return true;
    }

    if times[0].tv_nsec == UTIME_NOW as i64 && times[1].tv_nsec == UTIME_NOW as i64 {
        return false;
    }

    if times[0].tv_nsec == UTIME_OMIT as i64 {
        times[0] = TimeSpec {
            tv_sec: stat.atime(),
            tv_nsec: stat.atime_nsec() as i64,
        };
    } else if times[0].tv_nsec == UTIME_NOW as i64 {
        times[0] = TimeSpec::now();
    }

    if times[1].tv_nsec == UTIME_OMIT as i64 {
        times[1] = TimeSpec {
            tv_sec: stat.mtime(),
            tv_nsec: stat.mtime_nsec() as i64,
        };
    } else if times[1].tv_nsec == UTIME_NOW as i64 {
        times[1] = TimeSpec::now();
    }

    false
}

pub fn fdutimens(fd: Option<&File>, path: Option<&Path>, times: Option<[TimeSpec; 2]>) -> io::Result<()> {
    let mut adjusted_times = match times {
        Some(t) => t,
        None => return set_file_times_now(fd, path),
    };

    let adjustment_needed = validate_timespec(&adjusted_times)?;

    if fd.is_none() && path.is_none() {
        return Err(Error::new(ErrorKind::InvalidInput, "Either fd or path must be specified"));
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(fd) = fd {
            let raw_fd = fd.as_raw_fd();
            let ts = [
                adjusted_times[0].to_timespec(),
                adjusted_times[1].to_timespec(),
            ];
            let res = unsafe { libc::futimens(raw_fd, ts.as_ptr()) };
            if res == 0 {
                return Ok(());
            }
        } else if let Some(p) = path {
            let ts = [
                adjusted_times[0].to_timespec(),
                adjusted_times[1].to_timespec(),
            ];
            let res = unsafe { libc::utimensat(AT_FDCWD, p.as_os_str().as_bytes().as_ptr() as *const i8, ts.as_ptr(), 0) };
            if res == 0 {
                return Ok(());
            }
        }
    }

    let stat = if let Some(fd) = fd {
        fd.metadata()?
    } else if let Some(p) = path {
        metadata(p)?
    } else {
        return Err(Error::new(ErrorKind::InvalidInput, "No file descriptor or path provided"));
    };

    if adjustment_needed > 0 {
        update_timespec(&stat, &mut adjusted_times);
    }

    let atime = FileTime::from_unix_time(adjusted_times[0].tv_sec, adjusted_times[0].tv_nsec as u32);
    let mtime = FileTime::from_unix_time(adjusted_times[1].tv_sec, adjusted_times[1].tv_nsec as u32);

    if let Some(fd) = fd {
        fd.set_file_times(atime, mtime)?;
    } else if let Some(p) = path {
        filetime::set_file_times(p, atime, mtime)?;
    }

    Ok(())
}

pub fn utimens(path: &Path, times: Option<[TimeSpec; 2]>) -> io::Result<()> {
    fdutimens(None, Some(path), times)
}

pub fn lutimens(path: &Path, times: Option<[TimeSpec; 2]>) -> io::Result<()> {
    let mut adjusted_times = match times {
        Some(t) => t,
        None => return set_file_times_now(None, Some(path)),
    };

    let adjustment_needed = validate_timespec(&adjusted_times)?;

    #[cfg(target_os = "linux")]
    {
        let ts = [
            adjusted_times[0].to_timespec(),
            adjusted_times[1].to_timespec(),
        ];
        let res = unsafe { libc::utimensat(AT_FDCWD, path.as_os_str().as_bytes().as_ptr() as *const i8, ts.as_ptr(), AT_SYMLINK_NOFOLLOW) };
        if res == 0 {
            return Ok(());
        }
    }

    let stat = symlink_metadata(path)?;
    if stat.file_type().is_symlink() {
        return Err(Error::new(ErrorKind::Unsupported, "Symlink timestamp modification not supported"));
    }

    if adjustment_needed > 0 {
        update_timespec(&stat, &mut adjusted_times);
    }

    let atime = FileTime::from_unix_time(adjusted_times[0].tv_sec, adjusted_times[0].tv_nsec as u32);
    let mtime = FileTime::from_unix_time(adjusted_times[1].tv_sec, adjusted_times[1].tv_nsec as u32);

    filetime::set_file_times(path, atime, mtime)?;
    Ok(())
}

fn set_file_times_now(fd: Option<&File>, path: Option<&Path>) -> io::Result<()> {
    let now = TimeSpec::now();
    let times = [now.clone(), now];
    fdutimens(fd, path, Some(times))
}