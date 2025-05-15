/*  Copyright 1986-1992 Emmet P. Gray.
 *  Copyright 1994,1996-2002,2007-2009,2021-2022 Alain Knaff.
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
 * mcopy.rs
 * Copy an MSDOS files to and from Unix
 */

use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{self, time_t, stat, S_ISDIR, S_ISREG};
use nix::sys::stat::{fstat, stat};
use nix::fcntl::{open, OFlag};
use nix::sys::signal;
use nix::unistd;
use crate::mtools::{MainParam, ClashHandling, Direntry, Stream, DosName, get_time_now, print_oom};
use crate::utils::{copy_file, ask_confirmation, file_too_big};

#[derive(Debug)]
struct Arg {
    recursive: bool,
    preserve_attributes: bool,
    preserve_time: bool,
    attr: u8,
    path: Option<String>,
    textmode: bool,
    needfilter: bool,
    nowarn: bool,
    verbose: bool,
    file_type: bool,
    convert_charset: bool,
    mp: MainParam,
    ch: ClashHandling,
    no_clobber: bool,
    unix_target: Option<String>,
}

fn set_mtime(target: &str, mtime: time_t) -> io::Result<()> {
    if target != "-" && mtime != 0 {
        let times = [
            libc::timeval {
                tv_sec: mtime,
                tv_usec: 0,
            },
            libc::timeval {
                tv_sec: mtime,
                tv_usec: 0,
            },
        ];
        unsafe {
            libc::utimes(CString::new(target)?.as_ptr(), times.as_ptr());
        }
    }
    Ok(())
}

fn build_unix_filename(arg: &Arg, target: &str) -> Option<String> {
    let mut ret = PathBuf::from(arg.unix_target.as_ref()?);
    ret.push(target);
    Some(ret.to_string_lossy().into_owned())
}

fn unix_is_dir(name: &str) -> io::Result<bool> {
    let metadata = fs::metadata(name)?;
    Ok(metadata.is_dir())
}

fn unix_target_lookup(arg: &mut Arg, input: &str) -> io::Result<bool> {
    arg.unix_target = Some(input.to_string());
    
    match fs::metadata(input) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            if let Some(pos) = input.rfind('/') {
                let (dir, file) = input.split_at(pos);
                arg.unix_target = Some(dir.to_string());
                arg.mp.target_name = Some(file[1..].to_string());
                Ok(true)
            } else {
                arg.mp.target_name = Some(input.to_string());
                arg.unix_target = Some(".".to_string());
                Ok(true)
            }
        }
        Err(e) => Err(e),
    }
}

fn target_lookup(arg: &mut Arg, input: &str) -> io::Result<bool> {
    if input.len() > 1 && input.chars().nth(1) == Some(':') {
        // DOS target lookup
        unimplemented!("DOS target lookup not implemented")
    } else {
        unix_target_lookup(arg, input)
    }
}

fn mt_unix_write(mp: &MainParam, needfilter: bool, unix_file: &str) -> io::Result<()> {
    let arg = unsafe { &mut *(mp.arg as *mut Arg) };
    let mtime = mp.file.get_data()?.mtime;
    
    if !arg.preserve_time {
        mtime = 0;
    }

    if !arg.file_type {
        if !arg.nowarn && Path::new(unix_file).exists() {
            if arg.no_clobber {
                eprintln!("File \"{}\" exists. To overwrite, try again, and explicitly specify target directory", unix_file);
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "File exists"));
            }

            let metadata = fs::metadata(unix_file)?;
            if !metadata.is_file() {
                eprintln!("\"{}\" is not a regular file", unix_file);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not a regular file"));
            }

            if ask_confirmation(&format!("File \"{}\" exists, overwrite (y/n)? ", unix_file)) {
                return Ok(());
            }
        }

        if arg.verbose {
            eprint!("Copying ");
            mp.print_filename();
            eprintln!();
        }
    }

    if signal::kill(unistd::getpid(), None).is_err() {
        return Err(io::Error::new(io::ErrorKind::Interrupted, "Signal received"));
    }

    let mut target = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(unix_file)?;

    let mut source = mp.file.copy()?;
    if needfilter && arg.textmode {
        source = open_dos2unix(source, arg.convert_charset)?;
    }

    copy_file(&mut source, &mut target)?;
    
    if !arg.file_type {
        set_mtime(unix_file, mtime)?;
    }

    Ok(())
}

