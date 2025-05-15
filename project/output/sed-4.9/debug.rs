use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type re_dfa_t;
    pub type dfa;
    static mut mb_cur_max: i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    static mut stdout: *mut _IO_FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn puts(__s: *const i8) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = u64;
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
pub enum C2RustUnnamed_0 {
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
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::_ISprint => 16384,
            C2RustUnnamed_0::_ISalnum => 8,
            C2RustUnnamed_0::_ISpunct => 4,
            C2RustUnnamed_0::_IScntrl => 2,
            C2RustUnnamed_0::_ISblank => 1,
            C2RustUnnamed_0::_ISgraph => 32768,
            C2RustUnnamed_0::_ISspace => 8192,
            C2RustUnnamed_0::_ISxdigit => 4096,
            C2RustUnnamed_0::_ISdigit => 2048,
            C2RustUnnamed_0::_ISalpha => 1024,
            C2RustUnnamed_0::_ISlower => 512,
            C2RustUnnamed_0::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            16384 => C2RustUnnamed_0::_ISprint,
            8 => C2RustUnnamed_0::_ISalnum,
            4 => C2RustUnnamed_0::_ISpunct,
            2 => C2RustUnnamed_0::_IScntrl,
            1 => C2RustUnnamed_0::_ISblank,
            32768 => C2RustUnnamed_0::_ISgraph,
            8192 => C2RustUnnamed_0::_ISspace,
            4096 => C2RustUnnamed_0::_ISxdigit,
            2048 => C2RustUnnamed_0::_ISdigit,
            1024 => C2RustUnnamed_0::_ISalpha,
            512 => C2RustUnnamed_0::_ISlower,
            256 => C2RustUnnamed_0::_ISupper,
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
static mut block_level: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn debug_print_char(mut c: i8) {
    if 1 as i32 != 0
        && *(*__ctype_b_loc()).offset(c as i32 as isize) as i32
            & C2RustUnnamed_0::_ISprint as i32 as libc::c_ushort as i32 != 0
        && c as i32 != '\\' as i32
    {
        putchar_unlocked(c as i32);
        return;
    }
    putchar_unlocked('\\' as i32);
    match c as i32 {
        7 => {
            putchar_unlocked('a' as i32);
        }
        12 => {
            putchar_unlocked('f' as i32);
        }
        13 => {
            putchar_unlocked('r' as i32);
        }
        9 => {
            putchar_unlocked('t' as i32);
        }
        11 => {
            putchar_unlocked('v' as i32);
        }
        10 => {
            putchar_unlocked('n' as i32);
        }
        92 => {
            putchar_unlocked('\\' as i32);
        }
        _ => {
            printf(b"o%03o\0" as *const u8 as *const i8, c as u32);
        }
    };
}
unsafe extern "C" fn debug_print_regex_pattern(mut pat: *const i8, mut len: size_t) {
    let mut p: *const i8 = pat;
    loop {
        let fresh1 = len;
        len = len.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        if *p as i32 == '/' as i32 {
            fputs_unlocked(b"\\/\0" as *const u8 as *const i8, stdout);
        } else {
            debug_print_char(*p);
        }
        p = p.offset(1);
        p;
    };
}
unsafe extern "C" fn debug_print_regex_flags(mut r: *const regex, mut addr: bool) {
    if r.is_null() {
        return;
    }
    if (*r).flags & (1 as i32) << 1 as i32 != 0 {
        putchar_unlocked(if addr as i32 != 0 { 'I' as i32 } else { 'i' as i32 });
    }
    if (*r).flags & (1 as i32) << 2 as i32 != 0 {
        putchar_unlocked(if addr as i32 != 0 { 'M' as i32 } else { 'm' as i32 });
    }
}
unsafe extern "C" fn debug_print_regex(mut r: *const regex) {
    if r.is_null() {
        fputs_unlocked(b"//\0" as *const u8 as *const i8, stdout);
        return;
    }
    putchar_unlocked('/' as i32);
    debug_print_regex_pattern(((*r).re).as_ptr(), (*r).sz);
    putchar_unlocked('/' as i32);
}
unsafe extern "C" fn debug_print_addr(mut a: *const addr) {
    if a.is_null() {
        return;
    }
    match (*a).addr_type as u32 {
        0 => {
            fputs_unlocked(b"[ADDR-NULL]\0" as *const u8 as *const i8, stdout);
        }
        1 => {
            debug_print_regex((*a).addr_regex);
            debug_print_regex_flags((*a).addr_regex, 1 as i32 != 0);
        }
        2 => {
            printf(b"%lu\0" as *const u8 as *const i8, (*a).addr_number);
        }
        3 => {
            printf(
                b"%lu~%lu\0" as *const u8 as *const i8,
                (*a).addr_number,
                (*a).addr_step,
            );
        }
        4 => {
            printf(b"+%lu\0" as *const u8 as *const i8, (*a).addr_step);
        }
        5 => {
            printf(b"~%lu\0" as *const u8 as *const i8, (*a).addr_step);
        }
        6 => {
            putchar_unlocked('$' as i32);
        }
        _ => {}
    };
}
unsafe extern "C" fn debug_print_subst_replacement(mut r: *const replacement) {
    let mut last_repl_type: replacement_types = replacement_types::REPL_ASIS;
    if r.is_null() {
        return;
    }
    let mut p: *const replacement = r;
    while !p.is_null() {
        if (*p).repl_type as u32 != last_repl_type as u32 {
            putchar_unlocked('\\' as i32);
            if (*p).repl_type as u32 == 0 as i32 as u32 {
                putchar_unlocked('E' as i32);
            } else if (*p).repl_type as u32
                == replacement_types::REPL_UPPERCASE as i32 as u32
            {
                putchar_unlocked('U' as i32);
            } else if (*p).repl_type as u32
                == replacement_types::REPL_LOWERCASE as i32 as u32
            {
                putchar_unlocked('L' as i32);
            } else if (*p).repl_type as u32
                & replacement_types::REPL_MODIFIERS as i32 as u32
                == replacement_types::REPL_UPPERCASE_FIRST as i32 as u32
            {
                putchar_unlocked('u' as i32);
            } else if (*p).repl_type as u32
                & replacement_types::REPL_MODIFIERS as i32 as u32
                == replacement_types::REPL_LOWERCASE_FIRST as i32 as u32
            {
                putchar_unlocked('l' as i32);
            }
            last_repl_type = (*p).repl_type;
        }
        if (*p).prefix_length != 0 {
            if 0 != 0 && 0 != 0
                && (1 as i32 as size_t).wrapping_mul((*p).prefix_length)
                    <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
            {
                ({
                    let mut __ptr: *const i8 = (*p).prefix as *const i8;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as i32 as size_t).wrapping_mul((*p).prefix_length);
                    while __cnt > 0 as i32 as u64 {
                        if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                            as i32 as i64 != 0
                        {
                            let fresh2 = __ptr;
                            __ptr = __ptr.offset(1);
                            __overflow(__stream, *fresh2 as u8 as i32)
                        } else {
                            let fresh3 = __ptr;
                            __ptr = __ptr.offset(1);
                            let fresh4 = (*__stream)._IO_write_ptr;
                            (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                .offset(1);
                            *fresh4 = *fresh3;
                            *fresh4 as u8 as i32
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
                if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                    || 0 != 0 && (*p).prefix_length == 0 as i32 as u64
                {} else {
                    fwrite_unlocked(
                        (*p).prefix as *const libc::c_void,
                        1 as i32 as size_t,
                        (*p).prefix_length,
                        stdout,
                    );
                };
            };
            compile_error!("Conditional expression is not supposed to be used");
        }
        if (*p).subst_id != -(1 as i32) {
            if (*p).subst_id == 0 as i32 {
                putchar_unlocked('&' as i32);
            } else {
                printf(b"\\%d\0" as *const u8 as *const i8, (*p).subst_id);
            }
        }
        p = (*p).next;
    }
}
unsafe extern "C" fn debug_print_output_file(mut o: *const output) {
    if o.is_null() {
        return;
    }
    fputs_unlocked((*o).name, stdout);
}
unsafe extern "C" fn debug_print_subst(mut s: *const subst) {
    if s.is_null() {
        return;
    }
    debug_print_regex((*s).regx);
    debug_print_subst_replacement((*s).replacement);
    putchar_unlocked('/' as i32);
    debug_print_regex_flags((*s).regx, 0 as i32 != 0);
    if (*s).global() != 0 {
        putchar_unlocked('g' as i32);
    }
    if (*s).eval() != 0 {
        putchar_unlocked('e' as i32);
    }
    if (*s).print() != 0 {
        putchar_unlocked('p' as i32);
    }
    if (*s).numb != 0 {
        printf(b"%lu\0" as *const u8 as *const i8, (*s).numb);
    }
    if !((*s).outf).is_null() {
        putchar_unlocked('w' as i32);
        debug_print_output_file((*s).outf);
    }
}
unsafe extern "C" fn debug_print_translation(mut sc: *const sed_cmd) {
    let mut i: u32 = 0;
    if mb_cur_max > 1 as i32 {
        putchar_unlocked('/' as i32);
        i = 0 as i32 as u32;
        while !(*((*sc).x.translatemb)
            .offset((2 as i32 as u32).wrapping_mul(i) as isize))
            .is_null()
        {
            fputs_unlocked(
                *((*sc).x.translatemb)
                    .offset((2 as i32 as u32).wrapping_mul(i) as isize),
                stdout,
            );
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
        i = 0 as i32 as u32;
        while !(*((*sc).x.translatemb)
            .offset((2 as i32 as u32).wrapping_mul(i) as isize))
            .is_null()
        {
            fputs_unlocked(
                *((*sc).x.translatemb)
                    .offset(
                        (2 as i32 as u32).wrapping_mul(i).wrapping_add(1 as i32 as u32)
                            as isize,
                    ),
                stdout,
            );
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
    } else {
        putchar_unlocked('/' as i32);
        i = 0 as i32 as u32;
        while i < 256 as i32 as u32 {
            if *((*sc).x.translate).offset(i as isize) as i32 != i as u8 as i32 {
                putchar_unlocked(i as u8 as i32);
            }
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
        i = 0 as i32 as u32;
        while i < 256 as i32 as u32 {
            if *((*sc).x.translate).offset(i as isize) as i32 != i as u8 as i32 {
                putchar_unlocked(*((*sc).x.translate).offset(i as isize) as i32);
            }
            i = i.wrapping_add(1);
            i;
        }
        putchar_unlocked('/' as i32);
    };
}
unsafe extern "C" fn debug_print_function(
    mut program: *const vector,
    mut sc: *const sed_cmd,
) {
    if sc.is_null() {
        return;
    }
    putchar_unlocked((*sc).cmd as i32);
    let mut current_block_26: u64;
    match (*sc).cmd as i32 {
        58 => {
            printf(b"%s\0" as *const u8 as *const i8, (*sc).x.label_name);
            current_block_26 = 5330834795799507926;
        }
        35 => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"sed/debug.c\0" as *const u8 as *const i8,
                291 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_6272: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"sed/debug.c\0" as *const u8 as *const i8,
                    291 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[i8; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 1804405408969159832;
        }
        97 | 99 | 105 => {
            current_block_26 = 1804405408969159832;
        }
        98 | 116 | 84 => {
            if (*sc).x.jump_index < (*program).v_length {
                let mut label_name: *const i8 = (*((*program).v)
                    .offset((*sc).x.jump_index as isize))
                    .x
                    .label_name;
                if !label_name.is_null() {
                    printf(b" %s\0" as *const u8 as *const i8, label_name);
                }
            }
            current_block_26 = 5330834795799507926;
        }
        101 => {
            putchar_unlocked(' ' as i32);
            if 0 != 0 && 0 != 0
                && (1 as i32 as size_t).wrapping_mul((*sc).x.cmd_txt.text_length)
                    <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
            {
                ({
                    let mut __ptr: *const i8 = (*sc).x.cmd_txt.text as *const i8;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as i32 as size_t)
                        .wrapping_mul((*sc).x.cmd_txt.text_length);
                    while __cnt > 0 as i32 as u64 {
                        if (if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end)
                            as i32 as i64 != 0
                        {
                            let fresh8 = __ptr;
                            __ptr = __ptr.offset(1);
                            __overflow(__stream, *fresh8 as u8 as i32)
                        } else {
                            let fresh9 = __ptr;
                            __ptr = __ptr.offset(1);
                            let fresh10 = (*__stream)._IO_write_ptr;
                            (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                .offset(1);
                            *fresh10 = *fresh9;
                            *fresh10 as u8 as i32
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
                if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                    || 0 != 0 && (*sc).x.cmd_txt.text_length == 0 as i32 as u64
                {} else {
                    fwrite_unlocked(
                        (*sc).x.cmd_txt.text as *const libc::c_void,
                        1 as i32 as size_t,
                        (*sc).x.cmd_txt.text_length,
                        stdout,
                    );
                };
            };
            compile_error!("Conditional expression is not supposed to be used");
            current_block_26 = 5330834795799507926;
        }
        76 | 108 | 113 | 81 => {
            if (*sc).x.int_arg != -(1 as i32) {
                printf(b" %d\0" as *const u8 as *const i8, (*sc).x.int_arg);
            }
            current_block_26 = 5330834795799507926;
        }
        114 => {
            putchar_unlocked(' ' as i32);
            fputs_unlocked((*sc).x.readcmd.fname, stdout);
            current_block_26 = 5330834795799507926;
        }
        82 => {
            putchar_unlocked(' ' as i32);
            fputs_unlocked((*(*sc).x.inf).name, stdout);
            current_block_26 = 5330834795799507926;
        }
        115 => {
            debug_print_subst((*sc).x.cmd_subst);
            current_block_26 = 5330834795799507926;
        }
        118 => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"sed/debug.c\0" as *const u8 as *const i8,
                382 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_4812: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"sed/debug.c\0" as *const u8 as *const i8,
                    382 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[i8; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 10730382391303010067;
        }
        87 => {
            current_block_26 = 10730382391303010067;
        }
        119 => {
            debug_print_output_file((*sc).x.outf);
            current_block_26 = 5330834795799507926;
        }
        121 => {
            debug_print_translation(sc);
            current_block_26 = 5330834795799507926;
        }
        61 | 123 | 125 | 68 | 100 | 70 | 103 | 71 | 104 | 72 | 110 | 78 | 80 | 112 | 120
        | 122 => {
            current_block_26 = 5330834795799507926;
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"sed/debug.c\0" as *const u8 as *const i8,
                404 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                ))
                    .as_ptr(),
            );
            'c_4502: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"sed/debug.c\0" as *const u8 as *const i8,
                    404 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[i8; 73],
                    >(
                        b"void debug_print_function(const struct vector *, const struct sed_cmd *)\0",
                    ))
                        .as_ptr(),
                );
            };
            current_block_26 = 5330834795799507926;
        }
    }
    match current_block_26 {
        1804405408969159832 => {
            fputs_unlocked(b"\\\0" as *const u8 as *const i8, stdout);
            if (*sc).x.cmd_txt.text_length != 0 {
                if 0 != 0 && 0 != 0
                    && (1 as i32 as size_t).wrapping_mul((*sc).x.cmd_txt.text_length)
                        <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
                {
                    ({
                        let mut __ptr: *const i8 = (*sc).x.cmd_txt.text as *const i8;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as i32 as size_t)
                            .wrapping_mul((*sc).x.cmd_txt.text_length);
                        while __cnt > 0 as i32 as u64 {
                            if (if ((*__stream)._IO_write_ptr
                                >= (*__stream)._IO_write_end) as i32 as i64 != 0
                            {
                                let fresh5 = __ptr;
                                __ptr = __ptr.offset(1);
                                __overflow(__stream, *fresh5 as u8 as i32)
                            } else {
                                let fresh6 = __ptr;
                                __ptr = __ptr.offset(1);
                                let fresh7 = (*__stream)._IO_write_ptr;
                                (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr)
                                    .offset(1);
                                *fresh7 = *fresh6;
                                *fresh7 as u8 as i32
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
                    if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                        || 0 != 0 && (*sc).x.cmd_txt.text_length == 0 as i32 as u64
                    {} else {
                        fwrite_unlocked(
                            (*sc).x.cmd_txt.text as *const libc::c_void,
                            1 as i32 as size_t,
                            (*sc).x.cmd_txt.text_length,
                            stdout,
                        );
                    };
                };
                compile_error!("Conditional expression is not supposed to be used");
            }
        }
        10730382391303010067 => {
            debug_print_output_file((*sc).x.outf);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn debug_print_command(
    mut program: *const vector,
    mut sc: *const sed_cmd,
) {
    let mut addr_bang: bool = false;
    if program.is_null() {
        return;
    }
    if (*sc).cmd as i32 == '}' as i32 {
        block_level -= 1;
        block_level;
    }
    let mut j: i32 = 0 as i32;
    while j < block_level {
        fputs_unlocked(b"  \0" as *const u8 as *const i8, stdout);
        j += 1;
        j;
    }
    debug_print_addr((*sc).a1);
    if !((*sc).a2).is_null() {
        putchar_unlocked(',' as i32);
    }
    debug_print_addr((*sc).a2);
    addr_bang = (*sc).addr_bang != 0;
    if (*sc).cmd as i32 == '{' as i32 {
        addr_bang = !addr_bang;
    }
    if addr_bang {
        putchar_unlocked('!' as i32);
    }
    if !((*sc).a1).is_null() || !((*sc).a2).is_null() {
        putchar_unlocked(' ' as i32);
    }
    debug_print_function(program, sc);
    putchar_unlocked('\n' as i32);
    if (*sc).cmd as i32 == '{' as i32 {
        block_level += 1;
        block_level;
    }
}
#[no_mangle]
pub unsafe extern "C" fn debug_print_program(mut program: *const vector) {
    if program.is_null() {
        return;
    }
    block_level = 1 as i32;
    puts(b"SED PROGRAM:\0" as *const u8 as *const i8);
    let mut i: size_t = 0 as i32 as size_t;
    while i < (*program).v_length {
        debug_print_command(program, &mut *((*program).v).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    block_level = 0 as i32;
}