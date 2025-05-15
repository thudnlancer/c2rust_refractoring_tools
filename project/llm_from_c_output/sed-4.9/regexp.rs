/*  GNU SED, a batch stream editor.
    Copyright (C) 1999-2022 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; If not, see <https://www.gnu.org/licenses/>. */

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::slice;
use std::cmp;
use std::env;
use std::collections::HashMap;
use regex_syntax::hir::Hir;
use regex_syntax::Parser;
use regex::Regex;
use regex::bytes::Regex as BytesRegex;

static ERRORS: &[&str] = &[
    "no previous regular expression",
    "cannot specify modifiers on empty regexp",
];

const NO_REGEX: &str = ERRORS[0];
const BAD_MODIF: &str = ERRORS[1];

#[derive(Debug)]
struct RegexFlags {
    icase: bool,
    newline: bool,
    extended: bool,
}

#[derive(Debug)]
struct RegexPattern {
    re: Regex,
    bytes_re: BytesRegex,
    fastmap: Option<Vec<u8>>,
    newline_anchor: bool,
    translate: Option<Vec<u8>>,
    no_sub: bool,
    regs_allocated: bool,
}

#[derive(Debug)]
struct RegexInfo {
    re: String,
    sz: usize,
    flags: RegexFlags,
    pattern: RegexPattern,
    begline: bool,
    endline: bool,
    dfa: Option<DFA>,
}

impl RegexInfo {
    fn new(re: String, sz: usize, flags: RegexFlags) -> Self {
        RegexInfo {
            re,
            sz,
            flags,
            pattern: RegexPattern {
                re: Regex::new("").unwrap(),
                bytes_re: BytesRegex::new("").unwrap(),
                fastmap: None,
                newline_anchor: false,
                translate: None,
                no_sub: false,
                regs_allocated: false,
            },
            begline: false,
            endline: false,
            dfa: None,
        }
    }
}

#[derive(Debug)]
struct DFA;

fn dfaalloc() -> Box<DFA> {
    Box::new(DFA)
}

fn dfafree(_dfa: Box<DFA>) {}

fn dfasyntax(_dfa: &mut DFA, _localeinfo: &(), _syntax: u32, _dfaopts: u32) {}

fn dfacomp(_re: &str, _sz: usize, _dfa: &mut DFA, _fast: bool) {}

fn dfasuperset(_dfa: &DFA) -> Option<&DFA> {
    None
}

fn dfaexec(_dfa: &DFA, _buf: &[u8], _end: &[u8], _bol: bool, _regs: Option<&mut Registers>, _backref: &mut bool) -> bool {
    false
}

fn dfaisfast(_dfa: &DFA) -> bool {
    false
}

#[derive(Debug)]
struct Registers {
    start: Vec<i32>,
    end: Vec<i32>,
    num_regs: usize,
}

impl Registers {
    fn new(num_regs: usize) -> Self {
        Registers {
            start: vec![-1; num_regs],
            end: vec![-1; num_regs],
            num_regs,
        }
    }
}

fn panic(msg: &str) {
    eprintln!("{}", msg);
    std::process::exit(1);
}

fn bad_prog(msg: &str) {
    panic(msg);
}

fn compile_regex_1(new_regex: &mut RegexInfo, needed_sub: i32) {
    let syntax = if new_regex.flags.extended {
        regex_syntax::hir::Hir::empty().kind().to_owned()
    } else {
        regex_syntax::hir::Hir::empty().kind().to_owned()
    };

    let syntax = syntax & !regex_syntax::hir::DotNotNewline;
    let syntax = syntax | regex_syntax::hir::NoPosixBacktracking;

    let posixicity = 0; // TODO: Define proper posixicity enum
    let syntax = match posixicity {
        0 => syntax & !regex_syntax::hir::UnmatchedRightParenOrd,
        1 => syntax | regex_syntax::hir::UnmatchedRightParenOrd,
        2 => syntax | regex_syntax::hir::UnmatchedRightParenOrd | regex_syntax::hir::NoGnuOps,
        _ => syntax,
    };

    if new_regex.flags.icase {
        // TODO: Handle case insensitive
    } else {
        new_regex.pattern.fastmap = Some(vec![0; 256]);
    }

    if needed_sub == 0 {
        // TODO: Handle no_sub
    }

    if new_regex.flags.newline {
        // TODO: Handle newline flags
    }

    let re = match Regex::new(&new_regex.re) {
        Ok(re) => re,
        Err(e) => {
            bad_prog(&e.to_string());
            return;
        }
    };
    new_regex.pattern.re = re;

    let bytes_re = match BytesRegex::new(&new_regex.re) {
        Ok(re) => re,
        Err(e) => {
            bad_prog(&e.to_string());
            return;
        }
    };
    new_regex.pattern.bytes_re = bytes_re;

    new_regex.pattern.newline_anchor = false; // TODO: Handle buffer_delimiter

    if new_regex.flags.icase {
        let mut translate = vec![0; 256];
        for i in 0..256 {
            translate[i] = i.to_ascii_lowercase() as u8;
        }
        new_regex.pattern.translate = Some(translate);
    }

    if needed_sub > 0 && posixicity == 0 {
        let msg = format!("invalid reference \\{} on `s' command's RHS", needed_sub - 1);
        bad_prog(&msg);
    }

    let dfaopts = 0; // TODO: Handle buffer_delimiter
    new_regex.dfa = Some(*dfaalloc());
    dfasyntax(new_regex.dfa.as_mut().unwrap(), &(), 0, dfaopts);
    dfacomp(&new_regex.re, new_regex.sz, new_regex.dfa.as_mut().unwrap(), true);

    if new_regex.sz == 1 {
        if new_regex.re.starts_with('^') {
            new_regex.begline = true;
        }
        if new_regex.re.starts_with('$') {
            new_regex.endline = true;
        }
    }
}

