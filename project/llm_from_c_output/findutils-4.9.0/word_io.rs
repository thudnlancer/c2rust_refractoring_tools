/* word_io.rs -- word oriented I/O
   Copyright (C) 2007-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io::{self, Read};
use std::mem;
use byteorder::{ByteOrder, NativeEndian, BigEndian, LittleEndian};
use std::convert::TryInto;

const WORDBYTES: usize = 4;

#[derive(Debug, PartialEq)]
enum GetwordEndianState {
    Initial,
    Native,
    Swab,
}

fn decode_value(
    data: &[u8; WORDBYTES],
    limit: i32,
    endian_state_flag: &mut GetwordEndianState,
    filename: &str,
) -> i32 {
    let native_val = i32::from_ne_bytes(*data);
    let swapped_val = i32::from_be_bytes(data.swap_bytes());

    if *endian_state_flag == GetwordEndianState::Initial {
        if native_val <= limit {
            if swapped_val > limit {
                /* the native value is inside the limit and the
                 * swapped value is not. We take this as proof
                 * that we should be using the native byte order.
                 */
                *endian_state_flag = GetwordEndianState::Native;
            }
            native_val
        } else {
            if swapped_val <= limit {
                /* Aha, now we know we have to byte-swap. */
                eprintln!(
                    "WARNING: locate database {} was built with a different byte order",
                    filename
                );
                *endian_state_flag = GetwordEndianState::Swab;
                swapped_val
            } else {
                /* native_val > limit and swapped_val > limit. For the moment, assume
                 * native ordering.
                 */
                native_val
            }
        }
    } else {
        /* We already know the byte order. */
        if *endian_state_flag == GetwordEndianState::Swab {
            swapped_val
        } else {
            native_val
        }
    }
}

fn getword<R: Read>(
    fp: &mut R,
    filename: &str,
    maxvalue: usize,
    endian_state_flag: &mut GetwordEndianState,
) -> Result<i32, io::Error> {
    let mut data = [0u8; WORDBYTES];
    match fp.read_exact(&mut data) {
        Ok(()) => Ok(decode_value(
            &data,
            maxvalue.try_into().unwrap(),
            endian_state_flag,
            filename,
        )),
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!("unexpected EOF in {}", filename),
            ))
        }
        Err(e) => Err(io::Error::new(
            e.kind(),
            format!("error reading a word from {}: {}", filename, e),
        )),
    }
}

trait SwapBytes {
    fn swap_bytes(&self) -> [u8; 4];
}

impl SwapBytes for [u8; 4] {
    fn swap_bytes(&self) -> [u8; 4] {
        [self[3], self[2], self[1], self[0]]
    }
}