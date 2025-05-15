use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, RawFd};
use std::io;

#[derive(Debug)]
struct CdBuffer {
    fd: Option<File>,
}

impl CdBuffer {
    fn new() -> Self {
        CdBuffer { fd: None }
    }

    fn fchdir(&self) -> io::Result<()> {
        if let Some(file) = &self.fd {
            let res = unsafe { libc::fchdir(file.as_raw_fd()) };
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No directory fd"))
        }
    }

    fn advance_fd(&mut self, dir: &Path) -> io::Result<()> {
        let flags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW;
        let parent_fd = self.fd.as_ref().map(|f| f.as_raw_fd()).unwrap_or(libc::AT_FDCWD);
        
        let c_path = CString::new(dir.as_os_str().as_bytes()).unwrap();
        let fd = unsafe { libc::openat(parent_fd, c_path.as_ptr(), flags) };
        
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        
        // Close previous fd if it exists
        if let Some(old_file) = self.fd.take() {
            drop(old_file); // File's Drop impl will close the fd
        }
        
        self.fd = Some(unsafe { File::from_raw_fd(fd) });
        Ok(())
    }
}

impl Drop for CdBuffer {
    fn drop(&mut self) {
        if let Some(file) = self.fd.take() {
            drop(file); // File's Drop impl will close the fd
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
    if std::env::set_current_dir(dir).is_ok() {
        return Ok(());
    }
    
    let err = io::Error::last_os_error();
    if err.raw_os_error() != Some(libc::ENAMETOOLONG) {
        return Err(err);
    }
    
    let dir_str = dir.as_os_str().as_bytes();
    if dir_str.len() < 4096 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path length should be at least 4096 for long directory handling",
        ));
    }
    
    let mut cdb = CdBuffer::new();
    let mut remaining_path = dir_str;
    
    // Handle leading slashes
    let leading_slashes = remaining_path.iter().take_while(|&&c| c == b'/').count();
    
    if leading_slashes == 2 {
        // Handle network paths like //host/path
        if let Some(slash_pos) = remaining_path[3..].iter().position(|&c| c == b'/') {
            let host_end = 3 + slash_pos;
            let host_path = Path::new(OsStr::from_bytes(&remaining_path[..host_end]));
            
            cdb.advance_fd(host_path)?;
            remaining_path = find_non_slash(&remaining_path[host_end + 1..]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid network path format",
            ));
        }
    } else if leading_slashes > 0 {
        cdb.advance_fd(Path::new("/"))?;
        remaining_path = find_non_slash(remaining_path);
    }
    
    while remaining_path.len() >= 4096 {
        let search_slice = &remaining_path[..4096];
        if let Some(slash_pos) = search_slice.iter().rposition(|&c| c == b'/') {
            let partial_path = Path::new(OsStr::from_bytes(&remaining_path[..slash_pos]));
            cdb.advance_fd(partial_path)?;
            remaining_path = find_non_slash(&remaining_path[slash_pos + 1..]);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Path component too long",
            ));
        }
    }
    
    if !remaining_path.is_empty() {
        cdb.advance_fd(Path::new(OsStr::from_bytes(remaining_path)))?;
    }
    
    cdb.fchdir()
}