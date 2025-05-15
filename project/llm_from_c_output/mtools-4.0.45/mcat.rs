/*
 *  Copyright 1999-2003,2007,2009 Alain Knaff.
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
 * mcat.rs
 * Same thing as cat /dev/fd0 or cat file >/dev/fd0
 * Something, that isn't possible with floppyd anymore.
 */

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process;

const BUF_SIZE: usize = if cfg!(target_os = "cygwin") { 512 } else { 16000 };

fn usage() -> ! {
    eprintln!("Mtools version {}, dated {}", mversion(), mdate());
    eprintln!("Usage: mcat [-V] [-w] device");
    eprintln!("       -w write on device else read");
    process::exit(1);
}

fn buf_len(blocksize: usize, total_size: u64, address: u64) -> usize {
    if total_size == 0 {
        blocksize
    } else if (blocksize as u64) > total_size - address {
        (total_size - address) as usize
    } else {
        blocksize
    }
}

fn mcat() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut mode = std::fs::OpenOptions::new().read(true);
    let mut image = None;
    let mut optind = 1;

    while optind < args.len() {
        let arg = &args[optind];
        if arg == "-w" {
            mode = std::fs::OpenOptions::new().write(true);
            optind += 1;
        } else if arg == "-i" {
            if optind + 1 >= args.len() {
                usage();
            }
            image = Some(args[optind + 1].clone());
            optind += 2;
        } else if arg.starts_with('-') {
            usage();
        } else {
            break;
        }
    }

    if args.len() - optind > 1 {
        usage();
    }

    let drive = if args.len() - optind == 1 {
        let device = &args[optind];
        if device.is_empty() || !device.ends_with(':') {
            usage();
        }
        device.chars().next().unwrap().to_ascii_uppercase()
    } else {
        get_default_drive()
    };

    let device_path = format!("{}:", drive);
    let mut stream = match OpenOptions::new()
        .read(mode.read)
        .write(mode.write)
        .open(&device_path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Drive '{}' not supported: {}", device_path, e);
            process::exit(1);
        }
    };

    let mut address = 0u64;
    let mut max_size = 0u64;

    if mode.write {
        let metadata = stream.metadata()?;
        max_size = metadata.len();
        let mut stdin = io::stdin();
        let mut buf = [0u8; BUF_SIZE];

        loop {
            let len = buf_len(BUF_SIZE, max_size, address);
            let bytes_read = stdin.read(&mut buf[..len])?;
            if bytes_read == 0 {
                break;
            }
            stream.write_all(&buf[..bytes_read])?;
            eprintln!("Wrote to {}", address);
            address += bytes_read as u64;
        }
    } else {
        let mut stdout = io::stdout();
        let mut buf = [0u8; BUF_SIZE];

        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {
                break;
            }
            stdout.write_all(&buf[..bytes_read])?;
            address += bytes_read as u64;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = mcat() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

// Placeholder functions - these would need to be implemented
fn mversion() -> &'static str {
    "unknown"
}

fn mdate() -> &'static str {
    "unknown"
}

fn get_default_drive() -> char {
    'A'
}