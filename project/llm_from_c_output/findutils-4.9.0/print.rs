// print.rs -- print/printf-related code.
// Copyright (C) 1990-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{
    ffi::{CStr, CString},
    fmt,
    fs::File,
    io::{self, Write},
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::{Path, PathBuf},
    ptr,
    time::{SystemTime, UNIX_EPOCH},
};

use libc::{self, c_char, c_int, c_void, mode_t, stat, time_t, tm};

use crate::{
    defs::*,
    error::{die, error, nonfatal_nontarget_file_error},
    predicate::Predicate,
    system::*,
    xalloc::{xmalloc, xrealloc},
};

#[derive(Debug)]
pub struct Segment {
    pub segkind: i32,
    pub format_char: [u8; 2],
    pub next: *mut Segment,
    pub text_len: usize,
    pub text: *mut u8,
}

#[derive(Debug)]
pub struct FormatVal {
    pub stream: *mut libc::FILE,
    pub filename: *const c_char,
    pub quote_opts: i32,
    pub dest_is_tty: bool,
    pub segment: *mut Segment,
}

#[derive(Debug)]
pub struct ParserTable {
    // Fields would depend on actual usage
}

extern "C" {
    fn localtime(time: *const time_t) -> *mut tm;
    fn strftime(
        s: *mut c_char,
        maxsize: usize,
        format: *const c_char,
        timeptr: *const tm,
    ) -> usize;
    fn getgrgid(gid: u32) -> *mut libc::group;
    fn getpwuid(uid: u32) -> *mut libc::passwd;
    fn human_readable(
        val: u64,
        buf: *mut c_char,
        ceiling: i32,
        blocksize: u64,
        blockcount: u64,
    ) -> *mut c_char;
    fn filemodestring(stat_buf: *const stat, modestring: *mut c_char);
    fn filesystem_type(stat_buf: *const stat, pathname: *const c_char) -> *const c_char;
    fn safely_quote_err_filename(flags: i32, pathname: *const c_char) -> *const c_char;
    fn areadlinkat(
        dirfd: i32,
        pathname: *const c_char,
        buffer: *mut *mut c_char,
        bufsize: *mut usize,
    ) -> i32;
    fn freecon(scontext: *mut c_char);
    fn xgetfilecon(
        dirfd: i32,
        pathname: *const c_char,
        scontext: *mut *mut c_char,
    ) -> i32;
}

const KIND_PLAIN: i32 = 0;
const KIND_STOP: i32 = 1;
const KIND_FORMAT: i32 = 2;

const MODE_ALL: mode_t = 0o7777;

const S_IFMT: mode_t = 0o170000;
const S_IFREG: mode_t = 0o100000;
const S_IFDIR: mode_t = 0o040000;
const S_IFLNK: mode_t = 0o120000;
const S_IFSOCK: mode_t = 0o140000;
const S_IFBLK: mode_t = 0o060000;
const S_IFCHR: mode_t = 0o020000;
const S_IFIFO: mode_t = 0o010000;
const S_ISUID: mode_t = 0o4000;
const S_ISGID: mode_t = 0o2000;
const S_ISVTX: mode_t = 0o1000;
const S_IRUSR: mode_t = 0o400;
const S_IWUSR: mode_t = 0o200;
const S_IXUSR: mode_t = 0o100;
const S_IRGRP: mode_t = 0o40;
const S_IWGRP: mode_t = 0o20;
const S_IXGRP: mode_t = 0o10;
const S_IROTH: mode_t = 0o4;
const S_IWOTH: mode_t = 0o2;
const S_IXOTH: mode_t = 0o1;

const ST_NBLOCKSIZE: u64 = 512;
const ST_NBLOCKS: fn(stat) -> i64 = |s| s.st_blocks;

const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const HUGE_VAL: f64 = f64::INFINITY;
const LONGEST_HUMAN_READABLE: usize = 19;

static mut state: State = State {
    cwd_dir_fd: 0,
    rel_pathname: ptr::null(),
    starting_path_length: 0,
    curdepth: 0,
    exit_status: 0,
};

static mut options: Options = Options {
    x_getfilecon: xgetfilecon,
};

#[derive(Debug)]
struct State {
    cwd_dir_fd: i32,
    rel_pathname: *const c_char,
    starting_path_length: usize,
    curdepth: i32,
    exit_status: i32,
}

#[derive(Debug)]
struct Options {
    x_getfilecon: extern "C" fn(i32, *const c_char, *mut *mut c_char) -> i32,
}

