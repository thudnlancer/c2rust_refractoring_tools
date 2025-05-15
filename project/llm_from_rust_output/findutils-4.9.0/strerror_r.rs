use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use std::ptr;

fn safe_copy(buf: &mut [u8], msg: &[u8]) -> Result<(), Error> {
    let len = msg.len();
    let moved = if len < buf.len() {
        len
    } else {
        buf.len().saturating_sub(1)
    };

    if moved > 0 {
        buf[..moved].copy_from_slice(&msg[..moved]);
    }
    if buf.len() > moved {
        buf[moved] = 0;
    }

    if len < buf.len() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "buffer too small"))
    }
}

pub fn rpl_strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = 0;
        }
        return Err(Error::new(ErrorKind::Other, "buffer too small"));
    }

    buf[0] = 0;
    let saved_errno = Error::last_os_error();

    let ret = unsafe {
        let mut ret = 0;
        let c_str = CStr::from_ptr(ptr::null());
        if !c_str.as_ptr().is_null() {
            match safe_copy(buf, c_str.to_bytes()) {
                Ok(()) => ret = 0,
                Err(e) => ret = e.raw_os_error().unwrap_or(34),
            }
            return if ret == 0 { Ok(()) } else { Err(Error::from_raw_os_error(ret)) };
        }

        ret = libc::__xpg_strerror_r(errnum, buf.as_mut_ptr() as *mut i8, buf.len());
        if ret < 0 {
            ret = Error::last_os_error().raw_os_error().unwrap_or(34);
        }
        ret
    };

    if buf[0] == 0 {
        let errstring = unsafe {
            libc::strerror_r(errnum, buf.as_mut_ptr() as *mut i8, buf.len())
        };
        if !errstring.is_null() {
            let c_str = unsafe { CStr::from_ptr(errstring) };
            match safe_copy(buf, c_str.to_bytes()) {
                Ok(()) => return Ok(()),
                Err(e) => return Err(e),
            }
        } else {
            return Err(Error::last_os_error());
        }
    }

    if ret == 22 && buf[0] == 0 {
        let msg = format!("Unknown error {}", errnum);
        let bytes = msg.as_bytes();
        let len = bytes.len().min(buf.len().saturating_sub(1));
        buf[..len].copy_from_slice(&bytes[..len]);
        if len < buf.len() {
            buf[len] = 0;
        }
    }

    unsafe {
        libc::__errno_location().write(saved_errno.raw_os_error().unwrap_or(0));
    }

    if ret == 0 {
        Ok(())
    } else {
        Err(Error::from_raw_os_error(ret))
    }
}