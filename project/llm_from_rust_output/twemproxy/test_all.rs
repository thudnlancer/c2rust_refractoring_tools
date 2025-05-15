use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_uchar, c_ushort, c_long, c_void};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{FILE, _IO_FILE, _IO_marker, __off_t, __off64_t, __pid_t, __uint32_t, __uint16_t, __uint8_t, __uint64_t, __int64_t};
use std::collections::HashMap;

type rstatus_t = c_int;
type err_t = c_int;
type uint32_t = __uint32_t;
type size_t = c_ulong;
type uint8_t = __uint8_t;
type uint16_t = __uint16_t;
type int64_t = __int64_t;
type uint64_t = __uint64_t;
type pid_t = __pid_t;
type socklen_t = c_uint;
type in_addr_t = uint32_t;
type in_port_t = uint16_t;
type sa_family_t = c_ushort;
type mode_t = c_uint;
type yaml_char_t = c_uchar;

#[derive(Debug, Clone, Copy)]
struct string {
    len: uint32_t,
    data: *mut uint8_t,
}

#[derive(Debug, Clone, Copy)]
struct array {
    nelem: uint32_t,
    elem: *mut c_void,
    size: size_t,
    nalloc: uint32_t,
}

#[derive(Debug, Clone, Copy)]
struct context {
    id: uint32_t,
    cf: *mut conf,
    stats: *mut stats,
    pool: array,
    evb: *mut event_base,
    max_timeout: c_int,
    timeout: c_int,
    max_nfd: uint32_t,
    max_ncconn: uint32_t,
    max_nsconn: uint32_t,
}

#[derive(Debug, Clone, Copy)]
struct event_base {
    ep: c_int,
    event: *mut epoll_event,
    nevent: c_int,
    cb: Option<unsafe extern "C" fn(*mut c_void, uint32_t) -> c_int>,
}

#[derive(Debug, Clone, Copy)]
struct stats {
    port: uint16_t,
    interval: c_int,
    addr: string,
    start_ts: int64_t,
    buf: stats_buffer,
    current: array,
    shadow: array,
    sum: array,
    tid: pthread_t,
    sd: c_int,
    service_str: string,
    service: string,
    source_str: string,
    source: string,
    version_str: string,
    version: string,
    uptime_str: string,
    timestamp_str: string,
    ntotal_conn_str: string,
    ncurr_conn_str: string,
    aggregate: c_int,
    updated: c_int,
}

type pthread_t = c_ulong;

#[derive(Debug, Clone, Copy)]
struct stats_buffer {
    len: size_t,
    data: *mut uint8_t,
    size: size_t,
}

#[derive(Debug, Clone, Copy)]
struct conf {
    fname: *const c_char,
    fh: *mut FILE,
    arg: array,
    pool: array,
    depth: uint32_t,
    parser: yaml_parser_t,
    event: yaml_event_t,
    token: yaml_token_t,
    seq_valid_parser_valid_event_valid_token_sound_parsed_valid: [u8; 1],
    c2rust_padding: [u8; 7],
}

