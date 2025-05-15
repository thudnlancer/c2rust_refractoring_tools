use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type epoll_event;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    fn exit(_: i32) -> !;
    fn close(__fd: i32) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const i8) -> i32;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn _exit(_: i32) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const i8) -> i32;
    static mut optarg: *mut i8;
    static mut opterr: i32;
    static mut optopt: i32;
    fn gethostname(__name: *mut i8, __len: size_t) -> i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn getopt_long(
        ___argc: i32,
        ___argv: *const *mut i8,
        __shortopts: *const i8,
        __longopts: *const option,
        __longind: *mut i32,
    ) -> i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn uname(__name: *mut utsname) -> i32;
    fn core_loop(ctx: *mut context) -> rstatus_t;
    fn core_stop(ctx: *mut context);
    fn core_start(nci: *mut instance) -> *mut context;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn _nc_atoi(line: *const uint8_t, n: size_t) -> i32;
    fn nc_valid_port(n: i32) -> bool;
    fn log_init(level: i32, filename: *const i8) -> i32;
    fn log_deinit();
    fn log_loggable(level: i32) -> i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn _log_stderr(fmt: *const i8, _: ...);
    fn stats_describe();
    fn conf_create(filename: *const i8) -> *mut conf;
    fn conf_destroy(cf: *mut conf);
    fn signal_init() -> rstatus_t;
    fn signal_deinit();
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __mode_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
pub type pid_t = __pid_t;
pub type int64_t = __int64_t;
pub type pthread_t = u64;
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [i8; 65],
    pub nodename: [i8; 65],
    pub release: [i8; 65],
    pub version: [i8; 65],
    pub machine: [i8; 65],
    pub domainname: [i8; 65],
}
pub type rstatus_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: i32,
    pub timeout: i32,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: i32,
    pub event: *mut epoll_event,
    pub nevent: i32,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option<unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: i32,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: i32,
    pub service_str: string,
    pub service: string,
    pub source_str: string,
    pub source: string,
    pub version_str: string,
    pub version: string,
    pub uptime_str: string,
    pub timestamp_str: string,
    pub ntotal_conn_str: string,
    pub ncurr_conn_str: string,
    pub aggregate: i32,
    pub updated: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf {
    pub fname: *const i8,
    pub fh: *mut FILE,
    pub arg: array,
    pub pool: array,
    pub depth: uint32_t,
    pub parser: yaml_parser_t,
    pub event: yaml_event_t,
    pub token: yaml_token_t,
    #[bitfield(name = "seq", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "valid_parser", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "valid_event", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "valid_token", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "sound", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "parsed", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "6..=6")]
    pub seq_valid_parser_valid_event_valid_token_sound_parsed_valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type yaml_token_t = yaml_token_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
pub type yaml_mark_t = yaml_mark_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_6,
    pub alias: C2RustUnnamed_5,
    pub anchor: C2RustUnnamed_4,
    pub tag: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub version_directive: C2RustUnnamed_1,
    pub tag_directive: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_char_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: i32,
    pub minor: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_scalar_style_t = yaml_scalar_style_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_scalar_style_e {
    YAML_FOLDED_SCALAR_STYLE = 5,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_ANY_SCALAR_STYLE = 0,
}
impl yaml_scalar_style_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_scalar_style_e::YAML_FOLDED_SCALAR_STYLE => 5,
            yaml_scalar_style_e::YAML_LITERAL_SCALAR_STYLE => 4,
            yaml_scalar_style_e::YAML_DOUBLE_QUOTED_SCALAR_STYLE => 3,
            yaml_scalar_style_e::YAML_SINGLE_QUOTED_SCALAR_STYLE => 2,
            yaml_scalar_style_e::YAML_PLAIN_SCALAR_STYLE => 1,
            yaml_scalar_style_e::YAML_ANY_SCALAR_STYLE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_scalar_style_e {
        match value {
            5 => yaml_scalar_style_e::YAML_FOLDED_SCALAR_STYLE,
            4 => yaml_scalar_style_e::YAML_LITERAL_SCALAR_STYLE,
            3 => yaml_scalar_style_e::YAML_DOUBLE_QUOTED_SCALAR_STYLE,
            2 => yaml_scalar_style_e::YAML_SINGLE_QUOTED_SCALAR_STYLE,
            1 => yaml_scalar_style_e::YAML_PLAIN_SCALAR_STYLE,
            0 => yaml_scalar_style_e::YAML_ANY_SCALAR_STYLE,
            _ => panic!("Invalid value for yaml_scalar_style_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_scalar_style_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_scalar_style_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_scalar_style_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_scalar_style_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_scalar_style_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_scalar_style_e {
    type Output = yaml_scalar_style_e;
    fn add(self, rhs: u32) -> yaml_scalar_style_e {
        yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_scalar_style_e {
    type Output = yaml_scalar_style_e;
    fn sub(self, rhs: u32) -> yaml_scalar_style_e {
        yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_scalar_style_e {
    type Output = yaml_scalar_style_e;
    fn mul(self, rhs: u32) -> yaml_scalar_style_e {
        yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_scalar_style_e {
    type Output = yaml_scalar_style_e;
    fn div(self, rhs: u32) -> yaml_scalar_style_e {
        yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_scalar_style_e {
    type Output = yaml_scalar_style_e;
    fn rem(self, rhs: u32) -> yaml_scalar_style_e {
        yaml_scalar_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_encoding_t = yaml_encoding_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_encoding_e {
    YAML_ANY_ENCODING,
    YAML_UTF8_ENCODING,
    YAML_UTF16LE_ENCODING,
    YAML_UTF16BE_ENCODING,
}
impl yaml_encoding_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_encoding_e::YAML_ANY_ENCODING => 0,
            yaml_encoding_e::YAML_UTF8_ENCODING => 1,
            yaml_encoding_e::YAML_UTF16LE_ENCODING => 2,
            yaml_encoding_e::YAML_UTF16BE_ENCODING => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_encoding_e {
        match value {
            0 => yaml_encoding_e::YAML_ANY_ENCODING,
            1 => yaml_encoding_e::YAML_UTF8_ENCODING,
            2 => yaml_encoding_e::YAML_UTF16LE_ENCODING,
            3 => yaml_encoding_e::YAML_UTF16BE_ENCODING,
            _ => panic!("Invalid value for yaml_encoding_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_encoding_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_encoding_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_encoding_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_encoding_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_encoding_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_encoding_e {
    type Output = yaml_encoding_e;
    fn add(self, rhs: u32) -> yaml_encoding_e {
        yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_encoding_e {
    type Output = yaml_encoding_e;
    fn sub(self, rhs: u32) -> yaml_encoding_e {
        yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_encoding_e {
    type Output = yaml_encoding_e;
    fn mul(self, rhs: u32) -> yaml_encoding_e {
        yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_encoding_e {
    type Output = yaml_encoding_e;
    fn div(self, rhs: u32) -> yaml_encoding_e {
        yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_encoding_e {
    type Output = yaml_encoding_e;
    fn rem(self, rhs: u32) -> yaml_encoding_e {
        yaml_encoding_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type yaml_token_type_t = yaml_token_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_token_type_e {
    YAML_SCALAR_TOKEN = 21,
    YAML_TAG_TOKEN = 20,
    YAML_ANCHOR_TOKEN = 19,
    YAML_ALIAS_TOKEN = 18,
    YAML_VALUE_TOKEN = 17,
    YAML_KEY_TOKEN = 16,
    YAML_FLOW_ENTRY_TOKEN = 15,
    YAML_BLOCK_ENTRY_TOKEN = 14,
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    YAML_BLOCK_END_TOKEN = 9,
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    YAML_DOCUMENT_END_TOKEN = 6,
    YAML_DOCUMENT_START_TOKEN = 5,
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    YAML_STREAM_END_TOKEN = 2,
    YAML_STREAM_START_TOKEN = 1,
    YAML_NO_TOKEN = 0,
}
impl yaml_token_type_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_token_type_e::YAML_SCALAR_TOKEN => 21,
            yaml_token_type_e::YAML_TAG_TOKEN => 20,
            yaml_token_type_e::YAML_ANCHOR_TOKEN => 19,
            yaml_token_type_e::YAML_ALIAS_TOKEN => 18,
            yaml_token_type_e::YAML_VALUE_TOKEN => 17,
            yaml_token_type_e::YAML_KEY_TOKEN => 16,
            yaml_token_type_e::YAML_FLOW_ENTRY_TOKEN => 15,
            yaml_token_type_e::YAML_BLOCK_ENTRY_TOKEN => 14,
            yaml_token_type_e::YAML_FLOW_MAPPING_END_TOKEN => 13,
            yaml_token_type_e::YAML_FLOW_MAPPING_START_TOKEN => 12,
            yaml_token_type_e::YAML_FLOW_SEQUENCE_END_TOKEN => 11,
            yaml_token_type_e::YAML_FLOW_SEQUENCE_START_TOKEN => 10,
            yaml_token_type_e::YAML_BLOCK_END_TOKEN => 9,
            yaml_token_type_e::YAML_BLOCK_MAPPING_START_TOKEN => 8,
            yaml_token_type_e::YAML_BLOCK_SEQUENCE_START_TOKEN => 7,
            yaml_token_type_e::YAML_DOCUMENT_END_TOKEN => 6,
            yaml_token_type_e::YAML_DOCUMENT_START_TOKEN => 5,
            yaml_token_type_e::YAML_TAG_DIRECTIVE_TOKEN => 4,
            yaml_token_type_e::YAML_VERSION_DIRECTIVE_TOKEN => 3,
            yaml_token_type_e::YAML_STREAM_END_TOKEN => 2,
            yaml_token_type_e::YAML_STREAM_START_TOKEN => 1,
            yaml_token_type_e::YAML_NO_TOKEN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_token_type_e {
        match value {
            21 => yaml_token_type_e::YAML_SCALAR_TOKEN,
            20 => yaml_token_type_e::YAML_TAG_TOKEN,
            19 => yaml_token_type_e::YAML_ANCHOR_TOKEN,
            18 => yaml_token_type_e::YAML_ALIAS_TOKEN,
            17 => yaml_token_type_e::YAML_VALUE_TOKEN,
            16 => yaml_token_type_e::YAML_KEY_TOKEN,
            15 => yaml_token_type_e::YAML_FLOW_ENTRY_TOKEN,
            14 => yaml_token_type_e::YAML_BLOCK_ENTRY_TOKEN,
            13 => yaml_token_type_e::YAML_FLOW_MAPPING_END_TOKEN,
            12 => yaml_token_type_e::YAML_FLOW_MAPPING_START_TOKEN,
            11 => yaml_token_type_e::YAML_FLOW_SEQUENCE_END_TOKEN,
            10 => yaml_token_type_e::YAML_FLOW_SEQUENCE_START_TOKEN,
            9 => yaml_token_type_e::YAML_BLOCK_END_TOKEN,
            8 => yaml_token_type_e::YAML_BLOCK_MAPPING_START_TOKEN,
            7 => yaml_token_type_e::YAML_BLOCK_SEQUENCE_START_TOKEN,
            6 => yaml_token_type_e::YAML_DOCUMENT_END_TOKEN,
            5 => yaml_token_type_e::YAML_DOCUMENT_START_TOKEN,
            4 => yaml_token_type_e::YAML_TAG_DIRECTIVE_TOKEN,
            3 => yaml_token_type_e::YAML_VERSION_DIRECTIVE_TOKEN,
            2 => yaml_token_type_e::YAML_STREAM_END_TOKEN,
            1 => yaml_token_type_e::YAML_STREAM_START_TOKEN,
            0 => yaml_token_type_e::YAML_NO_TOKEN,
            _ => panic!("Invalid value for yaml_token_type_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_token_type_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_token_type_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_token_type_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_token_type_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_token_type_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_token_type_e {
    type Output = yaml_token_type_e;
    fn add(self, rhs: u32) -> yaml_token_type_e {
        yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_token_type_e {
    type Output = yaml_token_type_e;
    fn sub(self, rhs: u32) -> yaml_token_type_e {
        yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_token_type_e {
    type Output = yaml_token_type_e;
    fn mul(self, rhs: u32) -> yaml_token_type_e {
        yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_token_type_e {
    type Output = yaml_token_type_e;
    fn div(self, rhs: u32) -> yaml_token_type_e {
        yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_token_type_e {
    type Output = yaml_token_type_e;
    fn rem(self, rhs: u32) -> yaml_token_type_e {
        yaml_token_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type yaml_event_t = yaml_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_event_s {
    pub type_0: yaml_event_type_t,
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub stream_start: C2RustUnnamed_15,
    pub document_start: C2RustUnnamed_13,
    pub document_end: C2RustUnnamed_12,
    pub alias: C2RustUnnamed_11,
    pub scalar: C2RustUnnamed_10,
    pub sequence_start: C2RustUnnamed_9,
    pub mapping_start: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_mapping_style_t,
}
pub type yaml_mapping_style_t = yaml_mapping_style_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_mapping_style_e {
    YAML_FLOW_MAPPING_STYLE = 2,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_ANY_MAPPING_STYLE = 0,
}
impl yaml_mapping_style_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_mapping_style_e::YAML_FLOW_MAPPING_STYLE => 2,
            yaml_mapping_style_e::YAML_BLOCK_MAPPING_STYLE => 1,
            yaml_mapping_style_e::YAML_ANY_MAPPING_STYLE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_mapping_style_e {
        match value {
            2 => yaml_mapping_style_e::YAML_FLOW_MAPPING_STYLE,
            1 => yaml_mapping_style_e::YAML_BLOCK_MAPPING_STYLE,
            0 => yaml_mapping_style_e::YAML_ANY_MAPPING_STYLE,
            _ => panic!("Invalid value for yaml_mapping_style_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_mapping_style_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_mapping_style_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_mapping_style_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_mapping_style_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_mapping_style_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_mapping_style_e {
    type Output = yaml_mapping_style_e;
    fn add(self, rhs: u32) -> yaml_mapping_style_e {
        yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_mapping_style_e {
    type Output = yaml_mapping_style_e;
    fn sub(self, rhs: u32) -> yaml_mapping_style_e {
        yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_mapping_style_e {
    type Output = yaml_mapping_style_e;
    fn mul(self, rhs: u32) -> yaml_mapping_style_e {
        yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_mapping_style_e {
    type Output = yaml_mapping_style_e;
    fn div(self, rhs: u32) -> yaml_mapping_style_e {
        yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_mapping_style_e {
    type Output = yaml_mapping_style_e;
    fn rem(self, rhs: u32) -> yaml_mapping_style_e {
        yaml_mapping_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_sequence_style_t,
}
pub type yaml_sequence_style_t = yaml_sequence_style_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_sequence_style_e {
    YAML_FLOW_SEQUENCE_STYLE = 2,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_ANY_SEQUENCE_STYLE = 0,
}
impl yaml_sequence_style_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_sequence_style_e::YAML_FLOW_SEQUENCE_STYLE => 2,
            yaml_sequence_style_e::YAML_BLOCK_SEQUENCE_STYLE => 1,
            yaml_sequence_style_e::YAML_ANY_SEQUENCE_STYLE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_sequence_style_e {
        match value {
            2 => yaml_sequence_style_e::YAML_FLOW_SEQUENCE_STYLE,
            1 => yaml_sequence_style_e::YAML_BLOCK_SEQUENCE_STYLE,
            0 => yaml_sequence_style_e::YAML_ANY_SEQUENCE_STYLE,
            _ => panic!("Invalid value for yaml_sequence_style_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_sequence_style_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_sequence_style_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_sequence_style_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_sequence_style_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_sequence_style_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_sequence_style_e {
    type Output = yaml_sequence_style_e;
    fn add(self, rhs: u32) -> yaml_sequence_style_e {
        yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_sequence_style_e {
    type Output = yaml_sequence_style_e;
    fn sub(self, rhs: u32) -> yaml_sequence_style_e {
        yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_sequence_style_e {
    type Output = yaml_sequence_style_e;
    fn mul(self, rhs: u32) -> yaml_sequence_style_e {
        yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_sequence_style_e {
    type Output = yaml_sequence_style_e;
    fn div(self, rhs: u32) -> yaml_sequence_style_e {
        yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_sequence_style_e {
    type Output = yaml_sequence_style_e;
    fn rem(self, rhs: u32) -> yaml_sequence_style_e {
        yaml_sequence_style_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: i32,
    pub quoted_implicit: i32,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub implicit: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub implicit: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: i32,
    pub minor: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_type_t = yaml_event_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_event_type_e {
    YAML_MAPPING_END_EVENT = 10,
    YAML_MAPPING_START_EVENT = 9,
    YAML_SEQUENCE_END_EVENT = 8,
    YAML_SEQUENCE_START_EVENT = 7,
    YAML_SCALAR_EVENT = 6,
    YAML_ALIAS_EVENT = 5,
    YAML_DOCUMENT_END_EVENT = 4,
    YAML_DOCUMENT_START_EVENT = 3,
    YAML_STREAM_END_EVENT = 2,
    YAML_STREAM_START_EVENT = 1,
    YAML_NO_EVENT = 0,
}
impl yaml_event_type_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_event_type_e::YAML_MAPPING_END_EVENT => 10,
            yaml_event_type_e::YAML_MAPPING_START_EVENT => 9,
            yaml_event_type_e::YAML_SEQUENCE_END_EVENT => 8,
            yaml_event_type_e::YAML_SEQUENCE_START_EVENT => 7,
            yaml_event_type_e::YAML_SCALAR_EVENT => 6,
            yaml_event_type_e::YAML_ALIAS_EVENT => 5,
            yaml_event_type_e::YAML_DOCUMENT_END_EVENT => 4,
            yaml_event_type_e::YAML_DOCUMENT_START_EVENT => 3,
            yaml_event_type_e::YAML_STREAM_END_EVENT => 2,
            yaml_event_type_e::YAML_STREAM_START_EVENT => 1,
            yaml_event_type_e::YAML_NO_EVENT => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_event_type_e {
        match value {
            10 => yaml_event_type_e::YAML_MAPPING_END_EVENT,
            9 => yaml_event_type_e::YAML_MAPPING_START_EVENT,
            8 => yaml_event_type_e::YAML_SEQUENCE_END_EVENT,
            7 => yaml_event_type_e::YAML_SEQUENCE_START_EVENT,
            6 => yaml_event_type_e::YAML_SCALAR_EVENT,
            5 => yaml_event_type_e::YAML_ALIAS_EVENT,
            4 => yaml_event_type_e::YAML_DOCUMENT_END_EVENT,
            3 => yaml_event_type_e::YAML_DOCUMENT_START_EVENT,
            2 => yaml_event_type_e::YAML_STREAM_END_EVENT,
            1 => yaml_event_type_e::YAML_STREAM_START_EVENT,
            0 => yaml_event_type_e::YAML_NO_EVENT,
            _ => panic!("Invalid value for yaml_event_type_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_event_type_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_event_type_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_event_type_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_event_type_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_event_type_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_event_type_e {
    type Output = yaml_event_type_e;
    fn add(self, rhs: u32) -> yaml_event_type_e {
        yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_event_type_e {
    type Output = yaml_event_type_e;
    fn sub(self, rhs: u32) -> yaml_event_type_e {
        yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_event_type_e {
    type Output = yaml_event_type_e;
    fn mul(self, rhs: u32) -> yaml_event_type_e {
        yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_event_type_e {
    type Output = yaml_event_type_e;
    fn div(self, rhs: u32) -> yaml_event_type_e {
        yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_event_type_e {
    type Output = yaml_event_type_e;
    fn rem(self, rhs: u32) -> yaml_event_type_e {
        yaml_event_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_parser_s {
    pub error: yaml_error_type_t,
    pub problem: *const i8,
    pub problem_offset: size_t,
    pub problem_value: i32,
    pub problem_mark: yaml_mark_t,
    pub context: *const i8,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut libc::c_void,
    pub input: C2RustUnnamed_33,
    pub eof: i32,
    pub buffer: C2RustUnnamed_32,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: i32,
    pub stream_end_produced: i32,
    pub flow_level: i32,
    pub tokens: C2RustUnnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: i32,
    pub indents: C2RustUnnamed_29,
    pub indent: i32,
    pub simple_key_allowed: i32,
    pub simple_keys: C2RustUnnamed_28,
    pub states: C2RustUnnamed_27,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_26,
    pub tag_directives: C2RustUnnamed_25,
    pub aliases: C2RustUnnamed_24,
    pub document: *mut yaml_document_t,
}
pub type yaml_document_t = yaml_document_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_17,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_16,
    pub start_implicit: i32,
    pub end_implicit: i32,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_s {
    pub type_0: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: C2RustUnnamed_18,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub scalar: C2RustUnnamed_23,
    pub sequence: C2RustUnnamed_21,
    pub mapping: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub pairs: C2RustUnnamed_20,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
pub type yaml_node_pair_t = yaml_node_pair_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_pair_s {
    pub key: i32,
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub items: C2RustUnnamed_22,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_type_t = yaml_node_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_node_type_e {
    YAML_MAPPING_NODE = 3,
    YAML_SEQUENCE_NODE = 2,
    YAML_SCALAR_NODE = 1,
    YAML_NO_NODE = 0,
}
impl yaml_node_type_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_node_type_e::YAML_MAPPING_NODE => 3,
            yaml_node_type_e::YAML_SEQUENCE_NODE => 2,
            yaml_node_type_e::YAML_SCALAR_NODE => 1,
            yaml_node_type_e::YAML_NO_NODE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_node_type_e {
        match value {
            3 => yaml_node_type_e::YAML_MAPPING_NODE,
            2 => yaml_node_type_e::YAML_SEQUENCE_NODE,
            1 => yaml_node_type_e::YAML_SCALAR_NODE,
            0 => yaml_node_type_e::YAML_NO_NODE,
            _ => panic!("Invalid value for yaml_node_type_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_node_type_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_node_type_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_node_type_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_node_type_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_node_type_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_node_type_e {
    type Output = yaml_node_type_e;
    fn add(self, rhs: u32) -> yaml_node_type_e {
        yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_node_type_e {
    type Output = yaml_node_type_e;
    fn sub(self, rhs: u32) -> yaml_node_type_e {
        yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_node_type_e {
    type Output = yaml_node_type_e;
    fn mul(self, rhs: u32) -> yaml_node_type_e {
        yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_node_type_e {
    type Output = yaml_node_type_e;
    fn div(self, rhs: u32) -> yaml_node_type_e {
        yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_node_type_e {
    type Output = yaml_node_type_e;
    fn rem(self, rhs: u32) -> yaml_node_type_e {
        yaml_node_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: i32,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
pub type yaml_parser_state_t = yaml_parser_state_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_parser_state_e {
    YAML_PARSE_END_STATE = 23,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE = 22,
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE = 21,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE = 20,
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE = 19,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE = 18,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE = 17,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE = 16,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE = 15,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE = 14,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE = 13,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE = 12,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE = 11,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE = 10,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE = 9,
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE = 8,
    YAML_PARSE_FLOW_NODE_STATE = 7,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE = 6,
    YAML_PARSE_BLOCK_NODE_STATE = 5,
    YAML_PARSE_DOCUMENT_END_STATE = 4,
    YAML_PARSE_DOCUMENT_CONTENT_STATE = 3,
    YAML_PARSE_DOCUMENT_START_STATE = 2,
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE = 1,
    YAML_PARSE_STREAM_START_STATE = 0,
}
impl yaml_parser_state_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_parser_state_e::YAML_PARSE_END_STATE => 23,
            yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE => 22,
            yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_VALUE_STATE => 21,
            yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_KEY_STATE => 20,
            yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE => 19,
            yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE => 18,
            yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE => 17,
            yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE => 16,
            yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE => 15,
            yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE => 14,
            yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_VALUE_STATE => 13,
            yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_KEY_STATE => 12,
            yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE => 11,
            yaml_parser_state_e::YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE => 10,
            yaml_parser_state_e::YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE => 9,
            yaml_parser_state_e::YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE => 8,
            yaml_parser_state_e::YAML_PARSE_FLOW_NODE_STATE => 7,
            yaml_parser_state_e::YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE => 6,
            yaml_parser_state_e::YAML_PARSE_BLOCK_NODE_STATE => 5,
            yaml_parser_state_e::YAML_PARSE_DOCUMENT_END_STATE => 4,
            yaml_parser_state_e::YAML_PARSE_DOCUMENT_CONTENT_STATE => 3,
            yaml_parser_state_e::YAML_PARSE_DOCUMENT_START_STATE => 2,
            yaml_parser_state_e::YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE => 1,
            yaml_parser_state_e::YAML_PARSE_STREAM_START_STATE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_parser_state_e {
        match value {
            23 => yaml_parser_state_e::YAML_PARSE_END_STATE,
            22 => yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE,
            21 => yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_VALUE_STATE,
            20 => yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_KEY_STATE,
            19 => yaml_parser_state_e::YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE,
            18 => yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE,
            17 => yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE,
            16 => yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE,
            15 => yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE,
            14 => yaml_parser_state_e::YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE,
            13 => yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_VALUE_STATE,
            12 => yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_KEY_STATE,
            11 => yaml_parser_state_e::YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE,
            10 => yaml_parser_state_e::YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE,
            9 => yaml_parser_state_e::YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE,
            8 => yaml_parser_state_e::YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE,
            7 => yaml_parser_state_e::YAML_PARSE_FLOW_NODE_STATE,
            6 => yaml_parser_state_e::YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE,
            5 => yaml_parser_state_e::YAML_PARSE_BLOCK_NODE_STATE,
            4 => yaml_parser_state_e::YAML_PARSE_DOCUMENT_END_STATE,
            3 => yaml_parser_state_e::YAML_PARSE_DOCUMENT_CONTENT_STATE,
            2 => yaml_parser_state_e::YAML_PARSE_DOCUMENT_START_STATE,
            1 => yaml_parser_state_e::YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE,
            0 => yaml_parser_state_e::YAML_PARSE_STREAM_START_STATE,
            _ => panic!("Invalid value for yaml_parser_state_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_parser_state_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_parser_state_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_parser_state_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_parser_state_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_parser_state_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_parser_state_e {
    type Output = yaml_parser_state_e;
    fn add(self, rhs: u32) -> yaml_parser_state_e {
        yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_parser_state_e {
    type Output = yaml_parser_state_e;
    fn sub(self, rhs: u32) -> yaml_parser_state_e {
        yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_parser_state_e {
    type Output = yaml_parser_state_e;
    fn mul(self, rhs: u32) -> yaml_parser_state_e {
        yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_parser_state_e {
    type Output = yaml_parser_state_e;
    fn div(self, rhs: u32) -> yaml_parser_state_e {
        yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_parser_state_e {
    type Output = yaml_parser_state_e;
    fn rem(self, rhs: u32) -> yaml_parser_state_e {
        yaml_parser_state_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: i32,
    pub required: i32,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub string: C2RustUnnamed_34,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub start: *const u8,
    pub end: *const u8,
    pub current: *const u8,
}
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut u8,
    size_t,
    *mut size_t,
) -> i32;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_error_type_e {
    YAML_EMITTER_ERROR = 7,
    YAML_WRITER_ERROR = 6,
    YAML_COMPOSER_ERROR = 5,
    YAML_PARSER_ERROR = 4,
    YAML_SCANNER_ERROR = 3,
    YAML_READER_ERROR = 2,
    YAML_MEMORY_ERROR = 1,
    YAML_NO_ERROR = 0,
}
impl yaml_error_type_e {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            yaml_error_type_e::YAML_EMITTER_ERROR => 7,
            yaml_error_type_e::YAML_WRITER_ERROR => 6,
            yaml_error_type_e::YAML_COMPOSER_ERROR => 5,
            yaml_error_type_e::YAML_PARSER_ERROR => 4,
            yaml_error_type_e::YAML_SCANNER_ERROR => 3,
            yaml_error_type_e::YAML_READER_ERROR => 2,
            yaml_error_type_e::YAML_MEMORY_ERROR => 1,
            yaml_error_type_e::YAML_NO_ERROR => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> yaml_error_type_e {
        match value {
            7 => yaml_error_type_e::YAML_EMITTER_ERROR,
            6 => yaml_error_type_e::YAML_WRITER_ERROR,
            5 => yaml_error_type_e::YAML_COMPOSER_ERROR,
            4 => yaml_error_type_e::YAML_PARSER_ERROR,
            3 => yaml_error_type_e::YAML_SCANNER_ERROR,
            2 => yaml_error_type_e::YAML_READER_ERROR,
            1 => yaml_error_type_e::YAML_MEMORY_ERROR,
            0 => yaml_error_type_e::YAML_NO_ERROR,
            _ => panic!("Invalid value for yaml_error_type_e: {}", value),
        }
    }
}
impl AddAssign<u32> for yaml_error_type_e {
    fn add_assign(&mut self, rhs: u32) {
        *self = yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for yaml_error_type_e {
    fn sub_assign(&mut self, rhs: u32) {
        *self = yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for yaml_error_type_e {
    fn mul_assign(&mut self, rhs: u32) {
        *self = yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for yaml_error_type_e {
    fn div_assign(&mut self, rhs: u32) {
        *self = yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for yaml_error_type_e {
    fn rem_assign(&mut self, rhs: u32) {
        *self = yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for yaml_error_type_e {
    type Output = yaml_error_type_e;
    fn add(self, rhs: u32) -> yaml_error_type_e {
        yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for yaml_error_type_e {
    type Output = yaml_error_type_e;
    fn sub(self, rhs: u32) -> yaml_error_type_e {
        yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for yaml_error_type_e {
    type Output = yaml_error_type_e;
    fn mul(self, rhs: u32) -> yaml_error_type_e {
        yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for yaml_error_type_e {
    type Output = yaml_error_type_e;
    fn div(self, rhs: u32) -> yaml_error_type_e {
        yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for yaml_error_type_e {
    type Output = yaml_error_type_e;
    fn rem(self, rhs: u32) -> yaml_error_type_e {
        yaml_error_type_e::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instance {
    pub ctx: *mut context,
    pub log_level: i32,
    pub log_filename: *const i8,
    pub conf_filename: *const i8,
    pub stats_port: uint16_t,
    pub stats_interval: i32,
    pub stats_addr: *const i8,
    pub hostname: [i8; 256],
    pub mbuf_chunk_size: size_t,
    pub pid: pid_t,
    pub pid_filename: *const i8,
    #[bitfield(name = "pidfile", ty = "libc::c_uint", bits = "0..=0")]
    pub pidfile: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
static mut show_help: i32 = 0;
static mut show_version: i32 = 0;
static mut test_conf: i32 = 0;
static mut daemonize: i32 = 0;
static mut describe_stats: i32 = 0;
static mut long_options: [option; 14] = [
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
            name: b"version\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"test-conf\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"daemonize\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"describe-stats\0" as *const u8 as *const i8,
            has_arg: 0 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"conf-file\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-port\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-interval\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stats-addr\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"pid-file\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mbuf-size\0" as *const u8 as *const i8,
            has_arg: 1 as i32,
            flag: 0 as *const i32 as *mut i32,
            val: 'm' as i32,
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
static mut short_options: [i8; 22] = unsafe {
    *::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"hVtdDv:o:c:s:i:a:p:m:\0")
};
unsafe extern "C" fn nc_daemonize(mut dump_core: i32) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut pid: pid_t = 0;
    let mut sid: pid_t = 0;
    let mut fd: i32 = 0;
    pid = fork();
    match pid {
        -1 => {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc.c\0" as *const u8 as *const i8,
                    83 as i32,
                    0 as i32,
                    b"fork() failed: %s\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as i32);
        }
        0 => {}
        _ => {
            _exit(0 as i32);
        }
    }
    sid = setsid();
    if sid < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                98 as i32,
                0 as i32,
                b"setsid() failed: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    if signal(
        1 as i32,
        ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as i32 as libc::intptr_t),
    )
        == ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as i32) as libc::intptr_t)
    {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                103 as i32,
                0 as i32,
                b"signal(SIGHUP, SIG_IGN) failed: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    pid = fork();
    match pid {
        -1 => {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc.c\0" as *const u8 as *const i8,
                    110 as i32,
                    0 as i32,
                    b"fork() failed: %s\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as i32);
        }
        0 => {}
        _ => {
            _exit(0 as i32);
        }
    }
    if dump_core == 0 as i32 {
        status = chdir(b"/\0" as *const u8 as *const i8);
        if status < 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc.c\0" as *const u8 as *const i8,
                    127 as i32,
                    0 as i32,
                    b"chdir(\"/\") failed: %s\0" as *const u8 as *const i8,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as i32);
        }
    }
    umask(0 as i32 as __mode_t);
    fd = open(b"/dev/null\0" as *const u8 as *const i8, 0o2 as i32);
    if fd < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                139 as i32,
                0 as i32,
                b"open(\"/dev/null\") failed: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    status = dup2(fd, 0 as i32);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                145 as i32,
                0 as i32,
                b"dup2(%d, STDIN) failed: %s\0" as *const u8 as *const i8,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as i32);
    }
    status = dup2(fd, 1 as i32);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                152 as i32,
                0 as i32,
                b"dup2(%d, STDOUT) failed: %s\0" as *const u8 as *const i8,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as i32);
    }
    status = dup2(fd, 2 as i32);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                159 as i32,
                0 as i32,
                b"dup2(%d, STDERR) failed: %s\0" as *const u8 as *const i8,
                fd,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as i32);
    }
    if fd > 2 as i32 {
        status = close(fd);
        if status < 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc.c\0" as *const u8 as *const i8,
                    167 as i32,
                    0 as i32,
                    b"close(%d) failed: %s\0" as *const u8 as *const i8,
                    fd,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn nc_print_run(mut nci: *const instance) {
    let mut status: i32 = 0;
    let mut name: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    status = uname(&mut name);
    if status < 0 as i32 {
        _log(
            b"nc.c\0" as *const u8 as *const i8,
            183 as i32,
            0 as i32,
            b"nutcracker-%s started on pid %d\0" as *const u8 as *const i8,
            b"0.5.0\0" as *const u8 as *const i8,
            (*nci).pid,
        );
    } else {
        _log(
            b"nc.c\0" as *const u8 as *const i8,
            187 as i32,
            0 as i32,
            b"nutcracker-%s built for %s %s %s started on pid %d\0" as *const u8
                as *const i8,
            b"0.5.0\0" as *const u8 as *const i8,
            (name.sysname).as_mut_ptr(),
            (name.release).as_mut_ptr(),
            (name.machine).as_mut_ptr(),
            (*nci).pid,
        );
    }
    _log(
        b"nc.c\0" as *const u8 as *const i8,
        192 as i32,
        0 as i32,
        b"run, rabbit run / dig that hole, forget the sun / and when at last the work is done / don't sit down / it's time to dig another one\0"
            as *const u8 as *const i8,
    );
}
unsafe extern "C" fn nc_print_done() {
    _log(
        b"nc.c\0" as *const u8 as *const i8,
        198 as i32,
        0 as i32,
        b"done, rabbit done\0" as *const u8 as *const i8,
    );
}
unsafe extern "C" fn nc_show_usage() {
    _log_stderr(
        b"Usage: nutcracker [-?hVdDt] [-v verbosity level] [-o output file]\r\n                  [-c conf file] [-s stats port] [-a stats addr]\r\n                  [-i stats interval] [-p pid file] [-m mbuf size]\r\n\0"
            as *const u8 as *const i8,
    );
    _log_stderr(
        b"Options:\r\n  -h, --help             : this help\r\n  -V, --version          : show version and exit\r\n  -t, --test-conf        : test configuration for syntax errors and exit\r\n  -d, --daemonize        : run as a daemon\r\n  -D, --describe-stats   : print stats description and exit\0"
            as *const u8 as *const i8,
    );
    _log_stderr(
        b"  -v, --verbose=N        : set logging level (default: %d, min: %d, max: %d)\r\n  -o, --output=S         : set logging file (default: %s)\r\n  -c, --conf-file=S      : set configuration file (default: %s)\r\n  -s, --stats-port=N     : set stats monitoring port (default: %d)\r\n  -a, --stats-addr=S     : set stats monitoring ip (default: %s)\r\n  -i, --stats-interval=N : set stats aggregation interval in msec (default: %d msec)\r\n  -p, --pid-file=S       : set pid file (default: %s)\r\n  -m, --mbuf-size=N      : set size of mbuf chunk in bytes (default: %d bytes)\r\n\0"
            as *const u8 as *const i8,
        5 as i32,
        0 as i32,
        11 as i32,
        if !(0 as *mut libc::c_void).is_null() {
            0 as *const i8
        } else {
            b"stderr\0" as *const u8 as *const i8
        },
        b"conf/nutcracker.yml\0" as *const u8 as *const i8,
        22222 as i32,
        b"0.0.0.0\0" as *const u8 as *const i8,
        30 as i32 * 1000 as i32,
        if !(0 as *mut libc::c_void).is_null() {
            0 as *const i8
        } else {
            b"off\0" as *const u8 as *const i8
        },
        16384 as i32,
    );
}
unsafe extern "C" fn nc_create_pidfile(mut nci: *mut instance) -> rstatus_t {
    let mut pid: [i8; 21] = [0; 21];
    let mut fd: i32 = 0;
    let mut pid_len: i32 = 0;
    let mut n: ssize_t = 0;
    fd = open(
        (*nci).pid_filename,
        0o1 as i32 | 0o100 as i32 | 0o1000 as i32,
        0o644 as i32,
    );
    if fd < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                244 as i32,
                0 as i32,
                b"opening pid file '%s' failed: %s\0" as *const u8 as *const i8,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    (*nci).set_pidfile(1 as i32 as u32);
    pid_len = snprintf(
        pid.as_mut_ptr(),
        (20 as i32 + 1 as i32) as size_t,
        b"%d\0" as *const u8 as *const i8,
        (*nci).pid,
    );
    n = write(fd, pid.as_mut_ptr() as *const libc::c_void, pid_len as size_t);
    if n < 0 as i32 as i64 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                254 as i32,
                0 as i32,
                b"write to pid file '%s' failed: %s\0" as *const u8 as *const i8,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    close(fd);
    return 0 as i32;
}
unsafe extern "C" fn nc_remove_pidfile(mut nci: *mut instance) {
    let mut status: i32 = 0;
    status = unlink((*nci).pid_filename);
    if status < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                271 as i32,
                0 as i32,
                b"unlink of pid file '%s' failed, ignored: %s\0" as *const u8
                    as *const i8,
                (*nci).pid_filename,
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn nc_set_default_options(mut nci: *mut instance) {
    let mut status: i32 = 0;
    (*nci).ctx = 0 as *mut context;
    (*nci).log_level = 5 as i32;
    (*nci).log_filename = 0 as *const i8;
    (*nci).conf_filename = b"conf/nutcracker.yml\0" as *const u8 as *const i8;
    (*nci).stats_port = 22222 as i32 as uint16_t;
    (*nci).stats_addr = b"0.0.0.0\0" as *const u8 as *const i8;
    (*nci).stats_interval = 30 as i32 * 1000 as i32;
    status = gethostname(((*nci).hostname).as_mut_ptr(), 256 as i32 as size_t);
    if status < 0 as i32 {
        if log_loggable(4 as i32) != 0 as i32 {
            _log(
                b"nc.c\0" as *const u8 as *const i8,
                293 as i32,
                0 as i32,
                b"gethostname failed, ignored: %s\0" as *const u8 as *const i8,
                strerror(*__errno_location()),
            );
        }
        snprintf(
            ((*nci).hostname).as_mut_ptr(),
            256 as i32 as size_t,
            b"unknown\0" as *const u8 as *const i8,
        );
    }
    (*nci).hostname[(256 as i32 - 1 as i32) as usize] = '\0' as i32 as i8;
    (*nci).mbuf_chunk_size = 16384 as i32 as size_t;
    (*nci).pid = -(1 as i32);
    (*nci).pid_filename = 0 as *const i8;
    (*nci).set_pidfile(0 as i32 as u32);
}
unsafe extern "C" fn nc_get_options(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut nci: *mut instance,
) -> rstatus_t {
    let mut c: i32 = 0;
    let mut value: i32 = 0;
    opterr = 0 as i32;
    loop {
        c = getopt_long(
            argc,
            argv,
            short_options.as_ptr(),
            long_options.as_ptr(),
            0 as *mut i32,
        );
        if c == -(1 as i32) {
            break;
        }
        match c {
            104 => {
                show_version = 1 as i32;
                show_help = 1 as i32;
            }
            86 => {
                show_version = 1 as i32;
            }
            116 => {
                test_conf = 1 as i32;
            }
            100 => {
                daemonize = 1 as i32;
            }
            68 => {
                describe_stats = 1 as i32;
                show_version = 1 as i32;
            }
            118 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as i32 {
                    _log_stderr(
                        b"nutcracker: option -v requires a number\0" as *const u8
                            as *const i8,
                    );
                    return -(1 as i32);
                }
                (*nci).log_level = value;
            }
            111 => {
                (*nci).log_filename = optarg;
            }
            99 => {
                (*nci).conf_filename = optarg;
            }
            115 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as i32 {
                    _log_stderr(
                        b"nutcracker: option -s requires a number\0" as *const u8
                            as *const i8,
                    );
                    return -(1 as i32);
                }
                if !nc_valid_port(value) {
                    _log_stderr(
                        b"nutcracker: option -s value %d is not a valid port\0"
                            as *const u8 as *const i8,
                        value,
                    );
                    return -(1 as i32);
                }
                (*nci).stats_port = value as uint16_t;
            }
            105 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value < 0 as i32 {
                    _log_stderr(
                        b"nutcracker: option -i requires a number\0" as *const u8
                            as *const i8,
                    );
                    return -(1 as i32);
                }
                (*nci).stats_interval = value;
            }
            97 => {
                (*nci).stats_addr = optarg;
            }
            112 => {
                (*nci).pid_filename = optarg;
            }
            109 => {
                value = _nc_atoi(optarg as *mut uint8_t, strlen(optarg));
                if value <= 0 as i32 {
                    _log_stderr(
                        b"nutcracker: option -m requires a non-zero number\0"
                            as *const u8 as *const i8,
                    );
                    return -(1 as i32);
                }
                if value < 512 as i32 || value > 16777216 as i32 {
                    _log_stderr(
                        b"nutcracker: mbuf chunk size must be between %d and %d bytes\0"
                            as *const u8 as *const i8,
                        512 as i32,
                        16777216 as i32,
                    );
                    return -(1 as i32);
                }
                (*nci).mbuf_chunk_size = value as size_t;
            }
            63 => {
                match optopt {
                    111 | 99 | 112 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a file name\0" as *const u8
                                as *const i8,
                            optopt,
                        );
                    }
                    109 | 118 | 115 | 105 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a number\0" as *const u8
                                as *const i8,
                            optopt,
                        );
                    }
                    97 => {
                        _log_stderr(
                            b"nutcracker: option -%c requires a string\0" as *const u8
                                as *const i8,
                            optopt,
                        );
                    }
                    _ => {
                        _log_stderr(
                            b"nutcracker: invalid option -- '%c'\0" as *const u8
                                as *const i8,
                            optopt,
                        );
                    }
                }
                return -(1 as i32);
            }
            _ => {
                _log_stderr(
                    b"nutcracker: invalid option -- '%c'\0" as *const u8 as *const i8,
                    optopt,
                );
                return -(1 as i32);
            }
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn nc_test_conf(mut nci: *const instance) -> bool {
    let mut cf: *mut conf = 0 as *mut conf;
    cf = conf_create((*nci).conf_filename);
    if cf.is_null() {
        _log_stderr(
            b"nutcracker: configuration file '%s' syntax is invalid\0" as *const u8
                as *const i8,
            (*nci).conf_filename,
        );
        return 0 as i32 != 0;
    }
    conf_destroy(cf);
    _log_stderr(
        b"nutcracker: configuration file '%s' syntax is ok\0" as *const u8 as *const i8,
        (*nci).conf_filename,
    );
    return 1 as i32 != 0;
}
unsafe extern "C" fn nc_pre_run(mut nci: *mut instance) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = log_init((*nci).log_level, (*nci).log_filename);
    if status != 0 as i32 {
        return status;
    }
    if daemonize != 0 {
        status = nc_daemonize(1 as i32);
        if status != 0 as i32 {
            return status;
        }
    }
    (*nci).pid = getpid();
    status = signal_init();
    if status != 0 as i32 {
        return status;
    }
    if !((*nci).pid_filename).is_null() {
        status = nc_create_pidfile(nci);
        if status != 0 as i32 {
            return status;
        }
    }
    nc_print_run(nci);
    return 0 as i32;
}
unsafe extern "C" fn nc_post_run(mut nci: *mut instance) {
    if (*nci).pidfile() != 0 {
        nc_remove_pidfile(nci);
    }
    signal_deinit();
    nc_print_done();
    log_deinit();
}
unsafe extern "C" fn nc_run(mut nci: *mut instance) {
    let mut status: rstatus_t = 0;
    let mut ctx: *mut context = 0 as *mut context;
    ctx = core_start(nci);
    if ctx.is_null() {
        return;
    }
    loop {
        status = core_loop(ctx);
        if status != 0 as i32 {
            break;
        }
    }
    core_stop(ctx);
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut status: rstatus_t = 0;
    let mut nci: instance = instance {
        ctx: 0 as *mut context,
        log_level: 0,
        log_filename: 0 as *const i8,
        conf_filename: 0 as *const i8,
        stats_port: 0,
        stats_interval: 0,
        stats_addr: 0 as *const i8,
        hostname: [0; 256],
        mbuf_chunk_size: 0,
        pid: 0,
        pid_filename: 0 as *const i8,
        pidfile: [0; 1],
        c2rust_padding: [0; 7],
    };
    nc_set_default_options(&mut nci);
    status = nc_get_options(argc, argv, &mut nci);
    if status != 0 as i32 {
        nc_show_usage();
        exit(1 as i32);
    }
    if show_version != 0 {
        _log_stderr(
            b"This is nutcracker-%s\0" as *const u8 as *const i8,
            b"0.5.0\0" as *const u8 as *const i8,
        );
        _log_stderr(b"async event backend: epoll\0" as *const u8 as *const i8);
        _log_stderr(b"\0" as *const u8 as *const i8);
        if show_help != 0 {
            nc_show_usage();
        }
        if describe_stats != 0 {
            stats_describe();
        }
        exit(0 as i32);
    }
    if test_conf != 0 {
        if !nc_test_conf(&mut nci) {
            exit(1 as i32);
        }
        exit(0 as i32);
    }
    status = nc_pre_run(&mut nci);
    if status != 0 as i32 {
        nc_post_run(&mut nci);
        exit(1 as i32);
    }
    nc_run(&mut nci);
    nc_post_run(&mut nci);
    exit(1 as i32);
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