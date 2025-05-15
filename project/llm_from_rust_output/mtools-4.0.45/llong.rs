use std::io::{self, SeekFrom};
use std::process;

type MtOffT = i64;
type UInt32 = u32;

const MAX_UINT32: u64 = u32::MAX as u64;

pub static MAX_OFF_T_31: MtOffT = ((1 << 30) - 1) << 1 | 1;
pub static MAX_OFF_T_32: MtOffT = ((1 << 31) - 1) << 1 | 1;
pub static MAX_OFF_T_41: MtOffT = ((1 << 40) - 1) << 1 | 1;
pub static MAX_OFF_T_SEEK: MtOffT = ((1 << (std::mem::size_of::<i64>() * 8 - 2)) - 1) << 1 | 1;

pub fn file_too_big(off: MtOffT) -> bool {
    off & !MAX_OFF_T_32 != 0
}

pub fn trunc_bytes_32(off: MtOffT) -> Result<i64, String> {
    if file_too_big(off) {
        Err("Internal error, offset too big".to_string())
    } else {
        Ok(off)
    }
}

pub fn trunc_mt_off_to_32u(off: MtOffT) -> Result<UInt32, String> {
    if file_too_big(off) {
        Err("Internal error, offset too big".to_string())
    } else {
        Ok(off as UInt32)
    }
}

pub fn trunc_size_to_32u(siz: usize) -> Result<UInt32, String> {
    if siz > MAX_UINT32 as usize {
        Err("Internal error, size too big".to_string())
    } else {
        Ok(siz as UInt32)
    }
}

pub fn mt_lseek(fd: &mut std::fs::File, offset: MtOffT, whence: SeekFrom) -> io::Result<()> {
    fd.seek(whence)?;
    Ok(())
}

pub fn log_2(size: u32) -> u32 {
    for i in 0..24 {
        if (1 << i) == size {
            return i;
        }
    }
    24
}

fn handle_error(err: String) -> ! {
    eprintln!("{}", err);
    process::exit(1);
}

// Example usage wrappers that handle errors
pub fn safe_trunc_bytes_32(off: MtOffT) -> i64 {
    trunc_bytes_32(off).unwrap_or_else(handle_error)
}

pub fn safe_trunc_mt_off_to_32u(off: MtOffT) -> UInt32 {
    trunc_mt_off_to_32u(off).unwrap_or_else(handle_error)
}

pub fn safe_trunc_size_to_32u(siz: usize) -> UInt32 {
    trunc_size_to_32u(siz).unwrap_or_else(handle_error)
}