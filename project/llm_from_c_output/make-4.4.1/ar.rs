/* Interface to 'ar' archives for GNU Make.
Copyright (C) 1988-2023 Free Software Foundation, Inc.

This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs;
use fnmatch::fnmatch;
use libc::{uid_t, gid_t, mode_t};

#[derive(Debug)]
struct Nameseq {
    name: String,
    next: Option<Box<Nameseq>>,
}

/// Return true if NAME is an archive-member reference, false if not.
/// An archive-member reference is a name like 'lib(member)' where member is a non-empty string.
/// If a name like 'lib((entry))' is used, a fatal error is signaled at the attempt to use this unsupported feature.
pub fn ar_name(name: &str) -> bool {
    let p = match name.find('(') {
        Some(pos) => pos,
        None => return false,
    };

    if p == 0 {
        return false;
    }

    let end = name.rfind(')').unwrap_or(0);
    if end == 0 || end == p + 1 {
        return false;
    }

    if name[p+1..].starts_with('(') && name[end-1..].starts_with(')') {
        panic!("attempt to use unsupported feature: '{}'", name);
    }

    true
}

/// Parse the archive-member reference NAME into the archive and member names.
/// Returns a tuple of (archive_name, member_name).
pub fn ar_parse_name(name: &str) -> (String, String) {
    if !ar_name(name) {
        panic!("Internal: ar_parse_name: bad name '{}'", name);
    }

    let p = name.find('(').unwrap();
    let archive = name[..p].to_string();
    let member = name[p+1..name.len()-1].to_string();

    (archive, member)
}

/// Return the modtime of NAME.
pub fn ar_member_date(name: &str) -> Option<SystemTime> {
    let (arname, memname) = ar_parse_name(name);

    // Check if archive exists and get its modtime
    if Path::new(&arname).exists() {
        if let Ok(metadata) = fs::metadata(&arname) {
            if let Ok(modtime) = metadata.modified() {
                // TODO: Implement ar_scan equivalent
                // let val = ar_scan(&arname, ar_member_date_1, &memname);
                // if val > 0 {
                //     return Some(UNIX_EPOCH + Duration::from_secs(val as u64));
                // }
            }
        }
    }

    None
}

/// Set the archive-member NAME's modtime to now.
#[cfg(not(target_os = "vms"))]
pub fn ar_touch(name: &str) -> Result<(), String> {
    let (arname, memname) = ar_parse_name(name);

    // Make sure archive exists and get its modtime first
    if !Path::new(&arname).exists() {
        return Err(format!("touch: Archive '{}' does not exist", arname));
    }

    // TODO: Implement ar_member_touch equivalent
    // match ar_member_touch(&arname, &memname) {
    //     0 => Ok(()),
    //     -1 => Err(format!("touch: Archive '{}' does not exist", arname)),
    //     -2 => Err(format!("touch: '{}' is not a valid archive", arname)),
    //     -3 => Err(format!("touch: {}", std::io::Error::last_os_error())),
    //     1 => Err(format!("touch: Member '{}' does not exist in '{}'", memname, arname)),
    //     _ => Err(format!("touch: Bad return code from ar_member_touch on '{}'", name)),
    // }
    Ok(())
}

#[cfg(target_os = "vms")]
pub fn ar_touch(_name: &str) -> Result<(), String> {
    Err("touch archive member is not available on VMS".to_string())
}

/// State of an 'ar_glob' run
struct ArGlobState {
    arname: String,
    pattern: String,
    #[cfg(target_os = "vms")]
    suffix: Option<String>,
    size: usize,
    chain: Option<Box<Nameseq>>,
    n: usize,
}

/// Return true if PATTERN contains any metacharacters.
/// Metacharacters can be quoted with backslashes if QUOTE is true.
fn ar_glob_pattern_p(pattern: &str, quote: bool) -> bool {
    let mut opened = false;
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '?' | '*' => return true,
            '\\' if quote => {
                if chars.next().is_none() {
                    break;
                }
            }
            '[' => opened = true,
            ']' if opened => return true,
            _ => (),
        }
    }

    false
}

/// Glob for MEMBER_PATTERN in archive ARNAME.
/// Returns a chain of matching elements (or None if none).
pub fn ar_glob(arname: &str, member_pattern: &str, size: usize) -> Option<Box<Nameseq>> {
    if !ar_glob_pattern_p(member_pattern, true) {
        return None;
    }

    let mut state = ArGlobState {
        arname: arname.to_string(),
        pattern: member_pattern.to_string(),
        #[cfg(target_os = "vms")]
        suffix: None,
        size,
        chain: None,
        n: 0,
    };

    #[cfg(target_os = "vms")]
    {
        if let Some(lastdot) = member_pattern.rfind('.') {
            state.suffix = Some(member_pattern[lastdot..].to_string());
            state.pattern = member_pattern[..lastdot].to_string();
        }
    }

    // TODO: Implement ar_scan equivalent
    // ar_scan(arname, ar_glob_match, &mut state);

    #[cfg(target_os = "vms")]
    {
        // Clean up VMS-specific data
    }

    if state.n == 0 {
        return None;
    }

    // TODO: Implement sorting logic
    state.chain
}