#[derive(Debug, Clone, Copy)]
struct yaml_token_t {
    type_: yaml_token_type_t,
    data: C2RustUnnamed,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_mark_t {
    index: size_t,
    line: size_t,
    column: size_t,
}

#[derive(Debug, Clone, Copy)]
union C2RustUnnamed {
    stream_start: C2RustUnnamed_6,
    alias: C2RustUnnamed_5,
    anchor: C2RustUnnamed_4,
    tag: C2RustUnnamed_3,
    scalar: C2RustUnnamed_2,
    version_directive: C2RustUnnamed_1,
    tag_directive: C2RustUnnamed_0,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_0 {
    handle: *mut yaml_char_t,
    prefix: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_1 {
    major: c_int,
    minor: c_int,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_2 {
    value: *mut yaml_char_t,
    length: size_t,
    style: yaml_scalar_style_t,
}

type yaml_scalar_style_t = yaml_scalar_style_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_scalar_style_e {
    YAML_ANY_SCALAR_STYLE = 0,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_FOLDED_SCALAR_STYLE = 5,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_3 {
    handle: *mut yaml_char_t,
    suffix: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_4 {
    value: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_5 {
    value: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_6 {
    encoding: yaml_encoding_t,
}

type yaml_encoding_t = yaml_encoding_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_encoding_e {
    YAML_ANY_ENCODING = 0,
    YAML_UTF8_ENCODING = 1,
    YAML_UTF16LE_ENCODING = 2,
    YAML_UTF16BE_ENCODING = 3,
}

type yaml_token_type_t = yaml_token_type_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_token_type_e {
    YAML_NO_TOKEN = 0,
    YAML_STREAM_START_TOKEN = 1,
    YAML_STREAM_END_TOKEN = 2,
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    YAML_DOCUMENT_START_TOKEN = 5,
    YAML_DOCUMENT_END_TOKEN = 6,
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    YAML_BLOCK_END_TOKEN = 9,
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    YAML_BLOCK_ENTRY_TOKEN = 14,
    YAML_FLOW_ENTRY_TOKEN = 15,
    YAML_KEY_TOKEN = 16,
    YAML_VALUE_TOKEN = 17,
    YAML_ALIAS_TOKEN = 18,
    YAML_ANCHOR_TOKEN = 19,
    YAML_TAG_TOKEN = 20,
    YAML_SCALAR_TOKEN = 21,
}

#[derive(Debug, Clone, Copy)]
struct yaml_event_t {
    type_: yaml_event_type_t,
    data: C2RustUnnamed_7,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

#[derive(Debug, Clone, Copy)]
union C2RustUnnamed_7 {
    stream_start: C2RustUnnamed_15,
    document_start: C2RustUnnamed_13,
    document_end: C2RustUnnamed_12,
    alias: C2RustUnnamed_11,
    scalar: C2RustUnnamed_10,
    sequence_start: C2RustUnnamed_9,
    mapping_start: C2RustUnnamed_8,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_8 {
    anchor: *mut yaml_char_t,
    tag: *mut yaml_char_t,
    implicit: c_int,
    style: yaml_mapping_style_t,
}

type yaml_mapping_style_t = yaml_mapping_style_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_mapping_style_e {
    YAML_ANY_MAPPING_STYLE = 0,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_FLOW_MAPPING_STYLE = 2,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_9 {
    anchor: *mut yaml_char_t,
    tag: *mut yaml_char_t,
    implicit: c_int,
    style: yaml_sequence_style_t,
}

type yaml_sequence_style_t = yaml_sequence_style_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_sequence_style_e {
    YAML_ANY_SEQUENCE_STYLE = 0,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_FLOW_SEQUENCE_STYLE = 2,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_10 {
    anchor: *mut yaml_char_t,
    tag: *mut yaml_char_t,
    value: *mut yaml_char_t,
    length: size_t,
    plain_implicit: c_int,
    quoted_implicit: c_int,
    style: yaml_scalar_style_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_11 {
    anchor: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_12 {
    implicit: c_int,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_13 {
    version_directive: *mut yaml_version_directive_t,
    tag_directives: C2RustUnnamed_14,
    implicit: c_int,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_14 {
    start: *mut yaml_tag_directive_t,
    end: *mut yaml_tag_directive_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_tag_directive_t {
    handle: *mut yaml_char_t,
    prefix: *mut yaml_char_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_version_directive_t {
    major: c_int,
    minor: c_int,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_15 {
    encoding: yaml_encoding_t,
}

type yaml_event_type_t = yaml_event_type_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_event_type_e {
    YAML_NO_EVENT = 0,
    YAML_STREAM_START_EVENT = 1,
    YAML_STREAM_END_EVENT = 2,
    YAML_DOCUMENT_START_EVENT = 3,
    YAML_DOCUMENT_END_EVENT = 4,
    YAML_ALIAS_EVENT = 5,
    YAML_SCALAR_EVENT = 6,
    YAML_SEQUENCE_START_EVENT = 7,
    YAML_SEQUENCE_END_EVENT = 8,
    YAML_MAPPING_START_EVENT = 9,
    YAML_MAPPING_END_EVENT = 10,
}

#[derive(Debug, Clone, Copy)]
struct yaml_parser_t {
    error: yaml_error_type_t,
    problem: *const c_char,
    problem_offset: size_t,
    problem_value: c_int,
    problem_mark: yaml_mark_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    read_handler: Option<yaml_read_handler_t>,
    read_handler_data: *mut c_void,
    input: C2RustUnnamed_33,
    eof: c_int,
    buffer: C2RustUnnamed_32,
    unread: size_t,
    raw_buffer: C2RustUnnamed_31,
    encoding: yaml_encoding_t,
    offset: size_t,
    mark: yaml_mark_t,
    stream_start_produced: c_int,
    stream_end_produced: c_int,
    flow_level: c_int,
    tokens: C2RustUnnamed_30,
    tokens_parsed: size_t,
    token_available: c_int,
    indents: C2RustUnnamed_29,
    indent: c_int,
    simple_key_allowed: c_int,
    simple_keys: C2RustUnnamed_28,
    states: C2RustUnnamed_27,
    state: yaml_parser_state_t,
    marks: C2RustUnnamed_26,
    tag_directives: C2RustUnnamed_25,
    aliases: C2RustUnnamed_24,
    document: *mut yaml_document_t,
}

type yaml_read_handler_t = unsafe extern "C" fn(*mut c_void, *mut c_uchar, size_t, *mut size_t) -> c_int;

type yaml_error_type_t = yaml_error_type_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_error_type_e {
    YAML_NO_ERROR = 0,
    YAML_MEMORY_ERROR = 1,
    YAML_READER_ERROR = 2,
    YAML_SCANNER_ERROR = 3,
    YAML_PARSER_ERROR = 4,
    YAML_COMPOSER_ERROR = 5,
    YAML_WRITER_ERROR = 6,
    YAML_EMITTER_ERROR = 7,
}

#[derive(Debug, Clone, Copy)]
struct yaml_document_t {
    nodes: C2RustUnnamed_17,
    version_directive: *mut yaml_version_directive_t,
    tag_directives: C2RustUnnamed_16,
    start_implicit: c_int,
    end_implicit: c_int,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_16 {
    start: *mut yaml_tag_directive_t,
    end: *mut yaml_tag_directive_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_17 {
    start: *mut yaml_node_t,
    end: *mut yaml_node_t,
    top: *mut yaml_node_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_node_t {
    type_: yaml_node_type_t,
    tag: *mut yaml_char_t,
    data: C2RustUnnamed_18,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
}

#[derive(Debug, Clone, Copy)]
union C2RustUnnamed_18 {
    scalar: C2RustUnnamed_23,
    sequence: C2RustUnnamed_21,
    mapping: C2RustUnnamed_19,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_19 {
    pairs: C2RustUnnamed_20,
    style: yaml_mapping_style_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_20 {
    start: *mut yaml_node_pair_t,
    end: *mut yaml_node_pair_t,
    top: *mut yaml_node_pair_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_node_pair_t {
    key: c_int,
    value: c_int,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_21 {
    items: C2RustUnnamed_22,
    style: yaml_sequence_style_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_22 {
    start: *mut yaml_node_item_t,
    end: *mut yaml_node_item_t,
    top: *mut yaml_node_item_t,
}

type yaml_node_item_t = c_int;

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_23 {
    value: *mut yaml_char_t,
    length: size_t,
    style: yaml_scalar_style_t,
}

type yaml_node_type_t = yaml_node_type_e;

#[derive(Debug, Clone, Copy, PartialEq)]
enum yaml_node_type_e {
    YAML_NO_NODE = 0,
    YAML_SCALAR_NODE = 1,
    YAML_SEQUENCE_NODE = 2,
    YAML_MAPPING_NODE = 3,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_24 {
    start: *mut yaml_alias_data_t,
    end: *mut yaml_alias_data_t,
    top: *mut yaml_alias_data_t,
}

#[derive(Debug, Clone, Copy)]
struct yaml_alias_data_t {
    anchor: *mut yaml_char_t,
    index: c_int,
    mark: yaml_mark_t,
}

#[derive(Debug, Clone, Copy)]
struct C2RustUnnamed_25 {
    start: *mut yaml_tag_directive_t,
    end: *mut yaml_tag_directive_t,
    top: *mut yaml_tag_directive_t,
}

#[derive(Debug, Clone, Copy)]
struct C2R