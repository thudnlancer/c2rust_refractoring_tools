/* qset-acl.c - set access control list equivalent to a mode

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

use std::ffi::CStr;
use std::os::unix::fs::PermissionsExt;
use std::fs::{File, Permissions};
use std::io;

#[derive(Default)]
struct PermissionContext {
    mode: u32,
}

impl PermissionContext {
    fn free(&mut self) {
        // No resources to free in Rust version
    }
}

/// Set the access control lists of a file. If `desc` is a valid file
/// descriptor, use file descriptor operations where available, else use
/// filename based operations on `name`. If access control lists are not
/// available, fchmod the target file to `mode`. Also sets the
/// non-permission bits of the destination file (S_ISUID, S_ISGID, S_ISVTX)
/// to those from `mode` if any are set.
/// Returns Ok(()) if successful. Returns Err(io::Error) upon failure.
pub fn qset_acl(name: &CStr, desc: Option<&File>, mode: u32) -> io::Result<()> {
    let mut ctx = PermissionContext { mode };
    let result = set_permissions(&ctx, name, desc);
    ctx.free();
    result
}

fn set_permissions(ctx: &PermissionContext, name: &CStr, desc: Option<&File>) -> io::Result<()> {
    // In Rust, we would typically use platform-specific crates for ACL operations
    // For this translation, we'll fall back to simple permission setting
    let permissions = Permissions::from_mode(ctx.mode);
    
    match desc {
        Some(file) => file.set_permissions(permissions),
        None => std::fs::set_permissions(name.to_str()?, permissions),
    }
}