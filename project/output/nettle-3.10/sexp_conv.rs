use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strerror(_: i32) -> *mut i8;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
    static nettle_md5: nettle_hash;
    static nettle_sha1: nettle_hash;
    static nettle_sha256: nettle_hash;
    static nettle_base64: nettle_armor;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn die(format: *const i8, _: ...) -> !;
    fn werror(format: *const i8, _: ...);
    fn xalloc(size: size_t) -> *mut libc::c_void;
    fn sexp_input_init(input: *mut sexp_input, f: *mut FILE);
    fn sexp_get_char(input: *mut sexp_input);
    fn sexp_output_init(
        output: *mut sexp_output,
        f: *mut FILE,
        width: u32,
        prefer_hex: i32,
    );
    fn sexp_output_hash_init(
        output: *mut sexp_output,
        hash: *const nettle_hash,
        ctx: *mut libc::c_void,
    );
    fn sexp_put_newline(output: *mut sexp_output, indent: u32);
    fn sexp_put_soft_newline(output: *mut sexp_output, indent: u32);
    fn sexp_put_char(output: *mut sexp_output, c: uint8_t);
    fn sexp_put_data(output: *mut sexp_output, length: u32, data: *const uint8_t);
    fn sexp_put_code_start(output: *mut sexp_output, coding: *const nettle_armor);
    fn sexp_put_code_end(output: *mut sexp_output);
    fn sexp_put_string(
        output: *mut sexp_output,
        mode: sexp_mode,
        string: *mut nettle_buffer,
    );
    fn sexp_put_digest(output: *mut sexp_output);
    fn sexp_compound_token_init(token: *mut sexp_compound_token);
    fn sexp_compound_token_clear(token: *mut sexp_compound_token);
    fn sexp_parse_init(
        parser: *mut sexp_parser,
        input: *mut sexp_input,
        mode: sexp_mode,
    );
    fn sexp_parse(parser: *mut sexp_parser, token: *mut sexp_compound_token);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
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
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_type: libc::c_short,
    pub l_whence: libc::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
