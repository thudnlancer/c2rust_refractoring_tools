/* set-acl.rs - set access control list equivalent to a mode

   Copyright (C) 2002-2003, 2005-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.

   Written by Paul Eggert and Andreas Gruenbacher, and Bruno Haible.  */

use std::os::unix::fs::PermissionsExt;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

use gettextrs::gettext;
use quote::quote;

/* Set the access control lists of a file. If DESC is a valid file
   descriptor, use file descriptor operations where available, else use
   filename based operations on NAME.  If access control lists are not
   available, fchmod the target file to MODE.  Also sets the
   non-permission bits of the destination file (S_ISUID, S_ISGID, S_ISVTX)
   to those from MODE if any are set.
   Return Ok(()) if successful.  On failure, output a diagnostic and
   return an io::Error.  */

pub fn set_acl(name: &Path, desc: Option<&File>, mode: u32) -> io::Result<()> {
    match qset_acl(name, desc, mode) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{}", gettext(&format!("setting permissions for {}", quote(name.to_str().unwrap_or("")))));
            Err(e)
        }
    }
}

fn qset_acl(name: &Path, desc: Option<&File>, mode: u32) -> io::Result<()> {
    // Implementation of qset_acl would go here
    // For now, we'll fall back to simple permission setting
    if let Some(file) = desc {
        file.set_permissions(std::fs::Permissions::from_mode(mode))
    } else {
        std::fs::set_permissions(name, std::fs::Permissions::from_mode(mode))
    }
}