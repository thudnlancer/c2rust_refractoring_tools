/*
 *  Copyright 1997,2000-2002,2009 Alain Knaff.
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
 * mdu.rs:
 * Display the space occupied by an MSDOS directory (Rust implementation)
 */

use std::ffi::CString;
use std::os::unix::prelude::*;
use std::path::Path;
use std::process;
use std::io::{self, Write};
use std::env;

use mtools::{MainParam, MainLoopCallback, Direntry, count_blocks, get_start, is_root_dir, fprint_pwd};
use clap::{App, Arg, ArgMatches};

struct MduArgs {
    all: bool,
    in_dir: bool,
    summary: bool,
    parent: Option<Box<MduArgs>>,
    target: Option<String>,
    path: Option<String>,
    blocks: u32,
    mp: MainParam,
}

fn usage(ret: i32) -> ! {
    let progname = env::args().next().unwrap_or_else(|| "mdu".to_string());
    writeln!(io::stderr(), "Mtools version {}, dated {}", mtools::MVERSION, mtools::MDATE).unwrap();
    writeln!(io::stderr(), "Usage: {}: msdosdirectory", progname).unwrap();
    process::exit(ret);
}

fn file_mdu(entry: &Direntry, mp: &MainParam) -> i32 {
    let arg = unsafe { &mut *(mp.arg as *mut MduArgs) };
    let blocks = count_blocks(entry.dir, get_start(entry.dir, &entry.dir_entry));
    
    if arg.all || !arg.in_dir {
        fprint_pwd(io::stdout(), entry, false).unwrap();
        println!(" {}", blocks);
    }
    arg.blocks += blocks;
    mtools::GOT_ONE
}

fn dir_mdu(entry: &Direntry, mp: &MainParam) -> i32 {
    let parent_arg = unsafe { &mut *(mp.arg as *mut MduArgs) };
    let mut arg = MduArgs {
        all: parent_arg.all,
        in_dir: true,
        summary: parent_arg.summary,
        parent: Some(Box::new(unsafe { std::ptr::read(parent_arg) })),
        target: parent_arg.target.clone(),
        path: parent_arg.path.clone(),
        blocks: 0,
        mp: MainParam {
            callback: parent_arg.mp.callback,
            dir_callback: parent_arg.mp.dir_callback,
            open_flags: parent_arg.mp.open_flags,
            lookup_flags: parent_arg.mp.lookup_flags,
            arg: std::ptr::null_mut(),
        },
    };
    
    arg.mp.arg = &mut arg as *mut MduArgs as *mut libc::c_void;

    if !is_root_dir(entry.dir) {
        arg.blocks = count_blocks(entry.dir, get_start(entry.dir, &entry.dir_entry));
    } else {
        arg.blocks = 0;
    }

    let ret = mp.loop_fn(mp.file, &arg.mp, "*");
    if !arg.summary || !parent_arg.in_dir {
        fprint_pwd(io::stdout(), entry, false).unwrap();
        println!(" {}", arg.blocks);
    }
    parent_arg.blocks += arg.blocks;
    ret
}

fn mdu(argc: i32, argv: *mut *mut libc::c_char, _type: i32) -> ! {
    let mut arg = MduArgs {
        all: false,
        in_dir: false,
        summary: false,
        parent: None,
        target: None,
        path: None,
        blocks: 0,
        mp: MainParam::new(),
    };

    let args: Vec<String> = unsafe {
        (0..argc)
            .map(|i| CString::from_raw(*argv.offset(i as isize)).into_string().unwrap())
            .collect()
    };

    let matches = App::new("mdu")
        .version(mtools::MVERSION)
        .about("Display the space occupied by an MSDOS directory")
        .arg(Arg::with_name("all")
            .short('a')
            .help("Show all files"))
        .arg(Arg::with_name("summary")
            .short('s')
            .help("Show only summary"))
        .arg(Arg::with_name("image")
            .short('i')
            .takes_value(true)
            .help("Set disk image"))
        .arg(Arg::with_name("help")
            .short('h')
            .help("Show help"))
        .get_matches_from(args);

    if matches.is_present("help") {
        usage(0);
    }

    arg.all = matches.is_present("all");
    arg.summary = matches.is_present("summary");

    if arg.summary && arg.all {
        writeln!(io::stderr(), "-a and -s options are mutually exclusive").unwrap();
        usage(1);
    }

    if let Some(image) = matches.value_of("image") {
        mtools::set_cmd_line_image(image);
    }

    arg.mp.callback = file_mdu;
    arg.mp.open_flags = libc::O_RDONLY;
    arg.mp.dir_callback = dir_mdu;
    arg.mp.arg = &mut arg as *mut MduArgs as *mut libc::c_void;
    arg.mp.lookup_flags = mtools::ACCEPT_PLAIN | mtools::ACCEPT_DIR | mtools::DO_OPEN_DIRS | mtools::NO_DOTS;

    let remaining_args: Vec<String> = matches.values_of("").unwrap_or_default().map(|s| s.to_string()).collect();
    let exit_code = mtools::main_loop(&arg.mp, &remaining_args);
    process::exit(exit_code);
}