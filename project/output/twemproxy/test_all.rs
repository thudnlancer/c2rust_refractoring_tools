#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type epoll_event;
    fn ketama_hash(
        key: *const libc::c_char,
        key_length: size_t,
        alignment: uint32_t,
    ) -> uint32_t;
    fn hash_murmur(key: *const libc::c_char, length: size_t) -> uint32_t;
    fn hash_jenkins(key: *const libc::c_char, length: size_t) -> uint32_t;
    fn hash_hsieh(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_fnv1a_32(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_fnv1_32(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_fnv1a_64(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_fnv1_64(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_crc32a(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_crc32(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_crc16(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_md5(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn hash_one_at_a_time(key: *const libc::c_char, key_length: size_t) -> uint32_t;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn log_init(level: libc::c_int, filename: *const libc::c_char) -> libc::c_int;
    fn log_deinit();
    fn mbuf_init(nci: *const instance);
    fn mbuf_deinit();
    fn mbuf_get() -> *mut mbuf;
    fn mbuf_insert(mhdr: *mut mhdr, mbuf: *mut mbuf);
    fn mbuf_copy(mbuf: *mut mbuf, pos: *const uint8_t, n: size_t);
    fn memcache_parse_req(r: *mut msg);
    fn memcache_parse_rsp(r: *mut msg);
    fn redis_parse_req(r: *mut msg);
    fn redis_parse_rsp(r: *mut msg);
    fn msg_put(msg: *mut msg);
    fn msg_get(conn: *mut conn, request: bool, redis: bool) -> *mut msg;
    fn msg_deinit();
    fn msg_init();
    fn conf_destroy(cf: *mut conf);
    fn conf_create(filename: *const libc::c_char) -> *mut conf;
}
pub type rstatus_t = libc::c_int;
pub type err_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub nelem: uint32_t,
    pub elem: *mut libc::c_void,
    pub size: size_t,
    pub nalloc: uint32_t,
}
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string {
    pub len: uint32_t,
    pub data: *mut uint8_t,
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub id: uint32_t,
    pub cf: *mut conf,
    pub stats: *mut stats,
    pub pool: array,
    pub evb: *mut event_base,
    pub max_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub max_nfd: uint32_t,
    pub max_ncconn: uint32_t,
    pub max_nsconn: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_base {
    pub ep: libc::c_int,
    pub event: *mut epoll_event,
    pub nevent: libc::c_int,
    pub cb: event_cb_t,
}
pub type event_cb_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, uint32_t) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub port: uint16_t,
    pub interval: libc::c_int,
    pub addr: string,
    pub start_ts: int64_t,
    pub buf: stats_buffer,
    pub current: array,
    pub shadow: array,
    pub sum: array,
    pub tid: pthread_t,
    pub sd: libc::c_int,
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
    pub aggregate: libc::c_int,
    pub updated: libc::c_int,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_buffer {
    pub len: size_t,
    pub data: *mut uint8_t,
    pub size: size_t,
}
pub type int64_t = __int64_t;
pub type __int64_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conf {
    pub fname: *const libc::c_char,
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
pub type yaml_char_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: libc::c_int,
    pub minor: libc::c_int,
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
    YAML_FOLDED_SCALAR_STYLE,
    YAML_LITERAL_SCALAR_STYLE,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
    YAML_SINGLE_QUOTED_SCALAR_STYLE,
    YAML_PLAIN_SCALAR_STYLE,
    YAML_ANY_SCALAR_STYLE,
}  // end of enum

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
}  // end of enum

pub type yaml_token_type_t = yaml_token_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_token_type_e {
    YAML_SCALAR_TOKEN,
    YAML_TAG_TOKEN,
    YAML_ANCHOR_TOKEN,
    YAML_ALIAS_TOKEN,
    YAML_VALUE_TOKEN,
    YAML_KEY_TOKEN,
    YAML_FLOW_ENTRY_TOKEN,
    YAML_BLOCK_ENTRY_TOKEN,
    YAML_FLOW_MAPPING_END_TOKEN,
    YAML_FLOW_MAPPING_START_TOKEN,
    YAML_FLOW_SEQUENCE_END_TOKEN,
    YAML_FLOW_SEQUENCE_START_TOKEN,
    YAML_BLOCK_END_TOKEN,
    YAML_BLOCK_MAPPING_START_TOKEN,
    YAML_BLOCK_SEQUENCE_START_TOKEN,
    YAML_DOCUMENT_END_TOKEN,
    YAML_DOCUMENT_START_TOKEN,
    YAML_TAG_DIRECTIVE_TOKEN,
    YAML_VERSION_DIRECTIVE_TOKEN,
    YAML_STREAM_END_TOKEN,
    YAML_STREAM_START_TOKEN,
    YAML_NO_TOKEN,
}  // end of enum

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
    pub implicit: libc::c_int,
    pub style: yaml_mapping_style_t,
}
pub type yaml_mapping_style_t = yaml_mapping_style_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_mapping_style_e {
    YAML_FLOW_MAPPING_STYLE,
    YAML_BLOCK_MAPPING_STYLE,
    YAML_ANY_MAPPING_STYLE,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: libc::c_int,
    pub style: yaml_sequence_style_t,
}
pub type yaml_sequence_style_t = yaml_sequence_style_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_sequence_style_e {
    YAML_FLOW_SEQUENCE_STYLE,
    YAML_BLOCK_SEQUENCE_STYLE,
    YAML_ANY_SEQUENCE_STYLE,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: libc::c_int,
    pub quoted_implicit: libc::c_int,
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
    pub implicit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub implicit: libc::c_int,
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
    pub major: libc::c_int,
    pub minor: libc::c_int,
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
    YAML_MAPPING_END_EVENT,
    YAML_MAPPING_START_EVENT,
    YAML_SEQUENCE_END_EVENT,
    YAML_SEQUENCE_START_EVENT,
    YAML_SCALAR_EVENT,
    YAML_ALIAS_EVENT,
    YAML_DOCUMENT_END_EVENT,
    YAML_DOCUMENT_START_EVENT,
    YAML_STREAM_END_EVENT,
    YAML_STREAM_START_EVENT,
    YAML_NO_EVENT,
}  // end of enum

pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_parser_s {
    pub error: yaml_error_type_t,
    pub problem: *const libc::c_char,
    pub problem_offset: size_t,
    pub problem_value: libc::c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const libc::c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option::<yaml_read_handler_t>,
    pub read_handler_data: *mut libc::c_void,
    pub input: C2RustUnnamed_33,
    pub eof: libc::c_int,
    pub buffer: C2RustUnnamed_32,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: libc::c_int,
    pub stream_end_produced: libc::c_int,
    pub flow_level: libc::c_int,
    pub tokens: C2RustUnnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: libc::c_int,
    pub indents: C2RustUnnamed_29,
    pub indent: libc::c_int,
    pub simple_key_allowed: libc::c_int,
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
    pub start_implicit: libc::c_int,
    pub end_implicit: libc::c_int,
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
    pub key: libc::c_int,
    pub value: libc::c_int,
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
pub type yaml_node_item_t = libc::c_int;
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
    YAML_MAPPING_NODE,
    YAML_SEQUENCE_NODE,
    YAML_SCALAR_NODE,
    YAML_NO_NODE,
}  // end of enum

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
    pub index: libc::c_int,
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
    YAML_PARSE_END_STATE,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE,
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE,
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE,
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE,
    YAML_PARSE_FLOW_NODE_STATE,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE,
    YAML_PARSE_BLOCK_NODE_STATE,
    YAML_PARSE_DOCUMENT_END_STATE,
    YAML_PARSE_DOCUMENT_CONTENT_STATE,
    YAML_PARSE_DOCUMENT_START_STATE,
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE,
    YAML_PARSE_STREAM_START_STATE,
}  // end of enum

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
    pub possible: libc::c_int,
    pub required: libc::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub start: *mut libc::c_int,
    pub end: *mut libc::c_int,
    pub top: *mut libc::c_int,
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
    pub start: *mut libc::c_uchar,
    pub end: *mut libc::c_uchar,
    pub pointer: *mut libc::c_uchar,
    pub last: *mut libc::c_uchar,
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
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub start: *const libc::c_uchar,
    pub end: *const libc::c_uchar,
    pub current: *const libc::c_uchar,
}
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_uchar,
    size_t,
    *mut size_t,
) -> libc::c_int;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum yaml_error_type_e {
    YAML_EMITTER_ERROR,
    YAML_WRITER_ERROR,
    YAML_COMPOSER_ERROR,
    YAML_PARSER_ERROR,
    YAML_SCANNER_ERROR,
    YAML_READER_ERROR,
    YAML_MEMORY_ERROR,
    YAML_NO_ERROR,
}  // end of enum

#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct conn {
    pub conn_tqe: C2RustUnnamed_41,
    pub owner: *mut libc::c_void,
    pub sd: libc::c_int,
    pub family: libc::c_int,
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
pub type conn_msgq_t = Option::<
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
    pub state: libc::c_int,
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
pub type __uint64_t = libc::c_ulong;
pub type msg_type_t = msg_type;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum msg_type {
    MSG_SENTINEL,
    MSG_RSP_REDIS_MULTIBULK,
    MSG_RSP_REDIS_BULK,
    MSG_RSP_REDIS_INTEGER,
    MSG_RSP_REDIS_ERROR_NOREPLICAS,
    MSG_RSP_REDIS_ERROR_MASTERDOWN,
    MSG_RSP_REDIS_ERROR_EXECABORT,
    MSG_RSP_REDIS_ERROR_WRONGTYPE,
    MSG_RSP_REDIS_ERROR_READONLY,
    MSG_RSP_REDIS_ERROR_NOSCRIPT,
    MSG_RSP_REDIS_ERROR_MISCONF,
    MSG_RSP_REDIS_ERROR_BUSYKEY,
    MSG_RSP_REDIS_ERROR_LOADING,
    MSG_RSP_REDIS_ERROR_NOAUTH,
    MSG_RSP_REDIS_ERROR_BUSY,
    MSG_RSP_REDIS_ERROR_OOM,
    MSG_RSP_REDIS_ERROR_ERR,
    MSG_RSP_REDIS_ERROR,
    MSG_RSP_REDIS_STATUS,
    MSG_REQ_REDIS_LOLWUT,
    MSG_REQ_REDIS_COMMAND,
    MSG_REQ_REDIS_SELECT,
    MSG_REQ_REDIS_AUTH,
    MSG_REQ_REDIS_QUIT,
    MSG_REQ_REDIS_PING,
    MSG_REQ_REDIS_EVALSHA,
    MSG_REQ_REDIS_EVAL,
    MSG_REQ_REDIS_GEOSEARCHSTORE,
    MSG_REQ_REDIS_GEOSEARCH,
    MSG_REQ_REDIS_GEOPOS,
    MSG_REQ_REDIS_GEORADIUSBYMEMBER,
    MSG_REQ_REDIS_GEORADIUS,
    MSG_REQ_REDIS_GEOHASH,
    MSG_REQ_REDIS_GEODIST,
    MSG_REQ_REDIS_GEOADD,
    MSG_REQ_REDIS_ZUNIONSTORE,
    MSG_REQ_REDIS_ZSCORE,
    MSG_REQ_REDIS_ZSCAN,
    MSG_REQ_REDIS_ZUNION,
    MSG_REQ_REDIS_ZREVRANK,
    MSG_REQ_REDIS_ZREVRANGEBYSCORE,
    MSG_REQ_REDIS_ZREVRANGEBYLEX,
    MSG_REQ_REDIS_ZREVRANGE,
    MSG_REQ_REDIS_ZREMRANGEBYSCORE,
    MSG_REQ_REDIS_ZREMRANGEBYLEX,
    MSG_REQ_REDIS_ZREMRANGEBYRANK,
    MSG_REQ_REDIS_ZREM,
    MSG_REQ_REDIS_ZRANK,
    MSG_REQ_REDIS_ZRANGESTORE,
    MSG_REQ_REDIS_ZRANGEBYSCORE,
    MSG_REQ_REDIS_ZRANGEBYLEX,
    MSG_REQ_REDIS_ZRANGE,
    MSG_REQ_REDIS_ZRANDMEMBER,
    MSG_REQ_REDIS_ZPOPMAX,
    MSG_REQ_REDIS_ZPOPMIN,
    MSG_REQ_REDIS_ZMSCORE,
    MSG_REQ_REDIS_ZLEXCOUNT,
    MSG_REQ_REDIS_ZINTERSTORE,
    MSG_REQ_REDIS_ZINTER,
    MSG_REQ_REDIS_ZINCRBY,
    MSG_REQ_REDIS_ZDIFFSTORE,
    MSG_REQ_REDIS_ZDIFF,
    MSG_REQ_REDIS_ZCOUNT,
    MSG_REQ_REDIS_ZCARD,
    MSG_REQ_REDIS_ZADD,
    MSG_REQ_REDIS_SSCAN,
    MSG_REQ_REDIS_SUNIONSTORE,
    MSG_REQ_REDIS_SUNION,
    MSG_REQ_REDIS_SREM,
    MSG_REQ_REDIS_SRANDMEMBER,
    MSG_REQ_REDIS_SPOP,
    MSG_REQ_REDIS_SMOVE,
    MSG_REQ_REDIS_SMEMBERS,
    MSG_REQ_REDIS_SMISMEMBER,
    MSG_REQ_REDIS_SISMEMBER,
    MSG_REQ_REDIS_SINTERSTORE,
    MSG_REQ_REDIS_SINTER,
    MSG_REQ_REDIS_SDIFFSTORE,
    MSG_REQ_REDIS_SDIFF,
    MSG_REQ_REDIS_SCARD,
    MSG_REQ_REDIS_SADD,
    MSG_REQ_REDIS_RPUSHX,
    MSG_REQ_REDIS_RPUSH,
    MSG_REQ_REDIS_RPOPLPUSH,
    MSG_REQ_REDIS_RPOP,
    MSG_REQ_REDIS_PFMERGE,
    MSG_REQ_REDIS_PFCOUNT,
    MSG_REQ_REDIS_PFADD,
    MSG_REQ_REDIS_LTRIM,
    MSG_REQ_REDIS_LSET,
    MSG_REQ_REDIS_LREM,
    MSG_REQ_REDIS_LRANGE,
    MSG_REQ_REDIS_LPUSHX,
    MSG_REQ_REDIS_LPUSH,
    MSG_REQ_REDIS_LPOS,
    MSG_REQ_REDIS_LPOP,
    MSG_REQ_REDIS_LMOVE,
    MSG_REQ_REDIS_LLEN,
    MSG_REQ_REDIS_LINSERT,
    MSG_REQ_REDIS_LINDEX,
    MSG_REQ_REDIS_HVALS,
    MSG_REQ_REDIS_HSTRLEN,
    MSG_REQ_REDIS_HSCAN,
    MSG_REQ_REDIS_HSETNX,
    MSG_REQ_REDIS_HSET,
    MSG_REQ_REDIS_HRANDFIELD,
    MSG_REQ_REDIS_HMSET,
    MSG_REQ_REDIS_HMGET,
    MSG_REQ_REDIS_HLEN,
    MSG_REQ_REDIS_HKEYS,
    MSG_REQ_REDIS_HINCRBYFLOAT,
    MSG_REQ_REDIS_HINCRBY,
    MSG_REQ_REDIS_HGETALL,
    MSG_REQ_REDIS_HGET,
    MSG_REQ_REDIS_HEXISTS,
    MSG_REQ_REDIS_HDEL,
    MSG_REQ_REDIS_STRLEN,
    MSG_REQ_REDIS_SETRANGE,
    MSG_REQ_REDIS_SETNX,
    MSG_REQ_REDIS_SETEX,
    MSG_REQ_REDIS_SETBIT,
    MSG_REQ_REDIS_SET,
    MSG_REQ_REDIS_RESTORE,
    MSG_REQ_REDIS_PSETEX,
    MSG_REQ_REDIS_MSET,
    MSG_REQ_REDIS_MGET,
    MSG_REQ_REDIS_INCRBYFLOAT,
    MSG_REQ_REDIS_INCRBY,
    MSG_REQ_REDIS_INCR,
    MSG_REQ_REDIS_GETSET,
    MSG_REQ_REDIS_GETRANGE,
    MSG_REQ_REDIS_GETEX,
    MSG_REQ_REDIS_GETDEL,
    MSG_REQ_REDIS_GETBIT,
    MSG_REQ_REDIS_GET,
    MSG_REQ_REDIS_DUMP,
    MSG_REQ_REDIS_DECRBY,
    MSG_REQ_REDIS_DECR,
    MSG_REQ_REDIS_BITPOS,
    MSG_REQ_REDIS_BITFIELD,
    MSG_REQ_REDIS_BITCOUNT,
    MSG_REQ_REDIS_APPEND,
    MSG_REQ_REDIS_UNLINK,
    MSG_REQ_REDIS_TYPE,
    MSG_REQ_REDIS_TTL,
    MSG_REQ_REDIS_TOUCH,
    MSG_REQ_REDIS_SORT,
    MSG_REQ_REDIS_PTTL,
    MSG_REQ_REDIS_PERSIST,
    MSG_REQ_REDIS_PEXPIREAT,
    MSG_REQ_REDIS_PEXPIRE,
    MSG_REQ_REDIS_MOVE,
    MSG_REQ_REDIS_EXPIREAT,
    MSG_REQ_REDIS_EXPIRE,
    MSG_REQ_REDIS_EXISTS,
    MSG_REQ_REDIS_DEL,
    MSG_REQ_REDIS_COPY,
    MSG_RSP_MC_SERVER_ERROR,
    MSG_RSP_MC_CLIENT_ERROR,
    MSG_RSP_MC_ERROR,
    MSG_RSP_MC_VERSION,
    MSG_RSP_MC_TOUCHED,
    MSG_RSP_MC_DELETED,
    MSG_RSP_MC_VALUE,
    MSG_RSP_MC_END,
    MSG_RSP_MC_NOT_FOUND,
    MSG_RSP_MC_EXISTS,
    MSG_RSP_MC_NOT_STORED,
    MSG_RSP_MC_STORED,
    MSG_RSP_MC_NUM,
    MSG_REQ_MC_VERSION,
    MSG_REQ_MC_QUIT,
    MSG_REQ_MC_TOUCH,
    MSG_REQ_MC_DECR,
    MSG_REQ_MC_INCR,
    MSG_REQ_MC_PREPEND,
    MSG_REQ_MC_APPEND,
    MSG_REQ_MC_REPLACE,
    MSG_REQ_MC_ADD,
    MSG_REQ_MC_SET,
    MSG_REQ_MC_CAS,
    MSG_REQ_MC_DELETE,
    MSG_REQ_MC_GETS,
    MSG_REQ_MC_GET,
    MSG_UNKNOWN,
}  // end of enum

pub type msg_coalesce_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
pub type msg_failure_t = Option::<unsafe extern "C" fn(*const msg) -> bool>;
pub type msg_add_auth_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut conn) -> rstatus_t,
>;
pub type msg_reply_t = Option::<unsafe extern "C" fn(*mut msg) -> rstatus_t>;
pub type msg_fragment_t = Option::<
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
}  // end of enum

