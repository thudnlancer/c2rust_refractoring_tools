/*  Copyright 1986-1992 Emmet P. Gray.
 *  Copyright 1996-1998,2000-2002,2005,2007-2009 Alain Knaff.
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
 * mlabel.rs
 * Make an MSDOS volume label
 */

use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::c_int;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::AsRawFd;
use libc::{wchar_t, wint_t, iswlower, iswupper, wcschr, strchr};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{read, write, close};
use rand::Rng;

const VBUFSIZE: usize = 256;
const DOS_NAME_LEN: usize = 11;

#[derive(Debug)]
struct DosName {
    base: [u8; 8],
    ext: [u8; 3],
    sentinel: u8,
}

impl Default for DosName {
    fn default() -> Self {
        DosName {
            base: [b' '; 8],
            ext: [b' '; 3],
            sentinel: 0,
        }
    }
}

#[derive(Debug)]
struct Direntry {
    dir: DirEntry,
}

#[derive(Debug)]
struct DirEntry {
    attr: u8,
    // Other fields omitted for brevity
}

#[derive(Debug)]
struct Stream {
    // Implementation details omitted
}

#[derive(Debug)]
struct ClashHandling {
    name_converter: fn(doscp: &DosCp, filename: &str, verbose: i32, mangled: &mut i32, ans: &mut DosName),
    ignore_entry: i32,
    is_label: i32,
}

impl Default for ClashHandling {
    fn default() -> Self {
        ClashHandling {
            name_converter: label_name_uc,
            ignore_entry: -2,
            is_label: 1,
        }
    }
}

#[derive(Debug)]
struct DosCp {
    // Implementation details omitted
}

fn mt_label_name(
    cp: &DosCp,
    filename: &str,
    verbose: i32,
    mangled: &mut i32,
    ans: &mut DosName,
    preserve_case: bool,
) {
    let mut wbuffer = [0 as wchar_t; 12];
    let len = native_to_wchar(filename, &mut wbuffer, 11, 0, 0);
    
    *ans = DosName::default();
    ans.sentinel = b'\0';
    
    if len > 11 {
        *mangled = 1;
    } else {
        *mangled = 0;
    }

    let mut have_lower = false;
    let mut have_upper = false;

    for i in 0..len {
        if unsafe { iswlower(wbuffer[i] as wint_t) } != 0 {
            have_lower = true;
        }
        if unsafe { iswupper(wbuffer[i] as wint_t) } != 0 {
            have_upper = true;
        }
        
        if !preserve_case {
            wbuffer[i] = ch_towupper(wbuffer[i]);
        }

        let forbidden_chars = if cfg!(feature = "wchar") {
            unsafe { wcschr("^+=/[]:,?*\\<>|\"..".as_ptr() as *const wchar_t, wbuffer[i]) }
        } else {
            unsafe { strchr("^+=/[]:,?*\\<>|\"..".as_ptr() as *const i8, wbuffer[i] as i8) }
        };

        if !forbidden_chars.is_null() {
            *mangled = 1;
            wbuffer[i] = '~' as wchar_t;
        }
    }

    if have_lower && have_upper {
        *mangled = 1;
    }

    wchar_to_dos(cp, &wbuffer, &mut ans.base, len, mangled);
}

fn label_name_uc(cp: &DosCp, filename: &str, verbose: i32, mangled: &mut i32, ans: &mut DosName) {
    mt_label_name(cp, filename, verbose, mangled, ans, false);
}

fn label_name_pc(cp: &DosCp, filename: &str, verbose: i32, mangled: &mut i32, ans: &mut DosName) {
    mt_label_name(cp, filename, verbose, mangled, ans, true);
}

fn labelit(
    dosname: &DosName,
    longname: Option<&str>,
    arg0: Option<&()>,
    entry: &mut Direntry,
) -> i32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    mk_entry(dosname, 0x8, 0, 0, now as i32, &mut entry.dir);
    0
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", mversion, mdate);
    eprintln!("Usage: {} [-vscVn] [-N serial] drive:", progname);
    std::process::exit(ret);
}

