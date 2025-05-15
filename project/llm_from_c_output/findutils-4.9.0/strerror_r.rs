use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::str;
use std::sync::OnceLock;
use std::io::{Error, ErrorKind};

static STRERROR_LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();

fn strerror_override(errnum: i32) -> Option<&'static str> {
    // Placeholder for strerror override functionality
    None
}

fn safe_copy(buf: &mut [u8], msg: &str) -> Result<(), Error> {
    let msg_bytes = msg.as_bytes();
    let copy_len = std::cmp::min(buf.len().saturating_sub(1), msg_bytes.len());
    
    if copy_len < msg_bytes.len() {
        buf[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
        buf[copy_len] = b'\0';
        Err(Error::new(ErrorKind::Other, "ERANGE"))
    } else {
        buf[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
        buf[copy_len] = b'\0';
        Ok(())
    }
}

pub fn strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = b'\0';
        }
        return Err(Error::new(ErrorKind::Other, "ERANGE"));
    }
    buf[0] = b'\0';

    if let Some(msg) = strerror_override(errnum) {
        return safe_copy(buf, msg);
    }

    let saved_errno = Error::last_os_error();

    #[cfg(any(target_os = "netbsd", target_os = "hpux", windows, target_os = "cygwin"))]
    {
        extern {
            static sys_nerr: c_int;
            static sys_errlist: *const *const i8;
        }

        if errnum >= 0 && errnum < unsafe { sys_nerr } {
            let errptr = unsafe { *sys_errlist.offset(errnum as isize) };
            if !errptr.is_null() {
                let errmsg = unsafe { CStr::from_ptr(errptr) }.to_str().unwrap_or("");
                if !errmsg.is_empty() {
                    return safe_copy(buf, errmsg);
                }
            }
        }
    }

    #[cfg(any(target_os = "solaris", target_os = "irix"))]
    {
        extern {
            static sys_nerr: c_int;
        }

        if errnum >= 0 && errnum < unsafe { sys_nerr } {
            let errmsg = Error::from_raw_os_error(errnum).to_string();
            return safe_copy(buf, &errmsg);
        }
    }

    #[cfg(not(any(
        target_os = "netbsd",
        target_os = "hpux",
        windows,
        target_os = "cygwin",
        target_os = "solaris",
        target_os = "irix"
    )))]
    {
        let lock = STRERROR_LOCK.get_or_init(|| std::sync::Mutex::new(()));
        let _guard = lock.lock().unwrap();
        
        let errmsg = Error::from_raw_os_error(errnum).to_string();
        return safe_copy(buf, &errmsg);
    }

    #[cfg(windows)]
    {
        let errmsg = match errnum {
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
        
        if !errmsg.is_empty() {
            return safe_copy(buf, errmsg);
        }
    }

    if buf[0] == b'\0' {
        let unknown_msg = if cfg!(target_os = "haiku") {
            format!("Unknown Application Error ({})", errnum)
        } else {
            format!("Unknown error {}", errnum)
        };
        return safe_copy(buf, &unknown_msg);
    }

    unsafe { Error::set_last_os_error(saved_errno.raw_os_error().unwrap_or(0)) };
    Ok(())
}