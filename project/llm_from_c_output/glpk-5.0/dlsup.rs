/* dlsup.rs (dynamic linking support) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2008-2013 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::ffi::CString;
use std::ptr;
use libloading::{Library, Symbol};

#[cfg(feature = "ltdl")]
mod gnu_version {
    use super::*;
    use libltdl::{self, LtDl};

    pub fn xdlopen(module: &str) -> Option<LtDl> {
        let module = CString::new(module).ok()?;
        unsafe {
            if libltdl::lt_dlinit() != 0 {
                put_err_msg(libltdl::lt_dlerror().to_string_lossy());
                return None;
            }
            let h = libltdl::lt_dlopen(module.as_ptr());
            if h.is_null() {
                put_err_msg(libltdl::lt_dlerror().to_string_lossy());
                if libltdl::lt_dlexit() != 0 {
                    xerror(&format!("xdlopen: {}", libltdl::lt_dlerror().to_string_lossy()));
                }
                None
            } else {
                Some(LtDl::from_ptr(h))
            }
        }
    }

    pub fn xdlsym<T>(h: &LtDl, symbol: &str) -> Result<Symbol<T>, String> {
        let symbol = CString::new(symbol).map_err(|e| e.to_string())?;
        unsafe {
            let ptr = libltdl::lt_dlsym(h.as_ptr(), symbol.as_ptr());
            if ptr.is_null() {
                Err(format!("xdlsym: {}: {}", symbol.to_string_lossy(), libltdl::lt_dlerror().to_string_lossy()))
            } else {
                Ok(Symbol::from_raw(ptr))
            }
        }
    }

    pub fn xdlclose(h: LtDl) -> Result<(), String> {
        unsafe {
            if libltdl::lt_dlclose(h.into_ptr()) != 0 {
                return Err(format!("xdlclose: {}", libltdl::lt_dlerror().to_string_lossy()));
            }
            if libltdl::lt_dlexit() != 0 {
                return Err(format!("xdlclose: {}", libltdl::lt_dlerror().to_string_lossy()));
            }
            Ok(())
        }
    }
}

#[cfg(feature = "dlfcn")]
mod posix_version {
    use super::*;

    pub fn xdlopen(module: &str) -> Option<Library> {
        let module = CString::new(module).ok()?;
        unsafe {
            let h = libloading::os::unix::Library::open(Some(&module), libloading::os::unix::RTLD_NOW)
                .map_err(|e| put_err_msg(e.to_string())).ok()?;
            Some(Library::from(h))
        }
    }

    pub fn xdlsym<T>(h: &Library, symbol: &str) -> Result<Symbol<T>, String> {
        h.get(symbol.as_bytes())
            .map_err(|e| format!("xdlsym: {}: {}", symbol, e))
    }

    pub fn xdlclose(h: Library) -> Result<(), String> {
        drop(h);
        Ok(())
    }
}

#[cfg(windows)]
mod windows_version {
    use super::*;
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;

    pub fn xdlopen(module: &str) -> Option<Library> {
        let module = OsStr::new(module)
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        unsafe {
            let h = libloading::os::windows::Library::load_with_flags(&module, 0)
                .map_err(|e| {
                    put_err_msg(format!("Error {}", e.raw_os_error().unwrap_or(0)));
                }).ok()?;
            Some(Library::from(h))
        }
    }

    pub fn xdlsym<T>(h: &Library, symbol: &str) -> Result<Symbol<T>, String> {
        h.get(symbol.as_bytes())
            .map_err(|e| format!("xdlsym: {}: Error {}", symbol, e.raw_os_error().unwrap_or(0)))
    }

    pub fn xdlclose(h: Library) -> Result<(), String> {
        drop(h);
        Ok(())
    }
}

#[cfg(not(any(feature = "ltdl", feature = "dlfcn", windows)))]
mod null_version {
    use super::*;

    pub fn xdlopen(_module: &str) -> Option<()> {
        put_err_msg("Shared libraries not supported");
        None
    }

    pub fn xdlsym<T>(_h: &(), _symbol: &str) -> Result<Symbol<T>, String> {
        Err("Shared libraries not supported".to_string())
    }

    pub fn xdlclose(_h: ()) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(feature = "ltdl")]
pub use gnu_version::{xdlopen, xdlsym, xdlclose};
#[cfg(feature = "dlfcn")]
pub use posix_version::{xdlopen, xdlsym, xdlclose};
#[cfg(windows)]
pub use windows_version::{xdlopen, xdlsym, xdlclose};
#[cfg(not(any(feature = "ltdl", feature = "dlfcn", windows)))]
pub use null_version::{xdlopen, xdlsym, xdlclose};

fn put_err_msg(msg: impl AsRef<str>) {
    // Implementation depends on your error handling system
}

fn xerror(msg: &str) {
    // Implementation depends on your error handling system
}