#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32;
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: i32);
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: i32);
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut FILE);
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> i32;
    fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> i32;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32;
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn yaml_stream_start_event_initialize(
        event: *mut yaml_event_t,
        encoding: yaml_encoding_t,
    ) -> i32;
    fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> i32;
    fn yaml_document_start_event_initialize(
        event: *mut yaml_event_t,
        version_directive: *mut yaml_version_directive_t,
        tag_directives_start: *mut yaml_tag_directive_t,
        tag_directives_end: *mut yaml_tag_directive_t,
        implicit: i32,
    ) -> i32;
    fn yaml_document_end_event_initialize(
        event: *mut yaml_event_t,
        implicit: i32,
    ) -> i32;
    fn yaml_scalar_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        value: *const yaml_char_t,
        length: i32,
        plain_implicit: i32,
        quoted_implicit: i32,
        style: yaml_scalar_style_t,
    ) -> i32;
    fn yaml_sequence_start_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        implicit: i32,
        style: yaml_sequence_style_t,
    ) -> i32;
    fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> i32;
    fn yaml_mapping_start_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
        tag: *const yaml_char_t,
        implicit: i32,
        style: yaml_mapping_style_t,
    ) -> i32;
    fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> i32;
    fn yaml_event_delete(event: *mut yaml_event_t);
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
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut current_block: u64;
    let mut help: i32 = 0 as i32;
    let mut canonical: i32 = 0 as i32;
    let mut unicode: i32 = 0 as i32;
    let mut k: i32 = 0;
    let mut done: i32 = 0 as i32;
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
    let mut input_event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed_7 {
            stream_start: C2RustUnnamed_15 {
                encoding: YAML_ANY_ENCODING,
            },
        },
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
    let mut output_event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed_7 {
            stream_start: C2RustUnnamed_15 {
                encoding: YAML_ANY_ENCODING,
            },
        },
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
    memset(
        &mut parser as *mut yaml_parser_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_parser_t>() as u64,
    );
    memset(
        &mut emitter as *mut yaml_emitter_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_emitter_t>() as u64,
    );
    memset(
        &mut input_event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    memset(
        &mut output_event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    k = 1 as i32;
    while k < argc {
        if strcmp(*argv.offset(k as isize), b"-h\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(*argv.offset(k as isize), b"--help\0" as *const u8 as *const i8)
                == 0 as i32
        {
            help = 1 as i32;
        } else if strcmp(*argv.offset(k as isize), b"-c\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(
                *argv.offset(k as isize),
                b"--canonical\0" as *const u8 as *const i8,
            ) == 0 as i32
        {
            canonical = 1 as i32;
        } else if strcmp(*argv.offset(k as isize), b"-u\0" as *const u8 as *const i8)
            == 0 as i32
            || strcmp(*argv.offset(k as isize), b"--unicode\0" as *const u8 as *const i8)
                == 0 as i32
        {
            unicode = 1 as i32;
        } else {
            fprintf(
                stderr,
                b"Unrecognized option: %s\nTry `%s --help` for more information.\n\0"
                    as *const u8 as *const i8,
                *argv.offset(k as isize),
                *argv.offset(0 as i32 as isize),
            );
            return 1 as i32;
        }
        k += 1;
        k;
    }
    if help != 0 {
        printf(
            b"%s <input\nor\n%s -h | --help\nDeconstruct a YAML stream\n\nOptions:\n-h, --help\t\tdisplay this help and exit\n-c, --canonical\t\toutput in the canonical YAML format\n-u, --unicode\t\toutput unescaped non-ASCII characters\n\0"
                as *const u8 as *const i8,
            *argv.offset(0 as i32 as isize),
            *argv.offset(0 as i32 as isize),
        );
        return 0 as i32;
    }
    if yaml_parser_initialize(&mut parser) == 0 {
        fprintf(
            stderr,
            b"Could not initialize the parser object\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    if yaml_emitter_initialize(&mut emitter) == 0 {
        yaml_parser_delete(&mut parser);
        fprintf(
            stderr,
            b"Could not inialize the emitter object\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    yaml_parser_set_input_file(&mut parser, stdin);
    yaml_emitter_set_output_file(&mut emitter, stdout);
    yaml_emitter_set_canonical(&mut emitter, canonical);
    yaml_emitter_set_unicode(&mut emitter, unicode);
    if !(yaml_stream_start_event_initialize(&mut output_event, YAML_UTF8_ENCODING) == 0)
    {
        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
            current_block = 8961051500484970212;
        } else if yaml_document_start_event_initialize(
            &mut output_event,
            0 as *mut yaml_version_directive_t,
            0 as *mut yaml_tag_directive_t,
            0 as *mut yaml_tag_directive_t,
            0 as i32,
        ) == 0
        {
            current_block = 16336806485327817624;
        } else if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
            current_block = 8961051500484970212;
        } else if yaml_sequence_start_event_initialize(
            &mut output_event,
            0 as *const yaml_char_t,
            b"tag:yaml.org,2002:seq\0" as *const u8 as *const i8 as *mut yaml_char_t,
            1 as i32,
            YAML_BLOCK_SEQUENCE_STYLE,
        ) == 0
        {
            current_block = 16336806485327817624;
        } else if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
            current_block = 8961051500484970212;
        } else {
            's_177: loop {
                if !(done == 0) {
                    current_block = 16517180880614114163;
                    break;
                }
                if yaml_parser_parse(&mut parser, &mut input_event) == 0 {
                    current_block = 8688973151516351212;
                    break;
                }
                if input_event.type_0 as u32 == YAML_STREAM_END_EVENT as i32 as u32 {
                    done = 1 as i32;
                }
                if yaml_mapping_start_event_initialize(
                    &mut output_event,
                    0 as *const yaml_char_t,
                    b"tag:yaml.org,2002:map\0" as *const u8 as *const i8
                        as *mut yaml_char_t,
                    1 as i32,
                    YAML_BLOCK_MAPPING_STYLE,
                ) == 0
                {
                    current_block = 16336806485327817624;
                    break;
                }
                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                    current_block = 8961051500484970212;
                    break;
                }
                match input_event.type_0 as u32 {
                    1 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"STREAM-START\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if input_event.data.stream_start.encoding as u64 != 0 {
                            let mut encoding: yaml_encoding_t = input_event
                                .data
                                .stream_start
                                .encoding;
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"encoding\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                (if encoding as u32 == YAML_UTF8_ENCODING as i32 as u32 {
                                    b"utf-8\0" as *const u8 as *const i8
                                } else if encoding as u32
                                    == YAML_UTF16LE_ENCODING as i32 as u32
                                {
                                    b"utf-16-le\0" as *const u8 as *const i8
                                } else if encoding as u32
                                    == YAML_UTF16BE_ENCODING as i32 as u32
                                {
                                    b"utf-16-be\0" as *const u8 as *const i8
                                } else {
                                    b"unknown\0" as *const u8 as *const i8
                                }) as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                    }
                    2 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"STREAM-END\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    3 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"DOCUMENT-START\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if !(input_event.data.document_start.version_directive).is_null()
                        {
                            let mut version: *mut yaml_version_directive_t = input_event
                                .data
                                .document_start
                                .version_directive;
                            let mut number: [i8; 64] = [0; 64];
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"version\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_mapping_start_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:map\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                1 as i32,
                                YAML_FLOW_MAPPING_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"major\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            sprintf(
                                number.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const i8,
                                (*version).major,
                            );
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:int\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                number.as_mut_ptr() as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"minor\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            sprintf(
                                number.as_mut_ptr(),
                                b"%d\0" as *const u8 as *const i8,
                                (*version).minor,
                            );
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:int\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                number.as_mut_ptr() as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_mapping_end_event_initialize(&mut output_event) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if input_event.data.document_start.tag_directives.start
                            != input_event.data.document_start.tag_directives.end
                        {
                            let mut tag: *mut yaml_tag_directive_t = 0
                                as *mut yaml_tag_directive_t;
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"tags\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_sequence_start_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:seq\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                1 as i32,
                                YAML_BLOCK_SEQUENCE_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            tag = input_event.data.document_start.tag_directives.start;
                            while tag
                                != input_event.data.document_start.tag_directives.end
                            {
                                if yaml_mapping_start_event_initialize(
                                    &mut output_event,
                                    0 as *const yaml_char_t,
                                    b"tag:yaml.org,2002:map\0" as *const u8 as *const i8
                                        as *mut yaml_char_t,
                                    1 as i32,
                                    YAML_FLOW_MAPPING_STYLE,
                                ) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                if yaml_scalar_event_initialize(
                                    &mut output_event,
                                    0 as *const yaml_char_t,
                                    b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                        as *mut yaml_char_t,
                                    b"handle\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                    -(1 as i32),
                                    1 as i32,
                                    1 as i32,
                                    YAML_PLAIN_SCALAR_STYLE,
                                ) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                if yaml_scalar_event_initialize(
                                    &mut output_event,
                                    0 as *const yaml_char_t,
                                    b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                        as *mut yaml_char_t,
                                    (*tag).handle,
                                    -(1 as i32),
                                    0 as i32,
                                    1 as i32,
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                ) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                if yaml_scalar_event_initialize(
                                    &mut output_event,
                                    0 as *const yaml_char_t,
                                    b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                        as *mut yaml_char_t,
                                    b"prefix\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                    -(1 as i32),
                                    1 as i32,
                                    1 as i32,
                                    YAML_PLAIN_SCALAR_STYLE,
                                ) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                if yaml_scalar_event_initialize(
                                    &mut output_event,
                                    0 as *const yaml_char_t,
                                    b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                        as *mut yaml_char_t,
                                    (*tag).prefix,
                                    -(1 as i32),
                                    0 as i32,
                                    1 as i32,
                                    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                                ) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                if yaml_mapping_end_event_initialize(&mut output_event) == 0
                                {
                                    current_block = 16336806485327817624;
                                    break 's_177;
                                }
                                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                    current_block = 8961051500484970212;
                                    break 's_177;
                                }
                                tag = tag.offset(1);
                                tag;
                            }
                            if yaml_sequence_end_event_initialize(&mut output_event) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"implicit\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.document_start.implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    4 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"DOCUMENT-END\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"implicit\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.document_end.implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    5 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"ALIAS\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"anchor\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            input_event.data.alias.anchor,
                            -(1 as i32),
                            0 as i32,
                            1 as i32,
                            YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    6 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"SCALAR\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if !(input_event.data.scalar.anchor).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"anchor\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.scalar.anchor,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if !(input_event.data.scalar.tag).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"tag\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.scalar.tag,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"value\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            input_event.data.scalar.value,
                            input_event.data.scalar.length as i32,
                            0 as i32,
                            1 as i32,
                            YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"implicit\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_mapping_start_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:map\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            1 as i32,
                            YAML_FLOW_MAPPING_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"plain\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.scalar.plain_implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"non-plain\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.scalar.quoted_implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_mapping_end_event_initialize(&mut output_event) == 0 {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if input_event.data.scalar.style as u64 != 0 {
                            let mut style: yaml_scalar_style_t = input_event
                                .data
                                .scalar
                                .style;
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"style\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                (if style as u32 == YAML_PLAIN_SCALAR_STYLE as i32 as u32 {
                                    b"plain\0" as *const u8 as *const i8
                                } else if style as u32
                                    == YAML_SINGLE_QUOTED_SCALAR_STYLE as i32 as u32
                                {
                                    b"single-quoted\0" as *const u8 as *const i8
                                } else if style as u32
                                    == YAML_DOUBLE_QUOTED_SCALAR_STYLE as i32 as u32
                                {
                                    b"double-quoted\0" as *const u8 as *const i8
                                } else if style as u32
                                    == YAML_LITERAL_SCALAR_STYLE as i32 as u32
                                {
                                    b"literal\0" as *const u8 as *const i8
                                } else if style as u32
                                    == YAML_FOLDED_SCALAR_STYLE as i32 as u32
                                {
                                    b"folded\0" as *const u8 as *const i8
                                } else {
                                    b"unknown\0" as *const u8 as *const i8
                                }) as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                    }
                    7 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"SEQUENCE-START\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if !(input_event.data.sequence_start.anchor).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"anchor\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.sequence_start.anchor,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if !(input_event.data.sequence_start.tag).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"tag\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.sequence_start.tag,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"implicit\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.sequence_start.implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if input_event.data.sequence_start.style as u64 != 0 {
                            let mut style_0: yaml_sequence_style_t = input_event
                                .data
                                .sequence_start
                                .style;
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"style\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                (if style_0 as u32
                                    == YAML_BLOCK_SEQUENCE_STYLE as i32 as u32
                                {
                                    b"block\0" as *const u8 as *const i8
                                } else if style_0 as u32
                                    == YAML_FLOW_SEQUENCE_STYLE as i32 as u32
                                {
                                    b"flow\0" as *const u8 as *const i8
                                } else {
                                    b"unknown\0" as *const u8 as *const i8
                                }) as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                    }
                    8 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"SEQUENCE-END\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    9 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"MAPPING-START\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if !(input_event.data.mapping_start.anchor).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"anchor\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.mapping_start.anchor,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if !(input_event.data.mapping_start.tag).is_null() {
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"tag\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                input_event.data.mapping_start.tag,
                                -(1 as i32),
                                0 as i32,
                                1 as i32,
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"implicit\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:bool\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            (if input_event.data.mapping_start.implicit != 0 {
                                b"true\0" as *const u8 as *const i8
                            } else {
                                b"false\0" as *const u8 as *const i8
                            }) as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            0 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if input_event.data.mapping_start.style as u64 != 0 {
                            let mut style_1: yaml_mapping_style_t = input_event
                                .data
                                .mapping_start
                                .style;
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                b"style\0" as *const u8 as *const i8 as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                            if yaml_scalar_event_initialize(
                                &mut output_event,
                                0 as *const yaml_char_t,
                                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                    as *mut yaml_char_t,
                                (if style_1 as u32 == YAML_BLOCK_MAPPING_STYLE as i32 as u32
                                {
                                    b"block\0" as *const u8 as *const i8
                                } else if style_1 as u32
                                    == YAML_FLOW_MAPPING_STYLE as i32 as u32
                                {
                                    b"flow\0" as *const u8 as *const i8
                                } else {
                                    b"unknown\0" as *const u8 as *const i8
                                }) as *mut yaml_char_t,
                                -(1 as i32),
                                1 as i32,
                                1 as i32,
                                YAML_PLAIN_SCALAR_STYLE,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                                break;
                            }
                            if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                                current_block = 8961051500484970212;
                                break;
                            }
                        }
                    }
                    10 => {
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"type\0" as *const u8 as *const i8 as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                        if yaml_scalar_event_initialize(
                            &mut output_event,
                            0 as *const yaml_char_t,
                            b"tag:yaml.org,2002:str\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            b"MAPPING-END\0" as *const u8 as *const i8
                                as *mut yaml_char_t,
                            -(1 as i32),
                            1 as i32,
                            1 as i32,
                            YAML_PLAIN_SCALAR_STYLE,
                        ) == 0
                        {
                            current_block = 16336806485327817624;
                            break;
                        }
                        if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                            current_block = 8961051500484970212;
                            break;
                        }
                    }
                    _ => {}
                }
                yaml_event_delete(&mut input_event);
                if yaml_mapping_end_event_initialize(&mut output_event) == 0 {
                    current_block = 16336806485327817624;
                    break;
                }
                if yaml_emitter_emit(&mut emitter, &mut output_event) == 0 {
                    current_block = 8961051500484970212;
                    break;
                }
            }
            match current_block {
                8961051500484970212 => {}
                16336806485327817624 => {}
                _ => {
                    match current_block {
                        8688973151516351212 => {
                            match parser.error as u32 {
                                1 => {
                                    fprintf(
                                        stderr,
                                        b"Memory error: Not enough memory for parsing\n\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                                2 => {
                                    if parser.problem_value != -(1 as i32) {
                                        fprintf(
                                            stderr,
                                            b"Reader error: %s: #%X at %ld\n\0" as *const u8
                                                as *const i8,
                                            parser.problem,
                                            parser.problem_value,
                                            parser.problem_offset as i64,
                                        );
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"Reader error: %s at %ld\n\0" as *const u8 as *const i8,
                                            parser.problem,
                                            parser.problem_offset as i64,
                                        );
                                    }
                                }
                                3 => {
                                    if !(parser.context).is_null() {
                                        fprintf(
                                            stderr,
                                            b"Scanner error: %s at line %d, column %d\n%s at line %d, column %d\n\0"
                                                as *const u8 as *const i8,
                                            parser.context,
                                            parser.context_mark.line as i32 + 1 as i32,
                                            parser.context_mark.column as i32 + 1 as i32,
                                            parser.problem,
                                            parser.problem_mark.line as i32 + 1 as i32,
                                            parser.problem_mark.column as i32 + 1 as i32,
                                        );
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"Scanner error: %s at line %d, column %d\n\0" as *const u8
                                                as *const i8,
                                            parser.problem,
                                            parser.problem_mark.line as i32 + 1 as i32,
                                            parser.problem_mark.column as i32 + 1 as i32,
                                        );
                                    }
                                }
                                4 => {
                                    if !(parser.context).is_null() {
                                        fprintf(
                                            stderr,
                                            b"Parser error: %s at line %d, column %d\n%s at line %d, column %d\n\0"
                                                as *const u8 as *const i8,
                                            parser.context,
                                            parser.context_mark.line as i32 + 1 as i32,
                                            parser.context_mark.column as i32 + 1 as i32,
                                            parser.problem,
                                            parser.problem_mark.line as i32 + 1 as i32,
                                            parser.problem_mark.column as i32 + 1 as i32,
                                        );
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"Parser error: %s at line %d, column %d\n\0" as *const u8
                                                as *const i8,
                                            parser.problem,
                                            parser.problem_mark.line as i32 + 1 as i32,
                                            parser.problem_mark.column as i32 + 1 as i32,
                                        );
                                    }
                                }
                                _ => {
                                    fprintf(
                                        stderr,
                                        b"Internal error\n\0" as *const u8 as *const i8,
                                    );
                                }
                            }
                            yaml_event_delete(&mut input_event);
                            yaml_parser_delete(&mut parser);
                            yaml_emitter_delete(&mut emitter);
                            return 1 as i32;
                        }
                        _ => {
                            if yaml_sequence_end_event_initialize(&mut output_event) == 0
                            {
                                current_block = 16336806485327817624;
                            } else if yaml_emitter_emit(&mut emitter, &mut output_event)
                                == 0
                            {
                                current_block = 8961051500484970212;
                            } else if yaml_document_end_event_initialize(
                                &mut output_event,
                                0 as i32,
                            ) == 0
                            {
                                current_block = 16336806485327817624;
                            } else if yaml_emitter_emit(&mut emitter, &mut output_event)
                                == 0
                            {
                                current_block = 8961051500484970212;
                            } else if yaml_stream_end_event_initialize(&mut output_event)
                                == 0
                            {
                                current_block = 16336806485327817624;
                            } else if yaml_emitter_emit(&mut emitter, &mut output_event)
                                == 0
                            {
                                current_block = 8961051500484970212;
                            } else {
                                yaml_parser_delete(&mut parser);
                                yaml_emitter_delete(&mut emitter);
                                return 0 as i32;
                            }
                        }
                    }
                }
            }
        }
        match current_block {
            16336806485327817624 => {}
            _ => {
                match emitter.error as u32 {
                    1 => {
                        fprintf(
                            stderr,
                            b"Memory error: Not enough memory for emitting\n\0"
                                as *const u8 as *const i8,
                        );
                    }
                    6 => {
                        fprintf(
                            stderr,
                            b"Writer error: %s\n\0" as *const u8 as *const i8,
                            emitter.problem,
                        );
                    }
                    7 => {
                        fprintf(
                            stderr,
                            b"Emitter error: %s\n\0" as *const u8 as *const i8,
                            emitter.problem,
                        );
                    }
                    _ => {
                        fprintf(stderr, b"Internal error\n\0" as *const u8 as *const i8);
                    }
                }
                yaml_event_delete(&mut input_event);
                yaml_parser_delete(&mut parser);
                yaml_emitter_delete(&mut emitter);
                return 1 as i32;
            }
        }
    }
    fprintf(
        stderr,
        b"Memory error: Not enough memory for creating an event\n\0" as *const u8
            as *const i8,
    );
    yaml_event_delete(&mut input_event);
    yaml_parser_delete(&mut parser);
    yaml_emitter_delete(&mut emitter);
    return 1 as i32;
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