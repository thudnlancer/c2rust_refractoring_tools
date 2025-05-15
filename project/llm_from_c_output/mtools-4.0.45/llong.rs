// Copyright 1999,2001-2004,2007-2009,2021 Alain Knaff.
// This file is part of mtools.
//
// Mtools is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mtools is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mtools.  If not, see <http://www.gnu.org/licenses/>.

use std::convert::TryFrom;
use std::io::{Seek, SeekFrom};
use std::process;

pub type MtOffT = i64; // Assuming 64-bit off_t for modern systems

pub const MAX_OFF_T_31: MtOffT = max_off_t_b(31); // Floppyd
const MAX_OFF_T_32: MtOffT = max_off_t_b(32); // Directory
pub const MAX_OFF_T_41: MtOffT = max_off_t_b(41); // SCSI
pub const MAX_OFF_T_SEEK: MtOffT = max_off_t_b(63); // Assuming 64-bit seek

pub type SmtOffT = u32; // Common supertype for 32-bit systems

pub fn to_mt_off_t(off: u32) -> MtOffT {
    if off > u32::MAX >> 1 {
        eprintln!("File size/pos {} too big for this platform", off);
        process::exit(1);
    }
    off as MtOffT
}

pub fn file_too_big(off: MtOffT) -> bool {
    (off & !MAX_OFF_T_32) != 0
}

pub fn trunc_bytes32(off: MtOffT) -> Result<i32, String> {
    if file_too_big(off) {
        return Err("Internal error, offset too big".to_string());
    }
    Ok(off as i32)
}

pub fn trunc_mt_off_to32u(off: MtOffT) -> Result<u32, String> {
    if file_too_big(off) {
        return Err("Internal error, offset too big".to_string());
    }
    Ok(off as u32)
}

pub fn trunc_size_to32u(siz: usize) -> Result<u32, String> {
    match u32::try_from(siz) {
        Ok(val) => Ok(val),
        Err(_) => Err("Internal error, size too big".to_string()),
    }
}

pub fn mt_lseek<F: Seek>(file: &mut F, where_: MtOffT, whence: SeekFrom) -> std::io::Result<()> {
    file.seek(whence)?;
    Ok(())
}

pub fn log_2(size: u32) -> u32 {
    for i in 0..24 {
        if 1u32 << i == size {
            return i;
        }
    }
    24
}

const fn max_off_t_b(bits: u32) -> MtOffT {
    (((1 as MtOffT) << std::cmp::min(bits - 1, (std::mem::size_of::<MtOffT>() * 8 - 2) as u32) - 1) << 1) | 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_off_t_b() {
        assert_eq!(max_off_t_b(31), 0x7FFFFFFF);
        assert_eq!(max_off_t_b(32), 0xFFFFFFFF);
        assert_eq!(max_off_t_b(41), 0x1FFFFFFFFFF);
    }

    #[test]
    fn test_file_too_big() {
        assert!(!file_too_big(MAX_OFF_T_32));
        assert!(file_too_big(MAX_OFF_T_32 + 1));
    }
}