//! Module for creating string descriptions of file modes.
//!
//! This code is based on the original C implementation from GNU coreutils,
//! translated to safe Rust while maintaining the same functionality.

use std::os::unix::fs::FileTypeExt;
use std::ffi::CStr;
use libc::{mode_t, stat};

/// Returns a character indicating the type of file described by the mode bits.
/// 
/// The characters returned are:
/// '-' regular file
/// 'b' block special file
/// 'c' character special file
/// 'C' high performance ("contiguous data") file
/// 'd' directory
/// 'D' door
/// 'l' symbolic link
/// 'm' multiplexed file (7th edition Unix; obsolete)
/// 'n' network special file (HP-UX)
/// 'p' fifo (named pipe)
/// 'P' port
/// 's' socket
/// 'w' whiteout (4.4BSD)
/// '?' some other file type
fn ftypelet(mode: mode_t) -> char {
    let ft = std::fs::FileType::from_mode(mode);
    
    if ft.is_file() {
        return '-';
    }
    if ft.is_dir() {
        return 'd';
    }
    if ft.is_block_device() {
        return 'b';
    }
    if ft.is_char_device() {
        return 'c';
    }
    if ft.is_symlink() {
        return 'l';
    }
    if ft.is_fifo() {
        return 'p';
    }
    if ft.is_socket() {
        return 's';
    }

    // Check for non-standard file types
    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    if (mode & libc::S_IFDOOR) != 0 {
        return 'D';
    }

    #[cfg(target_os = "linux")]
    if (mode & libc::S_IFREG) != 0 && (mode & libc::S_ISVTX) != 0 {
        return 'C';
    }

    #[cfg(target_os = "hpux")]
    if (mode & libc::S_IFNWK) != 0 {
        return 'n';
    }

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    if (mode & libc::S_IFPORT) != 0 {
        return 'P';
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd", target_os = "netbsd"))]
    if (mode & libc::S_IFWHT) != 0 {
        return 'w';
    }

    '?'
}

/// Creates a mode string similar to `strmode` in C.
/// 
/// The string will be 12 characters long, including the null terminator.
pub fn strmode(mode: mode_t, str: &mut [u8; 12]) {
    str[0] = ftypelet(mode) as u8;
    str[1] = if mode & libc::S_IRUSR != 0 { b'r' } else { b'-' };
    str[2] = if mode & libc::S_IWUSR != 0 { b'w' } else { b'-' };
    str[3] = match (mode & libc::S_ISUID, mode & libc::S_IXUSR) {
        (0, 0) => b'-',
        (0, _) => b'x',
        (_, 0) => b'S',
        (_, _) => b's',
    };
    str[4] = if mode & libc::S_IRGRP != 0 { b'r' } else { b'-' };
    str[5] = if mode & libc::S_IWGRP != 0 { b'w' } else { b'-' };
    str[6] = match (mode & libc::S_ISGID, mode & libc::S_IXGRP) {
        (0, 0) => b'-',
        (0, _) => b'x',
        (_, 0) => b'S',
        (_, _) => b's',
    };
    str[7] = if mode & libc::S_IROTH != 0 { b'r' } else { b'-' };
    str[8] = if mode & libc::S_IWOTH != 0 { b'w' } else { b'-' };
    str[9] = match (mode & libc::S_ISVTX, mode & libc::S_IXOTH) {
        (0, 0) => b'-',
        (0, _) => b'x',
        (_, 0) => b'T',
        (_, _) => b't',
    };
    str[10] = b' ';
    str[11] = b'\0';
}

/// Creates a mode string from a stat structure.
/// 
/// Handles special file types that can't be determined from mode alone.
pub fn filemodestring(statp: &stat, str: &mut [u8; 12]) {
    strmode(statp.st_mode, str);

    #[cfg(any(target_os = "linux", target_os = "solaris", target_os = "illumos"))]
    {
        // Check for special file types
        unsafe {
            if S_TYPEISSEM(statp) {
                str[0] = b'F';
            } else if S_TYPEISMQ(statp) {
                str[0] = b'Q';
            } else if S_TYPEISSHM(statp) {
                str[0] = b'S';
            } else if S_TYPEISTMO(statp) {
                str[0] = b'T';
            }
        }
    }
}

// Helper functions for checking special file types
#[cfg(any(target_os = "linux", target_os = "solaris", target_os = "illumos"))]
unsafe fn S_TYPEISSEM(statp: &stat) -> bool {
    libc::S_ISSEM(statp.st_mode)
}

#[cfg(any(target_os = "linux", target_os = "solaris", target_os = "illumos"))]
unsafe fn S_TYPEISMQ(statp: &stat) -> bool {
    libc::S_ISMQ(statp.st_mode)
}

#[cfg(any(target_os = "linux", target_os = "solaris", target_os = "illumos"))]
unsafe fn S_TYPEISSHM(statp: &stat) -> bool {
    libc::S_ISSHM(statp.st_mode)
}

#[cfg(any(target_os = "linux", target_os = "solaris", target_os = "illumos"))]
unsafe fn S_TYPEISTMO(statp: &stat) -> bool {
    libc::S_ISTMO(statp.st_mode)
}