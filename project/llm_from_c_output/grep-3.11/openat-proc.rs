use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};
use std::ffi::CString;
use libc::{F_OK, O_CLOEXEC, O_DIRECTORY, O_NOCTTY, O_NONBLOCK};

const OPENAT_BUFFER_SIZE: usize = 1024; // Adjust as needed

/// Create /proc/self/fd-related names for subfiles of open directories.
/// 
/// Returns a PathBuf containing the constructed path. If the path fits in the provided buffer,
/// it will be used. Otherwise, a new buffer will be allocated.
/// 
/// # Errors
/// Returns an io::Error if the operation fails.
pub fn openat_proc_name(buf: Option<PathBuf>, fd: i32, file: &str) -> io::Result<PathBuf> {
    // Make sure the caller gets ENOENT when appropriate
    if file.is_empty() {
        return Ok(PathBuf::new());
    }

    #[cfg(not(any(target_os = "os2", target_os = "zos"))] {
        // Generic code for Linux, Solaris, and similar platforms
        const PROC_SELF_FD_FORMAT: &str = "/proc/self/fd/";
        
        // Check if /proc/self/fd is reliable
        static PROC_STATUS: std::sync::OnceLock<io::Result<bool>> = std::sync::OnceLock::new();
        let proc_status = PROC_STATUS.get_or_init(|| {
            let proc_self_fd = OpenOptions::new()
                .read(true)
                .custom_flags(O_DIRECTORY | O_NOCTTY | O_NONBLOCK | O_CLOEXEC)
                .open("/proc/self/fd")?;

            // Detect whether /proc/self/fd/%i/../fd exists
            let fd_num = proc_self_fd.as_raw_fd();
            let dotdot_path = format!("{}{}/../fd", PROC_SELF_FD_FORMAT, fd_num);
            let exists = Path::new(&dotdot_path).exists();
            Ok(exists)
        });

        match proc_status {
            Ok(true) => (),
            Ok(false) => return Err(io::Error::new(io::ErrorKind::Other, "/proc/self/fd not reliable")),
            Err(e) => return Err(io::Error::new(e.kind(), e.to_string())),
        }

        let dir_part = format!("{}{}/", PROC_SELF_FD_FORMAT, fd);
        let full_path = PathBuf::from(dir_part).join(file);

        if let Some(mut buf) = buf {
            if buf.capacity() >= full_path.as_os_str().len() {
                buf.clear();
                buf.push(&full_path);
                Ok(buf)
            } else {
                Ok(full_path)
            }
        } else {
            Ok(full_path)
        }
    }

    #[cfg(any(target_os = "os2", target_os = "zos"))] {
        // OS/2 or z/OS specific implementation
        let dir = if cfg!(target_os = "os2") {
            // OS/2 kLIBC implementation
            let mut dir_buf = [0u8; _MAX_PATH as usize];
            if unsafe { __libc_Back_ioFHToPath(fd, dir_buf.as_mut_ptr() as *mut _, dir_buf.len()) } != 0 {
                return Err(io::Error::last_os_error());
            }
            CString::new(dir_buf).unwrap().into_string().unwrap()
        } else {
            // z/OS implementation
            let mut dir_buf = [0u8; _XOPEN_PATH_MAX as usize];
            if unsafe { w_ioctl(fd, _IOCC_GPN, dir_buf.len(), dir_buf.as_mut_ptr() as *mut _) } < 0 {
                return Err(io::Error::last_os_error());
            }
            let dirlen = unsafe { __e2a_l(dir_buf.as_ptr() as *const _, dir_buf.len()) };
            if dirlen < 0 || dirlen >= dir_buf.len() as isize {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "E2A conversion failed"));
            }
            CString::new(&dir_buf[..dirlen as usize]).unwrap().into_string().unwrap()
        };

        let full_path = PathBuf::from(dir).join(file);

        if let Some(mut buf) = buf {
            if buf.capacity() >= full_path.as_os_str().len() {
                buf.clear();
                buf.push(&full_path);
                Ok(buf)
            } else {
                Ok(full_path)
            }
        } else {
            Ok(full_path)
        }
    }
}