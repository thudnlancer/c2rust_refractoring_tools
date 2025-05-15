/*
 *  Copyright 2021 Alain Knaff.
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
 *
 * filter to support filesystems stored at an offset into their image
 */

use std::io::{self, Read, Write};
use std::fmt;

struct Offset {
    next: Box<dyn Stream>,
    offset: i64,
}

trait Stream: Read + Write {
    fn pread(&mut self, buf: &mut [u8], start: i64) -> io::Result<usize>;
    fn pwrite(&mut self, buf: &[u8], start: i64) -> io::Result<usize>;
    fn set_geometry(&mut self, geometry: Geometry) -> io::Result<()>;
    fn get_dos_conversion(&self) -> DosConversion;
}

impl Read for Offset {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.pread(buf, 0)
    }
}

impl Write for Offset {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.pwrite(buf, 0)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        self.next.flush()
    }
}

impl Stream for Offset {
    fn pread(&mut self, buf: &mut [u8], start: i64) -> io::Result<usize> {
        self.next.pread(buf, start + self.offset)
    }

    fn pwrite(&mut self, buf: &[u8], start: i64) -> io::Result<usize> {
        self.next.pwrite(buf, start + self.offset)
    }

    fn set_geometry(&mut self, geometry: Geometry) -> io::Result<()> {
        self.next.set_geometry(geometry)
    }

    fn get_dos_conversion(&self) -> DosConversion {
        self.next.get_dos_conversion()
    }
}

struct Geometry {
    // Geometry fields
}

struct DosConversion {
    // Dos conversion fields
}

fn open_offset(
    next: Box<dyn Stream>,
    dev: &mut Device,
    offset: i64,
    errmsg: &mut String,
    max_size: Option<&mut i64>,
) -> Result<Box<dyn Stream>, io::Error> {
    let mut this = Box::new(Offset {
        next,
        offset,
    });

    if let Some(max_size) = max_size {
        if this.offset > *max_size {
            if !errmsg.is_empty() {
                *errmsg = "init: Big disks not supported".to_string();
            }
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Offset too large"));
        }
        *max_size -= this.offset;
    }

    if adjust_total_sectors(dev, this.offset, errmsg) < 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Failed to adjust sectors"));
    }

    Ok(this)
}

fn adjust_total_sectors(dev: &mut Device, offset: i64, errmsg: &mut String) -> i32 {
    // Implementation of adjust_total_sectors
    -1
}

struct Device {
    // Device fields
}

impl Device {
    // Device methods
}