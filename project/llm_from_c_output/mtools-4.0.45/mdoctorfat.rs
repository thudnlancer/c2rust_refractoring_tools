/*
 *  Copyright 1999,2001,2002,2009 Alain Knaff.
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
 * Test program for doctoring the fat
 */

use std::env;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::process;

use clap::{App, Arg, ArgMatches};

struct ArgData {
    target: String,
    fat: u32,
    markbad: bool,
    setsize: bool,
    size: u32,
    offset: u32,
}

fn parse_number(number: &str) -> Result<(u32, u32), ParseIntError> {
    let mut parts = number.split('-');
    let begin = parts.next().unwrap().parse::<u32>()?;
    let end = parts.next().map_or(begin, |s| s.parse::<u32>().unwrap_or(begin));
    Ok((begin, end))
}

fn dos_doctorfat(entry: &mut Direntry, arg: &ArgData) -> io::Result<()> {
    if !arg.markbad && entry.is_root() {
        entry.dir.start = (arg.fat & 0xffff) as u16;
        entry.dir.start_hi = (arg.fat >> 16) as u16;
        if arg.setsize {
            entry.dir.size = arg.size;
        }
        entry.write()?;
    }
    Ok(())
}

fn unix_doctorfat() -> io::Result<()> {
    eprintln!("File does not reside on a Dos fs");
    Err(io::Error::new(io::ErrorKind::Other, "Not a DOS filesystem"))
}

fn usage() {
    eprintln!("Mtools version {}, dated {}", mversion(), mdate());
    eprintln!("Usage: [-b] {} file fat", progname());
    process::exit(1);
}

fn main() {
    let matches = App::new("mdoctorfat")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Modifies FAT entries")
        .arg(Arg::with_name("bad")
            .short('b')
            .help("Mark clusters as bad"))
        .arg(Arg::with_name("offset")
            .short('o')
            .takes_value(true)
            .help("Offset value"))
        .arg(Arg::with_name("size")
            .short('s')
            .takes_value(true)
            .help("Set file size"))
        .arg(Arg::with_name("FILE")
            .required(true)
            .index(1))
        .arg(Arg::with_name("FAT")
            .required(true)
            .index(2))
        .get_matches();

    let mut arg = ArgData {
        target: matches.value_of("FILE").unwrap().to_string(),
        fat: matches.value_of("FAT").unwrap().parse().unwrap(),
        markbad: matches.is_present("bad"),
        setsize: matches.is_present("size"),
        size: matches.value_of("size").map_or(0, |s| s.parse().unwrap()),
        offset: matches.value_of("offset").map_or(0, |s| s.parse().unwrap()),
    };

    let mut file = match OpenOptions::new().read(true).write(true).open(&arg.target) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    };

    let mut fs = match Fs::new(&mut file) {
        Ok(fs) => fs,
        Err(_) => {
            unix_doctorfat().unwrap_err();
            process::exit(1);
        }
    };

    let mut entry = match Direntry::new(&fs) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading directory entry: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = dos_doctorfat(&mut entry, &arg) {
        eprintln!("Error modifying FAT: {}", e);
        process::exit(1);
    }

    let numbers: Vec<&str> = matches.values_of("FAT").unwrap().collect();
    let mut address = 0;

    for number in numbers {
        let (begin, end) = match parse_number(number) {
            Ok((b, e)) => (b, e),
            Err(e) => {
                eprintln!("Invalid number format: {}", e);
                process::exit(1);
            }
        };

        for j in begin..=end {
            let cluster = j + arg.offset;
            if arg.markbad {
                fs.fat_encode(cluster, fs.last_fat() ^ 6 ^ 8);
            } else {
                if address != 0 {
                    fs.fat_encode(address, cluster);
                }
                address = cluster;
            }
        }
    }

    if address != 0 && !arg.markbad {
        fs.fat_encode(address, fs.end_fat());
    }
}

// Note: The following types and functions would need to be implemented
// based on the original C code's structures and functionality:
// - Direntry
// - Fs
// - mversion()
// - mdate()
// - progname()
// - The FAT encoding/decoding logic