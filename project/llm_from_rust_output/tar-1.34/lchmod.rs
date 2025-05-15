use std::fs::{File, OpenOptions, Metadata};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::io;
use libc::{mode_t, c_int};

pub fn lchmod(file: &Path, mode: mode_t) -> io::Result<()> {
    // Open file with O_PATH | O_NOFOLLOW | O_CLOEXEC flags
    let fd = OpenOptions::new()
        .custom_flags(libc::O_PATH | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .read(true)
        .open(file)?;
    
    // Get file metadata using fstatat with AT_EMPTY_PATH flag
    let metadata = {
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        let res = unsafe {
            libc::fstatat(
                fd.as_raw_fd(),
                b"\0".as_ptr() as *const libc::c_char,
                stat.as_mut_ptr(),
                libc::AT_EMPTY_PATH,
            )
        };
        if res != 0 {
            let err = io::Error::last_os_error();
            return Err(err);
        }
        unsafe { stat.assume_init() }
    };

    // Check if file is a symlink
    if (metadata.st_mode & libc::S_IFMT) == libc::S_IFLNK {
        return Err(io::Error::from_raw_os_error(95)); // ENOTSUP
    }

    // Try changing permissions through /proc/self/fd
    let proc_path = format!("/proc/self/fd/{}", fd.as_raw_fd());
    match std::fs::set_permissions(&proc_path, std::fs::Permissions::from_mode(mode)) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.raw_os_error() == Some(libc::ENOENT) {
                // Fall back to regular chmod if /proc doesn't exist
                std::fs::set_permissions(file, std::fs::Permissions::from_mode(mode))
            } else {
                Err(e)
            }
        }
    }
}