fn compile_regex(b: &[u8], flags: RegexFlags, needed_sub: i32) -> Option<Box<RegexInfo>> {
    if b.is_empty() {
        // TODO: Check flags
        bad_prog(BAD_MODIF);
        return None;
    }

    let re_len = b.len();
    let re = String::from_utf8_lossy(b).into_owned();
    let mut new_regex = Box::new(RegexInfo::new(re, re_len, flags));

    // TODO: Handle normalize_text
    compile_regex_1(&mut new_regex, needed_sub);
    Some(new_regex)
}

fn match_regex(
    regex: Option<&mut RegexInfo>,
    buf: &[u8],
    buflen: usize,
    buf_start_offset: usize,
    regarray: Option<&mut Registers>,
    regsize: usize,
) -> bool {
    static mut REGEX_LAST: Option<Box<RegexInfo>> = None;

    let regex = match regex {
        Some(r) => {
            unsafe {
                REGEX_LAST = Some(Box::new(RegexInfo {
                    re: r.re.clone(),
                    sz: r.sz,
                    flags: RegexFlags {
                        icase: r.flags.icase,
                        newline: r.flags.newline,
                        extended: r.flags.extended,
                    },
                    pattern: RegexPattern {
                        re: r.pattern.re.clone(),
                        bytes_re: r.pattern.bytes_re.clone(),
                        fastmap: r.pattern.fastmap.clone(),
                        newline_anchor: r.pattern.newline_anchor,
                        translate: r.pattern.translate.clone(),
                        no_sub: r.pattern.no_sub,
                        regs_allocated: r.pattern.regs_allocated,
                    },
                    begline: r.begline,
                    endline: r.endline,
                    dfa: None,
                }));
            }
            r
        }
        None => unsafe {
            match &REGEX_LAST {
                Some(r) => &mut *(r as *const _ as *mut _),
                None => {
                    bad_prog(NO_REGEX);
                    return false;
                }
            }
        },
    };

    if buflen >= i32::MAX as usize {
        panic("regex input buffer length larger than INT_MAX");
    }

    if regex.pattern.no_sub && regsize > 0 {
        if let Some(dfa) = regex.dfa.take() {
            dfafree(Box::new(dfa));
        }
        // TODO: Handle regfree
        compile_regex_1(regex, regsize as i32);
    }

    regex.pattern.regs_allocated = true;

    if regex.begline || regex.endline {
        let offset = if regex.endline {
            if regex.flags.newline {
                buf[buf_start_offset..]
                    .iter()
                    .position(|&b| b == b'\n')
                    .map(|p| p + buf_start_offset)
                    .unwrap_or(buflen)
            } else {
                buflen
            }
        } else if buf_start_offset == 0 {
            0
        } else if !regex.flags.newline {
            return false;
        } else if buf[buf_start_offset - 1] == b'\n' {
            buf_start_offset
        } else {
            match buf[buf_start_offset..].iter().position(|&b| b == b'\n') {
                Some(p) => p + buf_start_offset + 1,
                None => return false,
            }
        };

        if let Some(regs) = regarray {
            if regs.start.is_empty() {
                *regs = Registers::new(1);
            }
            regs.start[0] = offset as i32;
            regs.end[0] = offset as i32;
            for i in 1..regs.num_regs {
                regs.start[i] = -1;
                regs.end[i] = -1;
            }
        }
        return true;
    }

    if buf_start_offset == 0 {
        let superset = dfasuperset(regex.dfa.as_ref().unwrap());
        if superset.is_some() && !dfaexec(superset.unwrap(), buf, &[], true, None, &mut false) {
            return false;
        }

        if (regsize == 0 && regex.flags.newline) || (superset.is_none() && dfaisfast(regex.dfa.as_ref().unwrap())) {
            let mut backref = false;
            if !dfaexec(regex.dfa.as_ref().unwrap(), buf, &[], true, None, &mut backref) {
                return false;
            }
            if regsize == 0 && regex.flags.newline && !backref {
                return true;
            }
        }
    }

    if regex.flags.newline {
        let mut beg = &buf[0..];
        if buf_start_offset > 0 {
            if let Some(eol) = buf[..buf_start_offset].iter().rposition(|&b| b == b'\n') {
                beg = &buf[eol + 1..];
            }
        }

        let mut start = &buf[buf_start_offset..];
        loop {
            let end = match beg.iter().position(|&b| b == b'\n') {
                Some(pos) => &beg[..pos],
                None => beg,
            };

            let ret = regex.pattern.bytes_re.find_at(end, 0).is_some();
            if ret {
                // TODO: Handle register offsets
                return true;
            }

            if end.len() == beg.len() {
                break;
            }
            beg = &beg[end.len() + 1..];
            start = beg;
        }
    } else {
        let ret = regex.pattern.bytes_re.find_at(&buf[buf_start_offset..], 0).is_some();
        return ret;
    }

    false
}

fn release_regex(regex: Box<RegexInfo>) {
    if let Some(dfa) = regex.dfa {
        dfafree(Box::new(dfa));
    }
    // TODO: Handle regfree
}