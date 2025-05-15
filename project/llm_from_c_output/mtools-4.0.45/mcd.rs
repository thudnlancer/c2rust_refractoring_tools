/*  Copyright 1986-1992 Emmet P. Gray.
 *  Copyright 1996,1997,2000-2002,2009 Alain Knaff.
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
 * mcd.c: Change MSDOS directories
 */

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::process;
use std::path::PathBuf;
use getopts::Options;

#[derive(Debug)]
enum MCDError {
    IoError(io::Error),
    OtherError(&'static str),
}

impl From<io::Error> for MCDError {
    fn from(err: io::Error) -> Self {
        MCDError::IoError(err)
    }
}

fn mcd_callback(entry: &direntry_t, mp: &MainParam) -> Result<u32, MCDError> {
    let mut fp = match open_mcwd("w") {
        Some(file) => file,
        None => {
            eprintln!("mcd: Can't open mcwd .file for writing");
            return Err(MCDError::OtherError("Failed to open mcwd file"));
        }
    };

    fprint_pwd(&mut fp, entry, 0)?;
    writeln!(fp)?;
    Ok(GOT_ONE | STOP_NOW)
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", mversion, mdate);
    eprintln!("Usage: {}: [-V] [-i image] msdosdirectory", progname);
    process::exit(ret);
}

fn mcd(args: Vec<String>) -> Result<(), MCDError> {
    let mut opts = Options::new();
    opts.optopt("i", "", "set image file", "IMAGE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f);
            usage(1);
        }
    };

    if matches.opt_present("h") {
        usage(0);
    }

    if let Some(img) = matches.opt_str("i") {
        set_cmd_line_image(&img);
    }

    if matches.free.len() > 1 {
        usage(1);
    }

    let mut mp = MainParam::new();
    mp.lookupflags = ACCEPT_DIR | NO_DOTS;
    mp.dir_callback = mcd_callback;

    if args.len() == 1 {
        println!("{}", mp.mcwd);
        process::exit(0);
    } else {
        let exit_code = main_loop(&mp, &matches.free, 1)?;
        process::exit(exit_code);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Err(e) = mcd(args) {
        match e {
            MCDError::IoError(err) => eprintln!("IO error: {}", err),
            MCDError::OtherError(msg) => eprintln!("Error: {}", msg),
        }
        process::exit(1);
    }
}