/*
 *  Copyright 1986-1992 Emmet P. Gray.
 *  Copyright 1996-1998,2001,2002,2007,2009 Alain Knaff.
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
 * A program to create a manifest (shipping list) that is a shell script
 * to return a Unix file name to it's original state after it has been
 * clobbered by MSDOS's file name restrictions.
 *
 * This code also used in arc, mtools, and pcomm
 */

use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::io::{self, Write};

const DEVICES: [&str; 9] = ["con", "aux", "com1", "com2", "lpt1", "prn", "lpt2", "lpt3", "nul"];
const MAX_PATH: usize = 260;

fn dos_name2(name: &str) -> String {
    let mut buf = name.to_lowercase();
    let mut ext = None;
    let mut dot_pos = None;

    // Separate the name from extension
    if let Some(pos) = buf.rfind('.') {
        dot_pos = Some(pos);
        ext = Some(buf[pos + 1..].to_string());
        buf.truncate(pos);
    }

    // If no name
    if buf.is_empty() {
        buf = "x".to_string();
    } else {
        // If name is a device
        if DEVICES.iter().any(|&d| d == buf) {
            buf = "x".to_string();
        }

        // Name too long?
        if buf.len() > 8 {
            buf.truncate(8);
        }

        // Extension too long?
        if let Some(ref mut e) = ext {
            if e.len() > 3 {
                e.truncate(3);
            }
        }

        // Illegal characters?
        let illegal_chars: &[char] = &['^', '+', '=', '/', '[', ']', ':', '\'', ',', '?', '*', '\\', '<', '>', '|', '"', '.', ' '];
        buf = buf.replace(illegal_chars, "x");
        if let Some(ref mut e) = ext {
            *e = e.replace(illegal_chars, "x");
        }
    }

    let mut ans = buf;
    if let Some(e) = ext {
        if !e.is_empty() {
            ans.push('.');
            ans.push_str(&e);
        }
    }

    ans
}

fn basename(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or(path)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Print the version
    if args.len() >= 2 && args[1] == "-V" {
        println!("Mtools version {}, dated {}", "unknown", "unknown");
        return Ok(());
    }

    if args.len() == 1 {
        writeln!(io::stderr(), "Usage: mkmanifest [-V] <list-of-files>")?;
        return Ok(());
    }

    for arg in args.iter().skip(1) {
        // Zap the leading path
        let name = basename(arg);
        // Create new name
        let new_name = dos_name2(name);

        if new_name.to_lowercase() != name.to_lowercase() {
            println!("mv {} {}", new_name, name);
        }
    }

    Ok(())
}