#[no_mangle]
pub extern "C" fn make_segment(
    segment: *mut *mut Segment,
    format: *mut c_char,
    len: i32,
    kind: i32,
    format_char: c_char,
    aux_format_char: c_char,
    pred: *mut Predicate,
) -> *mut *mut Segment {
    unsafe {
        let mycost = EvaluationCost::NeedsNothing;
        let mut fmt: *mut c_char;

        assert!(format_char != b'{' as c_char);
        assert!(format_char != b'[' as c_char);
        assert!(format_char != b'(' as c_char);

        *segment = xmalloc(std::mem::size_of::<Segment>()) as *mut Segment;

        (*segment).segkind = kind;
        (*segment).format_char[0] = format_char as u8;
        (*segment).format_char[1] = aux_format_char as u8;
        (*segment).next = ptr::null_mut();
        (*segment).text_len = len as usize;

        fmt = (*segment).text = xmalloc(len as usize + std::mem::size_of_val(&b"d"[0])) as *mut u8;
        libc::strncpy(fmt as *mut c_char, format, len as usize);
        fmt = fmt.add(len as usize);

        if kind == KIND_PLAIN || kind == KIND_STOP {
            assert!(format_char == 0);
            assert!(aux_format_char == 0);
            *fmt = b'\0';
            if mycost as u32 > (*pred).p_cost as u32 {
                (*pred).p_cost = EvaluationCost::NeedsNothing;
            }
            return &mut (*segment).next;
        }

        assert!(kind == KIND_FORMAT);
        match format_char as u8 {
            b'%' => {
                *fmt = b'%';
                fmt = fmt.add(1);
            }
            b'l' => {
                (*pred).need_stat = true;
                // mycost = NeedsLinkName;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'y' => {
                (*pred).need_type = true;
                // mycost = NeedsType;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'i' => {
                (*pred).need_inum = true;
                // mycost = NeedsInodeNumber;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'a' | b'A' | b'B' | b'c' | b'C' | b'F' | b'g' | b'M' | b's' | b't' | b'T' | b'u' => {
                (*pred).need_stat = true;
                // mycost = NeedsStatInfo;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'S' => {
                (*pred).need_stat = true;
                // mycost = NeedsStatInfo;
                *fmt = b'g';
                fmt = fmt.add(1);
            }
            b'Y' => {
                (*pred).need_stat = true;
                // mycost = NeedsType;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'f' | b'h' | b'p' | b'P' => {
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'Z' => {
                // mycost = NeedsAccessInfo;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'H' => {
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'G' | b'U' | b'b' | b'D' | b'k' | b'n' => {
                (*pred).need_stat = true;
                // mycost = NeedsStatInfo;
                *fmt = b's';
                fmt = fmt.add(1);
            }
            b'd' => {
                *fmt = b'd';
                fmt = fmt.add(1);
            }
            b'm' => {
                *fmt = b'o';
                (*pred).need_stat = true;
                // mycost = NeedsStatInfo;
            }
            _ => {}
        }
        *fmt = b'\0';

        if mycost as u32 > (*pred).p_cost as u32 {
            (*pred).p_cost = mycost;
        }
        &mut (*segment).next
    }
}

#[no_mangle]
pub extern "C" fn insert_fprintf(
    vec: *mut FormatVal,
    entry: *const ParserTable,
    format: *mut c_char,
) -> bool {
    unsafe {
        let segstart = format;
        let mut fmt_editpos: *mut c_char;
        let mut segmentp: *mut *mut Segment;
        let our_pred: *mut Predicate;

        our_pred = insert_primary_withpred(entry, pred_fprintf, format);
        (*our_pred).side_effects = true;
        (*our_pred).no_default_print = true;
        (*our_pred).args.printf_vec = *vec;
        (*our_pred).need_type = false;
        (*our_pred).need_stat = false;
        (*our_pred).p_cost = EvaluationCost::NeedsNothing;

        segmentp = &mut (*our_pred).args.printf_vec.segment;
        *segmentp = ptr::null_mut();

        fmt_editpos = segstart;
        while *fmt_editpos != 0 {
            if fmt_editpos[0] == b'\\' as c_char && fmt_editpos[1] == b'c' as c_char {
                make_segment(
                    segmentp,
                    segstart,
                    fmt_editpos.offset_from(segstart) as i32,
                    KIND_STOP,
                    0,
                    0,
                    our_pred,
                );
                if (*our_pred).need_stat && (*our_pred).p_cost as u32 < EvaluationCost::NeedsStatInfo as u32 {
                    (*our_pred).p_cost = EvaluationCost::NeedsStatInfo;
                }
                return true;
            } else if *fmt_editpos == b'\\' as c_char {
                let mut readpos: usize = 1;
                if fmt_editpos[readpos] == 0 {
                    error(
                        0,
                        0,
                        b"warning: escape `\\' followed by nothing at all\0".as_ptr() as *const _,
                    );
                    readpos -= 1;
                } else if is_octal_char(fmt_editpos[readpos]) {
                    let mut consumed: usize = 0;
                    *fmt_editpos = parse_octal_escape(fmt_editpos.add(readpos), &mut consumed);
                    readpos += consumed;
                } else {
                    let val = parse_escape_char(fmt_editpos[readpos]);
                    if val != 0 {
                        fmt_editpos[0] = val;
                    } else {
                        error(
                            0,
                            0,
                            b"warning: unrecognized escape `\\%c'\0".as_ptr() as *const _,
                            fmt_editpos[readpos],
                        );
                        fmt_editpos = fmt_editpos.add(readpos);
                        continue;
                    }
                }
                segmentp = make_segment(
                    segmentp,
                    segstart,
                    fmt_editpos.offset_from(segstart) as i32 + 1,
                    KIND_PLAIN,
                    0,
                    0,
                    our_pred,
                );
                segstart = fmt_editpos.add(readpos + 1);
                fmt_editpos = fmt_editpos.add(readpos);
            } else if fmt_editpos[0] == b'%' as c_char {
                let len: usize;
                if fmt_editpos[1] == 0 {
                    die(
                        EXIT_FAILURE,
                        0,
                        b"error: %s at end of format string\0".as_ptr() as *const _,
                        fmt_editpos,
                    );
                }

                if fmt_editpos[1] == b'%' as c_char {
                    len = 1;
                } else {
                    len = get_format_flags_length(fmt_editpos);
                }
                fmt_editpos = fmt_editpos.add(len);

                len = get_format_specifer_length(fmt_editpos[0]);
                if len != 0 && fmt_editpos[len - 1] != 0 {
                    let fmt2 = if len == 2 { fmt_editpos[1] } else { 0 };
                    segmentp = make_segment(
                        segmentp,
                        segstart,
                        fmt_editpos.offset_from(segstart) as i32,
                        KIND_FORMAT,
                        fmt_editpos[0],
                        fmt2,
                        our_pred,
                    );
                    fmt_editpos = fmt_editpos.add(len - 1);
                } else {
                    if [b'{', b'[', b'('].contains(&fmt_editpos[0]) {
                        die(
                            EXIT_FAILURE,
                            0,
                            b"error: the format directive `%%%c' is reserved for future use\0"
                                .as_ptr() as *const _,
                            fmt_editpos[0] as i32,
                        );
                    }

                    if len == 2 && fmt_editpos[1] == 0 {
                        error(
                            0,
                            0,
                            b"warning: format directive `%%%c' should be followed by another character\0"
                                .as_ptr() as *const _,
                            fmt_editpos[0],
                        );
                    } else {
                        error(
                            0,
                            0,
                            b"warning: unrecognized format directive `%%%c'\0"
                                .as_ptr() as *const _,
                            fmt_editpos[0],
                        );
                    }
                    segmentp = make_segment(
                        segmentp,
                        segstart,
                        fmt_editpos.offset_from(segstart) as i32 + 1,
                        KIND_PLAIN,
                        0,
                        0,
                        our_pred,
                    );
                }
                segstart = fmt_editpos.add(1);
            }
            fmt_editpos = fmt_editpos.add(1);
        }

        if fmt_editpos > segstart {
            make_segment(
                segmentp,
                segstart,
                fmt_editpos.offset_from(segstart) as i32,
                KIND_PLAIN,
                0,
                0,
                our_pred,
            );
        }
        true
    }
}

unsafe fn is_octal_char(ch: c_char) -> bool {
    ch >= b'0' as c_char && ch <= b'7' as c_char
}

unsafe fn parse_octal_escape(p: *const c_char, consumed: &mut usize) -> c_char {
    let mut n = 0;
    let mut i = 0;
    let mut pos = 0;

    while i < 3 && is_octal_char(p[pos]) {
        n = 8 * n + (p[pos] - b'0' as c_char) as i32;
        i += 1;
        pos += 1;
    }
    pos -= 1;
    *consumed = pos;
    n as c_char
}

unsafe fn parse_escape_char(ch: c_char) -> c_char {
    match ch {
        b'a' => b'\a',
        b'b' => b'\b',
        b'f' => b'\f',
        b'n' => b'\n',
        b'r' => b'\r',
        b't' => b'\t',
        b'v' => b'\v',
        b'\\' => b'\\',
        _ => 0,
    }
}

unsafe fn get_format_flags_length(p: *const c_char) -> usize {
    let mut n = 0;
    while p[n + 1] != 0 && [b'-', b'+', b' ', b'#'].contains(&p[n + 1]) {
        n += 1;
    }
    while isdigit(p[n + 1] as u8) {
        n += 1;
    }
    if p[n + 1] == b'.' as c_char {
        n += 1;
        while isdigit(p[n + 1] as u8) {
            n += 1;
        }
    }
    n
}

unsafe fn