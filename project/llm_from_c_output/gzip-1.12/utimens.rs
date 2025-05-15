use std::os::unix::fs::{FileTimesExt, MetadataExt};
use std::fs::{File, OpenOptions, metadata, symlink_metadata};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{timespec, AT_FDCWD, AT_SYMLINK_NOFOLLOW};
use std::io;
use std::ffi::CString;

const UTIME_NOW: i64 = 1_073_741_823;
const UTIME_OMIT: i64 = 1_073_741_822;
const TIMESPEC_HZ: i64 = 1_000_000_000;

static mut UTIMENSAT_WORKS_REALLY: i32 = 0;
static mut LUTIMENSAT_WORKS_REALLY: i32 = 0;

fn validate_timespec(timespec: &[timespec; 2]) -> Result<i32, io::Error> {
    let mut result = 0;
    let mut utime_omit_count = 0;

    for ts in timespec {
        if ts.tv_nsec != UTIME_NOW && ts.tv_nsec != UTIME_OMIT && 
           !(0 <= ts.tv_nsec && ts.tv_nsec < TIMESPEC_HZ) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid timespec"));
        }
    }

    if timespec[0].tv_nsec == UTIME_NOW || timespec[0].tv_nsec == UTIME_OMIT {
        result = 1;
        if timespec[0].tv_nsec == UTIME_OMIT {
            utime_omit_count += 1;
        }
    }

    if timespec[1].tv_nsec == UTIME_NOW || timespec[1].tv_nsec == UTIME_OMIT {
        result = 1;
        if timespec[1].tv_nsec == UTIME_OMIT {
            utime_omit_count += 1;
        }
    }

    Ok(result + (utime_omit_count == 1) as i32)
}

fn update_timespec(statbuf: &std::fs::Metadata, ts: &mut Option<[timespec; 2]>) -> bool {
    if let Some(timespec) = ts {
        if timespec[0].tv_nsec == UTIME_OMIT && timespec[1].tv_nsec == UTIME_OMIT {
            return true;
        }

        if timespec[0].tv_nsec == UTIME_NOW && timespec[1].tv_nsec == UTIME_NOW {
            *ts = None;
            return false;
        }

        if timespec[0].tv_nsec == UTIME_OMIT {
            timespec[0].tv_sec = statbuf.atime();
            timespec[0].tv_nsec = statbuf.atime_nsec() as i64;
        } else if timespec[0].tv_nsec == UTIME_NOW {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();
            timespec[0].tv_sec = now.as_secs() as i64;
            timespec[0].tv_nsec = now.subsec_nanos() as i64;
        }

        if timespec[1].tv_nsec == UTIME_OMIT {
            timespec[1].tv_sec = statbuf.mtime();
            timespec[1].tv_nsec = statbuf.mtime_nsec() as i64;
        } else if timespec[1].tv_nsec == UTIME_NOW {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap();
            timespec[1].tv_sec = now.as_secs() as i64;
            timespec[1].tv_nsec = now.subsec_nanos() as i64;
        }
    }
    false
}

