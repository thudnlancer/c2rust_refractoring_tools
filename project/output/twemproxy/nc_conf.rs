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
    pub type epoll_event;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn server_init(
        server: *mut array,
        conf_server: *mut array,
        sp: *mut server_pool,
    ) -> rstatus_t;
    fn array_init(a: *mut array, n: uint32_t, size: size_t) -> rstatus_t;
    fn array_deinit(a: *mut array);
    fn array_idx(a: *const array, elem: *const libc::c_void) -> uint32_t;
    fn array_push(a: *mut array) -> *mut libc::c_void;
    fn array_pop(a: *mut array) -> *mut libc::c_void;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn array_top(a: *const array) -> *mut libc::c_void;
    fn array_sort(a: *mut array, compare: array_compare_t);
    fn string_init(str: *mut string);
    fn string_deinit(str: *mut string);
    fn string_duplicate(dst: *mut string, src: *const string) -> rstatus_t;
    fn string_copy(dst: *mut string, src: *const uint8_t, srclen: uint32_t) -> rstatus_t;
    fn string_compare(s1: *const string, s2: *const string) -> i32;
    fn _nc_atoi(line: *const uint8_t, n: size_t) -> i32;
    fn nc_valid_port(n: i32) -> bool;
    fn _nc_alloc(size: size_t, name: *const i8, line: i32) -> *mut libc::c_void;
    fn _nc_free(ptr: *mut libc::c_void, name: *const i8, line: i32);
    fn nc_resolve(name: *const string, port: i32, si: *mut sockinfo) -> i32;
    fn log_loggable(level: i32) -> i32;
    fn _log(file: *const i8, line: i32, panic: i32, fmt: *const i8, _: ...);
    fn _log_stderr(fmt: *const i8, _: ...);
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn hash_murmur(key: *const i8, length: size_t) -> uint32_t;
    fn hash_jenkins(key: *const i8, length: size_t) -> uint32_t;
    fn hash_hsieh(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_fnv1a_32(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_fnv1_32(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_fnv1a_64(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_fnv1_64(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_crc32a(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_crc32(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_crc16(key: *const i8, key_length: size_t) -> uint32_t;
    fn hash_md5(key: *const i8, key_length: size_t) -> uint32_t;
    fn yaml_token_delete(token: *mut yaml_token_t);
    fn yaml_event_delete(event: *mut yaml_event_t);
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t;
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32;
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_scan(parser: *mut yaml_parser_t, token: *mut yaml_token_t) -> i32;
    fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> i32;
    fn yaml_parser_load(
        parser: *mut yaml_parser_t,
        document: *mut yaml_document_t,
    ) -> i32;
    fn hash_one_at_a_time(key: *const i8, key_length: size_t) -> uint32_t;
}
pub type rstatus_t = i32;
pub type err_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
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
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type int64_t = __int64_t;
pub type __int64_t = i64;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
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
pub type FILE = _IO_FILE;
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
pub struct conn {
    pub conn_tqe: C2RustUnnamed_41,
    pub owner: *mut libc::c_void,
    pub sd: i32,
    pub family: i32,
    pub addrlen: socklen_t,
    pub addr: *mut sockaddr,
    pub imsg_q: msg_tqh,
    pub omsg_q: msg_tqh,
    pub rmsg: *mut msg,
    pub smsg: *mut msg,
    pub recv: conn_recv_t,
    pub recv_next: conn_recv_next_t,
    pub recv_done: conn_recv_done_t,
    pub send: conn_send_t,
    pub send_next: conn_send_next_t,
    pub send_done: conn_send_done_t,
    pub close: conn_close_t,
    pub active: conn_active_t,
    pub post_connect: conn_post_connect_t,
    pub swallow_msg: conn_swallow_msg_t,
    pub ref_0: conn_ref_t,
    pub unref: conn_unref_t,
    pub enqueue_inq: conn_msgq_t,
    pub dequeue_inq: conn_msgq_t,
    pub enqueue_outq: conn_msgq_t,
    pub dequeue_outq: conn_msgq_t,
    pub recv_bytes: size_t,
    pub send_bytes: size_t,
    pub events: uint32_t,
    pub err: err_t,
    #[bitfield(name = "recv_active", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "recv_ready", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "send_active", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "send_ready", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "client", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "proxy", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "connecting", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "connected", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "eof", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "authenticated", ty = "libc::c_uint", bits = "11..=11")]
    pub recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
pub type conn_msgq_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct msg {
    pub c_tqe: C2RustUnnamed_38,
    pub s_tqe: C2RustUnnamed_37,
    pub m_tqe: C2RustUnnamed_36,
    pub id: uint64_t,
    pub peer: *mut msg,
    pub owner: *mut conn,
    pub tmo_rbe: rbnode,
    pub mhdr: mhdr,
    pub mlen: uint32_t,
    pub start_ts: int64_t,
    pub state: i32,
    pub pos: *mut uint8_t,
    pub token: *mut uint8_t,
    pub parser: msg_parse_t,
    pub result: msg_parse_result_t,
    pub fragment: msg_fragment_t,
    pub reply: msg_reply_t,
    pub add_auth: msg_add_auth_t,
    pub failure: msg_failure_t,
    pub pre_coalesce: msg_coalesce_t,
    pub post_coalesce: msg_coalesce_t,
    pub type_0: msg_type_t,
    pub keys: *mut array,
    pub vlen: uint32_t,
    pub end: *mut uint8_t,
    pub narg_start: *mut uint8_t,
    pub narg_end: *mut uint8_t,
    pub narg: uint32_t,
    pub rnarg: uint32_t,
    pub rlen: uint32_t,
    pub integer: uint32_t,
    pub is_top_level: uint8_t,
    pub frag_owner: *mut msg,
    pub nfrag: uint32_t,
    pub nfrag_done: uint32_t,
    pub frag_id: uint64_t,
    pub frag_seq: *mut *mut msg,
    pub err: err_t,
    #[bitfield(name = "error", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "ferror", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "request", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "quit", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "noreply", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "noforward", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "done", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "fdone", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "swallow", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "9..=9")]
    pub error_ferror_request_quit_noreply_noforward_done_fdone_swallow_redis: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = u64;
pub type msg_type_t = msg_type;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum msg_type {
    MSG_SENTINEL = 184,
    MSG_RSP_REDIS_MULTIBULK = 183,
    MSG_RSP_REDIS_BULK = 182,
    MSG_RSP_REDIS_INTEGER = 181,
    MSG_RSP_REDIS_ERROR_NOREPLICAS = 180,
    MSG_RSP_REDIS_ERROR_MASTERDOWN = 179,
    MSG_RSP_REDIS_ERROR_EXECABORT = 178,
    MSG_RSP_REDIS_ERROR_WRONGTYPE = 177,
    MSG_RSP_REDIS_ERROR_READONLY = 176,
    MSG_RSP_REDIS_ERROR_NOSCRIPT = 175,
    MSG_RSP_REDIS_ERROR_MISCONF = 174,
    MSG_RSP_REDIS_ERROR_BUSYKEY = 173,
    MSG_RSP_REDIS_ERROR_LOADING = 172,
    MSG_RSP_REDIS_ERROR_NOAUTH = 171,
    MSG_RSP_REDIS_ERROR_BUSY = 170,
    MSG_RSP_REDIS_ERROR_OOM = 169,
    MSG_RSP_REDIS_ERROR_ERR = 168,
    MSG_RSP_REDIS_ERROR = 167,
    MSG_RSP_REDIS_STATUS = 166,
    MSG_REQ_REDIS_LOLWUT = 165,
    MSG_REQ_REDIS_COMMAND = 164,
    MSG_REQ_REDIS_SELECT = 163,
    MSG_REQ_REDIS_AUTH = 162,
    MSG_REQ_REDIS_QUIT = 161,
    MSG_REQ_REDIS_PING = 160,
    MSG_REQ_REDIS_EVALSHA = 159,
    MSG_REQ_REDIS_EVAL = 158,
    MSG_REQ_REDIS_GEOSEARCHSTORE = 157,
    MSG_REQ_REDIS_GEOSEARCH = 156,
    MSG_REQ_REDIS_GEOPOS = 155,
    MSG_REQ_REDIS_GEORADIUSBYMEMBER = 154,
    MSG_REQ_REDIS_GEORADIUS = 153,
    MSG_REQ_REDIS_GEOHASH = 152,
    MSG_REQ_REDIS_GEODIST = 151,
    MSG_REQ_REDIS_GEOADD = 150,
    MSG_REQ_REDIS_ZUNIONSTORE = 149,
    MSG_REQ_REDIS_ZSCORE = 148,
    MSG_REQ_REDIS_ZSCAN = 147,
    MSG_REQ_REDIS_ZUNION = 146,
    MSG_REQ_REDIS_ZREVRANK = 145,
    MSG_REQ_REDIS_ZREVRANGEBYSCORE = 144,
    MSG_REQ_REDIS_ZREVRANGEBYLEX = 143,
    MSG_REQ_REDIS_ZREVRANGE = 142,
    MSG_REQ_REDIS_ZREMRANGEBYSCORE = 141,
    MSG_REQ_REDIS_ZREMRANGEBYLEX = 140,
    MSG_REQ_REDIS_ZREMRANGEBYRANK = 139,
    MSG_REQ_REDIS_ZREM = 138,
    MSG_REQ_REDIS_ZRANK = 137,
    MSG_REQ_REDIS_ZRANGESTORE = 136,
    MSG_REQ_REDIS_ZRANGEBYSCORE = 135,
    MSG_REQ_REDIS_ZRANGEBYLEX = 134,
    MSG_REQ_REDIS_ZRANGE = 133,
    MSG_REQ_REDIS_ZRANDMEMBER = 132,
    MSG_REQ_REDIS_ZPOPMAX = 131,
    MSG_REQ_REDIS_ZPOPMIN = 130,
    MSG_REQ_REDIS_ZMSCORE = 129,
    MSG_REQ_REDIS_ZLEXCOUNT = 128,
    MSG_REQ_REDIS_ZINTERSTORE = 127,
    MSG_REQ_REDIS_ZINTER = 126,
    MSG_REQ_REDIS_ZINCRBY = 125,
    MSG_REQ_REDIS_ZDIFFSTORE = 124,
    MSG_REQ_REDIS_ZDIFF = 123,
    MSG_REQ_REDIS_ZCOUNT = 122,
    MSG_REQ_REDIS_ZCARD = 121,
    MSG_REQ_REDIS_ZADD = 120,
    MSG_REQ_REDIS_SSCAN = 119,
    MSG_REQ_REDIS_SUNIONSTORE = 118,
    MSG_REQ_REDIS_SUNION = 117,
    MSG_REQ_REDIS_SREM = 116,
    MSG_REQ_REDIS_SRANDMEMBER = 115,
    MSG_REQ_REDIS_SPOP = 114,
    MSG_REQ_REDIS_SMOVE = 113,
    MSG_REQ_REDIS_SMEMBERS = 112,
    MSG_REQ_REDIS_SMISMEMBER = 111,
    MSG_REQ_REDIS_SISMEMBER = 110,
    MSG_REQ_REDIS_SINTERSTORE = 109,
    MSG_REQ_REDIS_SINTER = 108,
    MSG_REQ_REDIS_SDIFFSTORE = 107,
    MSG_REQ_REDIS_SDIFF = 106,
    MSG_REQ_REDIS_SCARD = 105,
    MSG_REQ_REDIS_SADD = 104,
    MSG_REQ_REDIS_RPUSHX = 103,
    MSG_REQ_REDIS_RPUSH = 102,
    MSG_REQ_REDIS_RPOPLPUSH = 101,
    MSG_REQ_REDIS_RPOP = 100,
    MSG_REQ_REDIS_PFMERGE = 99,
    MSG_REQ_REDIS_PFCOUNT = 98,
    MSG_REQ_REDIS_PFADD = 97,
    MSG_REQ_REDIS_LTRIM = 96,
    MSG_REQ_REDIS_LSET = 95,
    MSG_REQ_REDIS_LREM = 94,
    MSG_REQ_REDIS_LRANGE = 93,
    MSG_REQ_REDIS_LPUSHX = 92,
    MSG_REQ_REDIS_LPUSH = 91,
    MSG_REQ_REDIS_LPOS = 90,
    MSG_REQ_REDIS_LPOP = 89,
    MSG_REQ_REDIS_LMOVE = 88,
    MSG_REQ_REDIS_LLEN = 87,
    MSG_REQ_REDIS_LINSERT = 86,
    MSG_REQ_REDIS_LINDEX = 85,
    MSG_REQ_REDIS_HVALS = 84,
    MSG_REQ_REDIS_HSTRLEN = 83,
    MSG_REQ_REDIS_HSCAN = 82,
    MSG_REQ_REDIS_HSETNX = 81,
    MSG_REQ_REDIS_HSET = 80,
    MSG_REQ_REDIS_HRANDFIELD = 79,
    MSG_REQ_REDIS_HMSET = 78,
    MSG_REQ_REDIS_HMGET = 77,
    MSG_REQ_REDIS_HLEN = 76,
    MSG_REQ_REDIS_HKEYS = 75,
    MSG_REQ_REDIS_HINCRBYFLOAT = 74,
    MSG_REQ_REDIS_HINCRBY = 73,
    MSG_REQ_REDIS_HGETALL = 72,
    MSG_REQ_REDIS_HGET = 71,
    MSG_REQ_REDIS_HEXISTS = 70,
    MSG_REQ_REDIS_HDEL = 69,
    MSG_REQ_REDIS_STRLEN = 68,
    MSG_REQ_REDIS_SETRANGE = 67,
    MSG_REQ_REDIS_SETNX = 66,
    MSG_REQ_REDIS_SETEX = 65,
    MSG_REQ_REDIS_SETBIT = 64,
    MSG_REQ_REDIS_SET = 63,
    MSG_REQ_REDIS_RESTORE = 62,
    MSG_REQ_REDIS_PSETEX = 61,
    MSG_REQ_REDIS_MSET = 60,
    MSG_REQ_REDIS_MGET = 59,
    MSG_REQ_REDIS_INCRBYFLOAT = 58,
    MSG_REQ_REDIS_INCRBY = 57,
    MSG_REQ_REDIS_INCR = 56,
    MSG_REQ_REDIS_GETSET = 55,
    MSG_REQ_REDIS_GETRANGE = 54,
    MSG_REQ_REDIS_GETEX = 53,
    MSG_REQ_REDIS_GETDEL = 52,
    MSG_REQ_REDIS_GETBIT = 51,
    MSG_REQ_REDIS_GET = 50,
    MSG_REQ_REDIS_DUMP = 49,
    MSG_REQ_REDIS_DECRBY = 48,
    MSG_REQ_REDIS_DECR = 47,
    MSG_REQ_REDIS_BITPOS = 46,
    MSG_REQ_REDIS_BITFIELD = 45,
    MSG_REQ_REDIS_BITCOUNT = 44,
    MSG_REQ_REDIS_APPEND = 43,
    MSG_REQ_REDIS_UNLINK = 42,
    MSG_REQ_REDIS_TYPE = 41,
    MSG_REQ_REDIS_TTL = 40,
    MSG_REQ_REDIS_TOUCH = 39,
    MSG_REQ_REDIS_SORT = 38,
    MSG_REQ_REDIS_PTTL = 37,
    MSG_REQ_REDIS_PERSIST = 36,
    MSG_REQ_REDIS_PEXPIREAT = 35,
    MSG_REQ_REDIS_PEXPIRE = 34,
    MSG_REQ_REDIS_MOVE = 33,
    MSG_REQ_REDIS_EXPIREAT = 32,
    MSG_REQ_REDIS_EXPIRE = 31,
    MSG_REQ_REDIS_EXISTS = 30,
    MSG_REQ_REDIS_DEL = 29,
    MSG_REQ_REDIS_COPY = 28,
    MSG_RSP_MC_SERVER_ERROR = 27,
    MSG_RSP_MC_CLIENT_ERROR = 26,
    MSG_RSP_MC_ERROR = 25,
    MSG_RSP_MC_VERSION = 24,
    MSG_RSP_MC_TOUCHED = 23,
    MSG_RSP_MC_DELETED = 22,
    MSG_RSP_MC_VALUE = 21,
    MSG_RSP_MC_END = 20,
    MSG_RSP_MC_NOT_FOUND = 19,
    MSG_RSP_MC_EXISTS = 18,
    MSG_RSP_MC_NOT_STORED = 17,
    MSG_RSP_MC_STORED = 16,
    MSG_RSP_MC_NUM = 15,
    MSG_REQ_MC_VERSION = 14,
    MSG_REQ_MC_QUIT = 13,
    MSG_REQ_MC_TOUCH = 12,
    MSG_REQ_MC_DECR = 11,
    MSG_REQ_MC_INCR = 10,
    MSG_REQ_MC_PREPEND = 9,
    MSG_REQ_MC_APPEND = 8,
    MSG_REQ_MC_REPLACE = 7,
    MSG_REQ_MC_ADD = 6,
    MSG_REQ_MC_SET = 5,
    MSG_REQ_MC_CAS = 4,
    MSG_REQ_MC_DELETE = 3,
    MSG_REQ_MC_GETS = 2,
    MSG_REQ_MC_GET = 1,
    MSG_UNKNOWN = 0,
}
impl msg_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            msg_type::MSG_SENTINEL => 184,
            msg_type::MSG_RSP_REDIS_MULTIBULK => 183,
            msg_type::MSG_RSP_REDIS_BULK => 182,
            msg_type::MSG_RSP_REDIS_INTEGER => 181,
            msg_type::MSG_RSP_REDIS_ERROR_NOREPLICAS => 180,
            msg_type::MSG_RSP_REDIS_ERROR_MASTERDOWN => 179,
            msg_type::MSG_RSP_REDIS_ERROR_EXECABORT => 178,
            msg_type::MSG_RSP_REDIS_ERROR_WRONGTYPE => 177,
            msg_type::MSG_RSP_REDIS_ERROR_READONLY => 176,
            msg_type::MSG_RSP_REDIS_ERROR_NOSCRIPT => 175,
            msg_type::MSG_RSP_REDIS_ERROR_MISCONF => 174,
            msg_type::MSG_RSP_REDIS_ERROR_BUSYKEY => 173,
            msg_type::MSG_RSP_REDIS_ERROR_LOADING => 172,
            msg_type::MSG_RSP_REDIS_ERROR_NOAUTH => 171,
            msg_type::MSG_RSP_REDIS_ERROR_BUSY => 170,
            msg_type::MSG_RSP_REDIS_ERROR_OOM => 169,
            msg_type::MSG_RSP_REDIS_ERROR_ERR => 168,
            msg_type::MSG_RSP_REDIS_ERROR => 167,
            msg_type::MSG_RSP_REDIS_STATUS => 166,
            msg_type::MSG_REQ_REDIS_LOLWUT => 165,
            msg_type::MSG_REQ_REDIS_COMMAND => 164,
            msg_type::MSG_REQ_REDIS_SELECT => 163,
            msg_type::MSG_REQ_REDIS_AUTH => 162,
            msg_type::MSG_REQ_REDIS_QUIT => 161,
            msg_type::MSG_REQ_REDIS_PING => 160,
            msg_type::MSG_REQ_REDIS_EVALSHA => 159,
            msg_type::MSG_REQ_REDIS_EVAL => 158,
            msg_type::MSG_REQ_REDIS_GEOSEARCHSTORE => 157,
            msg_type::MSG_REQ_REDIS_GEOSEARCH => 156,
            msg_type::MSG_REQ_REDIS_GEOPOS => 155,
            msg_type::MSG_REQ_REDIS_GEORADIUSBYMEMBER => 154,
            msg_type::MSG_REQ_REDIS_GEORADIUS => 153,
            msg_type::MSG_REQ_REDIS_GEOHASH => 152,
            msg_type::MSG_REQ_REDIS_GEODIST => 151,
            msg_type::MSG_REQ_REDIS_GEOADD => 150,
            msg_type::MSG_REQ_REDIS_ZUNIONSTORE => 149,
            msg_type::MSG_REQ_REDIS_ZSCORE => 148,
            msg_type::MSG_REQ_REDIS_ZSCAN => 147,
            msg_type::MSG_REQ_REDIS_ZUNION => 146,
            msg_type::MSG_REQ_REDIS_ZREVRANK => 145,
            msg_type::MSG_REQ_REDIS_ZREVRANGEBYSCORE => 144,
            msg_type::MSG_REQ_REDIS_ZREVRANGEBYLEX => 143,
            msg_type::MSG_REQ_REDIS_ZREVRANGE => 142,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYSCORE => 141,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYLEX => 140,
            msg_type::MSG_REQ_REDIS_ZREMRANGEBYRANK => 139,
            msg_type::MSG_REQ_REDIS_ZREM => 138,
            msg_type::MSG_REQ_REDIS_ZRANK => 137,
            msg_type::MSG_REQ_REDIS_ZRANGESTORE => 136,
            msg_type::MSG_REQ_REDIS_ZRANGEBYSCORE => 135,
            msg_type::MSG_REQ_REDIS_ZRANGEBYLEX => 134,
            msg_type::MSG_REQ_REDIS_ZRANGE => 133,
            msg_type::MSG_REQ_REDIS_ZRANDMEMBER => 132,
            msg_type::MSG_REQ_REDIS_ZPOPMAX => 131,
            msg_type::MSG_REQ_REDIS_ZPOPMIN => 130,
            msg_type::MSG_REQ_REDIS_ZMSCORE => 129,
            msg_type::MSG_REQ_REDIS_ZLEXCOUNT => 128,
            msg_type::MSG_REQ_REDIS_ZINTERSTORE => 127,
            msg_type::MSG_REQ_REDIS_ZINTER => 126,
            msg_type::MSG_REQ_REDIS_ZINCRBY => 125,
            msg_type::MSG_REQ_REDIS_ZDIFFSTORE => 124,
            msg_type::MSG_REQ_REDIS_ZDIFF => 123,
            msg_type::MSG_REQ_REDIS_ZCOUNT => 122,
            msg_type::MSG_REQ_REDIS_ZCARD => 121,
            msg_type::MSG_REQ_REDIS_ZADD => 120,
            msg_type::MSG_REQ_REDIS_SSCAN => 119,
            msg_type::MSG_REQ_REDIS_SUNIONSTORE => 118,
            msg_type::MSG_REQ_REDIS_SUNION => 117,
            msg_type::MSG_REQ_REDIS_SREM => 116,
            msg_type::MSG_REQ_REDIS_SRANDMEMBER => 115,
            msg_type::MSG_REQ_REDIS_SPOP => 114,
            msg_type::MSG_REQ_REDIS_SMOVE => 113,
            msg_type::MSG_REQ_REDIS_SMEMBERS => 112,
            msg_type::MSG_REQ_REDIS_SMISMEMBER => 111,
            msg_type::MSG_REQ_REDIS_SISMEMBER => 110,
            msg_type::MSG_REQ_REDIS_SINTERSTORE => 109,
            msg_type::MSG_REQ_REDIS_SINTER => 108,
            msg_type::MSG_REQ_REDIS_SDIFFSTORE => 107,
            msg_type::MSG_REQ_REDIS_SDIFF => 106,
            msg_type::MSG_REQ_REDIS_SCARD => 105,
            msg_type::MSG_REQ_REDIS_SADD => 104,
            msg_type::MSG_REQ_REDIS_RPUSHX => 103,
            msg_type::MSG_REQ_REDIS_RPUSH => 102,
            msg_type::MSG_REQ_REDIS_RPOPLPUSH => 101,
            msg_type::MSG_REQ_REDIS_RPOP => 100,
            msg_type::MSG_REQ_REDIS_PFMERGE => 99,
            msg_type::MSG_REQ_REDIS_PFCOUNT => 98,
            msg_type::MSG_REQ_REDIS_PFADD => 97,
            msg_type::MSG_REQ_REDIS_LTRIM => 96,
            msg_type::MSG_REQ_REDIS_LSET => 95,
            msg_type::MSG_REQ_REDIS_LREM => 94,
            msg_type::MSG_REQ_REDIS_LRANGE => 93,
            msg_type::MSG_REQ_REDIS_LPUSHX => 92,
            msg_type::MSG_REQ_REDIS_LPUSH => 91,
            msg_type::MSG_REQ_REDIS_LPOS => 90,
            msg_type::MSG_REQ_REDIS_LPOP => 89,
            msg_type::MSG_REQ_REDIS_LMOVE => 88,
            msg_type::MSG_REQ_REDIS_LLEN => 87,
            msg_type::MSG_REQ_REDIS_LINSERT => 86,
            msg_type::MSG_REQ_REDIS_LINDEX => 85,
            msg_type::MSG_REQ_REDIS_HVALS => 84,
            msg_type::MSG_REQ_REDIS_HSTRLEN => 83,
            msg_type::MSG_REQ_REDIS_HSCAN => 82,
            msg_type::MSG_REQ_REDIS_HSETNX => 81,
            msg_type::MSG_REQ_REDIS_HSET => 80,
            msg_type::MSG_REQ_REDIS_HRANDFIELD => 79,
            msg_type::MSG_REQ_REDIS_HMSET => 78,
            msg_type::MSG_REQ_REDIS_HMGET => 77,
            msg_type::MSG_REQ_REDIS_HLEN => 76,
            msg_type::MSG_REQ_REDIS_HKEYS => 75,
            msg_type::MSG_REQ_REDIS_HINCRBYFLOAT => 74,
            msg_type::MSG_REQ_REDIS_HINCRBY => 73,
            msg_type::MSG_REQ_REDIS_HGETALL => 72,
            msg_type::MSG_REQ_REDIS_HGET => 71,
            msg_type::MSG_REQ_REDIS_HEXISTS => 70,
            msg_type::MSG_REQ_REDIS_HDEL => 69,
            msg_type::MSG_REQ_REDIS_STRLEN => 68,
            msg_type::MSG_REQ_REDIS_SETRANGE => 67,
            msg_type::MSG_REQ_REDIS_SETNX => 66,
            msg_type::MSG_REQ_REDIS_SETEX => 65,
            msg_type::MSG_REQ_REDIS_SETBIT => 64,
            msg_type::MSG_REQ_REDIS_SET => 63,
            msg_type::MSG_REQ_REDIS_RESTORE => 62,
            msg_type::MSG_REQ_REDIS_PSETEX => 61,
            msg_type::MSG_REQ_REDIS_MSET => 60,
            msg_type::MSG_REQ_REDIS_MGET => 59,
            msg_type::MSG_REQ_REDIS_INCRBYFLOAT => 58,
            msg_type::MSG_REQ_REDIS_INCRBY => 57,
            msg_type::MSG_REQ_REDIS_INCR => 56,
            msg_type::MSG_REQ_REDIS_GETSET => 55,
            msg_type::MSG_REQ_REDIS_GETRANGE => 54,
            msg_type::MSG_REQ_REDIS_GETEX => 53,
            msg_type::MSG_REQ_REDIS_GETDEL => 52,
            msg_type::MSG_REQ_REDIS_GETBIT => 51,
            msg_type::MSG_REQ_REDIS_GET => 50,
            msg_type::MSG_REQ_REDIS_DUMP => 49,
            msg_type::MSG_REQ_REDIS_DECRBY => 48,
            msg_type::MSG_REQ_REDIS_DECR => 47,
            msg_type::MSG_REQ_REDIS_BITPOS => 46,
            msg_type::MSG_REQ_REDIS_BITFIELD => 45,
            msg_type::MSG_REQ_REDIS_BITCOUNT => 44,
            msg_type::MSG_REQ_REDIS_APPEND => 43,
            msg_type::MSG_REQ_REDIS_UNLINK => 42,
            msg_type::MSG_REQ_REDIS_TYPE => 41,
            msg_type::MSG_REQ_REDIS_TTL => 40,
            msg_type::MSG_REQ_REDIS_TOUCH => 39,
            msg_type::MSG_REQ_REDIS_SORT => 38,
            msg_type::MSG_REQ_REDIS_PTTL => 37,
            msg_type::MSG_REQ_REDIS_PERSIST => 36,
            msg_type::MSG_REQ_REDIS_PEXPIREAT => 35,
            msg_type::MSG_REQ_REDIS_PEXPIRE => 34,
            msg_type::MSG_REQ_REDIS_MOVE => 33,
            msg_type::MSG_REQ_REDIS_EXPIREAT => 32,
            msg_type::MSG_REQ_REDIS_EXPIRE => 31,
            msg_type::MSG_REQ_REDIS_EXISTS => 30,
            msg_type::MSG_REQ_REDIS_DEL => 29,
            msg_type::MSG_REQ_REDIS_COPY => 28,
            msg_type::MSG_RSP_MC_SERVER_ERROR => 27,
            msg_type::MSG_RSP_MC_CLIENT_ERROR => 26,
            msg_type::MSG_RSP_MC_ERROR => 25,
            msg_type::MSG_RSP_MC_VERSION => 24,
            msg_type::MSG_RSP_MC_TOUCHED => 23,
            msg_type::MSG_RSP_MC_DELETED => 22,
            msg_type::MSG_RSP_MC_VALUE => 21,
            msg_type::MSG_RSP_MC_END => 20,
            msg_type::MSG_RSP_MC_NOT_FOUND => 19,
            msg_type::MSG_RSP_MC_EXISTS => 18,
            msg_type::MSG_RSP_MC_NOT_STORED => 17,
            msg_type::MSG_RSP_MC_STORED => 16,
            msg_type::MSG_RSP_MC_NUM => 15,
            msg_type::MSG_REQ_MC_VERSION => 14,
            msg_type::MSG_REQ_MC_QUIT => 13,
            msg_type::MSG_REQ_MC_TOUCH => 12,
            msg_type::MSG_REQ_MC_DECR => 11,
            msg_type::MSG_REQ_MC_INCR => 10,
            msg_type::MSG_REQ_MC_PREPEND => 9,
            msg_type::MSG_REQ_MC_APPEND => 8,
            msg_type::MSG_REQ_MC_REPLACE => 7,
            msg_type::MSG_REQ_MC_ADD => 6,
            msg_type::MSG_REQ_MC_SET => 5,
            msg_type::MSG_REQ_MC_CAS => 4,
            msg_type::MSG_REQ_MC_DELETE => 3,
            msg_type::MSG_REQ_MC_GETS => 2,
            msg_type::MSG_REQ_MC_GET => 1,
            msg_type::MSG_UNKNOWN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> msg_type {
        match value {
            184 => msg_type::MSG_SENTINEL,
            183 => msg_type::MSG_RSP_REDIS_MULTIBULK,
            182 => msg_type::MSG_RSP_REDIS_BULK,
            181 => msg_type::MSG_RSP_REDIS_INTEGER,
            180 => msg_type::MSG_RSP_REDIS_ERROR_NOREPLICAS,
            179 => msg_type::MSG_RSP_REDIS_ERROR_MASTERDOWN,
            178 => msg_type::MSG_RSP_REDIS_ERROR_EXECABORT,
            177 => msg_type::MSG_RSP_REDIS_ERROR_WRONGTYPE,
            176 => msg_type::MSG_RSP_REDIS_ERROR_READONLY,
            175 => msg_type::MSG_RSP_REDIS_ERROR_NOSCRIPT,
            174 => msg_type::MSG_RSP_REDIS_ERROR_MISCONF,
            173 => msg_type::MSG_RSP_REDIS_ERROR_BUSYKEY,
            172 => msg_type::MSG_RSP_REDIS_ERROR_LOADING,
            171 => msg_type::MSG_RSP_REDIS_ERROR_NOAUTH,
            170 => msg_type::MSG_RSP_REDIS_ERROR_BUSY,
            169 => msg_type::MSG_RSP_REDIS_ERROR_OOM,
            168 => msg_type::MSG_RSP_REDIS_ERROR_ERR,
            167 => msg_type::MSG_RSP_REDIS_ERROR,
            166 => msg_type::MSG_RSP_REDIS_STATUS,
            165 => msg_type::MSG_REQ_REDIS_LOLWUT,
            164 => msg_type::MSG_REQ_REDIS_COMMAND,
            163 => msg_type::MSG_REQ_REDIS_SELECT,
            162 => msg_type::MSG_REQ_REDIS_AUTH,
            161 => msg_type::MSG_REQ_REDIS_QUIT,
            160 => msg_type::MSG_REQ_REDIS_PING,
            159 => msg_type::MSG_REQ_REDIS_EVALSHA,
            158 => msg_type::MSG_REQ_REDIS_EVAL,
            157 => msg_type::MSG_REQ_REDIS_GEOSEARCHSTORE,
            156 => msg_type::MSG_REQ_REDIS_GEOSEARCH,
            155 => msg_type::MSG_REQ_REDIS_GEOPOS,
            154 => msg_type::MSG_REQ_REDIS_GEORADIUSBYMEMBER,
            153 => msg_type::MSG_REQ_REDIS_GEORADIUS,
            152 => msg_type::MSG_REQ_REDIS_GEOHASH,
            151 => msg_type::MSG_REQ_REDIS_GEODIST,
            150 => msg_type::MSG_REQ_REDIS_GEOADD,
            149 => msg_type::MSG_REQ_REDIS_ZUNIONSTORE,
            148 => msg_type::MSG_REQ_REDIS_ZSCORE,
            147 => msg_type::MSG_REQ_REDIS_ZSCAN,
            146 => msg_type::MSG_REQ_REDIS_ZUNION,
            145 => msg_type::MSG_REQ_REDIS_ZREVRANK,
            144 => msg_type::MSG_REQ_REDIS_ZREVRANGEBYSCORE,
            143 => msg_type::MSG_REQ_REDIS_ZREVRANGEBYLEX,
            142 => msg_type::MSG_REQ_REDIS_ZREVRANGE,
            141 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYSCORE,
            140 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYLEX,
            139 => msg_type::MSG_REQ_REDIS_ZREMRANGEBYRANK,
            138 => msg_type::MSG_REQ_REDIS_ZREM,
            137 => msg_type::MSG_REQ_REDIS_ZRANK,
            136 => msg_type::MSG_REQ_REDIS_ZRANGESTORE,
            135 => msg_type::MSG_REQ_REDIS_ZRANGEBYSCORE,
            134 => msg_type::MSG_REQ_REDIS_ZRANGEBYLEX,
            133 => msg_type::MSG_REQ_REDIS_ZRANGE,
            132 => msg_type::MSG_REQ_REDIS_ZRANDMEMBER,
            131 => msg_type::MSG_REQ_REDIS_ZPOPMAX,
            130 => msg_type::MSG_REQ_REDIS_ZPOPMIN,
            129 => msg_type::MSG_REQ_REDIS_ZMSCORE,
            128 => msg_type::MSG_REQ_REDIS_ZLEXCOUNT,
            127 => msg_type::MSG_REQ_REDIS_ZINTERSTORE,
            126 => msg_type::MSG_REQ_REDIS_ZINTER,
            125 => msg_type::MSG_REQ_REDIS_ZINCRBY,
            124 => msg_type::MSG_REQ_REDIS_ZDIFFSTORE,
            123 => msg_type::MSG_REQ_REDIS_ZDIFF,
            122 => msg_type::MSG_REQ_REDIS_ZCOUNT,
            121 => msg_type::MSG_REQ_REDIS_ZCARD,
            120 => msg_type::MSG_REQ_REDIS_ZADD,
            119 => msg_type::MSG_REQ_REDIS_SSCAN,
            118 => msg_type::MSG_REQ_REDIS_SUNIONSTORE,
            117 => msg_type::MSG_REQ_REDIS_SUNION,
            116 => msg_type::MSG_REQ_REDIS_SREM,
            115 => msg_type::MSG_REQ_REDIS_SRANDMEMBER,
            114 => msg_type::MSG_REQ_REDIS_SPOP,
            113 => msg_type::MSG_REQ_REDIS_SMOVE,
            112 => msg_type::MSG_REQ_REDIS_SMEMBERS,
            111 => msg_type::MSG_REQ_REDIS_SMISMEMBER,
            110 => msg_type::MSG_REQ_REDIS_SISMEMBER,
            109 => msg_type::MSG_REQ_REDIS_SINTERSTORE,
            108 => msg_type::MSG_REQ_REDIS_SINTER,
            107 => msg_type::MSG_REQ_REDIS_SDIFFSTORE,
            106 => msg_type::MSG_REQ_REDIS_SDIFF,
            105 => msg_type::MSG_REQ_REDIS_SCARD,
            104 => msg_type::MSG_REQ_REDIS_SADD,
            103 => msg_type::MSG_REQ_REDIS_RPUSHX,
            102 => msg_type::MSG_REQ_REDIS_RPUSH,
            101 => msg_type::MSG_REQ_REDIS_RPOPLPUSH,
            100 => msg_type::MSG_REQ_REDIS_RPOP,
            99 => msg_type::MSG_REQ_REDIS_PFMERGE,
            98 => msg_type::MSG_REQ_REDIS_PFCOUNT,
            97 => msg_type::MSG_REQ_REDIS_PFADD,
            96 => msg_type::MSG_REQ_REDIS_LTRIM,
            95 => msg_type::MSG_REQ_REDIS_LSET,
            94 => msg_type::MSG_REQ_REDIS_LREM,
            93 => msg_type::MSG_REQ_REDIS_LRANGE,
            92 => msg_type::MSG_REQ_REDIS_LPUSHX,
            91 => msg_type::MSG_REQ_REDIS_LPUSH,
            90 => msg_type::MSG_REQ_REDIS_LPOS,
            89 => msg_type::MSG_REQ_REDIS_LPOP,
            88 => msg_type::MSG_REQ_REDIS_LMOVE,
            87 => msg_type::MSG_REQ_REDIS_LLEN,
            86 => msg_type::MSG_REQ_REDIS_LINSERT,
            85 => msg_type::MSG_REQ_REDIS_LINDEX,
            84 => msg_type::MSG_REQ_REDIS_HVALS,
            83 => msg_type::MSG_REQ_REDIS_HSTRLEN,
            82 => msg_type::MSG_REQ_REDIS_HSCAN,
            81 => msg_type::MSG_REQ_REDIS_HSETNX,
            80 => msg_type::MSG_REQ_REDIS_HSET,
            79 => msg_type::MSG_REQ_REDIS_HRANDFIELD,
            78 => msg_type::MSG_REQ_REDIS_HMSET,
            77 => msg_type::MSG_REQ_REDIS_HMGET,
            76 => msg_type::MSG_REQ_REDIS_HLEN,
            75 => msg_type::MSG_REQ_REDIS_HKEYS,
            74 => msg_type::MSG_REQ_REDIS_HINCRBYFLOAT,
            73 => msg_type::MSG_REQ_REDIS_HINCRBY,
            72 => msg_type::MSG_REQ_REDIS_HGETALL,
            71 => msg_type::MSG_REQ_REDIS_HGET,
            70 => msg_type::MSG_REQ_REDIS_HEXISTS,
            69 => msg_type::MSG_REQ_REDIS_HDEL,
            68 => msg_type::MSG_REQ_REDIS_STRLEN,
            67 => msg_type::MSG_REQ_REDIS_SETRANGE,
            66 => msg_type::MSG_REQ_REDIS_SETNX,
            65 => msg_type::MSG_REQ_REDIS_SETEX,
            64 => msg_type::MSG_REQ_REDIS_SETBIT,
            63 => msg_type::MSG_REQ_REDIS_SET,
            62 => msg_type::MSG_REQ_REDIS_RESTORE,
            61 => msg_type::MSG_REQ_REDIS_PSETEX,
            60 => msg_type::MSG_REQ_REDIS_MSET,
            59 => msg_type::MSG_REQ_REDIS_MGET,
            58 => msg_type::MSG_REQ_REDIS_INCRBYFLOAT,
            57 => msg_type::MSG_REQ_REDIS_INCRBY,
            56 => msg_type::MSG_REQ_REDIS_INCR,
            55 => msg_type::MSG_REQ_REDIS_GETSET,
            54 => msg_type::MSG_REQ_REDIS_GETRANGE,
            53 => msg_type::MSG_REQ_REDIS_GETEX,
            52 => msg_type::MSG_REQ_REDIS_GETDEL,
            51 => msg_type::MSG_REQ_REDIS_GETBIT,
            50 => msg_type::MSG_REQ_REDIS_GET,
            49 => msg_type::MSG_REQ_REDIS_DUMP,
            48 => msg_type::MSG_REQ_REDIS_DECRBY,
            47 => msg_type::MSG_REQ_REDIS_DECR,
            46 => msg_type::MSG_REQ_REDIS_BITPOS,
            45 => msg_type::MSG_REQ_REDIS_BITFIELD,
            44 => msg_type::MSG_REQ_REDIS_BITCOUNT,
            43 => msg_type::MSG_REQ_REDIS_APPEND,
            42 => msg_type::MSG_REQ_REDIS_UNLINK,
            41 => msg_type::MSG_REQ_REDIS_TYPE,
            40 => msg_type::MSG_REQ_REDIS_TTL,
            39 => msg_type::MSG_REQ_REDIS_TOUCH,
            38 => msg_type::MSG_REQ_REDIS_SORT,
            37 => msg_type::MSG_REQ_REDIS_PTTL,
            36 => msg_type::MSG_REQ_REDIS_PERSIST,
            35 => msg_type::MSG_REQ_REDIS_PEXPIREAT,
            34 => msg_type::MSG_REQ_REDIS_PEXPIRE,
            33 => msg_type::MSG_REQ_REDIS_MOVE,
            32 => msg_type::MSG_REQ_REDIS_EXPIREAT,
            31 => msg_type::MSG_REQ_REDIS_EXPIRE,
            30 => msg_type::MSG_REQ_REDIS_EXISTS,
            29 => msg_type::MSG_REQ_REDIS_DEL,
            28 => msg_type::MSG_REQ_REDIS_COPY,
            27 => msg_type::MSG_RSP_MC_SERVER_ERROR,
            26 => msg_type::MSG_RSP_MC_CLIENT_ERROR,
            25 => msg_type::MSG_RSP_MC_ERROR,
            24 => msg_type::MSG_RSP_MC_VERSION,
            23 => msg_type::MSG_RSP_MC_TOUCHED,
            22 => msg_type::MSG_RSP_MC_DELETED,
            21 => msg_type::MSG_RSP_MC_VALUE,
            20 => msg_type::MSG_RSP_MC_END,
            19 => msg_type::MSG_RSP_MC_NOT_FOUND,
            18 => msg_type::MSG_RSP_MC_EXISTS,
            17 => msg_type::MSG_RSP_MC_NOT_STORED,
            16 => msg_type::MSG_RSP_MC_STORED,
            15 => msg_type::MSG_RSP_MC_NUM,
            14 => msg_type::MSG_REQ_MC_VERSION,
            13 => msg_type::MSG_REQ_MC_QUIT,
            12 => msg_type::MSG_REQ_MC_TOUCH,
            11 => msg_type::MSG_REQ_MC_DECR,
            10 => msg_type::MSG_REQ_MC_INCR,
            9 => msg_type::MSG_REQ_MC_PREPEND,
            8 => msg_type::MSG_REQ_MC_APPEND,
            7 => msg_type::MSG_REQ_MC_REPLACE,
            6 => msg_type::MSG_REQ_MC_ADD,
            5 => msg_type::MSG_REQ_MC_SET,
            4 => msg_type::MSG_REQ_MC_CAS,
            3 => msg_type::MSG_REQ_MC_DELETE,
            2 => msg_type::MSG_REQ_MC_GETS,
            1 => msg_type::MSG_REQ_MC_GET,
            0 => msg_type::MSG_UNKNOWN,
            _ => panic!("Invalid value for msg_type: {}", value),
        }
    }
}
impl AddAssign<u32> for msg_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for msg_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for msg_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for msg_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for msg_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = msg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for msg_type {
    type Output = msg_type;
    fn add(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for msg_type {
    type Output = msg_type;
    fn sub(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for msg_type {
    type Output = msg_type;
    fn mul(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for msg_type {
    type Output = msg_type;
    fn div(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for msg_type {
    type Output = msg_type;
    fn rem(self, rhs: u32) -> msg_type {
        msg_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type msg_coalesce_t = Option<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option<
    unsafe extern "C" fn(*mut msg, uint32_t, *mut msg_tqh) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg_tqh {
    pub tqh_first: *mut msg,
    pub tqh_last: *mut *mut msg,
}
pub type msg_parse_result_t = msg_parse_result;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum msg_parse_result {
    MSG_PARSE_OK,
    MSG_PARSE_ERROR,
    MSG_PARSE_REPAIR,
    MSG_PARSE_AGAIN,
}
impl msg_parse_result {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            msg_parse_result::MSG_PARSE_OK => 0,
            msg_parse_result::MSG_PARSE_ERROR => 1,
            msg_parse_result::MSG_PARSE_REPAIR => 2,
            msg_parse_result::MSG_PARSE_AGAIN => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> msg_parse_result {
        match value {
            0 => msg_parse_result::MSG_PARSE_OK,
            1 => msg_parse_result::MSG_PARSE_ERROR,
            2 => msg_parse_result::MSG_PARSE_REPAIR,
            3 => msg_parse_result::MSG_PARSE_AGAIN,
            _ => panic!("Invalid value for msg_parse_result: {}", value),
        }
    }
}
impl AddAssign<u32> for msg_parse_result {
    fn add_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for msg_parse_result {
    fn sub_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for msg_parse_result {
    fn mul_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for msg_parse_result {
    fn div_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for msg_parse_result {
    fn rem_assign(&mut self, rhs: u32) {
        *self = msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn add(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn sub(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn mul(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn div(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for msg_parse_result {
    type Output = msg_parse_result;
    fn rem(self, rhs: u32) -> msg_parse_result {
        msg_parse_result::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type msg_parse_t = Option<unsafe extern "C" fn(*mut msg) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mhdr {
    pub stqh_first: *mut mbuf,
    pub stqh_last: *mut *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuf {
    pub magic: uint32_t,
    pub next: C2RustUnnamed_35,
    pub pos: *mut uint8_t,
    pub last: *mut uint8_t,
    pub start: *mut uint8_t,
    pub end: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub stqe_next: *mut mbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rbnode {
    pub left: *mut rbnode,
    pub right: *mut rbnode,
    pub parent: *mut rbnode,
    pub key: int64_t,
    pub data: *mut libc::c_void,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub tqe_next: *mut msg,
    pub tqe_prev: *mut *mut msg,
}
pub type conn_unref_t = Option<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut server) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub idx: uint32_t,
    pub owner: *mut server_pool,
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub weight: uint32_t,
    pub info: sockinfo,
    pub ns_conn_q: uint32_t,
    pub s_conn_q: conn_tqh,
    pub next_retry: int64_t,
    pub failure_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conn_tqh {
    pub tqh_first: *mut conn,
    pub tqh_last: *mut *mut conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockinfo {
    pub family: i32,
    pub addrlen: socklen_t,
    pub addr: C2RustUnnamed_39,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub in_0: sockaddr_in,
    pub in6: sockaddr_in6,
    pub un: sockaddr_un,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [i8; 108],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_40,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_40 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type socklen_t = __socklen_t;
pub type __socklen_t = u32;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct server_pool {
    pub idx: uint32_t,
    pub ctx: *mut context,
    pub p_conn: *mut conn,
    pub nc_conn_q: uint32_t,
    pub c_conn_q: conn_tqh,
    pub server: array,
    pub ncontinuum: uint32_t,
    pub nserver_continuum: uint32_t,
    pub continuum: *mut continuum,
    pub nlive_server: uint32_t,
    pub next_rebuild: int64_t,
    pub name: string,
    pub addrstr: string,
    pub port: uint16_t,
    pub info: sockinfo,
    pub perm: mode_t,
    pub dist_type: i32,
    pub key_hash_type: i32,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: i32,
    pub backlog: i32,
    pub redis_db: i32,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: u32,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option<unsafe extern "C" fn(*const i8, size_t) -> uint32_t>;
pub type mode_t = __mode_t;
pub type __mode_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [i8; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
pub type array_compare_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum hash_type {
    HASH_SENTINEL = 12,
    HASH_JENKINS = 11,
    HASH_MURMUR = 10,
    HASH_HSIEH = 9,
    HASH_FNV1A_32 = 8,
    HASH_FNV1_32 = 7,
    HASH_FNV1A_64 = 6,
    HASH_FNV1_64 = 5,
    HASH_CRC32A = 4,
    HASH_CRC32 = 3,
    HASH_CRC16 = 2,
    HASH_MD5 = 1,
    HASH_ONE_AT_A_TIME = 0,
}
impl hash_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            hash_type::HASH_SENTINEL => 12,
            hash_type::HASH_JENKINS => 11,
            hash_type::HASH_MURMUR => 10,
            hash_type::HASH_HSIEH => 9,
            hash_type::HASH_FNV1A_32 => 8,
            hash_type::HASH_FNV1_32 => 7,
            hash_type::HASH_FNV1A_64 => 6,
            hash_type::HASH_FNV1_64 => 5,
            hash_type::HASH_CRC32A => 4,
            hash_type::HASH_CRC32 => 3,
            hash_type::HASH_CRC16 => 2,
            hash_type::HASH_MD5 => 1,
            hash_type::HASH_ONE_AT_A_TIME => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> hash_type {
        match value {
            12 => hash_type::HASH_SENTINEL,
            11 => hash_type::HASH_JENKINS,
            10 => hash_type::HASH_MURMUR,
            9 => hash_type::HASH_HSIEH,
            8 => hash_type::HASH_FNV1A_32,
            7 => hash_type::HASH_FNV1_32,
            6 => hash_type::HASH_FNV1A_64,
            5 => hash_type::HASH_FNV1_64,
            4 => hash_type::HASH_CRC32A,
            3 => hash_type::HASH_CRC32,
            2 => hash_type::HASH_CRC16,
            1 => hash_type::HASH_MD5,
            0 => hash_type::HASH_ONE_AT_A_TIME,
            _ => panic!("Invalid value for hash_type: {}", value),
        }
    }
}
impl AddAssign<u32> for hash_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = hash_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for hash_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = hash_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for hash_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = hash_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for hash_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = hash_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for hash_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = hash_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for hash_type {
    type Output = hash_type;
    fn add(self, rhs: u32) -> hash_type {
        hash_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for hash_type {
    type Output = hash_type;
    fn sub(self, rhs: u32) -> hash_type {
        hash_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for hash_type {
    type Output = hash_type;
    fn mul(self, rhs: u32) -> hash_type {
        hash_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for hash_type {
    type Output = hash_type;
    fn div(self, rhs: u32) -> hash_type {
        hash_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for hash_type {
    type Output = hash_type;
    fn rem(self, rhs: u32) -> hash_type {
        hash_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type hash_type_t = hash_type;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum dist_type {
    DIST_SENTINEL = 3,
    DIST_RANDOM = 2,
    DIST_MODULA = 1,
    DIST_KETAMA = 0,
}
impl dist_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            dist_type::DIST_SENTINEL => 3,
            dist_type::DIST_RANDOM => 2,
            dist_type::DIST_MODULA => 1,
            dist_type::DIST_KETAMA => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> dist_type {
        match value {
            3 => dist_type::DIST_SENTINEL,
            2 => dist_type::DIST_RANDOM,
            1 => dist_type::DIST_MODULA,
            0 => dist_type::DIST_KETAMA,
            _ => panic!("Invalid value for dist_type: {}", value),
        }
    }
}
impl AddAssign<u32> for dist_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = dist_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for dist_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = dist_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for dist_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = dist_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for dist_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = dist_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for dist_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = dist_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for dist_type {
    type Output = dist_type;
    fn add(self, rhs: u32) -> dist_type {
        dist_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for dist_type {
    type Output = dist_type;
    fn sub(self, rhs: u32) -> dist_type {
        dist_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for dist_type {
    type Output = dist_type;
    fn mul(self, rhs: u32) -> dist_type {
        dist_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for dist_type {
    type Output = dist_type;
    fn div(self, rhs: u32) -> dist_type {
        dist_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for dist_type {
    type Output = dist_type;
    fn rem(self, rhs: u32) -> dist_type {
        dist_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type dist_type_t = dist_type;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf_listen {
    pub pname: string,
    pub name: string,
    pub port: i32,
    pub perm: mode_t,
    pub info: sockinfo,
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "0..=0")]
    pub valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf_server {
    pub pname: string,
    pub name: string,
    pub addrstr: string,
    pub port: i32,
    pub weight: i32,
    pub info: sockinfo,
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "0..=0")]
    pub valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf_pool {
    pub name: string,
    pub listen: conf_listen,
    pub hash: hash_type_t,
    pub hash_tag: string,
    pub distribution: dist_type_t,
    pub timeout: i32,
    pub backlog: i32,
    pub client_connections: i32,
    pub tcpkeepalive: i32,
    pub redis: i32,
    pub redis_auth: string,
    pub redis_db: i32,
    pub preconnect: i32,
    pub auto_eject_hosts: i32,
    pub server_connections: i32,
    pub server_retry_timeout: i32,
    pub server_failure_limit: i32,
    pub server: array,
    #[bitfield(name = "valid", ty = "libc::c_uint", bits = "0..=0")]
    pub valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub reuseport: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub name: string,
    pub set: Option<
        unsafe extern "C" fn(*mut conf, *const command, *mut libc::c_void) -> *const i8,
    >,
    pub offset: i32,
}
#[inline]
unsafe extern "C" fn array_null(mut a: *mut array) {
    (*a).nelem = 0 as i32 as uint32_t;
    (*a).elem = 0 as *mut libc::c_void;
    (*a).size = 0 as i32 as size_t;
    (*a).nalloc = 0 as i32 as uint32_t;
}
#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[inline]
unsafe extern "C" fn _nc_strrchr(
    mut p: *mut uint8_t,
    mut start: *mut uint8_t,
    mut c: uint8_t,
) -> *mut uint8_t {
    while p >= start {
        if *p as i32 == c as i32 {
            return p;
        }
        p = p.offset(-1);
        p;
    }
    return 0 as *mut uint8_t;
}
static mut hash_strings: [string; 13] = [string {
    len: 0,
    data: 0 as *mut uint8_t,
}; 13];
static mut hash_algos: [hash_t; 13] = unsafe {
    [
        Some(hash_one_at_a_time as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_md5 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_crc16 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_crc32 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_crc32a as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_fnv1_64 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_fnv1a_64 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_fnv1_32 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_fnv1a_32 as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_hsieh as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_murmur as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        Some(hash_jenkins as unsafe extern "C" fn(*const i8, size_t) -> uint32_t),
        None,
    ]
};
static mut dist_strings: [string; 4] = [string {
    len: 0,
    data: 0 as *mut uint8_t,
}; 4];
static mut conf_commands: [command; 19] = [command {
    name: string {
        len: 0,
        data: 0 as *mut uint8_t,
    },
    set: None,
    offset: 0,
}; 19];
static mut true_str: string = string {
    len: 0,
    data: 0 as *mut uint8_t,
};
static mut false_str: string = string {
    len: 0,
    data: 0 as *mut uint8_t,
};
unsafe extern "C" fn conf_server_init(mut cs: *mut conf_server) {
    string_init(&mut (*cs).pname);
    string_init(&mut (*cs).name);
    string_init(&mut (*cs).addrstr);
    (*cs).port = 0 as i32;
    (*cs).weight = 0 as i32;
    memset(
        &mut (*cs).info as *mut sockinfo as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    (*cs).set_valid(0 as i32 as u32);
}
unsafe extern "C" fn conf_server_deinit(mut cs: *mut conf_server) {
    string_deinit(&mut (*cs).pname);
    string_deinit(&mut (*cs).name);
    string_deinit(&mut (*cs).addrstr);
    (*cs).set_valid(0 as i32 as u32);
}
#[no_mangle]
pub unsafe extern "C" fn conf_server_each_transform(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut cs: *mut conf_server = elem as *mut conf_server;
    let mut server: *mut array = data as *mut array;
    let mut s: *mut server = 0 as *mut server;
    s = array_push(server) as *mut server;
    (*s).idx = array_idx(server, s as *const libc::c_void);
    (*s).owner = 0 as *mut server_pool;
    (*s).pname = (*cs).pname;
    (*s).name = (*cs).name;
    (*s).addrstr = (*cs).addrstr;
    (*s).port = (*cs).port as uint16_t;
    (*s).weight = (*cs).weight as uint32_t;
    memcpy(
        &mut (*s).info as *mut sockinfo as *mut libc::c_void,
        &mut (*cs).info as *mut sockinfo as *const libc::c_void,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    (*s).ns_conn_q = 0 as i32 as uint32_t;
    (*s).s_conn_q.tqh_first = 0 as *mut conn;
    (*s).s_conn_q.tqh_last = &mut (*s).s_conn_q.tqh_first;
    (*s).next_retry = 0 as libc::c_longlong as int64_t;
    (*s).failure_count = 0 as i32 as uint32_t;
    return 0 as i32;
}
unsafe extern "C" fn conf_pool_init(
    mut cp: *mut conf_pool,
    mut name: *const string,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    string_init(&mut (*cp).name);
    string_init(&mut (*cp).listen.pname);
    string_init(&mut (*cp).listen.name);
    string_init(&mut (*cp).redis_auth);
    (*cp).listen.port = 0 as i32;
    memset(
        &mut (*cp).listen.info as *mut sockinfo as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    ((*cp).listen).set_valid(0 as i32 as u32);
    (*cp).hash = 4294967295 as hash_type_t;
    string_init(&mut (*cp).hash_tag);
    (*cp).distribution = 4294967295 as dist_type_t;
    (*cp).timeout = -(1 as i32);
    (*cp).backlog = -(1 as i32);
    (*cp).client_connections = -(1 as i32);
    (*cp).redis = -(1 as i32);
    (*cp).tcpkeepalive = -(1 as i32);
    (*cp).reuseport = -(1 as i32);
    (*cp).redis_db = -(1 as i32);
    (*cp).preconnect = -(1 as i32);
    (*cp).auto_eject_hosts = -(1 as i32);
    (*cp).server_connections = -(1 as i32);
    (*cp).server_retry_timeout = -(1 as i32);
    (*cp).server_failure_limit = -(1 as i32);
    array_null(&mut (*cp).server);
    (*cp).set_valid(0 as i32 as u32);
    status = string_duplicate(&mut (*cp).name, name);
    if status != 0 as i32 {
        return status;
    }
    status = array_init(
        &mut (*cp).server,
        8 as i32 as uint32_t,
        ::core::mem::size_of::<conf_server>() as u64,
    );
    if status != 0 as i32 {
        string_deinit(&mut (*cp).name);
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_pool_deinit(mut cp: *mut conf_pool) {
    string_deinit(&mut (*cp).name);
    string_deinit(&mut (*cp).listen.pname);
    string_deinit(&mut (*cp).listen.name);
    if (*cp).redis_auth.len > 0 as i32 as u32 {
        string_deinit(&mut (*cp).redis_auth);
    }
    while array_n(&mut (*cp).server) != 0 as i32 as u32 {
        conf_server_deinit(array_pop(&mut (*cp).server) as *mut conf_server);
    }
    array_deinit(&mut (*cp).server);
}
#[no_mangle]
pub unsafe extern "C" fn conf_pool_each_transform(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut cp: *mut conf_pool = elem as *mut conf_pool;
    let mut server_pool: *mut array = data as *mut array;
    let mut sp: *mut server_pool = 0 as *mut server_pool;
    sp = array_push(server_pool) as *mut server_pool;
    (*sp).idx = array_idx(server_pool, sp as *const libc::c_void);
    (*sp).ctx = 0 as *mut context;
    (*sp).p_conn = 0 as *mut conn;
    (*sp).nc_conn_q = 0 as i32 as uint32_t;
    (*sp).c_conn_q.tqh_first = 0 as *mut conn;
    (*sp).c_conn_q.tqh_last = &mut (*sp).c_conn_q.tqh_first;
    array_null(&mut (*sp).server);
    (*sp).ncontinuum = 0 as i32 as uint32_t;
    (*sp).nserver_continuum = 0 as i32 as uint32_t;
    (*sp).continuum = 0 as *mut continuum;
    (*sp).nlive_server = 0 as i32 as uint32_t;
    (*sp).next_rebuild = 0 as libc::c_longlong as int64_t;
    (*sp).name = (*cp).name;
    (*sp).addrstr = (*cp).listen.pname;
    (*sp).port = (*cp).listen.port as uint16_t;
    memcpy(
        &mut (*sp).info as *mut sockinfo as *mut libc::c_void,
        &mut (*cp).listen.info as *mut sockinfo as *const libc::c_void,
        ::core::mem::size_of::<sockinfo>() as u64,
    );
    (*sp).perm = (*cp).listen.perm;
    (*sp).key_hash_type = (*cp).hash as i32;
    (*sp).key_hash = hash_algos[(*cp).hash as usize];
    (*sp).dist_type = (*cp).distribution as i32;
    (*sp).hash_tag = (*cp).hash_tag;
    (*sp)
        .set_tcpkeepalive(
            (if (*cp).tcpkeepalive != 0 { 1 as i32 } else { 0 as i32 }) as u32,
        );
    (*sp).set_reuseport((if (*cp).reuseport != 0 { 1 as i32 } else { 0 as i32 }) as u32);
    (*sp).set_redis((if (*cp).redis != 0 { 1 as i32 } else { 0 as i32 }) as u32);
    (*sp).timeout = (*cp).timeout;
    (*sp).backlog = (*cp).backlog;
    (*sp).redis_db = (*cp).redis_db;
    (*sp).redis_auth = (*cp).redis_auth;
    (*sp).require_auth = (if (*cp).redis_auth.len > 0 as i32 as u32 {
        1 as i32
    } else {
        0 as i32
    }) as u32;
    (*sp).client_connections = (*cp).client_connections as uint32_t;
    (*sp).server_connections = (*cp).server_connections as uint32_t;
    (*sp).server_retry_timeout = ((*cp).server_retry_timeout as int64_t
        as libc::c_longlong * 1000 as libc::c_longlong) as int64_t;
    (*sp).server_failure_limit = (*cp).server_failure_limit as uint32_t;
    (*sp)
        .set_auto_eject_hosts(
            (if (*cp).auto_eject_hosts != 0 { 1 as i32 } else { 0 as i32 }) as u32,
        );
    (*sp)
        .set_preconnect(
            (if (*cp).preconnect != 0 { 1 as i32 } else { 0 as i32 }) as u32,
        );
    status = server_init(&mut (*sp).server, &mut (*cp).server, sp);
    if status != 0 as i32 {
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_dump(mut cf: *const conf) {
    let mut i: uint32_t = 0;
    let mut j: uint32_t = 0;
    let mut npool: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    let mut cp: *mut conf_pool = 0 as *mut conf_pool;
    let mut s: *mut string = 0 as *mut string;
    npool = array_n(&(*cf).pool);
    if npool == 0 as i32 as u32 {
        return;
    }
    i = 0 as i32 as uint32_t;
    while i < npool {
        cp = array_get(&(*cf).pool, i) as *mut conf_pool;
        nserver = array_n(&mut (*cp).server);
        j = 0 as i32 as uint32_t;
        while j < nserver {
            s = array_get(&mut (*cp).server, j) as *mut string;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn conf_yaml_init(mut cf: *mut conf) -> rstatus_t {
    let mut rv: i32 = 0;
    rv = fseek((*cf).fh, 0 as i64, 0 as i32);
    if rv < 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                385 as i32,
                0 as i32,
                b"conf: failed to seek to the beginning of file '%s': %s\0" as *const u8
                    as *const i8,
                (*cf).fname,
                strerror(*__errno_location()),
            );
        }
        return -(1 as i32);
    }
    rv = yaml_parser_initialize(&mut (*cf).parser);
    if rv == 0 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                392 as i32,
                0 as i32,
                b"conf: failed (err %d) to initialize yaml parser\0" as *const u8
                    as *const i8,
                (*cf).parser.error as u32,
            );
        }
        return -(1 as i32);
    }
    yaml_parser_set_input_file(&mut (*cf).parser, (*cf).fh);
    (*cf).set_valid_parser(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_yaml_deinit(mut cf: *mut conf) {
    if (*cf).valid_parser() != 0 {
        yaml_parser_delete(&mut (*cf).parser);
        (*cf).set_valid_parser(0 as i32 as u32);
    }
}
unsafe extern "C" fn conf_token_next(mut cf: *mut conf) -> rstatus_t {
    let mut rv: i32 = 0;
    rv = yaml_parser_scan(&mut (*cf).parser, &mut (*cf).token);
    if rv == 0 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                420 as i32,
                0 as i32,
                b"conf: failed (err %d) to scan next token\0" as *const u8 as *const i8,
                (*cf).parser.error as u32,
            );
        }
        return -(1 as i32);
    }
    (*cf).set_valid_token(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_token_done(mut cf: *mut conf) {
    if (*cf).valid_token() != 0 {
        yaml_token_delete(&mut (*cf).token);
        (*cf).set_valid_token(0 as i32 as u32);
    }
}
unsafe extern "C" fn conf_event_next(mut cf: *mut conf) -> rstatus_t {
    let mut rv: i32 = 0;
    rv = yaml_parser_parse(&mut (*cf).parser, &mut (*cf).event);
    if rv == 0 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                448 as i32,
                0 as i32,
                b"conf: failed (err %d) to get next event\0" as *const u8 as *const i8,
                (*cf).parser.error as u32,
            );
        }
        return -(1 as i32);
    }
    (*cf).set_valid_event(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_event_done(mut cf: *mut conf) {
    if (*cf).valid_event() != 0 {
        yaml_event_delete(&mut (*cf).event);
        (*cf).set_valid_event(0 as i32 as u32);
    }
}
unsafe extern "C" fn conf_push_scalar(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut value: *mut string = 0 as *mut string;
    let mut scalar: *mut uint8_t = 0 as *mut uint8_t;
    let mut scalar_len: uint32_t = 0;
    scalar = (*cf).event.data.scalar.value;
    scalar_len = (*cf).event.data.scalar.length as uint32_t;
    if scalar_len == 0 as i32 as u32 {
        return -(1 as i32);
    }
    value = array_push(&mut (*cf).arg) as *mut string;
    if value.is_null() {
        return -(3 as i32);
    }
    string_init(value);
    status = string_copy(value, scalar, scalar_len);
    if status != 0 as i32 {
        array_pop(&mut (*cf).arg);
        return status;
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_pop_scalar(mut cf: *mut conf) {
    let mut value: *mut string = 0 as *mut string;
    value = array_pop(&mut (*cf).arg) as *mut string;
    string_deinit(value);
}
unsafe extern "C" fn conf_handler(
    mut cf: *mut conf,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut cmd: *const command = 0 as *const command;
    let mut key: *mut string = 0 as *mut string;
    let mut value: *mut string = 0 as *mut string;
    let mut narg: uint32_t = 0;
    if array_n(&mut (*cf).arg) == 1 as i32 as u32 {
        value = array_top(&mut (*cf).arg) as *mut string;
        return conf_pool_init(data as *mut conf_pool, value);
    }
    narg = array_n(&mut (*cf).arg);
    value = array_get(&mut (*cf).arg, narg.wrapping_sub(1 as i32 as u32)) as *mut string;
    key = array_get(&mut (*cf).arg, narg.wrapping_sub(2 as i32 as u32)) as *mut string;
    cmd = conf_commands.as_ptr();
    while (*cmd).name.len != 0 as i32 as u32 {
        let mut rv: *const i8 = 0 as *const i8;
        if string_compare(key, &(*cmd).name) != 0 as i32 {
            cmd = cmd.offset(1);
            cmd;
        } else {
            rv = ((*cmd).set).expect("non-null function pointer")(cf, cmd, data);
            if rv != 0 as *mut libc::c_void as *const i8 {
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        535 as i32,
                        0 as i32,
                        b"conf: directive \"%.*s\" %s\0" as *const u8 as *const i8,
                        (*key).len,
                        (*key).data,
                        rv,
                    );
                }
                return -(1 as i32);
            }
            return 0 as i32;
        }
    }
    if log_loggable(1 as i32) != 0 as i32 {
        _log(
            b"nc_conf.c\0" as *const u8 as *const i8,
            542 as i32,
            0 as i32,
            b"conf: directive \"%.*s\" is unknown\0" as *const u8 as *const i8,
            (*key).len,
            (*key).data,
        );
    }
    return -(1 as i32);
}
unsafe extern "C" fn conf_begin_parse(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut done: bool = false;
    status = conf_yaml_init(cf);
    if status != 0 as i32 {
        return status;
    }
    done = 0 as i32 != 0;
    loop {
        status = conf_event_next(cf);
        if status != 0 as i32 {
            return status;
        }
        match (*cf).event.type_0 as u32 {
            9 => {
                (*cf).depth = ((*cf).depth).wrapping_add(1);
                (*cf).depth;
                done = 1 as i32 != 0;
            }
            1 | 3 | _ => {}
        }
        conf_event_done(cf);
        if done {
            break;
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_end_parse(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut done: bool = false;
    done = 0 as i32 != 0;
    loop {
        status = conf_event_next(cf);
        if status != 0 as i32 {
            return status;
        }
        match (*cf).event.type_0 as u32 {
            2 => {
                done = 1 as i32 != 0;
            }
            4 | _ => {}
        }
        conf_event_done(cf);
        if done {
            break;
        }
    }
    conf_yaml_deinit(cf);
    return 0 as i32;
}
unsafe extern "C" fn conf_parse_core(
    mut cf: *mut conf,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut done: bool = false;
    let mut leaf: bool = false;
    let mut new_pool: bool = false;
    status = conf_event_next(cf);
    if status != 0 as i32 {
        return status;
    }
    done = 0 as i32 != 0;
    leaf = 0 as i32 != 0;
    new_pool = 0 as i32 != 0;
    match (*cf).event.type_0 as u32 {
        10 => {
            (*cf).depth = ((*cf).depth).wrapping_sub(1);
            (*cf).depth;
            if (*cf).depth == 1 as i32 as u32 {
                conf_pop_scalar(cf);
            } else if (*cf).depth == 0 as i32 as u32 {
                done = 1 as i32 != 0;
            }
        }
        9 => {
            (*cf).depth = ((*cf).depth).wrapping_add(1);
            (*cf).depth;
        }
        7 => {
            (*cf).set_seq(1 as i32 as u32);
        }
        8 => {
            conf_pop_scalar(cf);
            (*cf).set_seq(0 as i32 as u32);
        }
        6 => {
            status = conf_push_scalar(cf);
            if !(status != 0 as i32) {
                if (*cf).seq() != 0 {
                    leaf = 1 as i32 != 0;
                } else if (*cf).depth == 1 as i32 as u32 {
                    data = array_push(&mut (*cf).pool);
                    if data.is_null() {
                        status = -(3 as i32);
                    } else {
                        new_pool = 1 as i32 != 0;
                    }
                } else if array_n(&mut (*cf).arg)
                    == ((*cf).depth).wrapping_add(1 as i32 as u32)
                {
                    leaf = 1 as i32 != 0;
                }
            }
        }
        _ => {}
    }
    conf_event_done(cf);
    if status != 0 as i32 {
        return status;
    }
    if done {
        return 0 as i32;
    }
    if leaf as i32 != 0 || new_pool as i32 != 0 {
        status = conf_handler(cf, data);
        if leaf {
            conf_pop_scalar(cf);
            if (*cf).seq() == 0 {
                conf_pop_scalar(cf);
            }
        }
        if status != 0 as i32 {
            return status;
        }
    }
    return conf_parse_core(cf, data);
}
unsafe extern "C" fn conf_parse(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = conf_begin_parse(cf);
    if status != 0 as i32 {
        return status;
    }
    status = conf_parse_core(cf, 0 as *mut libc::c_void);
    if status != 0 as i32 {
        return status;
    }
    status = conf_end_parse(cf);
    if status != 0 as i32 {
        return status;
    }
    (*cf).set_parsed(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_open(mut filename: *const i8) -> *mut conf {
    let mut status: rstatus_t = 0;
    let mut cf: *mut conf = 0 as *mut conf;
    let mut fh: *mut FILE = 0 as *mut FILE;
    fh = fopen(filename, b"r\0" as *const u8 as *const i8);
    if fh.is_null() {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                771 as i32,
                0 as i32,
                b"conf: failed to open configuration '%s': %s\0" as *const u8
                    as *const i8,
                filename,
                strerror(*__errno_location()),
            );
        }
        return 0 as *mut conf;
    }
    cf = _nc_alloc(
        ::core::mem::size_of::<conf>() as u64,
        b"nc_conf.c\0" as *const u8 as *const i8,
        775 as i32,
    ) as *mut conf;
    if cf.is_null() {
        fclose(fh);
        return 0 as *mut conf;
    }
    status = array_init(
        &mut (*cf).arg,
        3 as i32 as uint32_t,
        ::core::mem::size_of::<string>() as u64,
    );
    if status != 0 as i32 {
        _nc_free(
            cf as *mut libc::c_void,
            b"nc_conf.c\0" as *const u8 as *const i8,
            783 as i32,
        );
        cf = 0 as *mut conf;
        fclose(fh);
        return 0 as *mut conf;
    }
    status = array_init(
        &mut (*cf).pool,
        8 as i32 as uint32_t,
        ::core::mem::size_of::<conf_pool>() as u64,
    );
    if status != 0 as i32 {
        array_deinit(&mut (*cf).arg);
        _nc_free(
            cf as *mut libc::c_void,
            b"nc_conf.c\0" as *const u8 as *const i8,
            791 as i32,
        );
        cf = 0 as *mut conf;
        fclose(fh);
        return 0 as *mut conf;
    }
    (*cf).fname = filename;
    (*cf).fh = fh;
    (*cf).depth = 0 as i32 as uint32_t;
    (*cf).set_seq(0 as i32 as u32);
    (*cf).set_valid_parser(0 as i32 as u32);
    (*cf).set_valid_event(0 as i32 as u32);
    (*cf).set_valid_token(0 as i32 as u32);
    (*cf).set_sound(0 as i32 as u32);
    (*cf).set_parsed(0 as i32 as u32);
    (*cf).set_valid(0 as i32 as u32);
    return cf;
}
unsafe extern "C" fn conf_validate_document(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut count: uint32_t = 0;
    let mut done: bool = false;
    status = conf_yaml_init(cf);
    if status != 0 as i32 {
        return status;
    }
    count = 0 as i32 as uint32_t;
    done = 0 as i32 != 0;
    loop {
        let mut document: yaml_document_t = yaml_document_t {
            nodes: C2RustUnnamed_17 {
                start: 0 as *mut yaml_node_t,
                end: 0 as *mut yaml_node_t,
                top: 0 as *mut yaml_node_t,
            },
            version_directive: 0 as *mut yaml_version_directive_t,
            tag_directives: C2RustUnnamed_16 {
                start: 0 as *mut yaml_tag_directive_t,
                end: 0 as *mut yaml_tag_directive_t,
            },
            start_implicit: 0,
            end_implicit: 0,
            start_mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
            end_mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
        };
        let mut node: *mut yaml_node_t = 0 as *mut yaml_node_t;
        let mut rv: i32 = 0;
        rv = yaml_parser_load(&mut (*cf).parser, &mut document);
        if rv == 0 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc_conf.c\0" as *const u8 as *const i8,
                    835 as i32,
                    0 as i32,
                    b"conf: failed (err %d) to get the next yaml document\0" as *const u8
                        as *const i8,
                    (*cf).parser.error as u32,
                );
            }
            conf_yaml_deinit(cf);
            return -(1 as i32);
        }
        node = yaml_document_get_root_node(&mut document);
        if node.is_null() {
            done = 1 as i32 != 0;
        } else {
            count = count.wrapping_add(1);
            count;
        }
        yaml_document_delete(&mut document);
        if done {
            break;
        }
    }
    conf_yaml_deinit(cf);
    if count != 1 as i32 as u32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                854 as i32,
                0 as i32,
                b"conf: '%s' must contain only 1 document; found %u documents\0"
                    as *const u8 as *const i8,
                (*cf).fname,
                count,
            );
        }
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_validate_tokens(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut done: bool = false;
    let mut error: bool = false;
    let mut type_0: i32 = 0;
    status = conf_yaml_init(cf);
    if status != 0 as i32 {
        return status;
    }
    done = 0 as i32 != 0;
    error = 0 as i32 != 0;
    loop {
        status = conf_token_next(cf);
        if status != 0 as i32 {
            return status;
        }
        type_0 = (*cf).token.type_0 as i32;
        match type_0 {
            0 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        885 as i32,
                        0 as i32,
                        b"conf: no token (%d) is disallowed\0" as *const u8 as *const i8,
                        type_0,
                    );
                }
            }
            3 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        890 as i32,
                        0 as i32,
                        b"conf: version directive token (%d) is disallowed\0"
                            as *const u8 as *const i8,
                        type_0,
                    );
                }
            }
            4 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        895 as i32,
                        0 as i32,
                        b"conf: tag directive token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            5 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        900 as i32,
                        0 as i32,
                        b"conf: document start token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            6 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        905 as i32,
                        0 as i32,
                        b"conf: document end token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            10 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        910 as i32,
                        0 as i32,
                        b"conf: flow sequence start token (%d) is disallowed\0"
                            as *const u8 as *const i8,
                        type_0,
                    );
                }
            }
            11 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        915 as i32,
                        0 as i32,
                        b"conf: flow sequence end token (%d) is disallowed\0"
                            as *const u8 as *const i8,
                        type_0,
                    );
                }
            }
            12 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        920 as i32,
                        0 as i32,
                        b"conf: flow mapping start token (%d) is disallowed\0"
                            as *const u8 as *const i8,
                        type_0,
                    );
                }
            }
            13 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        925 as i32,
                        0 as i32,
                        b"conf: flow mapping end token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            15 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        930 as i32,
                        0 as i32,
                        b"conf: flow entry token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            18 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        935 as i32,
                        0 as i32,
                        b"conf: alias token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            19 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        940 as i32,
                        0 as i32,
                        b"conf: anchor token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            20 => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        945 as i32,
                        0 as i32,
                        b"conf: tag token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
            7 | 8 | 9 | 14 | 16 | 17 | 21 | 1 => {}
            2 => {
                done = 1 as i32 != 0;
            }
            _ => {
                error = 1 as i32 != 0;
                if log_loggable(1 as i32) != 0 as i32 {
                    _log(
                        b"nc_conf.c\0" as *const u8 as *const i8,
                        969 as i32,
                        0 as i32,
                        b"conf: unknown token (%d) is disallowed\0" as *const u8
                            as *const i8,
                        type_0,
                    );
                }
            }
        }
        conf_token_done(cf);
        if !(!done && !error) {
            break;
        }
    }
    conf_yaml_deinit(cf);
    return if !error { 0 as i32 } else { -(1 as i32) };
}
unsafe extern "C" fn conf_validate_structure(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut type_0: i32 = 0;
    let mut depth: i32 = 0;
    let mut i: uint32_t = 0;
    let mut count: [uint32_t; 3] = [0; 3];
    let mut done: bool = false;
    let mut error: bool = false;
    let mut seq: bool = false;
    status = conf_yaml_init(cf);
    if status != 0 as i32 {
        return status;
    }
    done = 0 as i32 != 0;
    error = 0 as i32 != 0;
    seq = 0 as i32 != 0;
    depth = 0 as i32;
    i = 0 as i32 as uint32_t;
    while i < (1 as i32 + 1 as i32 + 1 as i32) as u32 {
        count[i as usize] = 0 as i32 as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    loop {
        status = conf_event_next(cf);
        if status != 0 as i32 {
            return status;
        }
        type_0 = (*cf).event.type_0 as i32;
        match type_0 {
            2 => {
                done = 1 as i32 != 0;
            }
            9 => {
                if depth == 1 as i32 && count[depth as usize] != 1 as i32 as u32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1050 as i32,
                            0 as i32,
                            b"conf: '%s' has more than one \"key:value\" at depth %d\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                            depth,
                        );
                    }
                } else if depth >= 1 as i32 + 1 as i32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1054 as i32,
                            0 as i32,
                            b"conf: '%s' has a depth greater than %d\0" as *const u8
                                as *const i8,
                            (*cf).fname,
                            1 as i32 + 1 as i32,
                        );
                    }
                }
                depth += 1;
                depth;
            }
            10 => {
                if depth == 1 as i32 + 1 as i32 {
                    if seq {
                        seq = 0 as i32 != 0;
                    } else {
                        error = 1 as i32 != 0;
                        if log_loggable(1 as i32) != 0 as i32 {
                            _log(
                                b"nc_conf.c\0" as *const u8 as *const i8,
                                1066 as i32,
                                0 as i32,
                                b"conf: '%s' missing sequence directive at depth %d\0"
                                    as *const u8 as *const i8,
                                (*cf).fname,
                                depth,
                            );
                        }
                    }
                }
                depth -= 1;
                depth;
                count[depth as usize] = 0 as i32 as uint32_t;
            }
            7 => {
                if seq {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1077 as i32,
                            0 as i32,
                            b"conf: '%s' has more than one sequence directive\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                        );
                    }
                } else if depth != 1 as i32 + 1 as i32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1081 as i32,
                            0 as i32,
                            b"conf: '%s' has sequence at depth %d instead of %d\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                            depth,
                            1 as i32 + 1 as i32,
                        );
                    }
                } else if count[depth as usize] != 1 as i32 as u32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1085 as i32,
                            0 as i32,
                            b"conf: '%s' has invalid \"key:value\" at depth %d\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                            depth,
                        );
                    }
                }
                seq = 1 as i32 != 0;
            }
            8 => {
                count[depth as usize] = 0 as i32 as uint32_t;
            }
            6 => {
                if depth == 0 as i32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1099 as i32,
                            0 as i32,
                            b"conf: '%s' has invalid empty \"key:\" at depth %d\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                            depth,
                        );
                    }
                } else if depth == 1 as i32 && count[depth as usize] != 0 as i32 as u32 {
                    error = 1 as i32 != 0;
                    if log_loggable(1 as i32) != 0 as i32 {
                        _log(
                            b"nc_conf.c\0" as *const u8 as *const i8,
                            1103 as i32,
                            0 as i32,
                            b"conf: '%s' has invalid mapping \"key:\" at depth %d\0"
                                as *const u8 as *const i8,
                            (*cf).fname,
                            depth,
                        );
                    }
                } else if depth == 1 as i32 + 1 as i32
                    && count[depth as usize] == 2 as i32 as u32
                {
                    count[depth as usize] = 0 as i32 as uint32_t;
                }
                count[depth as usize] = (count[depth as usize]).wrapping_add(1);
                count[depth as usize];
            }
            1 | 3 | 4 | _ => {}
        }
        conf_event_done(cf);
        if !(!done && !error) {
            break;
        }
    }
    conf_yaml_deinit(cf);
    return if !error { 0 as i32 } else { -(1 as i32) };
}
unsafe extern "C" fn conf_pre_validate(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = conf_validate_document(cf);
    if status != 0 as i32 {
        return status;
    }
    status = conf_validate_tokens(cf);
    if status != 0 as i32 {
        return status;
    }
    status = conf_validate_structure(cf);
    if status != 0 as i32 {
        return status;
    }
    (*cf).set_sound(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_server_name_cmp(
    mut t1: *const libc::c_void,
    mut t2: *const libc::c_void,
) -> i32 {
    let mut s1: *const conf_server = t1 as *const conf_server;
    let mut s2: *const conf_server = t2 as *const conf_server;
    return string_compare(&(*s1).name, &(*s2).name);
}
unsafe extern "C" fn conf_pool_name_cmp(
    mut t1: *const libc::c_void,
    mut t2: *const libc::c_void,
) -> i32 {
    let mut p1: *const conf_pool = t1 as *const conf_pool;
    let mut p2: *const conf_pool = t2 as *const conf_pool;
    return string_compare(&(*p1).name, &(*p2).name);
}
unsafe extern "C" fn conf_pool_listen_cmp(
    mut t1: *const libc::c_void,
    mut t2: *const libc::c_void,
) -> i32 {
    let mut p1: *const conf_pool = t1 as *const conf_pool;
    let mut p2: *const conf_pool = t2 as *const conf_pool;
    return string_compare(&(*p1).listen.pname, &(*p2).listen.pname);
}
unsafe extern "C" fn conf_validate_server(
    mut cf: *mut conf,
    mut cp: *mut conf_pool,
) -> rstatus_t {
    let mut i: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    let mut valid: bool = false;
    nserver = array_n(&mut (*cp).server);
    if nserver == 0 as i32 as u32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                1181 as i32,
                0 as i32,
                b"conf: pool '%.*s' has no servers\0" as *const u8 as *const i8,
                (*cp).name.len,
                (*cp).name.data,
            );
        }
        return -(1 as i32);
    }
    array_sort(
        &mut (*cp).server,
        Some(
            conf_server_name_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    valid = 1 as i32 != 0;
    i = 0 as i32 as uint32_t;
    while i < nserver.wrapping_sub(1 as i32 as u32) {
        let mut cs1: *mut conf_server = 0 as *mut conf_server;
        let mut cs2: *mut conf_server = 0 as *mut conf_server;
        cs1 = array_get(&mut (*cp).server, i) as *mut conf_server;
        cs2 = array_get(&mut (*cp).server, i.wrapping_add(1 as i32 as u32))
            as *mut conf_server;
        if string_compare(&mut (*cs1).name, &mut (*cs2).name) == 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc_conf.c\0" as *const u8 as *const i8,
                    1201 as i32,
                    0 as i32,
                    b"conf: pool '%.*s' has servers with same name '%.*s'\0" as *const u8
                        as *const i8,
                    (*cp).name.len,
                    (*cp).name.data,
                    (*cs1).name.len,
                    (*cs1).name.data,
                );
            }
            valid = 0 as i32 != 0;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !valid {
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn conf_validate_pool(
    mut cf: *mut conf,
    mut cp: *mut conf_pool,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    if ((*cp).listen).valid() == 0 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                1222 as i32,
                0 as i32,
                b"conf: directive \"listen:\" is missing\0" as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    if (*cp).distribution as u32 == 4294967295 as dist_type_t as u32 {
        (*cp).distribution = dist_type::DIST_KETAMA;
    }
    if (*cp).hash as u32 == 4294967295 as hash_type_t as u32 {
        (*cp).hash = hash_type::HASH_FNV1A_64;
    }
    if (*cp).timeout == -(1 as i32) {
        (*cp).timeout = -(1 as i32);
    }
    if (*cp).backlog == -(1 as i32) {
        (*cp).backlog = 512 as i32;
    }
    (*cp).client_connections = 0 as i32;
    if (*cp).redis == -(1 as i32) {
        (*cp).redis = 0 as i32;
    }
    if (*cp).tcpkeepalive == -(1 as i32) {
        (*cp).tcpkeepalive = 0 as i32;
    }
    if (*cp).reuseport == -(1 as i32) {
        (*cp).reuseport = 0 as i32;
    }
    if (*cp).redis_db == -(1 as i32) {
        (*cp).redis_db = 0 as i32;
    }
    if (*cp).preconnect == -(1 as i32) {
        (*cp).preconnect = 0 as i32;
    }
    if (*cp).auto_eject_hosts == -(1 as i32) {
        (*cp).auto_eject_hosts = 0 as i32;
    }
    if (*cp).server_connections == -(1 as i32) {
        (*cp).server_connections = 1 as i32;
    } else if (*cp).server_connections == 0 as i32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                1273 as i32,
                0 as i32,
                b"conf: directive \"server_connections:\" cannot be 0\0" as *const u8
                    as *const i8,
            );
        }
        return -(1 as i32);
    }
    if (*cp).server_retry_timeout == -(1 as i32) {
        (*cp).server_retry_timeout = 30 as i32 * 1000 as i32;
    }
    if (*cp).server_failure_limit == -(1 as i32) {
        (*cp).server_failure_limit = 2 as i32;
    }
    if (*cp).redis == 0 && (*cp).redis_auth.len > 0 as i32 as u32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                1286 as i32,
                0 as i32,
                b"conf: directive \"redis_auth:\" is only valid for a redis pool\0"
                    as *const u8 as *const i8,
            );
        }
        return -(1 as i32);
    }
    status = conf_validate_server(cf, cp);
    if status != 0 as i32 {
        return status;
    }
    (*cp).set_valid(1 as i32 as u32);
    return 0 as i32;
}
unsafe extern "C" fn conf_post_validate(mut cf: *mut conf) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    let mut valid: bool = false;
    npool = array_n(&mut (*cf).pool);
    if npool == 0 as i32 as u32 {
        if log_loggable(1 as i32) != 0 as i32 {
            _log(
                b"nc_conf.c\0" as *const u8 as *const i8,
                1312 as i32,
                0 as i32,
                b"conf: '%s' has no pools\0" as *const u8 as *const i8,
                (*cf).fname,
            );
        }
        return -(1 as i32);
    }
    i = 0 as i32 as uint32_t;
    while i < npool {
        let mut cp: *mut conf_pool = array_get(&mut (*cf).pool, i) as *mut conf_pool;
        status = conf_validate_pool(cf, cp);
        if status != 0 as i32 {
            return status;
        }
        i = i.wrapping_add(1);
        i;
    }
    array_sort(
        &mut (*cf).pool,
        Some(
            conf_pool_listen_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    valid = 1 as i32 != 0;
    i = 0 as i32 as uint32_t;
    while i < npool.wrapping_sub(1 as i32 as u32) {
        let mut p1: *mut conf_pool = 0 as *mut conf_pool;
        let mut p2: *mut conf_pool = 0 as *mut conf_pool;
        p1 = array_get(&mut (*cf).pool, i) as *mut conf_pool;
        p2 = array_get(&mut (*cf).pool, i.wrapping_add(1 as i32 as u32))
            as *mut conf_pool;
        if string_compare(&mut (*p1).listen.pname, &mut (*p2).listen.pname) == 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc_conf.c\0" as *const u8 as *const i8,
                    1338 as i32,
                    0 as i32,
                    b"conf: pools '%.*s' and '%.*s' have the same listen address '%.*s'\0"
                        as *const u8 as *const i8,
                    (*p1).name.len,
                    (*p1).name.data,
                    (*p2).name.len,
                    (*p2).name.data,
                    (*p1).listen.pname.len,
                    (*p1).listen.pname.data,
                );
            }
            valid = 0 as i32 != 0;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !valid {
        return -(1 as i32);
    }
    array_sort(
        &mut (*cf).pool,
        Some(
            conf_pool_name_cmp
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    valid = 1 as i32 != 0;
    i = 0 as i32 as uint32_t;
    while i < npool.wrapping_sub(1 as i32 as u32) {
        let mut p1_0: *mut conf_pool = 0 as *mut conf_pool;
        let mut p2_0: *mut conf_pool = 0 as *mut conf_pool;
        p1_0 = array_get(&mut (*cf).pool, i) as *mut conf_pool;
        p2_0 = array_get(&mut (*cf).pool, i.wrapping_add(1 as i32 as u32))
            as *mut conf_pool;
        if string_compare(&mut (*p1_0).name, &mut (*p2_0).name) == 0 as i32 {
            if log_loggable(1 as i32) != 0 as i32 {
                _log(
                    b"nc_conf.c\0" as *const u8 as *const i8,
                    1357 as i32,
                    0 as i32,
                    b"conf: '%s' has pools with same name %.*s'\0" as *const u8
                        as *const i8,
                    (*cf).fname,
                    (*p1_0).name.len,
                    (*p1_0).name.data,
                );
            }
            valid = 0 as i32 != 0;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !valid {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn conf_create(mut filename: *const i8) -> *mut conf {
    let mut status: rstatus_t = 0;
    let mut cf: *mut conf = 0 as *mut conf;
    cf = conf_open(filename);
    if cf.is_null() {
        return 0 as *mut conf;
    }
    status = conf_pre_validate(cf);
    if !(status != 0 as i32) {
        status = conf_parse(cf);
        if !(status != 0 as i32) {
            status = conf_post_validate(cf);
            if !(status != 0 as i32) {
                conf_dump(cf);
                fclose((*cf).fh);
                (*cf).fh = 0 as *mut FILE;
                return cf;
            }
        }
    }
    _log_stderr(
        b"nutcracker: configuration file '%s' syntax is invalid\0" as *const u8
            as *const i8,
        filename,
    );
    fclose((*cf).fh);
    (*cf).fh = 0 as *mut FILE;
    conf_destroy(cf);
    return 0 as *mut conf;
}
#[no_mangle]
pub unsafe extern "C" fn conf_destroy(mut cf: *mut conf) {
    while array_n(&mut (*cf).arg) != 0 as i32 as u32 {
        conf_pop_scalar(cf);
    }
    array_deinit(&mut (*cf).arg);
    while array_n(&mut (*cf).pool) != 0 as i32 as u32 {
        conf_pool_deinit(array_pop(&mut (*cf).pool) as *mut conf_pool);
    }
    array_deinit(&mut (*cf).pool);
    _nc_free(
        cf as *mut libc::c_void,
        b"nc_conf.c\0" as *const u8 as *const i8,
        1427 as i32,
    );
    cf = 0 as *mut conf;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_string(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut status: rstatus_t = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut field: *mut string = 0 as *mut string;
    let mut value: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    field = p.offset((*cmd).offset as isize) as *mut string;
    if !((*field).data).is_null() {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    status = string_duplicate(field, value);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    return 0 as *mut libc::c_void as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_listen(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut status: rstatus_t = 0;
    let mut value: *mut string = 0 as *mut string;
    let mut field: *mut conf_listen = 0 as *mut conf_listen;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    let mut namelen: uint32_t = 0;
    p = conf as *mut uint8_t;
    field = p.offset((*cmd).offset as isize) as *mut conf_listen;
    if (*field).valid() as i32 == 1 as i32 {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *mut string;
    status = string_duplicate(&mut (*field).pname, value);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    if *((*value).data).offset(0 as i32 as isize) as i32 == '/' as i32 {
        let mut q: *mut uint8_t = 0 as *mut uint8_t;
        let mut start: *mut uint8_t = 0 as *mut uint8_t;
        let mut perm: *mut uint8_t = 0 as *mut uint8_t;
        p = ((*value).data).offset((*value).len as isize).offset(-(1 as i32 as isize));
        start = (*value).data;
        q = _nc_strrchr(p, start, ' ' as i32 as uint8_t);
        if q.is_null() {
            name = (*value).data;
            namelen = (*value).len;
            (*field).perm = 0 as i32 as mode_t;
        } else {
            perm = q.offset(1 as i32 as isize);
            p = q.offset(-(1 as i32 as isize));
            name = start;
            namelen = (p.offset_from(start) as i64 + 1 as i32 as i64) as uint32_t;
            *__errno_location() = 0 as i32;
            (*field).perm = strtol(perm as *mut i8, 0 as *mut *mut i8, 8 as i32)
                as mode_t;
            if *__errno_location() != 0 || (*field).perm > 0o777 as i32 as u32 {
                return b"has an invalid file permission in \"socket_path permission\" format string\0"
                    as *const u8 as *const i8;
            }
        }
    } else {
        let mut q_0: *mut uint8_t = 0 as *mut uint8_t;
        let mut start_0: *mut uint8_t = 0 as *mut uint8_t;
        let mut port: *mut uint8_t = 0 as *mut uint8_t;
        let mut portlen: uint32_t = 0;
        p = ((*value).data).offset((*value).len as isize).offset(-(1 as i32 as isize));
        start_0 = (*value).data;
        q_0 = _nc_strrchr(p, start_0, ':' as i32 as uint8_t);
        if q_0.is_null() {
            return b"has an invalid \"hostname:port\" format string\0" as *const u8
                as *const i8;
        }
        port = q_0.offset(1 as i32 as isize);
        portlen = (p.offset_from(port) as i64 + 1 as i32 as i64) as uint32_t;
        p = q_0.offset(-(1 as i32 as isize));
        name = start_0;
        namelen = (p.offset_from(start_0) as i64 + 1 as i32 as i64) as uint32_t;
        (*field).port = _nc_atoi(port, portlen as size_t);
        if (*field).port < 0 as i32 || !nc_valid_port((*field).port) {
            return b"has an invalid port in \"hostname:port\" format string\0"
                as *const u8 as *const i8;
        }
    }
    status = string_copy(&mut (*field).name, name, namelen);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    status = nc_resolve(&mut (*field).name, (*field).port, &mut (*field).info);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    (*field).set_valid(1 as i32 as u32);
    return 0 as *mut libc::c_void as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_add_server(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut status: rstatus_t = 0;
    let mut a: *mut array = 0 as *mut array;
    let mut value: *mut string = 0 as *mut string;
    let mut field: *mut conf_server = 0 as *mut conf_server;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut q: *mut uint8_t = 0 as *mut uint8_t;
    let mut start: *mut uint8_t = 0 as *mut uint8_t;
    let mut pname: *mut uint8_t = 0 as *mut uint8_t;
    let mut addr: *mut uint8_t = 0 as *mut uint8_t;
    let mut port: *mut uint8_t = 0 as *mut uint8_t;
    let mut weight: *mut uint8_t = 0 as *mut uint8_t;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    let mut k: uint32_t = 0;
    let mut delimlen: uint32_t = 0;
    let mut pnamelen: uint32_t = 0;
    let mut addrlen: uint32_t = 0;
    let mut portlen: uint32_t = 0;
    let mut weightlen: uint32_t = 0;
    let mut namelen: uint32_t = 0;
    let delim: *const i8 = b" ::\0" as *const u8 as *const i8;
    p = conf as *mut uint8_t;
    a = p.offset((*cmd).offset as isize) as *mut array;
    field = array_push(a) as *mut conf_server;
    if field.is_null() {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    conf_server_init(field);
    value = array_top(&mut (*cf).arg) as *mut string;
    p = ((*value).data).offset((*value).len as isize).offset(-(1 as i32 as isize));
    start = (*value).data;
    addr = 0 as *mut uint8_t;
    addrlen = 0 as i32 as uint32_t;
    weight = 0 as *mut uint8_t;
    weightlen = 0 as i32 as uint32_t;
    port = 0 as *mut uint8_t;
    portlen = 0 as i32 as uint32_t;
    name = 0 as *mut uint8_t;
    namelen = 0 as i32 as uint32_t;
    delimlen = (if *((*value).data).offset(0 as i32 as isize) as i32 == '/' as i32 {
        2 as i32
    } else {
        3 as i32
    }) as uint32_t;
    k = 0 as i32 as uint32_t;
    while (k as u64) < ::core::mem::size_of::<*const i8>() as u64 {
        q = _nc_strrchr(p, start, *delim.offset(k as isize) as uint8_t);
        if q.is_null() {
            if !(k == 0 as i32 as u32) {
                break;
            }
        } else {
            match k {
                0 => {
                    name = q.offset(1 as i32 as isize);
                    namelen = (p.offset_from(name) as i64 + 1 as i32 as i64) as uint32_t;
                }
                1 => {
                    weight = q.offset(1 as i32 as isize);
                    weightlen = (p.offset_from(weight) as i64 + 1 as i32 as i64)
                        as uint32_t;
                }
                2 => {
                    port = q.offset(1 as i32 as isize);
                    portlen = (p.offset_from(port) as i64 + 1 as i32 as i64) as uint32_t;
                }
                _ => {}
            }
            p = q.offset(-(1 as i32 as isize));
        }
        k = k.wrapping_add(1);
        k;
    }
    if k != delimlen {
        return b"has an invalid \"hostname:port:weight [name]\"or \"/path/unix_socket:weight [name]\" format string\0"
            as *const u8 as *const i8;
    }
    pname = (*value).data;
    pnamelen = if namelen > 0 as i32 as u32 {
        ((*value).len).wrapping_sub(namelen.wrapping_add(1 as i32 as u32))
    } else {
        (*value).len
    };
    status = string_copy(&mut (*field).pname, pname, pnamelen);
    if status != 0 as i32 {
        array_pop(a);
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    addr = start;
    addrlen = (p.offset_from(start) as i64 + 1 as i32 as i64) as uint32_t;
    (*field).weight = _nc_atoi(weight, weightlen as size_t);
    if (*field).weight < 0 as i32 {
        return b"has an invalid weight in \"hostname:port:weight [name]\" format string\0"
            as *const u8 as *const i8
    } else if (*field).weight == 0 as i32 {
        return b"has a zero weight in \"hostname:port:weight [name]\" format string\0"
            as *const u8 as *const i8
    }
    if *((*value).data).offset(0 as i32 as isize) as i32 != '/' as i32 {
        (*field).port = _nc_atoi(port, portlen as size_t);
        if (*field).port < 0 as i32 || !nc_valid_port((*field).port) {
            return b"has an invalid port in \"hostname:port:weight [name]\" format string\0"
                as *const u8 as *const i8;
        }
    }
    if name.is_null() {
        if (*field).port == 11211 as i32 {
            name = addr;
            namelen = addrlen;
        } else {
            name = addr;
            namelen = addrlen.wrapping_add(1 as i32 as u32).wrapping_add(portlen);
        }
    }
    status = string_copy(&mut (*field).name, name, namelen);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    status = string_copy(&mut (*field).addrstr, addr, addrlen);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    (*field).set_valid(1 as i32 as u32);
    return 0 as *mut libc::c_void as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_num(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut num: i32 = 0;
    let mut np: *mut i32 = 0 as *mut i32;
    let mut value: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    np = p.offset((*cmd).offset as isize) as *mut i32;
    if *np != -(1 as i32) {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    num = _nc_atoi((*value).data, (*value).len as size_t);
    if num < 0 as i32 {
        return b"is not a number\0" as *const u8 as *const i8;
    }
    *np = num;
    return 0 as *mut libc::c_void as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_bool(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut bp: *mut i32 = 0 as *mut i32;
    let mut value: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    bp = p.offset((*cmd).offset as isize) as *mut i32;
    if *bp != -(1 as i32) {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    if string_compare(value, &true_str) == 0 as i32 {
        *bp = 1 as i32;
    } else if string_compare(value, &false_str) == 0 as i32 {
        *bp = 0 as i32;
    } else {
        return b"is not \"true\" or \"false\"\0" as *const u8 as *const i8
    }
    return 0 as *mut libc::c_void as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_hash(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut hp: *mut hash_type_t = 0 as *mut hash_type_t;
    let mut value: *const string = 0 as *const string;
    let mut hash: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    hp = p.offset((*cmd).offset as isize) as *mut hash_type_t;
    if *hp as u32 != 4294967295 as hash_type_t as u32 {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    hash = hash_strings.as_ptr();
    while (*hash).len != 0 as i32 as u32 {
        if string_compare(value, hash) != 0 as i32 {
            hash = hash.offset(1);
            hash;
        } else {
            *hp = hash.offset_from(hash_strings.as_ptr()) as i64 as hash_type_t;
            return 0 as *mut libc::c_void as *const i8;
        }
    }
    return b"is not a valid hash\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_distribution(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut dp: *mut dist_type_t = 0 as *mut dist_type_t;
    let mut value: *const string = 0 as *const string;
    let mut dist: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    dp = p.offset((*cmd).offset as isize) as *mut dist_type_t;
    if *dp as u32 != 4294967295 as dist_type_t as u32 {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    dist = dist_strings.as_ptr();
    while (*dist).len != 0 as i32 as u32 {
        if string_compare(value, dist) != 0 as i32 {
            dist = dist.offset(1);
            dist;
        } else {
            *dp = dist.offset_from(dist_strings.as_ptr()) as i64 as dist_type_t;
            return 0 as *mut libc::c_void as *const i8;
        }
    }
    return b"is not a valid distribution\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn conf_set_hashtag(
    mut cf: *mut conf,
    mut cmd: *const command,
    mut conf: *mut libc::c_void,
) -> *const i8 {
    let mut status: rstatus_t = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut field: *mut string = 0 as *mut string;
    let mut value: *const string = 0 as *const string;
    p = conf as *mut uint8_t;
    field = p.offset((*cmd).offset as isize) as *mut string;
    if !((*field).data).is_null() {
        return b"is a duplicate\0" as *const u8 as *const i8;
    }
    value = array_top(&mut (*cf).arg) as *const string;
    if (*value).len != 2 as i32 as u32 {
        return b"is not a valid hash tag string with two characters\0" as *const u8
            as *const i8;
    }
    status = string_duplicate(field, value);
    if status != 0 as i32 {
        return b"has an invalid value\0" as *const u8 as *const i8 as *mut libc::c_void
            as *const i8;
    }
    return 0 as *mut libc::c_void as *const i8;
}
unsafe extern "C" fn run_static_initializers() {
    hash_strings = [
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 14]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"one_at_a_time\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 4]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"md5\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"crc16\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"crc32\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"crc32a\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"fnv1_64\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"fnv1a_64\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"fnv1_32\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"fnv1a_32\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 6]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"hsieh\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"murmur\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 8]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"jenkins\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: 0 as i32 as uint32_t,
                data: 0 as *mut uint8_t,
            };
            init
        },
    ];
    dist_strings = [
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"ketama\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"modula\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: (::core::mem::size_of::<[i8; 7]>() as u64)
                    .wrapping_sub(1 as i32 as u64) as uint32_t,
                data: b"random\0" as *const u8 as *const i8 as *mut uint8_t,
            };
            init
        },
        {
            let mut init = string {
                len: 0 as i32 as uint32_t,
                data: 0 as *mut uint8_t,
            };
            init
        },
    ];
    conf_commands = [
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 7]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"listen\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_listen
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 16 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 5]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"hash\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_hash
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 184 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 9]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"hash_tag\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_hashtag
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 192 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 13]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"distribution\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_distribution
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 208 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"timeout\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 212 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"backlog\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 216 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 19]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"client_connections\0" as *const u8 as *const i8
                            as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 220 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 6]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"redis\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_bool
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 228 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 13]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"tcpkeepalive\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_bool
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 224 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 10]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"reuseport\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_bool
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 308 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"redis_auth\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_string
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 232 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 9]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"redis_db\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 248 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 11]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"preconnect\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_bool
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 252 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 17]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"auto_eject_hosts\0" as *const u8 as *const i8
                            as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_bool
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 256 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 19]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_connections\0" as *const u8 as *const i8
                            as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 260 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 21]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_retry_timeout\0" as *const u8 as *const i8
                            as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 264 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 21]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"server_failure_limit\0" as *const u8 as *const i8
                            as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_set_num
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 268 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: (::core::mem::size_of::<[i8; 8]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as uint32_t,
                        data: b"servers\0" as *const u8 as *const i8 as *mut uint8_t,
                    };
                    init
                },
                set: Some(
                    conf_add_server
                        as unsafe extern "C" fn(
                            *mut conf,
                            *const command,
                            *mut libc::c_void,
                        ) -> *const i8,
                ),
                offset: 272 as u64 as i32,
            };
            init
        },
        {
            let mut init = command {
                name: {
                    let mut init = string {
                        len: 0 as i32 as uint32_t,
                        data: 0 as *mut uint8_t,
                    };
                    init
                },
                set: None,
                offset: 0 as i32,
            };
            init
        },
    ];
    true_str = {
        let mut init = string {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint32_t,
            data: b"true\0" as *const u8 as *const i8 as *mut uint8_t,
        };
        init
    };
    false_str = {
        let mut init = string {
            len: (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint32_t,
            data: b"false\0" as *const u8 as *const i8 as *mut uint8_t,
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];