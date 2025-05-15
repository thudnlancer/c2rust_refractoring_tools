use std::io::{self, Read};
use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetwordEndianState {
    Initial = 0,
    Native = 1,
    Swab = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
union WordData {
    ival: c_int,
    data: [c_uchar; 4],
}

const WORDBYTES: usize = 4;

fn decode_value(
    data: &[u8; WORDBYTES],
    limit: c_int,
    endian_state_flag: &mut GetwordEndianState,
    filename: &CStr,
) -> c_int {
    let mut u = WordData { ival: 0 };
    unsafe {
        u.data.copy_from_slice(data);
    }

    let swapped = unsafe { u.ival.swap_bytes() };

    match *endian_state_flag {
        GetwordEndianState::Initial => {
            if unsafe { u.ival } <= limit {
                if swapped > limit {
                    *endian_state_flag = GetwordEndianState::Native;
                }
                unsafe { u.ival }
            } else if swapped <= limit {
                eprintln!(
                    "WARNING: locate database {} was built with a different byte order",
                    filename.to_string_lossy()
                );
                *endian_state_flag = GetwordEndianState::Swab;
                swapped
            } else {
                unsafe { u.ival }
            }
        }
        GetwordEndianState::Swab => swapped,
        GetwordEndianState::Native => unsafe { u.ival },
    }
}

pub fn getword(
    mut reader: impl Read,
    filename: &CStr,
    maxvalue: usize,
    endian_state_flag: &mut GetwordEndianState,
) -> io::Result<c_int> {
    let mut data = [0u8; WORDBYTES];
    reader.read_exact(&mut data)?;

    Ok(decode_value(
        &data,
        maxvalue as c_int,
        endian_state_flag,
        filename,
    ))
}