/* Invoke openat, but avoid some glitches.

   Copyright (C) 2005-2006, 2008-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert for open, ported by Eric Blake for openat.  */

use std::os::unix::prelude::*;
use std::fs::{OpenOptions, File};
use std::path::Path;
use nix::fcntl::{openat, OFlag};
use nix::sys::stat::Mode;
use nix::dir::Dir;
use crate::unistd_safer::fd_safer;

pub fn openat_safer(fd: i32, file: &str, flags: OFlag, mode: Option<Mode>) -> std::io::Result<File> {
    let mode = mode.unwrap_or_else(|| Mode::empty());
    
    let fd = openat(fd, Path::new(file), flags, mode)
        .map_err(|e| std::io::Error::from_raw_os_error(e as i32))?;
    
    fd_safer(fd)
}