use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::{env, fs, io};

const PATH_MAX: usize = 4096;

struct CdBuffer {
    fd: Option<fs::File>,
}

impl CdBuffer {
    fn new() -> Self {
        CdBuffer { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(fd) = &self.fd {
            let res = unsafe { libc::fchdir(fd.as_raw_fd()) };
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No file descriptor",
            ))
        }
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let c_dir = CString::new(dir.as_os_str().as_bytes())?;
        let fd = unsafe {
            libc::openat(
                self.fd.as_ref().map_or(-1, |f| f.as_raw_fd()),
                c_dir.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW,
            )
        };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        self.fd = Some(unsafe { fs::File::from_raw_fd(fd) });
        Ok(())
    }
}

impl Drop for CdBuffer {
    fn drop(&mut self) {
        if let Some(fd) = self.fd.take() {
            let _ = unsafe { libc::close(fd.as_raw_fd()) };
        }
    }
}

fn find_non_slash(s: &[u8]) -> &[u8] {
    s.iter()
        .position(|&c| c != b'/')
        .map(|pos| &s[pos..])
        .unwrap_or(&[])
}

pub fn chdir_long(dir: &Path) -> io::Result<()> {
    // First try normal chdir
    if env::set_current_dir(dir).is_ok() {
        return Ok(());
    }

    let err = io::Error::last_os_error();
    if err.raw_os_error() != Some(libc::ENAMETOOLONG) {
        return Err(err);
    }

    let dir_str = dir.as_os_str().as_bytes();
    if dir_str.len() <= PATH_MAX {
        return Err(err);
    }

    let mut cdb = CdBuffer::new();
    let n_leading_slash = dir_str.iter().take_while(|&&c| c == b'/').count();

    if n_leading_slash == 2 {
        // Handle // prefix case
        let after_prefix = &dir_str[2..];
        if let Some(slash_pos) = after_prefix.iter().position(|&c| c == b'/') {
            let component = &dir_str[..2 + slash_pos];
            let remaining = &dir_str[2 + slash_pos + 1..];
            
            cdb.advance_fd(Path::new(std::str::from_utf8(component).unwrap()))?;
            chdir_long_remaining(&mut cdb, remaining)?;
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"));
        }
    } else if n_leading_slash > 0 {
        cdb.advance_fd(Path::new("/"))?;
        chdir_long_remaining(&mut cdb, &dir_str[n_leading_slash..])?;
    } else {
        chdir_long_remaining(&mut cdb, dir_str)?;
    }

    cdb.fchdir()
}

fn chdir_long_remaining(cdb: &mut CdBuffer, remaining: &[u8]) -> io::Result<()> {
    let mut current = remaining;
    
    while current.len() > PATH_MAX {
        let search_window = &current[..PATH_MAX];
        let slash_pos = search_window.iter().rposition(|&c| c == b'/')
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Path component too long"))?;
            
        let component = &current[..slash_pos];
        current = find_non_slash(&current[slash_pos + 1..]);
        
        cdb.advance_fd(Path::new(std::str::from_utf8(component).unwrap()))?;
    }

    if !current.is_empty() {
        cdb.advance_fd(Path::new(std::str::from_utf8(current).unwrap()))?;
    }

    Ok(())
}