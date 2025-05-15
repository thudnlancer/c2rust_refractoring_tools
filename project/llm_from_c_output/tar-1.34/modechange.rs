/* modechange.rs -- file mode manipulation in Rust

   Translated from C code originally written by David MacKenzie <djm@ai.mit.edu>
   and maintained by the Free Software Foundation.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

// Traditional octal values for mode bits
const SUID: u32 = 0o4000;
const SGID: u32 = 0o2000;
const SVTX: u32 = 0o1000;
const RUSR: u32 = 0o0400;
const WUSR: u32 = 0o0200;
const XUSR: u32 = 0o0100;
const RGRP: u32 = 0o0040;
const WGRP: u32 = 0o0020;
const XGRP: u32 = 0o0010;
const ROTH: u32 = 0o0004;
const WOTH: u32 = 0o0002;
const XOTH: u32 = 0o0001;
const ALLM: u32 = 0o7777; // All octal mode bits

// All file mode bits that chmod can change
const CHMOD_MODE_BITS: u32 = 0o7777;

// Special operations flags
#[derive(Debug, Clone, Copy)]
enum ModeFlag {
    Done,            // Sentinel at end of changes array
    OrdinaryChange,  // Typical case
    XIfAnyX,         // Affect execute bits if any execute bit set or directory
    CopyExisting,    // Copy existing permissions
}

// Description of a mode change
#[derive(Debug, Clone)]
struct ModeChange {
    op: char,         // One of "=+-"
    flag: ModeFlag,   // Special operations flag
    affected: u32,    // Set for u, g, o, or a
    value: u32,       // Bits to add/remove
    mentioned: u32,   // Bits explicitly mentioned
}

// Convert traditional octal to mode_t value
fn octal_to_mode(octal: u32) -> u32 {
    if cfg!(target_os = "linux") || cfg!(target_os = "unix") {
        // On Unix-like systems, mode bits match traditional octal values
        octal
    } else {
        // Fallback conversion for other platforms
        ((octal & SUID) | (octal & SGID) | (octal & SVTX)
            | (octal & RUSR) | (octal & WUSR) | (octal & XUSR)
            | (octal & RGRP) | (octal & WGRP) | (octal & XGRP)
            | (octal & ROTH) | (octal & WOTH) | (octal & XOTH)
    }
}

// Create a mode change node for "=ddd" operation
fn make_node_op_equals(new_mode: u32, mentioned: u32) -> Vec<ModeChange> {
    vec![
        ModeChange {
            op: '=',
            flag: ModeFlag::OrdinaryChange,
            affected: CHMOD_MODE_BITS,
            value: new_mode,
            mentioned,
        },
        ModeChange {
            op: ' ',
            flag: ModeFlag::Done,
            affected: 0,
            value: 0,
            mentioned: 0,
        },
    ]
}

// Compile mode string into mode change operations
pub fn mode_compile(mode_string: &str) -> Option<Vec<ModeChange>> {
    let mut mc = Vec::new();
    let mut p = mode_string.chars().peekable();

    // Handle octal mode
    if mode_string.starts_with(|c: char| c.is_ascii_digit() && c < '8') {
        let mut octal_mode = 0;
        let mut len = 0;

        for c in mode_string.chars() {
            if !c.is_ascii_digit() || c >= '8' {
                return None;
            }
            octal_mode = 8 * octal_mode + c.to_digit(8).unwrap();
            len += 1;
            if octal_mode > ALLM {
                return None;
            }
        }

        let mode = octal_to_mode(octal_mode);
        let mentioned = if len < 5 {
            (mode & (SUID | SGID)) | SVTX | RUSR | WUSR | XUSR | RGRP | WGRP | XGRP | ROTH | WOTH | XOTH
        } else {
            CHMOD_MODE_BITS
        };

        return Some(make_node_op_equals(mode, mentioned));
    }

    // Handle symbolic mode
    while let Some(c) = p.peek() {
        let mut affected = 0;

        // Parse [ugoa]*
        loop {
            match p.peek() {
                Some('u') => {
                    affected |= SUID | RUSR | WUSR | XUSR;
                    p.next();
                }
                Some('g') => {
                    affected |= SGID | RGRP | WGRP | XGRP;
                    p.next();
                }
                Some('o') => {
                    affected |= SVTX | ROTH | WOTH | XOTH;
                    p.next();
                }
                Some('a') => {
                    affected |= CHMOD_MODE_BITS;
                    p.next();
                }
                Some('=') | Some('+') | Some('-') => break,
                _ => return None,
            }
        }

        // Parse [-+=]...
        loop {
            let op = match p.next() {
                Some('=') => '=',
                Some('+') => '+',
                Some('-') => '-',
                _ => return None,
            };

            let mut value = 0;
            let mut mentioned = 0;
            let mut flag = ModeFlag::CopyExisting;

            match p.peek() {
                Some(c) if c.is_ascii_digit() && *c < '8' => {
                    // Octal mode
                    let mut octal_mode = 0;
                    while let Some(c) = p.peek() {
                        if !c.is_ascii_digit() || *c >= '8' {
                            break;
                        }
                        octal_mode = 8 * octal_mode + c.to_digit(8).unwrap();
                        p.next();
                        if octal_mode > ALLM {
                            return None;
                        }
                    }

                    if affected != 0 || p.peek().is_some() && *p.peek().unwrap() != ',' {
                        return None;
                    }
                    affected = CHMOD_MODE_BITS;
                    mentioned = CHMOD_MODE_BITS;
                    value = octal_to_mode(octal_mode);
                    flag = ModeFlag::OrdinaryChange;
                }
                Some('u') => {
                    value = RUSR | WUSR | XUSR;
                    p.next();
                }
                Some('g') => {
                    value = RGRP | WGRP | XGRP;
                    p.next();
                }
                Some('o') => {
                    value = ROTH | WOTH | XOTH;
                    p.next();
                }
                _ => {
                    // Parse [rwxXst]*
                    flag = ModeFlag::OrdinaryChange;
                    while let Some(c) = p.peek() {
                        match c {
                            'r' => value |= RUSR | RGRP | ROTH,
                            'w' => value |= WUSR | WGRP | WOTH,
                            'x' => value |= XUSR | XGRP | XOTH,
                            'X' => flag = ModeFlag::XIfAnyX,
                            's' => value |= SUID | SGID,
                            't' => value |= SVTX,
                            _ => break,
                        }
                        p.next();
                    }
                }
            }

            mc.push(ModeChange {
                op,
                flag,
                affected,
                value,
                mentioned: if mentioned != 0 {
                    mentioned
                } else if affected != 0 {
                    affected & value
                } else {
                    value
                },
            });

            if p.peek() != Some(&'=') && p.peek() != Some(&'+') && p.peek() != Some(&'-') {
                break;
            }
        }

        if p.peek() != Some(&',') {
            break;
        }
        p.next(); // Skip comma
    }

    if p.peek().is_none() {
        mc.push(ModeChange {
            op: ' ',
            flag: ModeFlag::Done,
            affected: 0,
            value: 0,
            mentioned: 0,
        });
        Some(mc)
    } else {
        None
    }
}

// Create mode change from reference file
pub fn mode_create_from_ref(ref_file: &Path) -> std::io::Result<Vec<ModeChange>> {
    let metadata = fs::metadata(ref_file)?;
    Ok(make_node_op_equals(metadata.mode() & CHMOD_MODE_BITS, CHMOD_MODE_BITS))
}

// Adjust mode according to changes
pub fn mode_adjust(
    oldmode: u32,
    dir: bool,
    umask_value: u32,
    changes: &[ModeChange],
) -> (u32, u32) {
    let mut newmode = oldmode & CHMOD_MODE_BITS;
    let mut mode_bits = 0;

    for change in changes {
        if let ModeFlag::Done = change.flag {
            break;
        }

        let mut affected = change.affected;
        let omit_change = if dir {
            (SUID | SGID) & !change.mentioned
        } else {
            0
        };
        let mut value = change.value;

        match change.flag {
            ModeFlag::OrdinaryChange => {}
            ModeFlag::CopyExisting => {
                value &= newmode;
                value |= if (value & (RUSR | RGRP | ROTH)) != 0 {
                    RUSR | RGRP | ROTH
                } else {
                    0
                } | if (value & (WUSR | WGRP | WOTH)) != 0 {
                    WUSR | WGRP | WOTH
                } else {
                    0
                } | if (value & (XUSR | XGRP | XOTH)) != 0 {
                    XUSR | XGRP | XOTH
                } else {
                    0
                };
            }
            ModeFlag::XIfAnyX => {
                if (newmode & (XUSR | XGRP | XOTH)) != 0 || dir {
                    value |= XUSR | XGRP | XOTH;
                }
            }
            _ => {}
        }

        value &= (if affected != 0 { affected } else { !umask_value }) & !omit_change;

        match change.op {
            '=' => {
                let preserved = if affected != 0 { !affected } else { 0 } | omit_change;
                mode_bits |= CHMOD_MODE_BITS & !preserved;
                newmode = (newmode & preserved) | value;
            }
            '+' => {
                mode_bits |= value;
                newmode |= value;
            }
            '-' => {
                mode_bits |= value;
                newmode &= !value;
            }
            _ => {}
        }
    }

    (newmode, mode_bits)
}