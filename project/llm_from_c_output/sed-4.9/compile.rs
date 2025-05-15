/* 
   GNU SED, a batch stream editor.
   Copyright (C) 1989-2022 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; If not, see <https://www.gnu.org/licenses/>.
*/

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write, BufReader, BufWriter},
    path::Path,
    str,
    collections::HashMap,
    ffi::CString,
    os::unix::ffi::OsStrExt,
    mem,
    ptr,
    ffi::CStr,
    os::raw::c_char,
    fmt,
    error::Error,
};

const YMAP_LENGTH: usize = 256;
const VECTOR_ALLOC_INCREMENT: usize = 40;

const OPEN_BRACKET: char = '[';
const CLOSE_BRACKET: char = ']';
const CLOSE_BRACE: char = '}';

struct ProgInfo {
    base: *const u8,
    cur: *const u8,
    end: *const u8,
    file: Option<File>,
}

struct ErrorInfo {
    name: Option<String>,
    line: u64,
    string_expr_count: u64,
}

struct SedLabel {
    v_index: usize,
    name: String,
    err_info: ErrorInfo,
    next: Option<Box<SedLabel>>,
}

struct SpecialFiles {
    outf: Output,
    pfp: Option<File>,
}

struct Output {
    name: String,
    missing_newline: bool,
    fp: Option<File>,
    link: Option<Box<Output>>,
}

static mut MY_STDIN: Option<File> = None;
static mut MY_STDOUT: Option<File> = None;
static mut MY_STDERR: Option<File> = None;

static mut SPECIAL_FILES: [SpecialFiles; 4] = [
    SpecialFiles {
        outf: Output {
            name: "/dev/stdin".to_string(),
            missing_newline: false,
            fp: None,
            link: None,
        },
        pfp: None,
    },
    SpecialFiles {
        outf: Output {
            name: "/dev/stdout".to_string(),
            missing_newline: false,
            fp: None,
            link: None,
        },
        pfp: None,
    },
    SpecialFiles {
        outf: Output {
            name: "/dev/stderr".to_string(),
            missing_newline: false,
            fp: None,
            link: None,
        },
        pfp: None,
    },
    SpecialFiles {
        outf: Output {
            name: String::new(),
            missing_newline: false,
            fp: None,
            link: None,
        },
        pfp: None,
    },
];

static mut PROG: ProgInfo = ProgInfo {
    base: ptr::null(),
    cur: ptr::null(),
    end: ptr::null(),
    file: None,
};

static mut CUR_INPUT: ErrorInfo = ErrorInfo {
    name: None,
    line: 0,
    string_expr_count: 0,
};

static mut JUMPS: Option<Box<SedLabel>> = None;
static mut LABELS: Option<Box<SedLabel>> = None;
static mut FIRST_SCRIPT: bool = true;
static mut PENDING_TEXT: Option<Vec<u8>> = None;
static mut OLD_TEXT_BUF: Option<Vec<u8>> = None;
static mut BLOCKS: Option<Box<SedLabel>> = None;

struct SedCmd {
    a1: Option<Addr>,
    a2: Option<Addr>,
    range_state: RangeState,
    addr_bang: bool,
    cmd: char,
    x: CmdX,
}

enum CmdX {
    JumpIndex(usize),
    IntArg(i32),
    ReadCmd { fname: String, append: bool },
    WriteCmd { outf: Output },
    Subst(Subst),
    Translate(Vec<u8>),
    TranslateMB(Vec<String>),
    CmdTxt { text: Vec<u8>, text_length: usize },
    LabelName(String),
}

struct Addr {
    addr_type: AddrType,
    addr_step: u64,
    addr_number: u64,
    addr_regex: Option<String>,
}

enum AddrType {
    IsNull,
    IsRegex,
    IsNum,
    IsNumMod,
    IsStep,
    IsStepMod,
    IsLast,
}

enum RangeState {
    Inactive,
    Active,
    Pending,
}

struct Subst {
    global: bool,
    print: bool,
    eval: bool,
    numb: u32,
    outf: Option<Output>,
    regx: Option<String>,
    replacement: Option<String>,
    max_id: u32,
}

fn bad_command(ch: char) -> ! {
    let msg = format!("unknown command: `{}'", ch);
    bad_prog(&msg);
}

fn bad_prog(why: &str) -> ! {
    unsafe {
        if let Some(name) = &CUR_INPUT.name {
            eprintln!(
                "{}: file {} line {}: {}",
                program_name(),
                name,
                CUR_INPUT.line,
                why
            );
        } else {
            eprintln!(
                "{}: -e expression #{}, char {}: {}",
                program_name(),
                CUR_INPUT.string_expr_count,
                (PROG.cur as usize - PROG.base as usize),
                why
            );
        }
    }
    std::process::exit(1);
}

fn program_name() -> &'static str {
    std::env::args().next().unwrap_or_default()
}

