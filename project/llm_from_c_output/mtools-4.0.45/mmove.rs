/*
 * Copyright 1996-1998,2000-2002,2005,2007-2009 Alain Knaff.
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
 *
 * mmove.c
 * Renames/moves an MSDOS file
 */

use std::ffi::{CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::io::{self, Write, Error, ErrorKind};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::ptr;
use std::mem;
use std::convert::TryInto;
use libc::{O_RDWR, c_int, c_void};

use crate::sysincludes::*;
use crate::mtools::*;
use crate::mainloop::*;
use crate::plain_io::*;
use crate::nameclash::*;
use crate::file::*;
use crate::fs::*;

struct Arg {
    fromname: Option<String>,
    verbose: bool,
    mp: MainParam,
    entry: Option<Direntry>,
    ch: ClashHandling,
}

fn renameit(dosname: &DosName, longname: Option<&str>, arg0: &mut Arg, target_entry: &mut Direntry) -> Result<(), Error> {
    let arg = arg0;
    let fat: u32;

    target_entry.dir = arg.entry.as_ref().unwrap().dir.clone();
    dosname_to_direntry(dosname, &mut target_entry.dir)?;

    if is_dir(target_entry) {
        let moved_entry = get_direntry(&arg.mp.file)?;

        if moved_entry.dir != target_entry.dir {
            let mut sub_entry = Direntry::new();
            initialize_direntry(&mut sub_entry, &arg.mp.file)?;

            match vfat_lookup(&mut sub_entry, "..", 2, ACCEPT_DIR, None, 0, None, 0) {
                -1 => {
                    eprintln!(" Directory has no parent entry");
                },
                -2 => return Err(Error::new(ErrorKind::Other, "Error in vfat_lookup")),
                0 => {
                    get_data(&target_entry.dir, 0, 0, 0, &mut fat)?;
                    if fat == fat32_root_cluster(&target_entry.dir) {
                        fat = 0;
                    }

                    sub_entry.dir.start[1] = ((fat >> 8) & 0xff) as u8;
                    sub_entry.dir.start[0] = (fat & 0xff) as u8;
                    dir_write(&sub_entry)?;
                    if arg.verbose {
                        eprintln!("Easy, isn't it? I wonder why DOS can't do this.");
                    }
                },
                _ => unreachable!(),
            }

            wipe_entry(&moved_entry)?;

            let old_dir = moved_entry.dir.clone();
            *moved_entry = target_entry.clone();
            target_entry.dir = old_dir;
            return Ok(());
        }
    }

    wipe_entry(arg.mp.direntry.as_mut().unwrap())?;
    Ok(())
}

fn rename_file(entry: &mut Direntry, mp: &mut MainParam) -> Result<i32, Error> {
    let arg = unsafe { &mut *(mp.arg as *mut Arg) };
    arg.entry = Some(entry.clone());

    let target_dir = mp.target_dir.as_ref().unwrap();
    if target_dir == &entry.dir {
        arg.ch.ignore_entry = -1;
        arg.ch.source = entry.entry;
        arg.ch.source_entry = entry.entry;
    } else {
        arg.ch.ignore_entry = -1;
        arg.ch.source = -2;
    }

    let longname = mp_pick_target_name(mp)?;
    let shortname = None;
    let result = mwrite_one(target_dir, longname, shortname, renameit, arg as *mut Arg as *mut c_void, &mut arg.ch)?;

    if result == 1 {
        Ok(GOT_ONE)
    } else {
        Ok(ERROR_ONE)
    }
}

fn rename_directory(entry: &mut Direntry, mp: &mut MainParam) -> Result<i32, Error> {
    if is_subdir_of(&mp.target_dir.as_ref().unwrap(), &mp.file) {
        eprint!("Cannot move directory ");
        fprint_pwd(io::stderr(), entry, false)?;
        eprint!(" into one of its own subdirectories (");
        fprint_pwd(io::stderr(), &get_direntry(&mp.target_dir.as_ref().unwrap())?, false)?;
        eprintln!(")");
        return Ok(ERROR_ONE);
    }

    if is_root_entry(entry) {
        eprint!("Cannot move a root directory: ");
        fprint_pwd(io::stderr(), entry, false)?;
        return Ok(ERROR_ONE);
    }

    let ret = rename_file(entry, mp)?;
    if ret & ERROR_ONE != 0 {
        return Ok(ret);
    }

    Ok(ret)
}

fn rename_oldsyntax(entry: &mut Direntry, mp: &mut MainParam) -> Result<i32, Error> {
    let arg = unsafe { &mut *(mp.arg as *mut Arg) };
    arg.entry = Some(entry.clone());

    let target_dir = &entry.dir;
    arg.ch.ignore_entry = -1;
    arg.ch.source = entry.entry;
    arg.ch.source_entry = entry.entry;

    let longname = mp.target_name.as_ref().unwrap();
    let shortname = None;
    let result = mwrite_one(target_dir, longname, shortname, renameit, arg as *mut Arg as *mut c_void, &mut arg.ch)?;

    if result == 1 {
        Ok(GOT_ONE)
    } else {
        Ok(ERROR_ONE)
    }
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", mversion, mdate);
    eprintln!("Usage: {} [-vV] [-D clash_option] file targetfile", progname);
    eprintln!("       {} [-vV] [-D clash_option] file [files...] target_directory", progname);
    std::process::exit(ret);
}

pub fn mmove(argc: i32, argv: Vec<String>, oldsyntax: bool) -> ! {
    let mut arg = Arg {
        fromname: None,
        verbose: false,
        mp: MainParam::new(),
        entry: None,
        ch: ClashHandling::new(),
    };

    init_clash_handling(&mut arg.ch);

    if help_flag(argc, &argv) {
        usage(0);
    }

    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help message");
    opts.optopt("i", "", "set image file", "IMAGE");
    opts.optflag("v", "", "verbose output");
    opts.optflag("o", "", "overwrite existing files");
    opts.optopt("D", "", "clash handling option", "OPTION");

    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            usage(1);
        }
    };

    if matches.opt_present("h") {
        usage(0);
    }

    if let Some(img) = matches.opt_str("i") {
        set_cmd_line_image(&img);
    }

    arg.verbose = matches.opt_present("v");

    if matches.opt_present("o") {
        handle_clash_options(&mut arg.ch, 'o');
    }

    if let Some(opt) = matches.opt_str("D") {
        if handle_clash_options(&mut arg.ch, opt.chars().next().unwrap()) {
            usage(1);
        }
    }

    if matches.free.len() < 2 {
        usage(1);
    }

    init_mp(&mut arg.mp);
    arg.mp.arg = &mut arg as *mut Arg as *mut c_void;
    arg.mp.openflags = O_RDWR;

    let mut def_drive = None;
    for file in &matches.free {
        if file.len() > 1 && file.chars().nth(1).unwrap() == ':' {
            let drive = file.chars().next().unwrap().to_ascii_uppercase();
            if let Some(d) = def_drive {
                if d != drive {
                    eprintln!("Cannot move files across different drives");
                    std::process::exit(1);
                }
            } else {
                def_drive = Some(drive);
            }
        }
    }

    if let Some(drive) = def_drive {
        arg.mp.mcwd[0] = drive;
    }

    let is_oldsyntax = oldsyntax && (matches.free.len() == 2 && !matches.free[1].contains(|c| c == ':' || c == '/'));
    
    arg.mp.lookupflags = ACCEPT_PLAIN | ACCEPT_DIR | DO_OPEN_DIRS | NO_DOTS;

    if !is_oldsyntax {
        dos_target_lookup(&mut arg.mp, &matches.free[matches.free.len() - 1]);
        arg.mp.callback = Some(rename_file);
        arg.mp.dir_callback = Some(rename_directory);
    } else {
        arg.fromname = Some(matches.free[0].clone());
        if arg.fromname.as_ref().unwrap().len() > 1 && arg.fromname.as_ref().unwrap().chars().nth(1).unwrap() == ':' {
            arg.fromname = Some(arg.fromname.unwrap()[2..].to_string());
        }
        arg.fromname = Some(mt_basename(&arg.fromname.unwrap()));
        arg.mp.target_name = Some(matches.free[1].clone());
        arg.mp.callback = Some(rename_oldsyntax);
    }

    let mut longname = [0; 4 * MAX_VNAMELEN + 1];
    longname[0] = 0;
    arg.mp.longname.data = longname.as_mut_ptr();
    arg.mp.longname.len = longname.len();

    let mut shortname = [0; 12 * 4 + 1];
    shortname[0] = 0;
    arg.mp.shortname.data = shortname.as_mut_ptr();
    arg.mp.shortname.len = shortname.len();

    let exit_code = main_loop(&mut arg.mp, &matches.free[..matches.free.len() - 1]);
    std::process::exit(exit_code);
}