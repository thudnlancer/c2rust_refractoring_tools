use std::error::Error;
use std::ffi::CStr;
use std::fmt::Write;
use std::io::{Error as IoError, ErrorKind};
use std::sync::OnceLock;

static STRERROR_LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();

fn strerror_lock() -> &'static std::sync::Mutex<()> {
    STRERROR_LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

fn safe_copy(buf: &mut [u8], msg: &str) -> Result<(), IoError> {
    let msg_bytes = msg.as_bytes();
    let copy_len = std::cmp::min(buf.len().saturating_sub(1), msg_bytes.len());
    
    buf[..copy_len].copy_from_slice(&msg_bytes[..copy_len]);
    buf[copy_len] = b'\0';
    
    if copy_len < msg_bytes.len() {
        Err(IoError::new(ErrorKind::Other, "buffer too small"))
    } else {
        Ok(())
    }
}

fn strerror_override(errnum: i32) -> Option<&'static str> {
    // Placeholder for strerror_override functionality
    None
}

pub fn strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), IoError> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = b'\0';
        }
        return Err(IoError::new(ErrorKind::Other, "buffer too small"));
    }

    buf[0] = b'\0';

    if let Some(msg) = strerror_override(errnum) {
        return safe_copy(buf, msg);
    }

    let saved_errno = IoError::last_os_error();

    let result = unsafe {
        let c_str = libc::strerror(errnum);
        if c_str.is_null() {
            Err(IoError::from_raw_os_error(errnum))
        } else {
            let msg = CStr::from_ptr(c_str).to_str().unwrap_or("Unknown error");
            safe_copy(buf, msg)
        }
    };

    if let Err(e) = &result {
        if e.kind() == ErrorKind::InvalidInput && buf[0] == b'\0' {
            let _ = write!(unsafe { std::str::from_utf8_unchecked_mut(buf) }, "Unknown error {}", errnum);
            buf[buf.len() - 1] = b'\0';
        }
    }

    let _ = IoError::last_os_error();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strerror_r() {
        let mut buf = [0u8; 256];
        assert!(strerror_r(libc::EINVAL, &mut buf).is_ok());
        assert!(!buf.starts_with(b"\0"));
    }

    #[test]
    fn test_small_buffer() {
        let mut buf = [0u8; 1];
        assert!(strerror_r(libc::EINVAL, &mut buf).is_err());
    }
}