pub fn fdutimens(fd: Option<&File>, file: Option<&Path>, timespec: Option<[timespec; 2]>) -> io::Result<()> {
    let mut adjusted_timespec = timespec;
    let mut adjustment_needed = 0;

    if let Some(ts) = &adjusted_timespec {
        adjustment_needed = validate_timespec(ts)?;
    }

    if fd.is_none() && file.is_none() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Neither fd nor file specified"));
    }

    // Try utimensat/futimens first
    unsafe {
        if UTIMENSAT_WORKS_REALLY >= 0 {
            if let Some(ts) = &adjusted_timespec {
                if adjustment_needed == 2 {
                    let st = if let Some(f) = fd {
                        f.metadata()?
                    } else if let Some(p) = file {
                        metadata(p)?
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No file specified"));
                    };

                    if ts[0].tv_nsec == UTIME_OMIT {
                        adjusted_timespec.as_mut().unwrap()[0].tv_sec = st.atime();
                        adjusted_timespec.as_mut().unwrap()[0].tv_nsec = st.atime_nsec() as i64;
                    } else if ts[1].tv_nsec == UTIME_OMIT {
                        adjusted_timespec.as_mut().unwrap()[1].tv_sec = st.mtime();
                        adjusted_timespec.as_mut().unwrap()[1].tv_nsec = st.mtime_nsec() as i64;
                    }
                    adjustment_needed += 1;
                }
            }

            let result = if let Some(f) = fd {
                libc::futimens(f.as_raw_fd(), adjusted_timespec.as_ref().map_or(std::ptr::null(), |t| t.as_ptr()))
            } else if let Some(p) = file {
                let c_path = CString::new(p.as_os_str().as_bytes()).unwrap();
                libc::utimensat(AT_FDCWD, c_path.as_ptr(), adjusted_timespec.as_ref().map_or(std::ptr::null(), |t| t.as_ptr()), 0)
            } else {
                -1
            };

            if result == 0 {
                UTIMENSAT_WORKS_REALLY = 1;
                return Ok(());
            } else if io::Error::last_os_error().raw_os_error() != Some(libc::ENOSYS) {
                return Err(io::Error::last_os_error());
            }
        }
        UTIMENSAT_WORKS_REALLY = -1;
    }

    // Fallback to older APIs
    if adjustment_needed != 0 || (file.is_some() && fd.is_none()) {
        let st = if adjustment_needed != 3 {
            if let Some(f) = fd {
                f.metadata()?
            } else if let Some(p) = file {
                metadata(p)?
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "No file specified"));
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid adjustment"));
        };

        if update_timespec(&st, &mut adjusted_timespec) {
            return Ok(());
        }
    }

    // Final fallback to utimes/utime
    if let Some(ts) = adjusted_timespec {
        let atime = SystemTime::UNIX_EPOCH + std::time::Duration::new(ts[0].tv_sec as u64, ts[0].tv_nsec as u32);
        let mtime = SystemTime::UNIX_EPOCH + std::time::Duration::new(ts[1].tv_sec as u64, ts[1].tv_nsec as u32);
        
        if let Some(f) = fd {
            f.set_times(atime, mtime)?;
        } else if let Some(p) = file {
            let file = OpenOptions::new().write(true).open(p)?;
            file.set_times(atime, mtime)?;
        }
    } else {
        let now = SystemTime::now();
        if let Some(f) = fd {
            f.set_times(now, now)?;
        } else if let Some(p) = file {
            let file = OpenOptions::new().write(true).open(p)?;
            file.set_times(now, now)?;
        }
    }

    Ok(())
}

pub fn utimens(file: &Path, timespec: Option<[timespec; 2]>) -> io::Result<()> {
    fdutimens(None, Some(file), timespec)
}

pub fn lutimens(file: &Path, timespec: Option<[timespec; 2]>) -> io::Result<()> {
    let mut adjusted_timespec = timespec;
    let mut adjustment_needed = 0;

    if let Some(ts) = &adjusted_timespec {
        adjustment_needed = validate_timespec(ts)?;
    }

    unsafe {
        if LUTIMENSAT_WORKS_REALLY >= 0 {
            if let Some(ts) = &adjusted_timespec {
                if adjustment_needed == 2 {
                    let st = symlink_metadata(file)?;
                    if ts[0].tv_nsec == UTIME_OMIT {
                        adjusted_timespec.as_mut().unwrap()[0].tv_sec = st.atime();
                        adjusted_timespec.as_mut().unwrap()[0].tv_nsec = st.atime_nsec() as i64;
                    } else if ts[1].tv_nsec == UTIME_OMIT {
                        adjusted_timespec.as_mut().unwrap()[1].tv_sec = st.mtime();
                        adjusted_timespec.as_mut().unwrap()[1].tv_nsec = st.mtime_nsec() as i64;
                    }
                    adjustment_needed += 1;
                }
            }

            let c_path = CString::new(file.as_os_str().as_bytes()).unwrap();
            let result = libc::utimensat(
                AT_FDCWD,
                c_path.as_ptr(),
                adjusted_timespec.as_ref().map_or(std::ptr::null(), |t| t.as_ptr()),
                AT_SYMLINK_NOFOLLOW,
            );

            if result == 0 {
                UTIMENSAT_WORKS_REALLY = 1;
                LUTIMENSAT_WORKS_REALLY = 1;
                return Ok(());
            } else if io::Error::last_os_error().raw_os_error() != Some(libc::ENOSYS) {
                return Err(io::Error::last_os_error());
            }
        }
        LUTIMENSAT_WORKS_REALLY = -1;
    }

    let st = symlink_metadata(file)?;
    if !st.file_type().is_symlink() {
        return fdutimens(None, Some(file), adjusted_timespec);
    }

    Err(io::Error::new(io::ErrorKind::Unsupported, "Setting timestamps on symlinks not supported"))
}