use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};
use std::ptr;

#[repr(C)]
pub struct re_dfa_t;

#[repr(C)]
pub struct dfa;

static mut mb_cur_max: c_int = 0;

#[repr(C)]
pub struct _IO_FILE {
    _flags: c_int,
    _IO_read_ptr: *mut c_char,
    _IO_read_end: *mut c_char,
    _IO_read_base: *mut c_char,
    _IO_write_base: *mut c_char,
    _IO_write_ptr: *mut c_char,
    _IO_write_end: *mut c_char,
    _IO_buf_base: *mut c_char,
    _IO_buf_end: *mut c_char,
    _IO_save_base: *mut c_char,
    _IO_backup_base: *mut c_char,
    _IO_save_end: *mut c_char,
    _markers: *mut _IO_marker,
    _chain: *mut _IO_FILE,
    _fileno: c_int,
    _flags2: c_int,
    _old_offset: libc::off_t,
    _cur_column: c_ushort,
    _vtable_offset: c_schar,
    _shortbuf: [c_char; 1],
    _lock: *mut libc::c_void,
    _offset: libc::off64_t,
    __pad1: *mut libc::c_void,
    __pad2: *mut libc::c_void,
    __pad3: *mut libc::c_void,
    __pad4: *mut libc::c_void,
    __pad5: size_t,
    _mode: c_int,
    _unused2: [c_char; 20],
}

pub type size_t = c_ulong;

#[repr(C)]
pub struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: c_int,
}

pub type FILE = _IO_FILE;

pub type countT = c_ulong;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = c_ulong;

#[repr(C)]
pub struct re_pattern_buffer {
    buffer: *mut re_dfa_t,
    allocated: __re_long_size_t,
    used: __re_long_size_t,
    syntax: reg_syntax_t,
    fastmap: *mut c_char,
    translate: *mut c_uchar,
    re_nsub: size_t,
    flags: u8,
    _padding: [u8; 7],
}

pub type regex_t = re_pattern_buffer;

#[repr(C)]
pub struct vector {
    v: *mut sed_cmd,
    v_allocated: size_t,
    v_length: size_t,
}

#[repr(C)]
pub struct sed_cmd {
    a1: *mut addr,
    a2: *mut addr,
    range_state: addr_state,
    addr_bang: c_char,
    cmd: c_char,
    x: C2RustUnnamed,
}

#[repr(C)]
pub union C2RustUnnamed {
    cmd_txt: text_buf,
    int_arg: c_int,
    jump_index: countT,
    readcmd: readcmd,
    cmd_subst: *mut subst,
    outf: *mut output,
    inf: *mut output,
    translate: *mut c_uchar,
    translatemb: *mut *mut c_char,
    label_name: *mut c_char,
}

#[repr(C)]
pub struct output {
    name: *mut c_char,
    missing_newline: bool,
    fp: *mut FILE,
    link: *mut output,
}

#[repr(C)]
pub struct subst {
    regx: *mut regex,
    replacement: *mut replacement,
    numb: countT,
    outf: *mut output,
    flags: u8,
    _padding: [u8; 7],
}

#[repr(C)]
pub struct replacement {
    prefix: *mut c_char,
    prefix_length: size_t,
    subst_id: c_int,
    repl_type: replacement_types,
    next: *mut replacement,
}

#[repr(u32)]
pub enum replacement_types {
    REPL_ASIS = 0,
    REPL_UPPERCASE = 1,
    REPL_LOWERCASE = 2,
    REPL_UPPERCASE_FIRST = 4,
    REPL_LOWERCASE_FIRST = 8,
    REPL_UPPERCASE_UPPERCASE = 5,
    REPL_UPPERCASE_LOWERCASE = 6,
    REPL_LOWERCASE_UPPERCASE = 9,
    REPL_LOWERCASE_LOWERCASE = 10,
    REPL_MODIFIERS = 12,
}

#[repr(C)]
pub struct regex {
    pattern: regex_t,
    flags: c_int,
    sz: size_t,
    dfa: *mut dfa,
    begline: bool,
    endline: bool,
    re: [c_char; 1],
}

#[repr(C)]
pub struct readcmd {
    fname: *mut c_char,
    append: bool,
}

#[repr(C)]
pub struct text_buf {
    text: *mut c_char,
    text_length: size_t,
}

#[repr(u32)]
pub enum addr_state {
    RANGE_INACTIVE = 0,
    RANGE_ACTIVE = 1,
    RANGE_CLOSED = 2,
}

#[repr(C)]
pub struct addr {
    addr_type: addr_types,
    addr_number: countT,
    addr_step: countT,
    addr_regex: *mut regex,
}

#[repr(u32)]
pub enum addr_types {
    ADDR_IS_NULL = 0,
    ADDR_IS_REGEX = 1,
    ADDR_IS_NUM = 2,
    ADDR_IS_NUM_MOD = 3,
    ADDR_IS_STEP = 4,
    ADDR_IS_STEP_MOD = 5,
    ADDR_IS_LAST = 6,
}

