use std::os::unix::fs::{FileTimeExt, MetadataExt};
use std::fs::{File, OpenOptions, metadata, symlink_metadata};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_int, c_char, timespec, timeval, stat};
use std::ptr;

const TIMESPEC_HZ: u32 = 1_000_000_000;
const UTIME_NOW: i64 = ((1 << 30) - 1);
const UTIME_OMIT: i64 = ((1 << 30) - 2);

#[derive(Debug)]
enum TimeSpec {
    Now,
    Omit,
    Value(timespec),
}

fn validate_timespec(ts: &[timespec; 2]) -> Result<(bool, bool), i32> {
    let mut utime_omit_count = 0;
    let mut needs_adjustment = false;

    for t in ts {
        if t.tv_nsec != UTIME_NOW && t.tv_nsec != UTIME_OMIT && 
           !(0 <= t.tv_nsec && (t.tv_nsec as u32) < TIMESPEC_HZ) {
            return Err(libc::EINVAL);
        }

        if t.tv_nsec == UTIME_NOW || t.tv_nsec == UTIME_OMIT {
            needs_adjustment = true;
            if t.tv_nsec == UTIME_OMIT {
                utime_omit_count += 1;
            }
        }
    }

    Ok((needs_adjustment, utime_omit_count == 1))
}

fn update_timespec(stat: &stat, ts: &mut [timespec; 2]) -> bool {
    if ts[0].tv_nsec == UTIME_OMIT && ts[1].tv_nsec == UTIME_OMIT {
        return true;
    }

    if ts[0].tv_nsec == UTIME_NOW && ts[1].tv_nsec == UTIME_NOW {
        return false;
    }

    if ts[0].tv_nsec == UTIME_OMIT {
        ts[0] = stat.st_atim;
    } else if ts[0].tv_nsec == UTIME_NOW {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[0].tv_sec = now.as_secs() as i64;
        ts[0].tv_nsec = now.subsec_nanos() as i64;
    }

    if ts[1].tv_nsec == UTIME_OMIT {
        ts[1] = stat.st_mtim;
    } else if ts[1].tv_nsec == UTIME_NOW {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        ts[1].tv_sec = now.as_secs() as i64;
        ts[1].tv_nsec = now.subsec_nanos() as i64;
    }

    false
}

fn fdutimens(fd: Option<&File>, path: Option<&Path>, times: Option<&[timespec; 2]>) -> Result<(), i32> {
    let mut adjusted_times = match times {
        Some(ts) => {
            let mut adjusted = *ts;
            let (needs_adj, has_omit) = validate_timespec(&adjusted)?;
            
            if needs_adj {
                let stat = if let Some(fd) = fd {
                    let metadata = fd.metadata().map_err(|_| libc::EBADF)?;
                    metadata_to_stat(&metadata)
                } else if let Some(p) = path {
                    let metadata = metadata(p).map_err(|_| libc::ENOENT)?;
                    metadata_to_stat(&metadata)
                } else {
                    return Err(libc::EBADF);
                };

                if has_omit {
                    if adjusted[0].tv_nsec == UTIME_OMIT {
                        adjusted[0] = stat.st_atim;
                    }
                    if adjusted[1].tv_nsec == UTIME_OMIT {
                        adjusted[1] = stat.st_mtim;
                    }
                }

                if update_timespec(&stat, &mut adjusted) {
                    return Ok(());
                }
            }
            Some(adjusted)
        }
        None => None,
    };

    // Try utimensat first
    unsafe {
        let res = if let Some(p) = path {
            libc::utimensat(
                libc::AT_FDCWD,
                p.as_os_str().as_bytes().as_ptr() as *const c_char,
                adjusted_times.as_ref().map(|t| t.as_ptr()).unwrap_or(ptr::null()),
                0,
            )
        } else if let Some(f) = fd {
            libc::futimens(f.as_raw_fd(), adjusted_times.as_ref().map(|t| t.as_ptr()).unwrap_or(ptr::null()))
        } else {
            return Err(libc::EBADF);
        };

        if res == 0 {
            return Ok(());
        }
    }

    // Fallback to futimes/utimes
    let mut timevals = match adjusted_times {
        Some(ts) => {
            let mut tvs = [
                timeval { tv_sec: ts[0].tv_sec, tv_usec: ts[0].tv_nsec / 1000 },
                timeval { tv_sec: ts[1].tv_sec, tv_usec: ts[1].tv_nsec / 1000 },
            ];
            Some(tvs)
        }
        None => None,
    };

    unsafe {
        let res = if let Some(p) = path {
            libc::utimes(
                p.as_os_str().as_bytes().as_ptr() as *const c_char,
                timevals.as_ref().map(|t| t.as_ptr()).unwrap_or(ptr::null()),
            )
        } else if let Some(f) = fd {
            libc::futimes(f.as_raw_fd(), timevals.as_ref().map(|t| t.as_ptr()).unwrap_or(ptr::null()))
        } else {
            return Err(libc::EBADF);
        };

        if res == 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(libc::EINVAL))
        }
    }
}

pub fn utimens(path: &Path, times: Option<&[timespec; 2]>) -> Result<(), i32> {
    fdutimens(None, Some(path), times)
}

pub fn lutimens(path: &Path, times: Option<&[timespec; 2]>) -> Result<(), i32> {
    let metadata = symlink_metadata(path).map_err(|_| libc::ENOENT)?;
    if metadata.file_type().is_symlink() {
        return Err(libc::ENOSYS);
    }
    fdutimens(None, Some(path), times)
}

fn metadata_to_stat(metadata: &std::fs::Metadata) -> stat {
    stat {
        st_dev: metadata.dev() as u64,
        st_ino: metadata.ino() as u64,
        st_nlink: metadata.nlink() as u64,
        st_mode: metadata.mode() as u32,
        st_uid: metadata.uid(),
        st_gid: metadata.gid(),
        __pad0: 0,
        st_rdev: metadata.rdev() as u64,
        st_size: metadata.size() as i64,
        st_blksize: metadata.blksize() as i64,
        st_blocks: metadata.blocks() as i64,
        st_atim: timespec {
            tv_sec: metadata.atime() as i64,
            tv_nsec: metadata.atime_nsec() as i64,
        },
        st_mtim: timespec {
            tv_sec: metadata.mtime() as i64,
            tv_nsec: metadata.mtime_nsec() as i64,
        },
        st_ctim: timespec {
            tv_sec: metadata.ctime() as i64,
            tv_nsec: metadata.ctime_nsec() as i64,
        },
        __glibc_reserved: [0; 3],
    }
}