fn inchar() -> Option<u8> {
    unsafe {
        if !PROG.cur.is_null() {
            if PROG.cur < PROG.end {
                let ch = *PROG.cur;
                PROG.cur = PROG.cur.add(1);
                if ch == b'\n' {
                    CUR_INPUT.line += 1;
                }
                Some(ch)
            } else {
                None
            }
        } else if let Some(file) = &mut PROG.file {
            let mut buf = [0];
            match file.read_exact(&mut buf) {
                Ok(_) => {
                    if buf[0] == b'\n' {
                        CUR_INPUT.line += 1;
                    }
                    Some(buf[0])
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

fn savchar(ch: Option<u8>) {
    if ch.is_none() {
        return;
    }
    let ch = ch.unwrap();
    unsafe {
        if ch == b'\n' && CUR_INPUT.line > 0 {
            CUR_INPUT.line -= 1;
        }
        if !PROG.cur.is_null() {
            if PROG.cur > PROG.base {
                PROG.cur = PROG.cur.sub(1);
                if *PROG.cur != ch {
                    panic!("Called savchar with unexpected pushback");
                }
            }
        } else if let Some(file) = &mut PROG.file {
            file.seek(io::SeekFrom::Current(-1)).unwrap();
        }
    }
}

fn in_nonblank() -> Option<u8> {
    loop {
        let ch = inchar();
        if ch.is_none() || !ch.unwrap().is_ascii_whitespace() {
            return ch;
        }
    }
}

fn read_end_of_cmd() {
    let ch = in_nonblank();
    match ch {
        Some(b'}') | Some(b'#') => savchar(ch),
        Some(b'\n') | Some(b';') | None => (),
        _ => bad_prog("extra characters after command"),
    }
}

fn in_integer(mut ch: u8) -> u64 {
    let mut num = 0;
    while ch.is_ascii_digit() {
        num = num * 10 + (ch - b'0') as u64;
        ch = inchar().unwrap_or(0);
    }
    savchar(Some(ch));
    num
}

fn add_then_next(b: &mut Vec<u8>, ch: u8) -> Option<u8> {
    b.push(ch);
    inchar()
}

fn convert_number(result: &mut u8, buf: &[u8], bufend: usize, base: u32) -> usize {
    let mut n = 0;
    let mut max = 1;
    let mut p = 1;

    while p < bufend && max <= 255 {
        let d = match buf[p] {
            b'0'..=b'9' => (buf[p] - b'0') as i32,
            b'A' | b'a' => 0xa,
            b'B' | b'b' => 0xb,
            b'C' | b'c' => 0xc,
            b'D' | b'd' => 0xd,
            b'E' | b'e' => 0xe,
            b'F' | b'f' => 0xf,
            _ => -1,
        };

        if d < 0 || base as i32 <= d {
            break;
        }

        n = n * base as i32 + d;
        max *= base;
        p += 1;
    }

    if p == 1 {
        *result = buf[0];
    } else {
        *result = n as u8;
    }
    p
}

fn read_filename() -> Vec<u8> {
    if sandbox() {
        bad_prog("e/r/w commands disabled in sandbox mode");
    }

    let mut b = Vec::new();
    let mut ch = in_nonblank();

    while let Some(c) = ch {
        if c == b'\n' {
            break;
        }
        ch = add_then_next(&mut b, c);
    }

    b.push(b'\0');
    b
}

fn get_openfile(file_ptrs: &mut Option<Box<Output>>, mode: &str, fail: bool) -> Output {
    let b = read_filename();
    let file_name = String::from_utf8_lossy(&b).trim_end_matches('\0').to_string();

    if file_name.is_empty() {
        bad_prog("missing filename in r/R/w/W commands");
    }

    let mut p = file_ptrs.as_ref();
    while let Some(ptr) = p {
        if ptr.name == file_name {
            break;
        }
        p = ptr.link.as_ref();
    }

    if posixicity() == Posixicity::PosixlyExtended {
        unsafe {
            MY_STDIN = Some(std::io::stdin());
            MY_STDOUT = Some(std::io::stdout());
            MY_STDERR = Some(std::io::stderr());

            for special in &mut SPECIAL_FILES {
                if special.outf.name == file_name {
                    special.outf.fp = special.pfp.take();
                    return special.outf.clone();
                }
            }
        }
    }

    let mut p = file_ptrs.as_ref();
    while let Some(ptr) = p {
        if ptr.name == file_name {
            break;
        }
        p = ptr.link.as_ref();
    }

    if p.is_none() {
        let mut outf = Output {
            name: file_name.clone(),
            fp: None,
            missing_newline: false,
            link: file_ptrs.take(),
        };

        outf.fp = match OpenOptions::new()
            .write(mode.contains('w'))
            .read(mode.contains('r'))
            .create(mode.contains('w'))
            .truncate(mode.contains('w'))
            .open(&file_name)
        {
            Ok(file) => Some(file),
            Err(e) if fail => {
                eprintln!("Failed to open file {}: {}", file_name, e);
                std::process::exit(1);
            }
            Err(_) => None,
        };

        *file_ptrs = Some(Box::new(outf));
        file_ptrs.as_ref().unwrap().clone()
    } else {
        p.unwrap().clone()
    }
}

fn next_cmd_entry(vector: &mut Vec<SedCmd>) -> &mut SedCmd {
    vector.push(SedCmd {
        a1: None,
        a2: None,
        range_state: RangeState::Inactive,
        addr_bang: false,
        cmd: '\0',
        x: CmdX::JumpIndex(0),
    });
    vector.last_mut().unwrap()
}

fn snarf_char_class(b: &mut Vec<u8>) -> Option<u8> {
    let mut ch = inchar()?;
    let mut state = 0;

    if ch == b'^' {
        ch = add_then_next(b, ch)?;
    }
    if ch == CLOSE_BRACKET as u8 {
        ch = add_then_next(b, ch)?;
    }

    loop {
        ch = add_then_next(b, ch)?;

        match ch {
            b'\n' | 0 => return Some(ch),
            b'.' | b':' | b'=' => {
                if state == 1 {
                    state = 2;
                } else if state == 2 && ch == b'.' {
                    state = 3;
                }
            }
            b'[' => {
                if state == 0 {
                    state = 1;
                }
            }
            b']' => {
                if state == 0 || state == 1 {
                    return Some(ch);
                } else if state == 3 {
                    state = 0;
                }
            }
            _ => (),
        }

        state &= !1;
    }
}

fn match_slash(slash: u8, regex: bool) -> Option<Vec<u8>> {
    let mut b = Vec::new();
    let mut cur_stat = 0;

    let mut ch = inchar()?;
    while ch != b'\n' {
        if ch == slash {
            return Some(b);
        } else if ch == b'\\' {
            ch = inchar()?;
            if ch != b'\n' && (ch != slash || (!regex && ch == b'&')) {
                b.push(b'\\');
            }
        } else if ch == OPEN_BRACKET as u8 && regex {
            b.push(ch);
            ch = snarf_char_class(&mut b)?;
            if ch != CLOSE_BRACKET as u8 {
                break;
            }
        }

        b.push(ch);
        ch = inchar()?;
    }

    if ch == b'\n' {
        savchar(Some(ch));
    }
    None
}

fn mark_subst_opts(cmd: &mut Subst) -> u32 {
    let mut flags = 0;
    loop {
        let ch = in_nonblank();
        match ch {
            Some(b'i') | Some(b'I') => {
                if posixicity() == Posixicity::PosixlyBasic {
                    bad_prog("unknown option to `s'");
                }
                flags |= 1; // REG_ICASE
            }
            Some(b'm') | Some(b'M') => {
                if posixicity() == Posixicity::PosixlyBasic {
                    bad_prog("unknown option to `s'");
                }
                flags |= 2; // REG_NEWLINE
            }
            Some(b'e') => {
                if posixicity() == Posixicity::PosixlyBasic {
                    bad_prog("unknown option to `s'");
                }
                cmd.eval = true;
            }
            Some(b'p') => {
                if cmd.print {
                    bad_prog("multiple `p' options to `s' command");
                }
                cmd.print |= 1 << cmd.eval as u32;
            }
            Some(b'g') => {
                if cmd.global {
                    bad_prog("multiple `g' options to `s' command");
                }
                cmd.global = true;
            }
            Some(b'w') => {
                cmd.outf = Some(get_openfile(&mut file_write, "w", true));
                return flags;
            }
            Some(b'0'..=b'9') => {
                if cmd.numb != 0 {
                    bad_prog("multiple number options to `s' command");
                }
                cmd.numb = in_integer(ch.unwrap());
                if cmd.numb == 0 {
                    bad_prog("number option to `s' command may not be zero");
                }
            }
            Some(b'}') | Some(b'#') => {
                savchar(ch);
                return flags;
            }
            Some(b'\n') | Some(b';') | None => return flags,
            Some(b'\r') => {
                if inchar() == Some(b'\n') {
                    return flags;
                }
            }
            _ => bad_prog("unknown option to `s'"),
        }
    }
}

fn read_label() -> String {
    let mut b = Vec::new();
    let mut ch = in_nonblank();

    while let Some(c) = ch {
        if c == b'\n' || c.is_ascii_whitespace() || c == b';' || c == CLOSE_BRACE as u8 || c == b'#' {
            savchar(Some(c));
            break;
        }
        ch = add_then_next(&mut b, c);
    }

    b.push(b'\0');
    String::from_utf8_lossy(&b).trim_end_matches('\0').to_string()
}

fn setup_label(
    list: Option<Box<SedLabel>>,
    idx: usize,
    name: String,
    err_info: Option<ErrorInfo>,
) -> Option<Box<SedLabel>> {
    Some(Box::new(SedLabel {
        v_index: idx,
        name,
        err_info: err_info.unwrap_or(ErrorInfo {
            name: None,
            line: 0,
            string_expr_count: 0,
        }),
        next: list,
    }))
}

fn release_label(list_head: Option<Box<SedLabel>>) -> Option<Box<SedLabel>> {
    list_head.and_then(|mut head| {
        let next = head.next.take();
        next