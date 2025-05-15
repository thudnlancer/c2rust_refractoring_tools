use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use std::ptr;

fn safe_copy(buf: &mut [u8], msg: &[u8]) -> Result<(), ErrorKind> {
    let len = msg.len();
    let buflen = buf.len();
    let moved = if len < buflen {
        len
    } else {
        buflen.saturating_sub(1)
    };

    if moved > 0 {
        buf[..moved].copy_from_slice(&msg[..moved]);
    }
    if moved < buflen {
        buf[moved] = 0;
    }

    if len < buflen {
        Ok(())
    } else {
        Err(ErrorKind::InvalidData)
    }
}

pub fn rpl_strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = 0;
        }
        return Err(Error::new(ErrorKind::InvalidData, "buffer too small"));
    }

    buf[0] = 0;

    let saved_errno = Error::last_os_error().raw_os_error().unwrap_or(0);

    let ret = unsafe {
        let mut ret = 0;
        let c_buf = buf.as_mut_ptr() as *mut libc::c_char;
        ret = libc::__xpg_strerror_r(errnum, c_buf, buf.len());
        if ret < 0 {
            ret = Error::last_os_error().raw_os_error().unwrap_or(0);
        }
        ret
    };

    if buf[0] == 0 {
        let errstring = unsafe {
            let c_buf = buf.as_mut_ptr() as *mut libc::c_char;
            libc::strerror_r(errnum, c_buf, buf.len())
        };

        if !errstring.is_null() {
            let msg = unsafe { CStr::from_ptr(errstring).to_bytes() };
            if let Err(e) = safe_copy(buf, msg) {
                return Err(Error::new(e, "failed to copy error message"));
            }
        } else {
            return Err(Error::last_os_error());
        }
    }

    if ret == 22 && buf[0] == 0 {
        let msg = format!("Unknown error {}", errnum);
        let msg_bytes = msg.as_bytes();
        let copied = std::cmp::min(buf.len().saturating_sub(1), msg_bytes.len());
        buf[..copied].copy_from_slice(&msg_bytes[..copied]);
        buf[copied] = 0;
    }

    unsafe {
        libc::__errno_location().write(saved_errno);
    }

    if ret == 0 {
        Ok(())
    } else {
        Err(Error::from_raw_os_error(ret))
    }
}