use std::os::unix::fs::{FileTime, Utimensat};
use std::path::Path;
use std::fs::{File, OpenOptions, Metadata};
use std::io::{self, Error, ErrorKind};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, timespec, UTIME_NOW, UTIME_OMIT};
use std::os::unix::prelude::*;

const TIMESPEC_HZ: i64 = 1_000_000_000;

#[derive(Debug)]
struct TimeSpec {
    tv_sec: i64,
    tv_nsec: i64,
}

impl TimeSpec {
    fn new(sec: i64, nsec: i64) -> Self {
        TimeSpec { tv_sec: sec, tv_nsec: nsec }
    }

    fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        TimeSpec {
            tv_sec: now.as_secs() as i64,
            tv_nsec: now.subsec_nanos() as i64,
        }
    }
}

fn validate_timespec(timespec: &[TimeSpec; 2]) -> Result<i32, Error> {
    let mut result = 0;
    let mut utime_omit_count = 0;

    for ts in timespec.iter() {
        if ts.tv_nsec != UTIME_NOW as i64 && 
           ts.tv_nsec != UTIME_OMIT as i64 && 
           !(0 <= ts.tv_nsec && ts.tv_nsec < TIMESPEC_HZ) {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid timespec value"));
        }
    }

    if timespec[0].tv_nsec == UTIME_NOW as i64 || timespec[0].tv_nsec == UTIME_OMIT as i64 {
        result = 1;
        if timespec[0].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    if timespec[1].tv_nsec == UTIME_NOW as i64 || timespec[1].tv_nsec == UTIME_OMIT as i64 {
        result = 1;
        if timespec[1].tv_nsec == UTIME_OMIT as i64 {
            utime_omit_count += 1;
        }
    }

    Ok(result + if utime_omit_count == 1 { 1 } else { 0 })
}

fn update_timespec(statbuf: &Metadata, ts: &mut [TimeSpec; 2]) -> bool {
    if ts[0].tv_nsec == UTIME_OMIT as i64 && ts[1].tv_nsec == UTIME_OMIT as i64 {
        return true;
    }

    if ts[0].tv_nsec == UTIME_NOW as i64 && ts[1].tv_nsec == UTIME_NOW as i64 {
        return false;
    }

    if ts[0].tv_nsec == UTIME_OMIT as i64 {
        let atime = statbuf.accessed().unwrap();
        ts[0] = TimeSpec::new(
            atime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            atime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
        );
    } else if ts[0].tv_nsec == UTIME_NOW as i64 {
        ts[0] = TimeSpec::now();
    }

    if ts[1].tv_nsec == UTIME_OMIT as i64 {
        let mtime = statbuf.modified().unwrap();
        ts[1] = TimeSpec::new(
            mtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            mtime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
        );
    } else if ts[1].tv_nsec == UTIME_NOW as i64 {
        ts[1] = TimeSpec::now();
    }

    false
}

pub fn fdutimens(fd: Option<&File>, file: Option<&Path>, timespec: Option<[TimeSpec; 2]>) -> io::Result<()> {
    let mut adjusted_timespec = match ×pec {
        Some(ts) => ts,
        None => return set_current_time(fd, file),
    };

    let adjustment_needed = validate_timespec(&adjusted_timespec)?;

    if fd.is_none() && file.is_none() {
        return Err(Error::new(ErrorKind::InvalidInput, "Either fd or file must be specified"));
    }

    if adjustment_needed == 2 {
        let metadata = if let Some(f) = fd {
            f.metadata()?
        } else {
            file.unwrap().metadata()?
        };

        if adjusted_timespec[0].tv_nsec == UTIME_OMIT as i64 {
            let atime = metadata.accessed()?;
            adjusted_timespec[0] = TimeSpec::new(
                atime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                atime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            );
        } else if adjusted_timespec[1].tv_nsec == UTIME_OMIT as i64 {
            let mtime = metadata.modified()?;
            adjusted_timespec[1] = TimeSpec::new(
                mtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                mtime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            );
        }
    }

    if let Some(f) = fd {
        set_file_times(f, &adjusted_timespec)
    } else {
        set_file_times_at(file.unwrap(), &adjusted_timespec)
    }
}

fn set_current_time(fd: Option<&File>, file: Option<&Path>) -> io::Result<()> {
    let now = TimeSpec::now();
    let times = [now.clone(), now];
    
    if let Some(f) = fd {
        set_file_times(f, &times)
    } else {
        set_file_times_at(file.unwrap(), &times)
    }
}

fn set_file_times(file: &File, times: &[TimeSpec; 2]) -> io::Result<()> {
    let atime = FileTime::from_unix_time(times[0].tv_sec, times[0].tv_nsec as u32);
    let mtime = FileTime::from_unix_time(times[1].tv_sec, times[1].tv_nsec as u32);
    file.set_times(atime, mtime)
}

fn set_file_times_at(path: &Path, times: &[TimeSpec; 2]) -> io::Result<()> {
    let atime = FileTime::from_unix_time(times[0].tv_sec, times[0].tv_nsec as u32);
    let mtime = FileTime::from_unix_time(times[1].tv_sec, times[1].tv_nsec as u32);
    utimensat(None, path, Some((atime, mtime)), 0)
}

pub fn utimens(file: &Path, timespec: Option<[TimeSpec; 2]>) -> io::Result<()> {
    fdutimens(None, Some(file), timespec)
}

pub fn lutimens(file: &Path, timespec: Option<[TimeSpec; 2]>) -> io::Result<()> {
    let metadata = file.symlink_metadata()?;
    if metadata.file_type().is_symlink() {
        let mut adjusted_timespec = match ×pec {
            Some(ts) => ts,
            None => [TimeSpec::now(), TimeSpec::now()],
        };

        validate_timespec(&adjusted_timespec)?;
        
        if adjusted_timespec[0].tv_nsec == UTIME_OMIT as i64 {
            let atime = metadata.accessed()?;
            adjusted_timespec[0] = TimeSpec::new(
                atime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                atime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            );
        }

        if adjusted_timespec[1].tv_nsec == UTIME_OMIT as i64 {
            let mtime = metadata.modified()?;
            adjusted_timespec[1] = TimeSpec::new(
                mtime.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                mtime.duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as i64,
            );
        }

        utimensat(None, file, Some((
            FileTime::from_unix_time(adjusted_timespec[0].tv_sec, adjusted_timespec[0].tv_nsec as u32),
            FileTime::from_unix_time(adjusted_timespec[1].tv_sec, adjusted_timespec[1].tv_nsec as u32),
        )), libc::AT_SYMLINK_NOFOLLOW)
    } else {
        fdutimens(None, Some(file), timespec)
    }
}