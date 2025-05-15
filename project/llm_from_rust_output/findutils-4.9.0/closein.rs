use std::ffi::CStr;
use std::io::{self, SeekFrom, Seek};
use std::process;
use std::ptr;
use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};

static mut FILE_NAME: Option<&'static CStr> = None;

pub fn close_stdin_set_file_name(file: &'static CStr) {
    unsafe {
        FILE_NAME = Some(file);
    }
}

pub fn close_stdin() -> io::Result<()> {
    let stdin = unsafe { File::from_raw_fd(0) };
    
    // Check if there's any unread data
    let mut buf = [0u8; 1];
    let unread_data = stdin.try_clone()?.bytes().next().is_some();
    
    if unread_data {
        // Seek to current position and flush
        stdin.try_clone()?.seek(SeekFrom::Current(0))?;
        stdin.try_clone()?.sync_all()?;
    }
    
    // Close stdin
    drop(stdin);
    
    // Handle errors
    if let Some(file_name) = unsafe { FILE_NAME } {
        let err = io::Error::last_os_error();
        if err.kind() != io::ErrorKind::WouldBlock {
            eprintln!("{}: {}", file_name.to_string_lossy(), err);
            process::exit(1);
        }
    } else {
        let err = io::Error::last_os_error();
        if err.kind() != io::ErrorKind::WouldBlock {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    
    Ok(())
}

pub fn close_stdout() {
    if let Err(e) = io::stdout().flush() {
        eprintln!("Failed to flush stdout: {}", e);
        process::exit(1);
    }
}