const _ISprint: c_uint = 16384;
const _ISalnum: c_uint = 8;
const _ISpunct: c_uint = 4;
const _IScntrl: c_uint = 2;
const _ISblank: c_uint = 1;
const _ISgraph: c_uint = 32768;
const _ISspace: c_uint = 8192;
const _ISxdigit: c_uint = 4096;
const _ISdigit: c_uint = 2048;
const _ISalpha: c_uint = 1024;
const _ISlower: c_uint = 512;
const _ISupper: c_uint = 256;

static mut block_level: c_int = 0;

fn debug_print_char(c: c_char) {
    unsafe {
        if 1 != 0 && (*__ctype_b_loc()).offset(c as isize) as c_int & _ISprint as c_int != 0
            && c as c_int != '\\' as i32
        {
            putchar_unlocked(c as c_int);
            return;
        }
        putchar_unlocked('\\' as i32);
        match c as c_int {
            7 => putchar_unlocked('a' as i32),
            12 => putchar_unlocked('f' as i32),
            13 => putchar_unlocked('r' as i32),
            9 => putchar_unlocked('t' as i32),
            11 => putchar_unlocked('v' as i32),
            10 => putchar_unlocked('n' as i32),
            92 => putchar_unlocked('\\' as i32),
            _ => printf(b"o%03o\0".as_ptr() as *const c_char, c as c_uint),
        }
    }
}

fn debug_print_regex_pattern(pat: *const c_char, len: size_t) {
    unsafe {
        let mut p = pat;
        let mut remaining = len;
        while remaining != 0 {
            remaining -= 1;
            if *p as c_int == '/' as i32 {
                fputs_unlocked(b"\\/\0".as_ptr() as *const c_char, stdout);
            } else {
                debug_print_char(*p);
            }
            p = p.offset(1);
        }
    }
}

fn debug_print_regex_flags(r: *const regex, addr: bool) {
    unsafe {
        if r.is_null() {
            return;
        }
        if (*r).flags & (1 << 1) != 0 {
            putchar_unlocked(if addr { 'I' as i32 } else { 'i' as i32 });
        }
        if (*r).flags & (1 << 2) != 0 {
            putchar_unlocked(if addr { 'M' as i32 } else { 'm' as i32 });
        }
    }
}

fn debug_print_regex(r: *const regex) {
    unsafe {
        if r.is_null() {
            fputs_unlocked(b"//\0".as_ptr() as *const c_char, stdout);
            return;
        }
        putchar_unlocked('/' as i32);
        debug_print_regex_pattern(((*r).re).as_ptr(), (*r).sz);
        putchar_unlocked('/' as i32);
    }
}

fn debug_print_addr(a: *const addr) {
    unsafe {
        if a.is_null() {
            return;
        }
        match (*a).addr_type {
            addr_types::ADDR_IS_NULL => {
                fputs_unlocked(b"[ADDR-NULL]\0".as_ptr() as *const c_char, stdout);
            }
            addr_types::ADDR_IS_REGEX => {
                debug_print_regex((*a).addr_regex);
                debug_print_regex_flags((*a).addr_regex, true);
            }
            addr_types::ADDR_IS_NUM => {
                printf(b"%lu\0".as_ptr() as *const c_char, (*a).addr_number);
            }
            addr_types::ADDR_IS_NUM_MOD => {
                printf(
                    b"%lu~%lu\0".as_ptr() as *const c_char,
                    (*a).addr_number,
                    (*a).addr_step,
                );
            }
            addr_types::ADDR_IS_STEP => {
                printf(b"+%lu\0".as_ptr() as *const c_char, (*a).addr_step);
            }
            addr_types::ADDR_IS_STEP_MOD => {
                printf(b"~%lu\0".as_ptr() as *const c_char, (*a).addr_step);
            }
            addr_types::ADDR_IS_LAST => {
                putchar_unlocked('$' as i32);
            }
            _ => {}
        }
    }
}

fn debug_print_subst_replacement(r: *const replacement) {
    unsafe {
        let mut last_repl_type = replacement_types::REPL_ASIS;
        if r.is_null() {
            return;
        }
        let mut p = r;
        while !p.is_null() {
            if (*p).repl_type != last_repl_type {
                putchar_unlocked('\\' as i32);
                match (*p).repl_type {
                    replacement_types::REPL_ASIS => putchar_unlocked('E' as i32),
                    replacement_types::REPL_UPPERCASE => putchar_unlocked('U' as i32),
                    replacement_types::REPL_LOWERCASE => putchar_unlocked('L' as i32),
                    _ if (*p).repl_type as u32 & replacement_types::REPL_MODIFIERS as u32
                        == replacement_types::REPL_UPPERCASE_FIRST as u32 =>
                    {
                        putchar_unlocked('u' as i32)
                    }
                    _ if (*p).repl_type as u32 & replacement_types::REPL_MODIFIERS as u32
                        == replacement_types::REPL_LOWERCASE_FIRST as u32 =>
                    {
                        putchar_unlocked('l' as i32)
                    }
                    _ => {}
                }
                last_repl_type = (*p).repl_type;
            }
            if (*p).prefix_length != 0 {
                fwrite_unlocked(
                    (*p).prefix as *const libc::c_void,
                    1,
                    (*p).prefix_length,
                    stdout,
                );
            }
            if (*p).subst_id != -1 {
                if (*p).subst_id == 0 {
                    putchar_unlocked('&' as i32);
                } else {
                    printf(b"\\%d\0".as_ptr() as *const c_char, (*p).subst_id);
                }
            }
            p = (*p).next;
        }
    }
}

