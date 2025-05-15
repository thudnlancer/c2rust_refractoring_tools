use std::ffi::CStr;
use std::io::{Error, ErrorKind};
use std::ptr;

const UNKNOWN_ERROR_MSG: &str = "Unknown error";

fn safe_copy(buf: &mut [u8], msg: &[u8]) -> Result<(), ErrorKind> {
    let len = msg.len();
    let buflen = buf.len();
    
    if len >= buflen {
        if buflen == 0 {
            return Err(ErrorKind::InvalidInput);
        }
        let copy_len = buflen - 1;
        buf[..copy_len].copy_from_slice(&msg[..copy_len]);
        buf[copy_len] = 0;
        Err(ErrorKind::InvalidData)
    } else {
        buf[..len].copy_from_slice(msg);
        buf[len] = 0;
        Ok(())
    }
}

pub fn rpl_strerror_r(errnum: i32, buf: &mut [u8]) -> Result<(), Error> {
    if buf.len() <= 1 {
        if !buf.is_empty() {
            buf[0] = 0;
        }
        return Err(Error::from(ErrorKind::InvalidData));
    }

    buf[0] = 0;

    let msg = match Error::from_raw_os_error(errnum).to_string() {
        Ok(s) => s,
        Err(_) => {
            if buf.len() >= UNKNOWN_ERROR_MSG.len() + 10 {
                let msg = format!("{} {}", UNKNOWN_ERROR_MSG, errnum);
                return safe_copy(buf, msg.as_bytes())
                    .map_err(|e| Error::from(e));
            }
            return Err(Error::from(ErrorKind::InvalidData));
        }
    };

    safe_copy(buf, msg.as_bytes())
        .map_err(|e| Error::from(e))
}