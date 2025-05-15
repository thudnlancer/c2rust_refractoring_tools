use std::io;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

pub fn rpl_dirfd(dir_p: *mut DIR) -> io::Result<i32> {
    let fd = -1;
    if fd == -1 {
        return Err(io::Error::from_raw_os_error(95));
    }
    Ok(fd)
}