// unlinkdir.rs - determine whether we can unlink directories
//
// Copyright (C) 2005-2006, 2009-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
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
use nix::unistd::geteuid;
use nix::unistd::Uid;

// Assume ROOT_UID is 0 as in most Unix systems
const ROOT_UID: u32 = 0;

static mut CANNOT: bool = false;
static INIT: Once = Once::new();

/// Return true if we cannot unlink directories, false if we might be
/// able to unlink directories.
pub fn cannot_unlink_dir() -> bool {
    unsafe {
        INIT.call_once(|| {
            #[cfg(any(target_os = "solaris", target_os = "illumos"))]
            {
                // We might be able to unlink directories if we cannot
                // determine our privileges, or if we have the
                // PRIV_SYS_LINKDIR privilege.
                // Note: priv_set_ismember equivalent not available in Rust,
                // this would need platform-specific implementation
                CANNOT = false; // Placeholder - actual implementation needed
            }
            
            #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
            {
                // In traditional Unix, only root can unlink directories.
                CANNOT = geteuid() != Uid::from_raw(ROOT_UID);
            }
        });
        CANNOT
    }
}