pub type uint8_t = __uint8_t;
pub type nettle_realloc_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const i8,
) -> i32;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(*mut libc::c_void) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_buffer {
    pub contents: *mut uint8_t,
    pub alloc: size_t,
    pub realloc_ctx: *mut libc::c_void,
    pub realloc: Option<nettle_realloc_func>,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const i8,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<nettle_armor_init_func>,
    pub encode_length: Option<nettle_armor_length_func>,
    pub encode_update: Option<nettle_armor_encode_update_func>,
    pub encode_final: Option<nettle_armor_encode_final_func>,
    pub decode_init: Option<nettle_armor_init_func>,
    pub decode_length: Option<nettle_armor_length_func>,
    pub decode_update: Option<nettle_armor_decode_update_func>,
    pub decode_final: Option<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_mode {
    SEXP_TRANSPORT = 2,
    SEXP_ADVANCED = 1,
    SEXP_CANONICAL = 0,
}
impl sexp_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_mode::SEXP_TRANSPORT => 2,
            sexp_mode::SEXP_ADVANCED => 1,
            sexp_mode::SEXP_CANONICAL => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_mode {
        match value {
            2 => sexp_mode::SEXP_TRANSPORT,
            1 => sexp_mode::SEXP_ADVANCED,
            0 => sexp_mode::SEXP_CANONICAL,
            _ => panic!("Invalid value for sexp_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_mode {
    type Output = sexp_mode;
    fn add(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_mode {
    type Output = sexp_mode;
    fn sub(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_mode {
    type Output = sexp_mode;
    fn mul(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_mode {
    type Output = sexp_mode;
    fn div(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_mode {
    type Output = sexp_mode;
    fn rem(self, rhs: u32) -> sexp_mode {
        sexp_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_token {
    SEXP_CODING_END = 9,
    SEXP_TRANSPORT_START = 8,
    SEXP_DISPLAY_END = 7,
    SEXP_DISPLAY_START = 6,
    SEXP_EOF = 5,
    SEXP_LIST_END = 4,
    SEXP_LIST_START = 3,
    SEXP_COMMENT = 2,
    SEXP_DISPLAY = 1,
    SEXP_STRING = 0,
}
impl sexp_token {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_token::SEXP_CODING_END => 9,
            sexp_token::SEXP_TRANSPORT_START => 8,
            sexp_token::SEXP_DISPLAY_END => 7,
            sexp_token::SEXP_DISPLAY_START => 6,
            sexp_token::SEXP_EOF => 5,
            sexp_token::SEXP_LIST_END => 4,
            sexp_token::SEXP_LIST_START => 3,
            sexp_token::SEXP_COMMENT => 2,
            sexp_token::SEXP_DISPLAY => 1,
            sexp_token::SEXP_STRING => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_token {
        match value {
            9 => sexp_token::SEXP_CODING_END,
            8 => sexp_token::SEXP_TRANSPORT_START,
            7 => sexp_token::SEXP_DISPLAY_END,
            6 => sexp_token::SEXP_DISPLAY_START,
            5 => sexp_token::SEXP_EOF,
            4 => sexp_token::SEXP_LIST_END,
            3 => sexp_token::SEXP_LIST_START,
            2 => sexp_token::SEXP_COMMENT,
            1 => sexp_token::SEXP_DISPLAY,
            0 => sexp_token::SEXP_STRING,
            _ => panic!("Invalid value for sexp_token: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_token {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_token {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_token {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_token {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_token {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_token::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_token {
    type Output = sexp_token;
    fn add(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_token {
    type Output = sexp_token;
    fn sub(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_token {
    type Output = sexp_token;
    fn mul(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_token {
    type Output = sexp_token;
    fn div(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_token {
    type Output = sexp_token;
    fn rem(self, rhs: u32) -> sexp_token {
        sexp_token::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_decode_ctx {
    pub table: *const libc::c_schar,
    pub word: libc::c_ushort,
    pub bits: u8,
    pub padding: u8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum sexp_char_type {
    SEXP_END_CHAR = 2,
    SEXP_EOF_CHAR = 1,
    SEXP_NORMAL_CHAR = 0,
}
impl sexp_char_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            sexp_char_type::SEXP_END_CHAR => 2,
            sexp_char_type::SEXP_EOF_CHAR => 1,
            sexp_char_type::SEXP_NORMAL_CHAR => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> sexp_char_type {
        match value {
            2 => sexp_char_type::SEXP_END_CHAR,
            1 => sexp_char_type::SEXP_EOF_CHAR,
            0 => sexp_char_type::SEXP_NORMAL_CHAR,
            _ => panic!("Invalid value for sexp_char_type: {}", value),
        }
    }
}
impl AddAssign<u32> for sexp_char_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for sexp_char_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for sexp_char_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for sexp_char_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for sexp_char_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn add(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn sub(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn mul(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn div(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for sexp_char_type {
    type Output = sexp_char_type;
    fn rem(self, rhs: u32) -> sexp_char_type {
        sexp_char_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_input {
    pub f: *mut FILE,
    pub ctype: sexp_char_type,
    pub c: uint8_t,
    pub coding: *const nettle_armor,
    pub state: C2RustUnnamed,
    pub terminator: uint8_t,
    pub token: sexp_token,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub base64: base64_decode_ctx,
    pub hex: base16_decode_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_output {
    pub f: *mut FILE,
    pub line_width: u32,
    pub coding: *const nettle_armor,
    pub coding_indent: u32,
    pub prefer_hex: i32,
    pub hash: *const nettle_hash,
    pub ctx: *mut libc::c_void,
    pub base64: base64_decode_ctx,
    pub pos: u32,
    pub soft_newline: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_compound_token {
    pub type_0: sexp_token,
    pub display: nettle_buffer,
    pub string: nettle_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sexp_parser {
    pub input: *mut sexp_input,
    pub mode: sexp_mode,
    pub level: u32,
    pub transport: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conv_options {
    pub mode: sexp_mode,
    pub prefer_hex: i32,
    pub once: i32,
    pub lock: i32,
    pub width: u32,
    pub hash: *const nettle_hash,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    OPT_HELP = 303,
    OPT_LOCK = 302,
    OPT_HASH = 301,
    OPT_ONCE = 300,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::OPT_HELP => 303,
            C2RustUnnamed_0::OPT_LOCK => 302,
            C2RustUnnamed_0::OPT_HASH => 301,
            C2RustUnnamed_0::OPT_ONCE => 300,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            303 => C2RustUnnamed_0::OPT_HELP,
            302 => C2RustUnnamed_0::OPT_LOCK,
            301 => C2RustUnnamed_0::OPT_HASH,
            300 => C2RustUnnamed_0::OPT_ONCE,
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
unsafe extern "C" fn sexp_convert_item(
    mut parser: *mut sexp_parser,
    mut token: *mut sexp_compound_token,
    mut output: *mut sexp_output,
    mut mode_out: sexp_mode,
    mut indent: u32,
) {
    if mode_out as u32 == sexp_mode::SEXP_TRANSPORT as i32 as u32 {
        sexp_put_char(output, '{' as i32 as uint8_t);
        sexp_put_code_start(output, &nettle_base64);
        sexp_convert_item(
            parser,
            token,
            output,
            sexp_mode::SEXP_CANONICAL,
            0 as i32 as u32,
        );
        sexp_put_code_end(output);
        sexp_put_char(output, '}' as i32 as uint8_t);
    } else {
        match (*token).type_0 as u32 {
            4 => {
                die(b"Unmatched end of list.\n\0" as *const u8 as *const i8);
            }
            5 => {
                die(b"Unexpected end of file.\n\0" as *const u8 as *const i8);
            }
            9 => {
                die(b"Unexpected end of coding.\n\0" as *const u8 as *const i8);
            }
            3 => {
                let mut item: u32 = 0;
                sexp_put_char(output, '(' as i32 as uint8_t);
                item = 0 as i32 as u32;
                loop {
                    sexp_parse(parser, token);
                    if !((*token).type_0 as u32
                        != sexp_token::SEXP_LIST_END as i32 as u32)
                    {
                        break;
                    }
                    if mode_out as u32 == sexp_mode::SEXP_ADVANCED as i32 as u32 {
                        match item {
                            0 => {
                                if (*token).type_0 as u32
                                    == sexp_token::SEXP_COMMENT as i32 as u32
                                {
                                    indent = (*output).pos;
                                    item = item.wrapping_add(1);
                                    item;
                                }
                            }
                            1 => {
                                sexp_put_char(output, ' ' as i32 as uint8_t);
                                indent = (*output).pos;
                            }
                            _ => {
                                sexp_put_newline(output, indent);
                            }
                        }
                    }
                    sexp_convert_item(parser, token, output, mode_out, indent);
                    item = item.wrapping_add(1);
                    item;
                }
                sexp_put_char(output, ')' as i32 as uint8_t);
            }
            0 => {
                sexp_put_string(output, mode_out, &mut (*token).string);
            }
            1 => {
                sexp_put_char(output, '[' as i32 as uint8_t);
                sexp_put_string(output, mode_out, &mut (*token).display);
                sexp_put_char(output, ']' as i32 as uint8_t);
                sexp_put_string(output, mode_out, &mut (*token).string);
            }
            2 => {
                if mode_out as u32 == sexp_mode::SEXP_ADVANCED as i32 as u32 {
                    sexp_put_data(
                        output,
                        (*token).string.size as u32,
                        (*token).string.contents,
                    );
                    sexp_put_soft_newline(output, indent);
                }
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe extern "C" fn match_argument(mut given: *const i8, mut name: *const i8) -> i32 {
    if !given.is_null() && !name.is_null() {} else {
        __assert_fail(
            b"given != NULL && name != NULL\0" as *const u8 as *const i8,
            b"sexp-conv.c\0" as *const u8 as *const i8,
            220 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[i8; 47],
            >(b"int match_argument(const char *, const char *)\0"))
                .as_ptr(),
        );
    }
    'c_3835: {
        if !given.is_null() && !name.is_null() {} else {
            __assert_fail(
                b"given != NULL && name != NULL\0" as *const u8 as *const i8,
                b"sexp-conv.c\0" as *const u8 as *const i8,
                220 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[i8; 47],
                >(b"int match_argument(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (strcmp(given, name) == 0) as i32;
}
unsafe extern "C" fn parse_options(
    mut o: *mut conv_options,
    mut argc: i32,
    mut argv: *mut *mut i8,
) {
    (*o).mode = sexp_mode::SEXP_ADVANCED;
    (*o).prefer_hex = 0 as i32;
    (*o).once = 0 as i32;
    (*o).lock = 0 as i32;
    (*o).hash = 0 as *const nettle_hash;
    (*o).width = 72 as i32 as u32;
    loop {
        static mut hashes: [*const nettle_hash; 4] = unsafe {
            [
                &nettle_md5 as *const nettle_hash,
                &nettle_sha1 as *const nettle_hash,
                &nettle_sha256 as *const nettle_hash,
                0 as *const nettle_hash,
            ]
        };
        static mut options: [option; 9] = [
            {
                let mut init = option {
                    name: b"help\0" as *const u8 as *const i8,
                    has_arg: 0 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: C2RustUnnamed_0::OPT_HELP as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"version\0" as *const u8 as *const i8,
                    has_arg: 0 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: 'V' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"once\0" as *const u8 as *const i8,
                    has_arg: 0 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: C2RustUnnamed_0::OPT_ONCE as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"syntax\0" as *const u8 as *const i8,
                    has_arg: 1 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: 's' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"hash\0" as *const u8 as *const i8,
                    has_arg: 2 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: C2RustUnnamed_0::OPT_HASH as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"raw-hash\0" as *const u8 as *const i8,
                    has_arg: 2 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: C2RustUnnamed_0::OPT_HASH as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"width\0" as *const u8 as *const i8,
                    has_arg: 1 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: 'w' as i32,
                };
                init
            },
            {
                let mut init = option {
                    name: b"lock\0" as *const u8 as *const i8,
                    has_arg: 0 as i32,
                    flag: 0 as *const i32 as *mut i32,
                    val: C2RustUnnamed_0::OPT_LOCK as i32,
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
        let mut c: i32 = 0;
        let mut option_index: i32 = 0 as i32;
        let mut i: u32 = 0;
        c = getopt_long(
            argc,
            argv,
            b"Vs:w:\0" as *const u8 as *const i8,
            options.as_ptr(),
            &mut option_index,
        );
        match c {
            -1 => {
                if optind != argc {
                    die(
                        b"sexp-conv: Command line takes no arguments, only options.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                return;
            }
            63 => {
                exit(1 as i32);
            }
            119 => {
                let mut end: *mut i8 = 0 as *mut i8;
                let mut width: i32 = 0;
                if !optarg.is_null() {} else {
                    __assert_fail(
                        b"optarg != NULL\0" as *const u8 as *const i8,
                        b"sexp-conv.c\0" as *const u8 as *const i8,
                        284 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 56],
                            &[i8; 56],
                        >(b"void parse_options(struct conv_options *, int, char **)\0"))
                            .as_ptr(),
                    );
                }
                'c_4267: {
                    if !optarg.is_null() {} else {
                        __assert_fail(
                            b"optarg != NULL\0" as *const u8 as *const i8,
                            b"sexp-conv.c\0" as *const u8 as *const i8,
                            284 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 56],
                                &[i8; 56],
                            >(
                                b"void parse_options(struct conv_options *, int, char **)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                width = strtol(optarg, &mut end, 0 as i32) as i32;
                if *optarg == 0 || *end as i32 != 0 || width < 0 as i32 {
                    die(
                        b"sexp-conv: Invalid width `%s'.\n\0" as *const u8 as *const i8,
                        optarg,
                    );
                }
                (*o).width = width as u32;
            }
            115 => {
                if !((*o).hash).is_null() {
                    werror(
                        b"sexp-conv: Combining --hash and -s usually makes no sense.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                if match_argument(optarg, b"advanced\0" as *const u8 as *const i8) != 0 {
                    (*o).mode = sexp_mode::SEXP_ADVANCED;
                } else if match_argument(
                    optarg,
                    b"transport\0" as *const u8 as *const i8,
                ) != 0
                {
                    (*o).mode = sexp_mode::SEXP_TRANSPORT;
                } else if match_argument(
                    optarg,
                    b"canonical\0" as *const u8 as *const i8,
                ) != 0
                {
                    (*o).mode = sexp_mode::SEXP_CANONICAL;
                } else if match_argument(optarg, b"hex\0" as *const u8 as *const i8) != 0
                {
                    (*o).mode = sexp_mode::SEXP_ADVANCED;
                    (*o).prefer_hex = 1 as i32;
                } else {
                    die(
                        b"Available syntax variants: advanced, transport, canonical\n\0"
                            as *const u8 as *const i8,
                    );
                }
            }
            300 => {
                (*o).once = 1 as i32;
            }
            301 => {
                (*o).mode = sexp_mode::SEXP_CANONICAL;
                if optarg.is_null() {
                    (*o).hash = &nettle_sha1;
                } else {
                    i = 0 as i32 as u32;
                    loop {
                        if (hashes[i as usize]).is_null() {
                            die(
                                b"sexp_conv: Unknown hash algorithm `%s'\n\0" as *const u8
                                    as *const i8,
                                optarg,
                            );
                        }
                        if match_argument(optarg, (*hashes[i as usize]).name) != 0 {
                            (*o).hash = hashes[i as usize];
                            break;
                        } else {
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                }
            }
            302 => {
                (*o).lock = 1 as i32;
            }
            303 => {
                printf(
                    b"Usage: sexp-conv [OPTION...]\n  Conversion:     sexp-conv [OPTION...] <INPUT-SEXP\n  Fingerprinting: sexp-conv --hash=HASH <INPUT-SEXP\n\nReads an s-expression on stdin, and outputs the same\nsexp on stdout, possibly with a different syntax.\n\n       --hash[=ALGORITHM]   Outputs only the hash of the expression.\n                            Available hash algorithms:\n                            \0"
                        as *const u8 as *const i8,
                );
                i = 0 as i32 as u32;
                while !(hashes[i as usize]).is_null() {
                    if i != 0 {
                        printf(b", \0" as *const u8 as *const i8);
                    }
                    printf(
                        b"%s\0" as *const u8 as *const i8,
                        (*hashes[i as usize]).name,
                    );
                    i = i.wrapping_add(1);
                    i;
                }
                printf(
                    b" (default is sha1).\n   -s, --syntax=SYNTAX      The syntax used for the output. Available\n                            variants: advanced, hex, transport, canonical\n       --once               Process only the first s-expression.\n   -w, --width=WIDTH        Linewidth for base64 encoded data.\n                            Zero means no limit.\n       --lock               Lock output file.\n       --raw-hash           Alias for --hash, for compatibility\n                            with lsh-1.x.\n\nReport bugs to nettle-bugs@lists.lysator.liu.se.\n\0"
                        as *const u8 as *const i8,
                );
                exit(0 as i32);
            }
            86 => {
                printf(b"sexp-conv (nettle 3.10)\n\0" as *const u8 as *const i8);
                exit(0 as i32);
            }
            _ => {
                abort();
            }
        }
    };
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut options: conv_options = conv_options {
        mode: sexp_mode::SEXP_CANONICAL,
        prefer_hex: 0,
        once: 0,
        lock: 0,
        width: 0,
        hash: 0 as *const nettle_hash,
    };
    let mut input: sexp_input = sexp_input {
        f: 0 as *mut FILE,
        ctype: sexp_char_type::SEXP_NORMAL_CHAR,
        c: 0,
        coding: 0 as *const nettle_armor,
        state: C2RustUnnamed {
            base64: base64_decode_ctx {
                table: 0 as *const libc::c_schar,
                word: 0,
                bits: 0,
                padding: 0,
            },
        },
        terminator: 0,
        token: sexp_token::SEXP_STRING,
    };
    let mut parser: sexp_parser = sexp_parser {
        input: 0 as *mut sexp_input,
        mode: sexp_mode::SEXP_CANONICAL,
        level: 0,
        transport: 0,
    };
    let mut token: sexp_compound_token = sexp_compound_token {
        type_0: sexp_token::SEXP_STRING,
        display: nettle_buffer {
            contents: 0 as *mut uint8_t,
            alloc: 0,
            realloc_ctx: 0 as *mut libc::c_void,
            realloc: None,
            size: 0,
        },
        string: nettle_buffer {
            contents: 0 as *mut uint8_t,
            alloc: 0,
            realloc_ctx: 0 as *mut libc::c_void,
            realloc: None,
            size: 0,
        },
    };
    let mut output: sexp_output = sexp_output {
        f: 0 as *mut FILE,
        line_width: 0,
        coding: 0 as *const nettle_armor,
        coding_indent: 0,
        prefer_hex: 0,
        hash: 0 as *const nettle_hash,
        ctx: 0 as *mut libc::c_void,
        base64: base64_decode_ctx {
            table: 0 as *const libc::c_schar,
            word: 0,
            bits: 0,
            padding: 0,
        },
        pos: 0,
        soft_newline: 0,
    };
    parse_options(&mut options, argc, argv);
    sexp_input_init(&mut input, stdin);
    sexp_parse_init(&mut parser, &mut input, sexp_mode::SEXP_ADVANCED);
    sexp_compound_token_init(&mut token);
    sexp_output_init(&mut output, stdout, options.width, options.prefer_hex);
    if options.lock != 0 {
        let mut fl: flock = flock {
            l_type: 0,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        memset(
            &mut fl as *mut flock as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<flock>() as u64,
        );
        fl.l_type = 1 as i32 as libc::c_short;
        fl.l_whence = 0 as i32 as libc::c_short;
        fl.l_start = 0 as i32 as __off_t;
        fl.l_len = 0 as i32 as __off_t;
        if fcntl(1 as i32, 7 as i32, &mut fl as *mut flock) == -(1 as i32) {
            die(
                b"Locking output file failed: %s\n\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
    }
    if !(options.hash).is_null() {
        let mut ctx: *mut libc::c_void = xalloc((*options.hash).context_size as size_t);
        sexp_output_hash_init(&mut output, options.hash, ctx);
    }
    sexp_get_char(&mut input);
    sexp_parse(&mut parser, &mut token);
    if token.type_0 as u32 == sexp_token::SEXP_EOF as i32 as u32 {
        if options.once != 0 {
            die(b"sexp-conv: No input expression.\n\0" as *const u8 as *const i8);
        }
        return 0 as i32;
    }
    loop {
        sexp_convert_item(
            &mut parser,
            &mut token,
            &mut output,
            options.mode,
            0 as i32 as u32,
        );
        if !(options.hash).is_null() {
            sexp_put_digest(&mut output);
            sexp_put_newline(&mut output, 0 as i32 as u32);
        } else if options.mode as u32 != sexp_mode::SEXP_CANONICAL as i32 as u32 {
            sexp_put_newline(&mut output, 0 as i32 as u32);
        }
        sexp_parse(&mut parser, &mut token);
        if !(options.once == 0
            && token.type_0 as u32 != sexp_token::SEXP_EOF as i32 as u32)
        {
            break;
        }
    }
    sexp_compound_token_clear(&mut token);
    if fflush(output.f) < 0 as i32 {
        die(
            b"Final fflush failed: %s.\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    return 0 as i32;
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