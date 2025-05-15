/*
 *  Copyright 1996,1997,1999,2001-2003,2008,2009,2021 Alain Knaff.
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

use std::io::{self, Read};

struct Filter {
    next: Box<dyn Read>,
    mode: i32,
    // convert_charset: bool,
}

impl Filter {
    fn new(next: Box<dyn Read>, _convert_charset: bool) -> io::Result<Self> {
        Ok(Self {
            next,
            mode: 0,
            // convert_charset,
        })
    }
}

impl Read for Filter {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut temp_buf = vec![0; buf.len()];
        let ret = self.next.read(&mut temp_buf)?;

        let mut j = 0;
        for i in 0..ret {
            if temp_buf[i] == b'\r' {
                continue;
            }
            if temp_buf[i] == 0x1a {
                break;
            }
            let mut newchar = temp_buf[i];
            /*
            if self.convert_charset {
                newchar = contents_to_unix(newchar);
            }
            */
            buf[j] = newchar;
            j += 1;
        }

        Ok(j)
    }
}

fn open_dos2unix(next: Box<dyn Read>, _convert_charset: bool) -> io::Result<Box<dyn Read>> {
    Filter::new(next, _convert_charset).map(|f| Box::new(f) as Box<dyn Read>)
}