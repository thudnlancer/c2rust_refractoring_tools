/*  Copyright 2010 Alain Knaff.
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
 * mshortname.c
 * Change MSDOS file attribute flags
 */

use std::env;
use std::io::{self, Write};
use std::process;
use getopts::Options;

const GOT_ONE: i32 = 1;

struct MainParam {
    callback: fn(&Direntry, &MainParam) -> i32,
    arg: Option<Box<dyn std::any::Any>>,
    lookupflags: u32,
}

struct Direntry;

fn print_short_name(entry: &Direntry, _mp: &MainParam) -> i32 {
    // Assuming fprintShortPwd is implemented elsewhere
    // fprintShortPwd(stdout, entry);
    println!();
    GOT_ONE
}

fn usage(ret: i32) -> ! {
    let progname = env::args().next().unwrap_or_else(|| "mshortname".to_string());
    eprintln!("Mtools version {}, dated {}", mversion(), mdate());
    eprintln!("Usage: {} msdosfile [msdosfiles...]", progname);
    process::exit(ret);
}

fn mshortname(args: Vec<String>) -> ! {
    let mut opts = Options::new();
    opts.optopt("i", "", "set command line image", "IMAGE");
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

    if let Some(image) = matches.opt_str("i") {
        set_cmd_line_image(&image);
    }

    if matches.free.is_empty() {
        usage(0);
    }

    let mut mp = MainParam {
        callback: print_short_name,
        arg: None,
        lookupflags: ACCEPT_PLAIN | ACCEPT_DIR,
    };

    init_mp(&mut mp);
    let exit_code = main_loop(&mp, &matches.free);
    process::exit(exit_code);
}

// Placeholder for external functions that would be implemented elsewhere
fn mversion() -> &'static str { "" }
fn mdate() -> &'static str { "" }
fn set_cmd_line_image(_image: &str) {}
fn init_mp(_mp: &mut MainParam) {}
fn main_loop(_mp: &MainParam, _args: &[String]) -> i32 { 0 }
const ACCEPT_PLAIN: u32 = 0x1;
const ACCEPT_DIR: u32 = 0x2;