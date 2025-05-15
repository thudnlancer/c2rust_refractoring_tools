use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::str;
use std::sync::OnceLock;

static STRERROR_LOCK: OnceLock<()> = OnceLock::new();

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

#[cfg(not(target_os = "windows"))]
fn get_system_error(errnum: c_int) -> Option<&'static str> {
    unsafe {
        let errmsg = libc::strerror(errnum);
        if errmsg.is_null() {
            None
        } else {
            CStr::from_ptr(errmsg).to_str().ok()
        }
    }
}

#[cfg(target_os = "windows")]
fn get_system_error(errnum: c_int) -> Option<&'static str> {
    // Windows specific error messages
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

    // Check for overrides first (not implemented in this translation)
    // if let Some(msg) = strerror_override(errnum) {
    //     return safe_copy(buf, msg);
    // }

    let saved_errno = std::io::Error::last_os_error();

    let result = if cfg!(target_os = "haiku") {
        // Haiku specific handling
        if let Some(errmsg) = get_system_error(errnum) {
            safe_copy(buf, errmsg)
        } else {
            let msg = format!("Unknown Application Error ({})", errnum);
            safe_copy(buf, &msg)
        }
    } else {
        // Try system strerror_r if available
        #[cfg(not(target_os = "windows"))]
        {
            let mut local_buf = [0u8; 256];
            let ret = unsafe {
                libc::strerror_r(errnum, local_buf.as_mut_ptr() as *mut _, local_buf.len())
            };

            if ret == 0 {
                let msg = CStr::from_ptr(local_buf.as_ptr())
                    .to_str()
                    .unwrap_or("Unknown error");
                safe_copy(buf, msg)
            } else {
                // Fallback to strerror
                let _lock = STRERROR_LOCK.get_or_init(|| ());
                if let Some(errmsg) = get_system_error(errnum) {
                    safe_copy(buf, errmsg)
                } else {
                    let msg = format!("Unknown error {}", errnum);
                    safe_copy(buf, &msg)
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            let _lock = STRERROR_LOCK.get_or_init(|| ());
            if let Some(errmsg) = get_system_error(errnum) {
                safe_copy(buf, errmsg)
            } else {
                let msg = format!("Unknown error {}", errnum);
                safe_copy(buf, &msg)
            }
        }
    };

    // Restore original errno
    if saved_errno.raw_os_error().is_some() {
        unsafe { libc::errno = saved_errno.raw_os_error().unwrap() };
    }

    result
}