#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    static mut stdout: *mut _IO_FILE;
    fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> i32;
    fn yaml_emitter_dump(
        emitter: *mut yaml_emitter_t,
        document: *mut yaml_document_t,
    ) -> i32;
    fn yaml_emitter_close(emitter: *mut yaml_emitter_t) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn yaml_document_initialize(
        document: *mut yaml_document_t,
        version_directive: *mut yaml_version_directive_t,
        tag_directives_start: *mut yaml_tag_directive_t,
        tag_directives_end: *mut yaml_tag_directive_t,
        start_implicit: i32,
        end_implicit: i32,
    ) -> i32;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_document_get_node(
        document: *mut yaml_document_t,
        index: i32,
    ) -> *mut yaml_node_t;
    fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t;
    fn yaml_document_add_scalar(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        value: *const yaml_char_t,
        length: i32,
        style: yaml_scalar_style_t,
    ) -> i32;
    fn yaml_document_add_sequence(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_sequence_style_t,
    ) -> i32;
    fn yaml_document_add_mapping(
        document: *mut yaml_document_t,
        tag: *const yaml_char_t,
        style: yaml_mapping_style_t,
    ) -> i32;
    fn yaml_document_append_sequence_item(
        document: *mut yaml_document_t,
        sequence: i32,
        item: i32,
    ) -> i32;
    fn yaml_document_append_mapping_pair(
        document: *mut yaml_document_t,
        mapping: i32,
        key: i32,
        value: i32,
    ) -> i32;
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32;
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_set_input_string(
        parser: *mut yaml_parser_t,
        input: *const u8,
        size: size_t,
    );
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_load(
        parser: *mut yaml_parser_t,
        document: *mut yaml_document_t,
    ) -> i32;
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> i32;
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_set_output_string(
        emitter: *mut yaml_emitter_t,
        output: *mut u8,
        size: size_t,
        size_written: *mut size_t,
    );
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: i32);
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: i32);
    fn yaml_emitter_open(emitter: *mut yaml_emitter_t) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub type yaml_char_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: i32,
    pub minor: i32,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
