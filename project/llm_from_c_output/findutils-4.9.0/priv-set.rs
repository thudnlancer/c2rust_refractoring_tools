//! Query, remove, or restore a Solaris privilege.
//!
//! Copyright (C) 2009-2022 Free Software Foundation, Inc.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! Written by David Bartley.

#[cfg(all(feature = "solaris", target_os = "solaris"))]
mod solaris {
    use std::sync::Once;
    use std::ptr;
    use std::ffi::CString;
    use std::io;

    static INIT: Once = Once::new();
    static mut EFF_SET: *mut libc::priv_set_t = ptr::null_mut();
    static mut REM_SET: *mut libc::priv_set_t = ptr::null_mut();

    fn initialize() -> io::Result<()> {
        unsafe {
            let mut success = true;
            let mut eff_set = ptr::null_mut();
            let mut rem_set = ptr::null_mut();

            INIT.call_once(|| {
                eff_set = libc::priv_allocset();
                if eff_set.is_null() {
                    success = false;
                    return;
                }

                rem_set = libc::priv_allocset();
                if rem_set.is_null() {
                    libc::priv_freeset(eff_set);
                    success = false;
                    return;
                }

                if libc::getppriv(libc::PRIV_EFFECTIVE, eff_set) != 0 {
                    libc::priv_freeset(eff_set);
                    libc::priv_freeset(rem_set);
                    success = false;
                    return;
                }

                libc::priv_emptyset(rem_set);
                EFF_SET = eff_set;
                REM_SET = rem_set;
            });

            if success {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    pub fn priv_set_ismember(priv_name: &str) -> io::Result<bool> {
        initialize()?;
        let c_priv = CString::new(priv_name)?;
        
        unsafe {
            Ok(libc::priv_ismember(EFF_SET, c_priv.as_ptr()) == 1)
        }
    }

    pub fn priv_set_remove(priv_name: &str) -> io::Result<()> {
        initialize()?;
        let c_priv = CString::new(priv_name)?;

        unsafe {
            if libc::priv_ismember(EFF_SET, c_priv.as_ptr()) == 1 {
                libc::priv_delset(EFF_SET, c_priv.as_ptr());
                if libc::setppriv(libc::PRIV_SET, libc::PRIV_EFFECTIVE, EFF_SET) != 0 {
                    libc::priv_addset(EFF_SET, c_priv.as_ptr());
                    return Err(io::Error::last_os_error());
                }
                libc::priv_addset(REM_SET, c_priv.as_ptr());
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Privilege not in effective set"))
            }
        }
    }

    pub fn priv_set_restore(priv_name: &str) -> io::Result<()> {
        initialize()?;
        let c_priv = CString::new(priv_name)?;

        unsafe {
            if libc::priv_ismember(REM_SET, c_priv.as_ptr()) == 1 {
                libc::priv_addset(EFF_SET, c_priv.as_ptr());
                if libc::setppriv(libc::PRIV_SET, libc::PRIV_EFFECTIVE, EFF_SET) != 0 {
                    libc::priv_delset(EFF_SET, c_priv.as_ptr());
                    return Err(io::Error::last_os_error());
                }
                libc::priv_delset(REM_SET, c_priv.as_ptr());
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Privilege not in removed set"))
            }
        }
    }

    pub fn priv_set_remove_linkdir() -> io::Result<()> {
        priv_set_remove(libc::PRIV_SYS_LINKDIR)
    }

    pub fn priv_set_restore_linkdir() -> io::Result<()> {
        priv_set_restore(libc::PRIV_SYS_LINKDIR)
    }
}

#[cfg(not(all(feature = "solaris", target_os = "solaris")))]
mod solaris {
    use std::io;

    pub fn priv_set_remove_linkdir() -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "Solaris privileges not supported on this platform"))
    }

    pub fn priv_set_restore_linkdir() -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "Solaris privileges not supported on this platform"))
    }
}

pub use solaris::{priv_set_remove_linkdir, priv_set_restore_linkdir};