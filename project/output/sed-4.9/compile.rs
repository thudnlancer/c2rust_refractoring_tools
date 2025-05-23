use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    pub type buffer;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn rpl_free(_: *mut libc::c_void);
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    static mut no_default_output: bool;
    static mut debug: bool;
    static mut read_mode: *const i8;
    static mut write_mode: *const i8;
    fn compile_regex(b: *mut buffer, flags: i32, needed_sub: i32) -> *mut regex;
    static mut sandbox: bool;
    fn is_mb_char(ch: i32, ps: *mut mbstate_t) -> i32;
    static mut mb_cur_max: i32;
    static mut posixicity: posixicity_types;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xreallocarray(p: *mut libc::c_void, n: size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn __uflow(_: *mut _IO_FILE) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn ungetc(__c: i32, __stream: *mut FILE) -> i32;
    fn ftell(__stream: *mut FILE) -> i64;
    fn rewind(__stream: *mut FILE);
    fn free_buffer(b: *mut buffer);
    fn add1_buffer(b: *mut buffer, ch: i32) -> *mut i8;
    fn panic(str: *const i8, _: ...);
    fn ck_fopen(name: *const i8, mode: *const i8, fail: i32) -> *mut FILE;
    fn ck_fclose(stream: *mut FILE);
    fn init_buffer() -> *mut buffer;
    fn get_buffer(b: *const buffer) -> *mut i8;
    fn size_buffer(b: *const buffer) -> size_t;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strverscmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
    static mut program_name: *const i8;
}
pub type size_t = u64;
pub type wchar_t = i32;
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
pub type ptrdiff_t = i64;
pub type __int32_t = i32;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
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
    pub x: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub enum text_types {
    TEXT_BUFFER,
    TEXT_REPLACEMENT,
    TEXT_REGEX,
}
impl text_types {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            text_types::TEXT_BUFFER => 0,
            text_types::TEXT_REPLACEMENT => 1,
            text_types::TEXT_REGEX => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> text_types {
        match value {
            0 => text_types::TEXT_BUFFER,
            1 => text_types::TEXT_REPLACEMENT,
            2 => text_types::TEXT_REGEX,
            _ => panic!("Invalid value for text_types: {}", value),
        }
    }
}
impl AddAssign<u32> for text_types {
    fn add_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for text_types {
    fn sub_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for text_types {
    fn mul_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for text_types {
    fn div_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for text_types {
    fn rem_assign(&mut self, rhs: u32) {
        *self = text_types::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for text_types {
    type Output = text_types;
    fn add(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for text_types {
    type Output = text_types;
    fn sub(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for text_types {
    type Output = text_types;
    fn mul(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for text_types {
    type Output = text_types;
    fn div(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for text_types {
    type Output = text_types;
    fn rem(self, rhs: u32) -> text_types {
        text_types::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
pub struct prog_info {
    pub base: *const u8,
    pub cur: *const u8,
    pub end: *const u8,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error_info {
    pub name: *const i8,
    pub line: countT,
    pub string_expr_count: countT,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    _ISblank = 1,
    _ISdigit = 2048,
    _ISspace = 8192,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISxdigit = 4096,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::_ISblank => 1,
            C2RustUnnamed_4::_ISdigit => 2048,
            C2RustUnnamed_4::_ISspace => 8192,
            C2RustUnnamed_4::_ISalnum => 8,
            C2RustUnnamed_4::_ISpunct => 4,
            C2RustUnnamed_4::_IScntrl => 2,
            C2RustUnnamed_4::_ISgraph => 32768,
            C2RustUnnamed_4::_ISprint => 16384,
            C2RustUnnamed_4::_ISxdigit => 4096,
            C2RustUnnamed_4::_ISalpha => 1024,
            C2RustUnnamed_4::_ISlower => 512,
            C2RustUnnamed_4::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            1 => C2RustUnnamed_4::_ISblank,
            2048 => C2RustUnnamed_4::_ISdigit,
            8192 => C2RustUnnamed_4::_ISspace,
            8 => C2RustUnnamed_4::_ISalnum,
            4 => C2RustUnnamed_4::_ISpunct,
            2 => C2RustUnnamed_4::_IScntrl,
            32768 => C2RustUnnamed_4::_ISgraph,
            16384 => C2RustUnnamed_4::_ISprint,
            4096 => C2RustUnnamed_4::_ISxdigit,
            1024 => C2RustUnnamed_4::_ISalpha,
            512 => C2RustUnnamed_4::_ISlower,
            256 => C2RustUnnamed_4::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed_4: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_4 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_4 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_4 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_4 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_4 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn add(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn sub(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn mul(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn div(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn rem(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct special_files {
    pub outf: output,
    pub pfp: *mut *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_label {
    pub v_index: countT,
    pub name: *mut i8,
    pub err_info: error_info,
    pub next: *mut sed_label,
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
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x10 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
static mut my_stdin: *mut FILE = 0 as *const FILE as *mut FILE;
static mut my_stdout: *mut FILE = 0 as *const FILE as *mut FILE;
static mut my_stderr: *mut FILE = 0 as *const FILE as *mut FILE;
static mut special_files: [special_files; 4] = unsafe {
    [
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stdin\0" as *const u8 as *const i8 as *mut i8,
                        missing_newline: 0 as i32 != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stdin as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stdout\0" as *const u8 as *const i8 as *mut i8,
                        missing_newline: 0 as i32 != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stdout as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: b"/dev/stderr\0" as *const u8 as *const i8 as *mut i8,
                        missing_newline: 0 as i32 != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: &my_stderr as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
        {
            let mut init = special_files {
                outf: {
                    let mut init = output {
                        name: 0 as *const i8 as *mut i8,
                        missing_newline: 0 as i32 != 0,
                        fp: 0 as *const FILE as *mut FILE,
                        link: 0 as *const output as *mut output,
                    };
                    init
                },
                pfp: 0 as *const *mut FILE as *mut *mut FILE,
            };
            init
        },
    ]
};
static mut prog: prog_info = prog_info {
    base: 0 as *const u8,
    cur: 0 as *const u8,
    end: 0 as *const u8,
    file: 0 as *const FILE as *mut FILE,
};
static mut cur_input: error_info = error_info {
    name: 0 as *const i8,
    line: 0,
    string_expr_count: 0,
};
static mut jumps: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut labels: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut first_script: bool = 1 as i32 != 0;
static mut pending_text: *mut buffer = 0 as *const buffer as *mut buffer;
static mut old_text_buf: *mut text_buf = 0 as *const text_buf as *mut text_buf;
static mut blocks: *mut sed_label = 0 as *const sed_label as *mut sed_label;
static mut obs: obstack = obstack {
    chunk_size: 0,
    chunk: 0 as *const _obstack_chunk as *mut _obstack_chunk,
    object_base: 0 as *const i8 as *mut i8,
    next_free: 0 as *const i8 as *mut i8,
    chunk_limit: 0 as *const i8 as *mut i8,
    temp: C2RustUnnamed_2 { i: 0 },
    alignment_mask: 0,
    chunkfun: C2RustUnnamed_1 { plain: None },
    freefun: C2RustUnnamed_0 { plain: None },
    extra_arg: 0 as *const libc::c_void as *mut libc::c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
    c2rust_padding: [0; 7],
};
static mut errors: [i8; 897] = unsafe {
    *::core::mem::transmute::<
        &[u8; 897],
        &[i8; 897],
    >(
        b"multiple `!'s\0unexpected `,'\0invalid usage of +N or ~N as first address\0unmatched `{'\0unexpected `}'\0extra characters after command\0expected \\ after `a', `c' or `i'\0`}' doesn't want any addresses\0: doesn't want any addresses\0comments don't accept any addresses\0missing command\0command only uses one address\0unterminated address regex\0unterminated `s' command\0unterminated `y' command\0unknown option to `s'\0multiple `p' options to `s' command\0multiple `g' options to `s' command\0multiple number options to `s' command\0number option to `s' command may not be zero\0strings for `y' command are different lengths\0delimiter character is not a single-byte character\0expected newer version of sed\0invalid usage of line address 0\0unknown command: `%c'\0incomplete command\0\":\" lacks a label\0recursive escaping after \\c not allowed\0e/r/w commands disabled in sandbox mode\0missing filename in r/R/w/W commands\0",
    )
};
static mut file_read: *mut output = 0 as *const output as *mut output;
static mut file_write: *mut output = 0 as *const output as *mut output;
unsafe extern "C" fn bad_command(mut ch: i8) {
    let mut msg: *const i8 = dcgettext(
        0 as *const i8,
        errors
            .as_ptr()
            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
            .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize),
        5 as i32,
    );
    let mut unknown_cmd: *mut i8 = xmalloc(strlen(msg)) as *mut i8;
    sprintf(unknown_cmd, msg, ch as i32);
    bad_prog(unknown_cmd);
}
#[no_mangle]
pub unsafe extern "C" fn bad_prog(mut why: *const i8) {
    if !(cur_input.name).is_null() {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: file %s line %lu: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
            cur_input.name,
            cur_input.line,
            why,
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"%s: -e expression #%lu, char %lu: %s\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
            program_name,
            cur_input.string_expr_count,
            (prog.cur).offset_from(prog.base) as i64 as u64,
            why,
        );
    }
    exit(exit_codes::EXIT_BAD_USAGE as i32);
}
unsafe extern "C" fn inchar() -> i32 {
    let mut ch: i32 = -(1 as i32);
    if !(prog.cur).is_null() {
        if prog.cur < prog.end {
            let fresh1 = prog.cur;
            prog.cur = (prog.cur).offset(1);
            ch = *fresh1 as i32;
        }
    } else if !(prog.file).is_null() {
        if feof_unlocked(prog.file) == 0 {
            ch = getc_unlocked(prog.file);
        }
    }
    if ch == '\n' as i32 {
        cur_input.line = (cur_input.line).wrapping_add(1);
        cur_input.line;
    }
    return ch;
}
unsafe extern "C" fn savchar(mut ch: i32) {
    if ch == -(1 as i32) {
        return;
    }
    if ch == '\n' as i32 && cur_input.line > 0 as i32 as u64 {
        cur_input.line = (cur_input.line).wrapping_sub(1);
        cur_input.line;
    }
    if !(prog.cur).is_null() {
        if prog.cur <= prog.base
            || {
                prog.cur = (prog.cur).offset(-1);
                *prog.cur as i32 != ch
            }
        {
            panic(
                b"Called savchar with unexpected pushback (%x)\0" as *const u8
                    as *const i8,
                ch as u32,
            );
        }
    } else {
        ungetc(ch, prog.file);
    };
}
unsafe extern "C" fn in_nonblank() -> i32 {
    let mut ch: i32 = 0;
    loop {
        ch = inchar();
        if !(1 as i32 != 0
            && *(*__ctype_b_loc()).offset(ch as isize) as i32
                & C2RustUnnamed_4::_ISblank as i32 as libc::c_ushort as i32 != 0)
        {
            break;
        }
    }
    return ch;
}
unsafe extern "C" fn read_end_of_cmd() {
    let ch: i32 = in_nonblank();
    if ch == '}' as i32 || ch == '#' as i32 {
        savchar(ch);
    } else if ch != -(1 as i32) && ch != '\n' as i32 && ch != ';' as i32 {
        bad_prog(
            dcgettext(
                0 as *const i8,
                errors
                    .as_ptr()
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize),
                5 as i32,
            ),
        );
    }
}
unsafe extern "C" fn in_integer(mut ch: i32) -> countT {
    let mut num: countT = 0 as i32 as countT;
    while 1 as i32 != 0
        && *(*__ctype_b_loc()).offset(ch as u8 as i32 as isize) as i32
            & C2RustUnnamed_4::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        num = num
            .wrapping_mul(10 as i32 as u64)
            .wrapping_add(ch as u64)
            .wrapping_sub('0' as i32 as u64);
        ch = inchar();
    }
    savchar(ch);
    return num;
}
unsafe extern "C" fn add_then_next(mut b: *mut buffer, mut ch: i32) -> i32 {
    add1_buffer(b, ch);
    return inchar();
}
unsafe extern "C" fn convert_number(
    mut result: *mut i8,
    mut buf: *mut i8,
    mut bufend: *const i8,
    mut base: i32,
) -> *mut i8 {
    let mut n: i32 = 0 as i32;
    let mut max: i32 = 1 as i32;
    let mut p: *mut i8 = 0 as *mut i8;
    p = buf.offset(1 as i32 as isize);
    while p < bufend as *mut i8 && max <= 255 as i32 {
        let mut d: i32 = -(1 as i32);
        match *p as i32 {
            48 => {
                d = 0 as i32;
            }
            49 => {
                d = 0x1 as i32;
            }
            50 => {
                d = 0x2 as i32;
            }
            51 => {
                d = 0x3 as i32;
            }
            52 => {
                d = 0x4 as i32;
            }
            53 => {
                d = 0x5 as i32;
            }
            54 => {
                d = 0x6 as i32;
            }
            55 => {
                d = 0x7 as i32;
            }
            56 => {
                d = 0x8 as i32;
            }
            57 => {
                d = 0x9 as i32;
            }
            65 | 97 => {
                d = 0xa as i32;
            }
            66 | 98 => {
                d = 0xb as i32;
            }
            67 | 99 => {
                d = 0xc as i32;
            }
            68 | 100 => {
                d = 0xd as i32;
            }
            69 | 101 => {
                d = 0xe as i32;
            }
            70 | 102 => {
                d = 0xf as i32;
            }
            _ => {}
        }
        if d < 0 as i32 || base <= d {
            break;
        }
        n = n * base + d;
        p = p.offset(1);
        p;
        max *= base;
    }
    if p == buf.offset(1 as i32 as isize) {
        *result = *buf;
    } else {
        *result = n as i8;
    }
    return p;
}
unsafe extern "C" fn read_filename() -> *mut buffer {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: i32 = 0;
    if sandbox {
        bad_prog(
            dcgettext(
                0 as *const i8,
                errors
                    .as_ptr()
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize),
                5 as i32,
            ),
        );
    }
    b = init_buffer();
    ch = in_nonblank();
    while ch != -(1 as i32) && ch != '\n' as i32 {
        ch = add_then_next(b, ch);
    }
    add1_buffer(b, '\0' as i32);
    return b;
}
unsafe extern "C" fn get_openfile(
    mut file_ptrs: *mut *mut output,
    mut mode: *const i8,
    mut fail: i32,
) -> *mut output {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut file_name: *mut i8 = 0 as *mut i8;
    let mut p: *mut output = 0 as *mut output;
    b = read_filename();
    file_name = get_buffer(b);
    if strlen(file_name) == 0 as i32 as u64 {
        bad_prog(
            dcgettext(
                0 as *const i8,
                errors
                    .as_ptr()
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize),
                5 as i32,
            ),
        );
    }
    p = *file_ptrs;
    while !p.is_null() {
        if strcmp((*p).name, file_name) == 0 as i32 {
            break;
        }
        p = (*p).link;
    }
    if posixicity as u32 == posixicity_types::POSIXLY_EXTENDED as i32 as u32 {
        let mut special: *mut special_files = special_files.as_mut_ptr();
        my_stdin = stdin;
        my_stdout = stdout;
        my_stderr = stderr;
        special = special_files.as_mut_ptr();
        while !((*special).outf.name).is_null() {
            if strcmp((*special).outf.name, file_name) == 0 as i32 {
                (*special).outf.fp = *(*special).pfp;
                free_buffer(b);
                return &mut (*special).outf;
            }
            special = special.offset(1);
            special;
        }
    }
    if p.is_null() {
        p = ({
            let mut __h: *mut obstack = &mut obs as *mut obstack;
            let mut __o: *mut obstack = __h;
            let mut __len: size_t = (1 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<output>() as u64);
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
            ({
                let mut __o1: *mut obstack = __h;
                let mut __value: *mut libc::c_void = (*__o1).object_base
                    as *mut libc::c_void;
                if (*__o1).next_free == __value as *mut i8 {
                    (*__o1).set_maybe_empty_object(1 as i32 as u32);
                }
                (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                {
                    (*__o1).object_base
                } else {
                    0 as *mut i8
                })
                    .offset(
                        ((((*__o1).next_free)
                            .offset_from(
                                (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                    < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                {
                                    (*__o1).object_base
                                } else {
                                    0 as *mut i8
                                }),
                            ) as i64 as u64)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
                    > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                        as size_t
                {
                    (*__o1).next_free = (*__o1).chunk_limit;
                }
                (*__o1).object_base = (*__o1).next_free;
                __value
            })
        }) as *mut output;
        (*p).name = xstrdup(file_name);
        (*p).fp = ck_fopen((*p).name, mode, fail);
        (*p).missing_newline = 0 as i32 != 0;
        (*p).link = *file_ptrs;
        *file_ptrs = p;
    }
    free_buffer(b);
    return p;
}
unsafe extern "C" fn next_cmd_entry(mut vectorp: *mut *mut vector) -> *mut sed_cmd {
    let mut cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut v: *mut vector = 0 as *mut vector;
    v = *vectorp;
    if (*v).v_length == (*v).v_allocated {
        (*v).v_allocated = ((*v).v_allocated as u64).wrapping_add(40 as i32 as u64)
            as size_t as size_t;
        (*v).v = xnrealloc(
            (*v).v as *mut libc::c_void,
            (*v).v_allocated,
            ::core::mem::size_of::<sed_cmd>() as u64,
        ) as *mut sed_cmd;
    }
    cmd = ((*v).v).offset((*v).v_length as isize);
    (*cmd).a1 = 0 as *mut addr;
    (*cmd).a2 = 0 as *mut addr;
    (*cmd).range_state = addr_state::RANGE_INACTIVE;
    (*cmd).addr_bang = 0 as i32 as i8;
    (*cmd).cmd = '\0' as i32 as i8;
    *vectorp = v;
    return cmd;
}
unsafe extern "C" fn snarf_char_class(
    mut b: *mut buffer,
    mut cur_stat: *mut mbstate_t,
) -> i32 {
    let mut ch: i32 = 0;
    let mut state: i32 = 0 as i32;
    let mut delim: i32 = 0;
    ch = inchar();
    if ch == '^' as i32 {
        ch = add_then_next(b, ch);
    }
    if ch == ']' as i32 {
        ch = add_then_next(b, ch);
    }
    let mut current_block_16: u64;
    loop {
        let mb_char: i32 = if mb_cur_max == 1 as i32 {
            0 as i32
        } else {
            is_mb_char(ch, cur_stat)
        };
        match ch {
            -1 | 10 => return ch,
            46 | 58 | 61 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else if state == 1 as i32 {
                    delim = ch;
                    state = 2 as i32;
                    current_block_16 = 15427931788582360902;
                } else if state == 2 as i32 && ch == delim {
                    state = 3 as i32;
                    current_block_16 = 15427931788582360902;
                } else {
                    current_block_16 = 2719512138335094285;
                }
            }
            91 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else {
                    if state == 0 as i32 {
                        state = 1 as i32;
                    }
                    current_block_16 = 15427931788582360902;
                }
            }
            93 => {
                if mb_char != 0 {
                    current_block_16 = 15427931788582360902;
                } else {
                    if state == 0 as i32 || state == 1 as i32 {
                        return ch
                    } else if state == 3 as i32 {
                        state = 0 as i32;
                    }
                    current_block_16 = 2719512138335094285;
                }
            }
            _ => {
                current_block_16 = 2719512138335094285;
            }
        }
        match current_block_16 {
            2719512138335094285 => {
                state &= !(1 as i32);
            }
            _ => {}
        }
        ch = add_then_next(b, ch);
    };
}
unsafe extern "C" fn match_slash(mut slash: i32, mut regex: i32) -> *mut buffer {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: i32 = 0;
    let mut cur_stat: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    if if mb_cur_max == 1 as i32 { 0 as i32 } else { is_mb_char(slash, &mut cur_stat) }
        != 0
    {
        bad_prog(
            errors
                .as_ptr()
                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize),
        );
    }
    memset(
        &mut cur_stat as *mut mbstate_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    b = init_buffer();
    loop {
        ch = inchar();
        if !(ch != -(1 as i32) && ch != '\n' as i32) {
            break;
        }
        let mb_char: i32 = if mb_cur_max == 1 as i32 {
            0 as i32
        } else {
            is_mb_char(ch, &mut cur_stat)
        };
        if mb_char == 0 {
            if ch == slash {
                return b
            } else if ch == '\\' as i32 {
                ch = inchar();
                if ch == -(1 as i32) {
                    break;
                }
                if ch != '\n' as i32 && (ch != slash || regex == 0 && ch == '&' as i32) {
                    add1_buffer(b, '\\' as i32);
                }
            } else if ch == '[' as i32 && regex != 0 {
                add1_buffer(b, ch);
                ch = snarf_char_class(b, &mut cur_stat);
                if ch != ']' as i32 {
                    break;
                }
            }
        }
        add1_buffer(b, ch);
    }
    if ch == '\n' as i32 {
        savchar(ch);
    }
    free_buffer(b);
    return 0 as *mut buffer;
}
unsafe extern "C" fn mark_subst_opts(mut cmd: *mut subst) -> i32 {
    let mut flags: i32 = 0 as i32;
    let mut ch: i32 = 0;
    (*cmd).set_global(0 as i32 as u32);
    (*cmd).set_print(0 as i32 as u32);
    (*cmd).set_eval(0 as i32 as u32);
    (*cmd).numb = 0 as i32 as countT;
    (*cmd).outf = 0 as *mut output;
    loop {
        let mut current_block_33: u64;
        ch = in_nonblank();
        match ch {
            105 => {
                current_block_33 = 10265717308518604954;
            }
            73 => {
                current_block_33 = 10265717308518604954;
            }
            109 | 77 => {
                if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                flags |= (1 as i32) << 2 as i32;
                current_block_33 = 4090602189656566074;
            }
            101 => {
                if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cmd).set_eval(1 as i32 as u32);
                current_block_33 = 4090602189656566074;
            }
            112 => {
                if (*cmd).print() != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cmd)
                    .set_print(
                        (*cmd).print() | ((1 as i32) << (*cmd).eval() as i32) as u32,
                    );
                current_block_33 = 4090602189656566074;
            }
            103 => {
                if (*cmd).global() != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cmd).set_global(1 as i32 as u32);
                current_block_33 = 4090602189656566074;
            }
            119 => {
                (*cmd).outf = get_openfile(&mut file_write, write_mode, 1 as i32);
                return flags;
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if (*cmd).numb != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cmd).numb = in_integer(ch);
                if (*cmd).numb == 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                current_block_33 = 4090602189656566074;
            }
            125 | 35 => {
                savchar(ch);
                current_block_33 = 18134060836872509562;
            }
            -1 | 10 | 59 => {
                current_block_33 = 18134060836872509562;
            }
            13 => {
                if inchar() == '\n' as i32 {
                    return flags;
                }
                current_block_33 = 2028025524609176363;
            }
            _ => {
                current_block_33 = 2028025524609176363;
            }
        }
        match current_block_33 {
            10265717308518604954 => {
                if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                flags |= (1 as i32) << 1 as i32;
            }
            18134060836872509562 => return flags,
            2028025524609176363 => {
                bad_prog(
                    dcgettext(
                        0 as *const i8,
                        errors
                            .as_ptr()
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                        5 as i32,
                    ),
                );
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn read_label() -> *mut i8 {
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: i32 = 0;
    let mut ret: *mut i8 = 0 as *mut i8;
    b = init_buffer();
    ch = in_nonblank();
    while ch != -(1 as i32) && ch != '\n' as i32
        && !(1 as i32 != 0
            && *(*__ctype_b_loc()).offset(ch as isize) as i32
                & C2RustUnnamed_4::_ISblank as i32 as libc::c_ushort as i32 != 0)
        && ch != ';' as i32 && ch != '}' as i32 && ch != '#' as i32
    {
        ch = add_then_next(b, ch);
    }
    savchar(ch);
    add1_buffer(b, '\0' as i32);
    ret = xstrdup(get_buffer(b));
    free_buffer(b);
    return ret;
}
unsafe extern "C" fn setup_label(
    mut list: *mut sed_label,
    mut idx: countT,
    mut name: *mut i8,
    mut err_info: *const error_info,
) -> *mut sed_label {
    let mut ret: *mut sed_label = ({
        let mut __h: *mut obstack = &mut obs as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = (1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<sed_label>() as u64);
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    }) as *mut sed_label;
    (*ret).v_index = idx;
    (*ret).name = name;
    if !err_info.is_null() {
        memcpy(
            &mut (*ret).err_info as *mut error_info as *mut libc::c_void,
            err_info as *const libc::c_void,
            ::core::mem::size_of::<error_info>() as u64,
        );
    }
    (*ret).next = list;
    return ret;
}
unsafe extern "C" fn release_label(mut list_head: *mut sed_label) -> *mut sed_label {
    let mut ret: *mut sed_label = 0 as *mut sed_label;
    if list_head.is_null() {
        return 0 as *mut sed_label;
    }
    ret = (*list_head).next;
    rpl_free((*list_head).name as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn new_replacement(
    mut text: *mut i8,
    mut length: size_t,
    mut type_0: replacement_types,
) -> *mut replacement {
    let mut r: *mut replacement = ({
        let mut __h: *mut obstack = &mut obs as *mut obstack;
        let mut __o: *mut obstack = __h;
        let mut __len: size_t = (1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<replacement>() as u64);
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        ({
            let mut __o1: *mut obstack = __h;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut i8 {
                (*__o1).set_maybe_empty_object(1 as i32 as u32);
            }
            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                < ::core::mem::size_of::<*mut libc::c_void>() as u64
            {
                (*__o1).object_base
            } else {
                0 as *mut i8
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            }),
                        ) as i64 as u64)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        })
    }) as *mut replacement;
    (*r).prefix = text;
    (*r).prefix_length = length;
    (*r).subst_id = -(1 as i32);
    (*r).repl_type = type_0;
    return r;
}
unsafe extern "C" fn setup_replacement(
    mut sub: *mut subst,
    mut text: *const i8,
    mut length: size_t,
) {
    let mut base: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut text_end: *mut i8 = 0 as *mut i8;
    let mut repl_type: replacement_types = replacement_types::REPL_ASIS;
    let mut save_type: replacement_types = replacement_types::REPL_ASIS;
    let mut root: replacement = replacement {
        prefix: 0 as *mut i8,
        prefix_length: 0,
        subst_id: 0,
        repl_type: replacement_types::REPL_ASIS,
        next: 0 as *mut replacement,
    };
    let mut tail: *mut replacement = 0 as *mut replacement;
    (*sub).set_max_id(0 as i32 as u32);
    base = xmemdup(
        text as *const libc::c_void,
        length.wrapping_mul(::core::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    length = normalize_text(base, length, text_types::TEXT_REPLACEMENT);
    text_end = base.offset(length as isize);
    tail = &mut root;
    p = base;
    while p < text_end {
        if *p as i32 == '\\' as i32 {
            (*tail).next = new_replacement(
                base,
                p.offset_from(base) as i64 as size_t,
                repl_type,
            );
            tail = (*tail).next;
            repl_type = save_type;
            p = p.offset(1);
            p;
            if p == text_end {
                (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                (*tail).prefix_length;
            } else if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32
                && !(1 as i32 != 0
                    && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                        & C2RustUnnamed_4::_ISdigit as i32 as libc::c_ushort as i32 != 0)
            {
                *p.offset(-(1 as i32) as isize) = *p;
                (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                (*tail).prefix_length;
            } else {
                match *p as i32 {
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        (*tail).subst_id = *p as i32 - '0' as i32;
                        if ((*sub).max_id() as i32) < (*tail).subst_id {
                            (*sub).set_max_id((*tail).subst_id as u32);
                        }
                    }
                    76 => {
                        repl_type = replacement_types::REPL_LOWERCASE;
                        save_type = replacement_types::REPL_LOWERCASE;
                    }
                    85 => {
                        repl_type = replacement_types::REPL_UPPERCASE;
                        save_type = replacement_types::REPL_UPPERCASE;
                    }
                    69 => {
                        repl_type = replacement_types::REPL_ASIS;
                        save_type = replacement_types::REPL_ASIS;
                    }
                    108 => {
                        save_type = repl_type;
                        repl_type = ::core::mem::transmute::<
                            u32,
                            replacement_types,
                        >(
                            repl_type as u32
                                | replacement_types::REPL_LOWERCASE_FIRST as i32 as u32,
                        );
                    }
                    117 => {
                        save_type = repl_type;
                        repl_type = ::core::mem::transmute::<
                            u32,
                            replacement_types,
                        >(
                            repl_type as u32
                                | replacement_types::REPL_UPPERCASE_FIRST as i32 as u32,
                        );
                    }
                    _ => {
                        *p.offset(-(1 as i32) as isize) = *p;
                        (*tail).prefix_length = ((*tail).prefix_length).wrapping_add(1);
                        (*tail).prefix_length;
                    }
                }
            }
            base = p.offset(1 as i32 as isize);
        } else if *p as i32 == '&' as i32 {
            (*tail).next = new_replacement(
                base,
                p.offset_from(base) as i64 as size_t,
                repl_type,
            );
            tail = (*tail).next;
            repl_type = save_type;
            (*tail).subst_id = 0 as i32;
            base = p.offset(1 as i32 as isize);
        }
        p = p.offset(1);
        p;
    }
    if base < text_end {
        (*tail).next = new_replacement(
            base,
            text_end.offset_from(base) as i64 as size_t,
            repl_type,
        );
        tail = (*tail).next;
    }
    (*tail).next = 0 as *mut replacement;
    (*sub).replacement = root.next;
}
unsafe extern "C" fn read_text(mut buf: *mut text_buf, mut leadin_ch: i32) {
    let mut ch: i32 = 0;
    if !buf.is_null() {
        if !pending_text.is_null() {
            free_buffer(pending_text);
        }
        pending_text = init_buffer();
        (*buf).text = 0 as *mut i8;
        (*buf).text_length = 0 as i32 as size_t;
        old_text_buf = buf;
    }
    if leadin_ch == -(1 as i32) {
        return;
    }
    if leadin_ch != '\n' as i32 {
        add1_buffer(pending_text, leadin_ch);
    }
    ch = inchar();
    while ch != -(1 as i32) && ch != '\n' as i32 {
        if ch == '\\' as i32 {
            ch = inchar();
            if ch != -(1 as i32) {
                add1_buffer(pending_text, '\\' as i32);
            }
        }
        if ch == -(1 as i32) {
            add1_buffer(pending_text, '\n' as i32);
            return;
        }
        ch = add_then_next(pending_text, ch);
    }
    add1_buffer(pending_text, '\n' as i32);
    if buf.is_null() {
        buf = old_text_buf;
    }
    (*buf).text_length = normalize_text(
        get_buffer(pending_text),
        size_buffer(pending_text),
        text_types::TEXT_BUFFER,
    );
    (*buf).text = xmemdup(
        get_buffer(pending_text) as *const libc::c_void,
        ((*buf).text_length).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    free_buffer(pending_text);
    pending_text = 0 as *mut buffer;
}
unsafe extern "C" fn compile_address(mut addr: *mut addr, mut ch: i32) -> bool {
    (*addr).addr_type = addr_types::ADDR_IS_NULL;
    (*addr).addr_step = 0 as i32 as countT;
    (*addr).addr_number = !(0 as i32 as countT);
    (*addr).addr_regex = 0 as *mut regex;
    if ch == '/' as i32 || ch == '\\' as i32 {
        let mut flags: i32 = 0 as i32;
        let mut b: *mut buffer = 0 as *mut buffer;
        (*addr).addr_type = addr_types::ADDR_IS_REGEX;
        if ch == '\\' as i32 {
            ch = inchar();
        }
        b = match_slash(ch, 1 as i32);
        if b.is_null() {
            bad_prog(
                dcgettext(
                    0 as *const i8,
                    errors
                        .as_ptr()
                        .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                        .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize),
                    5 as i32,
                ),
            );
        }
        loop {
            's_90: {
                let mut current_block_15: u64;
                ch = in_nonblank();
                if !(posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32)
                {
                    match ch {
                        73 => {
                            current_block_15 = 8566680111121466844;
                            match current_block_15 {
                                1993566170996213625 => {
                                    flags |= (1 as i32) << 2 as i32;
                                }
                                _ => {
                                    flags |= (1 as i32) << 1 as i32;
                                }
                            }
                            break 's_90;
                        }
                        77 => {
                            current_block_15 = 1993566170996213625;
                            match current_block_15 {
                                1993566170996213625 => {
                                    flags |= (1 as i32) << 2 as i32;
                                }
                                _ => {
                                    flags |= (1 as i32) << 1 as i32;
                                }
                            }
                            break 's_90;
                        }
                        _ => {}
                    }
                }
                savchar(ch);
                (*addr).addr_regex = compile_regex(b, flags, 0 as i32);
                free_buffer(b);
                return 1 as i32 != 0;
            }
        }
    } else if 1 as i32 != 0
        && *(*__ctype_b_loc()).offset(ch as u8 as i32 as isize) as i32
            & C2RustUnnamed_4::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        (*addr).addr_number = in_integer(ch);
        (*addr).addr_type = addr_types::ADDR_IS_NUM;
        ch = in_nonblank();
        if ch != '~' as i32
            || posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32
        {
            savchar(ch);
        } else {
            let mut step: countT = in_integer(in_nonblank());
            if step > 0 as i32 as u64 {
                (*addr).addr_step = step;
                (*addr).addr_type = addr_types::ADDR_IS_NUM_MOD;
            }
        }
    } else if (ch == '+' as i32 || ch == '~' as i32)
        && posixicity as u32 != posixicity_types::POSIXLY_BASIC as i32 as u32
    {
        (*addr).addr_step = in_integer(in_nonblank());
        if !((*addr).addr_step == 0 as i32 as u64) {
            if ch == '+' as i32 {
                (*addr).addr_type = addr_types::ADDR_IS_STEP;
            } else {
                (*addr).addr_type = addr_types::ADDR_IS_STEP_MOD;
            }
        }
    } else if ch == '$' as i32 {
        (*addr).addr_type = addr_types::ADDR_IS_LAST;
    } else {
        return 0 as i32 != 0
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn compile_program(mut vector: *mut vector) -> *mut vector {
    let mut cur_cmd: *mut sed_cmd = 0 as *mut sed_cmd;
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut ch: i32 = 0;
    if vector.is_null() {
        vector = (if ::core::mem::size_of::<vector>() as u64 == 1 as i32 as u64 {
            xzalloc(1 as i32 as size_t)
        } else {
            xcalloc(1 as i32 as size_t, ::core::mem::size_of::<vector>() as u64)
        }) as *mut vector;
        (*vector).v = 0 as *mut sed_cmd;
        (*vector).v_allocated = 0 as i32 as size_t;
        (*vector).v_length = 0 as i32 as size_t;
        _obstack_begin(
            &mut obs,
            0 as i32 as size_t,
            0 as i32 as size_t,
            Some(xzalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    if !pending_text.is_null() {
        read_text(0 as *mut text_buf, '\n' as i32);
    }
    let mut current_block_184: u64;
    loop {
        let mut a: addr = addr {
            addr_type: addr_types::ADDR_IS_NULL,
            addr_number: 0,
            addr_step: 0,
            addr_regex: 0 as *mut regex,
        };
        loop {
            ch = inchar();
            if !(ch == ';' as i32
                || 1 as i32 != 0
                    && *(*__ctype_b_loc()).offset(ch as isize) as i32
                        & C2RustUnnamed_4::_ISspace as i32 as libc::c_ushort as i32 != 0)
            {
                break;
            }
        }
        if ch == -(1 as i32) {
            break;
        }
        cur_cmd = next_cmd_entry(&mut vector);
        if compile_address(&mut a, ch) {
            if a.addr_type as u32 == addr_types::ADDR_IS_STEP as i32 as u32
                || a.addr_type as u32 == addr_types::ADDR_IS_STEP_MOD as i32 as u32
            {
                bad_prog(
                    dcgettext(
                        0 as *const i8,
                        errors
                            .as_ptr()
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize),
                        5 as i32,
                    ),
                );
            }
            (*cur_cmd).a1 = xmemdup(
                &mut a as *mut addr as *const libc::c_void,
                (1 as i32 as u64).wrapping_mul(::core::mem::size_of::<addr>() as u64),
            ) as *mut addr;
            ch = in_nonblank();
            if ch == ',' as i32 {
                if !compile_address(&mut a, in_nonblank()) {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cur_cmd).a2 = xmemdup(
                    &mut a as *mut addr as *const libc::c_void,
                    (1 as i32 as u64).wrapping_mul(::core::mem::size_of::<addr>() as u64),
                ) as *mut addr;
                ch = in_nonblank();
            }
            if (*(*cur_cmd).a1).addr_type as u32 == addr_types::ADDR_IS_NUM as i32 as u32
                && (*(*cur_cmd).a1).addr_number == 0 as i32 as u64
                && (((*cur_cmd).a2).is_null() && ch != 'r' as i32
                    || !((*cur_cmd).a2).is_null()
                        && (*(*cur_cmd).a2).addr_type as u32
                            != addr_types::ADDR_IS_REGEX as i32 as u32
                    || posixicity as u32
                        == posixicity_types::POSIXLY_BASIC as i32 as u32)
            {
                bad_prog(
                    dcgettext(
                        0 as *const i8,
                        errors
                            .as_ptr()
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize),
                        5 as i32,
                    ),
                );
            }
        }
        if ch == '!' as i32 {
            (*cur_cmd).addr_bang = 1 as i32 as i8;
            ch = in_nonblank();
            if ch == '!' as i32 {
                bad_prog(dcgettext(0 as *const i8, errors.as_ptr(), 5 as i32));
            }
        }
        if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32 {
            let mut current_block_35: u64;
            match ch {
                101 | 70 | 118 | 122 | 76 | 81 | 84 | 82 | 87 => {
                    bad_command(ch as i8);
                    current_block_35 = 12903280044251176665;
                }
                97 | 105 | 108 | 61 | 114 => {
                    current_block_35 = 12903280044251176665;
                }
                _ => {
                    current_block_35 = 14832935472441733737;
                }
            }
            match current_block_35 {
                12903280044251176665 => {
                    if !((*cur_cmd).a2).is_null() {
                        bad_prog(
                            dcgettext(
                                0 as *const i8,
                                errors
                                    .as_ptr()
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize),
                                5 as i32,
                            ),
                        );
                    }
                }
                _ => {}
            }
        }
        (*cur_cmd).cmd = ch as i8;
        match ch {
            35 => {
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                ch = inchar();
                if ch == 'n' as i32 && first_script as i32 != 0
                    && cur_input.line < 2 as i32 as u64
                {
                    if !(prog.base).is_null()
                        && prog.cur == (prog.base).offset(2 as i32 as isize)
                        || !(prog.file).is_null() && (prog.base).is_null()
                            && 2 as i32 as i64 == ftell(prog.file)
                    {
                        no_default_output = 1 as i32 != 0;
                    }
                }
                while ch != -(1 as i32) && ch != '\n' as i32 {
                    ch = inchar();
                }
                continue;
            }
            118 => {
                let mut version: *mut i8 = read_label();
                let mut compared_version: *const i8 = 0 as *const i8;
                compared_version = if *version as i32 == '\0' as i32 {
                    b"4.0\0" as *const u8 as *const i8
                } else {
                    version
                };
                if strverscmp(compared_version, b"4.9\0" as *const u8 as *const i8)
                    > 0 as i32
                {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                rpl_free(version as *mut libc::c_void);
                posixicity = posixicity_types::POSIXLY_EXTENDED;
                continue;
            }
            123 => {
                blocks = setup_label(
                    blocks,
                    (*vector).v_length,
                    0 as *mut i8,
                    &mut cur_input,
                );
                (*cur_cmd).addr_bang = ((*cur_cmd).addr_bang == 0) as i32 as i8;
                current_block_184 = 6957654774345280688;
            }
            125 => {
                if blocks.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                read_end_of_cmd();
                (*((*vector).v).offset((*blocks).v_index as isize)).x.jump_index = (*vector)
                    .v_length;
                blocks = release_label(blocks);
                current_block_184 = 6957654774345280688;
            }
            101 => {
                if sandbox {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                ch = in_nonblank();
                if ch == -(1 as i32) || ch == '\n' as i32 {
                    (*cur_cmd).x.cmd_txt.text_length = 0 as i32 as size_t;
                    current_block_184 = 6957654774345280688;
                } else {
                    current_block_184 = 12666756727116774827;
                }
            }
            97 | 105 | 99 => {
                ch = in_nonblank();
                current_block_184 = 12666756727116774827;
            }
            58 => {
                if !((*cur_cmd).a1).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                let mut label: *mut i8 = read_label();
                if *label == 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                labels = setup_label(
                    labels,
                    (*vector).v_length,
                    label,
                    0 as *const error_info,
                );
                if debug {
                    (*cur_cmd).x.label_name = strdup(label);
                }
                current_block_184 = 6957654774345280688;
            }
            84 | 98 | 116 => {
                jumps = setup_label(
                    jumps,
                    (*vector).v_length,
                    read_label(),
                    0 as *const error_info,
                );
                current_block_184 = 6957654774345280688;
            }
            81 | 113 => {
                if !((*cur_cmd).a2).is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                current_block_184 = 12491576746588794161;
            }
            76 | 108 => {
                current_block_184 = 12491576746588794161;
            }
            61 | 100 | 68 | 70 | 103 | 71 | 104 | 72 | 110 | 78 | 112 | 80 | 122
            | 120 => {
                read_end_of_cmd();
                current_block_184 = 6957654774345280688;
            }
            114 => {
                b = read_filename();
                if strlen(get_buffer(b)) == 0 as i32 as u64 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cur_cmd).x.readcmd.fname = xstrdup(get_buffer(b));
                if !((*cur_cmd).a1).is_null()
                    && (*(*cur_cmd).a1).addr_type as u32
                        == addr_types::ADDR_IS_NUM as i32 as u32
                    && (*(*cur_cmd).a1).addr_number == 0 as i32 as u64
                    && ((*cur_cmd).a2).is_null()
                {
                    (*(*cur_cmd).a1).addr_number = 1 as i32 as countT;
                    (*cur_cmd).x.readcmd.append = 0 as i32 != 0;
                } else {
                    (*cur_cmd).x.readcmd.append = 1 as i32 != 0;
                }
                free_buffer(b);
                current_block_184 = 6957654774345280688;
            }
            82 => {
                (*cur_cmd).x.inf = get_openfile(&mut file_read, read_mode, 0 as i32);
                current_block_184 = 6957654774345280688;
            }
            87 | 119 => {
                (*cur_cmd).x.outf = get_openfile(&mut file_write, write_mode, 1 as i32);
                current_block_184 = 6957654774345280688;
            }
            115 => {
                let mut b2: *mut buffer = 0 as *mut buffer;
                let mut flags: i32 = 0;
                let mut slash: i32 = 0;
                slash = inchar();
                b = match_slash(slash, 1 as i32);
                if b.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                b2 = match_slash(slash, 0 as i32);
                if b2.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                (*cur_cmd).x.cmd_subst = ({
                    let mut __h: *mut obstack = &mut obs as *mut obstack;
                    let mut __o: *mut obstack = __h;
                    let mut __len: size_t = (1 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<subst>() as u64);
                    if ({
                        let mut __o1: *const obstack = __o;
                        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64
                            as size_t
                    }) < __len
                    {
                        _obstack_newchunk(__o, __len);
                    }
                    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                    ({
                        let mut __o1: *mut obstack = __h;
                        let mut __value: *mut libc::c_void = (*__o1).object_base
                            as *mut libc::c_void;
                        if (*__o1).next_free == __value as *mut i8 {
                            (*__o1).set_maybe_empty_object(1 as i32 as u32);
                        }
                        (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>()
                            as u64) < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        })
                            .offset(
                                ((((*__o1).next_free)
                                    .offset_from(
                                        (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                        {
                                            (*__o1).object_base
                                        } else {
                                            0 as *mut i8
                                        }),
                                    ) as i64 as u64)
                                    .wrapping_add((*__o1).alignment_mask)
                                    & !(*__o1).alignment_mask) as isize,
                            );
                        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8)
                            as i64 as size_t
                            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8)
                                as i64 as size_t
                        {
                            (*__o1).next_free = (*__o1).chunk_limit;
                        }
                        (*__o1).object_base = (*__o1).next_free;
                        __value
                    })
                }) as *mut subst;
                setup_replacement(
                    (*cur_cmd).x.cmd_subst,
                    get_buffer(b2),
                    size_buffer(b2),
                );
                free_buffer(b2);
                flags = mark_subst_opts((*cur_cmd).x.cmd_subst);
                (*(*cur_cmd).x.cmd_subst).regx = compile_regex(
                    b,
                    flags,
                    (*(*cur_cmd).x.cmd_subst).max_id() as i32 + 1 as i32,
                );
                free_buffer(b);
                if (*(*cur_cmd).x.cmd_subst).eval() as i32 != 0 && sandbox as i32 != 0 {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 40]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                current_block_184 = 6957654774345280688;
            }
            121 => {
                let mut len: size_t = 0;
                let mut dest_len: size_t = 0;
                let mut slash_0: i32 = 0;
                let mut b2_0: *mut buffer = 0 as *mut buffer;
                let mut src_buf: *mut i8 = 0 as *mut i8;
                let mut dest_buf: *mut i8 = 0 as *mut i8;
                slash_0 = inchar();
                b = match_slash(slash_0, 0 as i32);
                if b.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                src_buf = get_buffer(b);
                len = normalize_text(src_buf, size_buffer(b), text_types::TEXT_BUFFER);
                b2_0 = match_slash(slash_0, 0 as i32);
                if b2_0.is_null() {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                dest_buf = get_buffer(b2_0);
                dest_len = normalize_text(
                    dest_buf,
                    size_buffer(b2_0),
                    text_types::TEXT_BUFFER,
                );
                if mb_cur_max > 1 as i32 {
                    let mut i: size_t = 0;
                    let mut j: size_t = 0;
                    let mut idx: size_t = 0;
                    let mut src_char_num: size_t = 0;
                    let mut src_lens: *mut size_t = (if ::core::mem::size_of::<size_t>()
                        as u64 == 1 as i32 as u64
                    {
                        xzalloc(len)
                    } else {
                        xcalloc(len, ::core::mem::size_of::<size_t>() as u64)
                    }) as *mut size_t;
                    let mut trans_pairs: *mut *mut i8 = 0 as *mut *mut i8;
                    let mut mbclen: size_t = 0;
                    let mut cur_stat: mbstate_t = {
                        let mut init = __mbstate_t {
                            __count: 0 as i32,
                            __value: C2RustUnnamed { __wch: 0 },
                        };
                        init
                    };
                    i = 0 as i32 as size_t;
                    j = 0 as i32 as size_t;
                    while i < len {
                        mbclen = if mb_cur_max == 1 as i32 {
                            1 as i32 as u64
                        } else {
                            rpl_mbrtowc(
                                0 as *mut wchar_t,
                                src_buf.offset(i as isize),
                                len.wrapping_sub(i),
                                &mut cur_stat,
                            )
                        };
                        if mbclen == -(1 as i32) as size_t
                            || mbclen == -(2 as i32) as size_t
                            || mbclen == 0 as i32 as u64
                        {
                            mbclen = 1 as i32 as size_t;
                        }
                        let fresh2 = j;
                        j = j.wrapping_add(1);
                        *src_lens.offset(fresh2 as isize) = mbclen;
                        i = (i as u64).wrapping_add(mbclen) as size_t as size_t;
                    }
                    src_char_num = j;
                    memset(
                        &mut cur_stat as *mut mbstate_t as *mut libc::c_void,
                        0 as i32,
                        ::core::mem::size_of::<mbstate_t>() as u64,
                    );
                    idx = 0 as i32 as size_t;
                    trans_pairs = (if ::core::mem::size_of::<*mut i8>() as u64
                        == 1 as i32 as u64
                    {
                        xzalloc(
                            (2 as i32 as u64)
                                .wrapping_mul(src_char_num)
                                .wrapping_add(1 as i32 as u64),
                        )
                    } else {
                        xcalloc(
                            (2 as i32 as u64)
                                .wrapping_mul(src_char_num)
                                .wrapping_add(1 as i32 as u64),
                            ::core::mem::size_of::<*mut i8>() as u64,
                        )
                    }) as *mut *mut i8;
                    (*cur_cmd).x.translatemb = trans_pairs;
                    i = 0 as i32 as size_t;
                    while i < src_char_num {
                        if idx >= dest_len {
                            bad_prog(
                                dcgettext(
                                    0 as *const i8,
                                    errors
                                        .as_ptr()
                                        .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                        .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize),
                                    5 as i32,
                                ),
                            );
                        }
                        let ref mut fresh3 = *trans_pairs
                            .offset((2 as i32 as u64).wrapping_mul(i) as isize);
                        *fresh3 = (if ::core::mem::size_of::<i8>() as u64
                            == 1 as i32 as u64
                        {
                            xzalloc(
                                (*src_lens.offset(i as isize)).wrapping_add(1 as i32 as u64),
                            )
                        } else {
                            xcalloc(
                                (*src_lens.offset(i as isize))
                                    .wrapping_add(1 as i32 as u64),
                                ::core::mem::size_of::<i8>() as u64,
                            )
                        }) as *mut i8;
                        memcpy(
                            *trans_pairs
                                .offset((2 as i32 as u64).wrapping_mul(i) as isize)
                                as *mut libc::c_void,
                            src_buf as *const libc::c_void,
                            *src_lens.offset(i as isize),
                        );
                        *(*trans_pairs
                            .offset((2 as i32 as u64).wrapping_mul(i) as isize))
                            .offset(*src_lens.offset(i as isize) as isize) = '\0' as i32
                            as i8;
                        src_buf = src_buf.offset(*src_lens.offset(i as isize) as isize);
                        mbclen = if mb_cur_max == 1 as i32 {
                            1 as i32 as u64
                        } else {
                            rpl_mbrtowc(
                                0 as *mut wchar_t,
                                dest_buf.offset(idx as isize),
                                dest_len.wrapping_sub(idx),
                                &mut cur_stat,
                            )
                        };
                        if mbclen == -(1 as i32) as size_t
                            || mbclen == -(2 as i32) as size_t
                            || mbclen == 0 as i32 as u64
                        {
                            mbclen = 1 as i32 as size_t;
                        }
                        let ref mut fresh4 = *trans_pairs
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(i)
                                    .wrapping_add(1 as i32 as u64) as isize,
                            );
                        *fresh4 = (if ::core::mem::size_of::<i8>() as u64
                            == 1 as i32 as u64
                        {
                            xzalloc(mbclen.wrapping_add(1 as i32 as u64))
                        } else {
                            xcalloc(
                                mbclen.wrapping_add(1 as i32 as u64),
                                ::core::mem::size_of::<i8>() as u64,
                            )
                        }) as *mut i8;
                        memcpy(
                            *trans_pairs
                                .offset(
                                    (2 as i32 as u64)
                                        .wrapping_mul(i)
                                        .wrapping_add(1 as i32 as u64) as isize,
                                ) as *mut libc::c_void,
                            dest_buf.offset(idx as isize) as *const libc::c_void,
                            mbclen,
                        );
                        *(*trans_pairs
                            .offset(
                                (2 as i32 as u64)
                                    .wrapping_mul(i)
                                    .wrapping_add(1 as i32 as u64) as isize,
                            ))
                            .offset(mbclen as isize) = '\0' as i32 as i8;
                        idx = (idx as u64).wrapping_add(mbclen) as size_t as size_t;
                        i = i.wrapping_add(1);
                        i;
                    }
                    let ref mut fresh5 = *trans_pairs
                        .offset((2 as i32 as u64).wrapping_mul(i) as isize);
                    *fresh5 = 0 as *mut i8;
                    if idx != dest_len {
                        bad_prog(
                            dcgettext(
                                0 as *const i8,
                                errors
                                    .as_ptr()
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize),
                                5 as i32,
                            ),
                        );
                    }
                } else {
                    let mut translate: *mut u8 = ({
                        let mut __h: *mut obstack = &mut obs as *mut obstack;
                        let mut __o: *mut obstack = __h;
                        let mut __len: size_t = (256 as i32 as u64)
                            .wrapping_mul(::core::mem::size_of::<u8>() as u64);
                        if ({
                            let mut __o1: *const obstack = __o;
                            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64
                                as size_t
                        }) < __len
                        {
                            _obstack_newchunk(__o, __len);
                        }
                        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
                        ({
                            let mut __o1: *mut obstack = __h;
                            let mut __value: *mut libc::c_void = (*__o1).object_base
                                as *mut libc::c_void;
                            if (*__o1).next_free == __value as *mut i8 {
                                (*__o1).set_maybe_empty_object(1 as i32 as u32);
                            }
                            (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>()
                                as u64) < ::core::mem::size_of::<*mut libc::c_void>() as u64
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut i8
                            })
                                .offset(
                                    ((((*__o1).next_free)
                                        .offset_from(
                                            (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                                < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                            {
                                                (*__o1).object_base
                                            } else {
                                                0 as *mut i8
                                            }),
                                        ) as i64 as u64)
                                        .wrapping_add((*__o1).alignment_mask)
                                        & !(*__o1).alignment_mask) as isize,
                                );
                            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8)
                                as i64 as size_t
                                > ((*__o1).chunk_limit)
                                    .offset_from((*__o1).chunk as *mut i8) as i64 as size_t
                            {
                                (*__o1).next_free = (*__o1).chunk_limit;
                            }
                            (*__o1).object_base = (*__o1).next_free;
                            __value
                        })
                    }) as *mut u8;
                    let mut ustring: *mut u8 = src_buf as *mut u8;
                    if len != dest_len {
                        bad_prog(
                            dcgettext(
                                0 as *const i8,
                                errors
                                    .as_ptr()
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize),
                                5 as i32,
                            ),
                        );
                    }
                    len = 0 as i32 as size_t;
                    while len < 256 as i32 as u64 {
                        *translate.offset(len as isize) = len as u8;
                        len = len.wrapping_add(1);
                        len;
                    }
                    loop {
                        let fresh6 = dest_len;
                        dest_len = dest_len.wrapping_sub(1);
                        if !(fresh6 != 0) {
                            break;
                        }
                        let fresh7 = dest_buf;
                        dest_buf = dest_buf.offset(1);
                        let fresh8 = ustring;
                        ustring = ustring.offset(1);
                        *translate.offset(*fresh8 as isize) = *fresh7 as u8;
                    }
                    (*cur_cmd).x.translate = translate;
                }
                read_end_of_cmd();
                free_buffer(b);
                free_buffer(b2_0);
                current_block_184 = 6957654774345280688;
            }
            -1 => {
                bad_prog(
                    dcgettext(
                        0 as *const i8,
                        errors
                            .as_ptr()
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize),
                        5 as i32,
                    ),
                );
                current_block_184 = 13605335638665870143;
            }
            _ => {
                current_block_184 = 13605335638665870143;
            }
        }
        match current_block_184 {
            12491576746588794161 => {
                ch = in_nonblank();
                if 1 as i32 != 0
                    && *(*__ctype_b_loc()).offset(ch as u8 as i32 as isize) as i32
                        & C2RustUnnamed_4::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    && posixicity as u32 != posixicity_types::POSIXLY_BASIC as i32 as u32
                {
                    (*cur_cmd).x.int_arg = in_integer(ch) as i32;
                } else {
                    (*cur_cmd).x.int_arg = -(1 as i32);
                    savchar(ch);
                }
                read_end_of_cmd();
            }
            12666756727116774827 => {
                if ch == -(1 as i32) {
                    bad_prog(
                        dcgettext(
                            0 as *const i8,
                            errors
                                .as_ptr()
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize),
                            5 as i32,
                        ),
                    );
                }
                if ch == '\\' as i32 {
                    ch = inchar();
                } else {
                    if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32
                    {
                        bad_prog(
                            dcgettext(
                                0 as *const i8,
                                errors
                                    .as_ptr()
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize),
                                5 as i32,
                            ),
                        );
                    }
                    savchar(ch);
                    ch = '\n' as i32;
                }
                read_text(&mut (*cur_cmd).x.cmd_txt, ch);
            }
            13605335638665870143 => {
                bad_command(ch as i8);
            }
            _ => {}
        }
        (*vector).v_length = ((*vector).v_length).wrapping_add(1);
        (*vector).v_length;
    }
    if posixicity as u32 == posixicity_types::POSIXLY_BASIC as i32 as u32
        && !pending_text.is_null()
    {
        bad_prog(
            dcgettext(
                0 as *const i8,
                errors
                    .as_ptr()
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize),
                5 as i32,
            ),
        );
    }
    return vector;
}
#[no_mangle]
pub unsafe extern "C" fn normalize_text(
    mut buf: *mut i8,
    mut len: size_t,
    mut buftype: text_types,
) -> size_t {
    let mut current_block: u64;
    let mut bufend: *const i8 = buf.offset(len as isize);
    let mut p: *mut i8 = buf;
    let mut q: *mut i8 = buf;
    let mut ch: i8 = 0;
    let mut base: i32 = 0;
    let mut bracket_state: i32 = 0 as i32;
    let mut mbclen: i32 = 0;
    let mut cur_stat: mbstate_t = {
        let mut init = __mbstate_t {
            __count: 0 as i32,
            __value: C2RustUnnamed { __wch: 0 },
        };
        init
    };
    while p < bufend as *mut i8 {
        mbclen = (if mb_cur_max == 1 as i32 {
            1 as i32 as u64
        } else {
            rpl_mbrtowc(
                0 as *mut wchar_t,
                p,
                bufend.offset_from(p) as i64 as size_t,
                &mut cur_stat,
            )
        }) as i32;
        if mbclen != 1 as i32 {
            if mbclen as u64 == -(1 as i32) as size_t
                || mbclen as u64 == -(2 as i32) as size_t || mbclen == 0 as i32
            {
                mbclen = 1 as i32;
            }
            memmove(q as *mut libc::c_void, p as *const libc::c_void, mbclen as u64);
            q = q.offset(mbclen as isize);
            p = p.offset(mbclen as isize);
        } else {
            if *p as i32 == '\\' as i32
                && p.offset(1 as i32 as isize) < bufend as *mut i8
                && bracket_state == 0 as i32
            {
                p = p.offset(1);
                match *p as i32 {
                    97 => {
                        let fresh9 = q;
                        q = q.offset(1);
                        *fresh9 = '\u{7}' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    102 => {
                        let fresh10 = q;
                        q = q.offset(1);
                        *fresh10 = '\u{c}' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    10 | 110 => {
                        let fresh11 = q;
                        q = q.offset(1);
                        *fresh11 = '\n' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    114 => {
                        let fresh12 = q;
                        q = q.offset(1);
                        *fresh12 = '\r' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    116 => {
                        let fresh13 = q;
                        q = q.offset(1);
                        *fresh13 = '\t' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    118 => {
                        let fresh14 = q;
                        q = q.offset(1);
                        *fresh14 = '\u{b}' as i32 as i8;
                        p = p.offset(1);
                        p;
                        continue;
                    }
                    100 => {
                        base = 10 as i32;
                        current_block = 7165673023302390207;
                    }
                    120 => {
                        base = 16 as i32;
                        current_block = 7165673023302390207;
                    }
                    111 => {
                        base = 8 as i32;
                        current_block = 7165673023302390207;
                    }
                    99 => {
                        p = p.offset(1);
                        if p < bufend as *mut i8 {
                            let fresh17 = q;
                            q = q.offset(1);
                            *fresh17 = (({
                                let mut __res: i32 = 0;
                                if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                                    if 0 != 0 {
                                        let mut __c: i32 = *p as u8 as i32;
                                        __res = (if __c < -(128 as i32) || __c > 255 as i32 {
                                            __c
                                        } else {
                                            *(*__ctype_toupper_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = toupper(*p as u8 as i32);
                                    }
                                } else {
                                    __res = *(*__ctype_toupper_loc())
                                        .offset(*p as u8 as i32 as isize);
                                }
                                __res
                            }) ^ 0x40 as i32) as i8;
                            if *p as i32 == '\\' as i32 {
                                p = p.offset(1);
                                p;
                                if *p as i32 != '\\' as i32 {
                                    bad_prog(
                                        errors
                                            .as_ptr()
                                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 33]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 31]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 29]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 16]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 27]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 25]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 36]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 39]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 45]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 46]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 51]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 30]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 32]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 22]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 19]>() as u64 as isize)
                                            .offset(::core::mem::size_of::<[i8; 18]>() as u64 as isize),
                                    );
                                }
                            }
                            p = p.offset(1);
                            p;
                            continue;
                        } else {
                            if buftype as u32 != text_types::TEXT_BUFFER as i32 as u32 {
                                let fresh18 = q;
                                q = q.offset(1);
                                *fresh18 = '\\' as i32 as i8;
                            }
                            continue;
                        }
                    }
                    _ => {
                        if buftype as u32 != text_types::TEXT_BUFFER as i32 as u32 {
                            let fresh19 = q;
                            q = q.offset(1);
                            *fresh19 = '\\' as i32 as i8;
                        }
                        current_block = 8545136480011357681;
                    }
                }
                match current_block {
                    8545136480011357681 => {}
                    _ => {
                        p = convert_number(&mut ch, p, bufend, base);
                        if buftype as u32 == text_types::TEXT_REPLACEMENT as i32 as u32
                            && (ch as i32 == '&' as i32 || ch as i32 == '\\' as i32)
                        {
                            let fresh15 = q;
                            q = q.offset(1);
                            *fresh15 = '\\' as i32 as i8;
                        }
                        let fresh16 = q;
                        q = q.offset(1);
                        *fresh16 = ch;
                        continue;
                    }
                }
            } else if buftype as u32 == text_types::TEXT_REGEX as i32 as u32
                && posixicity as u32 != posixicity_types::POSIXLY_EXTENDED as i32 as u32
            {
                match *p as i32 {
                    91 => {
                        if bracket_state == 0 {
                            bracket_state = -(1 as i32);
                        }
                    }
                    58 | 46 | 61 => {
                        if bracket_state == -(1 as i32)
                            && *p.offset(-(1 as i32) as isize) as i32 == '[' as i32
                        {
                            bracket_state = *p as i32;
                        }
                    }
                    93 => {
                        if !(bracket_state == 0 as i32) {
                            if bracket_state == -(1 as i32) {
                                bracket_state = 0 as i32;
                            } else if *p.offset(-(2 as i32) as isize) as i32
                                != bracket_state
                                && *p.offset(-(1 as i32) as isize) as i32 == bracket_state
                            {
                                bracket_state = -(1 as i32);
                            }
                        }
                    }
                    _ => {}
                }
            }
            let fresh20 = p;
            p = p.offset(1);
            let fresh21 = q;
            q = q.offset(1);
            *fresh21 = *fresh20;
        }
    }
    return q.offset_from(buf) as i64 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn compile_string(
    mut cur_program: *mut vector,
    mut str: *mut i8,
    mut len: size_t,
) -> *mut vector {
    static mut string_expr_count: countT = 0 as i32 as countT;
    let mut ret: *mut vector = 0 as *mut vector;
    prog.file = 0 as *mut FILE;
    prog.base = str as *mut u8;
    prog.cur = prog.base;
    prog.end = (prog.cur).offset(len as isize);
    cur_input.line = 0 as i32 as countT;
    cur_input.name = 0 as *const i8;
    string_expr_count = string_expr_count.wrapping_add(1);
    cur_input.string_expr_count = string_expr_count;
    ret = compile_program(cur_program);
    prog.base = 0 as *const u8;
    prog.cur = 0 as *const u8;
    prog.end = 0 as *const u8;
    first_script = 0 as i32 != 0;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn compile_file(
    mut cur_program: *mut vector,
    mut cmdfile: *const i8,
) -> *mut vector {
    let mut ret: *mut vector = 0 as *mut vector;
    prog.file = stdin;
    if *cmdfile.offset(0 as i32 as isize) as i32 != '-' as i32
        || *cmdfile.offset(1 as i32 as isize) as i32 != '\0' as i32
    {
        prog.file = ck_fopen(cmdfile, b"rt\0" as *const u8 as *const i8, 1 as i32);
    }
    cur_input.line = 1 as i32 as countT;
    cur_input.name = cmdfile;
    cur_input.string_expr_count = 0 as i32 as countT;
    ret = compile_program(cur_program);
    if prog.file != stdin {
        ck_fclose(prog.file);
    }
    prog.file = 0 as *mut FILE;
    first_script = 0 as i32 != 0;
    return ret;
}
unsafe extern "C" fn cleanup_program_filenames() {
    let mut p: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).name).is_null() {
            rpl_free((*p).name as *mut libc::c_void);
            (*p).name = 0 as *mut i8;
        }
        p = (*p).link;
    }
    p = file_write;
    while !p.is_null() {
        if !((*p).name).is_null() {
            rpl_free((*p).name as *mut libc::c_void);
            (*p).name = 0 as *mut i8;
        }
        p = (*p).link;
    }
}
#[no_mangle]
pub unsafe extern "C" fn check_final_program(mut program: *mut vector) {
    let mut go: *mut sed_label = 0 as *mut sed_label;
    let mut lbl: *mut sed_label = 0 as *mut sed_label;
    if !blocks.is_null() {
        memcpy(
            &mut cur_input as *mut error_info as *mut libc::c_void,
            &mut (*blocks).err_info as *mut error_info as *const libc::c_void,
            ::core::mem::size_of::<error_info>() as u64,
        );
        bad_prog(
            dcgettext(
                0 as *const i8,
                errors
                    .as_ptr()
                    .offset(::core::mem::size_of::<[i8; 14]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 15]>() as u64 as isize)
                    .offset(::core::mem::size_of::<[i8; 43]>() as u64 as isize),
                5 as i32,
            ),
        );
    }
    if !pending_text.is_null() {
        (*old_text_buf).text_length = size_buffer(pending_text);
        if (*old_text_buf).text_length != 0 {
            (*old_text_buf).text = xmemdup(
                get_buffer(pending_text) as *const libc::c_void,
                ((*old_text_buf).text_length)
                    .wrapping_mul(::core::mem::size_of::<i8>() as u64),
            ) as *mut i8;
        }
        free_buffer(pending_text);
        pending_text = 0 as *mut buffer;
    }
    go = jumps;
    while !go.is_null() {
        lbl = labels;
        while !lbl.is_null() {
            if strcmp((*lbl).name, (*go).name) == 0 as i32 {
                break;
            }
            lbl = (*lbl).next;
        }
        if !lbl.is_null() {
            (*((*program).v).offset((*go).v_index as isize)).x.jump_index = (*lbl)
                .v_index;
        } else {
            if *(*go).name != 0 {
                panic(
                    dcgettext(
                        0 as *const i8,
                        b"can't find label for jump to `%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*go).name,
                );
            }
            (*((*program).v).offset((*go).v_index as isize)).x.jump_index = (*program)
                .v_length;
        }
        go = release_label(go);
    }
    jumps = 0 as *mut sed_label;
    lbl = labels;
    while !lbl.is_null() {
        lbl = release_label(lbl);
    }
    labels = 0 as *mut sed_label;
}
#[no_mangle]
pub unsafe extern "C" fn rewind_read_files() {
    let mut p: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            rewind((*p).fp);
        }
        p = (*p).link;
    }
}
#[no_mangle]
pub unsafe extern "C" fn finish_program(mut program: *mut vector) {
    cleanup_program_filenames();
    let mut p: *mut output = 0 as *mut output;
    let mut q: *mut output = 0 as *mut output;
    p = file_read;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            ck_fclose((*p).fp);
        }
        q = (*p).link;
        p = q;
    }
    p = file_write;
    while !p.is_null() {
        if !((*p).fp).is_null() {
            ck_fclose((*p).fp);
        }
        q = (*p).link;
        p = q;
    }
    file_write = 0 as *mut output;
    file_read = file_write;
}