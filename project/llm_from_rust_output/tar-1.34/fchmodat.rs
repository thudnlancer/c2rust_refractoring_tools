use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::io::{self, Error, ErrorKind};
use libc::{mode_t, c_int};

#[derive(Debug)]
struct Stat {
    st_mode: mode_t,
    // Other fields omitted as they're not used in the logic
}

fn fstat(fd: i32) -> io::Result<Stat> {
    let mut stat_buf = unsafe { std::mem::zeroed() };
    let res = unsafe { libc::fstat(fd, &mut stat_buf) };
    if res == 0 {
        Ok(Stat {
            st_mode: stat_buf.st_mode,
        })
    } else {
        Err(Error::last_os_error())
    }
}

fn rpl_fchmodat(dir: Option<&File>, file: &Path, mode: mode_t, flags: c_int) -> io::Result<()> {
    const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
    
    if flags == AT_SYMLINK_NOFOLLOW {
        let fd = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_PATH | libc::O_NOFOLLOW)
            .open(file)?;
        
        let stat = fstat(fd.as_raw_fd())?;
        
        if (stat.st_mode & libc::S_IFMT) == libc::S_IFLNK {
            return Err(Error::new(ErrorKind::Other, "operation not supported on symlink"));
        }
        
        let proc_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
        std::fs::set_permissions(proc_path, std::fs::Permissions::from_mode(mode))
    } else {
        let dir_fd = dir.map_or(libc::AT_FDCWD, |f| f.as_raw_fd());
        let res = unsafe { libc::fchmodat(dir_fd, file.as_os_str().as_ptr(), mode, flags) };
        if res == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}