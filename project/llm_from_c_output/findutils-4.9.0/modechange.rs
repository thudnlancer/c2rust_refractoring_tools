/* modechange.rs -- file mode manipulation in Rust

   Copyright (C) 1989-1990, 1997, 2003-2006, 2009-2022 Free Software
   Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/* The traditional octal values corresponding to each mode bit.  */
const SUID: u32 = 0o4000;
const SGID: u32 = 0o2000;
const SVTX: u32 = 0o1000;
const RUSR: u32 = 0o400;
const WUSR: u32 = 0o200;
const XUSR: u32 = 0o100;
const RGRP: u32 = 0o40;
const WGRP: u32 = 0o20;
const XGRP: u32 = 0o10;
const ROTH: u32 = 0o4;
const WOTH: u32 = 0o2;
const XOTH: u32 = 0o1;
const ALLM: u32 = 0o7777; /* all octal mode bits */

/* Special operations flags.  */
#[derive(Debug, Clone, Copy)]
enum ModeFlag {
    Done,
    OrdinaryChange,
    XIfAnyX,
    CopyExisting,
}

/* Description of a mode change.  */
#[derive(Debug)]
struct ModeChange {
    op: char,
    flag: ModeFlag,
    affected: u32,
    value: u32,
    mentioned: u32,
}

impl ModeChange {
    fn new(op: char, flag: ModeFlag, affected: u32, value: u32, mentioned: u32) -> Self {
        ModeChange {
            op,
            flag,
            affected,
            value,
            mentioned,
        }
    }
}

/* Convert OCTAL, which uses one of the traditional octal values, to
   an internal mode_t value.  */
fn octal_to_mode(octal: u32) -> u32 {
    /* Help the compiler optimize the usual case where mode_t uses
       the traditional octal representation.  */
    if (libc::S_ISUID == SUID && libc::S_ISGID == SGID && libc::S_ISVTX == SVTX
        && libc::S_IRUSR == RUSR && libc::S_IWUSR == WUSR && libc::S_IXUSR == XUSR
        && libc::S_IRGRP == RGRP && libc::S_IWGRP == WGRP && libc::S_IXGRP == XGRP
        && libc::S_IROTH == ROTH && libc::S_IWOTH == WOTH && libc::S_IXOTH == XOTH)
    {
        octal
    } else {
        (if octal & SUID != 0 { libc::S_ISUID } else { 0 })
            | (if octal & SGID != 0 { libc::S_ISGID } else { 0 })
            | (if octal & SVTX != 0 { libc::S_ISVTX } else { 0 })
            | (if octal & RUSR != 0 { libc::S_IRUSR } else { 0 })
            | (if octal & WUSR != 0 { libc::S_IWUSR } else { 0 })
            | (if octal & XUSR != 0 { libc::S_IXUSR } else { 0 })
            | (if octal & RGRP != 0 { libc::S_IRGRP } else { 0 })
            | (if octal & WGRP != 0 { libc::S_IWGRP } else { 0 })
            | (if octal & XGRP != 0 { libc::S_IXGRP } else { 0 })
            | (if octal & ROTH != 0 { libc::S_IROTH } else { 0 })
            | (if octal & WOTH != 0 { libc::S_IWOTH } else { 0 })
            | (if octal & XOTH != 0 { libc::S_IXOTH } else { 0 })
    }
}

/* Return a mode_change array with the specified "=ddd"-style
   mode change operation, where NEW_MODE is "ddd" and MENTIONED
   contains the bits explicitly mentioned in the mode are MENTIONED.  */
fn make_node_op_equals(new_mode: u32, mentioned: u32) -> Vec<ModeChange> {
    vec![
        ModeChange::new('=', ModeFlag::OrdinaryChange, libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO, new_mode, mentioned),
        ModeChange::new('\0', ModeFlag::Done, 0, 0, 0),
    ]
}

/* Return a pointer to an array of file mode change operations created from
   MODE_STRING, an ASCII string that contains either an octal number
   specifying an absolute mode, or symbolic mode change operations with
   the form:
   [ugoa...][[+-=][rwxXstugo...]...][,...]

   Return None if 'mode_string' does not contain a valid
   representation of file mode change operations.  */
