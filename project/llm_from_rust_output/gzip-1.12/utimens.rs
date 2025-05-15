use std::os::unix::prelude::*;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use libc::{c_int, c_char, c_long, c_uint, c_ulong, timespec, stat, timeval};

const TIMESPEC_HZ: c_long = 1_000_000_000;
const UTIME_NOW: c_long = ((1 << 30) - 1);
const UTIME_OMIT: c_long = ((1 << 30) - 2);
const AT_SYMLINK_NOFOLLOW: c_int = 0x100;

#[derive(Debug)]
pub enum TimeSpec {
    Now,
    Omit,
    Specific(timespec),
}

impl TimeSpec {
    fn to_raw(&self) -> timespec {
        match self {
            TimeSpec::Now => timespec {
                tv_sec: 0,
                tv_nsec: UTIME_NOW,
            },
            TimeSpec::Omit => timespec {
                tv_sec: 0,
                tv_nsec: UTIME_OMIT,
            },
            TimeSpec::Specific(ts) => *ts,
        }
    }

    fn from_raw(ts: timespec) -> Self {
        if ts.tv_nsec == UTIME_NOW {
            TimeSpec::Now
        } else if ts.tv_nsec == UTIME_OMIT {
            TimeSpec::Omit
        } else {
            TimeSpec::Specific(ts)
        }
    }
}

pub fn fdutimens(
    fd: Option<&File>,
    file: Option<&Path>,
    atime: TimeSpec,
    mtime: TimeSpec,
) -> io::Result<()> {
    let mut ts = [atime.to_raw(), mtime.to_raw()];
    validate_timespec(&mut ts)?;

    let needs_stat = matches!((&atime, &mtime), (TimeSpec::Omit, TimeSpec::Omit))
        || matches!((&atime, &mtime), (TimeSpec::Now, TimeSpec::Now));

    if needs_stat {
        let st = if let Some(fd) = fd {
            fd.metadata()?
        } else if let Some(file) = file {
            file.metadata()?
        } else {
            return Err(io::Error::from_raw_os_error(libc::EBADF));
        };

        let st_atim = st.accessed().unwrap_or(UNIX_EPOCH);
        let st_mtim = st.modified().unwrap_or(UNIX_EPOCH);

        if let TimeSpec::Omit = atime {
            ts[0] = timespec {
                tv_sec: st_atim.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                tv_nsec: st_atim.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            };
        }

        if let TimeSpec::Omit = mtime {
            ts[1] = timespec {
                tv_sec: st_mtim.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                tv_nsec: st_mtim.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            };
        }
    }

    if let Some(fd) = fd {
        unsafe {
            if libc::futimens(fd.as_raw_fd(), ts.as_ptr()) == -1 {
                return Err(io::Error::last_os_error());
            }
        }
    } else if let Some(file) = file {
        unsafe {
            let c_path = std::ffi::CString::new(file.as_os_str().as_bytes()).unwrap();
            if libc::utimensat(libc::AT_FDCWD, c_path.as_ptr(), ts.as_ptr(), 0) == -1 {
                return Err(io::Error::last_os_error());
            }
        }
    } else {
        return Err(io::Error::from_raw_os_error(libc::EBADF));
    }

    Ok(())
}

pub fn utimens(file: &Path, atime: TimeSpec, mtime: TimeSpec) -> io::Result<()> {
    fdutimens(None, Some(file), atime, mtime)
}

pub fn lutimens(file: &Path, atime: TimeSpec, mtime: TimeSpec) -> io::Result<()> {
    let mut ts = [atime.to_raw(), mtime.to_raw()];
    validate_timespec(&mut ts)?;

    let st = file.symlink_metadata()?;
    let st_atim = st.accessed().unwrap_or(UNIX_EPOCH);
    let st_mtim = st.modified().unwrap_or(UNIX_EPOCH);

    if let TimeSpec::Omit = atime {
        ts[0] = timespec {
            tv_sec: st_atim.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            tv_nsec: st_atim.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
        };
    }

    if let TimeSpec::Omit = mtime {
        ts[1] = timespec {
            tv_sec: st_mtim.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            tv_nsec: st_mtim.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
        };
    }

    unsafe {
        let c_path = std::ffi::CString::new(file.as_os_str().as_bytes()).unwrap();
        if libc::utimensat(
            libc::AT_FDCWD,
            c_path.as_ptr(),
            ts.as_ptr(),
            AT_SYMLINK_NOFOLLOW,
        ) == -1
        {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

fn validate_timespec(ts: &mut [timespec; 2]) -> io::Result<()> {
    for i in 0..2 {
        if ts[i].tv_nsec != UTIME_NOW
            && ts[i].tv_nsec != UTIME_OMIT
            && (ts[i].tv_nsec < 0 || ts[i].tv_nsec >= TIMESPEC_HZ)
        {
            return Err(io::Error::from_raw_os_error(libc::EINVAL));
        }
    }
    Ok(())
}