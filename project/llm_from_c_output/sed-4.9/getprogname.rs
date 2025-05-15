/* Program name management.
   Copyright (C) 2016-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 2.1 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::sync::Once;

#[cfg(target_os = "linux")]
fn get_progname_linux() -> Option<String> {
    env::args_os()
        .next()
        .and_then(|s| s.into_string().ok())
        .map(|s| Path::new(&s).file_name().unwrap().to_string_lossy().into_owned())
}

#[cfg(target_os = "windows")]
fn get_progname_windows() -> Option<String> {
    env::args_os()
        .next()
        .and_then(|s| s.into_string().ok())
        .map(|s| Path::new(&s).file_name().unwrap().to_string_lossy().into_owned())
}

#[cfg(target_os = "macos")]
fn get_progname_macos() -> Option<String> {
    env::args_os()
        .next()
        .and_then(|s| s.into_string().ok())
        .map(|s| Path::new(&s).file_name().unwrap().to_string_lossy().into_owned())
}

#[cfg(target_os = "freebsd")]
fn get_progname_freebsd() -> Option<String> {
    extern "C" {
        fn getprogname() -> *const libc::c_char;
    }
    
    unsafe {
        let name = getprogname();
        if name.is_null() {
            None
        } else {
            Some(CString::from_raw(name as *mut libc::c_char).into_string().unwrap())
        }
    }
}

#[cfg(target_os = "netbsd")]
fn get_progname_netbsd() -> Option<String> {
    extern "C" {
        fn getprogname() -> *const libc::c_char;
    }
    
    unsafe {
        let name = getprogname();
        if name.is_null() {
            None
        } else {
            Some(CString::from_raw(name as *mut libc::c_char).into_string().unwrap())
        }
    }
}

#[cfg(target_os = "openbsd")]
fn get_progname_openbsd() -> Option<String> {
    extern "C" {
        static mut __progname: *const libc::c_char;
    }
    
    unsafe {
        if __progname.is_null() {
            None
        } else {
            Some(CString::from_raw(__progname as *mut libc::c_char).into_string().unwrap())
        }
    }
}

#[cfg(target_os = "solaris")]
fn get_progname_solaris() -> Option<String> {
    extern "C" {
        fn getexecname() -> *const libc::c_char;
    }
    
    unsafe {
        let name = getexecname();
        if name.is_null() {
            None
        } else {
            Some(CString::from_raw(name as *mut libc::c_char).into_string().unwrap())
        }
    }
}

#[cfg(target_os = "android")]
fn get_progname_android() -> Option<String> {
    extern "C" {
        static mut __progname: *const libc::c_char;
    }
    
    unsafe {
        if __progname.is_null() {
            None
        } else {
            Some(CString::from_raw(__progname as *mut libc::c_char).into_string().unwrap())
        }
    }
}

pub fn getprogname() -> String {
    #[cfg(target_os = "linux")]
    return get_progname_linux().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "windows")]
    return get_progname_windows().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "macos")]
    return get_progname_macos().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "freebsd")]
    return get_progname_freebsd().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "netbsd")]
    return get_progname_netbsd().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "openbsd")]
    return get_progname_openbsd().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "solaris")]
    return get_progname_solaris().unwrap_or_else(|| "?".to_string());
    
    #[cfg(target_os = "android")]
    return get_progname_android().unwrap_or_else(|| "?".to_string());
    
    #[cfg(not(any(
        target_os = "linux",
        target_os = "windows",
        target_os = "macos",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "solaris",
        target_os = "android"
    )))]
    compile_error!("getprogname module not ported to this OS");
}