pub fn mode_compile(mode_string: &str) -> Option<Vec<ModeChange>> {
    let mut mc = Vec::new();
    let mut p = mode_string.chars().peekable();

    if let Some(c) = p.peek() {
        if ('0'..='7').contains(c) {
            let mut octal_mode = 0u32;
            let mut len = 0;
            
            while let Some(c) = p.peek() {
                if !('0'..='7').contains(c) {
                    break;
                }
                octal_mode = 8 * octal_mode + c.to_digit(8).unwrap();
                if octal_mode > ALLM {
                    return None;
                }
                p.next();
                len += 1;
            }

            if p.peek().is_some() {
                return None;
            }

            let mode = octal_to_mode(octal_mode);
            let mentioned = if len < 5 {
                (mode & (libc::S_ISUID | libc::S_ISGID)) | libc::S_ISVTX | libc::S_IRWXUGO
            } else {
                libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO
            };
            return Some(make_node_op_equals(mode, mentioned));
        }
    }

    let mut changes = Vec::new();
    let mut p = mode_string.chars().peekable();

    while let Some(c) = p.peek() {
        let mut affected = 0u32;

        loop {
            match p.peek() {
                Some('u') => {
                    affected |= libc::S_ISUID | libc::S_IRWXU;
                    p.next();
                }
                Some('g') => {
                    affected |= libc::S_ISGID | libc::S_IRWXG;
                    p.next();
                }
                Some('o') => {
                    affected |= libc::S_ISVTX | libc::S_IRWXO;
                    p.next();
                }
                Some('a') => {
                    affected |= libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO;
                    p.next();
                }
                Some('=') | Some('+') | Some('-') => break,
                _ => return None,
            }
        }

        loop {
            let op = match p.next() {
                Some(c @ ('=' | '+' | '-')) => c,
                _ => return None,
            };

            let mut value = 0u32;
            let mut mentioned = 0u32;
            let mut flag = ModeFlag::CopyExisting;

            match p.peek() {
                Some(c) if ('0'..='7').contains(c) => {
                    let mut octal_mode = 0u32;
                    while let Some(c) = p.peek() {
                        if !('0'..='7').contains(c) {
                            break;
                        }
                        octal_mode = 8 * octal_mode + c.to_digit(8).unwrap();
                        if octal_mode > ALLM {
                            return None;
                        }
                        p.next();
                    }

                    if affected != 0 || p.peek().is_some() && *p.peek().unwrap() != ',' {
                        return None;
                    }
                    affected = libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO;
                    mentioned = affected;
                    value = octal_to_mode(octal_mode);
                    flag = ModeFlag::OrdinaryChange;
                }
                Some('u') => {
                    value = libc::S_IRWXU;
                    p.next();
                }
                Some('g') => {
                    value = libc::S_IRWXG;
                    p.next();
                }
                Some('o') => {
                    value = libc::S_IRWXO;
                    p.next();
                }
                _ => {
                    flag = ModeFlag::OrdinaryChange;
                    while let Some(c) = p.peek() {
                        match c {
                            'r' => {
                                value |= libc::S_IRUSR | libc::S_IRGRP | libc::S_IROTH;
                                p.next();
                            }
                            'w' => {
                                value |= libc::S_IWUSR | libc::S_IWGRP | libc::S_IWOTH;
                                p.next();
                            }
                            'x' => {
                                value |= libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH;
                                p.next();
                            }
                            'X' => {
                                flag = ModeFlag::XIfAnyX;
                                p.next();
                            }
                            's' => {
                                value |= libc::S_ISUID | libc::S_ISGID;
                                p.next();
                            }
                            't' => {
                                value |= libc::S_ISVTX;
                                p.next();
                            }
                            _ => break,
                        }
                    }
                }
            }

            mentioned = if mentioned != 0 {
                mentioned
            } else if affected != 0 {
                affected & value
            } else {
                value
            };

            changes.push(ModeChange::new(op, flag, affected, value, mentioned));

            if !matches!(p.peek(), Some('=' | '+' | '-')) {
                break;
            }
        }

        if p.peek() != Some(&',') {
            break;
        }
        p.next();
    }

    if p.peek().is_none() {
        changes.push(ModeChange::new('\0', ModeFlag::Done, 0, 0, 0));
        Some(changes)
    } else {
        None
    }
}

/* Return a file mode change operation that sets permissions to match those
   of REF_FILE.  Return None if REF_FILE can't be accessed.  */
pub fn mode_create_from_ref(ref_file: &Path) -> Option<Vec<ModeChange>> {
    match fs::metadata(ref_file) {
        Ok(metadata) => {
            let mode = metadata.permissions().mode() as u32;
            Some(make_node_op_equals(mode, libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO))
        }
        Err(_) => None,
    }
}

/* Return the file mode bits of OLDMODE (which is the mode of a
   directory if DIR), assuming the umask is UMASK_VALUE, adjusted as
   indicated by the list of change operations CHANGES.  If DIR, the
   type 'X' change affects the returned value even if no execute bits
   were set in OLDMODE, and set user and group ID bits are preserved
   unless CHANGES mentioned them.  If PMODE_BITS is not null, store into
   *PMODE_BITS a mask denoting file mode bits that are affected by
   CHANGES.

   The returned value and *PMODE_BITS contain only file mode bits.
   For example, they have the S_IFMT bits cleared on a standard
   Unix-like host.  */
pub fn mode_adjust(
    oldmode: u32,
    dir: bool,
    umask_value: u32,
    changes: &[ModeChange],
    pmode_bits: Option<&mut u32>,
) -> u32 {
    let mut newmode = oldmode & (libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO);
    let mut mode_bits = 0u32;

    for change in changes {
        if let ModeFlag::Done = change.flag {
            break;
        }

        let mut affected = change.affected;
        let omit_change = if dir {
            (libc::S_ISUID | libc::S_ISGID) & !change.mentioned
        } else {
            0
        };
        let mut value = change.value;

        match change.flag {
            ModeFlag::OrdinaryChange => {}
            ModeFlag::CopyExisting => {
                value &= newmode;
                value |= if value & (libc::S_IRUSR | libc::S_IRGRP | libc::S_IROTH) != 0 {
                    libc::S_IRUSR | libc::S_IRGRP | libc::S_IROTH
                } else {
                    0
                } | if value & (libc::S_IWUSR | libc::S_IWGRP | libc::S_IWOTH) != 0 {
                    libc::S_IWUSR | libc::S_IWGRP | libc::S_IWOTH
                } else {
                    0
                } | if value & (libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH) != 0 {
                    libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH
                } else {
                    0
                };
            }
            ModeFlag::XIfAnyX => {
                if (newmode & (libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH)) != 0 || dir {
                    value |= libc::S_IXUSR | libc::S_IXGRP | libc::S_IXOTH;
                }
            }
            ModeFlag::Done => unreachable!(),
        }

        value &= (if affected != 0 { affected } else { !umask_value }) & !omit_change;

        match change.op {
            '=' => {
                let preserved = (if affected != 0 { !affected } else { 0 }) | omit_change;
                mode_bits |= (libc::S_IRWXU | libc::S_IRWXG | libc::S_IRWXO) & !preserved;
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

    if let Some(pmb) = pmode_bits {
        *pmb = mode_bits;
    }
    newmode
}