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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_parser_fetch_more_tokens(parser: *mut yaml_parser_t) -> i32;
    fn yaml_stack_extend(
        start: *mut *mut libc::c_void,
        top: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> i32;
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_parse(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const i8,
            b"parser.c\0" as *const u8 as *const i8,
            172 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_11582: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const i8,
                b"parser.c\0" as *const u8 as *const i8,
                172 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !event.is_null() {} else {
        __assert_fail(
            b"event\0" as *const u8 as *const i8,
            b"parser.c\0" as *const u8 as *const i8,
            173 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_11548: {
        if !event.is_null() {} else {
            __assert_fail(
                b"event\0" as *const u8 as *const i8,
                b"parser.c\0" as *const u8 as *const i8,
                173 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"int yaml_parser_parse(yaml_parser_t *, yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as u32 != 0
        || (*parser).state as u32 == YAML_PARSE_END_STATE as i32 as u32
    {
        return 1 as i32;
    }
    return yaml_parser_state_machine(parser, event);
}
unsafe extern "C" fn yaml_parser_set_parser_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const i8,
    mut problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_set_parser_error_context(
    mut parser: *mut yaml_parser_t,
    mut context: *const i8,
    mut context_mark: yaml_mark_t,
    mut problem: *const i8,
    mut problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_state_machine(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    match (*parser).state as u32 {
        0 => return yaml_parser_parse_stream_start(parser, event),
        1 => return yaml_parser_parse_document_start(parser, event, 1 as i32),
        2 => return yaml_parser_parse_document_start(parser, event, 0 as i32),
        3 => return yaml_parser_parse_document_content(parser, event),
        4 => return yaml_parser_parse_document_end(parser, event),
        5 => {
            return yaml_parser_parse_node(parser, event, 1 as i32, 0 as i32);
        }
        6 => {
            return yaml_parser_parse_node(parser, event, 1 as i32, 1 as i32);
        }
        7 => {
            return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
        }
        8 => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 1 as i32);
        }
        9 => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 0 as i32);
        }
        10 => return yaml_parser_parse_indentless_sequence_entry(parser, event),
        11 => return yaml_parser_parse_block_mapping_key(parser, event, 1 as i32),
        12 => return yaml_parser_parse_block_mapping_key(parser, event, 0 as i32),
        13 => return yaml_parser_parse_block_mapping_value(parser, event),
        14 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 1 as i32);
        }
        15 => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 0 as i32);
        }
        16 => return yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event),
        17 => return yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event),
        18 => return yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event),
        19 => return yaml_parser_parse_flow_mapping_key(parser, event, 1 as i32),
        20 => return yaml_parser_parse_flow_mapping_key(parser, event, 0 as i32),
        21 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 0 as i32);
        }
        22 => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 1 as i32);
        }
        _ => {
            'c_2879: {};
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_parse_stream_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 != YAML_STREAM_START_TOKEN as i32 as u32 {
        return yaml_parser_set_parser_error(
            parser,
            b"did not find expected <stream-start>\0" as *const u8 as *const i8,
            (*token).start_mark,
        );
    }
    (*parser).state = YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_STREAM_START_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    (*event).data.stream_start.encoding = (*token).data.stream_start.encoding;
    (*parser).token_available = 0 as i32;
    (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
    (*parser).tokens_parsed;
    (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
        == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
    (*parser).tokens.head = ((*parser).tokens.head).offset(1);
    (*parser).tokens.head;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_parse_document_start(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut implicit: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives: C2RustUnnamed_35 = {
        let mut init = C2RustUnnamed_35 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if implicit == 0 {
        while (*token).type_0 as u32 == YAML_DOCUMENT_END_TOKEN as i32 as u32 {
            (*parser).token_available = 0 as i32;
            (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
            (*parser).tokens_parsed;
            (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
            (*parser).tokens.head;
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                return 0 as i32;
            }
        }
    }
    if implicit != 0
        && (*token).type_0 as u32 != YAML_VERSION_DIRECTIVE_TOKEN as i32 as u32
        && (*token).type_0 as u32 != YAML_TAG_DIRECTIVE_TOKEN as i32 as u32
        && (*token).type_0 as u32 != YAML_DOCUMENT_START_TOKEN as i32 as u32
        && (*token).type_0 as u32 != YAML_STREAM_END_TOKEN as i32 as u32
    {
        if yaml_parser_process_directives(
            parser,
            0 as *mut *mut yaml_version_directive_t,
            0 as *mut *mut yaml_tag_directive_t,
            0 as *mut *mut yaml_tag_directive_t,
        ) == 0
        {
            return 0 as i32;
        }
        if if (*parser).states.top != (*parser).states.end
            || yaml_stack_extend(
                &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh0 = (*parser).states.top;
            (*parser).states.top = ((*parser).states.top).offset(1);
            *fresh0 = YAML_PARSE_DOCUMENT_END_STATE;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).state = YAML_PARSE_BLOCK_NODE_STATE;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_DOCUMENT_START_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        (*event).data.document_start.version_directive = 0
            as *mut yaml_version_directive_t;
        (*event).data.document_start.tag_directives.start = 0
            as *mut yaml_tag_directive_t;
        (*event).data.document_start.tag_directives.end = 0 as *mut yaml_tag_directive_t;
        (*event).data.document_start.implicit = 1 as i32;
        return 1 as i32;
    } else if (*token).type_0 as u32 != YAML_STREAM_END_TOKEN as i32 as u32 {
        let mut start_mark: yaml_mark_t = yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        };
        let mut end_mark: yaml_mark_t = yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        };
        start_mark = (*token).start_mark;
        if yaml_parser_process_directives(
            parser,
            &mut version_directive,
            &mut tag_directives.start,
            &mut tag_directives.end,
        ) == 0
        {
            return 0 as i32;
        }
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if !token.is_null() {
            if (*token).type_0 as u32 != YAML_DOCUMENT_START_TOKEN as i32 as u32 {
                yaml_parser_set_parser_error(
                    parser,
                    b"did not find expected <document start>\0" as *const u8
                        as *const i8,
                    (*token).start_mark,
                );
            } else if !(if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh1 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh1 = YAML_PARSE_DOCUMENT_END_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0)
            {
                (*parser).state = YAML_PARSE_DOCUMENT_CONTENT_STATE;
                end_mark = (*token).end_mark;
                memset(
                    event as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<yaml_event_t>() as u64,
                );
                (*event).type_0 = YAML_DOCUMENT_START_EVENT;
                (*event).start_mark = start_mark;
                (*event).end_mark = end_mark;
                (*event).data.document_start.version_directive = version_directive;
                (*event).data.document_start.tag_directives.start = tag_directives.start;
                (*event).data.document_start.tag_directives.end = tag_directives.end;
                (*event).data.document_start.implicit = 0 as i32;
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                version_directive = 0 as *mut yaml_version_directive_t;
                tag_directives.end = 0 as *mut yaml_tag_directive_t;
                tag_directives.start = tag_directives.end;
                return 1 as i32;
            }
        }
        yaml_free(version_directive as *mut libc::c_void);
        while tag_directives.start != tag_directives.end {
            yaml_free(
                (*(tag_directives.end).offset(-(1 as i32) as isize)).handle
                    as *mut libc::c_void,
            );
            yaml_free(
                (*(tag_directives.end).offset(-(1 as i32) as isize)).prefix
                    as *mut libc::c_void,
            );
            tag_directives.end = (tag_directives.end).offset(-1);
            tag_directives.end;
        }
        yaml_free(tag_directives.start as *mut libc::c_void);
        return 0 as i32;
    } else {
        (*parser).state = YAML_PARSE_END_STATE;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_STREAM_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        return 1 as i32;
    };
}
unsafe extern "C" fn yaml_parser_parse_document_content(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_VERSION_DIRECTIVE_TOKEN as i32 as u32
        || (*token).type_0 as u32 == YAML_TAG_DIRECTIVE_TOKEN as i32 as u32
        || (*token).type_0 as u32 == YAML_DOCUMENT_START_TOKEN as i32 as u32
        || (*token).type_0 as u32 == YAML_DOCUMENT_END_TOKEN as i32 as u32
        || (*token).type_0 as u32 == YAML_STREAM_END_TOKEN as i32 as u32
    {
        (*parser).states.top = ((*parser).states.top).offset(-1);
        (*parser).state = *(*parser).states.top;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    } else {
        return yaml_parser_parse_node(parser, event, 1 as i32, 0 as i32)
    };
}
unsafe extern "C" fn yaml_parser_parse_document_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut implicit: i32 = 1 as i32;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    end_mark = (*token).start_mark;
    start_mark = end_mark;
    if (*token).type_0 as u32 == YAML_DOCUMENT_END_TOKEN as i32 as u32 {
        end_mark = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        implicit = 0 as i32;
    }
    while !((*parser).tag_directives.start == (*parser).tag_directives.top) {
        (*parser).tag_directives.top = ((*parser).tag_directives.top).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = *(*parser).tag_directives.top;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    (*parser).state = YAML_PARSE_DOCUMENT_START_STATE;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_DOCUMENT_END_EVENT;
    (*event).start_mark = start_mark;
    (*event).end_mark = end_mark;
    (*event).data.document_end.implicit = implicit;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_parse_node(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut block: i32,
    mut indentless_sequence: i32,
) -> i32 {
    let mut current_block: u64;
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    let mut anchor: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_handle: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag_suffix: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut tag: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut tag_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut implicit: i32 = 0;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_ALIAS_TOKEN as i32 as u32 {
        (*parser).states.top = ((*parser).states.top).offset(-1);
        (*parser).state = *(*parser).states.top;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_ALIAS_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*event).data.alias.anchor = (*token).data.alias.value;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        return 1 as i32;
    } else {
        end_mark = (*token).start_mark;
        start_mark = end_mark;
        if (*token).type_0 as u32 == YAML_ANCHOR_TOKEN as i32 as u32 {
            anchor = (*token).data.anchor.value;
            start_mark = (*token).start_mark;
            end_mark = (*token).end_mark;
            (*parser).token_available = 0 as i32;
            (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
            (*parser).tokens_parsed;
            (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
            (*parser).tokens.head;
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                current_block = 2030575129073774265;
            } else if (*token).type_0 as u32 == YAML_TAG_TOKEN as i32 as u32 {
                tag_handle = (*token).data.tag.handle;
                tag_suffix = (*token).data.tag.suffix;
                tag_mark = (*token).start_mark;
                end_mark = (*token).end_mark;
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 2030575129073774265;
                } else {
                    current_block = 2569451025026770673;
                }
            } else {
                current_block = 2569451025026770673;
            }
        } else if (*token).type_0 as u32 == YAML_TAG_TOKEN as i32 as u32 {
            tag_handle = (*token).data.tag.handle;
            tag_suffix = (*token).data.tag.suffix;
            tag_mark = (*token).start_mark;
            start_mark = tag_mark;
            end_mark = (*token).end_mark;
            (*parser).token_available = 0 as i32;
            (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
            (*parser).tokens_parsed;
            (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
            (*parser).tokens.head;
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                current_block = 2030575129073774265;
            } else if (*token).type_0 as u32 == YAML_ANCHOR_TOKEN as i32 as u32 {
                anchor = (*token).data.anchor.value;
                end_mark = (*token).end_mark;
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 2030575129073774265;
                } else {
                    current_block = 2569451025026770673;
                }
            } else {
                current_block = 2569451025026770673;
            }
        } else {
            current_block = 2569451025026770673;
        }
        match current_block {
            2569451025026770673 => {
                if !tag_handle.is_null() {
                    if *tag_handle == 0 {
                        tag = tag_suffix;
                        yaml_free(tag_handle as *mut libc::c_void);
                        tag_suffix = 0 as *mut yaml_char_t;
                        tag_handle = tag_suffix;
                        current_block = 1423531122933789233;
                    } else {
                        let mut tag_directive: *mut yaml_tag_directive_t = 0
                            as *mut yaml_tag_directive_t;
                        tag_directive = (*parser).tag_directives.start;
                        loop {
                            if !(tag_directive != (*parser).tag_directives.top) {
                                current_block = 1345366029464561491;
                                break;
                            }
                            if strcmp(
                                (*tag_directive).handle as *mut i8,
                                tag_handle as *mut i8,
                            ) == 0 as i32
                            {
                                let mut prefix_len: size_t = strlen(
                                    (*tag_directive).prefix as *mut i8,
                                );
                                let mut suffix_len: size_t = strlen(tag_suffix as *mut i8);
                                tag = yaml_malloc(
                                    prefix_len
                                        .wrapping_add(suffix_len)
                                        .wrapping_add(1 as i32 as u64),
                                ) as *mut yaml_char_t;
                                if tag.is_null() {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    current_block = 2030575129073774265;
                                    break;
                                } else {
                                    memcpy(
                                        tag as *mut libc::c_void,
                                        (*tag_directive).prefix as *const libc::c_void,
                                        prefix_len,
                                    );
                                    memcpy(
                                        tag.offset(prefix_len as isize) as *mut libc::c_void,
                                        tag_suffix as *const libc::c_void,
                                        suffix_len,
                                    );
                                    *tag.offset(prefix_len.wrapping_add(suffix_len) as isize) = '\0'
                                        as i32 as yaml_char_t;
                                    yaml_free(tag_handle as *mut libc::c_void);
                                    yaml_free(tag_suffix as *mut libc::c_void);
                                    tag_suffix = 0 as *mut yaml_char_t;
                                    tag_handle = tag_suffix;
                                    current_block = 1345366029464561491;
                                    break;
                                }
                            } else {
                                tag_directive = tag_directive.offset(1);
                                tag_directive;
                            }
                        }
                        match current_block {
                            2030575129073774265 => {}
                            _ => {
                                if tag.is_null() {
                                    yaml_parser_set_parser_error_context(
                                        parser,
                                        b"while parsing a node\0" as *const u8 as *const i8,
                                        start_mark,
                                        b"found undefined tag handle\0" as *const u8 as *const i8,
                                        tag_mark,
                                    );
                                    current_block = 2030575129073774265;
                                } else {
                                    current_block = 1423531122933789233;
                                }
                            }
                        }
                    }
                } else {
                    current_block = 1423531122933789233;
                }
                match current_block {
                    2030575129073774265 => {}
                    _ => {
                        implicit = (tag.is_null() || *tag == 0) as i32;
                        if indentless_sequence != 0
                            && (*token).type_0 as u32
                                == YAML_BLOCK_ENTRY_TOKEN as i32 as u32
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.sequence_start.anchor = anchor;
                            (*event).data.sequence_start.tag = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                            return 1 as i32;
                        } else if (*token).type_0 as u32
                            == YAML_SCALAR_TOKEN as i32 as u32
                        {
                            let mut plain_implicit: i32 = 0 as i32;
                            let mut quoted_implicit: i32 = 0 as i32;
                            end_mark = (*token).end_mark;
                            if (*token).data.scalar.style as u32
                                == YAML_PLAIN_SCALAR_STYLE as i32 as u32 && tag.is_null()
                                || !tag.is_null()
                                    && strcmp(tag as *mut i8, b"!\0" as *const u8 as *const i8)
                                        == 0 as i32
                            {
                                plain_implicit = 1 as i32;
                            } else if tag.is_null() {
                                quoted_implicit = 1 as i32;
                            }
                            (*parser).states.top = ((*parser).states.top).offset(-1);
                            (*parser).state = *(*parser).states.top;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_SCALAR_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.scalar.anchor = anchor;
                            (*event).data.scalar.tag = tag;
                            (*event).data.scalar.value = (*token).data.scalar.value;
                            (*event).data.scalar.length = (*token).data.scalar.length;
                            (*event).data.scalar.plain_implicit = plain_implicit;
                            (*event).data.scalar.quoted_implicit = quoted_implicit;
                            (*event).data.scalar.style = (*token).data.scalar.style;
                            (*parser).token_available = 0 as i32;
                            (*parser).tokens_parsed = ((*parser).tokens_parsed)
                                .wrapping_add(1);
                            (*parser).tokens_parsed;
                            (*parser).stream_end_produced = ((*(*parser).tokens.head)
                                .type_0 as u32 == YAML_STREAM_END_TOKEN as i32 as u32)
                                as i32;
                            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                            (*parser).tokens.head;
                            return 1 as i32;
                        } else if (*token).type_0 as u32
                            == YAML_FLOW_SEQUENCE_START_TOKEN as i32 as u32
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.sequence_start.anchor = anchor;
                            (*event).data.sequence_start.tag = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_FLOW_SEQUENCE_STYLE;
                            return 1 as i32;
                        } else if (*token).type_0 as u32
                            == YAML_FLOW_MAPPING_START_TOKEN as i32 as u32
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.mapping_start.anchor = anchor;
                            (*event).data.mapping_start.tag = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
                            return 1 as i32;
                        } else if block != 0
                            && (*token).type_0 as u32
                                == YAML_BLOCK_SEQUENCE_START_TOKEN as i32 as u32
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_SEQUENCE_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.sequence_start.anchor = anchor;
                            (*event).data.sequence_start.tag = tag;
                            (*event).data.sequence_start.implicit = implicit;
                            (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                            return 1 as i32;
                        } else if block != 0
                            && (*token).type_0 as u32
                                == YAML_BLOCK_MAPPING_START_TOKEN as i32 as u32
                        {
                            end_mark = (*token).end_mark;
                            (*parser).state = YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE;
                            memset(
                                event as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            (*event).type_0 = YAML_MAPPING_START_EVENT;
                            (*event).start_mark = start_mark;
                            (*event).end_mark = end_mark;
                            (*event).data.mapping_start.anchor = anchor;
                            (*event).data.mapping_start.tag = tag;
                            (*event).data.mapping_start.implicit = implicit;
                            (*event).data.mapping_start.style = YAML_BLOCK_MAPPING_STYLE;
                            return 1 as i32;
                        } else if !anchor.is_null() || !tag.is_null() {
                            let mut value: *mut yaml_char_t = yaml_malloc(
                                1 as i32 as size_t,
                            ) as *mut yaml_char_t;
                            if value.is_null() {
                                (*parser).error = YAML_MEMORY_ERROR;
                            } else {
                                *value.offset(0 as i32 as isize) = '\0' as i32
                                    as yaml_char_t;
                                (*parser).states.top = ((*parser).states.top).offset(-1);
                                (*parser).state = *(*parser).states.top;
                                memset(
                                    event as *mut libc::c_void,
                                    0 as i32,
                                    ::core::mem::size_of::<yaml_event_t>() as u64,
                                );
                                (*event).type_0 = YAML_SCALAR_EVENT;
                                (*event).start_mark = start_mark;
                                (*event).end_mark = end_mark;
                                (*event).data.scalar.anchor = anchor;
                                (*event).data.scalar.tag = tag;
                                (*event).data.scalar.value = value;
                                (*event).data.scalar.length = 0 as i32 as size_t;
                                (*event).data.scalar.plain_implicit = implicit;
                                (*event).data.scalar.quoted_implicit = 0 as i32;
                                (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                                return 1 as i32;
                            }
                        } else {
                            yaml_parser_set_parser_error_context(
                                parser,
                                if block != 0 {
                                    b"while parsing a block node\0" as *const u8 as *const i8
                                } else {
                                    b"while parsing a flow node\0" as *const u8 as *const i8
                                },
                                start_mark,
                                b"did not find expected node content\0" as *const u8
                                    as *const i8,
                                (*token).start_mark,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        yaml_free(anchor as *mut libc::c_void);
        yaml_free(tag_handle as *mut libc::c_void);
        yaml_free(tag_suffix as *mut libc::c_void);
        yaml_free(tag as *mut libc::c_void);
        return 0 as i32;
    };
}
unsafe extern "C" fn yaml_parser_parse_block_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh2 = (*parser).marks.top;
            (*parser).marks.top = ((*parser).marks.top).offset(1);
            *fresh2 = (*token).start_mark;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_BLOCK_ENTRY_TOKEN as i32 as u32 {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_BLOCK_ENTRY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_BLOCK_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh3 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh3 = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 1 as i32, 0 as i32);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_0 as u32 == YAML_BLOCK_END_TOKEN as i32 as u32 {
        (*parser).states.top = ((*parser).states.top).offset(-1);
        (*parser).state = *(*parser).states.top;
        (*parser).marks.top = ((*parser).marks.top).offset(-1);
        *(*parser).marks.top;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        return 1 as i32;
    } else {
        (*parser).marks.top = ((*parser).marks.top).offset(-1);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block collection\0" as *const u8 as *const i8,
            *(*parser).marks.top,
            b"did not find expected '-' indicator\0" as *const u8 as *const i8,
            (*token).start_mark,
        );
    };
}
unsafe extern "C" fn yaml_parser_parse_indentless_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_BLOCK_ENTRY_TOKEN as i32 as u32 {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_BLOCK_ENTRY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_KEY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_VALUE_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_BLOCK_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh4 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh4 = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 1 as i32, 0 as i32);
        } else {
            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).states.top = ((*parser).states.top).offset(-1);
        (*parser).state = *(*parser).states.top;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_SEQUENCE_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).start_mark;
        return 1 as i32;
    };
}
unsafe extern "C" fn yaml_parser_parse_block_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh5 = (*parser).marks.top;
            (*parser).marks.top = ((*parser).marks.top).offset(1);
            *fresh5 = (*token).start_mark;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_KEY_TOKEN as i32 as u32 {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_KEY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_VALUE_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_BLOCK_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh6 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh6 = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 1 as i32, 1 as i32);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_0 as u32 == YAML_BLOCK_END_TOKEN as i32 as u32 {
        (*parser).states.top = ((*parser).states.top).offset(-1);
        (*parser).state = *(*parser).states.top;
        (*parser).marks.top = ((*parser).marks.top).offset(-1);
        *(*parser).marks.top;
        memset(
            event as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_event_t>() as u64,
        );
        (*event).type_0 = YAML_MAPPING_END_EVENT;
        (*event).start_mark = (*token).start_mark;
        (*event).end_mark = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        return 1 as i32;
    } else {
        (*parser).marks.top = ((*parser).marks.top).offset(-1);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block mapping\0" as *const u8 as *const i8,
            *(*parser).marks.top,
            b"did not find expected key\0" as *const u8 as *const i8,
            (*token).start_mark,
        );
    };
}
unsafe extern "C" fn yaml_parser_parse_block_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_VALUE_TOKEN as i32 as u32 {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_KEY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_VALUE_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_BLOCK_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh7 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh7 = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 1 as i32, 1 as i32);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    };
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh8 = (*parser).marks.top;
            (*parser).marks.top = ((*parser).marks.top).offset(1);
            *fresh8 = (*token).start_mark;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 != YAML_FLOW_SEQUENCE_END_TOKEN as i32 as u32 {
        if first == 0 {
            if (*token).type_0 as u32 == YAML_FLOW_ENTRY_TOKEN as i32 as u32 {
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    return 0 as i32;
                }
            } else {
                (*parser).marks.top = ((*parser).marks.top).offset(-1);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow sequence\0" as *const u8 as *const i8,
                    *(*parser).marks.top,
                    b"did not find expected ',' or ']'\0" as *const u8 as *const i8,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_0 as u32 == YAML_KEY_TOKEN as i32 as u32 {
            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE;
            memset(
                event as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<yaml_event_t>() as u64,
            );
            (*event).type_0 = YAML_MAPPING_START_EVENT;
            (*event).start_mark = (*token).start_mark;
            (*event).end_mark = (*token).end_mark;
            (*event).data.mapping_start.anchor = 0 as *mut yaml_char_t;
            (*event).data.mapping_start.tag = 0 as *mut yaml_char_t;
            (*event).data.mapping_start.implicit = 1 as i32;
            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
            (*parser).token_available = 0 as i32;
            (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
            (*parser).tokens_parsed;
            (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
            (*parser).tokens.head;
            return 1 as i32;
        } else if (*token).type_0 as u32 != YAML_FLOW_SEQUENCE_END_TOKEN as i32 as u32 {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh9 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh9 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
        }
    }
    (*parser).states.top = ((*parser).states.top).offset(-1);
    (*parser).state = *(*parser).states.top;
    (*parser).marks.top = ((*parser).marks.top).offset(-1);
    *(*parser).marks.top;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    (*parser).token_available = 0 as i32;
    (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
    (*parser).tokens_parsed;
    (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
        == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
    (*parser).tokens.head = ((*parser).tokens.head).offset(1);
    (*parser).tokens.head;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 != YAML_VALUE_TOKEN as i32 as u32
        && (*token).type_0 as u32 != YAML_FLOW_ENTRY_TOKEN as i32 as u32
        && (*token).type_0 as u32 != YAML_FLOW_SEQUENCE_END_TOKEN as i32 as u32
    {
        if if (*parser).states.top != (*parser).states.end
            || yaml_stack_extend(
                &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
                &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh10 = (*parser).states.top;
            (*parser).states.top = ((*parser).states.top).offset(1);
            *fresh10 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
    } else {
        let mut mark: yaml_mark_t = (*token).end_mark;
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    };
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 == YAML_VALUE_TOKEN as i32 as u32 {
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_FLOW_ENTRY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_FLOW_SEQUENCE_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh11 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh11 = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
        }
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
    return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
}
unsafe extern "C" fn yaml_parser_parse_flow_sequence_entry_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).start_mark;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_parse_flow_mapping_key(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut first: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    if first != 0 {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if if (*parser).marks.top != (*parser).marks.end
            || yaml_stack_extend(
                &mut (*parser).marks.start as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.top as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
                &mut (*parser).marks.end as *mut *mut yaml_mark_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh12 = (*parser).marks.top;
            (*parser).marks.top = ((*parser).marks.top).offset(1);
            *fresh12 = (*token).start_mark;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
    }
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if (*token).type_0 as u32 != YAML_FLOW_MAPPING_END_TOKEN as i32 as u32 {
        if first == 0 {
            if (*token).type_0 as u32 == YAML_FLOW_ENTRY_TOKEN as i32 as u32 {
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    return 0 as i32;
                }
            } else {
                (*parser).marks.top = ((*parser).marks.top).offset(-1);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow mapping\0" as *const u8 as *const i8,
                    *(*parser).marks.top,
                    b"did not find expected ',' or '}'\0" as *const u8 as *const i8,
                    (*token).start_mark,
                );
            }
        }
        if (*token).type_0 as u32 == YAML_KEY_TOKEN as i32 as u32 {
            (*parser).token_available = 0 as i32;
            (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
            (*parser).tokens_parsed;
            (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
            (*parser).tokens.head = ((*parser).tokens.head).offset(1);
            (*parser).tokens.head;
            token = if (*parser).token_available != 0
                || yaml_parser_fetch_more_tokens(parser) != 0
            {
                (*parser).tokens.head
            } else {
                0 as *mut yaml_token_t
            };
            if token.is_null() {
                return 0 as i32;
            }
            if (*token).type_0 as u32 != YAML_VALUE_TOKEN as i32 as u32
                && (*token).type_0 as u32 != YAML_FLOW_ENTRY_TOKEN as i32 as u32
                && (*token).type_0 as u32 != YAML_FLOW_MAPPING_END_TOKEN as i32 as u32
            {
                if if (*parser).states.top != (*parser).states.end
                    || yaml_stack_extend(
                        &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                        &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh13 = (*parser).states.top;
                    (*parser).states.top = ((*parser).states.top).offset(1);
                    *fresh13 = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0
                {
                    return 0 as i32;
                }
                return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
            } else {
                (*parser).state = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                return yaml_parser_process_empty_scalar(
                    parser,
                    event,
                    (*token).start_mark,
                );
            }
        } else if (*token).type_0 as u32 != YAML_FLOW_MAPPING_END_TOKEN as i32 as u32 {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh14 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh14 = YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
        }
    }
    (*parser).states.top = ((*parser).states.top).offset(-1);
    (*parser).state = *(*parser).states.top;
    (*parser).marks.top = ((*parser).marks.top).offset(-1);
    *(*parser).marks.top;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_MAPPING_END_EVENT;
    (*event).start_mark = (*token).start_mark;
    (*event).end_mark = (*token).end_mark;
    (*parser).token_available = 0 as i32;
    (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
    (*parser).tokens_parsed;
    (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
        == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
    (*parser).tokens.head = ((*parser).tokens.head).offset(1);
    (*parser).tokens.head;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_parse_flow_mapping_value(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut empty: i32,
) -> i32 {
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    token = if (*parser).token_available != 0
        || yaml_parser_fetch_more_tokens(parser) != 0
    {
        (*parser).tokens.head
    } else {
        0 as *mut yaml_token_t
    };
    if token.is_null() {
        return 0 as i32;
    }
    if empty != 0 {
        (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }
    if (*token).type_0 as u32 == YAML_VALUE_TOKEN as i32 as u32 {
        (*parser).token_available = 0 as i32;
        (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
        (*parser).tokens_parsed;
        (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
            == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
        (*parser).tokens.head = ((*parser).tokens.head).offset(1);
        (*parser).tokens.head;
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if token.is_null() {
            return 0 as i32;
        }
        if (*token).type_0 as u32 != YAML_FLOW_ENTRY_TOKEN as i32 as u32
            && (*token).type_0 as u32 != YAML_FLOW_MAPPING_END_TOKEN as i32 as u32
        {
            if if (*parser).states.top != (*parser).states.end
                || yaml_stack_extend(
                    &mut (*parser).states.start as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.top as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).states.end as *mut *mut yaml_parser_state_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh15 = (*parser).states.top;
                (*parser).states.top = ((*parser).states.top).offset(1);
                *fresh15 = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            return yaml_parser_parse_node(parser, event, 0 as i32, 0 as i32);
        }
    }
    (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
    return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
}
unsafe extern "C" fn yaml_parser_process_empty_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut mark: yaml_mark_t,
) -> i32 {
    let mut value: *mut yaml_char_t = 0 as *mut yaml_char_t;
    value = yaml_malloc(1 as i32 as size_t) as *mut yaml_char_t;
    if value.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0 as i32;
    }
    *value.offset(0 as i32 as isize) = '\0' as i32 as yaml_char_t;
    memset(
        event as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    (*event).type_0 = YAML_SCALAR_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    (*event).data.scalar.anchor = 0 as *mut yaml_char_t;
    (*event).data.scalar.tag = 0 as *mut yaml_char_t;
    (*event).data.scalar.value = value;
    (*event).data.scalar.length = 0 as i32 as size_t;
    (*event).data.scalar.plain_implicit = 1 as i32;
    (*event).data.scalar.quoted_implicit = 0 as i32;
    (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_process_directives(
    mut parser: *mut yaml_parser_t,
    mut version_directive_ref: *mut *mut yaml_version_directive_t,
    mut tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    mut tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> i32 {
    let mut current_block: u64;
    let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
        {
            let mut init = yaml_tag_directive_s {
                handle: b"!\0" as *const u8 as *const i8 as *mut yaml_char_t,
                prefix: b"!\0" as *const u8 as *const i8 as *mut yaml_char_t,
            };
            init
        },
        {
            let mut init = yaml_tag_directive_s {
                handle: b"!!\0" as *const u8 as *const i8 as *mut yaml_char_t,
                prefix: b"tag:yaml.org,2002:\0" as *const u8 as *const i8
                    as *mut yaml_char_t,
            };
            init
        },
        {
            let mut init = yaml_tag_directive_s {
                handle: 0 as *mut yaml_char_t,
                prefix: 0 as *mut yaml_char_t,
            };
            init
        },
    ];
    let mut default_tag_directive: *mut yaml_tag_directive_t = 0
        as *mut yaml_tag_directive_t;
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut tag_directives: C2RustUnnamed_36 = {
        let mut init = C2RustUnnamed_36 {
            start: 0 as *mut yaml_tag_directive_t,
            end: 0 as *mut yaml_tag_directive_t,
            top: 0 as *mut yaml_tag_directive_t,
        };
        init
    };
    let mut token: *mut yaml_token_t = 0 as *mut yaml_token_t;
    tag_directives.start = yaml_malloc(
        (16 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<yaml_tag_directive_t>() as u64),
    ) as *mut yaml_tag_directive_t;
    if !(if !(tag_directives.start).is_null() {
        tag_directives.top = tag_directives.start;
        tag_directives.end = (tag_directives.start).offset(16 as i32 as isize);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        token = if (*parser).token_available != 0
            || yaml_parser_fetch_more_tokens(parser) != 0
        {
            (*parser).tokens.head
        } else {
            0 as *mut yaml_token_t
        };
        if !token.is_null() {
            loop {
                if !((*token).type_0 as u32 == YAML_VERSION_DIRECTIVE_TOKEN as i32 as u32
                    || (*token).type_0 as u32 == YAML_TAG_DIRECTIVE_TOKEN as i32 as u32)
                {
                    current_block = 1109700713171191020;
                    break;
                }
                if (*token).type_0 as u32 == YAML_VERSION_DIRECTIVE_TOKEN as i32 as u32 {
                    if !version_directive.is_null() {
                        yaml_parser_set_parser_error(
                            parser,
                            b"found duplicate %YAML directive\0" as *const u8
                                as *const i8,
                            (*token).start_mark,
                        );
                        current_block = 11478196916040693378;
                        break;
                    } else if (*token).data.version_directive.major != 1 as i32
                        || (*token).data.version_directive.minor != 1 as i32
                            && (*token).data.version_directive.minor != 2 as i32
                    {
                        yaml_parser_set_parser_error(
                            parser,
                            b"found incompatible YAML document\0" as *const u8
                                as *const i8,
                            (*token).start_mark,
                        );
                        current_block = 11478196916040693378;
                        break;
                    } else {
                        version_directive = yaml_malloc(
                            ::core::mem::size_of::<yaml_version_directive_t>() as u64,
                        ) as *mut yaml_version_directive_t;
                        if version_directive.is_null() {
                            (*parser).error = YAML_MEMORY_ERROR;
                            current_block = 11478196916040693378;
                            break;
                        } else {
                            (*version_directive).major = (*token)
                                .data
                                .version_directive
                                .major;
                            (*version_directive).minor = (*token)
                                .data
                                .version_directive
                                .minor;
                        }
                    }
                } else if (*token).type_0 as u32
                    == YAML_TAG_DIRECTIVE_TOKEN as i32 as u32
                {
                    let mut value: yaml_tag_directive_t = yaml_tag_directive_t {
                        handle: 0 as *mut yaml_char_t,
                        prefix: 0 as *mut yaml_char_t,
                    };
                    value.handle = (*token).data.tag_directive.handle;
                    value.prefix = (*token).data.tag_directive.prefix;
                    if yaml_parser_append_tag_directive(
                        parser,
                        value,
                        0 as i32,
                        (*token).start_mark,
                    ) == 0
                    {
                        current_block = 11478196916040693378;
                        break;
                    }
                    if if tag_directives.top != tag_directives.end
                        || yaml_stack_extend(
                            &mut tag_directives.start as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                            &mut tag_directives.top as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                            &mut tag_directives.end as *mut *mut yaml_tag_directive_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh16 = tag_directives.top;
                        tag_directives.top = (tag_directives.top).offset(1);
                        *fresh16 = value;
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0
                    {
                        current_block = 11478196916040693378;
                        break;
                    }
                }
                (*parser).token_available = 0 as i32;
                (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
                (*parser).tokens_parsed;
                (*parser).stream_end_produced = ((*(*parser).tokens.head).type_0 as u32
                    == YAML_STREAM_END_TOKEN as i32 as u32) as i32;
                (*parser).tokens.head = ((*parser).tokens.head).offset(1);
                (*parser).tokens.head;
                token = if (*parser).token_available != 0
                    || yaml_parser_fetch_more_tokens(parser) != 0
                {
                    (*parser).tokens.head
                } else {
                    0 as *mut yaml_token_t
                };
                if token.is_null() {
                    current_block = 11478196916040693378;
                    break;
                }
            }
            match current_block {
                11478196916040693378 => {}
                _ => {
                    default_tag_directive = default_tag_directives.as_mut_ptr();
                    loop {
                        if ((*default_tag_directive).handle).is_null() {
                            current_block = 2232869372362427478;
                            break;
                        }
                        if yaml_parser_append_tag_directive(
                            parser,
                            *default_tag_directive,
                            1 as i32,
                            (*token).start_mark,
                        ) == 0
                        {
                            current_block = 11478196916040693378;
                            break;
                        }
                        default_tag_directive = default_tag_directive.offset(1);
                        default_tag_directive;
                    }
                    match current_block {
                        11478196916040693378 => {}
                        _ => {
                            if !version_directive_ref.is_null() {
                                *version_directive_ref = version_directive;
                            }
                            if !tag_directives_start_ref.is_null() {
                                if tag_directives.start == tag_directives.top {
                                    *tag_directives_end_ref = 0 as *mut yaml_tag_directive_t;
                                    *tag_directives_start_ref = *tag_directives_end_ref;
                                    yaml_free(tag_directives.start as *mut libc::c_void);
                                    tag_directives.end = 0 as *mut yaml_tag_directive_t;
                                    tag_directives.top = tag_directives.end;
                                    tag_directives.start = tag_directives.top;
                                } else {
                                    *tag_directives_start_ref = tag_directives.start;
                                    *tag_directives_end_ref = tag_directives.top;
                                }
                            } else {
                                yaml_free(tag_directives.start as *mut libc::c_void);
                                tag_directives.end = 0 as *mut yaml_tag_directive_t;
                                tag_directives.top = tag_directives.end;
                                tag_directives.start = tag_directives.top;
                            }
                            if version_directive_ref.is_null() {
                                yaml_free(version_directive as *mut libc::c_void);
                            }
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(version_directive as *mut libc::c_void);
    while !(tag_directives.start == tag_directives.top) {
        tag_directives.top = (tag_directives.top).offset(-1);
        let mut tag_directive: yaml_tag_directive_t = *tag_directives.top;
        yaml_free(tag_directive.handle as *mut libc::c_void);
        yaml_free(tag_directive.prefix as *mut libc::c_void);
    }
    yaml_free(tag_directives.start as *mut libc::c_void);
    tag_directives.end = 0 as *mut yaml_tag_directive_t;
    tag_directives.top = tag_directives.end;
    tag_directives.start = tag_directives.top;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_append_tag_directive(
    mut parser: *mut yaml_parser_t,
    mut value: yaml_tag_directive_t,
    mut allow_duplicates: i32,
    mut mark: yaml_mark_t,
) -> i32 {
    let mut tag_directive: *mut yaml_tag_directive_t = 0 as *mut yaml_tag_directive_t;
    let mut copy: yaml_tag_directive_t = {
        let mut init = yaml_tag_directive_s {
            handle: 0 as *mut yaml_char_t,
            prefix: 0 as *mut yaml_char_t,
        };
        init
    };
    tag_directive = (*parser).tag_directives.start;
    while tag_directive != (*parser).tag_directives.top {
        if strcmp(value.handle as *mut i8, (*tag_directive).handle as *mut i8)
            == 0 as i32
        {
            if allow_duplicates != 0 {
                return 1 as i32;
            }
            return yaml_parser_set_parser_error(
                parser,
                b"found duplicate %TAG directive\0" as *const u8 as *const i8,
                mark,
            );
        }
        tag_directive = tag_directive.offset(1);
        tag_directive;
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if (copy.handle).is_null() || (copy.prefix).is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
    } else if !(if (*parser).tag_directives.top != (*parser).tag_directives.end
        || yaml_stack_extend(
            &mut (*parser).tag_directives.start as *mut *mut yaml_tag_directive_t
                as *mut *mut libc::c_void,
            &mut (*parser).tag_directives.top as *mut *mut yaml_tag_directive_t
                as *mut *mut libc::c_void,
            &mut (*parser).tag_directives.end as *mut *mut yaml_tag_directive_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh17 = (*parser).tag_directives.top;
        (*parser).tag_directives.top = ((*parser).tag_directives.top).offset(1);
        *fresh17 = copy;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        return 1 as i32
    }
    yaml_free(copy.handle as *mut libc::c_void);
    yaml_free(copy.prefix as *mut libc::c_void);
    return 0 as i32;
}