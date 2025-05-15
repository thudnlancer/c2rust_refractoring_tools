/*
 * Copyright 2021 Alain Knaff.
 * This file is part of mtools.
 *
 * Mtools is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Mtools is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::io::{self, Write};
use std::fmt;
use std::error::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapType {
    Data,
    Zero,
    Skip,
    Pos,
}

#[derive(Debug, Clone, Copy)]
struct Map {
    orig: u64,
    remapped: u64,
    type_: MapType,
}

struct Remap {
    next: Box<dyn Stream>,
    map: Vec<Map>,
    net_offset: u64,
}

impl Remap {
    fn remap(&self, start: &mut u64, len: &mut usize) -> MapType {
        for i in 0..self.map.len() - 1 {
            if *start < self.map[i + 1].remapped {
                *len = (*len).min((self.map[i + 1].remapped - *start) as usize);
                *start = *start - self.map[i].remapped + self.map[i].orig;
                return self.map[i].type_;
            }
        }
        *start = *start - self.map.last().unwrap().remapped + self.map.last().unwrap().orig;
        self.map.last().unwrap().type_
    }
}

impl Stream for Remap {
    fn pread(&self, buf: &mut [u8], start: u64) -> io::Result<usize> {
        let mut start = start;
        let mut len = buf.len();
        match self.remap(&mut start, &mut len) {
            MapType::Data => self.next.pread(&mut buf[..len], start),
            _ => {
                buf[..len].fill(0);
                Ok(len)
            }
        }
    }

    fn pwrite(&mut self, buf: &[u8], start: u64) -> io::Result<usize> {
        let mut start = start;
        let mut len = buf.len();
        match self.remap(&mut start, &mut len) {
            MapType::Data => self.next.pwrite(&buf[..len], start),
            _ => {
                if buf[..len].iter().any(|&b| b != 0) {
                    eprintln!("Bad data written to unmapped sectors");
                    Err(io::Error::new(io::ErrorKind::Other, "EFAULT"))
                } else {
                    Ok(len)
                }
            }
        }
    }
}

fn process_map(map_str: &str) -> Result<(Vec<Map>, u64), Box<dyn Error>> {
    let mut map = Vec::new();
    let mut orig = 0;
    let mut remapped = 0;
    let mut ptr = map_str;
    let mut at_end = false;

    while !at_end {
        let (type_, new_ptr) = if ptr.is_empty() {
            at_end = true;
            (MapType::Data, ptr)
        } else if ptr.starts_with("skip") {
            (MapType::Skip, &ptr[4..])
        } else if ptr.starts_with("zero") {
            (MapType::Zero, &ptr[4..])
        } else if ptr.starts_with("pos") {
            (MapType::Pos, &ptr[3..])
        } else {
            (MapType::Data, ptr)
        };

        let (len, eptr) = if new_ptr.is_empty() {
            (0, new_ptr)
        } else {
            let len_str = new_ptr.split(',').next().unwrap();
            let len = u64::from_str(len_str)?;
            (len, &new_ptr[len_str.len()..])
        };

        ptr = if !eptr.is_empty() && eptr.starts_with(',') {
            &eptr[1..]
        } else {
            eptr
        };

        if type_ == MapType::Pos {
            orig = len;
            continue;
        }

        if type_ != MapType::Skip {
            map.push(Map {
                orig,
                remapped,
                type_,
            });
            remapped += len;
        }

        if type_ != MapType::Zero {
            orig += len;
        }
    }

    let net_offset = orig - remapped;
    Ok((map, net_offset))
}

pub fn remap(next: Box<dyn Stream>, dev: &Device, errmsg: &mut String) -> Result<Box<dyn Stream>, Box<dyn Error>> {
    let (map, net_offset) = process_map(dev.data_map)?;

    if adjust_tot_sectors(dev, net_offset, errmsg).is_err() {
        return Err("Failed to adjust total sectors".into());
    }

    Ok(Box::new(Remap {
        next,
        map,
        net_offset,
    }))
}

trait Stream {
    fn pread(&self, buf: &mut [u8], start: u64) -> io::Result<usize>;
    fn pwrite(&mut self, buf: &[u8], start: u64) -> io::Result<usize>;
}

struct Device {
    data_map: String,
    // Other device fields...
}

fn adjust_tot_sectors(dev: &Device, net_offset: u64, errmsg: &mut String) -> Result<(), Box<dyn Error>> {
    // Implementation of adjust_tot_sectors
    Ok(())
}