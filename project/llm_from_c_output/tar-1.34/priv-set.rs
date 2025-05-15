/*
 * Query, remove, or restore a Solaris privilege.
 *
 * Copyright (C) 2009-2021 Free Software Foundation, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * Written by David Bartley.
 */

#[cfg(all(feature = "solaris", feature = "privileges"))]
mod solaris_priv {
    use std::ffi::CString;
    use std::ptr;
    use std::sync::Once;

    extern "C" {
        fn priv_allocset() -> *mut libc::priv_set_t;
        fn priv_freeset(set: *mut libc::priv_set_t);
        fn getppriv(which: libc::priv_ptype_t, set: *mut libc::priv_set_t) -> libc::c_int;
        fn priv_ismember(set: *const libc::priv_set_t, priv_name: *const libc::c_char) -> libc::c_int;
        fn priv_addset(set: *mut libc::priv_set_t, priv_name: *const libc::c_char) -> libc::c_int;
        fn priv_delset(set: *mut libc::priv_set_t, priv_name: *const libc::c_char) -> libc::c_int;
        fn setppriv(op: libc::priv_op_t, which: libc::priv_ptype_t, set: *const libc::priv_set_t) -> libc::c_int;
        fn priv_emptyset(set: *mut libc::priv_set_t) -> libc::c_int;
    }

    static INIT: Once = Once::new();
    static mut EFF_SET: *mut libc::priv_set_t = ptr::null_mut();
    static mut REM_SET: *mut libc::priv_set_t = ptr::null_mut();
    static mut INITIALIZED: bool = false;

    fn initialize() -> Result<(), std::io::Error> {
        unsafe {
            INIT.call_once(|| {
                EFF_SET = priv_allocset();
                if EFF_SET.is_null() {
                    return;
                }

                REM_SET = priv_allocset();
                if REM_SET.is_null() {
                    priv_freeset(EFF_SET);
                    return;
                }

                if getppriv(libc::PRIV_EFFECTIVE, EFF_SET) != 0 {
                    priv_freeset(EFF_SET);
                    priv_freeset(REM_SET);
                    return;
                }

                priv_emptyset(REM_SET);
                INITIALIZED = true;
            });

            if !INITIALIZED {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }

    pub fn priv_set_ismember(priv: &str) -> Result<bool, std::io::Error> {
        unsafe {
            if !INITIALIZED && initialize().is_err() {
                return Err(std::io::Error::last_os_error());
            }

            let priv_cstr = CString::new(priv).map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
            let result = priv_ismember(EFF_SET, priv_cstr.as_ptr());
            match result {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(std::io::Error::last_os_error()),
            }
        }
    }

    pub fn priv_set_remove(priv: &str) -> Result<(), std::io::Error> {
        unsafe {
            if !INITIALIZED && initialize().is_err() {
                return Err(std::io::Error::last_os_error());
            }

            let priv_cstr = CString::new(priv).map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
            if priv_ismember(EFF_SET, priv_cstr.as_ptr()) == 1 {
                priv_delset(EFF_SET, priv_cstr.as_ptr());
                if setppriv(libc::PRIV_SET, libc::PRIV_EFFECTIVE, EFF_SET) != 0 {
                    priv_addset(EFF_SET, priv_cstr.as_ptr());
                    return Err(std::io::Error::last_os_error());
                }
                priv_addset(REM_SET, priv_cstr.as_ptr());
                Ok(())
            } else {
                Err(std::io::Error::from_raw_os_error(libc::EINVAL))
            }
        }
    }

    pub fn priv_set_restore(priv: &str) -> Result<(), std::io::Error> {
        unsafe {
            if !INITIALIZED && initialize().is_err() {
                return Err(std::io::Error::last_os_error());
            }

            let priv_cstr = CString::new(priv).map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
            if priv_ismember(REM_SET, priv_cstr.as_ptr()) == 1 {
                priv_addset(EFF_SET, priv_cstr.as_ptr());
                if setppriv(libc::PRIV_SET, libc::PRIV_EFFECTIVE, EFF_SET) != 0 {
                    priv_delset(EFF_SET, priv_cstr.as_ptr());
                    return Err(std::io::Error::last_os_error());
                }
                priv_delset(REM_SET, priv_cstr.as_ptr());
                Ok(())
            } else {
                Err(std::io::Error::from_raw_os_error(libc::EINVAL))
            }
        }
    }

    pub fn priv_set_remove_linkdir() -> Result<(), std::io::Error> {
        priv_set_remove("PRIV_SYS_LINKDIR")
    }

    pub fn priv_set_restore_linkdir() -> Result<(), std::io::Error> {
        priv_set_restore("PRIV_SYS_LINKDIR")
    }
}

#[cfg(not(all(feature = "solaris", feature = "privileges")))]
mod solaris_priv {
    use std::io;

    pub fn priv_set_remove_linkdir() -> io::Result<()> {
        Err(io::Error::from_raw_os_error(libc::ENOSYS))
    }

    pub fn priv_set_restore_linkdir() -> io::Result<()> {
        Err(io::Error::from_raw_os_error(libc::ENOSYS))
    }
}

pub use solaris_priv::*;