#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> i32;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn initialize_mbcs();
    fn init_localeinfo(_: *mut localeinfo);
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn ck_fclose(stream: *mut FILE);
    fn remove_cleanup_file();
    fn compile_string(_: *mut vector, str: *mut i8, len: size_t) -> *mut vector;
    fn compile_file(_: *mut vector, cmdfile: *const i8) -> *mut vector;
    fn check_final_program(_: *mut vector);
    fn finish_program(_: *mut vector);
    fn debug_print_program(program: *const vector);
    fn process_files(_: *mut vector, argv: *mut *mut i8) -> i32;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    static mut program_name: *const i8;
    fn set_program_name(argv0: *const i8);
    static mut Version: *const i8;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const i8,
        package: *const i8,
        version: *const i8,
        _: ...
    );
}
pub type size_t = u64;
pub type wint_t = u32;
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
pub type countT = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [libc::c_schar; 256],
    pub sbctowc: [wint_t; 256],
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
    pub x: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    DEBUG_OPTION = 129,
    SANDBOX_OPTION = 128,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::DEBUG_OPTION => 129,
            C2RustUnnamed_0::SANDBOX_OPTION => 128,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            129 => C2RustUnnamed_0::DEBUG_OPTION,
            128 => C2RustUnnamed_0::SANDBOX_OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: i32) -> i32 {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as i32 as i64 != 0 {
        __overflow(stdout, __c as u8 as i32)
    } else {
        let fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = ((*stdout)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
#[no_mangle]
pub static mut extended_regexp_flags: i32 = 0 as i32;
#[no_mangle]
pub static mut buffer_delimiter: i8 = '\n' as i32 as i8;
#[no_mangle]
pub static mut unbuffered: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut no_default_output: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut separate_files: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut follow_symlinks: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut sandbox: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut debug: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut in_place_extension: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut read_mode: *const i8 = b"r\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut write_mode: *const i8 = b"w\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut posixicity: posixicity_types = posixicity_types::POSIXLY_EXTENDED;
#[no_mangle]
pub static mut lcmd_out_line_len: countT = 70 as i32 as countT;
static mut the_program: *mut vector = 0 as *const vector as *mut vector;
#[no_mangle]
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
unsafe extern "C" fn cleanup() {
    remove_cleanup_file();
}
unsafe extern "C" fn contact(mut errmsg: i32) {
    let mut out: *mut FILE = if errmsg != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"GNU sed home page: <https://www.gnu.org/software/sed/>.\nGeneral help using GNU software: <https://www.gnu.org/gethelp/>.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    if errmsg == 0 {
        fprintf(
            out,
            dcgettext(
                0 as *const i8,
                b"E-mail bug reports to: <%s>.\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            b"bug-sed@gnu.org\0" as *const u8 as *const i8,
        );
    }
}
unsafe extern "C" fn selinux_support() {
    putchar_unlocked('\n' as i32);
    puts(
        dcgettext(
            0 as *const i8,
            b"This sed program was built without SELinux support.\0" as *const u8
                as *const i8,
            5 as i32,
        ),
    );
    putchar_unlocked('\n' as i32);
}
unsafe extern "C" fn usage(mut status: i32) {
    let mut out: *mut FILE = if status != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"Usage: %s [OPTION]... {script-only-if-no-other-script} [input-file]...\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        program_name,
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -n, --quiet, --silent\n                 suppress automatic printing of pattern space\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"      --debug\n                 annotate program execution\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -e script, --expression=script\n                 add the script to the commands to be executed\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -f script-file, --file=script-file\n                 add the contents of script-file to the commands to be executed\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  --follow-symlinks\n                 follow symlinks when processing in place\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -i[SUFFIX], --in-place[=SUFFIX]\n                 edit files in place (makes backup if SUFFIX supplied)\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -l N, --line-length=N\n                 specify the desired line-wrap length for the `l' command\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  --posix\n                 disable all GNU extensions.\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -E, -r, --regexp-extended\n                 use extended regular expressions in the script\n                 (for portability use POSIX -E).\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -s, --separate\n                 consider files as separate rather than as a single,\n                 continuous long stream.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"      --sandbox\n                 operate in sandbox mode (disable e/r/w commands).\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -u, --unbuffered\n                 load minimal amounts of data from the input files and flush\n                 the output buffers more often\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"  -z, --null-data\n                 separate lines by NUL characters\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"      --help     display this help and exit\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"      --version  output version information and exit\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
    );
    fprintf(
        out,
        dcgettext(
            0 as *const i8,
            b"\nIf no -e, --expression, -f, or --file option is given, then the first\nnon-option argument is taken as the sed script to interpret.  All\nremaining arguments are names of input files; if no input files are\nspecified, then the standard input is read.\n\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    contact(status);
    ck_fclose(0 as *mut FILE);
    exit(status);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    static mut longopts: [option; 19] = [
        {
            let mut init = option {
                name: b"binary\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"regexp-extended\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"debug\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_0::DEBUG_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"expression\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"in-place\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-length\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"null-data\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"zero-terminated\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"posix\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"sandbox\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: C2RustUnnamed_0::SANDBOX_OPTION as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"separate\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"unbuffered\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"follow-symlinks\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
    ];
    let mut opt: i32 = 0;
    let mut return_code: i32 = 0;
    let mut cols: *const i8 = getenv(b"COLS\0" as *const u8 as *const i8);
    set_program_name(*argv.offset(0 as i32 as isize));
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    initialize_mbcs();
    init_localeinfo(&mut localeinfo);
    atexit(Some(cleanup as unsafe extern "C" fn() -> ()));
    bindtextdomain(
        b"sed\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"sed\0" as *const u8 as *const i8);
    if !(getenv(b"posixicity_types::POSIXLY_CORRECT\0" as *const u8 as *const i8))
        .is_null()
    {
        posixicity = posixicity_types::POSIXLY_CORRECT;
    } else {
        posixicity = posixicity_types::POSIXLY_EXTENDED;
    }
    if !cols.is_null() {
        let mut t: countT = atoi(cols) as countT;
        if t > 1 as i32 as u64 {
            lcmd_out_line_len = t.wrapping_sub(1 as i32 as u64);
        }
    }
    loop {
        opt = getopt_long(
            argc,
            argv,
            b"bsnrzuEe:f:l:i::V:\0" as *const u8 as *const i8,
            longopts.as_ptr(),
            0 as *mut i32,
        );
        if !(opt != -(1 as i32)) {
            break;
        }
        let mut current_block_45: u64;
        match opt {
            110 => {
                no_default_output = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            101 => {
                the_program = compile_string(the_program, optarg, strlen(optarg));
                current_block_45 = 10150597327160359210;
            }
            102 => {
                the_program = compile_file(the_program, optarg);
                current_block_45 = 10150597327160359210;
            }
            122 => {
                buffer_delimiter = 0 as i32 as i8;
                current_block_45 = 10150597327160359210;
            }
            70 => {
                follow_symlinks = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            105 => {
                separate_files = 1 as i32 != 0;
                if optarg.is_null() {
                    in_place_extension = xstrdup(b"*\0" as *const u8 as *const i8);
                } else if !(strchr(optarg, '*' as i32)).is_null() {
                    in_place_extension = xstrdup(optarg);
                } else {
                    in_place_extension = (if ::core::mem::size_of::<i8>() as u64
                        == 1 as i32 as u64
                    {
                        xzalloc((strlen(optarg)).wrapping_add(2 as i32 as u64))
                    } else {
                        xcalloc(
                            (strlen(optarg)).wrapping_add(2 as i32 as u64),
                            ::core::mem::size_of::<i8>() as u64,
                        )
                    }) as *mut i8;
                    *in_place_extension.offset(0 as i32 as isize) = '*' as i32 as i8;
                    strcpy(in_place_extension.offset(1 as i32 as isize), optarg);
                }
                current_block_45 = 10150597327160359210;
            }
            108 => {
                lcmd_out_line_len = atoi(optarg) as countT;
                current_block_45 = 10150597327160359210;
            }
            112 => {
                posixicity = posixicity_types::POSIXLY_BASIC;
                current_block_45 = 10150597327160359210;
            }
            98 => {
                read_mode = b"rb\0" as *const u8 as *const i8;
                write_mode = b"wb\0" as *const u8 as *const i8;
                current_block_45 = 10150597327160359210;
            }
            69 | 114 => {
                extended_regexp_flags = 1 as i32;
                current_block_45 = 10150597327160359210;
            }
            115 => {
                separate_files = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            128 => {
                sandbox = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            129 => {
                debug = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            117 => {
                unbuffered = 1 as i32 != 0;
                current_block_45 = 10150597327160359210;
            }
            118 => {
                version_etc(
                    stdout,
                    program_name,
                    b"GNU sed\0" as *const u8 as *const i8,
                    Version,
                    dcgettext(
                        0 as *const i8,
                        b"Jay Fenlason\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dcgettext(
                        0 as *const i8,
                        b"Tom Lord\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dcgettext(
                        0 as *const i8,
                        b"Ken Pizzini\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dcgettext(
                        0 as *const i8,
                        b"Paolo Bonzini\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dcgettext(
                        0 as *const i8,
                        b"Jim Meyering\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    dcgettext(
                        0 as *const i8,
                        b"Assaf Gordon\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    0 as *mut libc::c_void as *mut i8,
                );
                selinux_support();
                contact(0 as i32);
                ck_fclose(0 as *mut FILE);
                exit(0 as i32);
            }
            104 => {
                usage(0 as i32);
                current_block_45 = 9580004622280961298;
            }
            _ => {
                current_block_45 = 9580004622280961298;
            }
        }
        match current_block_45 {
            9580004622280961298 => {
                usage(exit_codes::EXIT_BAD_USAGE as i32);
            }
            _ => {}
        }
    }
    if the_program.is_null() {
        if optind < argc {
            let fresh1 = optind;
            optind = optind + 1;
            let mut arg: *mut i8 = *argv.offset(fresh1 as isize);
            the_program = compile_string(the_program, arg, strlen(arg));
        } else {
            usage(exit_codes::EXIT_BAD_USAGE as i32);
        }
    }
    check_final_program(the_program);
    if debug {
        debug_print_program(the_program);
    }
    return_code = process_files(the_program, argv.offset(optind as isize));
    finish_program(the_program);
    ck_fclose(0 as *mut FILE);
    return return_code;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}