fn mlabel(argc: i32, argv: Vec<String>, type_: i32) -> ! {
    let mut new_label = "";
    let mut verbose = 0;
    let mut clear = 0;
    let mut interactive = false;
    let mut show = 0;
    let mut entry = Direntry { dir: DirEntry { attr: 0 } };
    let mut result = 0;
    let mut longname = [0u8; VBUFSIZE];
    let mut shortname = [0u8; 45];
    let mut ch = ClashHandling::default();
    let mut root_dir: Option<Stream> = None;
    let mut set_serial = SerialOption::None;
    let mut serial = 0u32;
    let mut need_write_boot = false;
    let mut have_boot = false;
    let mut is_ro = false;
    let mut is_rop: Option<&mut i32> = None;
    let mut drive = '\0';

    // Initialize clash handling
    ch.name_converter = label_name_uc;
    ch.ignore_entry = -2;
    ch.is_label = 1;

    // Parse command line arguments
    // (Implementation omitted for brevity)

    if argc - optind > 1 {
        usage(1);
    }

    if argc - optind == 1 {
        let arg = &argv[optind as usize];
        if arg.is_empty() || arg.chars().nth(1) != Some(':') {
            usage(1);
        }
        drive = arg.chars().next().unwrap().to_ascii_uppercase();
        new_label = &arg[2..];
    } else {
        drive = get_default_drive();
    }

    if new_label.len() > VBUFSIZE {
        eprintln!("Label too long");
        std::process::exit(1);
    }

    interactive = !show && !clear && new_label.is_empty() && set_serial == SerialOption::None;
    if !clear && new_label.is_empty() {
        is_rop = Some(&mut is_ro);
    }

    if clear && !new_label.is_empty() {
        eprintln!("Both clear and new label specified");
        std::process::exit(1);
    }

    // Open root directory
    root_dir = open_root_dir(drive, if is_rop.is_some() { 0 } else { O_RDWR }, is_rop.is_some());
    if is_ro {
        show = 1;
        interactive = false;
    }

    if root_dir.is_none() {
        eprintln!("{}: Cannot initialize drive", argv[0]);
        std::process::exit(1);
    }

    initialize_direntry(&mut entry, root_dir.as_ref().unwrap());
    let r = vfat_lookup(
        &mut entry,
        0,
        0,
        ACCEPT_LABEL | MATCH_ANY,
        &mut shortname,
        shortname.len(),
        &mut longname,
        longname.len(),
    );

    if r == -2 {
        std::process::exit(1);
    }

    if show || interactive {
        if is_not_found(&entry) {
            println!(" Volume has no label");
        } else if longname[0] != 0 {
            println!(
                " Volume label is {} (abbr={})",
                String::from_utf8_lossy(&longname),
                String::from_utf8_lossy(&shortname)
            );
        } else {
            println!(" Volume label is {}", String::from_utf8_lossy(&shortname));
        }
    }

    if interactive {
        // Handle interactive input
        // (Implementation omitted for brevity)
    }

    if new_label.len() > 11 {
        eprintln!("New label too long");
        std::process::exit(1);
    }

    if (!show || !new_label.is_empty()) && !is_not_found(&entry) {
        if interactive && new_label.is_empty() {
            if ask_confirmation("Delete volume label (y/n): ") {
                std::process::exit(0);
            }
        }
        entry.dir.attr = 0;
        wipe_entry(&mut entry);
    }

    if !new_label.is_empty() {
        ch.ignore_entry = 1;
        result = if mwrite_one(
            root_dir.as_ref().unwrap(),
            new_label,
            0,
            labelit,
            None,
            &ch,
        ) {
            0
        } else {
            1
        };
    }

    have_boot = false;
    if (!show || !new_label.is_empty()) || set_serial != SerialOption::None {
        let fs = get_fs(root_dir.as_ref().unwrap());
        have_boot = force_pread(fs, &mut boot.characters, 0, std::mem::size_of::<BootSector>())
            == std::mem::size_of::<BootSector>();
    }

    // Rest of the implementation...
    // (Omitted for brevity)

    std::process::exit(result);
}

enum SerialOption {
    None,
    Random,
    Set,
}

// Helper functions and other implementations omitted for brevity