fn debug_print_output_file(o: *const output) {
    unsafe {
        if o.is_null() {
            return;
        }
        fputs_unlocked((*o).name, stdout);
    }
}

fn debug_print_subst(s: *const subst) {
    unsafe {
        if s.is_null() {
            return;
        }
        debug_print_regex((*s).regx);
        debug_print_subst_replacement((*s).replacement);
        putchar_unlocked('/' as i32);
        debug_print_regex_flags((*s).regx, false);
        if (*s).flags & 1 != 0 {
            putchar_unlocked('g' as i32);
        }
        if (*s).flags & (1 << 3) != 0 {
            putchar_unlocked('e' as i32);
        }
        if (*s).flags & (3 << 1) != 0 {
            putchar_unlocked('p' as i32);
        }
        if (*s).numb != 0 {
            printf(b"%lu\0".as_ptr() as *const c_char, (*s).numb);
        }
        if !(*s).outf.is_null() {
            putchar_unlocked('w' as i32);
            debug_print_output_file((*s).outf);
        }
    }
}

fn debug_print_translation(sc: *const sed_cmd) {
    unsafe {
        if mb_cur_max > 1 {
            putchar_unlocked('/' as i32);
            let mut i = 0;
            while !(*(*sc).x.translatemb.offset((2 * i) as isize)).is_null() {
                fputs_unlocked(
                    *(*sc).x.translatemb.offset((2 * i) as isize),
                    stdout,
                );
                i += 1;
            }
            putchar_unlocked('/' as i32);
            i = 0;
            while !(*(*sc).x.translatemb.offset((2 * i) as isize)).is_null() {
                fputs_unlocked(
                    *(*sc).x.translatemb.offset((2 * i + 1) as isize),
                    stdout,
                );
                i += 1;
            }
            putchar_unlocked('/' as i32);
        } else {
            putchar_unlocked('/' as i32);
            for i in 0..256 {
                if *(*sc).x.translate.offset(i as isize) as c_int != i as c_uchar as c_int {
                    putchar_unlocked(i as c_uchar as c_int);
                }
            }
            putchar_unlocked('/' as i32);
            for i in 0..256 {
                if *(*sc).x.translate.offset(i as isize) as c_int != i as c_uchar as c_int {
                    putchar_unlocked(*(*sc).x.translate.offset(i as isize) as c_int);
                }
            }
            putchar_unlocked('/' as i32);
        }
    }
}

fn debug_print_function(program: *const vector, sc: *const sed_cmd) {
    unsafe {
        if sc.is_null() {
            return;
        }
        putchar_unlocked((*sc).cmd as c_int);
        match (*sc).cmd as c_int {
            b':' as i32 => {
                printf(b"%s\0".as_ptr() as *const c_char, (*sc).x.label_name);
            }
            b'a' as i32 | b'c' as i32 | b'i' as i32 => {
                fputs_unlocked(b"\\\0".as_ptr() as *const c_char, stdout);
                if (*sc).x.cmd_txt.text_length != 0 {
                    fwrite_unlocked(
                        (*sc).x.cmd_txt.text as *const libc::c_void,
                        1,
                        (*sc).x.cmd_txt.text_length,
                        stdout,
                    );
                }
            }
            b'b' as i32 | b't' as i32 | b'T' as i32 => {
                if (*sc).x.jump_index < (*program).v_length {
                    let label_name = (*(*program).v.offset((*sc).x.jump_index as isize))
                        .x
                        .label_name;
                    if !label_name.is_null() {
                        printf(b" %s\0".as_ptr() as *const c_char, label_name);
                    }
                }
            }
            b'e' as i32 => {
                putchar_unlocked(' ' as i32);
                fwrite_unlocked(
                    (*sc).x.cmd_txt.text as *const libc::c_void,
                    1,
                    (*sc).x.cmd_txt.text_length,
                    stdout,
                );
            }
            b'L' as i32 | b'l' as i32 | b'q' as i32 | b'Q' as i32 => {
                if (*sc).x.int_arg != -1 {
                    printf(b" %d\0".as_ptr() as *const c_char, (*sc).x.int_arg);
                }
            }
            b'r' as i32 => {
                putchar_unlocked(' ' as i32);
                fputs_unlocked((*sc).x.readcmd.fname, stdout);
            }
            b'R' as i32 => {
                putchar_unlocked(' ' as i32);
                fputs_unlocked((*(*sc).x.inf).name, stdout);
            }
            b's' as i32 => {
                debug_print_subst((*sc).x.cmd_subst);
            }
            b'w' as i32 | b'W' as i32 => {
                debug_print_output_file((*sc).x.outf);
            }
            b'y' as i32 => {
                debug_print_translation(sc);
            }
            _ =>