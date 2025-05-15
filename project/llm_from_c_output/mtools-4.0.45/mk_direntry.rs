/*  Copyright 1995-1998,2000-2003,2005,2007-2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 * mk_direntry.rs
 * Make new directory entries, and handles name clashes
 *
 */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::str;
use std::io::{self, Write, Read};
use std::fs::File;
use std::path::Path;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClashAction {
    None,
    Error,
    Success,
    Grew,
    AutoRename,
    Rename,
    Prename,
    Overwrite,
    Skip,
    Quit,
}

#[derive(Debug, Clone)]
struct DosName {
    base: [u8; 8],
    ext: [u8; 3],
    sentinel: u8,
}

impl Default for DosName {
    fn default() -> Self {
        DosName {
            base: [0; 8],
            ext: [0; 3],
            sentinel: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct ClashHandling {
    ignore_entry: i32,
    source_entry: i32,
    nowarn: bool,
    namematch_default: [ClashAction; 2],
    name_converter: fn(&DosCp, &str, bool, &mut bool, &mut DosName),
    source: i32,
    is_label: bool,
    action: [ClashAction; 2],
    use_longname: i32,
}

impl Default for ClashHandling {
    fn default() -> Self {
        ClashHandling {
            ignore_entry: -1,
            source_entry: -2,
            nowarn: false,
            namematch_default: [ClashAction::AutoRename, ClashAction::None],
            name_converter: dos_name,
            source: -2,
            is_label: false,
            action: [ClashAction::None, ClashAction::None],
            use_longname: 0,
        }
    }
}

struct DosCp;

struct Stream;
struct Directory;
struct Direntry;

fn convert_to_shortname(cp: &DosCp, ch: &ClashHandling, un: &str, dn: &mut DosName) -> bool {
    let mut mangled = false;
    (ch.name_converter)(cp, un, false, &mut mangled, dn);
    dn.sentinel = 0;
    if dn.base[0] == 0xE5 {
        dn.base[0] = 0x05;
    }
    mangled
}

fn chomp(line: &mut String) {
    while line.ends_with('\n') || line.ends_with('\r') {
        line.pop();
    }
}

fn ask_rename(cp: &DosCp, ch: &ClashHandling, shortname: &mut DosName, longname: &mut String, isprimary: bool) -> bool {
    let mut mangled = false;
    let stdin = io::stdin();
    
    loop {
        print!("New {} name for \"{}\": ", if isprimary { "primary" } else { "secondary" }, longname);
        io::stdout().flush().unwrap();
        
        let mut tname = String::new();
        stdin.read_line(&mut tname).unwrap();
        chomp(&mut tname);
        
        if isprimary {
            *longname = tname;
        } else {
            mangled = convert_to_shortname(cp, ch, &tname, shortname);
        }
        
        if !(mangled & 1) {
            break;
        }
    }
    true
}

fn ask_namematch(
    cp: &DosCp,
    dosname: &DosName,
    longname: &str,
    isprimary: bool,
    ch: &mut ClashHandling,
    no_overwrite: bool,
    reason: usize,
) -> ClashAction {
    let mut ans = String::new();
    let mut a = ch.action[isprimary as usize];
    let mut perm = false;
    
    let reasons = [
        "already exists",
        "is reserved",
        "contains illegal character(s)",
    ];
    
    if a == ClashAction::None && !opentty(1) {
        return ClashAction::Skip;
    }
    
    let name = if !isprimary {
        unix_normalize(cp, dosname)
    } else {
        longname.to_string()
    };
    
    while a == ClashAction::None {
        println!(
            "{} file name \"{}\" {}.",
            if isprimary { "Long" } else { "Short" },
            name,
            reasons[reason]
        );
        print!(
            "a)utorename A)utorename-all r)ename R)ename-all {}",
            if !no_overwrite { "o)verwrite O)verwrite-all" } else { "" }
        );
        println!(
            "\ns)kip S)kip-all q)uit (aArR{})",
            if !no_overwrite { "oO" } else { "" }
        );
        io::stdout().flush().unwrap();
        
        if mtools_raw_tty {
            let mut buf = [0];
            io::stdin().read_exact(&mut buf).unwrap();
            println!();
            ans = (buf[0] as char).to_string();
        } else {
            io::stdin().read_line(&mut ans).unwrap();
        }
        
        perm = ans.chars().next().unwrap().is_ascii_uppercase();
        a = match ans.chars().next().unwrap().to_ascii_lowercase() {
            'a' => ClashAction::AutoRename,
            'r' => if isprimary { ClashAction::Prename } else { ClashAction::Rename },
            'o' => if no_overwrite { continue; } else { ClashAction::Overwrite },
            's' => ClashAction::Skip,
            'q' => { perm = false; ClashAction::Quit },
            _ => { perm = false; continue; },
        };
    }
    
    ch.action[isprimary as usize] = a;
    if perm {
        ch.namematch_default[isprimary as usize] = a;
    }
    
    if a == ClashAction::Overwrite {
        ch.action[isprimary as usize] = ClashAction::None;
    }
    a
}

fn process_namematch(
    cp: &DosCp,
    dosname: &mut DosName,
    longname: &mut String,
    isprimary: bool,
    ch: &mut ClashHandling,
    no_overwrite: bool,
    reason: usize,
) -> ClashAction {
    let action = ask_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    
    match action {
        ClashAction::Quit => {
            got_signal = true;
            ClashAction::Skip
        }
        ClashAction::Skip => ClashAction::Skip,
        ClashAction::Rename | ClashAction::Prename => {
            ask_rename(cp, ch, dosname, longname, isprimary);
            action
        }
        ClashAction::AutoRename => {
            if isprimary {
                autorename_long(longname, true);
                ClashAction::Prename
            } else {
                autorename_short(dosname, true);
                ClashAction::Rename
            }
        }
        ClashAction::Overwrite => {
            if no_overwrite {
                ClashAction::Skip
            } else {
                ClashAction::Overwrite
            }
        }
        _ => ClashAction::None,
    }
}

fn contains_illegals(string: &str, illegals: &str, len: usize) -> bool {
    string.chars().take(len).any(|c| {
        (c < ' ' && c != '\x05' && !c.is_ascii()) ||
        illegals.contains(c)
    })
}

fn is_reserved(ans: &str, islong: bool) -> bool {
    let dev3 = ["CON", "AUX", "PRN", "NUL", "   "];
    let dev4 = ["COM", "LPT"];
    
    dev3.iter().any(|&s| {
        ans.len() >= 3 && ans[..3].eq_ignore_ascii_case(s) &&
        ((islong && ans.len() == 3) || (!islong && ans[3..].trim().is_empty()))
    }) ||
    dev4.iter().any(|&s| {
        ans.len() >= 4 && ans[..3].eq_ignore_ascii_case(s) &&
        ans.chars().nth(3).unwrap().is_ascii_digit() &&
        ans.chars().nth(3).unwrap() <= '4' &&
        ((islong && ans.len() == 4) || (!islong && ans[4..].trim().is_empty()))
    })
}

fn get_slots(
    dir: &Stream,
    dosname: &mut DosName,
    longname: &mut String,
    ssp: &mut ScanState,
    ch: &mut ClashHandling,
) -> ClashAction {
    let cp = get_dosconvert(dir);
    let pessimistic_short_rename = ch.action[0] == ClashAction::AutoRename;
    let mut entry = Direntry { dir: dir.clone() };
    let mut no_overwrite = true;
    let (isprimary, reason) = if is_reserved(longname, true) || longname.trim_matches(|c| c == '.' || c == ' ').is_empty() {
        (true, RESERVED)
    } else if contains_illegals(longname, LONG_ILLEGALS, 1024) {
        (true, ILLEGALS)
    } else if is_reserved(&dosname.base.iter().map(|&b| b as char).collect::<String>(), false) {
        ch.use_longname = 1;
        (false, RESERVED)
    } else if !ch.is_label && contains_illegals(&dosname.base.iter().map(|&b| b as char).collect::<String>(), SHORT_ILLEGALS, 11) {
        ch.use_longname = 1;
        (false, ILLEGALS)
    } else {
        match lookup_for_insert(dir, &mut entry, dosname, longname, ssp, ch.ignore_entry, ch.source_entry, pessimistic_short_rename && ch.use_longname != 0, ch.use_longname != 0) {
            -1 => return ClashAction::Error,
            0 => return ClashAction::Skip,
            5 => return ClashAction::Grew,
            6 => return ClashAction::Success,
            _ => unreachable!(),
        }
    };
    
    let match_pos = if ssp.longmatch >= 0 {
        ssp.longmatch
    } else if (ch.use_longname & 1) != 0 && ssp.shortmatch != -1 {
        ssp.shortmatch
    } else if ssp.shortmatch >= 0 {
        ssp.shortmatch
    } else {
        return ClashAction::Rename;
    };
    
    if match_pos > -1 {
        entry.entry = match_pos;
        if dir_read(&mut entry).is_err() {
            return ClashAction::Error;
        }
        no_overwrite = match_pos == ch.source || is_dir(&entry);
    }
    
    let ret = process_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    
    if ret == ClashAction::Overwrite && match_pos > -1 {
        if (entry.dir.attr & 0x5) != 0 && ask_confirmation("file is read only, overwrite anyway (y/n)? ") {
            return ClashAction::Rename;
        }
        if fat_free_with_direntry(&entry).is_err() {
            return ClashAction::Error;
        }
        wipe_entry(&entry);
        return ClashAction::Rename;
    }
    
    ret
}

fn write_slots(
    dir: &Stream,
    dosname: &DosName,
    longname: &str,
    ssp: &ScanState,
    cb: &dyn Fn(&DosName, &str, &mut dyn Any, &Direntry) -> i32,
    arg: &mut dyn Any,
    case: i32,
) -> bool {
    if fat_error(dir) {
        return false;
    }
    
    let mut entry = Direntry { dir: dir.clone() };
    assert!(ssp.got_slots);
    set_entry_to_pos(&mut entry, ssp.slot);
    native_to_wchar(longname, &mut entry.name, MAX_VNAMELEN, 0, 0);
    entry.name[MAX_VNAMELEN] = '\0';
    entry.dir.case = case & (EXTCASE | BASECASE);
    
    if cb(dosname, longname, arg, &entry) >= 0 {
        if ssp.size_needed > 1 && ssp.free_end - ssp.free_start >= ssp.size_needed {
            ssp.slot = write_vfat(dir, dosname, longname, ssp.free_start, &entry);
        } else {
            ssp.size_needed = 1;
            write_vfat(dir, dosname, "", ssp.free_start, &entry);
        }
        true
    } else {
        false
    }
}

fn stripspaces(name: &mut String) {
    if let Some(last_non_space) = name.trim_end().chars().last() {
        name.truncate(name.trim_end().len());
    }
}

fn mt_mwrite_one(
    dir: &Stream,
    argname: &str,
    shortname: Option<&str>,
    cb: &dyn Fn(&DosName, &str, &mut dyn Any, &Direntry) -> i32,
    arg: &mut dyn Any,
    ch: &mut ClashHandling,
) -> i32 {
    let mut longname = String::with_capacity(VBUFSIZE);
    let cp = get_dosconvert(dir);
    let mut expanded = false;
    let mut dosname = DosName::default();
    
    if is_special(argname) {
        eprintln!("Cannot create entry named . or ..");
        return -1;
    }
    
    if ch.name_converter == dos_name {
        if let Some(sname) = shortname {
            let mut s = sname.to_string();
            stripspaces(&mut s);
        }
        let mut aname = argname.to_string();
        stripspaces(&mut aname);
    }
    
    if let Some(sname) = shortname {
        convert_to_shortname(cp, ch, sname, &mut dosname);
        if (ch.use_longname & 1) != 0 {
            // short name mangled, treat as long name
            longname = sname.to_string();
            shortname = None;
        }
    }
    
    let dstname = if argname.len() > 1 && argname.chars().nth(1) == Some(':') {
        &argname[2..]
    } else {
        argname
    };
    
    longname = dstname.to_string();
    
    if let Some(sname) = shortname {
        ch.use_longname = convert_to_shortname(cp, ch, sname, &mut dosname) as i32;
        if sname != longname {
            ch.use_longname |= 1;
        }
    } else {
        ch.use_longname = convert_to_shortname(cp, ch, &longname, &mut dosname) as i32;
    }
    
    ch.action[0] = ch.namematch_default[0];
    ch.action[1] = ch.namematch_default[1];
    
    let mut scan = ScanState::default();
    loop {
        match get_slots(dir, &mut dosname, &mut longname, &mut scan, ch) {
            ClashAction::Error => return -1,
            ClashAction::Skip => return -1,
            ClashAction::Prename => {
                ch.use_longname = convert_to_shortname(cp, ch, &longname, &mut dosname) as i32;
                continue;
            }
            ClashAction::Rename => continue,
            ClashAction::Grew => {
                if expanded {
                    eprintln!("{}: No directory slots", progname);
                    return -1;
                }
                expanded = true;
                if dir_grow(dir, scan.max_entry).is_err() {
                    return -1;
                }
                continue;
            }
            ClashAction::Overwrite | ClashAction::Success => {
                return if write_slots(dir, &dosname, &longname, &scan, cb, arg, ch.use_longname) {
                    0
                } else {
                    -1
                };
            }
            _ => {
                eprintln!("Internal error: clash_action={:?}", ret);
                return -1;
            }
        }
    }
}

fn mwrite_one(
    dir: &Stream,
    argname: Option<&str>,
    shortname: Option<&str>,
    cb: &dyn Fn(&DosName, &str, &mut dyn Any, &Direntry) -> i32,
    arg: &mut dyn Any,
    ch: &mut ClashHandling,
) -> i32 {
    let argname = argname.map(|s| s.to_string());
    let shortname = shortname.map(|s| s.to_string());
    mt_mwrite