pub type yaml_encoding_e = u32;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_e = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_e = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_e = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_e = 0;
pub type yaml_encoding_t = yaml_encoding_e;
pub type yaml_break_e = u32;
pub const YAML_CRLN_BREAK: yaml_break_e = 3;
pub const YAML_LN_BREAK: yaml_break_e = 2;
pub const YAML_CR_BREAK: yaml_break_e = 1;
pub const YAML_ANY_BREAK: yaml_break_e = 0;
pub type yaml_break_t = yaml_break_e;
pub type yaml_error_type_e = u32;
pub const YAML_EMITTER_ERROR: yaml_error_type_e = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_e = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_e = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_e = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_e = 3;
pub const YAML_READER_ERROR: yaml_error_type_e = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_e = 1;
pub const YAML_NO_ERROR: yaml_error_type_e = 0;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
pub type yaml_mark_t = yaml_mark_s;
pub type yaml_scalar_style_e = u32;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_e = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_e = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_e = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_e = 0;
pub type yaml_scalar_style_t = yaml_scalar_style_e;
pub type yaml_sequence_style_e = u32;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_e = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_e = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_e = 0;
pub type yaml_sequence_style_t = yaml_sequence_style_e;
pub type yaml_mapping_style_e = u32;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_e = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_e = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_e = 0;
pub type yaml_mapping_style_t = yaml_mapping_style_e;
pub type yaml_token_type_e = u32;
pub const YAML_SCALAR_TOKEN: yaml_token_type_e = 21;
pub const YAML_TAG_TOKEN: yaml_token_type_e = 20;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_e = 19;
pub const YAML_ALIAS_TOKEN: yaml_token_type_e = 18;
pub const YAML_VALUE_TOKEN: yaml_token_type_e = 17;
pub const YAML_KEY_TOKEN: yaml_token_type_e = 16;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_e = 15;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_e = 14;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_e = 13;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_e = 12;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_e = 11;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_e = 10;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_e = 9;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_e = 8;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_e = 7;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_e = 6;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_e = 5;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_e = 4;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_e = 3;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_e = 2;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_e = 1;
pub const YAML_NO_TOKEN: yaml_token_type_e = 0;
pub type yaml_token_type_t = yaml_token_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
pub type yaml_token_t = yaml_token_s;
pub type yaml_event_type_e = u32;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_e = 10;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_e = 9;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_e = 8;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_e = 7;
pub const YAML_SCALAR_EVENT: yaml_event_type_e = 6;
pub const YAML_ALIAS_EVENT: yaml_event_type_e = 5;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_e = 4;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_e = 3;
pub const YAML_STREAM_END_EVENT: yaml_event_type_e = 2;
pub const YAML_STREAM_START_EVENT: yaml_event_type_e = 1;
pub const YAML_NO_EVENT: yaml_event_type_e = 0;
pub type yaml_event_type_t = yaml_event_type_e;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_sequence_style_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_t = yaml_event_s;
pub type yaml_node_type_e = u32;
pub const YAML_MAPPING_NODE: yaml_node_type_e = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_e = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_e = 1;
pub const YAML_NO_NODE: yaml_node_type_e = 0;
pub type yaml_node_type_t = yaml_node_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_s {
    pub type_0: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: C2RustUnnamed_16,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub scalar: C2RustUnnamed_21,
    pub sequence: C2RustUnnamed_19,
    pub mapping: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pairs: C2RustUnnamed_18,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
pub struct C2RustUnnamed_19 {
    pub items: C2RustUnnamed_20,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_23,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_22,
    pub start_implicit: i32,
    pub end_implicit: i32,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut u8,
    size_t,
    *mut size_t,
) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: i32,
    pub required: i32,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
pub type yaml_parser_state_e = u32;
pub const YAML_PARSE_END_STATE: yaml_parser_state_e = 23;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_e = 22;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_e = 21;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_e = 20;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 19;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_e = 18;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_e = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_e = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 14;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_e = 13;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_e = 12;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 11;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 10;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 9;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 8;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_e = 7;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_e = 6;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_e = 5;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_e = 4;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_e = 3;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_e = 2;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_e = 1;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_e = 0;
pub type yaml_parser_state_t = yaml_parser_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: i32,
    pub mark: yaml_mark_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
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
pub type yaml_parser_t = yaml_parser_s;
pub type yaml_write_handler_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut u8,
    size_t,
) -> i32;
pub type yaml_emitter_state_e = u32;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_e = 17;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_e = 16;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 15;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_e = 14;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 13;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 12;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 11;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_e = 10;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 9;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_e = 8;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 7;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 6;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 5;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_e = 4;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_e = 3;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_e = 2;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_e = 1;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_e = 0;
pub type yaml_emitter_state_t = yaml_emitter_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_anchors_s {
    pub references: i32,
    pub anchor: i32,
    pub serialized: i32,
}
pub type yaml_anchors_t = yaml_anchors_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_emitter_s {
    pub error: yaml_error_type_t,
    pub problem: *const i8,
    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut libc::c_void,
    pub output: C2RustUnnamed_44,
    pub buffer: C2RustUnnamed_43,
    pub raw_buffer: C2RustUnnamed_42,
    pub encoding: yaml_encoding_t,
    pub canonical: i32,
    pub best_indent: i32,
    pub best_width: i32,
    pub unicode: i32,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_41,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_40,
    pub indents: C2RustUnnamed_39,
    pub tag_directives: C2RustUnnamed_38,
    pub indent: i32,
    pub flow_level: i32,
    pub root_context: i32,
    pub sequence_context: i32,
    pub mapping_context: i32,
    pub simple_key_context: i32,
    pub line: i32,
    pub column: i32,
    pub whitespace: i32,
    pub indention: i32,
    pub open_ended: i32,
    pub anchor_data: C2RustUnnamed_37,
    pub tag_data: C2RustUnnamed_36,
    pub scalar_data: C2RustUnnamed_35,
    pub opened: i32,
    pub closed: i32,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: i32,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: i32,
    pub flow_plain_allowed: i32,
    pub block_plain_allowed: i32,
    pub single_quoted_allowed: i32,
    pub block_allowed: i32,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_37 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_38 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_39 {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_40 {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_41 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_42 {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_43 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_44 {
    pub string: C2RustUnnamed_45,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_45 {
    pub buffer: *mut u8,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
#[no_mangle]
pub unsafe extern "C" fn copy_document(
    mut document_to: *mut yaml_document_t,
    mut document_from: *mut yaml_document_t,
) -> i32 {
    let mut current_block: u64;
    let mut node: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    if yaml_document_initialize(
        document_to,
        (*document_from).version_directive,
        (*document_from).tag_directives.start,
        (*document_from).tag_directives.end,
        (*document_from).start_implicit,
        (*document_from).end_implicit,
    ) == 0
    {
        return 0 as i32;
    }
    node = (*document_from).nodes.start;
    loop {
        if !(node < (*document_from).nodes.top) {
            current_block = 11050875288958768710;
            break;
        }
        match (*node).type_0 as u32 {
            1 => {
                if yaml_document_add_scalar(
                    document_to,
                    (*node).tag,
                    (*node).data.scalar.value,
                    (*node).data.scalar.length as i32,
                    (*node).data.scalar.style,
                ) == 0
                {
                    current_block = 3747555925497496152;
                    break;
                }
            }
            2 => {
                if yaml_document_add_sequence(
                    document_to,
                    (*node).tag,
                    (*node).data.sequence.style,
                ) == 0
                {
                    current_block = 3747555925497496152;
                    break;
                }
            }
            3 => {
                if yaml_document_add_mapping(
                    document_to,
                    (*node).tag,
                    (*node).data.mapping.style,
                ) == 0
                {
                    current_block = 3747555925497496152;
                    break;
                }
            }
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    44 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 56],
                        &[i8; 56],
                    >(b"int copy_document(yaml_document_t *, yaml_document_t *)\0"))
                        .as_ptr(),
                );
                'c_3243: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const i8,
                        b"run-dumper.c\0" as *const u8 as *const i8,
                        44 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 56],
                            &[i8; 56],
                        >(b"int copy_document(yaml_document_t *, yaml_document_t *)\0"))
                            .as_ptr(),
                    );
                };
            }
        }
        node = node.offset(1);
        node;
    }
    match current_block {
        11050875288958768710 => {
            node = (*document_from).nodes.start;
            's_67: loop {
                if !(node < (*document_from).nodes.top) {
                    current_block = 18386322304582297246;
                    break;
                }
                match (*node).type_0 as u32 {
                    2 => {
                        item = (*node).data.sequence.items.start;
                        while item < (*node).data.sequence.items.top {
                            if yaml_document_append_sequence_item(
                                document_to,
                                (node.offset_from((*document_from).nodes.start) as i64
                                    + 1 as i32 as i64) as i32,
                                *item,
                            ) == 0
                            {
                                current_block = 3747555925497496152;
                                break 's_67;
                            }
                            item = item.offset(1);
                            item;
                        }
                    }
                    3 => {
                        pair = (*node).data.mapping.pairs.start;
                        while pair < (*node).data.mapping.pairs.top {
                            if yaml_document_append_mapping_pair(
                                document_to,
                                (node.offset_from((*document_from).nodes.start) as i64
                                    + 1 as i32 as i64) as i32,
                                (*pair).key,
                                (*pair).value,
                            ) == 0
                            {
                                current_block = 3747555925497496152;
                                break 's_67;
                            }
                            pair = pair.offset(1);
                            pair;
                        }
                    }
                    _ => {}
                }
                node = node.offset(1);
                node;
            }
            match current_block {
                3747555925497496152 => {}
                _ => return 1 as i32,
            }
        }
        _ => {}
    }
    yaml_document_delete(document_to);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn compare_nodes(
    mut document1: *mut yaml_document_t,
    mut index1: i32,
    mut document2: *mut yaml_document_t,
    mut index2: i32,
    mut level: i32,
) -> i32 {
    let mut k: i32 = 0;
    let mut node1: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let mut node2: *mut yaml_node_t = 0 as *mut yaml_node_t;
    let fresh0 = level;
    level = level + 1;
    if fresh0 > 1000 as i32 {
        return 0 as i32;
    }
    node1 = yaml_document_get_node(document1, index1);
    node2 = yaml_document_get_node(document2, index2);
    if !node1.is_null() {} else {
        __assert_fail(
            b"node1\0" as *const u8 as *const i8,
            b"run-dumper.c\0" as *const u8 as *const i8,
            89 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[i8; 71],
            >(
                b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3835: {
        if !node1.is_null() {} else {
            __assert_fail(
                b"node1\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                89 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[i8; 71],
                >(
                    b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !node2.is_null() {} else {
        __assert_fail(
            b"node2\0" as *const u8 as *const i8,
            b"run-dumper.c\0" as *const u8 as *const i8,
            90 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[i8; 71],
            >(
                b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3802: {
        if !node2.is_null() {} else {
            __assert_fail(
                b"node2\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                90 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[i8; 71],
                >(
                    b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*node1).type_0 as u32 != (*node2).type_0 as u32 {
        return 0 as i32;
    }
    if strcmp((*node1).tag as *mut i8, (*node2).tag as *mut i8) != 0 as i32 {
        return 0 as i32;
    }
    match (*node1).type_0 as u32 {
        1 => {
            if (*node1).data.scalar.length != (*node2).data.scalar.length {
                return 0 as i32;
            }
            if strncmp(
                (*node1).data.scalar.value as *mut i8,
                (*node2).data.scalar.value as *mut i8,
                (*node1).data.scalar.length,
            ) != 0 as i32
            {
                return 0 as i32;
            }
        }
        2 => {
            if ((*node1).data.sequence.items.top)
                .offset_from((*node1).data.sequence.items.start) as i64
                != ((*node2).data.sequence.items.top)
                    .offset_from((*node2).data.sequence.items.start) as i64
            {
                return 0 as i32;
            }
            k = 0 as i32;
            while (k as i64)
                < ((*node1).data.sequence.items.top)
                    .offset_from((*node1).data.sequence.items.start) as i64
            {
                if compare_nodes(
                    document1,
                    *((*node1).data.sequence.items.start).offset(k as isize),
                    document2,
                    *((*node2).data.sequence.items.start).offset(k as isize),
                    level,
                ) == 0
                {
                    return 0 as i32;
                }
                k += 1;
                k;
            }
        }
        3 => {
            if ((*node1).data.mapping.pairs.top)
                .offset_from((*node1).data.mapping.pairs.start) as i64
                != ((*node2).data.mapping.pairs.top)
                    .offset_from((*node2).data.mapping.pairs.start) as i64
            {
                return 0 as i32;
            }
            k = 0 as i32;
            while (k as i64)
                < ((*node1).data.mapping.pairs.top)
                    .offset_from((*node1).data.mapping.pairs.start) as i64
            {
                if compare_nodes(
                    document1,
                    (*((*node1).data.mapping.pairs.start).offset(k as isize)).key,
                    document2,
                    (*((*node2).data.mapping.pairs.start).offset(k as isize)).key,
                    level,
                ) == 0
                {
                    return 0 as i32;
                }
                if compare_nodes(
                    document1,
                    (*((*node1).data.mapping.pairs.start).offset(k as isize)).value,
                    document2,
                    (*((*node2).data.mapping.pairs.start).offset(k as isize)).value,
                    level,
                ) == 0
                {
                    return 0 as i32;
                }
                k += 1;
                k;
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                125 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[i8; 71],
                >(
                    b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
                ))
                    .as_ptr(),
            );
            'c_3443: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    125 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 71],
                        &[i8; 71],
                    >(
                        b"int compare_nodes(yaml_document_t *, int, yaml_document_t *, int, int)\0",
                    ))
                        .as_ptr(),
                );
            };
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn compare_documents(
    mut document1: *mut yaml_document_t,
    mut document2: *mut yaml_document_t,
) -> i32 {
    let mut k: i32 = 0;
    if !((*document1).version_directive).is_null()
        && ((*document2).version_directive).is_null()
        || ((*document1).version_directive).is_null()
            && !((*document2).version_directive).is_null()
        || !((*document1).version_directive).is_null()
            && !((*document2).version_directive).is_null()
            && ((*(*document1).version_directive).major
                != (*(*document2).version_directive).major
                || (*(*document1).version_directive).minor
                    != (*(*document2).version_directive).minor)
    {
        return 0 as i32;
    }
    if ((*document1).tag_directives.end).offset_from((*document1).tag_directives.start)
        as i64
        != ((*document2).tag_directives.end)
            .offset_from((*document2).tag_directives.start) as i64
    {
        return 0 as i32;
    }
    k = 0 as i32;
    while (k as i64)
        < ((*document1).tag_directives.end)
            .offset_from((*document1).tag_directives.start) as i64
    {
        if strcmp(
            (*((*document1).tag_directives.start).offset(k as isize)).handle as *mut i8,
            (*((*document2).tag_directives.start).offset(k as isize)).handle as *mut i8,
        ) != 0 as i32
            || strcmp(
                (*((*document1).tag_directives.start).offset(k as isize)).prefix
                    as *mut i8,
                (*((*document2).tag_directives.start).offset(k as isize)).prefix
                    as *mut i8,
            ) != 0 as i32
        {
            return 0 as i32;
        }
        k += 1;
        k;
    }
    if ((*document1).nodes.top).offset_from((*document1).nodes.start) as i64
        != ((*document2).nodes.top).offset_from((*document2).nodes.start) as i64
    {
        return 0 as i32;
    }
    if (*document1).nodes.top != (*document1).nodes.start {
        if compare_nodes(document1, 1 as i32, document2, 1 as i32, 0 as i32) == 0 {
            return 0 as i32;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn print_output(
    mut name: *mut i8,
    mut buffer: *mut u8,
    mut size: size_t,
    mut count: i32,
) -> i32 {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut data: [i8; 65536] = [0; 65536];
    let mut data_size: size_t = 1 as i32 as size_t;
    let mut total_size: size_t = 0 as i32 as size_t;
    if count >= 0 as i32 {
        printf(
            b"FAILED (at the document #%d)\nSOURCE:\n\0" as *const u8 as *const i8,
            count + 1 as i32,
        );
    }
    file = fopen(name, b"rb\0" as *const u8 as *const i8);
    if !file.is_null() {} else {
        __assert_fail(
            b"file\0" as *const u8 as *const i8,
            b"run-dumper.c\0" as *const u8 as *const i8,
            175 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                .as_ptr(),
        );
    }
    'c_4337: {
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                175 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                    .as_ptr(),
            );
        }
    };
    while data_size > 0 as i32 as u64 {
        data_size = fread(
            data.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            65536 as i32 as size_t,
            file,
        );
        if ferror(file) == 0 {} else {
            __assert_fail(
                b"!ferror(file)\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                178 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                    .as_ptr(),
            );
        }
        'c_4276: {
            if ferror(file) == 0 {} else {
                __assert_fail(
                    b"!ferror(file)\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    178 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                        .as_ptr(),
                );
            }
        };
        if data_size == 0 {
            break;
        }
        if fwrite(
            data.as_mut_ptr() as *const libc::c_void,
            1 as i32 as size_t,
            data_size,
            stdout,
        ) == data_size
        {} else {
            __assert_fail(
                b"fwrite(data, 1, data_size, stdout) == data_size\0" as *const u8
                    as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                180 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                    .as_ptr(),
            );
        }
        'c_4209: {
            if fwrite(
                data.as_mut_ptr() as *const libc::c_void,
                1 as i32 as size_t,
                data_size,
                stdout,
            ) == data_size
            {} else {
                __assert_fail(
                    b"fwrite(data, 1, data_size, stdout) == data_size\0" as *const u8
                        as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    180 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"int print_output(char *, unsigned char *, size_t, int)\0"))
                        .as_ptr(),
                );
            }
        };
        total_size = (total_size as u64).wrapping_add(data_size) as size_t as size_t;
        if feof(file) != 0 {
            break;
        }
    }
    fclose(file);
    printf(b"#### (length: %ld)\n\0" as *const u8 as *const i8, total_size as i64);
    printf(
        b"OUTPUT:\n%s#### (length: %ld)\n\0" as *const u8 as *const i8,
        buffer,
        size as i64,
    );
    return 0 as i32;
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut number: i32 = 0;
    let mut canonical: i32 = 0 as i32;
    let mut unicode: i32 = 0 as i32;
    number = 1 as i32;
    while number < argc {
        if strcmp(*argv.offset(number as isize), b"-c\0" as *const u8 as *const i8)
            == 0 as i32
        {
            canonical = 1 as i32;
        } else if strcmp(
            *argv.offset(number as isize),
            b"-u\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            unicode = 1 as i32;
        } else if *(*argv.offset(number as isize)).offset(0 as i32 as isize) as i32
            == '-' as i32
        {
            printf(
                b"Unknown option: '%s'\n\0" as *const u8 as *const i8,
                *argv.offset(number as isize),
            );
            return 0 as i32;
        }
        if *(*argv.offset(number as isize)).offset(0 as i32 as isize) as i32
            == '-' as i32
        {
            if number < argc - 1 as i32 {
                memmove(
                    argv.offset(number as isize) as *mut libc::c_void,
                    argv.offset(number as isize).offset(1 as i32 as isize)
                        as *const libc::c_void,
                    ((argc - number - 1 as i32) as u64)
                        .wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                );
            }
            argc -= 1;
            argc;
        } else {
            number += 1;
            number;
        }
    }
    if argc < 2 as i32 {
        printf(
            b"Usage: %s [-c] [-u] file1.yaml ...\n\0" as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
        );
        return 0 as i32;
    }
    number = 1 as i32;
    while number < argc {
        let mut file: *mut FILE = 0 as *mut FILE;
        let mut parser: yaml_parser_t = yaml_parser_t {
            error: YAML_NO_ERROR,
            problem: 0 as *const i8,
            problem_offset: 0,
            problem_value: 0,
            problem_mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
            context: 0 as *const i8,
            context_mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
            read_handler: None,
            read_handler_data: 0 as *mut libc::c_void,
            input: C2RustUnnamed_33 {
                string: C2RustUnnamed_34 {
                    start: 0 as *const u8,
                    end: 0 as *const u8,
                    current: 0 as *const u8,
                },
            },
            eof: 0,
            buffer: C2RustUnnamed_32 {
                start: 0 as *mut yaml_char_t,
                end: 0 as *mut yaml_char_t,
                pointer: 0 as *mut yaml_char_t,
                last: 0 as *mut yaml_char_t,
            },
            unread: 0,
            raw_buffer: C2RustUnnamed_31 {
                start: 0 as *mut u8,
                end: 0 as *mut u8,
                pointer: 0 as *mut u8,
                last: 0 as *mut u8,
            },
            encoding: YAML_ANY_ENCODING,
            offset: 0,
            mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
            stream_start_produced: 0,
            stream_end_produced: 0,
            flow_level: 0,
            tokens: C2RustUnnamed_30 {
                start: 0 as *mut yaml_token_t,
                end: 0 as *mut yaml_token_t,
                head: 0 as *mut yaml_token_t,
                tail: 0 as *mut yaml_token_t,
            },
            tokens_parsed: 0,
            token_available: 0,
            indents: C2RustUnnamed_29 {
                start: 0 as *mut i32,
                end: 0 as *mut i32,
                top: 0 as *mut i32,
            },
            indent: 0,
            simple_key_allowed: 0,
            simple_keys: C2RustUnnamed_28 {
                start: 0 as *mut yaml_simple_key_t,
                end: 0 as *mut yaml_simple_key_t,
                top: 0 as *mut yaml_simple_key_t,
            },
            states: C2RustUnnamed_27 {
                start: 0 as *mut yaml_parser_state_t,
                end: 0 as *mut yaml_parser_state_t,
                top: 0 as *mut yaml_parser_state_t,
            },
            state: YAML_PARSE_STREAM_START_STATE,
            marks: C2RustUnnamed_26 {
                start: 0 as *mut yaml_mark_t,
                end: 0 as *mut yaml_mark_t,
                top: 0 as *mut yaml_mark_t,
            },
            tag_directives: C2RustUnnamed_25 {
                start: 0 as *mut yaml_tag_directive_t,
                end: 0 as *mut yaml_tag_directive_t,
                top: 0 as *mut yaml_tag_directive_t,
            },
            aliases: C2RustUnnamed_24 {
                start: 0 as *mut yaml_alias_data_t,
                end: 0 as *mut yaml_alias_data_t,
                top: 0 as *mut yaml_alias_data_t,
            },
            document: 0 as *mut yaml_document_t,
        };
        let mut emitter: yaml_emitter_t = yaml_emitter_t {
            error: YAML_NO_ERROR,
            problem: 0 as *const i8,
            write_handler: None,
            write_handler_data: 0 as *mut libc::c_void,
            output: C2RustUnnamed_44 {
                string: C2RustUnnamed_45 {
                    buffer: 0 as *mut u8,
                    size: 0,
                    size_written: 0 as *mut size_t,
                },
            },
            buffer: C2RustUnnamed_43 {
                start: 0 as *mut yaml_char_t,
                end: 0 as *mut yaml_char_t,
                pointer: 0 as *mut yaml_char_t,
                last: 0 as *mut yaml_char_t,
            },
            raw_buffer: C2RustUnnamed_42 {
                start: 0 as *mut u8,
                end: 0 as *mut u8,
                pointer: 0 as *mut u8,
                last: 0 as *mut u8,
            },
            encoding: YAML_ANY_ENCODING,
            canonical: 0,
            best_indent: 0,
            best_width: 0,
            unicode: 0,
            line_break: YAML_ANY_BREAK,
            states: C2RustUnnamed_41 {
                start: 0 as *mut yaml_emitter_state_t,
                end: 0 as *mut yaml_emitter_state_t,
                top: 0 as *mut yaml_emitter_state_t,
            },
            state: YAML_EMIT_STREAM_START_STATE,
            events: C2RustUnnamed_40 {
                start: 0 as *mut yaml_event_t,
                end: 0 as *mut yaml_event_t,
                head: 0 as *mut yaml_event_t,
                tail: 0 as *mut yaml_event_t,
            },
            indents: C2RustUnnamed_39 {
                start: 0 as *mut i32,
                end: 0 as *mut i32,
                top: 0 as *mut i32,
            },
            tag_directives: C2RustUnnamed_38 {
                start: 0 as *mut yaml_tag_directive_t,
                end: 0 as *mut yaml_tag_directive_t,
                top: 0 as *mut yaml_tag_directive_t,
            },
            indent: 0,
            flow_level: 0,
            root_context: 0,
            sequence_context: 0,
            mapping_context: 0,
            simple_key_context: 0,
            line: 0,
            column: 0,
            whitespace: 0,
            indention: 0,
            open_ended: 0,
            anchor_data: C2RustUnnamed_37 {
                anchor: 0 as *mut yaml_char_t,
                anchor_length: 0,
                alias: 0,
            },
            tag_data: C2RustUnnamed_36 {
                handle: 0 as *mut yaml_char_t,
                handle_length: 0,
                suffix: 0 as *mut yaml_char_t,
                suffix_length: 0,
            },
            scalar_data: C2RustUnnamed_35 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                multiline: 0,
                flow_plain_allowed: 0,
                block_plain_allowed: 0,
                single_quoted_allowed: 0,
                block_allowed: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
            opened: 0,
            closed: 0,
            anchors: 0 as *mut yaml_anchors_t,
            last_anchor_id: 0,
            document: 0 as *mut yaml_document_t,
        };
        let mut document: yaml_document_t = yaml_document_t {
            nodes: C2RustUnnamed_23 {
                start: 0 as *mut yaml_node_t,
                end: 0 as *mut yaml_node_t,
                top: 0 as *mut yaml_node_t,
            },
            version_directive: 0 as *mut yaml_version_directive_t,
            tag_directives: C2RustUnnamed_22 {
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
        let mut buffer: [u8; 65537] = [0; 65537];
        let mut written: size_t = 0 as i32 as size_t;
        let mut documents: [yaml_document_t; 16] = [yaml_document_t {
            nodes: C2RustUnnamed_23 {
                start: 0 as *mut yaml_node_t,
                end: 0 as *mut yaml_node_t,
                top: 0 as *mut yaml_node_t,
            },
            version_directive: 0 as *mut yaml_version_directive_t,
            tag_directives: C2RustUnnamed_22 {
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
        }; 16];
        let mut document_number: size_t = 0 as i32 as size_t;
        let mut done: i32 = 0 as i32;
        let mut count: i32 = 0 as i32;
        let mut error: i32 = 0 as i32;
        let mut k: i32 = 0;
        memset(
            buffer.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (65536 as i32 + 1 as i32) as u64,
        );
        memset(
            documents.as_mut_ptr() as *mut libc::c_void,
            0 as i32,
            (16 as i32 as u64)
                .wrapping_mul(::core::mem::size_of::<yaml_document_t>() as u64),
        );
        printf(
            b"[%d] Loading, dumping, and loading again '%s': \0" as *const u8
                as *const i8,
            number,
            *argv.offset(number as isize),
        );
        fflush(stdout);
        file = fopen(*argv.offset(number as isize), b"rb\0" as *const u8 as *const i8);
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                247 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_5193: {
            if !file.is_null() {} else {
                __assert_fail(
                    b"file\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    247 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        if yaml_parser_initialize(&mut parser) != 0 {} else {
            __assert_fail(
                b"yaml_parser_initialize(&parser)\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                249 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_5155: {
            if yaml_parser_initialize(&mut parser) != 0 {} else {
                __assert_fail(
                    b"yaml_parser_initialize(&parser)\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    249 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        yaml_parser_set_input_file(&mut parser, file);
        if yaml_emitter_initialize(&mut emitter) != 0 {} else {
            __assert_fail(
                b"yaml_emitter_initialize(&emitter)\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                251 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_5108: {
            if yaml_emitter_initialize(&mut emitter) != 0 {} else {
                __assert_fail(
                    b"yaml_emitter_initialize(&emitter)\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    251 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        if canonical != 0 {
            yaml_emitter_set_canonical(&mut emitter, 1 as i32);
        }
        if unicode != 0 {
            yaml_emitter_set_unicode(&mut emitter, 1 as i32);
        }
        yaml_emitter_set_output_string(
            &mut emitter,
            buffer.as_mut_ptr(),
            65536 as i32 as size_t,
            &mut written,
        );
        yaml_emitter_open(&mut emitter);
        while done == 0 {
            if yaml_parser_load(&mut parser, &mut document) == 0 {
                error = 1 as i32;
                break;
            } else {
                done = (yaml_document_get_root_node(&mut document)).is_null() as i32;
                if done == 0 {
                    if document_number < 16 as i32 as u64 {} else {
                        __assert_fail(
                            b"document_number < MAX_DOCUMENTS\0" as *const u8
                                as *const i8,
                            b"run-dumper.c\0" as *const u8 as *const i8,
                            270 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[i8; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_5001: {
                        if document_number < 16 as i32 as u64 {} else {
                            __assert_fail(
                                b"document_number < MAX_DOCUMENTS\0" as *const u8
                                    as *const i8,
                                b"run-dumper.c\0" as *const u8 as *const i8,
                                270 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 23],
                                    &[i8; 23],
                                >(b"int main(int, char **)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    let fresh1 = document_number;
                    document_number = document_number.wrapping_add(1);
                    if copy_document(
                        &mut *documents.as_mut_ptr().offset(fresh1 as isize),
                        &mut document,
                    ) != 0
                    {} else {
                        __assert_fail(
                            b"copy_document(&(documents[document_number++]), &document)\0"
                                as *const u8 as *const i8,
                            b"run-dumper.c\0" as *const u8 as *const i8,
                            271 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[i8; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_4948: {
                        let fresh1 = document_number;
                        document_number = document_number.wrapping_add(1);
                        if copy_document(
                            &mut *documents.as_mut_ptr().offset(fresh1 as isize),
                            &mut document,
                        ) != 0
                        {} else {
                            __assert_fail(
                                b"copy_document(&(documents[document_number++]), &document)\0"
                                    as *const u8 as *const i8,
                                b"run-dumper.c\0" as *const u8 as *const i8,
                                271 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 23],
                                    &[i8; 23],
                                >(b"int main(int, char **)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if yaml_emitter_dump(&mut emitter, &mut document) != 0
                        || yaml_emitter_flush(&mut emitter) != 0
                            && print_output(
                                *argv.offset(number as isize),
                                buffer.as_mut_ptr(),
                                written,
                                count,
                            ) != 0
                    {} else {
                        __assert_fail(
                            b"yaml_emitter_dump(&emitter, &document) || (yaml_emitter_flush(&emitter) && print_output(argv[number], buffer, written, count))\0"
                                as *const u8 as *const i8,
                            b"run-dumper.c\0" as *const u8 as *const i8,
                            273 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[i8; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_4858: {
                        if yaml_emitter_dump(&mut emitter, &mut document) != 0
                            || yaml_emitter_flush(&mut emitter) != 0
                                && print_output(
                                    *argv.offset(number as isize),
                                    buffer.as_mut_ptr(),
                                    written,
                                    count,
                                ) != 0
                        {} else {
                            __assert_fail(
                                b"yaml_emitter_dump(&emitter, &document) || (yaml_emitter_flush(&emitter) && print_output(argv[number], buffer, written, count))\0"
                                    as *const u8 as *const i8,
                                b"run-dumper.c\0" as *const u8 as *const i8,
                                273 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 23],
                                    &[i8; 23],
                                >(b"int main(int, char **)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    count += 1;
                    count;
                } else {
                    yaml_document_delete(&mut document);
                }
            }
        }
        yaml_parser_delete(&mut parser);
        if fclose(file) == 0 {} else {
            __assert_fail(
                b"!fclose(file)\0" as *const u8 as *const i8,
                b"run-dumper.c\0" as *const u8 as *const i8,
                282 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_4792: {
            if fclose(file) == 0 {} else {
                __assert_fail(
                    b"!fclose(file)\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    282 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        yaml_emitter_close(&mut emitter);
        yaml_emitter_delete(&mut emitter);
        if error == 0 {
            done = 0 as i32;
            count = done;
            if yaml_parser_initialize(&mut parser) != 0 {} else {
                __assert_fail(
                    b"yaml_parser_initialize(&parser)\0" as *const u8 as *const i8,
                    b"run-dumper.c\0" as *const u8 as *const i8,
                    289 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_4730: {
                if yaml_parser_initialize(&mut parser) != 0 {} else {
                    __assert_fail(
                        b"yaml_parser_initialize(&parser)\0" as *const u8 as *const i8,
                        b"run-dumper.c\0" as *const u8 as *const i8,
                        289 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[i8; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            yaml_parser_set_input_string(&mut parser, buffer.as_mut_ptr(), written);
            while done == 0 {
                if yaml_parser_load(&mut parser, &mut document) != 0
                    || print_output(
                        *argv.offset(number as isize),
                        buffer.as_mut_ptr(),
                        written,
                        count,
                    ) != 0
                {} else {
                    __assert_fail(
                        b"yaml_parser_load(&parser, &document) || print_output(argv[number], buffer, written, count)\0"
                            as *const u8 as *const i8,
                        b"run-dumper.c\0" as *const u8 as *const i8,
                        294 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[i8; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
                'c_4641: {
                    if yaml_parser_load(&mut parser, &mut document) != 0
                        || print_output(
                            *argv.offset(number as isize),
                            buffer.as_mut_ptr(),
                            written,
                            count,
                        ) != 0
                    {} else {
                        __assert_fail(
                            b"yaml_parser_load(&parser, &document) || print_output(argv[number], buffer, written, count)\0"
                                as *const u8 as *const i8,
                            b"run-dumper.c\0" as *const u8 as *const i8,
                            294 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[i8; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                };
                done = (yaml_document_get_root_node(&mut document)).is_null() as i32;
                if done == 0 {
                    if compare_documents(
                        documents.as_mut_ptr().offset(count as isize),
                        &mut document,
                    ) != 0
                        || print_output(
                            *argv.offset(number as isize),
                            buffer.as_mut_ptr(),
                            written,
                            count,
                        ) != 0
                    {} else {
                        __assert_fail(
                            b"compare_documents(documents+count, &document) || print_output(argv[number], buffer, written, count)\0"
                                as *const u8 as *const i8,
                            b"run-dumper.c\0" as *const u8 as *const i8,
                            297 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 23],
                                &[i8; 23],
                            >(b"int main(int, char **)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_4544: {
                        if compare_documents(
                            documents.as_mut_ptr().offset(count as isize),
                            &mut document,
                        ) != 0
                            || print_output(
                                *argv.offset(number as isize),
                                buffer.as_mut_ptr(),
                                written,
                                count,
                            ) != 0
                        {} else {
                            __assert_fail(
                                b"compare_documents(documents+count, &document) || print_output(argv[number], buffer, written, count)\0"
                                    as *const u8 as *const i8,
                                b"run-dumper.c\0" as *const u8 as *const i8,
                                297 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 23],
                                    &[i8; 23],
                                >(b"int main(int, char **)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    count += 1;
                    count;
                }
                yaml_document_delete(&mut document);
            }
            yaml_parser_delete(&mut parser);
        }
        k = 0 as i32;
        while (k as u64) < document_number {
            yaml_document_delete(documents.as_mut_ptr().offset(k as isize));
            k += 1;
            k;
        }
        printf(b"PASSED (length: %ld)\n\0" as *const u8 as *const i8, written as i64);
        print_output(
            *argv.offset(number as isize),
            buffer.as_mut_ptr(),
            written,
            -(1 as i32),
        );
        number += 1;
        number;
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