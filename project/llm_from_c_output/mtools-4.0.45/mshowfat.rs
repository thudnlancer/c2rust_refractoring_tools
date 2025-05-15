/*
 *  Copyright 1997,2000-2002,2009,2011 Alain Knaff.
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
 * mcopy.c
 * Copy an MSDOS files to and from Unix
 *
 */

use std::env;
use std::ffi::OsString;
use std::io::{self, Write};
use std::process;
use std::str::FromStr;
use getopts::Options;

struct Arg {
    mp: MainParam,
    offset: i64,
}

fn dos_showfat(entry: &direntry_t, mp: &MainParam) -> Result<i32, io::Error> {
    let file = &mp.File;
    let arg = unsafe { &*(mp.arg as *const Arg) };
    
    fprint_pwd(io::stdout(), entry, 0)?;
    print!(" ");
    
    if arg.offset == -1 {
        print_fat(file)?;
    } else {
        print_fat_with_offset(file, arg.offset)?;
    }
    
    println!();
    Ok(GOT_ONE)
}

fn unix_showfat(_mp: &MainParam) -> Result<i32, io::Error> {
    eprintln!("File does not reside on a Dos fs");
    Ok(ERROR_ONE)
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", mversion, mdate);
    eprintln!("Usage: {} files", progname);
    process::exit(ret);
}

fn mshowfat(args: &[OsString]) -> ! {
    let mut arg = Arg {
        mp: MainParam::new(),
        offset: -1,
    };
    
    let mut opts = Options::new();
    opts.optopt("o", "", "offset", "OFFSET");
    opts.optopt("i", "", "image", "IMAGE");
    opts.optflag("h", "help", "display this help");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            usage(1);
        }
    };
    
    if matches.opt_present("h") {
        usage(0);
    }
    
    if let Some(offset) = matches.opt_str("o") {
        arg.offset = match str_to_offset(&offset) {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Invalid offset: {}", e);
                usage(1);
            }
        };
    }
    
    if let Some(image) = matches.opt_str("i") {
        set_cmd_line_image(&image);
    }
    
    if matches.free.is_empty() {
        usage(1);
    }
    
    arg.mp.arg = &arg as *const _ as *mut _;
    arg.mp.callback = dos_showfat;
    arg.mp.unixcallback = unix_showfat;
    arg.mp.lookupflags = ACCEPT_PLAIN | ACCEPT_DIR | DO_OPEN;
    
    let ret = match main_loop(&arg.mp, &matches.free) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            ERROR_ONE
        }
    };
    
    process::exit(ret);
}

// Note: The following types and functions would need to be defined elsewhere in the Rust codebase:
// - MainParam, direntry_t, GOT_ONE, ERROR_ONE, ACCEPT_PLAIN, ACCEPT_DIR, DO_OPEN
// - fprint_pwd, print_fat, print_fat_with_offset, str_to_offset, set_cmd_line_image, main_loop
// - mversion, mdate, progname