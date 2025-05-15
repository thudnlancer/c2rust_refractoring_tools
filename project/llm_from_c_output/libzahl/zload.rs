/* See LICENSE file for copyright and license details. */
use std::mem;
use std::io::{self, Read};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Z {
    sign: i64,
    used: usize,
    chars: Vec<u8>,
}

impl Z {
    pub fn zero() -> Self {
        Self {
            sign: 0,
            used: 0,
            chars: Vec::new(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }
}

pub fn zload(buffer: &[u8]) -> io::Result<(Z, usize)> {
    let mut offset = 0;

    // Read sign (long in C)
    if offset + mem::size_of::<i64>() > buffer.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Buffer too short for sign"));
    }
    let sign = i64::from_ne_bytes(buffer[offset..offset + mem::size_of::<i64>()].try_into().unwrap());
    offset += mem::size_of::<i64>();

    // Read used (size_t in C)
    if offset + mem::size_of::<usize>() > buffer.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Buffer too short for used"));
    }
    let used = usize::from_ne_bytes(buffer[offset..offset + mem::size_of::<usize>()].try_into().unwrap());
    offset += mem::size_of::<usize>();

    let mut a = if sign != 0 {
        // Read chars if not zero
        let char_size = (used + 3) & !3; // Align to 4 bytes
        if offset + char_size > buffer.len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Buffer too short for chars"));
        }
        let chars = buffer[offset..offset + used].to_vec();
        offset += char_size;

        Z {
            sign,
            used,
            chars,
        }
    } else {
        Z::zero()
    };

    let bytes_read = mem::size_of::<i64>() + mem::size_of::<usize>() + 
        if a.is_zero() { 0 } else { ((a.used + 3) & !3) };

    Ok((a, bytes_read))
}