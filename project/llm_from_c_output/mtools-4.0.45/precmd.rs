/*  Copyright 1997,1999,2001-2004,2007,2009 Alain Knaff.
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
 * Do filename expansion with the shell.
 */

use std::process::{Command, exit};
use std::os::unix::process::CommandExt;
use std::ffi::CString;

pub struct Device {
    precmd: Option<String>,
}

pub fn precmd(dev: Option<&Device>) {
    if let Some(dev) = dev {
        postcmd(dev.precmd.as_deref());
    }
}

pub fn postcmd(cmd: Option<&str>) {
    #[cfg(not(target_os = "windows"))]
    {
        if let Some(cmd) = cmd {
            match unsafe { libc::fork() } {
                -1 => {
                    eprintln!("Could not fork");
                    exit(1);
                }
                0 => { // child process
                    let _ = Command::new("/bin/sh")
                        .arg("-c")
                        .arg(cmd)
                        .exec();
                }
                _ => { // parent process
                    let mut status = 0;
                    unsafe { libc::wait(&mut status) };
                }
            }
        }
    }
}