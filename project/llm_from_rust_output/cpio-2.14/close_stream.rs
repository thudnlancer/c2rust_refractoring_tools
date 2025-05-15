use std::fs::File;
use std::io::{self, ErrorKind, Write, Error};
use std::os::unix::io::AsRawFd;

pub fn close_stream(mut stream: File) -> io::Result<()> {
    let pending_result = stream.flush();
    let prev_fail = stream.try_clone().map_err(|_| ()).is_err();
    let fclose_fail = stream.sync_all().is_err();

    if prev_fail || (fclose_fail && (pending_result.is_ok() || Error::last_os_error().raw_os_error() != Some(9))) {
        if !fclose_fail {
            // Clear error if fclose didn't fail
            let _ = Error::last_os_error();
        }
        Err(Error::new(ErrorKind::Other, "Stream close failed"))
    } else {
        Ok(())
    }
}