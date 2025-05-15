/*  Copyright 1996-1999,2001,2002,2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::io::{self, Read, Write};
use std::error::Error;

/*
 * Copy the data from source to target
 */

pub fn copyfile(
    source: &mut dyn Read,
    target: &mut dyn Write,
    got_signal: &mut bool,
) -> Result<u64, Box<dyn Error>> {
    const BUFFER_SIZE: usize = 8 * 16384;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut pos: u64 = 0;

    loop {
        let ret = match source.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("file read: {}", e);
                return Err(Box::new(e));
            }
        };

        if *got_signal {
            return Err("Signal received".into());
        }

        match target.write_all(&buffer[..ret]) {
            Ok(_) => (),
            Err(e) => {
                if e.kind() == io::ErrorKind::WriteZero {
                    eprintln!("Short write {} instead of {}", e, ret);
                } else {
                    eprintln!("write in copy: {}", e);
                }

                if e.kind() == io::ErrorKind::StorageFull {
                    *got_signal = true;
                }
                return Err(Box::new(e));
            }
        }

        pos += ret as u64;
    }

    Ok(pos)
}