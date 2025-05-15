// unlinkdir.rs - determine whether we can unlink directories
//
// Copyright (C) 2005-2006, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert, Jim Meyering, and David Bartley.

use std::sync::Once;
use std::os::unix::fs::MetadataExt;
use libc::{uid_t, geteuid};

// Assume ROOT_UID is 0 as in most Unix systems
const ROOT_UID: uid_t = 0;

#[cfg(not(feature = "cannot_unlink_dir"))]
pub fn cannot_unlink_dir() -> bool {
    static INIT: Once = Once::new();
    static mut CANNOT: bool = false;

    unsafe {
        INIT.call_once(|| {
            #[cfg(feature = "priv_sys_linkdir")]
            {
                // We might be able to unlink directories if we cannot
                // determine our privileges, or if we have the
                // PRIV_SYS_LINKDIR privilege.
                // Note: priv_set_ismember would need to be implemented
                CANNOT = priv_set_ismember(PRIV_SYS_LINKDIR).map(|v| v == 0).unwrap_or(false);
            }
            #[cfg(not(feature = "priv_sys_linkdir"))]
            {
                // In traditional Unix, only root can unlink directories.
                CANNOT = unsafe { geteuid() } != ROOT_UID;
            }
        });
        CANNOT
    }
}

#[cfg(feature = "cannot_unlink_dir")]
pub fn cannot_unlink_dir() -> bool {
    true
}

// Placeholder for priv_set_ismember functionality
#[cfg(feature = "priv_sys_linkdir")]
fn priv_set_ismember(_priv: &str) -> Result<bool, ()> {
    // Implementation would depend on platform-specific functionality
    Err(())
}