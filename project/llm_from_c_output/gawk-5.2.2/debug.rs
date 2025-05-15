/*
 * debug.rs - gawk debugger
 *
 * Copyright (C) 2004, 2010-2013, 2016-2023 the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::{
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    os::unix::prelude::*,
    path::Path,
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

use libc::{c_char, c_int, c_void, O_RDONLY};

use crate::{
    awk::{
        assoc::{assoc_clear, assoc_length, assoc_list, assoc_lookup, assoc_remove, assoc_set},
        eval::{execute_code, interpret, push_context, pop_context},
        instruction::{Instruction, Opcode},
        node::{Node, NodeType},
        regex::Regex,
        symbol::{append_symbol, lookup},
        value::{force_string, valinfo},
    },
    extension::close_extensions,
    io::{nextfile, close_io},
    main::{exit_val, fatal_tag, fatal_tag_valid},
    parser::{parse_program, add_srcfile, free_srcfile},
    re::re_compile,
    symbol_table::{function_list, variable_list},
    util::{os_isatty, os_setbinmode, files_are_same},
};

static mut linebuf: *mut c_char = ptr::null_mut();
static mut linebuf_len: usize = 0;

static mut out_fp: *mut libc::FILE = ptr::null_mut();
static mut dbg_prompt: *const c_char = ptr::null();
static mut commands_prompt: *const c_char = b"> \0".as_ptr() as *const c_char;
static mut eval_prompt: *const c_char = b"@> \0".as_ptr() as *const c_char;

static mut input_from_tty: bool = false;
static mut input_fd: c_int = 0;

static mut cur_srcfile: *mut crate::parser::SrcFile = ptr::null_mut();
static mut cur_frame: i64 = 0;
static mut cur_pc: *mut Instruction = ptr::null_mut();
static mut cur_rule: c_int = 0;

static mut prog_running: bool = false;

struct Condition {
    code: *mut Instruction,
    ctxt: *mut crate::awk::eval::AwkContext,
    expr: *mut c_char,
}

struct CommandsItem {
    next: *mut CommandsItem,
    prev: *mut CommandsItem,
    cmd: c_int,
    cmd_string: *mut c_char,
    arg: *mut crate::debug::CmdArg,
}

struct BreakPoint {
    next: *mut BreakPoint,
    prev: *mut BreakPoint,
    number: c_int,
    ignore_count: i64,
    hit_count: i64,
    src: *mut c_char,
    bpi: *mut Instruction,
    commands: CommandsItem,
    silent: bool,
    cndn: Condition,
    flags: c_int,
}

static mut breakpoints: BreakPoint = BreakPoint {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
    number: 0,
    ignore_count: 0,
    hit_count: 0,
    src: ptr::null_mut(),
    bpi: ptr::null_mut(),
    commands: CommandsItem {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
        cmd: 0,
        cmd_string: ptr::null_mut(),
        arg: ptr::null_mut(),
    },
    silent: false,
    cndn: Condition {
        code: ptr::null_mut(),
        ctxt: ptr::null_mut(),
        expr: ptr::null_mut(),
    },
    flags: 0,
};

static mut last_printed_line: c_int = 0;
static mut last_print_count: c_int = 0;

struct ListItem {
    next: *mut ListItem,
    prev: *mut ListItem,
    number: c_int,
    symbol: *mut Node,
    subs: *mut *mut Node,
    num_subs: c_int,
    sname: *mut c_char,
    fcall_count: i64,
    commands: CommandsItem,
    silent: c_int,
    cndn: Condition,
    value: [Value; 2],
    flags: c_int,
}

union Value {
    n: *mut Node,
    l: i64,
}

static mut display_list: ListItem = ListItem {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
    number: 0,
    symbol: ptr::null_mut(),
    subs: ptr::null_mut(),
    num_subs: 0,
    sname: ptr::null_mut(),
    fcall_count: 0,
    commands: CommandsItem {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
        cmd: 0,
        cmd_string: ptr::null_mut(),
        arg: ptr::null_mut(),
    },
    silent: 0,
    cndn: Condition {
        code: ptr::null_mut(),
        ctxt: ptr::null_mut(),
        expr: ptr::null_mut(),
    },
    value: [Value { n: ptr::null_mut() }, Value { n: ptr::null_mut() }],
    flags: 0,
};

static mut watch_list: ListItem = ListItem {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
    number: 0,
    symbol: ptr::null_mut(),
    subs: ptr::null_mut(),
    num_subs: 0,
    sname: ptr::null_mut(),
    fcall_count: 0,
    commands: CommandsItem {
        next: ptr::null_mut(),
        prev: ptr::null_mut(),
        cmd: 0,
        cmd_string: ptr::null_mut(),
        arg: ptr::null_mut(),
    },
    silent: 0,
    cndn: Condition {
        code: ptr::null_mut(),
        ctxt: ptr::null_mut(),
        expr: ptr::null_mut(),
    },
    value: [Value { n: ptr::null_mut() }, Value { n: ptr::null_mut() }],
    flags: 0,
};

struct Stop {
    fcall_count: i64,
    sourceline: c_int,
    source: *mut c_char,
    pc: *mut Instruction,
    repeat_count: c_int,
    print_frame: bool,
    print_ret: bool,
    break_point: c_int,
    watch_point: c_int,
    check_func: Option<unsafe extern "C" fn(*mut *mut Instruction) -> c_int>,
    command: c_int,
}

static mut stop: Stop = Stop {
    fcall_count: 0,
    sourceline: 0,
    source: ptr::null_mut(),
    pc: ptr::null_mut(),
    repeat_count: 0,
    print_frame: false,
    print_ret: false,
    break_point: 0,
    watch_point: 0,
    check_func: None,
    command: 0,
};

static mut need_restart: bool = false;
static mut commands_string: *const c_char = ptr::null();
static mut commands_string_len: c_int = 0;
static mut line_sep: c_char = 0;

struct DbOption {
    name: *const c_char,
    num_val: *mut c_int,
    str_val: *mut *const c_char,
    assign: Option<unsafe extern "C" fn(*const c_char)>,
    help_txt: *const c_char,
}

static mut options_file: *const c_char = b"./.gawkrc\0".as_ptr() as *const c_char;
static mut history_file: *const c_char = b"./.gawk_history\0".as_ptr() as *const c_char;
static mut output_file: *const c_char = b"/dev/stdout\0".as_ptr() as *const c_char;
static mut dgawk_prompt: *const c_char = ptr::null();
static mut list_size: c_int = 15;
static mut do_trace: bool = false;
static mut do_save_history: bool = true;
static mut do_save_options: bool = true;
static mut history_size: c_int = 100;

static mut option_list: [DbOption; 8] = [
    DbOption {
        name: b"history_size\0".as_ptr() as *const c_char,
        num_val: &mut history_size as *mut c_int,
        str_val: ptr::null_mut(),
        assign: Some(set_history_size),
        help_txt: b"set or show the number of lines to keep in history file\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"listsize\0".as_ptr() as *const c_char,
        num_val: &mut list_size as *mut c_int,
        str_val: ptr::null_mut(),
        assign: Some(set_listsize),
        help_txt: b"set or show the list command window size\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"outfile\0".as_ptr() as *const c_char,
        num_val: ptr::null_mut(),
        str_val: &mut output_file as *mut *const c_char,
        assign: Some(set_gawk_output),
        help_txt: b"set or show gawk output file\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"prompt\0".as_ptr() as *const c_char,
        num_val: ptr::null_mut(),
        str_val: &mut dgawk_prompt as *mut *const c_char,
        assign: Some(set_prompt),
        help_txt: b"set or show debugger prompt\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"save_history\0".as_ptr() as *const c_char,
        num_val: &mut do_save_history as *mut c_int,
        str_val: ptr::null_mut(),
        assign: Some(set_save_history),
        help_txt: b"(un)set or show saving of command history (value=on|off)\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"save_options\0".as_ptr() as *const c_char,
        num_val: &mut do_save_options as *mut c_int,
        str_val: ptr::null_mut(),
        assign: Some(set_save_options),
        help_txt: b"(un)set or show saving of options (value=on|off)\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: b"trace\0".as_ptr() as *const c_char,
        num_val: &mut do_trace as *mut c_int,
        str_val: ptr::null_mut(),
        assign: Some(set_trace),
        help_txt: b"(un)set or show instruction tracing (value=on|off)\0".as_ptr() as *const c_char,
    },
    DbOption {
        name: ptr::null(),
        num_val: ptr::null_mut(),
        str_val: ptr::null_mut(),
        assign: None,
        help_txt: ptr::null(),
    },
];

static mut pager_quit_tag: libc::jmp_buf = [0; 16];
static mut pager_quit_tag_valid: c_int = 0;
static mut screen_width: c_int = i32::MAX;
static mut screen_height: c_int = i32::MAX;
static mut pager_lines_printed: c_int = 0;

struct PfData {
    print_func: Option<unsafe extern "C" fn(*mut libc::FILE, *const c_char, ...) -> c_int>,
    defn: bool,
    fp: *mut libc::FILE,
}

static mut pf_data: PfData = PfData {
    print_func: None,
    defn: false,
    fp: ptr::null_mut(),
};

static mut read_a_line: Option<unsafe extern "C" fn(*const c_char) -> *mut c_char> = None;

struct CommandSource {
    fd: c_int,
    is_tty: bool,
    read_func: Option<unsafe extern "C" fn(*const c_char) -> *mut c_char>,
    close_func: Option<unsafe extern "C" fn(c_int) -> c_int>,
    eof_status: c_int,
    cmd: c_int,
    str: *mut c_char,
    next: *mut CommandSource,
}

static mut cmd_src: *mut CommandSource = ptr::null_mut();

unsafe extern "C" fn g_readline(prompt: *const c_char) -> *mut c_char {
    let mut line = Vec::with_capacity(100);
    let mut buf = [0; 2];
    let mut n;

    if input_from_tty && !prompt.is_null() && *prompt != 0 {
        libc::fprintf(out_fp, b"%s\0".as_ptr() as *const c_char, prompt);
    }

    loop {
        n = libc::read(input_fd, buf.as_mut_ptr() as *mut c_void, 1);
        if n <= 0 {
            break;
        }
        if buf[0] == b'\n' {
            if !line.is_empty() && line[line.len() - 1] == b'\r' {
                line.pop();
            }
            break;
        }
        line.push(buf[0]);
    }

    if n == -1 || (n == 0 && line.is_empty()) {
        return ptr::null_mut();
    }

    line.push(0);
    CString::new(line).unwrap().into_raw()
}

unsafe extern "C" fn d_error(mesg: *const c_char, ...) {
    let mut args = libc::va_list::new();
    libc::vfprintf(out_fp, b"error: %s\n\0".as_ptr() as *const c_char, args);
}

unsafe extern "C" fn find_lines(s: *mut crate::parser::SrcFile) -> c_int {
    let mut buf = Vec::with_capacity((*s).bufsize as usize);
    let mut pos = Vec::with_capacity(((*s).srclines + 2) as usize);
    let mut ofs = 0;
    let mut numlines = 0;
    let mut maxlen = 0;
    let mut lastchar = 0;

    pos.push(0);

    loop {
        let n = libc::read((*s).fd, buf.as_mut_ptr() as *mut c_void, (*s).bufsize as usize);
        if n <= 0 {
            break;
        }

        let end = buf.as_ptr().add(n as usize);
        let mut p = buf.as_ptr();

        while p < end {
            if *p == b'\n' {
                numlines += 1;
                if numlines > (*s).srclines as usize {
                    pos.resize(pos.capacity() * 2, 0);
                }
                pos.push(ofs + (p as usize - buf.as_ptr() as usize) + 1);
                let len = pos[pos.len() - 1] - pos[pos.len() - 2];
                if len > maxlen {
                    maxlen = len;
                }
            }
            p = p.add(1);
        }

        ofs += n as usize;
    }

    if n == -1 {
        d_error(
            b"cannot read source file `%s': %s\0".as_ptr() as *const c_char,
            (*s).src,
            libc::strerror(*libc::__errno_location()),
        );
        return -1;
    }

    if ofs <= 0 {
        libc::fprintf(
            out_fp,
            b"source file `%s' is empty.\n\0".as_ptr() as *const c_char,
            (*s).src,
        );
        return -1;
    }

    if lastchar != b'\n' {
        numlines += 1;
        pos.push(ofs + 1);
        let len = pos[pos.len() - 1] - pos[pos.len() - 2];
        if len > maxlen {
            maxlen = len;
        }
    }

    (*s).maxlen = maxlen;
    (*s).srclines = numlines as c_int;
    0
}

// ... (继续实现其他函数)

unsafe extern "C" fn set_gawk_output(file: *const c_char) {
    let mut fd = -1;
    let mut fp = ptr::null_mut::<libc::FILE>();

    if output_fp != libc::stdout {
        if output_fp != libc::stderr {
            libc::fclose(output_fp);
            libc::free(output_file as *mut c_void);
        }
        output_fp = libc::stdout;
        output_file = b"/dev/stdout\0".as_ptr() as *const c_char;
    }

    if file.is_null() || *file == 0 {
        return;
    }

    fd = libc::open(file, libc::O_WRONLY);
    if fd == -1 {
        if libc::strncmp(file, b"/dev/\0".as_ptr() as *const c_char, 5) == 0 {
            let cp = file.add(5);
            if libc