use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::{io, mem};

const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
const O_PATH: i32 = 0x2000000;
const O_NOFOLLOW: i32 = 0x100;
const O_CLOEXEC: i32 = 0x80000;
const AT_EMPTY_PATH: i32 = 0x1000;

#[cfg(target_os = "linux")]
fn is_proc_mounted() -> bool {
    Path::new("/proc/self").exists()
}

#[cfg(not(target_os = "linux"))]
fn is_proc_mounted() -> bool {
    false
}

fn fchmodat(dir: &File, file: &Path, mode: u32, flags: i32) -> io::Result<()> {
    #[cfg(any(target_os = "linux", target_os = "android", target_os = "cygwin"))]
    {
        if flags == AT_SYMLINK_NOFOLLOW {
            let fd = OpenOptions::new()
                .custom_flags(O_PATH | O_NOFOLLOW | O_CLOEXEC)
                .open(file)?;

            let st = {
                let mut stat = unsafe { mem::zeroed() };
                let res = unsafe {
                    libc::fstatat(
                        fd.as_raw_fd(),
                        b"\0".as_ptr() as *const libc::c_char,
                        &mut stat,
                        AT_EMPTY_PATH,
                    )
                };
                if res != 0 {
                    let err = io::Error::last_os_error();
                    drop(fd);
                    return Err(err);
                }
                stat
            };

            if (st.st_mode & libc::S_IFMT) == libc::S_IFLNK {
                return Err(io::Error::from_raw_os_error(libc::EOPNOTSUPP));
            }

            if is_proc_mounted() {
                let path = format!("/proc/self/fd/{}", fd.as_raw_fd());
                match std::fs::set_permissions(path, PermissionsExt::from_mode(mode)) {
                    Ok(_) => return Ok(()),
                    Err(e) if e.kind() != io::ErrorKind::NotFound => return Err(e),
                    _ => (),
                }
            }

            // Fall back without flags if /proc isn't mounted
            return fchmodat(dir, file, mode, 0);
        }
    }

    let res = unsafe {
        libc::fchmodat(
            dir.as_raw_fd(),
            file.as_os_str().as_bytes().as_ptr() as *const libc::c_char,
            mode,
            flags,
        )
    };
    if res != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}