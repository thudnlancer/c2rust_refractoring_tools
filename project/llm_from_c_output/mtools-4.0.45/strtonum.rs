/*  Copyright 2018 Alain Knaff.
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
 */

use std::num::{ParseIntError, ParseFloatError};
use std::str::FromStr;
use std::process;

fn strtol_with_range(nptr: &str, base: u32, min: i64, max: i64) -> Result<i64, ParseIntError> {
    let l = i64::from_str_radix(nptr, base)?;
    if l > max {
        return Ok(max);
    }
    if l < min {
        return Ok(min);
    }
    Ok(l)
}

fn strtoul_with_range(nptr: &str, base: u32, max: u64) -> Result<u64, ParseIntError> {
    let l = u64::from_str_radix(nptr, base)?;
    if l > max {
        Ok(max)
    } else {
        Ok(l)
    }
}

pub fn strtoui(nptr: &str, base: u32) -> Result<u32, ParseIntError> {
    strtoul_with_range(nptr, base, u32::MAX as u64).map(|v| v as u32)
}

pub fn atoui(str: &str) -> Result<u32, ParseIntError> {
    strtoui(str, 10)
}

pub fn strtosi(nptr: &str, base: u32) -> Result<i32, ParseIntError> {
    strtol_with_range(nptr, base, i32::MIN as i64, i32::MAX as i64).map(|v| v as i32)
}

pub fn atoul(str: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(str, 10)
}

pub fn strtou8(nptr: &str, base: u32) -> Result<u8, ParseIntError> {
    strtoul_with_range(nptr, base, u8::MAX as u64).map(|v| v as u8)
}

pub fn atou8(str: &str) -> Result<u8, ParseIntError> {
    strtou8(str, 10)
}

pub fn strtou16(nptr: &str, base: u32) -> Result<u16, ParseIntError> {
    strtoul_with_range(nptr, base, u16::MAX as u64).map(|v| v as u16)
}

pub fn atou16(str: &str) -> Result<u16, ParseIntError> {
    strtou16(str, 10)
}

pub fn strtou32(nptr: &str, base: u32) -> Result<u32, ParseIntError> {
    strtoul_with_range(nptr, base, u32::MAX as u64).map(|v| v as u32)
}

pub fn atou32(str: &str) -> Result<u32, ParseIntError> {
    strtou32(str, 10)
}

fn check_overflow(tot_sectors: u32, bits: u32) {
    if tot_sectors > u32::MAX >> bits {
        eprintln!("Too many sectors");
        process::exit(1);
    }
}

/**
 * Parses a 32bit number of sectors.
 * Two similar functions exist in misc.c (str_to_offset_with_end and
 * str_to_off_with_end, which return a seekable number of bytes instead)
 */
pub fn parse_size(size_str: &str) -> u32 {
    let (num_part, suffix) = match size_str.find(|c: char| !c.is_digit(10)) {
        Some(pos) => size_str.split_at(pos),
        None => (size_str, ""),
    };

    let mut tot_sectors = match strtou32(num_part, 10) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Bad size {}", size_str);
            process::exit(1);
        }
    };

    let mut chars = suffix.chars();
    match chars.next() {
        Some(c) => match c.to_ascii_uppercase() {
            'T' => {
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
            }
            'G' => {
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
            }
            'M' => {
                check_overflow(tot_sectors, 10);
                tot_sectors *= 1024;
            }
            'K' => {
                check_overflow(tot_sectors, 1);
                tot_sectors *= 2;
            }
            'S' => (),
            _ => {
                eprintln!("Bad suffix {}", suffix);
                process::exit(1);
            }
        },
        None => (),
    }

    if !chars.as_str().is_empty() {
        eprintln!("Bad suffix {}", suffix);
        process::exit(1);
    }

    tot_sectors
}