pub type msg_parse_t = Option::<unsafe extern "C" fn(*mut msg) -> ()>;
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
pub type conn_unref_t = Option::<unsafe extern "C" fn(*mut conn) -> ()>;
pub type conn_ref_t = Option::<unsafe extern "C" fn(*mut conn, *mut libc::c_void) -> ()>;
pub type conn_swallow_msg_t = Option::<
    unsafe extern "C" fn(*mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_post_connect_t = Option::<
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
    pub family: libc::c_int,
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
    pub sun_path: [libc::c_char; 108],
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
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type socklen_t = __socklen_t;
pub type __socklen_t = libc::c_uint;
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
    pub dist_type: libc::c_int,
    pub key_hash_type: libc::c_int,
    pub key_hash: hash_t,
    pub hash_tag: string,
    pub timeout: libc::c_int,
    pub backlog: libc::c_int,
    pub redis_db: libc::c_int,
    pub client_connections: uint32_t,
    pub server_connections: uint32_t,
    pub server_retry_timeout: int64_t,
    pub server_failure_limit: uint32_t,
    pub redis_auth: string,
    pub require_auth: libc::c_uint,
    #[bitfield(name = "auto_eject_hosts", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "preconnect", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "redis", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "tcpkeepalive", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "reuseport", ty = "libc::c_uint", bits = "4..=4")]
    pub auto_eject_hosts_preconnect_redis_tcpkeepalive_reuseport: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type hash_t = Option::<
    unsafe extern "C" fn(*const libc::c_char, size_t) -> uint32_t,
>;
pub type mode_t = __mode_t;
pub type __mode_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct continuum {
    pub index: uint32_t,
    pub value: uint32_t,
}
pub type conn_active_t = Option::<unsafe extern "C" fn(*const conn) -> bool>;
pub type conn_close_t = Option::<unsafe extern "C" fn(*mut context, *mut conn) -> ()>;
pub type conn_send_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg) -> (),
>;
pub type conn_send_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> *mut msg,
>;
pub type conn_send_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
pub type conn_recv_done_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, *mut msg, *mut msg) -> (),
>;
pub type conn_recv_next_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn, bool) -> *mut msg,
>;
pub type conn_recv_t = Option::<
    unsafe extern "C" fn(*mut context, *mut conn) -> rstatus_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub tqe_next: *mut conn,
    pub tqe_prev: *mut *mut conn,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct instance {
    pub ctx: *mut context,
    pub log_level: libc::c_int,
    pub log_filename: *const libc::c_char,
    pub conf_filename: *const libc::c_char,
    pub stats_port: uint16_t,
    pub stats_interval: libc::c_int,
    pub stats_addr: *const libc::c_char,
    pub hostname: [libc::c_char; 256],
    pub mbuf_chunk_size: size_t,
    pub pid: pid_t,
    pub pid_filename: *const libc::c_char,
    #[bitfield(name = "pidfile", ty = "libc::c_uint", bits = "0..=0")]
    pub pidfile: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type pid_t = __pid_t;
pub type __pid_t = libc::c_int;
static mut failures: libc::c_int = 0 as libc::c_int;
static mut successes: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn expect_same_int(
    mut expected: libc::c_int,
    mut actual: libc::c_int,
    mut message: *const libc::c_char,
) {
    if expected != actual {
        printf(
            b"FAIL Expected %d, got %d (%s)\n\0" as *const u8 as *const libc::c_char,
            expected,
            actual,
            message,
        );
        failures += 1;
        failures;
    } else {
        successes += 1;
        successes;
    };
}
unsafe extern "C" fn expect_same_uint32_t(
    mut expected: uint32_t,
    mut actual: uint32_t,
    mut message: *const libc::c_char,
) {
    if expected != actual {
        printf(
            b"FAIL Expected %u, got %u (%s)\n\0" as *const u8 as *const libc::c_char,
            expected,
            actual,
            message,
        );
        failures += 1;
        failures;
    } else {
        successes += 1;
        successes;
    };
}
unsafe extern "C" fn expect_same_ptr(
    mut expected: *const libc::c_void,
    mut actual: *const libc::c_void,
    mut message: *const libc::c_char,
) {
    if expected != actual {
        printf(
            b"FAIL Expected %p, got %p (%s)\n\0" as *const u8 as *const libc::c_char,
            expected,
            actual,
            message,
        );
        failures += 1;
        failures;
    } else {
        successes += 1;
        successes;
    };
}
unsafe extern "C" fn test_hash_algorithms() {
    expect_same_uint32_t(
        2297466611 as libc::c_uint,
        hash_one_at_a_time(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected one_at_a_time hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        3195025439 as libc::c_uint,
        hash_md5(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected md5 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        3662830516 as libc::c_uint,
        hash_crc16(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected crc16 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        10542 as libc::c_uint,
        hash_crc32(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected crc32 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        2838417488 as libc::c_uint,
        hash_crc32a(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected crc32a hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        67176023 as libc::c_uint,
        hash_fnv1_32(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected fnv1_32 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        280767167 as libc::c_uint,
        hash_fnv1a_32(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected fnv1a_32 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        473199127 as libc::c_uint,
        hash_fnv1_64(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected fnv1_64 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        1488911807 as libc::c_uint,
        hash_fnv1a_64(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected fnv1a_64 hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        3738850110 as libc::c_uint,
        hash_hsieh(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected hsieh hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        1442444624 as libc::c_uint,
        hash_jenkins(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected jenkins hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        4142305122 as libc::c_uint,
        hash_murmur(
            b"apple\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ),
        b"should have expected murmur hash for key \"apple\"\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        3853726576 as libc::c_uint,
        ketama_hash(
            b"server1-8\0" as *const u8 as *const libc::c_char,
            strlen(b"server1-8\0" as *const u8 as *const libc::c_char),
            0 as libc::c_int as uint32_t,
        ),
        b"should have expected ketama_hash for server1-8 index 0\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_uint32_t(
        2667054752 as libc::c_uint,
        ketama_hash(
            b"server1-8\0" as *const u8 as *const libc::c_char,
            strlen(b"server1-8\0" as *const u8 as *const libc::c_char),
            3 as libc::c_int as uint32_t,
        ),
        b"should have expected ketama_hash for server1-8 index 3\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn test_config_parsing() {
    let mut conf_file: *const libc::c_char = b"../conf/nutcracker.yml\0" as *const u8
        as *const libc::c_char;
    let mut conf: *mut conf = conf_create(conf_file);
    if conf.is_null() {
        printf(
            b"FAIL could not parse %s (this test should be run within src/ folder)\n\0"
                as *const u8 as *const libc::c_char,
            conf_file,
        );
        failures += 1;
        failures;
    } else {
        printf(b"PASS parsed %s\n\0" as *const u8 as *const libc::c_char, conf_file);
        conf_destroy(conf);
        successes += 1;
        successes;
    };
}
unsafe extern "C" fn test_redis_parse_req_success_case(
    mut data: *const libc::c_char,
    mut expected_type: libc::c_int,
) {
    let original_failures: libc::c_int = failures;
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let mut req: *mut msg = msg_get(
        &mut fake_client,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    (*req).state = SW_START;
    (*req).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*req).mhdr.stqh_first = 0 as *mut mbuf;
    (*req).mhdr.stqh_last = &mut (*req).mhdr.stqh_first;
    mbuf_insert(&mut (*req).mhdr, m);
    (*req).pos = (*m).start;
    redis_parse_req(req);
    expect_same_ptr(
        (*m).last as *const libc::c_void,
        (*req).pos as *const libc::c_void,
        b"redis_parse_req: expected req->pos to be m->last\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        SW_START,
        (*req).state,
        b"redis_parse_req: expected full buffer to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        expected_type,
        (*req).type_0 as libc::c_int,
        b"redis_parse_req: expected request type to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        fake_client.err,
        b"redis_parse_req: expected no connection error\0" as *const u8
            as *const libc::c_char,
    );
    msg_put(req);
    if failures > original_failures {
        fprintf(
            stderr,
            b"test_redis_parse_req_success_case failed for (%s)\0" as *const u8
                as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_redis_parse_req_success() {
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\neval\r\n$10\r\nreturn 123\r\n$1\r\n1\r\n$1\r\n1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_EVAL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*7\r\n$4\r\neval\r\n$40\r\nreturn {KEYS[1],KEYS[2],ARGV[1],ARGV[2]}\r\n$1\r\n2\r\n$9\r\nkey1{tag}\r\n$4\r\narg1\r\n$9\r\nkey2{tag}\r\n$4\r\narg2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_EVAL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nappend\r\n$3\r\n999\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_APPEND as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$8\r\nbitcount\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_BITCOUNT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$8\r\nbitcount\r\n$3\r\nfoo\r\n$1\r\n1\r\n$1\r\n1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_BITCOUNT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*1\r\n$7\r\nCOMMAND\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_COMMAND as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\ndecr\r\n$7\r\ncounter\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_DECR as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\ndecrby\r\n$7\r\ncounter\r\n$3\r\n100\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_DECRBY as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$3\r\ndel\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_DEL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$3\r\ndel\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_DEL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\ndump\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_DUMP as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$6\r\nexists\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_EXISTS as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nexpire\r\n$3\r\nfoo\r\n$1\r\n0\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_EXPIRE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$8\r\nexpireat\r\n$3\r\nfoo\r\n$10\r\n1282463464\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_EXPIREAT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$3\r\nGET\r\n$3\r\nkey\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_GET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\ngetbit\r\n$3\r\nfoo\r\n$1\r\n1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_GETBIT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$8\r\ngetrange\r\n$3\r\nfoo\r\n$1\r\n1\r\n$1\r\n2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_GETRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\ngetset\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_GETSET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nhdel\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_HDEL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$7\r\nhexists\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_HEXISTS as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nhget\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_HGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$7\r\nhgetall\r\n$4\r\nhfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HGETALL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$7\r\nhincrby\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$3\r\n100\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HINCRBY as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$12\r\nhincrbyfloat\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$6\r\n100.12\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HINCRBYFLOAT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$5\r\nhkeys\r\n$4\r\nhfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HKEYS as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nhlen\r\n$4\r\nhfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HLEN as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nhmget\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_HMGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nhmget\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$6\r\n1dleif\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HMGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*6\r\n$5\r\nhmset\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$3\r\nbar\r\n$6\r\nfield2\r\n$3\r\nbas\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HMSET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nhset\r\n$4\r\nhfoo\r\n$6\r\n1dleif\r\n$3\r\nrab\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_HSET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nhsetnx\r\n$4\r\nhfoo\r\n$6\r\nfield1\r\n$3\r\nbar\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HSETNX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$5\r\nhvals\r\n$4\r\nhfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_HVALS as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nincr\r\n$7\r\ncounter\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_INCR as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nincrby\r\n$7\r\ncounter\r\n$3\r\n100\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_INCRBY as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$11\r\nincrbyfloat\r\n$7\r\ncounter\r\n$5\r\n10.10\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_INCRBYFLOAT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nlindex\r\n$4\r\nlfoo\r\n$1\r\n0\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LINDEX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$7\r\nlinsert\r\n$4\r\nlfoo\r\n$6\r\nBEFORE\r\n$3\r\nbar\r\n$3\r\nbaq\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LINSERT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nllen\r\n$4\r\nlfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LLEN as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nLLEN\r\n$6\r\nmylist\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LLEN as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*1\r\n$6\r\nLOLWUT\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LOLWUT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$6\r\nLOLWUT\r\n$2\r\n40\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LOLWUT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nlpop\r\n$4\r\nlfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LPOP as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nlpush\r\n$4\r\nlfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nlpush\r\n$4\r\nlfoo\r\n$3\r\nbaq\r\n$3\r\nbap\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*6\r\n$5\r\nlpush\r\n$4\r\nlfoo\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbau\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_LPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nlpushx\r\n$4\r\nlfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LPUSHX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nlrange\r\n$4\r\nlfoo\r\n$1\r\n0\r\n$1\r\n2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nlrange\r\n$4\r\nlfoo\r\n$1\r\n0\r\n$1\r\n3\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nlrem\r\n$4\r\nlfoo\r\n$1\r\n2\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LREM as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nlset\r\n$4\r\nlfoo\r\n$1\r\n0\r\n$3\r\nbaq\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LSET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nltrim\r\n$4\r\nlfoo\r\n$1\r\n0\r\n$1\r\n2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_LTRIM as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*13\r\n$4\r\nmget\r\n$3\r\nfoo\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n$3\r\nbar\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_MGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nMGET\r\n$1\r\nx\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_MGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nMGET\r\n$1\r\nx\r\n$10\r\nabcdefghij\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_MGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nmget\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_MGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nmget\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_MGET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$7\r\npersist\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_PERSIST as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$7\r\npexpire\r\n$3\r\nfoo\r\n$1\r\n0\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_PEXPIRE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\npfadd\r\n$7\r\n{pfoo}2\r\n$3\r\nbas\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_PFADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\npfadd\r\n$4\r\npfoo\r\n$3\r\nbar\r\n$3\r\nbas\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_PFADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$7\r\npfcount\r\n$4\r\npfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_PFCOUNT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$7\r\npfmerge\r\n$7\r\n{pfoo}3\r\n$1\r\n2\r\n$6\r\n{pfoo}\r\n$7\r\n{pfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_PFMERGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*1\r\n$4\r\nPING\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_PING as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\npsetex\r\n$3\r\nfoo\r\n$4\r\n1000\r\n$3\r\noof\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_PSETEX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\npttl\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_PTTL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$7\r\nrestore\r\n$3\r\nfoo\r\n$1\r\n0\r\n$3\r\noof\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_RESTORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\nrpop\r\n$4\r\nlfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_RPOP as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$9\r\nrpoplpush\r\n$6\r\n{lfoo}\r\n$7\r\n{lfoo}2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_RPOPLPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nrpush\r\n$4\r\nlfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_RPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nrpush\r\n$4\r\nlfoo\r\n$3\r\nbat\r\n$3\r\nbau\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_RPUSH as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nrpushx\r\n$4\r\nlfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_RPUSHX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nsadd\r\n$7\r\n{sfoo}2\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nsadd\r\n$4\r\nsfoo\r\n$3\r\nbar\r\n$3\r\nbas\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$4\r\nsadd\r\n$4\r\nsfoo\r\n$3\r\nbar\r\n$3\r\nbas\r\n$3\r\nbat\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$5\r\nscard\r\n$4\r\nsfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SCARD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nsdiff\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SDIFF as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$10\r\nsdiffstore\r\n$7\r\n{sfoo}3\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SDIFFSTORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$3\r\nSET\r\n$10\r\nkey4567890\r\n$5\r\nVALUE\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$3\r\nset\r\n$3\r\nbar\r\n$3\r\nrab\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SET as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nsetbit\r\n$3\r\nfoo\r\n$1\r\n1\r\n$1\r\n1\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SETBIT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nsetex\r\n$3\r\nfoo\r\n$4\r\n1000\r\n$3\r\noof\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SETEX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nsetnx\r\n$3\r\nfoo\r\n$3\r\noof\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SETNX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$8\r\nsetrange\r\n$3\r\nfoo\r\n$1\r\n1\r\n$3\r\noof\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SETRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nsinter\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SINTER as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$11\r\nsinterstore\r\n$7\r\n{sfoo}3\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SINTERSTORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$9\r\nsismember\r\n$4\r\nsfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SISMEMBER as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$8\r\nsmembers\r\n$4\r\nsfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SMEMBERS as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$5\r\nsmove\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n$3\r\nbas\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SMOVE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$11\r\nsrandmember\r\n$4\r\nsfoo\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SRANDMEMBER as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$11\r\nsrandmember\r\n$4\r\nsfoo\r\n$1\r\n2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SRANDMEMBER as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$4\r\nsrem\r\n$4\r\nsfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SREM as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$4\r\nsrem\r\n$4\r\nsfoo\r\n$3\r\nbas\r\n$3\r\nbat\r\n$3\r\nrab\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SREM as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nsunion\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_SUNION as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$11\r\nsunionstore\r\n$7\r\n{sfoo}3\r\n$6\r\n{sfoo}\r\n$7\r\n{sfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_SUNIONSTORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$3\r\nttl\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_TTL as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$4\r\ntype\r\n$3\r\nfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_TYPE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$4\r\nzadd\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*6\r\n$4\r\nzadd\r\n$4\r\nzfoo\r\n$3\r\n101\r\n$3\r\nbat\r\n$3\r\n102\r\n$3\r\nbau\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*8\r\n$4\r\nzadd\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\nbar\r\n$3\r\n101\r\n$3\r\nbat\r\n$3\r\n102\r\n$3\r\nbau\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZADD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*2\r\n$5\r\nzcard\r\n$4\r\nzfoo\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZCARD as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nzcount\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\n101\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZCOUNT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$7\r\nzincrby\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZINCRBY as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$11\r\nzinterstore\r\n$7\r\n{zfoo}3\r\n$1\r\n2\r\n$6\r\n{zfoo}\r\n$7\r\n{zfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZINTERSTORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$9\r\nzlexcount\r\n$4\r\nzfoo\r\n$1\r\n-\r\n$1\r\n+\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZLEXCOUNT as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$6\r\nzrange\r\n$4\r\nzfoo\r\n$1\r\n0\r\n$1\r\n3\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$6\r\nzrange\r\n$4\r\nzfoo\r\n$1\r\n0\r\n$1\r\n3\r\n$10\r\nWITHSCORES\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$11\r\nzrangebylex\r\n$4\r\nzfoo\r\n$1\r\n-\r\n$1\r\n+\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZRANGEBYLEX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$13\r\nzrangebyscore\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\n101\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZRANGEBYSCORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$5\r\nzrank\r\n$4\r\nzfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZRANK as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*8\r\n$4\r\nzrem\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\nbar\r\n$3\r\n101\r\n$3\r\nbat\r\n$3\r\n102\r\n$3\r\nbau\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZREM as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$14\r\nzremrangebylex\r\n$4\r\nzfoo\r\n$1\r\n-\r\n$1\r\n+\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZREMRANGEBYLEX as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$15\r\nzremrangebyrank\r\n$4\r\nzfoo\r\n$1\r\n0\r\n$1\r\n1\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZREMRANGEBYRANK as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$16\r\nzremrangebyscore\r\n$4\r\nzfoo\r\n$3\r\n100\r\n$3\r\n101\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZREMRANGEBYSCORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*4\r\n$9\r\nzrevrange\r\n$4\r\nzfoo\r\n$1\r\n0\r\n$1\r\n2\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZREVRANGE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$8\r\nzrevrank\r\n$4\r\nzfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZREVRANK as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*3\r\n$6\r\nzscore\r\n$4\r\nzfoo\r\n$3\r\nbar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_REDIS_ZSCORE as libc::c_int,
    );
    test_redis_parse_req_success_case(
        b"*5\r\n$11\r\nzunionstore\r\n$7\r\n{zfoo}3\r\n$1\r\n2\r\n$6\r\n{zfoo}\r\n$7\r\n{zfoo}2\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_REDIS_ZUNIONSTORE as libc::c_int,
    );
}
unsafe extern "C" fn test_redis_parse_rsp_success_case(
    mut data: *const libc::c_char,
    mut expected: libc::c_int,
) {
    let mut original_failures: libc::c_int = failures;
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let mut rsp: *mut msg = msg_get(
        &mut fake_client,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    (*rsp).state = SW_START;
    (*rsp).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*rsp).mhdr.stqh_first = 0 as *mut mbuf;
    (*rsp).mhdr.stqh_last = &mut (*rsp).mhdr.stqh_first;
    mbuf_insert(&mut (*rsp).mhdr, m);
    (*rsp).pos = (*m).start;
    *__errno_location() = 0 as libc::c_int;
    redis_parse_rsp(rsp);
    expect_same_ptr(
        (*m).last as *const libc::c_void,
        (*rsp).pos as *const libc::c_void,
        b"redis_parse_rsp: expected rsp->pos to be m->last\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        SW_START,
        (*rsp).state,
        b"redis_parse_rsp: expected full buffer to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        expected,
        (*rsp).type_0 as libc::c_int,
        b"redis_parse_rsp: expected response type to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        *__errno_location(),
        b"redis_parse_rsp: expected errno=0\0" as *const u8 as *const libc::c_char,
    );
    expect_same_uint32_t(
        1 as libc::c_int as uint32_t,
        if (*rsp).rnarg != 0 { (*rsp).rnarg } else { 1 as libc::c_int as libc::c_uint },
        b"expected remaining args to be 0 or 1\0" as *const u8 as *const libc::c_char,
    );
    msg_put(rsp);
    if failures > original_failures {
        fprintf(
            stderr,
            b"test_redis_parse_rsp_success_case failed for (%s)\0" as *const u8
                as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_redis_parse_rsp_success() {
    test_redis_parse_rsp_success_case(
        b"-CUSTOMERR\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_ERROR as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"-Error message\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_ERROR as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"+OK\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_STATUS as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"$6\r\nfoobar\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_BULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"$0\r\n\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_BULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"$-1\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_BULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"*0\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_MULTIBULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_MULTIBULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"*3\r\n:1\r\n:2\r\n:3\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_MULTIBULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"*-1\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_REDIS_MULTIBULK as libc::c_int,
    );
    test_redis_parse_rsp_success_case(
        b"*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_RSP_REDIS_MULTIBULK as libc::c_int,
    );
}
unsafe extern "C" fn test_redis_parse_rsp_failure_case(mut data: *const libc::c_char) {
    let mut original_failures: libc::c_int = failures;
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let mut rsp: *mut msg = msg_get(
        &mut fake_client,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    );
    (*rsp).state = SW_START;
    (*rsp).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*rsp).mhdr.stqh_first = 0 as *mut mbuf;
    (*rsp).mhdr.stqh_last = &mut (*rsp).mhdr.stqh_first;
    mbuf_insert(&mut (*rsp).mhdr, m);
    (*rsp).pos = (*m).start;
    *__errno_location() = 0 as libc::c_int;
    redis_parse_rsp(rsp);
    expect_same_ptr(
        (*m).start as *const libc::c_void,
        (*rsp).pos as *const libc::c_void,
        b"redis_parse_rsp: expected rsp->pos to be m->start\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        MSG_PARSE_ERROR as libc::c_int,
        (*rsp).result as libc::c_int,
        b"redis_parse_rsp: expected MSG_PARSE_ERROR\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        22 as libc::c_int,
        *__errno_location(),
        b"redis_parse_rsp: expected errno=EINVAL\0" as *const u8 as *const libc::c_char,
    );
    msg_put(rsp);
    if failures > original_failures {
        fprintf(
            stderr,
            b"test_redis_parse_rsp_failure_case failed for (%s)\0" as *const u8
                as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_redis_parse_rsp_failure() {
    test_redis_parse_rsp_failure_case(b"*\r\n\0" as *const u8 as *const libc::c_char);
    test_redis_parse_rsp_failure_case(b":x\r\n\0" as *const u8 as *const libc::c_char);
    test_redis_parse_rsp_failure_case(
        b"$6\r\nfoobarr\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_redis_parse_rsp_failure_case(
        b"$6\r\nfoobar\n\n\0" as *const u8 as *const libc::c_char,
    );
    test_redis_parse_rsp_failure_case(
        b"$0\r\nx\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_redis_parse_rsp_failure_case(b"$0\n\0" as *const u8 as *const libc::c_char);
    test_redis_parse_rsp_failure_case(
        b"*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n\r\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn test_memcache_parse_rsp_success_case(
    mut data: *const libc::c_char,
    mut expected: libc::c_int,
) {
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let original_failures: libc::c_int = failures;
    let mut rsp: *mut msg = msg_get(
        &mut fake_client,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    (*rsp).state = SW_START;
    (*rsp).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*rsp).mhdr.stqh_first = 0 as *mut mbuf;
    (*rsp).mhdr.stqh_last = &mut (*rsp).mhdr.stqh_first;
    mbuf_insert(&mut (*rsp).mhdr, m);
    (*rsp).pos = (*m).start;
    *__errno_location() = 0 as libc::c_int;
    memcache_parse_rsp(rsp);
    expect_same_ptr(
        (*m).last as *const libc::c_void,
        (*rsp).pos as *const libc::c_void,
        b"memcache_parse_rsp: expected rsp->pos to be m->last\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        SW_START,
        (*rsp).state,
        b"memcache_parse_rsp: expected state to be SW_START after parsing full buffer\0"
            as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        expected,
        (*rsp).type_0 as libc::c_int,
        b"memcache_parse_rsp: expected response type to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        fake_client.err,
        b"memcache_parse_rsp: expected no connection error\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*rsp).request() as libc::c_int,
        b"memcache_parse_rsp: expected response\0" as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*rsp).error() as libc::c_int,
        b"memcache_parse_rsp: expected no error\0" as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*rsp).swallow() as libc::c_int,
        b"memcache_parse_rsp: expected swallow=0\0" as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        *__errno_location(),
        b"memcache_parse_rsp: expected errno=0\0" as *const u8 as *const libc::c_char,
    );
    msg_put(rsp);
    if original_failures != failures {
        printf(
            b"Saw test failures for test_memcache_parse_rsp_success_case (%s)\n\0"
                as *const u8 as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_memcache_parse_rsp_success() {
    test_memcache_parse_rsp_success_case(
        b"0\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_NUM as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"0  \r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_NUM as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"9223372036854775807\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_NUM as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"DELETED\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_DELETED as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"END\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_END as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"ERROR\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_ERROR as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"EXISTS\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_EXISTS as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"NOT_FOUND\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_NOT_FOUND as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"STORED\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_STORED as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"TOUCHED\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_TOUCHED as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"VALUE key 0 2\r\nab\r\nEND\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_END as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"VALUE key 0 2\r\nab\r\nVALUE key2 0 2\r\ncd\r\nEND\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_RSP_MC_END as libc::c_int,
    );
    test_memcache_parse_rsp_success_case(
        b"VERSION 1.5.22\r\n\0" as *const u8 as *const libc::c_char,
        MSG_RSP_MC_VERSION as libc::c_int,
    );
}
unsafe extern "C" fn test_memcache_parse_rsp_failure_case(
    mut data: *const libc::c_char,
) {
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let original_failures: libc::c_int = failures;
    let mut rsp: *mut msg = msg_get(
        &mut fake_client,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    (*rsp).state = SW_START;
    (*rsp).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*rsp).mhdr.stqh_first = 0 as *mut mbuf;
    (*rsp).mhdr.stqh_last = &mut (*rsp).mhdr.stqh_first;
    mbuf_insert(&mut (*rsp).mhdr, m);
    (*rsp).pos = (*m).start;
    *__errno_location() = 0 as libc::c_int;
    memcache_parse_rsp(rsp);
    expect_same_ptr(
        (*m).start as *const libc::c_void,
        (*rsp).pos as *const libc::c_void,
        b"memcache_parse_rsp: expected rsp->pos to be m->start\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*rsp).type_0 as libc::c_int,
        b"memcache_parse_rsp: expected response type to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        MSG_PARSE_ERROR as libc::c_int,
        (*rsp).result as libc::c_int,
        b"memcache_parse_rsp: expected MSG_PARSE_ERROR\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        22 as libc::c_int,
        *__errno_location(),
        b"memcache_parse_rsp: expected EINVAL\0" as *const u8 as *const libc::c_char,
    );
    msg_put(rsp);
    if original_failures != failures {
        printf(
            b"Saw test failures for test_memcache_parse_rsp_success_case (%s)\n\0"
                as *const u8 as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_memcache_parse_rsp_failure() {
    test_memcache_parse_rsp_failure_case(b"\r\n\0" as *const u8 as *const libc::c_char);
    test_memcache_parse_rsp_failure_case(
        b"ENDD\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_rsp_failure_case(b"\r\0" as *const u8 as *const libc::c_char);
    test_memcache_parse_rsp_failure_case(
        b"-1\r\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn test_memcache_parse_req_success_case(
    mut data: *const libc::c_char,
    mut expected: libc::c_int,
) {
    let original_failures: libc::c_int = failures;
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let expected_noreply: libc::c_int = (strstr(
        data,
        b" noreply\0" as *const u8 as *const libc::c_char,
    ) != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    let mut req: *mut msg = msg_get(
        &mut fake_client,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    (*req).state = SW_START;
    (*req).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*req).mhdr.stqh_first = 0 as *mut mbuf;
    (*req).mhdr.stqh_last = &mut (*req).mhdr.stqh_first;
    mbuf_insert(&mut (*req).mhdr, m);
    (*req).pos = (*m).start;
    memcache_parse_req(req);
    expect_same_ptr(
        (*m).last as *const libc::c_void,
        (*req).pos as *const libc::c_void,
        b"memcache_parse_req: expected req->pos to be m->last\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        SW_START,
        (*req).state,
        b"memcache_parse_req: expected state to be SW_START after parsing full buffer\0"
            as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        expected,
        (*req).type_0 as libc::c_int,
        b"memcache_parse_req: expected response type to be parsed\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        expected_noreply,
        (*req).noreply() as libc::c_int,
        b"memcache_parse_req: unexpected noreply value\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*req).noforward() as libc::c_int,
        b"memcache_parse_req: unexpected noforward value\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        1 as libc::c_int,
        (*req).request() as libc::c_int,
        b"memcache_parse_req: expected request\0" as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        (*req).error() as libc::c_int,
        b"memcache_parse_req: expected no error\0" as *const u8 as *const libc::c_char,
    );
    expect_same_int(
        if !(strstr(data, b"quit\r\n\0" as *const u8 as *const libc::c_char)).is_null() {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        },
        (*req).quit() as libc::c_int,
        b"memcache_parse_req: unexpected quit value\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        0 as libc::c_int,
        fake_client.err,
        b"memcache_parse_req: expected no connection error\0" as *const u8
            as *const libc::c_char,
    );
    msg_put(req);
    if original_failures != failures {
        printf(
            b"Saw test failures for test_memcache_parse_req_success_case (%s)\n\0"
                as *const u8 as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_memcache_parse_req_success() {
    test_memcache_parse_req_success_case(
        b"add key 0 600 5\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_ADD as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"add key 0 0 1 noreply\r\n\n\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_ADD as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"append key 0 600 5\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_APPEND as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"append key 0 1 0 noreply\r\n\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_APPEND as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"cas key 0 600 5 123456\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_CAS as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"cas key 0 1 1 1 noreply\r\nx\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_CAS as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"decr key 0\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_DECR as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"decr key 0 noreply\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_DECR as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"delete a noreply\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_DELETE as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"delete key\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_DELETE as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"get a b xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\r\n\0"
            as *const u8 as *const libc::c_char,
        MSG_REQ_MC_GET as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"get key\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_GET as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"gets u\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_GETS as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"incr key 1\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_INCR as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"incr key 9223372036854775807 noreply\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_MC_INCR as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"prepend key 0 600 5\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_PREPEND as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"prepend key 0 600 0 noreply\r\n\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_PREPEND as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"quit\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_QUIT as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"replace key 0 600 5\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_REPLACE as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"replace key 0 9 0 noreply\r\n\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_REPLACE as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"set key 0 5 10 noreply\r\nvalue12345\r\n\0" as *const u8
            as *const libc::c_char,
        MSG_REQ_MC_SET as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"set key 0 600 5\r\nvalue\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_SET as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"touch key 12345\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_TOUCH as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"touch key 12345 noreply\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_TOUCH as libc::c_int,
    );
    test_memcache_parse_req_success_case(
        b"version\r\n\0" as *const u8 as *const libc::c_char,
        MSG_REQ_MC_VERSION as libc::c_int,
    );
}
unsafe extern "C" fn test_memcache_parse_req_failure_case(
    mut data: *const libc::c_char,
) {
    let original_failures: libc::c_int = failures;
    let mut fake_client: conn = {
        let mut init = conn {
            recv_active_recv_ready_send_active_send_ready_client_proxy_connecting_connected_eof_done_redis_authenticated: [0; 2],
            c2rust_padding: [0; 6],
            conn_tqe: {
                let mut init = C2RustUnnamed_41 {
                    tqe_next: 0 as *mut conn,
                    tqe_prev: 0 as *mut *mut conn,
                };
                init
            },
            owner: 0 as *mut libc::c_void,
            sd: 0,
            family: 0,
            addrlen: 0,
            addr: 0 as *mut sockaddr,
            imsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            omsg_q: msg_tqh {
                tqh_first: 0 as *mut msg,
                tqh_last: 0 as *mut *mut msg,
            },
            rmsg: 0 as *mut msg,
            smsg: 0 as *mut msg,
            recv: None,
            recv_next: None,
            recv_done: None,
            send: None,
            send_next: None,
            send_done: None,
            close: None,
            active: None,
            post_connect: None,
            swallow_msg: None,
            ref_0: None,
            unref: None,
            enqueue_inq: None,
            dequeue_inq: None,
            enqueue_outq: None,
            dequeue_outq: None,
            recv_bytes: 0,
            send_bytes: 0,
            events: 0,
            err: 0,
        };
        init.set_recv_active(0);
        init.set_recv_ready(0);
        init.set_send_active(0);
        init.set_send_ready(0);
        init.set_client(0);
        init.set_proxy(0);
        init.set_connecting(0);
        init.set_connected(0);
        init.set_eof(0);
        init.set_done(0);
        init.set_redis(0);
        init.set_authenticated(0);
        init
    };
    let mut m: *mut mbuf = mbuf_get();
    let SW_START: libc::c_int = 0 as libc::c_int;
    let mut req: *mut msg = msg_get(
        &mut fake_client,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    (*req).state = SW_START;
    (*req).token = 0 as *mut uint8_t;
    let datalen: size_t = strlen(data);
    mbuf_copy(m, data as *const uint8_t, datalen);
    (*req).mhdr.stqh_first = 0 as *mut mbuf;
    (*req).mhdr.stqh_last = &mut (*req).mhdr.stqh_first;
    mbuf_insert(&mut (*req).mhdr, m);
    (*req).pos = (*m).start;
    *__errno_location() = 0 as libc::c_int;
    memcache_parse_req(req);
    expect_same_ptr(
        (*m).start as *const libc::c_void,
        (*req).pos as *const libc::c_void,
        b"memcache_parse_rsp: expected rsp->pos to be m->start\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        MSG_PARSE_ERROR as libc::c_int,
        (*req).result as libc::c_int,
        b"memcache_parse_rsp: expected MSG_PARSE_ERROR\0" as *const u8
            as *const libc::c_char,
    );
    expect_same_int(
        22 as libc::c_int,
        *__errno_location(),
        b"memcache_parse_rsp: expected EINVAL\0" as *const u8 as *const libc::c_char,
    );
    msg_put(req);
    if original_failures != failures {
        printf(
            b"Saw test failures for test_memcache_parse_req_success_case (%s)\n\0"
                as *const u8 as *const libc::c_char,
            data,
        );
    }
}
unsafe extern "C" fn test_memcache_parse_req_failure() {
    test_memcache_parse_req_failure_case(
        b"add xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx 0 600 5\r\nvalue\r\n\0"
            as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"add\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"get\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"get \r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"get key\r\r\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"append key 0 600\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"cas key 0 600 5 \r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"decr key 0 extra\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"decr key\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"delete \r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"DELETE key\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"gets\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"incr\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"incr key 0notanint\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"prepend key 0 600 5\r\nvalueextra\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"prepend key 0 600 0 noreply\r\r\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"replace key 0 9 ?\r\n\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"set key 0 5 10 noreply\r\nvalue12345\r\r\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"set key 0 600 5\r\nvaluee\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"touch missingarg\r\n\0" as *const u8 as *const libc::c_char,
    );
    test_memcache_parse_req_failure_case(
        b"version extra\r\n\0" as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut nci: instance = {
        let mut init = instance {
            pidfile: [0; 1],
            c2rust_padding: [0; 7],
            ctx: 0 as *mut context,
            log_level: 0,
            log_filename: 0 as *const libc::c_char,
            conf_filename: 0 as *const libc::c_char,
            stats_port: 0,
            stats_interval: 0,
            stats_addr: 0 as *const libc::c_char,
            hostname: [0; 256],
            mbuf_chunk_size: 0,
            pid: 0,
            pid_filename: 0 as *const libc::c_char,
        };
        init.set_pidfile(0);
        init
    };
    nci.mbuf_chunk_size = 16384 as libc::c_int as size_t;
    mbuf_init(&mut nci);
    msg_init();
    log_init(7 as libc::c_int, 0 as *const libc::c_char);
    test_hash_algorithms();
    test_config_parsing();
    test_redis_parse_rsp_success();
    test_redis_parse_req_success();
    test_memcache_parse_rsp_success();
    test_memcache_parse_req_success();
    printf(
        b"Starting tests of request/response parsing failures\n\0" as *const u8
            as *const libc::c_char,
    );
    test_memcache_parse_rsp_failure();
    test_memcache_parse_req_failure();
    test_redis_parse_rsp_failure();
    printf(
        b"%d successes, %d failures\n\0" as *const u8 as *const libc::c_char,
        successes,
        failures,
    );
    msg_deinit();
    mbuf_deinit();
    log_deinit();
    return if failures > 0 as libc::c_int { 1 as libc::c_int } else { 0 as libc::c_int };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
