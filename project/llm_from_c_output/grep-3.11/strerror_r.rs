use std::ffi::CString;
use std::io::Error;
use std::ptr;
use std::str;

#[cfg(target_os = "windows")]
use winapi::um::winsock2;

const STACKBUF_LEN: usize = 256;

fn safe_copy(buf: &mut [u8], msg: &str) -> Result<(), Error> {
    let msg_bytes = msg.as_bytes();
    let copy_len = msg_bytes.len().min(buf.len().saturating_sub(1));
    
    buf[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
    if copy_len < buf.len() {
        buf[copy_len] = b'\0';
    }
    
    if msg_bytes.len() < buf.len() {
        Ok(())
    } else {
        Err(Error::from_raw_os_error(libc::ERANGE))
    }
}

pub fn strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = b'\0';
        }
        return Err(Error::from_raw_os_error(libc::ERANGE));
    }
    
    buf[0] = b'\0';
    
    // Check for overrides first
    if let Some(msg) = strerror_override(errnum) {
        return safe_copy(buf, msg);
    }
    
    let saved_errno = Error::last_os_error();
    
    let result = unsafe {
        let c_str = libc::strerror(errnum);
        if c_str.is_null() {
            Err(Error::from_raw_os_error(libc::EINVAL))
        } else {
            let msg = str::from_utf8_unchecked(std::ffi::CStr::from_ptr(c_str).to_bytes());
            safe_copy(buf, msg)
        }
    };
    
    // Handle Windows specific error codes
    #[cfg(target_os = "windows")]
    if let Err(e) = &result {
        if e.raw_os_error() == Some(libc::EINVAL) && buf[0] == b'\0' {
            let msg = match errnum {
                100 => "Address already in use",
                101 => "Cannot assign requested address",
                102 => "Address family not supported by protocol",
                103 => "Operation already in progress",
                105 => "Operation canceled",
                106 => "Software caused connection abort",
                107 => "Connection refused",
                108 => "Connection reset by peer",
                109 => "Destination address required",
                110 => "No route to host",
                112 => "Operation now in progress",
                113 => "Transport endpoint is already connected",
                114 => "Too many levels of symbolic links",
                115 => "Message too long",
                116 => "Network is down",
                117 => "Network dropped connection on reset",
                118 => "Network is unreachable",
                119 => "No buffer space available",
                123 => "Protocol not available",
                126 => "Transport endpoint is not connected",
                128 => "Socket operation on non-socket",
                129 => "Not supported",
                130 => "Operation not supported",
                132 => "Value too large for defined data type",
                133 => "Owner died",
                134 => "Protocol error",
                135 => "Protocol not supported",
                136 => "Protocol wrong type for socket",
                138 => "Connection timed out",
                140 => "Operation would block",
                _ => "",
            };
            
            if !msg.is_empty() {
                return safe_copy(buf, msg);
            }
        }
    }
    
    // Handle case where error is still unknown
    if let Err(e) = &result {
        if e.raw_os_error() == Some(libc::EINVAL) && buf[0] == b'\0' {
            let msg = if cfg!(target_os = "haiku") {
                format!("Unknown Application Error ({})", errnum)
            } else {
                format!("Unknown error {}", errnum)
            };
            return safe_copy(buf, &msg);
        }
    }
    
    // Restore original errno
    unsafe {
        libc::errno = saved_errno.raw_os_error();
    }
    
    result
}

// Placeholder for strerror_override function
fn strerror_override(_errnum: i32) -> Option<&'static str> {
    None
}