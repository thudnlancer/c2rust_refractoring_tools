#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type epoll_event;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn _stats_server_decr(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
    );
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn array_init(a: *mut array, n: uint32_t, size: size_t) -> rstatus_t;
    fn array_deinit(a: *mut array);
    fn array_pop(a: *mut array) -> *mut libc::c_void;
    fn array_get(a: *const array, idx: uint32_t) -> *mut libc::c_void;
    fn array_each(
        a: *const array,
        func: array_each_t,
        data: *mut libc::c_void,
    ) -> rstatus_t;
    fn string_empty(str: *const string) -> bool;
    fn nc_set_nonblocking(sd: libc::c_int) -> libc::c_int;
    fn nc_set_tcpnodelay(sd: libc::c_int) -> libc::c_int;
    fn _nc_free(ptr: *mut libc::c_void, name: *const libc::c_char, line: libc::c_int);
    fn nc_usec_now() -> int64_t;
    fn nc_resolve(
        name: *const string,
        port: libc::c_int,
        si: *mut sockinfo,
    ) -> libc::c_int;
    fn log_loggable(level: libc::c_int) -> libc::c_int;
    fn _log(
        file: *const libc::c_char,
        line: libc::c_int,
        panic: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn event_add_out(evb: *mut event_base, c: *mut conn) -> libc::c_int;
    fn event_add_conn(evb: *mut event_base, c: *mut conn) -> libc::c_int;
    fn _stats_pool_incr(
        ctx: *mut context,
        pool: *const server_pool,
        fidx: stats_pool_field_t,
    );
    fn _stats_server_incr(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
    );
    fn req_put(msg: *mut msg);
    fn req_done(conn: *const conn, msg: *mut msg) -> bool;
    fn rsp_put(msg: *mut msg);
    fn conn_get(owner: *mut libc::c_void, client: bool, redis: bool) -> *mut conn;
    fn conn_put(conn: *mut conn);
    fn _stats_server_set_ts(
        ctx: *mut context,
        server: *const server,
        fidx: stats_server_field_t,
        val: int64_t,
    );
    fn random_update(pool: *mut server_pool) -> rstatus_t;
    fn modula_update(pool: *mut server_pool) -> rstatus_t;
    fn random_dispatch(
        continuum: *const continuum,
        ncontinuum: uint32_t,
        hash: uint32_t,
    ) -> uint32_t;
    fn modula_dispatch(
        continuum: *const continuum,
        ncontinuum: uint32_t,
        hash: uint32_t,
    ) -> uint32_t;
    fn ketama_update(pool: *mut server_pool) -> rstatus_t;
    fn ketama_dispatch(
        continuum: *const continuum,
        ncontinuum: uint32_t,
        hash: uint32_t,
    ) -> uint32_t;
    fn conf_server_each_transform(
        elem: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> rstatus_t;
    fn conf_pool_each_transform(
        elem: *mut libc::c_void,
        data: *mut libc::c_void,
    ) -> rstatus_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type int64_t = __int64_t;
pub type pthread_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
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
    YAML_FOLDED_SCALAR_STYLE = 5,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_ANY_SCALAR_STYLE = 0,
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
    YAML_UTF16BE_ENCODING = 3,
    YAML_UTF16LE_ENCODING = 2,
    YAML_UTF8_ENCODING = 1,
    YAML_ANY_ENCODING = 0,
}  // end of enum

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
    YAML_FLOW_MAPPING_STYLE = 2,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_ANY_MAPPING_STYLE = 0,
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
    YAML_FLOW_SEQUENCE_STYLE = 2,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_ANY_SEQUENCE_STYLE = 0,
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
    YAML_MAPPING_NODE = 3,
    YAML_SEQUENCE_NODE = 2,
    YAML_SCALAR_NODE = 1,
    YAML_NO_NODE = 0,
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
pub type _IO_lock_t = ();
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
    YAML_EMITTER_ERROR = 7,
    YAML_WRITER_ERROR = 6,
    YAML_COMPOSER_ERROR = 5,
    YAML_PARSER_ERROR = 4,
    YAML_SCANNER_ERROR = 3,
    YAML_READER_ERROR = 2,
    YAML_MEMORY_ERROR = 1,
    YAML_NO_ERROR = 0,
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
    MSG_PARSE_AGAIN = 3,
    MSG_PARSE_REPAIR = 2,
    MSG_PARSE_ERROR = 1,
    MSG_PARSE_OK = 0,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum __socket_type {
    SOCK_NONBLOCK = 2048,
    SOCK_CLOEXEC = 524288,
    SOCK_PACKET = 10,
    SOCK_DCCP = 6,
    SOCK_SEQPACKET = 5,
    SOCK_RDM = 4,
    SOCK_RAW = 3,
    SOCK_DGRAM = 2,
    SOCK_STREAM = 1,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type array_each_t = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> rstatus_t,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_pool_field {
    STATS_POOL_NFIELD = 6,
    STATS_POOL_fragments = 5,
    STATS_POOL_forward_error = 4,
    STATS_POOL_server_ejects = 3,
    STATS_POOL_client_connections = 2,
    STATS_POOL_client_err = 1,
    STATS_POOL_client_eof = 0,
}  // end of enum

pub type stats_pool_field_t = stats_pool_field;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum stats_server_field {
    STATS_SERVER_NFIELD = 13,
    STATS_SERVER_out_queue_bytes = 12,
    STATS_SERVER_out_queue = 11,
    STATS_SERVER_in_queue_bytes = 10,
    STATS_SERVER_in_queue = 9,
    STATS_SERVER_response_bytes = 8,
    STATS_SERVER_responses = 7,
    STATS_SERVER_request_bytes = 6,
    STATS_SERVER_requests = 5,
    STATS_SERVER_server_ejected_at = 4,
    STATS_SERVER_server_connections = 3,
    STATS_SERVER_server_timedout = 2,
    STATS_SERVER_server_err = 1,
    STATS_SERVER_server_eof = 0,
}  // end of enum

pub type stats_server_field_t = stats_server_field;
pub const DIST_RANDOM: dist_type = 2;
pub const DIST_MODULA: dist_type = 1;
pub const DIST_KETAMA: dist_type = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum dist_type {
    DIST_RANDOM = 2,
    DIST_MODULA = 1,
    DIST_KETAMA = 0,
    DIST_SENTINEL = 3,
}  // end of enum

#[inline]
unsafe extern "C" fn array_n(mut a: *const array) -> uint32_t {
    return (*a).nelem;
}
#[inline]
unsafe extern "C" fn _nc_strchr(
    mut p: *mut uint8_t,
    mut last: *mut uint8_t,
    mut c: uint8_t,
) -> *mut uint8_t {
    while p < last {
        if *p as libc::c_int == c as libc::c_int {
            return p;
        }
        p = p.offset(1);
        p;
    }
    return 0 as *mut uint8_t;
}
unsafe extern "C" fn server_resolve(mut server: *mut server, mut conn: *mut conn) {
    let mut status: rstatus_t = 0;
    status = nc_resolve(
        &mut (*server).addrstr,
        (*server).port as libc::c_int,
        &mut (*server).info,
    );
    if status != 0 as libc::c_int {
        (*conn).err = 112 as libc::c_int;
        (*conn).set_done(1 as libc::c_int as libc::c_uint);
        return;
    }
    (*conn).family = (*server).info.family;
    (*conn).addrlen = (*server).info.addrlen;
    (*conn).addr = &mut (*server).info.addr as *mut C2RustUnnamed_39 as *mut sockaddr;
}
#[no_mangle]
pub unsafe extern "C" fn server_ref(mut conn: *mut conn, mut owner: *mut libc::c_void) {
    let mut server: *mut server = owner as *mut server;
    server_resolve(server, conn);
    (*server).ns_conn_q = ((*server).ns_conn_q).wrapping_add(1);
    (*server).ns_conn_q;
    (*conn).conn_tqe.tqe_next = 0 as *mut conn;
    (*conn).conn_tqe.tqe_prev = (*server).s_conn_q.tqh_last;
    *(*server).s_conn_q.tqh_last = conn;
    (*server).s_conn_q.tqh_last = &mut (*conn).conn_tqe.tqe_next;
    (*conn).owner = owner;
}
#[no_mangle]
pub unsafe extern "C" fn server_unref(mut conn: *mut conn) {
    let mut server: *mut server = 0 as *mut server;
    server = (*conn).owner as *mut server;
    (*conn).owner = 0 as *mut libc::c_void;
    (*server).ns_conn_q = ((*server).ns_conn_q).wrapping_sub(1);
    (*server).ns_conn_q;
    let mut oldnext: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_next
        as *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_prev
        as *mut *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*conn).conn_tqe.tqe_next).is_null() {
        (*(*conn).conn_tqe.tqe_next).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_prev;
    } else {
        (*server).s_conn_q.tqh_last = (*conn).conn_tqe.tqe_prev;
    }
    *(*conn).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn server_timeout(mut conn: *mut conn) -> libc::c_int {
    let mut server: *mut server = 0 as *mut server;
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    server = (*conn).owner as *mut server;
    pool = (*server).owner;
    return (*pool).timeout;
}
#[no_mangle]
pub unsafe extern "C" fn server_active(mut conn: *const conn) -> bool {
    if !((*conn).imsg_q.tqh_first).is_null() {
        return 1 as libc::c_int != 0;
    }
    if !((*conn).omsg_q.tqh_first).is_null() {
        return 1 as libc::c_int != 0;
    }
    if !((*conn).rmsg).is_null() {
        return 1 as libc::c_int != 0;
    }
    if !((*conn).smsg).is_null() {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn server_each_set_owner(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut s: *mut server = elem as *mut server;
    let mut sp: *mut server_pool = data as *mut server_pool;
    (*s).owner = sp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_init(
    mut server: *mut array,
    mut conf_server: *mut array,
    mut sp: *mut server_pool,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut nserver: uint32_t = 0;
    nserver = array_n(conf_server);
    status = array_init(
        server,
        nserver,
        ::core::mem::size_of::<server>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    status = array_each(
        conf_server,
        Some(
            conf_server_each_transform
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        server as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_deinit(server);
        return status;
    }
    status = array_each(
        server,
        Some(
            server_each_set_owner
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        sp as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_deinit(server);
        return status;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_deinit(mut server: *mut array) {
    let mut i: uint32_t = 0;
    let mut nserver: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    nserver = array_n(server);
    while i < nserver {
        let mut s: *mut server = 0 as *mut server;
        s = array_pop(server) as *mut server;
        i = i.wrapping_add(1);
        i;
    }
    array_deinit(server);
}
#[no_mangle]
pub unsafe extern "C" fn server_conn(mut server: *mut server) -> *mut conn {
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    let mut conn: *mut conn = 0 as *mut conn;
    pool = (*server).owner;
    if (*server).ns_conn_q < (*pool).server_connections {
        return conn_get(
            server as *mut libc::c_void,
            0 as libc::c_int != 0,
            (*pool).redis() != 0,
        );
    }
    conn = (*server).s_conn_q.tqh_first;
    let mut oldnext: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_next
        as *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    let mut oldprev: *mut *mut libc::c_void = &mut (*conn).conn_tqe.tqe_prev
        as *mut *mut *mut conn as *mut libc::c_void as *mut *mut libc::c_void;
    if !((*conn).conn_tqe.tqe_next).is_null() {
        (*(*conn).conn_tqe.tqe_next).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_prev;
    } else {
        (*server).s_conn_q.tqh_last = (*conn).conn_tqe.tqe_prev;
    }
    *(*conn).conn_tqe.tqe_prev = (*conn).conn_tqe.tqe_next;
    *oldnext = 0 as *mut libc::c_void;
    *oldprev = 0 as *mut libc::c_void;
    (*conn).conn_tqe.tqe_next = 0 as *mut conn;
    (*conn).conn_tqe.tqe_prev = (*server).s_conn_q.tqh_last;
    *(*server).s_conn_q.tqh_last = conn;
    (*server).s_conn_q.tqh_last = &mut (*conn).conn_tqe.tqe_next;
    return conn;
}
unsafe extern "C" fn server_each_preconnect(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut server: *mut server = 0 as *mut server;
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    let mut conn: *mut conn = 0 as *mut conn;
    server = elem as *mut server;
    pool = (*server).owner;
    conn = server_conn(server);
    if conn.is_null() {
        return -(3 as libc::c_int);
    }
    status = server_connect((*pool).ctx, server, conn);
    if status != 0 as libc::c_int {
        if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int,
                0 as libc::c_int,
                b"connect to server '%.*s' failed, ignored: %s\0" as *const u8
                    as *const libc::c_char,
                (*server).pname.len,
                (*server).pname.data,
                strerror(*__errno_location()),
            );
        }
        server_close((*pool).ctx, conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_each_disconnect(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut server: *mut server = 0 as *mut server;
    let mut pool: *mut server_pool = 0 as *mut server_pool;
    server = elem as *mut server;
    pool = (*server).owner;
    while !((*server).s_conn_q.tqh_first).is_null() {
        let mut conn: *mut conn = 0 as *mut conn;
        conn = (*server).s_conn_q.tqh_first;
        ((*conn).close).expect("non-null function pointer")((*pool).ctx, conn);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_failure(mut ctx: *mut context, mut server: *mut server) {
    let mut pool: *mut server_pool = (*server).owner;
    let mut now: int64_t = 0;
    let mut next: int64_t = 0;
    let mut status: rstatus_t = 0;
    if (*pool).auto_eject_hosts() == 0 {
        return;
    }
    (*server).failure_count = ((*server).failure_count).wrapping_add(1);
    (*server).failure_count;
    if (*server).failure_count < (*pool).server_failure_limit {
        return;
    }
    now = nc_usec_now();
    if now < 0 as libc::c_int as libc::c_long {
        return;
    }
    _stats_server_set_ts(ctx, server, STATS_SERVER_server_ejected_at, now);
    next = now + (*pool).server_retry_timeout;
    _stats_pool_incr(ctx, pool, STATS_POOL_server_ejects);
    (*server).failure_count = 0 as libc::c_int as uint32_t;
    (*server).next_retry = next;
    status = server_pool_run(pool);
    if status != 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int,
                0 as libc::c_int,
                b"updating pool %u '%.*s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*pool).idx,
                (*pool).name.len,
                (*pool).name.data,
                strerror(*__errno_location()),
            );
        }
    }
}
unsafe extern "C" fn server_close_stats(
    mut ctx: *mut context,
    mut server: *mut server,
    mut err: err_t,
    mut eof: libc::c_uint,
    mut connected: libc::c_uint,
) {
    if connected != 0 {
        _stats_server_decr(ctx, server, STATS_SERVER_server_connections);
    }
    if eof != 0 {
        _stats_server_incr(ctx, server, STATS_SERVER_server_eof);
        return;
    }
    match err {
        110 => {
            _stats_server_incr(ctx, server, STATS_SERVER_server_timedout);
        }
        32 | 104 | 103 | 111 | 107 | 100 | 101 | 112 | 113 | _ => {
            _stats_server_incr(ctx, server, STATS_SERVER_server_err);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn server_close(mut ctx: *mut context, mut conn: *mut conn) {
    let mut status: rstatus_t = 0;
    let mut msg: *mut msg = 0 as *mut msg;
    let mut nmsg: *mut msg = 0 as *mut msg;
    let mut c_conn: *mut conn = 0 as *mut conn;
    server_close_stats(
        ctx,
        (*conn).owner as *mut server,
        (*conn).err,
        (*conn).eof(),
        (*conn).connected(),
    );
    (*conn).set_connected(0 as libc::c_int as libc::c_uint);
    if (*conn).sd < 0 as libc::c_int {
        server_failure(ctx, (*conn).owner as *mut server);
        ((*conn).unref).expect("non-null function pointer")(conn);
        conn_put(conn);
        return;
    }
    msg = (*conn).imsg_q.tqh_first;
    while !msg.is_null() {
        nmsg = (*msg).s_tqe.tqe_next;
        ((*conn).dequeue_inq).expect("non-null function pointer")(ctx, conn, msg);
        if (*msg).swallow() as libc::c_int != 0 || (*msg).noreply() as libc::c_int != 0 {
            req_put(msg);
        } else {
            c_conn = (*msg).owner;
            (*msg).set_done(1 as libc::c_int as libc::c_uint);
            (*msg).set_error(1 as libc::c_int as libc::c_uint);
            (*msg).err = (*conn).err;
            if !((*msg).frag_owner).is_null() {
                (*(*msg).frag_owner)
                    .nfrag_done = ((*(*msg).frag_owner).nfrag_done).wrapping_add(1);
                (*(*msg).frag_owner).nfrag_done;
            }
            if req_done(c_conn, (*c_conn).omsg_q.tqh_first) {
                event_add_out((*ctx).evb, (*msg).owner);
            }
        }
        msg = nmsg;
    }
    msg = (*conn).omsg_q.tqh_first;
    while !msg.is_null() {
        nmsg = (*msg).s_tqe.tqe_next;
        ((*conn).dequeue_outq).expect("non-null function pointer")(ctx, conn, msg);
        if (*msg).swallow() != 0 {
            req_put(msg);
        } else {
            c_conn = (*msg).owner;
            (*msg).set_done(1 as libc::c_int as libc::c_uint);
            (*msg).set_error(1 as libc::c_int as libc::c_uint);
            (*msg).err = (*conn).err;
            if !((*msg).frag_owner).is_null() {
                (*(*msg).frag_owner)
                    .nfrag_done = ((*(*msg).frag_owner).nfrag_done).wrapping_add(1);
                (*(*msg).frag_owner).nfrag_done;
            }
            if req_done(c_conn, (*c_conn).omsg_q.tqh_first) {
                event_add_out((*ctx).evb, (*msg).owner);
            }
        }
        msg = nmsg;
    }
    msg = (*conn).rmsg;
    if !msg.is_null() {
        (*conn).rmsg = 0 as *mut msg;
        rsp_put(msg);
    }
    server_failure(ctx, (*conn).owner as *mut server);
    ((*conn).unref).expect("non-null function pointer")(conn);
    status = close((*conn).sd);
    if status < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                458 as libc::c_int,
                0 as libc::c_int,
                b"close s %d failed, ignored: %s\0" as *const u8 as *const libc::c_char,
                (*conn).sd,
                strerror(*__errno_location()),
            );
        }
    }
    (*conn).sd = -(1 as libc::c_int);
    conn_put(conn);
}
#[no_mangle]
pub unsafe extern "C" fn server_connect(
    mut ctx: *mut context,
    mut server: *mut server,
    mut conn: *mut conn,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    if (*conn).err != 0 {
        *__errno_location() = (*conn).err;
        return -(1 as libc::c_int);
    }
    if (*conn).sd > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*conn).sd = socket((*conn).family, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if (*conn).sd < 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int,
                0 as libc::c_int,
                b"socket for server '%.*s' failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*server).pname.len,
                (*server).pname.data,
                strerror(*__errno_location()),
            );
        }
        status = -(1 as libc::c_int);
    } else {
        status = nc_set_nonblocking((*conn).sd);
        if status != 0 as libc::c_int {
            if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                _log(
                    b"nc_server.c\0" as *const u8 as *const libc::c_char,
                    498 as libc::c_int,
                    0 as libc::c_int,
                    b"set nonblock on s %d for server '%.*s' failed: %s\0" as *const u8
                        as *const libc::c_char,
                    (*conn).sd,
                    (*server).pname.len,
                    (*server).pname.data,
                    strerror(*__errno_location()),
                );
            }
        } else {
            if *((*server).pname.data).offset(0 as libc::c_int as isize) as libc::c_int
                != '/' as i32
            {
                status = nc_set_tcpnodelay((*conn).sd);
                if status != 0 as libc::c_int {
                    if log_loggable(4 as libc::c_int) != 0 as libc::c_int {
                        _log(
                            b"nc_server.c\0" as *const u8 as *const libc::c_char,
                            507 as libc::c_int,
                            0 as libc::c_int,
                            b"set tcpnodelay on s %d for server '%.*s' failed, ignored: %s\0"
                                as *const u8 as *const libc::c_char,
                            (*conn).sd,
                            (*server).pname.len,
                            (*server).pname.data,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
            status = event_add_conn((*ctx).evb, conn);
            if status != 0 as libc::c_int {
                if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                    _log(
                        b"nc_server.c\0" as *const u8 as *const libc::c_char,
                        515 as libc::c_int,
                        0 as libc::c_int,
                        b"event add conn s %d for server '%.*s' failed: %s\0"
                            as *const u8 as *const libc::c_char,
                        (*conn).sd,
                        (*server).pname.len,
                        (*server).pname.data,
                        strerror(*__errno_location()),
                    );
                }
            } else {
                status = connect(
                    (*conn).sd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: (*conn).addr,
                    },
                    (*conn).addrlen,
                );
                if status != 0 as libc::c_int {
                    if *__errno_location() == 115 as libc::c_int {
                        (*conn).set_connecting(1 as libc::c_int as libc::c_uint);
                        return 0 as libc::c_int;
                    }
                    if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
                        _log(
                            b"nc_server.c\0" as *const u8 as *const libc::c_char,
                            531 as libc::c_int,
                            0 as libc::c_int,
                            b"connect on s %d to server '%.*s' failed: %s\0" as *const u8
                                as *const libc::c_char,
                            (*conn).sd,
                            (*server).pname.len,
                            (*server).pname.data,
                            strerror(*__errno_location()),
                        );
                    }
                } else {
                    (*conn).set_connected(1 as libc::c_int as libc::c_uint);
                    return 0 as libc::c_int;
                }
            }
        }
    }
    (*conn).err = *__errno_location();
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn server_connected(mut ctx: *mut context, mut conn: *mut conn) {
    let mut server: *mut server = (*conn).owner as *mut server;
    _stats_server_incr(ctx, server, STATS_SERVER_server_connections);
    (*conn).set_connecting(0 as libc::c_int as libc::c_uint);
    (*conn).set_connected(1 as libc::c_int as libc::c_uint);
    ((*conn).post_connect).expect("non-null function pointer")(ctx, conn, server);
}
#[no_mangle]
pub unsafe extern "C" fn server_ok(mut ctx: *mut context, mut conn: *mut conn) {
    let mut server: *mut server = (*conn).owner as *mut server;
    if (*server).failure_count != 0 as libc::c_int as libc::c_uint {
        (*server).failure_count = 0 as libc::c_int as uint32_t;
        (*server).next_retry = 0 as libc::c_longlong as int64_t;
    }
}
unsafe extern "C" fn server_pool_update(mut pool: *mut server_pool) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut now: int64_t = 0;
    let mut pnlive_server: uint32_t = 0;
    if (*pool).auto_eject_hosts() == 0 {
        return 0 as libc::c_int;
    }
    if (*pool).next_rebuild as libc::c_longlong == 0 as libc::c_longlong {
        return 0 as libc::c_int;
    }
    now = nc_usec_now();
    if now < 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    if now <= (*pool).next_rebuild {
        if (*pool).nlive_server == 0 as libc::c_int as libc::c_uint {
            *__errno_location() = 111 as libc::c_int;
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    pnlive_server = (*pool).nlive_server;
    status = server_pool_run(pool);
    if status != 0 as libc::c_int {
        if log_loggable(1 as libc::c_int) != 0 as libc::c_int {
            _log(
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                617 as libc::c_int,
                0 as libc::c_int,
                b"updating pool %u with dist %d failed: %s\0" as *const u8
                    as *const libc::c_char,
                (*pool).idx,
                (*pool).dist_type,
                strerror(*__errno_location()),
            );
        }
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_pool_hash(
    mut pool: *const server_pool,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> uint32_t {
    if array_n(&(*pool).server) == 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as uint32_t;
    }
    if keylen == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as uint32_t;
    }
    return ((*pool).key_hash)
        .expect(
            "non-null function pointer",
        )(key as *const libc::c_char, keylen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_idx(
    mut pool: *const server_pool,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut idx: uint32_t = 0;
    let mut nserver: uint32_t = array_n(&(*pool).server);
    if nserver == 1 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int as uint32_t;
    }
    if !string_empty(&(*pool).hash_tag) {
        let mut tag: *const string = &(*pool).hash_tag;
        let mut tag_start: *const uint8_t = 0 as *const uint8_t;
        let mut tag_end: *const uint8_t = 0 as *const uint8_t;
        tag_start = _nc_strchr(
            key as *mut uint8_t,
            key.offset(keylen as isize) as *mut uint8_t,
            *((*tag).data).offset(0 as libc::c_int as isize),
        );
        if !tag_start.is_null() {
            tag_end = _nc_strchr(
                tag_start.offset(1 as libc::c_int as isize) as *mut uint8_t,
                key.offset(keylen as isize) as *mut uint8_t,
                *((*tag).data).offset(1 as libc::c_int as isize),
            );
            if !tag_end.is_null()
                && tag_end.offset_from(tag_start) as libc::c_long
                    > 1 as libc::c_int as libc::c_long
            {
                key = tag_start.offset(1 as libc::c_int as isize);
                keylen = tag_end.offset_from(key) as libc::c_long as uint32_t;
            }
        }
    }
    match (*pool).dist_type {
        0 => {
            hash = server_pool_hash(pool, key, keylen);
            idx = ketama_dispatch((*pool).continuum, (*pool).ncontinuum, hash);
        }
        1 => {
            hash = server_pool_hash(pool, key, keylen);
            idx = modula_dispatch((*pool).continuum, (*pool).ncontinuum, hash);
        }
        2 => {
            idx = random_dispatch(
                (*pool).continuum,
                (*pool).ncontinuum,
                0 as libc::c_int as uint32_t,
            );
        }
        _ => return 0 as libc::c_int as uint32_t,
    }
    return idx;
}
unsafe extern "C" fn server_pool_server(
    mut pool: *mut server_pool,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> *mut server {
    let mut server: *mut server = 0 as *mut server;
    let mut idx: uint32_t = 0;
    idx = server_pool_idx(pool, key, keylen);
    server = array_get(&mut (*pool).server, idx) as *mut server;
    return server;
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_conn(
    mut ctx: *mut context,
    mut pool: *mut server_pool,
    mut key: *const uint8_t,
    mut keylen: uint32_t,
) -> *mut conn {
    let mut status: rstatus_t = 0;
    let mut server: *mut server = 0 as *mut server;
    let mut conn: *mut conn = 0 as *mut conn;
    status = server_pool_update(pool);
    if status != 0 as libc::c_int {
        return 0 as *mut conn;
    }
    server = server_pool_server(pool, key, keylen);
    if server.is_null() {
        return 0 as *mut conn;
    }
    conn = server_conn(server);
    if conn.is_null() {
        return 0 as *mut conn;
    }
    status = server_connect(ctx, server, conn);
    if status != 0 as libc::c_int {
        server_close(ctx, conn);
        return 0 as *mut conn;
    }
    return conn;
}
unsafe extern "C" fn server_pool_each_preconnect(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut sp: *mut server_pool = elem as *mut server_pool;
    if (*sp).preconnect() == 0 {
        return 0 as libc::c_int;
    }
    status = array_each(
        &mut (*sp).server,
        Some(
            server_each_preconnect
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_preconnect(mut ctx: *mut context) -> rstatus_t {
    let mut status: rstatus_t = 0;
    status = array_each(
        &mut (*ctx).pool,
        Some(
            server_pool_each_preconnect
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_pool_each_disconnect(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut sp: *mut server_pool = elem as *mut server_pool;
    status = array_each(
        &mut (*sp).server,
        Some(
            server_each_disconnect
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_disconnect(mut ctx: *mut context) {
    array_each(
        &mut (*ctx).pool,
        Some(
            server_pool_each_disconnect
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn server_pool_each_set_owner(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut sp: *mut server_pool = elem as *mut server_pool;
    let mut ctx: *mut context = data as *mut context;
    (*sp).ctx = ctx;
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_pool_each_calc_connections(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    let mut sp: *mut server_pool = elem as *mut server_pool;
    let mut ctx: *mut context = data as *mut context;
    (*ctx)
        .max_nsconn = ((*ctx).max_nsconn as libc::c_uint)
        .wrapping_add(
            ((*sp).server_connections).wrapping_mul(array_n(&mut (*sp).server)),
        ) as uint32_t as uint32_t;
    (*ctx)
        .max_nsconn = ((*ctx).max_nsconn as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_run(mut pool: *mut server_pool) -> rstatus_t {
    match (*pool).dist_type {
        0 => return ketama_update(pool),
        1 => return modula_update(pool),
        2 => return random_update(pool),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn server_pool_each_run(
    mut elem: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> rstatus_t {
    return server_pool_run(elem as *mut server_pool);
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_init(
    mut server_pool: *mut array,
    mut conf_pool: *mut array,
    mut ctx: *mut context,
) -> rstatus_t {
    let mut status: rstatus_t = 0;
    let mut npool: uint32_t = 0;
    npool = array_n(conf_pool);
    status = array_init(
        server_pool,
        npool,
        ::core::mem::size_of::<server_pool>() as libc::c_ulong,
    );
    if status != 0 as libc::c_int {
        return status;
    }
    status = array_each(
        conf_pool,
        Some(
            conf_pool_each_transform
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        server_pool as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_pool_deinit(server_pool);
        return status;
    }
    status = array_each(
        server_pool,
        Some(
            server_pool_each_set_owner
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        ctx as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_pool_deinit(server_pool);
        return status;
    }
    (*ctx).max_nsconn = 0 as libc::c_int as uint32_t;
    status = array_each(
        server_pool,
        Some(
            server_pool_each_calc_connections
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        ctx as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_pool_deinit(server_pool);
        return status;
    }
    status = array_each(
        server_pool,
        Some(
            server_pool_each_run
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> rstatus_t,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 as libc::c_int {
        server_pool_deinit(server_pool);
        return status;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn server_pool_deinit(mut server_pool: *mut array) {
    let mut i: uint32_t = 0;
    let mut npool: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    npool = array_n(server_pool);
    while i < npool {
        let mut sp: *mut server_pool = 0 as *mut server_pool;
        sp = array_pop(server_pool) as *mut server_pool;
        if !((*sp).continuum).is_null() {
            _nc_free(
                (*sp).continuum as *mut libc::c_void,
                b"nc_server.c\0" as *const u8 as *const libc::c_char,
                918 as libc::c_int,
            );
            (*sp).continuum = 0 as *mut continuum;
            (*sp).ncontinuum = 0 as libc::c_int as uint32_t;
            (*sp).nserver_continuum = 0 as libc::c_int as uint32_t;
            (*sp).nlive_server = 0 as libc::c_int as uint32_t;
        }
        server_deinit(&mut (*sp).server);
        i = i.wrapping_add(1);
        i;
    }
    array_deinit(server_pool);
}
