use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    fn __btowc_alias(__c: i32) -> wint_t;
    fn __wctob_alias(__c: wint_t) -> i32;
    fn wcrtomb(__s: *mut i8, __wc: wchar_t, __ps: *mut mbstate_t) -> size_t;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn towlower(__wc: wint_t) -> wint_t;
    fn towupper(__wc: wint_t) -> wint_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn abort() -> !;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn __uflow(_: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn setvbuf(__stream: *mut FILE, __buf: *mut i8, __modes: i32, __n: size_t) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    static mut lcmd_out_line_len: countT;
    static mut separate_files: bool;
    static mut follow_symlinks: bool;
    static mut write_mode: *const i8;
    static mut posixicity: posixicity_types;
    static mut read_mode: *const i8;
    static mut is_utf8: bool;
    fn panic(str: *const i8, _: ...);
    fn ck_fopen(name: *const i8, mode: *const i8, fail: i32) -> *mut FILE;
    fn ck_fwrite(
        ptr: *const libc::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    );
    fn ck_fread(
        ptr: *mut libc::c_void,
        size: size_t,
        nmemb: size_t,
        stream: *mut FILE,
    ) -> size_t;
    fn ck_fflush(stream: *mut FILE);
    fn ck_fclose(stream: *mut FILE);
    fn follow_symlink(path: *const i8) -> *const i8;
    fn ck_getdelim(
        text: *mut *mut i8,
        buflen: *mut size_t,
        buffer_delimiter_0: i8,
        stream: *mut FILE,
    ) -> size_t;
    fn ck_mkstemp(
        p_filename: *mut *mut i8,
        tmpdir: *const i8,
        base: *const i8,
        mode: *const i8,
    ) -> *mut FILE;
    fn ck_rename(from: *const i8, to: *const i8);
    fn cancel_cleanup();
    fn rewind_read_files();
    fn match_regex(
        regex: *mut regex,
        buf: *mut i8,
        buflen: size_t,
        buf_start_offset: size_t,
        regarray: *mut re_registers,
        regsize: i32,
    ) -> i32;
    fn debug_print_command(program: *const vector, sc: *const sed_cmd);
    fn debug_print_char(c: i8);
    static mut in_place_extension: *mut i8;
    static mut mb_cur_max: i32;
    static mut debug: bool;
    static mut no_default_output: bool;
    static mut buffer_delimiter: i8;
    static mut unbuffered: bool;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> i32;
    fn popen(__command: *const i8, __modes: *const i8) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn isatty(__fd: i32) -> i32;
    fn fchown(__fd: i32, __owner: __uid_t, __group: __gid_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn copy_acl(_: *const i8, _: i32, _: *const i8, _: i32, _: mode_t) -> i32;
    static mut program_name: *const i8;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type wint_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type countT = u64;
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = u64;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut i8,
    pub translate: *mut u8,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum exit_codes {
    EXIT_PANIC = 4,
    EXIT_BAD_INPUT = 2,
    EXIT_BAD_USAGE = 1,
}
impl exit_codes {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            exit_codes::EXIT_PANIC => 4,
            exit_codes::EXIT_BAD_INPUT => 2,
            exit_codes::EXIT_BAD_USAGE => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> exit_codes {
        match value {
            4 => exit_codes::EXIT_PANIC,
            2 => exit_codes::EXIT_BAD_INPUT,
            1 => exit_codes::EXIT_BAD_USAGE,
            _ => panic!("Invalid value for exit_codes: {}", value),
        }
    }
}
impl AddAssign<u32> for exit_codes {
    fn add_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for exit_codes {
    fn sub_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for exit_codes {
    fn mul_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for exit_codes {
    fn div_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for exit_codes {
    fn rem_assign(&mut self, rhs: u32) {
        *self = exit_codes::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for exit_codes {
    type Output = exit_codes;
    fn add(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for exit_codes {
    type Output = exit_codes;
    fn sub(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for exit_codes {
    type Output = exit_codes;
    fn mul(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for exit_codes {
    type Output = exit_codes;
    fn div(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for exit_codes {
    type Output = exit_codes;
    fn rem(self, rhs: u32) -> exit_codes {
        exit_codes::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: *mut sed_cmd,
    pub v_allocated: size_t,
    pub v_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_cmd {
    pub a1: *mut addr,
    pub a2: *mut addr,
    pub range_state: addr_state,
    pub addr_bang: i8,
    pub cmd: i8,
    pub x: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub cmd_txt: text_buf,
    pub int_arg: i32,
    pub jump_index: countT,
    pub readcmd: readcmd,
    pub cmd_subst: *mut subst,
    pub outf: *mut output,
    pub inf: *mut output,
    pub translate: *mut u8,
    pub translatemb: *mut *mut i8,
    pub label_name: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub name: *mut i8,
    pub missing_newline: bool,
    pub fp: *mut FILE,
    pub link: *mut output,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct subst {
    pub regx: *mut regex,
    pub replacement: *mut replacement,
    pub numb: countT,
    pub outf: *mut output,
    #[bitfield(name = "global", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "eval", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "max_id", ty = "libc::c_uint", bits = "4..=7")]
    pub global_print_eval_max_id: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replacement {
    pub prefix: *mut i8,
    pub prefix_length: size_t,
    pub subst_id: i32,
    pub repl_type: replacement_types,
    pub next: *mut replacement,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum replacement_types {
    REPL_ASIS = 0,
    REPL_UPPERCASE,
    REPL_LOWERCASE,
    REPL_UPPERCASE_FIRST,
    REPL_LOWERCASE_FIRST,
    REPL_MODIFIERS,
    REPL_UPPERCASE_UPPERCASE,
    REPL_UPPERCASE_LOWERCASE,
    REPL_LOWERCASE_UPPERCASE,
    REPL_LOWERCASE_LOWERCASE,
}
impl replacement_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            replacement_types::REPL_ASIS => 0,
            replacement_types::REPL_UPPERCASE => 1,
            replacement_types::REPL_LOWERCASE => 2,
            replacement_types::REPL_UPPERCASE_FIRST => 4,
            replacement_types::REPL_LOWERCASE_FIRST => 8,
            replacement_types::REPL_MODIFIERS => 12,
            replacement_types::REPL_UPPERCASE_UPPERCASE => 5,
            replacement_types::REPL_UPPERCASE_LOWERCASE => 6,
            replacement_types::REPL_LOWERCASE_UPPERCASE => 9,
            replacement_types::REPL_LOWERCASE_LOWERCASE => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> replacement_types {
        match value {
            0 => replacement_types::REPL_ASIS,
            1 => replacement_types::REPL_UPPERCASE,
            2 => replacement_types::REPL_LOWERCASE,
            4 => replacement_types::REPL_UPPERCASE_FIRST,
            8 => replacement_types::REPL_LOWERCASE_FIRST,
            12 => replacement_types::REPL_MODIFIERS,
            5 => replacement_types::REPL_UPPERCASE_UPPERCASE,
            6 => replacement_types::REPL_UPPERCASE_LOWERCASE,
            9 => replacement_types::REPL_LOWERCASE_UPPERCASE,
            10 => replacement_types::REPL_LOWERCASE_LOWERCASE,
            _ => panic!("Invalid value for replacement_types: {}", value),
        }
    }
}
impl AddAssign<u32> for replacement_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = replacement_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for replacement_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = replacement_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for replacement_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = replacement_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for replacement_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = replacement_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for replacement_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = replacement_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for replacement_types {
    type Output = replacement_types;
    fn add(self, rhs: u32) -> replacement_types {
        replacement_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for replacement_types {
    type Output = replacement_types;
    fn sub(self, rhs: u32) -> replacement_types {
        replacement_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for replacement_types {
    type Output = replacement_types;
    fn mul(self, rhs: u32) -> replacement_types {
        replacement_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for replacement_types {
    type Output = replacement_types;
    fn div(self, rhs: u32) -> replacement_types {
        replacement_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for replacement_types {
    type Output = replacement_types;
    fn rem(self, rhs: u32) -> replacement_types {
        replacement_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: i32,
    pub sz: size_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [i8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readcmd {
    pub fname: *mut i8,
    pub append: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_buf {
    pub text: *mut i8,
    pub text_length: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum addr_state {
    RANGE_INACTIVE,
    RANGE_ACTIVE,
    RANGE_CLOSED,
}
impl addr_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            addr_state::RANGE_INACTIVE => 0,
            addr_state::RANGE_ACTIVE => 1,
            addr_state::RANGE_CLOSED => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> addr_state {
        match value {
            0 => addr_state::RANGE_INACTIVE,
            1 => addr_state::RANGE_ACTIVE,
            2 => addr_state::RANGE_CLOSED,
            _ => panic!("Invalid value for addr_state: {}", value),
        }
    }
}
impl AddAssign<u32> for addr_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = addr_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for addr_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = addr_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for addr_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = addr_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for addr_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = addr_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for addr_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = addr_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for addr_state {
    type Output = addr_state;
    fn add(self, rhs: u32) -> addr_state {
        addr_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for addr_state {
    type Output = addr_state;
    fn sub(self, rhs: u32) -> addr_state {
        addr_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for addr_state {
    type Output = addr_state;
    fn mul(self, rhs: u32) -> addr_state {
        addr_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for addr_state {
    type Output = addr_state;
    fn div(self, rhs: u32) -> addr_state {
        addr_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for addr_state {
    type Output = addr_state;
    fn rem(self, rhs: u32) -> addr_state {
        addr_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
    pub addr_type: addr_types,
    pub addr_number: countT,
    pub addr_step: countT,
    pub addr_regex: *mut regex,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum addr_types {
    ADDR_IS_NULL,
    ADDR_IS_REGEX,
    ADDR_IS_NUM,
    ADDR_IS_NUM_MOD,
    ADDR_IS_STEP,
    ADDR_IS_STEP_MOD,
    ADDR_IS_LAST,
}
impl addr_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            addr_types::ADDR_IS_NULL => 0,
            addr_types::ADDR_IS_REGEX => 1,
            addr_types::ADDR_IS_NUM => 2,
            addr_types::ADDR_IS_NUM_MOD => 3,
            addr_types::ADDR_IS_STEP => 4,
            addr_types::ADDR_IS_STEP_MOD => 5,
            addr_types::ADDR_IS_LAST => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> addr_types {
        match value {
            0 => addr_types::ADDR_IS_NULL,
            1 => addr_types::ADDR_IS_REGEX,
            2 => addr_types::ADDR_IS_NUM,
            3 => addr_types::ADDR_IS_NUM_MOD,
            4 => addr_types::ADDR_IS_STEP,
            5 => addr_types::ADDR_IS_STEP_MOD,
            6 => addr_types::ADDR_IS_LAST,
            _ => panic!("Invalid value for addr_types: {}", value),
        }
    }
}
impl AddAssign<u32> for addr_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = addr_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for addr_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = addr_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for addr_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = addr_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for addr_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = addr_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for addr_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = addr_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for addr_types {
    type Output = addr_types;
    fn add(self, rhs: u32) -> addr_types {
        addr_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for addr_types {
    type Output = addr_types;
    fn sub(self, rhs: u32) -> addr_types {
        addr_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for addr_types {
    type Output = addr_types;
    fn mul(self, rhs: u32) -> addr_types {
        addr_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for addr_types {
    type Output = addr_types;
    fn div(self, rhs: u32) -> addr_types {
        addr_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for addr_types {
    type Output = addr_types;
    fn rem(self, rhs: u32) -> addr_types {
        addr_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum posixicity_types {
    POSIXLY_EXTENDED,
    POSIXLY_CORRECT,
    POSIXLY_BASIC,
}
impl posixicity_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            posixicity_types::POSIXLY_EXTENDED => 0,
            posixicity_types::POSIXLY_CORRECT => 1,
            posixicity_types::POSIXLY_BASIC => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> posixicity_types {
        match value {
            0 => posixicity_types::POSIXLY_EXTENDED,
            1 => posixicity_types::POSIXLY_CORRECT,
            2 => posixicity_types::POSIXLY_BASIC,
            _ => panic!("Invalid value for posixicity_types: {}", value),
        }
    }
}
impl AddAssign<u32> for posixicity_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for posixicity_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for posixicity_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for posixicity_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for posixicity_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = posixicity_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for posixicity_types {
    type Output = posixicity_types;
    fn add(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for posixicity_types {
    type Output = posixicity_types;
    fn sub(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for posixicity_types {
    type Output = posixicity_types;
    fn mul(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for posixicity_types {
    type Output = posixicity_types;
    fn div(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for posixicity_types {
    type Output = posixicity_types;
    fn rem(self, rhs: u32) -> posixicity_types {
        posixicity_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input {
    pub file_list: *mut *mut i8,
    pub bad_count: countT,
    pub line_number: countT,
    pub reset_at_next_file: bool,
    pub read_fn: Option<unsafe extern "C" fn(*mut input) -> bool>,
    pub out_file_name: *mut i8,
    pub in_file_name: *const i8,
    pub st: stat,
    pub fp: *mut FILE,
    pub no_buffering: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub text: *mut i8,
    pub active: *mut i8,
    pub length: size_t,
    pub alloc: size_t,
    pub chomped: bool,
    pub mbstate: mbstate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct append_queue {
    pub fname: *const i8,
    pub text: *mut i8,
    pub textlen: size_t,
    pub next: *mut append_queue,
    pub rpl_free: bool,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    _ISprint = 16384,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::_ISprint => 16384,
            C2RustUnnamed_1::_ISalnum => 8,
            C2RustUnnamed_1::_ISpunct => 4,
            C2RustUnnamed_1::_IScntrl => 2,
            C2RustUnnamed_1::_ISblank => 1,
            C2RustUnnamed_1::_ISgraph => 32768,
            C2RustUnnamed_1::_ISspace => 8192,
            C2RustUnnamed_1::_ISxdigit => 4096,
            C2RustUnnamed_1::_ISdigit => 2048,
            C2RustUnnamed_1::_ISalpha => 1024,
            C2RustUnnamed_1::_ISlower => 512,
            C2RustUnnamed_1::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            16384 => C2RustUnnamed_1::_ISprint,
            8 => C2RustUnnamed_1::_ISalnum,
            4 => C2RustUnnamed_1::_ISpunct,
            2 => C2RustUnnamed_1::_IScntrl,
            1 => C2RustUnnamed_1::_ISblank,
            32768 => C2RustUnnamed_1::_ISgraph,
            8192 => C2RustUnnamed_1::_ISspace,
            4096 => C2RustUnnamed_1::_ISxdigit,
            2048 => C2RustUnnamed_1::_ISdigit,
            1024 => C2RustUnnamed_1::_ISalpha,
            512 => C2RustUnnamed_1::_ISlower,
            256 => C2RustUnnamed_1::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn btowc(mut __c: i32) -> wint_t {
    return if 0 != 0 && __c >= '\0' as i32 && __c <= '\u{7f}' as i32 {
        __c as wint_t
    } else {
        __btowc_alias(__c)
    };
}
#[inline]
unsafe extern "C" fn wctob(mut __wc: wint_t) -> i32 {
    return if 0 != 0 && __wc >= '\0' as i32 as u32 && __wc <= '\u{7f}' as i32 as u32 {
        __wc as i32
    } else {
        __wctob_alias(__wc)
    };
}
#[inline]
unsafe extern "C" fn xnrealloc(
    mut p: *mut libc::c_void,
    mut n: size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    return xreallocarray(p, n, s);
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> i32 {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as i32 as i64 != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut u8) as i32
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: i32) -> i32 {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as i32 as i64 != 0 {
        __overflow(stdout, __c as u8 as i32)
    } else {
        let fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh1 = __c as i8;
        *fresh1 as u8 as i32
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x10 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn freecon(mut con: *mut i8) {}
#[inline]
unsafe extern "C" fn getfscreatecon(mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn setfscreatecon(mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn lgetfilecon(mut file: *const i8, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
static mut replaced: bool = 0 as i32 != 0;
static mut output_file: output = output {
    name: 0 as *const i8 as *mut i8,
    missing_newline: false,
    fp: 0 as *const FILE as *mut FILE,
    link: 0 as *const output as *mut output,
};
static mut line: line = line {
    text: 0 as *const i8 as *mut i8,
    active: 0 as *const i8 as *mut i8,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut s_accum: line = line {
    text: 0 as *const i8 as *mut i8,
    active: 0 as *const i8 as *mut i8,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut hold: line = line {
    text: 0 as *const i8 as *mut i8,
    active: 0 as *const i8 as *mut i8,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut buffer: line = line {
    text: 0 as *const i8 as *mut i8,
    active: 0 as *const i8 as *mut i8,
    length: 0,
    alloc: 0,
    chomped: false,
    mbstate: mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    },
};
static mut append_head: *mut append_queue = 0 as *const append_queue
    as *mut append_queue;
static mut append_tail: *mut append_queue = 0 as *const append_queue
    as *mut append_queue;
unsafe extern "C" fn resize_line(mut lb: *mut line, mut len: size_t) {
    let mut inactive: i32 = 0;
    inactive = ((*lb).active).offset_from((*lb).text) as i64 as i32;
    if inactive as u64 > ((*lb).alloc).wrapping_mul(2 as i32 as u64) {
        memmove(
            (*lb).text as *mut libc::c_void,
            (*lb).active as *const libc::c_void,
            (*lb).length,
        );
        (*lb).alloc = ((*lb).alloc as u64)
            .wrapping_add(((*lb).active).offset_from((*lb).text) as i64 as u64) as size_t
            as size_t;
        (*lb).active = (*lb).text;
        inactive = 0 as i32;
        if (*lb).alloc > len {
            return;
        }
    }
    (*lb).alloc = ((*lb).alloc as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
    if (*lb).alloc < len {
        (*lb).alloc = len;
    }
    if (*lb).alloc < 50 as i32 as u64 {
        (*lb).alloc = 50 as i32 as size_t;
    }
    (*lb).text = xnrealloc(
        (*lb).text as *mut libc::c_void,
        (inactive as u64).wrapping_add((*lb).alloc).wrapping_add(1 as i32 as u64),
        ::core::mem::size_of::<i8>() as u64,
    ) as *mut i8;
    (*lb).active = ((*lb).text).offset(inactive as isize);
}
unsafe extern "C" fn str_append(
    mut to: *mut line,
    mut string: *const i8,
    mut length: size_t,
) {
    let mut new_length: size_t = ((*to).length).wrapping_add(length);
    if (*to).alloc < new_length {
        resize_line(to, new_length);
    }
    memcpy(
        ((*to).active).offset((*to).length as isize) as *mut libc::c_void,
        string as *const libc::c_void,
        length,
    );
    (*to).length = new_length;
    if mb_cur_max > 1 as i32 && !is_utf8 {
        while length != 0 {
            let mut n: size_t = if mb_cur_max == 1 as i32 {
                1 as i32 as u64
            } else {
                rpl_mbrtowc(0 as *mut wchar_t, string, length, &mut (*to).mbstate)
            };
            if n == -(1 as i32) as size_t || n == -(2 as i32) as size_t {
                memset(
                    &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<mbstate_t>() as u64,
                );
                n = 1 as i32 as size_t;
            }
            if n == 0 as i32 as u64 {
                break;
            }
            string = string.offset(n as isize);
            length = (length as u64).wrapping_sub(n) as size_t as size_t;
        }
    }
}
unsafe extern "C" fn str_append_modified(
    mut to: *mut line,
    mut string: *const i8,
    mut length: size_t,
    mut type_0: replacement_types,
) {
    let mut from_stat: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    if type_0 as u32 == replacement_types::REPL_ASIS as i32 as u32 {
        str_append(to, string, length);
        return;
    }
    if ((*to).alloc).wrapping_sub((*to).length) < length.wrapping_mul(mb_cur_max as u64)
    {
        resize_line(
            to,
            ((*to).length).wrapping_add(length.wrapping_mul(mb_cur_max as u64)),
        );
    }
    memcpy(
        &mut from_stat as *mut mbstate_t as *mut libc::c_void,
        &mut (*to).mbstate as *mut mbstate_t as *const libc::c_void,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    while length != 0 {
        let mut wc: wchar_t = 0;
        let mut n: size_t = if mb_cur_max == 1 as i32 {
            wc = btowc(*(string as *mut u8) as i32) as wchar_t;
            1 as i32 as u64
        } else {
            rpl_mbrtowc(&mut wc, string, length, &mut from_stat)
        };
        if n == -(1 as i32) as size_t {
            type_0 = ::core::mem::transmute::<
                u32,
                replacement_types,
            >(
                type_0 as u32
                    & !(replacement_types::REPL_LOWERCASE_FIRST as i32
                        | replacement_types::REPL_UPPERCASE_FIRST as i32) as u32,
            );
            if type_0 as u32 == replacement_types::REPL_ASIS as i32 as u32 {
                str_append(to, string, length);
                return;
            }
            str_append(to, string, 1 as i32 as size_t);
            memset(
                &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            n = 1 as i32 as size_t;
            string = string.offset(n as isize);
            length = (length as u64).wrapping_sub(n) as size_t as size_t;
        } else {
            if n == 0 as i32 as u64 || n == -(2 as i32) as size_t {
                str_append(to, string, length);
                return;
            }
            string = string.offset(n as isize);
            length = (length as u64).wrapping_sub(n) as size_t as size_t;
            if type_0 as u32
                & (replacement_types::REPL_UPPERCASE_FIRST as i32
                    | replacement_types::REPL_LOWERCASE_FIRST as i32) as u32 != 0
            {
                if type_0 as u32 & replacement_types::REPL_UPPERCASE_FIRST as i32 as u32
                    != 0
                {
                    wc = towupper(wc as wint_t) as wchar_t;
                } else {
                    wc = towlower(wc as wint_t) as wchar_t;
                }
                type_0 = ::core::mem::transmute::<
                    u32,
                    replacement_types,
                >(
                    type_0 as u32
                        & !(replacement_types::REPL_LOWERCASE_FIRST as i32
                            | replacement_types::REPL_UPPERCASE_FIRST as i32) as u32,
                );
                if type_0 as u32 == replacement_types::REPL_ASIS as i32 as u32 {
                    n = if mb_cur_max == 1 as i32 {
                        *((*to).active).offset((*to).length as isize) = wctob(
                            wc as wint_t,
                        ) as i8;
                        1 as i32 as u64
                    } else {
                        wcrtomb(
                            ((*to).active).offset((*to).length as isize),
                            wc,
                            &mut (*to).mbstate,
                        )
                    };
                    (*to).length = ((*to).length as u64).wrapping_add(n) as size_t
                        as size_t;
                    if n == -(1 as i32) as size_t || n == -(2 as i32) as size_t {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"case conversion produced an invalid character\0"
                                    as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        abort();
                    }
                    str_append(to, string, length);
                    return;
                }
            } else if type_0 as u32 & replacement_types::REPL_UPPERCASE as i32 as u32
                != 0
            {
                wc = towupper(wc as wint_t) as wchar_t;
            } else {
                wc = towlower(wc as wint_t) as wchar_t;
            }
            n = if mb_cur_max == 1 as i32 {
                *((*to).active).offset((*to).length as isize) = wctob(wc as wint_t)
                    as i8;
                1 as i32 as u64
            } else {
                wcrtomb(
                    ((*to).active).offset((*to).length as isize),
                    wc,
                    &mut (*to).mbstate,
                )
            };
            (*to).length = ((*to).length as u64).wrapping_add(n) as size_t as size_t;
            if n == -(1 as i32) as u64 || n == -(2 as i32) as u64 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"case conversion produced an invalid character\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                abort();
            }
        }
    }
}
unsafe extern "C" fn line_init(
    mut buf: *mut line,
    mut state: *mut line,
    mut initial_size: size_t,
) {
    (*buf).text = (if ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64 {
        xzalloc(initial_size.wrapping_add(1 as i32 as u64))
    } else {
        xcalloc(
            initial_size.wrapping_add(1 as i32 as u64),
            ::core::mem::size_of::<i8>() as u64,
        )
    }) as *mut i8;
    (*buf).active = (*buf).text;
    (*buf).alloc = initial_size;
    (*buf).length = 0 as i32 as size_t;
    (*buf).chomped = 1 as i32 != 0;
    if !state.is_null() {
        memcpy(
            &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*state).mbstate as *mut mbstate_t as *const libc::c_void,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    } else {
        memset(
            &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    };
}
unsafe extern "C" fn line_reset(mut buf: *mut line, mut state: *mut line) {
    if (*buf).alloc == 0 as i32 as u64 {
        line_init(buf, state, 50 as i32 as size_t);
    } else {
        (*buf).length = 0 as i32 as size_t;
        if !state.is_null() {
            memcpy(
                &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
                &mut (*state).mbstate as *mut mbstate_t as *const libc::c_void,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
        } else {
            memset(
                &mut (*buf).mbstate as *mut mbstate_t as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
        }
    };
}
unsafe extern "C" fn line_copy(mut from: *mut line, mut to: *mut line, mut state: i32) {
    (*to).alloc = ((*to).alloc as u64)
        .wrapping_add(((*to).active).offset_from((*to).text) as i64 as u64) as size_t
        as size_t;
    if (*to).alloc < (*from).length {
        (*to).alloc = ((*to).alloc as u64).wrapping_mul(2 as i32 as u64) as size_t
            as size_t;
        if (*to).alloc < (*from).length {
            (*to).alloc = (*from).length;
        }
        if (*to).alloc < 50 as i32 as u64 {
            (*to).alloc = 50 as i32 as size_t;
        }
        rpl_free((*to).text as *mut libc::c_void);
        (*to).text = (if ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64 {
            xzalloc(((*to).alloc).wrapping_add(1 as i32 as u64))
        } else {
            xcalloc(
                ((*to).alloc).wrapping_add(1 as i32 as u64),
                ::core::mem::size_of::<i8>() as u64,
            )
        }) as *mut i8;
    }
    (*to).active = (*to).text;
    (*to).length = (*from).length;
    (*to).chomped = (*from).chomped;
    memcpy(
        (*to).active as *mut libc::c_void,
        (*from).active as *const libc::c_void,
        (*from).length,
    );
    if state != 0 {
        memcpy(
            &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*from).mbstate as *mut mbstate_t as *const libc::c_void,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    }
}
unsafe extern "C" fn line_append(
    mut from: *mut line,
    mut to: *mut line,
    mut state: i32,
) {
    str_append(to, &mut buffer_delimiter, 1 as i32 as size_t);
    str_append(to, (*from).active, (*from).length);
    (*to).chomped = (*from).chomped;
    if state != 0 {
        memcpy(
            &mut (*to).mbstate as *mut mbstate_t as *mut libc::c_void,
            &mut (*from).mbstate as *mut mbstate_t as *const libc::c_void,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    }
}
unsafe extern "C" fn line_exchange(mut a: *mut line, mut b: *mut line, mut state: i32) {
    let mut t: line = line {
        text: 0 as *const i8 as *mut i8,
        active: 0 as *const i8 as *mut i8,
        length: 0,
        alloc: 0,
        chomped: false,
        mbstate: mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    if state != 0 {
        memcpy(
            &mut t as *mut line as *mut libc::c_void,
            a as *const libc::c_void,
            ::core::mem::size_of::<line>() as u64,
        );
        memcpy(
            a as *mut libc::c_void,
            b as *const libc::c_void,
            ::core::mem::size_of::<line>() as u64,
        );
        memcpy(
            b as *mut libc::c_void,
            &mut t as *mut line as *const libc::c_void,
            ::core::mem::size_of::<line>() as u64,
        );
    } else {
        memcpy(
            &mut t as *mut line as *mut libc::c_void,
            a as *const libc::c_void,
            36 as u64,
        );
        memcpy(a as *mut libc::c_void, b as *const libc::c_void, 36 as u64);
        memcpy(
            b as *mut libc::c_void,
            &mut t as *mut line as *const libc::c_void,
            36 as u64,
        );
    };
}
unsafe extern "C" fn read_always_fail(mut input: *mut input) -> bool {
    return 0 as i32 != 0;
}
unsafe extern "C" fn read_file_line(mut input: *mut input) -> bool {
    static mut b: *mut i8 = 0 as *const i8 as *mut i8;
    static mut blen: size_t = 0;
    let mut result: i64 = ck_getdelim(&mut b, &mut blen, buffer_delimiter, (*input).fp)
        as i64;
    if result <= 0 as i32 as i64 {
        return 0 as i32 != 0;
    }
    if *b.offset((result - 1 as i32 as i64) as isize) as i32 == buffer_delimiter as i32 {
        result -= 1;
        result;
    } else {
        line.chomped = 0 as i32 != 0;
    }
    str_append(&mut line, b, result as size_t);
    return 1 as i32 != 0;
}
#[inline]
unsafe extern "C" fn output_missing_newline(mut outf: *mut output) {
    if (*outf).missing_newline {
        ck_fwrite(
            &mut buffer_delimiter as *mut i8 as *const libc::c_void,
            1 as i32 as size_t,
            1 as i32 as size_t,
            (*outf).fp,
        );
        (*outf).missing_newline = 0 as i32 != 0;
    }
}
#[inline]
unsafe extern "C" fn flush_output(mut fp: *mut FILE) {
    if unbuffered {
        ck_fflush(fp);
    }
}
unsafe extern "C" fn output_line(
    mut text: *const i8,
    mut length: size_t,
    mut nl: i32,
    mut outf: *mut output,
) {
    if text.is_null() {
        return;
    }
    output_missing_newline(outf);
    if length != 0 {
        ck_fwrite(text as *const libc::c_void, 1 as i32 as size_t, length, (*outf).fp);
    }
    if nl != 0 {
        ck_fwrite(
            &mut buffer_delimiter as *mut i8 as *const libc::c_void,
            1 as i32 as size_t,
            1 as i32 as size_t,
            (*outf).fp,
        );
    } else {
        (*outf).missing_newline = 1 as i32 != 0;
    }
    flush_output((*outf).fp);
}
unsafe extern "C" fn next_append_slot() -> *mut append_queue {
    let mut n: *mut append_queue = (if ::core::mem::size_of::<append_queue>() as u64
        == 1 as i32 as u64
    {
        xzalloc(1 as i32 as size_t)
    } else {
        xcalloc(1 as i32 as size_t, ::core::mem::size_of::<append_queue>() as u64)
    }) as *mut append_queue;
    (*n).fname = 0 as *const i8;
    (*n).text = 0 as *mut i8;
    (*n).textlen = 0 as i32 as size_t;
    (*n).next = 0 as *mut append_queue;
    (*n).rpl_free = 0 as i32 != 0;
    if !append_tail.is_null() {
        (*append_tail).next = n;
    } else {
        append_head = n;
    }
    append_tail = n;
    return append_tail;
}
unsafe extern "C" fn release_append_queue() {
    let mut p: *mut append_queue = 0 as *mut append_queue;
    let mut q: *mut append_queue = 0 as *mut append_queue;
    p = append_head;
    while !p.is_null() {
        if (*p).rpl_free {
            rpl_free((*p).text as *mut libc::c_void);
        }
        q = (*p).next;
        rpl_free(p as *mut libc::c_void);
        p = q;
    }
    append_tail = 0 as *mut append_queue;
    append_head = append_tail;
}
unsafe extern "C" fn print_file(mut infname: *const i8, mut outf: *mut FILE) {
    let mut buf: [i8; 8192] = [0; 8192];
    let mut cnt: size_t = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = ck_fopen(infname, read_mode, 0 as i32);
    if !fp.is_null() {
        loop {
            cnt = ck_fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                1 as i32 as size_t,
                ::core::mem::size_of::<[i8; 8192]>() as u64,
                fp,
            );
            if !(cnt > 0 as i32 as u64) {
                break;
            }
            ck_fwrite(
                buf.as_mut_ptr() as *const libc::c_void,
                1 as i32 as size_t,
                cnt,
                outf,
            );
        }
        ck_fclose(fp);
    }
}
unsafe extern "C" fn dump_append_queue() {
    let mut p: *mut append_queue = 0 as *mut append_queue;
    output_missing_newline(&mut output_file);
    p = append_head;
    while !p.is_null() {
        if !((*p).text).is_null() {
            ck_fwrite(
                (*p).text as *const libc::c_void,
                1 as i32 as size_t,
                (*p).textlen,
                output_file.fp,
            );
        }
        if !((*p).fname).is_null() {
            print_file((*p).fname, output_file.fp);
        }
        p = (*p).next;
    }
    flush_output(output_file.fp);
    release_append_queue();
}
unsafe extern "C" fn get_backup_file_name(mut name: *const i8) -> *mut i8 {
    let mut old_asterisk: *mut i8 = 0 as *mut i8;
    let mut asterisk: *mut i8 = 0 as *mut i8;
    let mut backup: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut name_length: i32 = strlen(name) as i32;
    let mut backup_length: i32 = strlen(in_place_extension) as i32;
    asterisk = in_place_extension.offset(-(1 as i32 as isize));
    old_asterisk = asterisk.offset(1 as i32 as isize);
    loop {
        asterisk = strchr(old_asterisk, '*' as i32);
        if asterisk.is_null() {
            break;
        }
        backup_length += name_length - 1 as i32;
        old_asterisk = asterisk.offset(1 as i32 as isize);
    }
    backup = xmalloc((backup_length + 1 as i32) as size_t) as *mut i8;
    p = backup;
    asterisk = in_place_extension.offset(-(1 as i32 as isize));
    old_asterisk = asterisk.offset(1 as i32 as isize);
    loop {
        asterisk = strchr(old_asterisk, '*' as i32);
        if asterisk.is_null() {
            break;
        }
        memcpy(
            p as *mut libc::c_void,
            old_asterisk as *const libc::c_void,
            asterisk.offset_from(old_asterisk) as i64 as u64,
        );
        p = p.offset(asterisk.offset_from(old_asterisk) as i64 as isize);
        strcpy(p, name);
        p = p.offset(name_length as isize);
        old_asterisk = asterisk.offset(1 as i32 as isize);
    }
    strcpy(p, old_asterisk);
    return backup;
}
unsafe extern "C" fn open_next_file(mut name: *const i8, mut input: *mut input) {
    buffer.length = 0 as i32 as size_t;
    (*input).in_file_name = name;
    if *name.offset(0 as i32 as isize) as i32 == '-' as i32
        && *name.offset(1 as i32 as isize) as i32 == '\0' as i32
        && in_place_extension.is_null()
    {
        clearerr_unlocked(stdin);
        (*input).fp = stdin;
    } else {
        if follow_symlinks {
            (*input).in_file_name = follow_symlink(name);
        }
        (*input).fp = ck_fopen(name, read_mode, 0 as i32);
        if ((*input).fp).is_null() {
            let mut ptr: *const i8 = strerror(*__errno_location());
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"%s: can't read %s: %s\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                program_name,
                name,
                ptr,
            );
            (*input).read_fn = Some(
                read_always_fail as unsafe extern "C" fn(*mut input) -> bool,
            );
            (*input).bad_count = ((*input).bad_count).wrapping_add(1);
            (*input).bad_count;
            return;
        }
    }
    (*input).read_fn = Some(read_file_line as unsafe extern "C" fn(*mut input) -> bool);
    if !in_place_extension.is_null() {
        let mut input_fd: i32 = 0;
        let mut tmpdir: *mut i8 = 0 as *mut i8;
        let mut p: *mut i8 = 0 as *mut i8;
        let mut old_fscreatecon: *mut i8 = 0 as *mut i8;
        let mut reset_fscreatecon: i32 = 0 as i32;
        memset(
            &mut old_fscreatecon as *mut *mut i8 as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<*mut i8>() as u64,
        );
        tmpdir = xstrdup((*input).in_file_name);
        p = strrchr(tmpdir, '/' as i32);
        if !p.is_null() {
            *p = 0 as i32 as i8;
        } else {
            strcpy(tmpdir, b".\0" as *const u8 as *const i8);
        }
        if isatty(fileno((*input).fp)) != 0 {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't edit %s: is a terminal\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*input).in_file_name,
            );
        }
        input_fd = fileno((*input).fp);
        fstat(input_fd, &mut (*input).st);
        if !((*input).st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't edit %s: not a regular file\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*input).in_file_name,
            );
        }
        if 0 as i32 > 0 as i32 {
            let mut con: *mut i8 = 0 as *mut i8;
            if lgetfilecon((*input).in_file_name, &mut con) != -(1 as i32) {
                reset_fscreatecon = (getfscreatecon(&mut old_fscreatecon) >= 0 as i32)
                    as i32;
                if setfscreatecon(con) < 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: warning: failed to set default file creation context to %s: %s\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        program_name,
                        con,
                        strerror(*__errno_location()),
                    );
                }
                freecon(con);
            } else if *__errno_location() != 38 as i32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"%s: warning: failed to get security context of %s: %s\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    program_name,
                    (*input).in_file_name,
                    strerror(*__errno_location()),
                );
            }
        }
        output_file.fp = ck_mkstemp(
            &mut (*input).out_file_name,
            tmpdir,
            b"sed\0" as *const u8 as *const i8,
            write_mode,
        );
        output_file.missing_newline = 0 as i32 != 0;
        rpl_free(tmpdir as *mut libc::c_void);
        if reset_fscreatecon != 0 {
            setfscreatecon(old_fscreatecon);
            freecon(old_fscreatecon);
        }
        if (output_file.fp).is_null() {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"couldn't open temporary file %s: %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                (*input).out_file_name,
                strerror(*__errno_location()),
            );
        }
    } else {
        if !((*input).fp).is_null() && unbuffered as i32 != 0 {
            setvbuf((*input).fp, 0 as *mut i8, 2 as i32, 0 as i32 as size_t);
        }
        output_file.fp = stdout;
    };
}
unsafe extern "C" fn closedown(mut input: *mut input) {
    (*input).read_fn = Some(
        read_always_fail as unsafe extern "C" fn(*mut input) -> bool,
    );
    if ((*input).fp).is_null() {
        return;
    }
    if !in_place_extension.is_null() && !(output_file.fp).is_null() {
        let mut target_name: *const i8 = 0 as *const i8;
        let mut input_fd: i32 = 0;
        let mut output_fd: i32 = 0;
        target_name = (*input).in_file_name;
        input_fd = fileno((*input).fp);
        output_fd = fileno(output_file.fp);
        if fchown(output_fd, (*input).st.st_uid, (*input).st.st_gid) == -(1 as i32) {
            fchown(output_fd, -(1 as i32) as __uid_t, (*input).st.st_gid);
        }
        copy_acl(
            (*input).in_file_name,
            input_fd,
            (*input).out_file_name,
            output_fd,
            (*input).st.st_mode,
        );
        ck_fclose((*input).fp);
        ck_fclose(output_file.fp);
        if strcmp(in_place_extension, b"*\0" as *const u8 as *const i8) != 0 as i32 {
            let mut backup_file_name: *mut i8 = get_backup_file_name(target_name);
            ck_rename(target_name, backup_file_name);
            rpl_free(backup_file_name as *mut libc::c_void);
        }
        ck_rename((*input).out_file_name, target_name);
        cancel_cleanup();
        rpl_free((*input).out_file_name as *mut libc::c_void);
    } else {
        ck_fclose((*input).fp);
    }
    (*input).fp = 0 as *mut FILE;
}
unsafe extern "C" fn reset_addresses(mut vec: *mut vector) {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut n: i32 = 0;
    cur_cmd = (*vec).v;
    n = (*vec).v_length as i32;
    loop {
        let fresh2 = n;
        n = n - 1;
        if !(fresh2 != 0) {
            break;
        }
        if !((*cur_cmd).a1).is_null()
            && (*(*cur_cmd).a1).addr_type as u32 == addr_types::ADDR_IS_NUM as i32 as u32
            && (*(*cur_cmd).a1).addr_number == 0 as i32 as u64
        {
            (*cur_cmd).range_state = addr_state::RANGE_ACTIVE;
        } else {
            (*cur_cmd).range_state = addr_state::RANGE_INACTIVE;
        }
        cur_cmd = cur_cmd.offset(1);
        cur_cmd;
    };
}
unsafe extern "C" fn read_pattern_space(
    mut input: *mut input,
    mut the_program: *mut vector,
    mut append: i32,
) -> bool {
    if !append_head.is_null() {
        dump_append_queue();
    }
    replaced = 0 as i32 != 0;
    if append == 0 {
        line.length = 0 as i32 as size_t;
    }
    line.chomped = 1 as i32 != 0;
    while !(Some(((*input).read_fn).expect("non-null function pointer")))
        .expect("non-null function pointer")(input)
    {
        closedown(input);
        if (*(*input).file_list).is_null() {
            return 0 as i32 != 0;
        }
        if (*input).reset_at_next_file {
            (*input).line_number = 0 as i32 as countT;
            hold.length = 0 as i32 as size_t;
            reset_addresses(the_program);
            rewind_read_files();
            if !in_place_extension.is_null() {
                output_file.missing_newline = 0 as i32 != 0;
            }
            (*input).reset_at_next_file = separate_files;
        }
        let fresh3 = (*input).file_list;
        (*input).file_list = ((*input).file_list).offset(1);
        open_next_file(*fresh3, input);
    }
    (*input).line_number = ((*input).line_number).wrapping_add(1);
    (*input).line_number;
    return 1 as i32 != 0;
}
unsafe extern "C" fn last_file_with_data_p(mut input: *mut input) -> bool {
    loop {
        let mut ch: i32 = 0;
        closedown(input);
        if (*(*input).file_list).is_null() {
            return 1 as i32 != 0;
        }
        let fresh4 = (*input).file_list;
        (*input).file_list = ((*input).file_list).offset(1);
        open_next_file(*fresh4, input);
        if !((*input).fp).is_null() {
            ch = getc_unlocked((*input).fp);
            if ch != -(1 as i32) {
                ungetc(ch, (*input).fp);
                return 0 as i32 != 0;
            }
        }
    };
}
unsafe extern "C" fn test_eof(mut input: *mut input) -> bool {
    let mut ch: i32 = 0;
    if buffer.length != 0 {
        return 0 as i32 != 0;
    }
    if ((*input).fp).is_null() {
        return separate_files as i32 != 0 || last_file_with_data_p(input) as i32 != 0;
    }
    if feof_unlocked((*input).fp) != 0 {
        return separate_files as i32 != 0 || last_file_with_data_p(input) as i32 != 0;
    }
    ch = getc_unlocked((*input).fp);
    if ch == -(1 as i32) {
        return separate_files as i32 != 0 || last_file_with_data_p(input) as i32 != 0;
    }
    ungetc(ch, (*input).fp);
    return 0 as i32 != 0;
}
unsafe extern "C" fn match_an_address_p(
    mut addr: *mut addr,
    mut input: *mut input,
) -> bool {
    match (*addr).addr_type as u32 {
        0 => return 1 as i32 != 0,
        1 => {
            return match_regex(
                (*addr).addr_regex,
                line.active,
                line.length,
                0 as i32 as size_t,
                0 as *mut re_registers,
                0 as i32,
            ) != 0;
        }
        3 => {
            return (*input).line_number >= (*addr).addr_number
                && ((*input).line_number)
                    .wrapping_sub((*addr).addr_number)
                    .wrapping_rem((*addr).addr_step) == 0 as i32 as u64;
        }
        4 | 5 => return (*addr).addr_number <= (*input).line_number,
        6 => return test_eof(input),
        2 => return (*addr).addr_number == (*input).line_number,
        _ => {
            panic(b"INTERNAL ERROR: bad address type\0" as *const u8 as *const i8);
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn match_address_p(
    mut cmd: *mut sed_cmd,
    mut input: *mut input,
) -> bool {
    if ((*cmd).a1).is_null() {
        return 1 as i32 != 0;
    }
    if (*cmd).range_state as u32 != addr_state::RANGE_ACTIVE as i32 as u32 {
        if ((*cmd).a2).is_null() {
            return match_an_address_p((*cmd).a1, input);
        }
        if (*(*cmd).a1).addr_type as u32 == addr_types::ADDR_IS_NUM as i32 as u32 {
            if (*cmd).range_state as u32 == addr_state::RANGE_CLOSED as i32 as u32
                || (*input).line_number < (*(*cmd).a1).addr_number
            {
                return 0 as i32 != 0;
            }
        } else if !match_an_address_p((*cmd).a1, input) {
            return 0 as i32 != 0
        }
        (*cmd).range_state = addr_state::RANGE_ACTIVE;
        match (*(*cmd).a2).addr_type as u32 {
            1 => return 1 as i32 != 0,
            2 => {
                if (*input).line_number >= (*(*cmd).a2).addr_number {
                    (*cmd).range_state = addr_state::RANGE_CLOSED;
                }
                return (*input).line_number <= (*(*cmd).a2).addr_number
                    || match_an_address_p((*cmd).a1, input) as i32 != 0;
            }
            4 => {
                (*(*cmd).a2).addr_number = ((*input).line_number)
                    .wrapping_add((*(*cmd).a2).addr_step);
                return 1 as i32 != 0;
            }
            5 => {
                (*(*cmd).a2).addr_number = ((*input).line_number)
                    .wrapping_add((*(*cmd).a2).addr_step)
                    .wrapping_sub(
                        ((*input).line_number).wrapping_rem((*(*cmd).a2).addr_step),
                    );
                return 1 as i32 != 0;
            }
            _ => {}
        }
    }
    if (*(*cmd).a2).addr_type as u32 == addr_types::ADDR_IS_NUM as i32 as u32 {
        if (*input).line_number >= (*(*cmd).a2).addr_number {
            (*cmd).range_state = addr_state::RANGE_CLOSED;
        }
        return (*input).line_number <= (*(*cmd).a2).addr_number;
    }
    if match_an_address_p((*cmd).a2, input) {
        (*cmd).range_state = addr_state::RANGE_CLOSED;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn do_list(mut line_len: i32) {
    let mut p: *mut u8 = line.active as *mut u8;
    let mut len: countT = line.length;
    let mut width: countT = 0 as i32 as countT;
    let mut obuf: [i8; 180] = [0; 180];
    let mut o: *mut i8 = 0 as *mut i8;
    let mut olen: size_t = 0;
    let mut fp: *mut FILE = output_file.fp;
    output_missing_newline(&mut output_file);
    loop {
        let fresh5 = len;
        len = len.wrapping_sub(1);
        if !(fresh5 != 0) {
            break;
        }
        o = obuf.as_mut_ptr();
        if *p as i32 & !(0x7f as i32) == 0 as i32
            && (1 as i32 != 0
                && *(*__ctype_b_loc()).offset(*p as i32 as isize) as i32
                    & C2RustUnnamed_1::_ISprint as i32 as libc::c_ushort as i32 != 0)
        {
            let fresh6 = o;
            o = o.offset(1);
            *fresh6 = *p as i8;
            if *p as i32 == '\\' as i32 {
                let fresh7 = o;
                o = o.offset(1);
                *fresh7 = '\\' as i32 as i8;
            }
        } else {
            let fresh8 = o;
            o = o.offset(1);
            *fresh8 = '\\' as i32 as i8;
            match *p as i32 {
                7 => {
                    let fresh9 = o;
                    o = o.offset(1);
                    *fresh9 = 'a' as i32 as i8;
                }
                8 => {
                    let fresh10 = o;
                    o = o.offset(1);
                    *fresh10 = 'b' as i32 as i8;
                }
                12 => {
                    let fresh11 = o;
                    o = o.offset(1);
                    *fresh11 = 'f' as i32 as i8;
                }
                10 => {
                    let fresh12 = o;
                    o = o.offset(1);
                    *fresh12 = 'n' as i32 as i8;
                }
                13 => {
                    let fresh13 = o;
                    o = o.offset(1);
                    *fresh13 = 'r' as i32 as i8;
                }
                9 => {
                    let fresh14 = o;
                    o = o.offset(1);
                    *fresh14 = 't' as i32 as i8;
                }
                11 => {
                    let fresh15 = o;
                    o = o.offset(1);
                    *fresh15 = 'v' as i32 as i8;
                }
                _ => {
                    sprintf(o, b"%03o\0" as *const u8 as *const i8, *p as i32);
                    o = o.offset(strlen(o) as isize);
                }
            }
        }
        olen = o.offset_from(obuf.as_mut_ptr()) as i64 as size_t;
        if width.wrapping_add(olen) >= line_len as u64 && line_len > 0 as i32 {
            ck_fwrite(
                b"\\\0" as *const u8 as *const i8 as *const libc::c_void,
                1 as i32 as size_t,
                1 as i32 as size_t,
                fp,
            );
            ck_fwrite(
                &mut buffer_delimiter as *mut i8 as *const libc::c_void,
                1 as i32 as size_t,
                1 as i32 as size_t,
                fp,
            );
            width = 0 as i32 as countT;
        }
        ck_fwrite(
            obuf.as_mut_ptr() as *const libc::c_void,
            1 as i32 as size_t,
            olen,
            fp,
        );
        width = (width as u64).wrapping_add(olen) as countT as countT;
        p = p.offset(1);
        p;
    }
    ck_fwrite(
        b"$\0" as *const u8 as *const i8 as *const libc::c_void,
        1 as i32 as size_t,
        1 as i32 as size_t,
        fp,
    );
    ck_fwrite(
        &mut buffer_delimiter as *mut i8 as *const libc::c_void,
        1 as i32 as size_t,
        1 as i32 as size_t,
        fp,
    );
    flush_output(fp);
}
unsafe extern "C" fn append_replacement(
    mut buf: *mut line,
    mut p: *mut replacement,
    mut regs: *mut re_registers,
) {
    let mut repl_mod: replacement_types = replacement_types::REPL_ASIS;
    while !p.is_null() {
        let mut i: i32 = (*p).subst_id;
        let mut curr_type: replacement_types = replacement_types::REPL_ASIS;
        curr_type = replacement_types::from_libc_c_uint(
            (if (*p).repl_type as u32 & replacement_types::REPL_MODIFIERS as i32 as u32
                != 0
            {
                (*p).repl_type as u32
            } else {
                (*p).repl_type as u32 | repl_mod as u32
            }) as u32,
        );
        repl_mod = replacement_types::REPL_ASIS;
        if (*p).prefix_length != 0 {
            str_append_modified(buf, (*p).prefix, (*p).prefix_length, curr_type);
            curr_type = ::core::mem::transmute::<
                u32,
                replacement_types,
            >(curr_type as u32 & !(replacement_types::REPL_MODIFIERS as i32) as u32);
        }
        if 0 as i32 <= i && (i as u64) < (*regs).num_regs {
            if *((*regs).end).offset(i as isize) == *((*regs).start).offset(i as isize)
                && (*p).repl_type as u32
                    & replacement_types::REPL_MODIFIERS as i32 as u32 != 0
            {
                repl_mod = replacement_types::from_libc_c_uint(
                    (curr_type as u32 & replacement_types::REPL_MODIFIERS as i32 as u32)
                        as u32,
                );
            } else if *((*regs).end).offset(i as isize)
                != *((*regs).start).offset(i as isize)
            {
                str_append_modified(
                    buf,
                    (line.active).offset(*((*regs).start).offset(i as isize) as isize),
                    (*((*regs).end).offset(i as isize)
                        - *((*regs).start).offset(i as isize)) as size_t,
                    curr_type,
                );
            }
        }
        p = (*p).next;
    }
}
unsafe extern "C" fn do_subst(mut sub: *mut subst) {
    let mut current_block: u64;
    let mut start: size_t = 0 as i32 as size_t;
    let mut last_end: size_t = 0 as i32 as size_t;
    let mut count: countT = 0 as i32 as countT;
    let mut again: bool = 1 as i32 != 0;
    static mut regs: re_registers = re_registers {
        num_regs: 0,
        start: 0 as *const regoff_t as *mut regoff_t,
        end: 0 as *const regoff_t as *mut regoff_t,
    };
    line_reset(&mut s_accum, &mut line);
    if match_regex(
        (*sub).regx,
        line.active,
        line.length,
        start,
        &mut regs,
        (*sub).max_id() as i32 + 1 as i32,
    ) == 0
    {
        return;
    }
    if debug {
        if regs.num_regs > 0 as i32 as u64
            && *(regs.start).offset(0 as i32 as isize) != -(1 as i32) as i64
        {
            puts(b"MATCHED REGEX REGISTERS\0" as *const u8 as *const i8);
        }
        let mut i: i32 = 0 as i32;
        while (i as u64) < regs.num_regs {
            if *(regs.start).offset(i as isize) == -(1 as i32) as i64 {
                break;
            }
            printf(
                b"  regex[%d] = %d-%d '\0" as *const u8 as *const i8,
                i,
                *(regs.start).offset(i as isize) as i32,
                *(regs.end).offset(i as isize) as i32,
            );
            if *(regs.start).offset(i as isize) != *(regs.end).offset(i as isize) {
                if 0 != 0 && 0 != 0
                    && ((*(regs.end).offset(i as isize)
                        - *(regs.start).offset(i as isize)) as size_t)
                        .wrapping_mul(1 as i32 as size_t) <= 8 as i32 as u64
                    && (*(regs.end).offset(i as isize)
                        - *(regs.start).offset(i as isize)) as size_t != 0 as i32 as u64
                {
                    ({
                        let mut __ptr: *const i8 = (line.active)
                            .offset(*(regs.start).offset(i as isize) as isize)
                            as *const i8;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = ((*(regs.end).offset(i as isize)
                            - *(regs.start).offset(i as isize)) as size_t)
                            .wrapping_mul(1 as i32 as size_t);
                        while __cnt > 0 as i32 as u64 {
                            if (if ((*__stream)._IO_write_ptr
                                >= (*__stream)._IO_write_end) as i32 as i64 != 0
                            {
                                let fresh16 = __ptr;
                                __ptr = __ptr.offset(1);
                                __overflow(__stream, *fresh16 as u8 as i32)
                            } else {
                                let fresh17 = __ptr;
                                __ptr = __ptr.offset(1);
                                let fresh18 = (*__stream)._IO_write_ptr;
                                (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                    .offset(1);
                                *fresh18 = *fresh17;
                                *fresh18 as u8 as i32
                            }) == -(1 as i32)
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        compile_error!("Binary expression is not supposed to be used")
                    });
                } else {
                    if 0 != 0
                        && (*(regs.end).offset(i as isize)
                            - *(regs.start).offset(i as isize)) as size_t
                            == 0 as i32 as u64
                        || 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                    {} else {
                        fwrite_unlocked(
                            (line.active)
                                .offset(*(regs.start).offset(i as isize) as isize)
                                as *const libc::c_void,
                            (*(regs.end).offset(i as isize)
                                - *(regs.start).offset(i as isize)) as size_t,
                            1 as i32 as size_t,
                            stdout,
                        );
                    };
                };
                compile_error!("Conditional expression is not supposed to be used");
            }
            puts(b"'\0" as *const u8 as *const i8);
            i += 1;
            i;
        }
    }
    if ((*sub).replacement).is_null() && (*sub).numb <= 1 as i32 as u64 {
        if *(regs.start).offset(0 as i32 as isize) == 0 as i32 as i64
            && (*sub).global() == 0
        {
            replaced = 1 as i32 != 0;
            line.active = (line.active)
                .offset(*(regs.end).offset(0 as i32 as isize) as isize);
            line.length = (line.length as u64)
                .wrapping_sub(*(regs.end).offset(0 as i32 as isize) as u64) as size_t
                as size_t;
            line.alloc = (line.alloc as u64)
                .wrapping_sub(*(regs.end).offset(0 as i32 as isize) as u64) as size_t
                as size_t;
            current_block = 8791965866217958940;
        } else if *(regs.end).offset(0 as i32 as isize) as u64 == line.length {
            replaced = 1 as i32 != 0;
            line.length = *(regs.start).offset(0 as i32 as isize) as size_t;
            current_block = 8791965866217958940;
        } else {
            current_block = 15089075282327824602;
        }
    } else {
        current_block = 15089075282327824602;
    }
    match current_block {
        15089075282327824602 => {
            loop {
                let mut offset: size_t = *(regs.start).offset(0 as i32 as isize)
                    as size_t;
                let mut matched: size_t = (*(regs.end).offset(0 as i32 as isize)
                    - *(regs.start).offset(0 as i32 as isize)) as size_t;
                if start < offset {
                    str_append(
                        &mut s_accum,
                        (line.active).offset(start as isize),
                        offset.wrapping_sub(start),
                    );
                    start = offset;
                }
                if (matched > 0 as i32 as u64 || count == 0 as i32 as u64
                    || offset > last_end)
                    && {
                        count = count.wrapping_add(1);
                        count >= (*sub).numb
                    }
                {
                    replaced = 1 as i32 != 0;
                    append_replacement(&mut s_accum, (*sub).replacement, &mut regs);
                    again = (*sub).global() != 0;
                } else {
                    if matched == 0 as i32 as u64 {
                        if !(start < line.length) {
                            break;
                        }
                        matched = 1 as i32 as size_t;
                    }
                    str_append(
                        &mut s_accum,
                        (line.active).offset(offset as isize),
                        matched,
                    );
                }
                start = offset.wrapping_add(matched);
                last_end = *(regs.end).offset(0 as i32 as isize) as size_t;
                if !(again as i32 != 0 && start <= line.length
                    && match_regex(
                        (*sub).regx,
                        line.active,
                        line.length,
                        start,
                        &mut regs,
                        (*sub).max_id() as i32 + 1 as i32,
                    ) != 0)
                {
                    break;
                }
            }
            if start < line.length {
                str_append(
                    &mut s_accum,
                    (line.active).offset(start as isize),
                    (line.length).wrapping_sub(start),
                );
            }
            s_accum.chomped = line.chomped;
            line_exchange(&mut line, &mut s_accum, 0 as i32);
            if count < (*sub).numb {
                return;
            }
        }
        _ => {}
    }
    if (*sub).print() as i32 & 1 as i32 != 0 {
        output_line(line.active, line.length, line.chomped as i32, &mut output_file);
    }
    if (*sub).eval() != 0 {
        let mut pipe_fp: *mut FILE = 0 as *mut FILE;
        line_reset(&mut s_accum, 0 as *mut line);
        str_append(&mut line, b"\0" as *const u8 as *const i8, 1 as i32 as size_t);
        pipe_fp = popen(line.active, b"r\0" as *const u8 as *const i8);
        if !pipe_fp.is_null() {
            while feof_unlocked(pipe_fp) == 0 {
                let mut buf: [i8; 4096] = [0; 4096];
                let mut n: i32 = (if 0 != 0 && 0 != 0
                    && (::core::mem::size_of::<i8>() as u64)
                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                {
                    ({
                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                        let mut __stream: *mut FILE = pipe_fp;
                        let mut __cnt: size_t = 0;
                        __cnt = (::core::mem::size_of::<i8>() as u64)
                            .wrapping_mul(4096 as i32 as size_t);
                        while __cnt > 0 as i32 as u64 {
                            let mut __c: i32 = if ((*__stream)._IO_read_ptr
                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                            {
                                __uflow(__stream)
                            } else {
                                let fresh19 = (*__stream)._IO_read_ptr;
                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                    .offset(1);
                                *(fresh19 as *mut u8) as i32
                            };
                            if __c == -(1 as i32) {
                                break;
                            }
                            let fresh20 = __ptr;
                            __ptr = __ptr.offset(1);
                            *fresh20 = __c as i8;
                            __cnt = __cnt.wrapping_sub(1);
                            __cnt;
                        }
                        (::core::mem::size_of::<i8>() as u64)
                            .wrapping_mul(4096 as i32 as size_t)
                            .wrapping_sub(__cnt)
                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                    })
                } else if 0 != 0
                    && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                    || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                {
                    0 as i32 as size_t
                } else {
                    fread_unlocked(
                        buf.as_mut_ptr() as *mut libc::c_void,
                        ::core::mem::size_of::<i8>() as u64,
                        4096 as i32 as size_t,
                        pipe_fp,
                    )
                }) as i32;
                if n > 0 as i32 {
                    str_append(&mut s_accum, buf.as_mut_ptr(), n as size_t);
                }
            }
            pclose(pipe_fp);
            line_exchange(&mut line, &mut s_accum, 1 as i32);
            if line.length != 0
                && *(line.active)
                    .offset((line.length).wrapping_sub(1 as i32 as u64) as isize) as i32
                    == buffer_delimiter as i32
            {
                line.length = (line.length).wrapping_sub(1);
                line.length;
            }
        } else {
            panic(
                dcgettext(
                    0 as *const i8,
                    b"error in subprocess\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    if (*sub).print() as i32 & 2 as i32 != 0 {
        output_line(line.active, line.length, line.chomped as i32, &mut output_file);
    }
    if !((*sub).outf).is_null() {
        output_line(line.active, line.length, line.chomped as i32, (*sub).outf);
    }
}
unsafe extern "C" fn translate_mb(mut trans: *const *mut i8) {
    let mut idx: size_t = 0;
    let mut mbstate: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    idx = 0 as i32 as size_t;
    while idx < line.length {
        let mut i: u32 = 0;
        let mut mbclen: size_t = if mb_cur_max == 1 as i32 {
            1 as i32 as u64
        } else {
            rpl_mbrtowc(
                0 as *mut wchar_t,
                (line.active).offset(idx as isize),
                (line.length).wrapping_sub(idx),
                &mut mbstate,
            )
        };
        if mbclen == -(1 as i32) as size_t || mbclen == -(2 as i32) as size_t
            || mbclen == 0 as i32 as u64
        {
            mbclen = 1 as i32 as size_t;
        }
        i = 0 as i32 as u32;
        while !(*trans.offset((2 as i32 as u32).wrapping_mul(i) as isize)).is_null() {
            if strncmp(
                (line.active).offset(idx as isize),
                *trans.offset((2 as i32 as u32).wrapping_mul(i) as isize),
                mbclen,
            ) == 0 as i32
            {
                let mut move_remain_buffer: bool = 0 as i32 != 0;
                let mut tr: *const i8 = *trans
                    .offset(
                        (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                            as isize,
                    );
                let mut trans_len: size_t = if *tr as i32 == '\0' as i32 {
                    1 as i32 as u64
                } else {
                    strlen(tr)
                };
                if mbclen < trans_len {
                    let mut new_len: size_t = (line.length)
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(trans_len)
                        .wrapping_sub(mbclen);
                    if line.alloc < new_len {
                        resize_line(&mut line, new_len);
                    }
                    move_remain_buffer = 1 as i32 != 0;
                } else if mbclen > trans_len {
                    move_remain_buffer = 1 as i32 != 0;
                }
                let mut prev_idx: size_t = idx;
                if move_remain_buffer {
                    let mut move_from: *const i8 = (line.active)
                        .offset(idx as isize)
                        .offset(mbclen as isize);
                    let mut move_to: *mut i8 = (line.active)
                        .offset(idx as isize)
                        .offset(trans_len as isize);
                    let mut move_len: size_t = (line.length)
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_sub(idx)
                        .wrapping_sub(mbclen);
                    let mut move_offset: size_t = trans_len.wrapping_sub(mbclen);
                    memmove(
                        move_to as *mut libc::c_void,
                        move_from as *const libc::c_void,
                        move_len,
                    );
                    line.length = (line.length as u64).wrapping_add(move_offset)
                        as size_t as size_t;
                    idx = (idx as u64).wrapping_add(move_offset) as size_t as size_t;
                }
                memcpy(
                    (line.active).offset(prev_idx as isize) as *mut libc::c_void,
                    *trans
                        .offset(
                            (2 as i32 as u32)
                                .wrapping_mul(i)
                                .wrapping_add(1 as i32 as u32) as isize,
                        ) as *const libc::c_void,
                    trans_len,
                );
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        idx = (idx as u64).wrapping_add(mbclen) as size_t as size_t;
    }
}
unsafe extern "C" fn debug_print_end_of_cycle() {
    puts(b"END-OF-CYCLE:\0" as *const u8 as *const i8);
}
unsafe extern "C" fn debug_print_input(mut input: *const input) {
    let mut is_stdin: bool = !((*input).fp).is_null() && fileno((*input).fp) == 0 as i32;
    printf(
        b"INPUT:   '%s' line %lu\n\0" as *const u8 as *const i8,
        if is_stdin as i32 != 0 {
            b"STDIN\0" as *const u8 as *const i8
        } else {
            (*input).in_file_name
        },
        (*input).line_number,
    );
}
unsafe extern "C" fn debug_print_line(mut ln: *mut line) {
    let mut src: *const i8 = if !((*ln).active).is_null() {
        (*ln).active
    } else {
        (*ln).text
    };
    let mut l: size_t = (*ln).length;
    let mut p: *const i8 = src;
    fputs_unlocked(
        if ln == &mut hold as *mut line {
            b"HOLD:    \0" as *const u8 as *const i8
        } else {
            b"PATTERN: \0" as *const u8 as *const i8
        },
        stdout,
    );
    loop {
        let fresh21 = l;
        l = l.wrapping_sub(1);
        if !(fresh21 != 0) {
            break;
        }
        let fresh22 = p;
        p = p.offset(1);
        debug_print_char(*fresh22);
    }
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn execute_program(
    mut vec: *mut vector,
    mut input: *mut input,
) -> i32 {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut end_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    cur_cmd = (*vec).v;
    end_cmd = ((*vec).v).offset((*vec).v_length as isize);
    let mut current_block_146: u64;
    while cur_cmd < end_cmd {
        if debug {
            fputs_unlocked(b"COMMAND: \0" as *const u8 as *const i8, stdout);
            debug_print_command(vec, cur_cmd);
        }
        if match_address_p(cur_cmd, input) as i32 != (*cur_cmd).addr_bang as i32 {
            match (*cur_cmd).cmd as i32 {
                97 => {
                    current_block_146 = 2244406721821505059;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                123 | 98 => {
                    current_block_146 = 16561871281012079094;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                125 | 35 | 58 => {}
                99 => {
                    current_block_146 = 1838606453089294538;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                100 => {
                    current_block_146 = 14557738807518773264;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                68 => {
                    current_block_146 = 5689001924483802034;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                101 => {
                    current_block_146 = 7172762164747879670;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                103 => {
                    current_block_146 = 2961662723340843848;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                71 => {
                    current_block_146 = 1556678417962160120;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                104 => {
                    current_block_146 = 1659277425109653256;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                72 => {
                    current_block_146 = 11755919723802969443;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                105 => {
                    current_block_146 = 640638260431735799;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                108 => {
                    current_block_146 = 1958939149152684883;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                110 => {
                    current_block_146 = 8143590154839972618;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                78 => {
                    current_block_146 = 17575987728467157134;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                112 => {
                    current_block_146 = 16005334357292621435;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                80 => {
                    current_block_146 = 13895000671692051671;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                113 => {
                    current_block_146 = 15860131741026162785;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                81 => {
                    current_block_146 = 11707886979257187789;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                114 => {
                    current_block_146 = 11004849488906185852;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                82 => {
                    current_block_146 = 13088278261633269453;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                115 => {
                    current_block_146 = 442103882833147892;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                116 => {
                    current_block_146 = 16801834866831430556;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                84 => {
                    current_block_146 = 12984810581895796367;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                119 => {
                    current_block_146 = 6466194886500373342;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                87 => {
                    current_block_146 = 8030768351463208789;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                120 => {
                    current_block_146 = 9426422318600939105;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                121 => {
                    current_block_146 = 17409469539518602963;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                122 => {
                    current_block_146 = 9933222667374193556;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                61 => {
                    current_block_146 = 1064101071442120849;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                70 => {
                    current_block_146 = 1294092529152747548;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
                _ => {
                    current_block_146 = 16625017989975992022;
                    match current_block_146 {
                        1294092529152747548 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%s%c\0" as *const u8 as *const i8,
                                (*input).in_file_name,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        1064101071442120849 => {
                            output_missing_newline(&mut output_file);
                            fprintf(
                                output_file.fp,
                                b"%lu%c\0" as *const u8 as *const i8,
                                (*input).line_number,
                                buffer_delimiter as i32,
                            );
                            flush_output(output_file.fp);
                            current_block_146 = 9255187738567101705;
                        }
                        9933222667374193556 => {
                            line.length = 0 as i32 as size_t;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17409469539518602963 => {
                            if mb_cur_max > 1 as i32 {
                                translate_mb((*cur_cmd).x.translatemb);
                            } else {
                                let mut p_2: *mut u8 = 0 as *mut u8;
                                let mut e: *mut u8 = 0 as *mut u8;
                                p_2 = line.active as *mut u8;
                                e = p_2.offset(line.length as isize);
                                while p_2 < e {
                                    *p_2 = *((*cur_cmd).x.translate).offset(*p_2 as isize);
                                    p_2 = p_2.offset(1);
                                    p_2;
                                }
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        9426422318600939105 => {
                            line_exchange(&mut line, &mut hold, 0 as i32);
                            if debug {
                                debug_print_line(&mut line);
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        12984810581895796367 => {
                            if !replaced {
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                replaced = 0 as i32 != 0;
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16801834866831430556 => {
                            if replaced {
                                replaced = 0 as i32 != 0;
                                cur_cmd = ((*vec).v)
                                    .offset((*cur_cmd).x.jump_index as isize);
                                continue;
                            } else {
                                current_block_146 = 9255187738567101705;
                            }
                        }
                        442103882833147892 => {
                            do_subst((*cur_cmd).x.cmd_subst);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        17575987728467157134 => {
                            str_append(
                                &mut line,
                                &mut buffer_delimiter,
                                1 as i32 as size_t,
                            );
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 1 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                line.length = (line.length).wrapping_sub(1);
                                line.length;
                                if posixicity as u32
                                    == posixicity_types::POSIXLY_EXTENDED as i32 as u32
                                    && !no_default_output
                                {
                                    output_line(
                                        line.active,
                                        line.length,
                                        line.chomped as i32,
                                        &mut output_file,
                                    );
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8143590154839972618 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            if test_eof(input) as i32 != 0
                                || !read_pattern_space(input, vec, 0 as i32)
                            {
                                if debug {
                                    debug_print_end_of_cycle();
                                }
                                return -(1 as i32);
                            }
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        11755919723802969443 => {
                            line_append(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1659277425109653256 => {
                            line_copy(&mut line, &mut hold, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        1556678417962160120 => {
                            line_append(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut line);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        2961662723340843848 => {
                            line_copy(&mut hold, &mut line, 1 as i32);
                            if debug {
                                debug_print_line(&mut hold);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        7172762164747879670 => {
                            let mut pipe_fp: *mut FILE = 0 as *mut FILE;
                            let mut cmd_length: size_t = (*cur_cmd)
                                .x
                                .cmd_txt
                                .text_length;
                            line_reset(&mut s_accum, 0 as *mut line);
                            if cmd_length == 0 {
                                str_append(
                                    &mut line,
                                    b"\0" as *const u8 as *const i8,
                                    1 as i32 as size_t,
                                );
                                pipe_fp = popen(
                                    line.active,
                                    b"r\0" as *const u8 as *const i8,
                                );
                            } else {
                                *((*cur_cmd).x.cmd_txt.text)
                                    .offset(
                                        cmd_length.wrapping_sub(1 as i32 as u64) as isize,
                                    ) = 0 as i32 as i8;
                                pipe_fp = popen(
                                    (*cur_cmd).x.cmd_txt.text,
                                    b"r\0" as *const u8 as *const i8,
                                );
                                output_missing_newline(&mut output_file);
                            }
                            if pipe_fp.is_null() {
                                panic(
                                    dcgettext(
                                        0 as *const i8,
                                        b"error in subprocess\0" as *const u8 as *const i8,
                                        5 as i32,
                                    ),
                                );
                            }
                            let mut buf: [i8; 4096] = [0; 4096];
                            let mut n: size_t = 0;
                            while feof_unlocked(pipe_fp) == 0 {
                                n = (if 0 != 0 && 0 != 0
                                    && (::core::mem::size_of::<i8>() as u64)
                                        .wrapping_mul(4096 as i32 as size_t) <= 8 as i32 as u64
                                    && ::core::mem::size_of::<i8>() as u64 != 0 as i32 as u64
                                {
                                    ({
                                        let mut __ptr: *mut i8 = buf.as_mut_ptr();
                                        let mut __stream: *mut FILE = pipe_fp;
                                        let mut __cnt: size_t = 0;
                                        __cnt = (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t);
                                        while __cnt > 0 as i32 as u64 {
                                            let mut __c: i32 = (if ((*__stream)._IO_read_ptr
                                                >= (*__stream)._IO_read_end) as i32 as i64 != 0
                                            {
                                                __uflow(__stream)
                                            } else {
                                                let fresh23 = (*__stream)._IO_read_ptr;
                                                (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                                    .offset(1);
                                                *(fresh23 as *mut u8) as i32
                                            });
                                            if __c == -(1 as i32) {
                                                break;
                                            }
                                            let fresh24 = __ptr;
                                            __ptr = __ptr.offset(1);
                                            *fresh24 = __c as i8;
                                            __cnt = __cnt.wrapping_sub(1);
                                            __cnt;
                                        }
                                        (::core::mem::size_of::<i8>() as u64)
                                            .wrapping_mul(4096 as i32 as size_t)
                                            .wrapping_sub(__cnt)
                                            .wrapping_div(::core::mem::size_of::<i8>() as u64)
                                    })
                                } else {
                                    (if 0 != 0
                                        && ::core::mem::size_of::<i8>() as u64 == 0 as i32 as u64
                                        || 0 != 0 && 4096 as i32 as size_t == 0 as i32 as u64
                                    {
                                        0 as i32 as size_t
                                    } else {
                                        fread_unlocked(
                                            buf.as_mut_ptr() as *mut libc::c_void,
                                            ::core::mem::size_of::<i8>() as u64,
                                            4096 as i32 as size_t,
                                            pipe_fp,
                                        )
                                    })
                                });
                                if n > 0 as i32 as u64 {
                                    if cmd_length == 0 {
                                        str_append(&mut s_accum, buf.as_mut_ptr(), n);
                                    } else {
                                        ck_fwrite(
                                            buf.as_mut_ptr() as *const libc::c_void,
                                            1 as i32 as size_t,
                                            n,
                                            output_file.fp,
                                        );
                                    }
                                }
                            }
                            pclose(pipe_fp);
                            if cmd_length == 0 {
                                if s_accum.length != 0
                                    && *(s_accum.active)
                                        .offset(
                                            (s_accum.length).wrapping_sub(1 as i32 as u64) as isize,
                                        ) as i32 == buffer_delimiter as i32
                                {
                                    s_accum.length = (s_accum.length).wrapping_sub(1);
                                    s_accum.length;
                                }
                                line_exchange(&mut line, &mut s_accum, 1 as i32);
                            } else {
                                flush_output(output_file.fp);
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        5689001924483802034 => {
                            let mut p: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            if p.is_null() {
                                return -(1 as i32);
                            }
                            p = p.offset(1);
                            p;
                            line.alloc = (line.alloc as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.length = (line.length as u64)
                                .wrapping_sub(p.offset_from(line.active) as i64 as u64)
                                as size_t as size_t;
                            line.active = (line.active)
                                .offset(p.offset_from(line.active) as i64 as isize);
                            cur_cmd = (*vec).v;
                            if debug {
                                debug_print_line(&mut line);
                            }
                            continue;
                        }
                        15860131741026162785 => {
                            if !no_default_output {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    &mut output_file,
                                );
                            }
                            dump_append_queue();
                            current_block_146 = 11707886979257187789;
                        }
                        2244406721821505059 => {
                            let mut aq: *mut append_queue = next_append_slot();
                            (*aq).text = (*cur_cmd).x.cmd_txt.text;
                            (*aq).textlen = (*cur_cmd).x.cmd_txt.text_length;
                            current_block_146 = 9255187738567101705;
                        }
                        16561871281012079094 => {
                            cur_cmd = ((*vec).v)
                                .offset((*cur_cmd).x.jump_index as isize);
                            continue;
                        }
                        1838606453089294538 => {
                            if (*cur_cmd).range_state as u32
                                != addr_state::RANGE_ACTIVE as i32 as u32
                            {
                                output_line(
                                    (*cur_cmd).x.cmd_txt.text,
                                    ((*cur_cmd).x.cmd_txt.text_length)
                                        .wrapping_sub(1 as i32 as u64),
                                    1 as i32,
                                    &mut output_file,
                                );
                            }
                            current_block_146 = 14557738807518773264;
                        }
                        640638260431735799 => {
                            output_line(
                                (*cur_cmd).x.cmd_txt.text,
                                ((*cur_cmd).x.cmd_txt.text_length)
                                    .wrapping_sub(1 as i32 as u64),
                                1 as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        1958939149152684883 => {
                            do_list(
                                (if (*cur_cmd).x.int_arg == -(1 as i32) {
                                    lcmd_out_line_len
                                } else {
                                    (*cur_cmd).x.int_arg as u64
                                }) as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        16005334357292621435 => {
                            output_line(
                                line.active,
                                line.length,
                                line.chomped as i32,
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        13895000671692051671 => {
                            let mut p_0: *mut i8 = memchr(
                                line.active as *const libc::c_void,
                                buffer_delimiter as i32,
                                line.length,
                            ) as *mut i8;
                            output_line(
                                line.active,
                                if !p_0.is_null() {
                                    p_0.offset_from(line.active) as i64 as u64
                                } else {
                                    line.length
                                },
                                if !p_0.is_null() { 1 as i32 } else { line.chomped as i32 },
                                &mut output_file,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        11004849488906185852 => {
                            if !((*cur_cmd).x.readcmd.fname).is_null() {
                                if (*cur_cmd).x.readcmd.append {
                                    let mut aq_0: *mut append_queue = next_append_slot();
                                    (*aq_0).fname = (*cur_cmd).x.readcmd.fname;
                                } else {
                                    print_file((*cur_cmd).x.readcmd.fname, output_file.fp);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        13088278261633269453 => {
                            if !((*(*cur_cmd).x.inf).fp).is_null()
                                && feof_unlocked((*(*cur_cmd).x.inf).fp) == 0
                            {
                                let mut aq_1: *mut append_queue = 0 as *mut append_queue;
                                let mut buflen: size_t = 0;
                                let mut text: *mut i8 = 0 as *mut i8;
                                let mut result: size_t = 0;
                                result = ck_getdelim(
                                    &mut text,
                                    &mut buflen,
                                    buffer_delimiter,
                                    (*(*cur_cmd).x.inf).fp,
                                );
                                if result != -(1 as i32) as u64 {
                                    aq_1 = next_append_slot();
                                    (*aq_1).rpl_free = 1 as i32 != 0;
                                    (*aq_1).text = text;
                                    (*aq_1).textlen = result;
                                } else {
                                    rpl_free(text as *mut libc::c_void);
                                }
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        6466194886500373342 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                output_line(
                                    line.active,
                                    line.length,
                                    line.chomped as i32,
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        8030768351463208789 => {
                            if !((*(*cur_cmd).x.outf).fp).is_null() {
                                let mut p_1: *mut i8 = memchr(
                                    line.active as *const libc::c_void,
                                    buffer_delimiter as i32,
                                    line.length,
                                ) as *mut i8;
                                output_line(
                                    line.active,
                                    if !p_1.is_null() {
                                        p_1.offset_from(line.active) as i64 as u64
                                    } else {
                                        line.length
                                    },
                                    if !p_1.is_null() { 1 as i32 } else { line.chomped as i32 },
                                    (*cur_cmd).x.outf,
                                );
                            }
                            current_block_146 = 9255187738567101705;
                        }
                        16625017989975992022 => {
                            panic(
                                b"INTERNAL ERROR: Bad cmd %c\0" as *const u8 as *const i8,
                                (*cur_cmd).cmd as i32,
                            );
                            current_block_146 = 9255187738567101705;
                        }
                        _ => {}
                    }
                    match current_block_146 {
                        9255187738567101705 => {}
                        _ => {
                            match current_block_146 {
                                11707886979257187789 => {
                                    return if (*cur_cmd).x.int_arg == -(1 as i32) {
                                        0 as i32
                                    } else {
                                        (*cur_cmd).x.int_arg
                                    };
                                }
                                _ => {
                                    if debug {
                                        debug_print_end_of_cycle();
                                    }
                                    return -(1 as i32);
                                }
                            }
                        }
                    }
                }
            }
        }
        cur_cmd = cur_cmd.offset(1);
        cur_cmd;
    }
    if debug {
        debug_print_end_of_cycle();
    }
    if !no_default_output {
        output_line(line.active, line.length, line.chomped as i32, &mut output_file);
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn process_files(
    mut the_program: *mut vector,
    mut argv: *mut *mut i8,
) -> i32 {
    static mut dash: [i8; 2] = unsafe {
        *::core::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b"-\0")
    };
    static mut stdin_argv: [*mut i8; 2] = unsafe {
        [dash.as_ptr() as *mut _, 0 as *const i8 as *mut i8]
    };
    let mut input: input = input {
        file_list: 0 as *mut *mut i8,
        bad_count: 0,
        line_number: 0,
        reset_at_next_file: false,
        read_fn: None,
        out_file_name: 0 as *mut i8,
        in_file_name: 0 as *const i8,
        st: stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        },
        fp: 0 as *mut FILE,
        no_buffering: false,
    };
    let mut status: i32 = 0;
    line_init(&mut line, 0 as *mut line, 50 as i32 as size_t);
    line_init(&mut hold, 0 as *mut line, 0 as i32 as size_t);
    line_init(&mut buffer, 0 as *mut line, 0 as i32 as size_t);
    input.reset_at_next_file = 1 as i32 != 0;
    if !argv.is_null() && !(*argv).is_null() {
        input.file_list = argv;
    } else if !in_place_extension.is_null() {
        panic(
            dcgettext(
                0 as *const i8,
                b"no input files\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        input.file_list = stdin_argv.as_mut_ptr();
    }
    input.bad_count = 0 as i32 as countT;
    input.line_number = 0 as i32 as countT;
    input.read_fn = Some(read_always_fail as unsafe extern "C" fn(*mut input) -> bool);
    input.fp = 0 as *mut FILE;
    status = 0 as i32;
    while read_pattern_space(&mut input, the_program, 0 as i32) {
        if debug {
            debug_print_input(&mut input);
            debug_print_line(&mut line);
        }
        status = execute_program(the_program, &mut input);
        if !(status == -(1 as i32)) {
            break;
        }
        status = 0 as i32;
    }
    closedown(&mut input);
    if input.bad_count != 0 {
        status = exit_codes::EXIT_BAD_INPUT as i32;
    }
    return status;
}