fn unix_write(mp: &MainParam, needfilter: bool) -> io::Result<()> {
    let arg = unsafe { &mut *(mp.arg as *mut Arg) };
    
    if arg.file_type {
        mt_unix_write(mp, needfilter, "-")
    } else {
        let unix_file = build_unix_filename(arg, &mp.pick_target_name())
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to build filename"))?;
        
        let result = mt_unix_write(mp, needfilter, &unix_file);
        if result.is_err() && !arg.file_type {
            fs::remove_file(unix_file).ok();
        }
        result
    }
}

fn make_unix_dir(filename: &str) -> io::Result<()> {
    fs::create_dir(filename).or_else(|e| {
        if e.kind() == io::ErrorKind::AlreadyExists {
            let metadata = fs::metadata(filename)?;
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::AlreadyExists, "Not a directory"))
            }
        } else {
            Err(e)
        }
    })
}

fn unix_copydir(entry: &Direntry, mp: &MainParam) -> io::Result<()> {
    let arg = unsafe { &mut *(mp.arg as *mut Arg) };
    
    if !arg.recursive && mp.basename_has_wildcard {
        return Ok(());
    }

    let mtime = mp.file.get_data()?.mtime;
    if !arg.preserve_time {
        mtime = 0;
    }

    if !arg.file_type && arg.verbose {
        eprint!("Copying ");
        entry.print_pwd();
        eprintln!();
    }

    if signal::kill(unistd::getpid(), None).is_err() {
        return Err(io::Error::new(io::ErrorKind::Interrupted, "Signal received"));
    }

    let unix_file = build_unix_filename(arg, &mp.pick_target_name())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to build filename"))?;

    if arg.file_type || make_unix_dir(&unix_file).is_ok() {
        let mut new_arg = arg.clone();
        new_arg.mp.arg = &mut new_arg as *mut _ as *mut libc::c_void;
        new_arg.unix_target = Some(unix_file.clone());
        new_arg.mp.target_name = None;
        new_arg.mp.basename_has_wildcard = true;

        let result = mp.loop_func(&mp.file, &new_arg.mp, "*");
        set_mtime(&unix_file, mtime)?;
        result
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("Failed to create directory {}: {}", unix_file, io::Error::last_os_error())))
    }
}

fn dos_to_unix(_entry: Option<&Direntry>, mp: &MainParam) -> io::Result<()> {
    unix_write(mp, true)
}

fn unix_to_unix(mp: &MainParam) -> io::Result<()> {
    unix_write(mp, false)
}

fn directory_dos_to_unix(entry: &Direntry, mp: &MainParam) -> io::Result<()> {
    unix_copydir(entry, mp)
}

// ... (remaining functions follow similar translation patterns)

fn mcopy(args: Vec<String>, file_type: bool) -> io::Result<()> {
    let mut arg = Arg {
        recursive: false,
        preserve_attributes: false,
        preserve_time: false,
        attr: 0,
        path: None,
        textmode: false,
        needfilter: false,
        nowarn: false,
        verbose: false,
        file_type,
        convert_charset: false,
        mp: MainParam::new(),
        ch: ClashHandling::new(),
        no_clobber: false,
        unix_target: None,
    };

    // Parse command line arguments
    // ... (argument parsing logic)

    if args.len() < 1 {
        eprintln!("Usage: mcopy [options] sourcefile targetfile");
        eprintln!("       mcopy [options] sourcefile [sourcefiles...] targetdirectory");
        return Ok(());
    }

    if file_type {
        arg.mp.target_name = Some("-".to_string());
        arg.unix_target = Some("".to_string());
        arg.mp.callback = dos_to_unix;
        arg.mp.dir_callback = unix_copydir;
        arg.mp.unix_callback = unix_to_unix;
    } else {
        let target = if args.len() == 1 {
            arg.no_clobber = true;
            "."
        } else {
            &args[args.len() - 1]
        };

        if !target_lookup(&mut arg, target)? {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Target not found"));
        }

        if arg.unix_target.is_some() {
            arg.mp.callback = dos_to_unix;
            arg.mp.dir_callback = directory_dos_to_unix;
            arg.mp.unix_callback = unix_to_unix;
        } else {
            arg.mp.dir_callback = dos_copydir;
            arg.mp.callback = dos_to_dos;
            arg.mp.unix_callback = unix_to_dos;
        }
    }

    main_loop(&arg.mp, &args)
}