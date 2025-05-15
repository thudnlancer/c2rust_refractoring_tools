/*
 *  Copyright 1994,1996-2002,2005-2007,2009 Alain Knaff.
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
 * Mount an MSDOS disk
 *
 * written by:
 *
 * Alain L. Knaff
 * alain@knaff.lu
 */

use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::os::unix::process::CommandExt;
use std::process::{Command, exit, Stdio};
use std::str;
use libc::{O_RDONLY, O_RDWR, O_LARGEFILE};
use nix::unistd::{fork, ForkResult, execvp, close};
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus;

#[cfg(target_os = "linux")]
pub fn mmount(args: &[String]) -> ! {
    if args.len() < 2 || args[1].len() != 2 || !args[1].ends_with(':') {
        eprintln!("Usage: {} -V drive:", args[0]);
        exit(1);
    }

    let drive = args[1].chars().next().unwrap().to_ascii_uppercase();
    
    // Placeholder for find_device functionality
    // let stream = match find_device(drive, O_RDONLY, &mut dev, &mut boot, &mut name, &mut media, 0, None) {
    //     Some(s) => s,
    //     None => exit(1),
    // };
    // drop(stream);
    
    // Placeholder for destroy_privs
    // destroy_privs();

    let mut name = String::new(); // Should be populated by find_device
    // if dev.partition != 0 {
    //     name.push_str(&format!("{}", dev.partition % 1000));
    // }

    match fork() {
        Ok(ForkResult::Parent { child }) => {
            match waitpid(child, None) {
                Ok(WaitStatus::Exited(_, status)) => {
                    if status == 0 {
                        exit(0);
                    }
                }
                _ => exit(1),
            }

            let mount_args = if args.len() > 2 {
                let mut new_args = vec![
                    CString::new("mount").unwrap(),
                    CString::new("-r").unwrap(),
                ];
                new_args.extend(args[2..].iter().map(|a| CString::new(a.as_str()).unwrap()));
                new_args
            } else {
                vec![
                    CString::new("mount").unwrap(),
                    CString::new("-r").unwrap(),
                    CString::new(name).unwrap(),
                ]
            };

            if let Err(e) = execvp(&CString::new("mount").unwrap(), &mount_args) {
                eprintln!("exec mount failed: {}", e);
                exit(1);
            }
        }
        Ok(ForkResult::Child) => {
            let _ = close(2);
            let _ = File::create("/dev/null").and_then(|f| {
                OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("/dev/null")
            });

            let mount_args = if args.len() > 2 {
                let mut new_args = vec![CString::new("mount").unwrap()];
                new_args.extend(args[2..].iter().map(|a| CString::new(a.as_str()).unwrap()));
                new_args
            } else {
                vec![
                    CString::new("mount").unwrap(),
                    CString::new(name).unwrap(),
                ]
            };

            if let Err(e) = execvp(&CString::new("mount").unwrap(), &mount_args) {
                eprintln!("exec mount failed: {}", e);
                exit(1);
            }
        }
        Err(_) => {
            eprintln!("fork failed");
            exit(1);
        }
    }
    exit(1);
}

#[cfg(not(target_os = "linux"))]
pub fn mmount(_args: &[String]) -> ! {
    eprintln!("This command is only available for LINUX");
    exit(1);
}