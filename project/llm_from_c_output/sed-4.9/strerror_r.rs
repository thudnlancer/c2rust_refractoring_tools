use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::str;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref STRERROR_LOCK: Mutex<()> = Mutex::new(());
}

const STACKBUF_LEN: usize = 256;

fn safe_copy(buf: &mut [u8], msg: &str) -> Result<(), std::io::Error> {
    let msg_bytes = msg.as_bytes();
    let copy_len = std::cmp::min(buf.len().saturating_sub(1), msg_bytes.len());
    
    buf[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
    buf[copy_len] = b'\0';
    
    if copy_len < msg_bytes.len() {
        Err(std::io::Error::from_raw_os_error(libc::ERANGE))
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn windows_specific_error(errnum: c_int) -> Option<&'static str> {
    match errnum {
        100 => Some("Address already in use"),
        101 => Some("Cannot assign requested address"),
        102 => Some("Address family not supported by protocol"),
        103 => Some("Operation already in progress"),
        105 => Some("Operation canceled"),
        106 => Some("Software caused connection abort"),
        107 => Some("Connection refused"),
        108 => Some("Connection reset by peer"),
        109 => Some("Destination address required"),
        110 => Some("No route to host"),
        112 => Some("Operation now in progress"),
        113 => Some("Transport endpoint is already connected"),
        114 => Some("Too many levels of symbolic links"),
        115 => Some("Message too long"),
        116 => Some("Network is down"),
        117 => Some("Network dropped connection on reset"),
        118 => Some("Network is unreachable"),
        119 => Some("No buffer space available"),
        123 => Some("Protocol not available"),
        126 => Some("Transport endpoint is not connected"),
        128 => Some("Socket operation on non-socket"),
        129 => Some("Not supported"),
        130 => Some("Operation not supported"),
        132 => Some("Value too large for defined data type"),
        133 => Some("Owner died"),
        134 => Some("Protocol error"),
        135 => Some("Protocol not supported"),
        136 => Some("Protocol wrong type for socket"),
        138 => Some("Connection timed out"),
        140 => Some("Operation would block"),
        _ => None,
    }
}

pub fn strerror_r(errnum: c_int, buf: &mut [u8]) -> Result<(), std::io::Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = b'\0';
        }
        return Err(std::io::Error::from_raw_os_error(libc::ERANGE));
    }

    buf[0] = b'\0';

    // Check for overrides first
    if let Some(msg) = strerror_override(errnum) {
        return safe_copy(buf, msg);
    }

    let saved_errno = std::io::Error::last_os_error();

    let result = unsafe {
        let mut ret = libc::strerror_r(errnum, buf.as_mut_ptr() as *mut i8, buf.len());
        
        if buf[0] == b'\0' {
            let mut stackbuf = [0u8; STACKBUF_LEN];
            ret = libc::strerror_r(errnum, stackbuf.as_mut_ptr() as *mut i8, stackbuf.len());
            
            if ret == 0 {
                let c_str = CStr::from_ptr(stackbuf.as_ptr() as *const i8);
                if let Ok(msg) = c_str.to_str() {
                    safe_copy(buf, msg)?;
                }
            } else {
                return Err(std::io::Error::last_os_error());
            }
        }
        
        ret
    };

    if result == 0 {
        // Check for truncation
        let len = unsafe { libc::strlen(buf.as_ptr() as *const i8) };
        if len == buf.len() - 1 {
            let mut stackbuf = [0u8; STACKBUF_LEN];
            unsafe {
                if libc::strerror_r(errnum, stackbuf.as_mut_ptr() as *mut i8, stackbuf.len()) == 0 {
                    let c_str = CStr::from_ptr(stackbuf.as_ptr() as *const i8);
                    if let Ok(msg) = c_str.to_str() {
                        if msg.len() >= buf.len() {
                            return Err(std::io::Error::from_raw_os_error(libc::ERANGE));
                        }
                    }
                }
            }
        }
    } else {
        // Handle platform-specific cases
        #[cfg(windows)]
        if result == libc::EINVAL && buf[0] == b'\0' {
            if let Some(msg) = windows_specific_error(errnum) {
                return safe_copy(buf, msg);
            }
        }

        if result == libc::EINVAL && buf[0] == b'\0' {
            let msg = if cfg!(target_os = "haiku") {
                format!("Unknown Application Error ({})", errnum)
            } else {
                format!("Unknown error {}", errnum)
            };
            safe_copy(buf, &msg)?;
        }
    }

    unsafe {
        libc::set_errno(saved_errno.raw_os_error().unwrap_or(0));
    }

    if result != 0 {
        Err(std::io::Error::from_raw_os_error(result))
    } else {
        Ok(())
    }
}

// Placeholder for strerror_override function
fn strerror_override(_errnum: c_int) -> Option<&'static str> {
    None
}