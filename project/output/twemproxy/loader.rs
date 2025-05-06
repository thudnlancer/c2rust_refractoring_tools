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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_parser_parse(parser: *mut yaml_parser_t, event: *mut yaml_event_t) -> i32;
    fn yaml_free(ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn yaml_stack_extend(
        start: *mut *mut libc::c_void,
        top: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> i32;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
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
pub struct loader_ctx {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_load(
    mut parser: *mut yaml_parser_t,
    mut document: *mut yaml_document_t,
) -> i32 {
    let mut current_block: u64;
    let mut event: yaml_event_t = yaml_event_t {
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
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            91 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[i8; 57],
            >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6317: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                91 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            92 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 57],
                &[i8; 57],
            >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6285: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                92 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 57],
                    &[i8; 57],
                >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        document as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_document_t>() as u64,
    );
    (*document).nodes.start = yaml_malloc(
        (16 as i32 as u64).wrapping_mul(::core::mem::size_of::<yaml_node_t>() as u64),
    ) as *mut yaml_node_t;
    if !(if !((*document).nodes.start).is_null() {
        (*document).nodes.top = (*document).nodes.start;
        (*document).nodes.end = ((*document).nodes.start).offset(16 as i32 as isize);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if (*parser).stream_start_produced == 0 {
            if yaml_parser_parse(parser, &mut event) == 0 {
                current_block = 13736911338061448644;
            } else {
                if event.type_0 as u32 == YAML_STREAM_START_EVENT as i32 as u32 {} else {
                    __assert_fail(
                        b"event.type == YAML_STREAM_START_EVENT\0" as *const u8
                            as *const i8,
                        b"loader.c\0" as *const u8 as *const i8,
                        100 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 57],
                            &[i8; 57],
                        >(b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"))
                            .as_ptr(),
                    );
                }
                'c_6153: {
                    if event.type_0 as u32 == YAML_STREAM_START_EVENT as i32 as u32
                    {} else {
                        __assert_fail(
                            b"event.type == YAML_STREAM_START_EVENT\0" as *const u8
                                as *const i8,
                            b"loader.c\0" as *const u8 as *const i8,
                            100 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 57],
                                &[i8; 57],
                            >(
                                b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            13736911338061448644 => {}
            _ => {
                if (*parser).stream_end_produced != 0 {
                    return 1 as i32;
                }
                if !(yaml_parser_parse(parser, &mut event) == 0) {
                    if event.type_0 as u32 == YAML_STREAM_END_EVENT as i32 as u32 {
                        return 1 as i32;
                    }
                    (*parser).aliases.start = yaml_malloc(
                        (16 as i32 as u64)
                            .wrapping_mul(
                                ::core::mem::size_of::<yaml_alias_data_t>() as u64,
                            ),
                    ) as *mut yaml_alias_data_t;
                    if !(if !((*parser).aliases.start).is_null() {
                        (*parser).aliases.top = (*parser).aliases.start;
                        (*parser).aliases.end = ((*parser).aliases.start)
                            .offset(16 as i32 as isize);
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0)
                    {
                        (*parser).document = document;
                        if !(yaml_parser_load_document(parser, &mut event) == 0) {
                            yaml_parser_delete_aliases(parser);
                            (*parser).document = 0 as *mut yaml_document_t;
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    }
    yaml_parser_delete_aliases(parser);
    yaml_document_delete(document);
    (*parser).document = 0 as *mut yaml_document_t;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_set_composer_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const i8,
    mut problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_set_composer_error_context(
    mut parser: *mut yaml_parser_t,
    mut context: *const i8,
    mut context_mark: yaml_mark_t,
    mut problem: *const i8,
    mut problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_delete_aliases(mut parser: *mut yaml_parser_t) {
    while !((*parser).aliases.start == (*parser).aliases.top) {
        (*parser).aliases.top = ((*parser).aliases.top).offset(-1);
        yaml_free((*(*parser).aliases.top).anchor as *mut libc::c_void);
    }
    yaml_free((*parser).aliases.start as *mut libc::c_void);
    (*parser).aliases.end = 0 as *mut yaml_alias_data_t;
    (*parser).aliases.top = (*parser).aliases.end;
    (*parser).aliases.start = (*parser).aliases.top;
}
unsafe extern "C" fn yaml_parser_load_document(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> i32 {
    let mut ctx: loader_ctx = {
        let mut init = loader_ctx {
            start: 0 as *mut i32,
            end: 0 as *mut i32,
            top: 0 as *mut i32,
        };
        init
    };
    if (*event).type_0 as u32 == YAML_DOCUMENT_START_EVENT as i32 as u32 {} else {
        __assert_fail(
            b"event->type == YAML_DOCUMENT_START_EVENT\0" as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            189 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 63],
                &[i8; 63],
            >(b"int yaml_parser_load_document(yaml_parser_t *, yaml_event_t *)\0"))
                .as_ptr(),
        );
    }
    'c_6005: {
        if (*event).type_0 as u32 == YAML_DOCUMENT_START_EVENT as i32 as u32 {} else {
            __assert_fail(
                b"event->type == YAML_DOCUMENT_START_EVENT\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                189 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 63],
                    &[i8; 63],
                >(b"int yaml_parser_load_document(yaml_parser_t *, yaml_event_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*(*parser).document).version_directive = (*event)
        .data
        .document_start
        .version_directive;
    (*(*parser).document).tag_directives.start = (*event)
        .data
        .document_start
        .tag_directives
        .start;
    (*(*parser).document).tag_directives.end = (*event)
        .data
        .document_start
        .tag_directives
        .end;
    (*(*parser).document).start_implicit = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;
    ctx.start = yaml_malloc(
        (16 as i32 as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    ) as *mut i32;
    if if !(ctx.start).is_null() {
        ctx.top = ctx.start;
        ctx.end = (ctx.start).offset(16 as i32 as isize);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        return 0 as i32;
    }
    if yaml_parser_load_nodes(parser, &mut ctx) == 0 {
        yaml_free(ctx.start as *mut libc::c_void);
        ctx.end = 0 as *mut i32;
        ctx.top = ctx.end;
        ctx.start = ctx.top;
        return 0 as i32;
    }
    yaml_free(ctx.start as *mut libc::c_void);
    ctx.end = 0 as *mut i32;
    ctx.top = ctx.end;
    ctx.start = ctx.top;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_load_nodes(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut event: yaml_event_t = yaml_event_t {
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
    loop {
        if yaml_parser_parse(parser, &mut event) == 0 {
            return 0 as i32;
        }
        match event.type_0 as u32 {
            5 => {
                if yaml_parser_load_alias(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            6 => {
                if yaml_parser_load_scalar(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            7 => {
                if yaml_parser_load_sequence(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            8 => {
                if yaml_parser_load_sequence_end(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            9 => {
                if yaml_parser_load_mapping(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            10 => {
                if yaml_parser_load_mapping_end(parser, &mut event, ctx) == 0 {
                    return 0 as i32;
                }
            }
            4 => {}
            _ => {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"loader.c\0" as *const u8 as *const i8,
                    246 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 65],
                        &[i8; 65],
                    >(
                        b"int yaml_parser_load_nodes(yaml_parser_t *, struct loader_ctx *)\0",
                    ))
                        .as_ptr(),
                );
                'c_3140: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const i8,
                        b"loader.c\0" as *const u8 as *const i8,
                        246 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 65],
                            &[i8; 65],
                        >(
                            b"int yaml_parser_load_nodes(yaml_parser_t *, struct loader_ctx *)\0",
                        ))
                            .as_ptr(),
                    );
                };
                return 0 as i32;
            }
        }
        if !(event.type_0 as u32 != YAML_DOCUMENT_END_EVENT as i32 as u32) {
            break;
        }
    }
    (*(*parser).document).end_implicit = event.data.document_end.implicit;
    (*(*parser).document).end_mark = event.end_mark;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_register_anchor(
    mut parser: *mut yaml_parser_t,
    mut index: i32,
    mut anchor: *mut yaml_char_t,
) -> i32 {
    let mut data: yaml_alias_data_t = yaml_alias_data_t {
        anchor: 0 as *mut yaml_char_t,
        index: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut alias_data: *mut yaml_alias_data_t = 0 as *mut yaml_alias_data_t;
    if anchor.is_null() {
        return 1 as i32;
    }
    data.anchor = anchor;
    data.index = index;
    data.mark = (*((*(*parser).document).nodes.start)
        .offset((index - 1 as i32) as isize))
        .start_mark;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp((*alias_data).anchor as *mut i8, anchor as *mut i8) == 0 as i32 {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0" as *const u8 as *const i8,
                (*alias_data).mark,
                b"second occurrence\0" as *const u8 as *const i8,
                data.mark,
            );
        }
        alias_data = alias_data.offset(1);
        alias_data;
    }
    if if (*parser).aliases.top != (*parser).aliases.end
        || yaml_stack_extend(
            &mut (*parser).aliases.start as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
            &mut (*parser).aliases.top as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
            &mut (*parser).aliases.end as *mut *mut yaml_alias_data_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh0 = (*parser).aliases.top;
        (*parser).aliases.top = ((*parser).aliases.top).offset(1);
        *fresh0 = data;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_free(anchor as *mut libc::c_void);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_load_node_add(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
    mut index: i32,
) -> i32 {
    let mut parent: *mut yaml_node_s = 0 as *mut yaml_node_s;
    let mut parent_index: i32 = 0;
    if (*ctx).start == (*ctx).top {
        return 1 as i32;
    }
    parent_index = *((*ctx).top).offset(-(1 as i32 as isize));
    parent = &mut *((*(*parser).document).nodes.start)
        .offset((parent_index - 1 as i32) as isize) as *mut yaml_node_t;
    let mut current_block_17: u64;
    match (*parent).type_0 as u32 {
        2 => {
            if if (((*parent).data.sequence.items.top)
                .offset_from((*parent).data.sequence.items.start) as i64)
                < (2147483647 as i32 - 1 as i32) as i64
            {
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
            if if (*parent).data.sequence.items.top != (*parent).data.sequence.items.end
                || yaml_stack_extend(
                    &mut (*parent).data.sequence.items.start
                        as *mut *mut yaml_node_item_t as *mut *mut libc::c_void,
                    &mut (*parent).data.sequence.items.top as *mut *mut yaml_node_item_t
                        as *mut *mut libc::c_void,
                    &mut (*parent).data.sequence.items.end as *mut *mut yaml_node_item_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh1 = (*parent).data.sequence.items.top;
                (*parent).data.sequence.items.top = ((*parent).data.sequence.items.top)
                    .offset(1);
                *fresh1 = index;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
        }
        3 => {
            let mut pair: yaml_node_pair_t = yaml_node_pair_t {
                key: 0,
                value: 0,
            };
            if !((*parent).data.mapping.pairs.start == (*parent).data.mapping.pairs.top)
            {
                let mut p: *mut yaml_node_pair_t = ((*parent).data.mapping.pairs.top)
                    .offset(-(1 as i32 as isize));
                if (*p).key != 0 as i32 && (*p).value == 0 as i32 {
                    (*p).value = index;
                    current_block_17 = 2370887241019905314;
                } else {
                    current_block_17 = 1841672684692190573;
                }
            } else {
                current_block_17 = 1841672684692190573;
            }
            match current_block_17 {
                2370887241019905314 => {}
                _ => {
                    pair.key = index;
                    pair.value = 0 as i32;
                    if if (((*parent).data.mapping.pairs.top)
                        .offset_from((*parent).data.mapping.pairs.start) as i64)
                        < (2147483647 as i32 - 1 as i32) as i64
                    {
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0
                    {
                        return 0 as i32;
                    }
                    if if (*parent).data.mapping.pairs.top
                        != (*parent).data.mapping.pairs.end
                        || yaml_stack_extend(
                            &mut (*parent).data.mapping.pairs.start
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                            &mut (*parent).data.mapping.pairs.top
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                            &mut (*parent).data.mapping.pairs.end
                                as *mut *mut yaml_node_pair_t as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh2 = (*parent).data.mapping.pairs.top;
                        (*parent).data.mapping.pairs.top = ((*parent)
                            .data
                            .mapping
                            .pairs
                            .top)
                            .offset(1);
                        *fresh2 = pair;
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0
                    {
                        return 0 as i32;
                    }
                }
            }
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                340 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"int yaml_parser_load_node_add(yaml_parser_t *, struct loader_ctx *, int)\0",
                ))
                    .as_ptr(),
            );
            'c_3595: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"loader.c\0" as *const u8 as *const i8,
                    340 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 73],
                        &[i8; 73],
                    >(
                        b"int yaml_parser_load_node_add(yaml_parser_t *, struct loader_ctx *, int)\0",
                    ))
                        .as_ptr(),
                );
            };
            return 0 as i32;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_load_alias(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut anchor: *mut yaml_char_t = (*event).data.alias.anchor;
    let mut alias_data: *mut yaml_alias_data_t = 0 as *mut yaml_alias_data_t;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp((*alias_data).anchor as *mut i8, anchor as *mut i8) == 0 as i32 {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.offset(1);
        alias_data;
    }
    yaml_free(anchor as *mut libc::c_void);
    return yaml_parser_set_composer_error(
        parser,
        b"found undefined alias\0" as *const u8 as *const i8,
        (*event).start_mark,
    );
}
unsafe extern "C" fn yaml_parser_load_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
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
    let mut index: i32 = 0;
    let mut tag: *mut yaml_char_t = (*event).data.scalar.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as i64)
        < (2147483647 as i32 - 1 as i32) as i64
    {
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if tag.is_null()
            || strcmp(tag as *mut i8, b"!\0" as *const u8 as *const i8) == 0 as i32
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:str\0" as *const u8 as *const i8 as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 12517232041451633041;
            } else {
                current_block = 15427931788582360902;
            }
        } else {
            current_block = 15427931788582360902;
        }
        match current_block {
            12517232041451633041 => {}
            _ => {
                memset(
                    &mut node as *mut yaml_node_t as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<yaml_node_t>() as u64,
                );
                node.type_0 = YAML_SCALAR_NODE;
                node.tag = tag;
                node.start_mark = (*event).start_mark;
                node.end_mark = (*event).end_mark;
                node.data.scalar.value = (*event).data.scalar.value;
                node.data.scalar.length = (*event).data.scalar.length;
                node.data.scalar.style = (*event).data.scalar.style;
                if !(if (*(*parser).document).nodes.top
                    != (*(*parser).document).nodes.end
                    || yaml_stack_extend(
                        &mut (*(*parser).document).nodes.start as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                        &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                            as *mut *mut libc::c_void,
                    ) != 0
                {
                    let fresh3 = (*(*parser).document).nodes.top;
                    (*(*parser).document).nodes.top = ((*(*parser).document).nodes.top)
                        .offset(1);
                    *fresh3 = node;
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0)
                {
                    index = ((*(*parser).document).nodes.top)
                        .offset_from((*(*parser).document).nodes.start) as i64 as i32;
                    if yaml_parser_register_anchor(
                        parser,
                        index,
                        (*event).data.scalar.anchor,
                    ) == 0
                    {
                        return 0 as i32;
                    }
                    return yaml_parser_load_node_add(parser, ctx, index);
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
    yaml_free((*event).data.scalar.value as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_load_sequence(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
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
    let mut items: C2RustUnnamed_36 = {
        let mut init = C2RustUnnamed_36 {
            start: 0 as *mut yaml_node_item_t,
            end: 0 as *mut yaml_node_item_t,
            top: 0 as *mut yaml_node_item_t,
        };
        init
    };
    let mut index: i32 = 0;
    let mut tag: *mut yaml_char_t = (*event).data.sequence_start.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as i64)
        < (2147483647 as i32 - 1 as i32) as i64
    {
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if tag.is_null()
            || strcmp(tag as *mut i8, b"!\0" as *const u8 as *const i8) == 0 as i32
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:seq\0" as *const u8 as *const i8 as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 9656768690171235327;
            } else {
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            9656768690171235327 => {}
            _ => {
                items.start = yaml_malloc(
                    (16 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<yaml_node_item_t>() as u64),
                ) as *mut yaml_node_item_t;
                if !(if !(items.start).is_null() {
                    items.top = items.start;
                    items.end = (items.start).offset(16 as i32 as isize);
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0)
                {
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as i32,
                        ::core::mem::size_of::<yaml_node_t>() as u64,
                    );
                    node.type_0 = YAML_SEQUENCE_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.sequence.items.start = items.start;
                    node.data.sequence.items.end = items.end;
                    node.data.sequence.items.top = items.start;
                    node.data.sequence.style = (*event).data.sequence_start.style;
                    if !(if (*(*parser).document).nodes.top
                        != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &mut (*(*parser).document).nodes.start
                                as *mut *mut yaml_node_t as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh4 = (*(*parser).document).nodes.top;
                        (*(*parser).document).nodes.top = ((*(*parser).document)
                            .nodes
                            .top)
                            .offset(1);
                        *fresh4 = node;
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .offset_from((*(*parser).document).nodes.start) as i64
                            as i32;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.sequence_start.anchor,
                        ) == 0
                        {
                            return 0 as i32;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as i32;
                        }
                        if if (((*ctx).top).offset_from((*ctx).start) as i64)
                            < (2147483647 as i32 - 1 as i32) as i64
                        {
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } == 0
                        {
                            return 0 as i32;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &mut (*ctx).start as *mut *mut i32
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).top as *mut *mut i32 as *mut *mut libc::c_void,
                                &mut (*ctx).end as *mut *mut i32 as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh5 = (*ctx).top;
                            (*ctx).top = ((*ctx).top).offset(1);
                            *fresh5 = index;
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } == 0
                        {
                            return 0 as i32;
                        }
                        return 1 as i32;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_load_sequence_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut index: i32 = 0;
    if ((*ctx).top).offset_from((*ctx).start) as i64 > 0 as i32 as i64 {} else {
        __assert_fail(
            b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            467 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[i8; 88],
            >(
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4755: {
        if ((*ctx).top).offset_from((*ctx).start) as i64 > 0 as i32 as i64 {} else {
            __assert_fail(
                b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                467 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[i8; 88],
                >(
                    b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    index = *((*ctx).top).offset(-(1 as i32 as isize));
    if (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize)).type_0
        as u32 == YAML_SEQUENCE_NODE as i32 as u32
    {} else {
        __assert_fail(
            b"parser->document->nodes.start[index-1].type == YAML_SEQUENCE_NODE\0"
                as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            470 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 88],
                &[i8; 88],
            >(
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4676: {
        if (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize))
            .type_0 as u32 == YAML_SEQUENCE_NODE as i32 as u32
        {} else {
            __assert_fail(
                b"parser->document->nodes.start[index-1].type == YAML_SEQUENCE_NODE\0"
                    as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                470 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 88],
                    &[i8; 88],
                >(
                    b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize))
        .end_mark = (*event).end_mark;
    (*ctx).top = ((*ctx).top).offset(-1);
    *(*ctx).top;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_load_mapping(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: 0 as *mut yaml_char_t,
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: 0 as *mut yaml_char_t,
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
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
    let mut pairs: C2RustUnnamed_35 = {
        let mut init = C2RustUnnamed_35 {
            start: 0 as *mut yaml_node_pair_t,
            end: 0 as *mut yaml_node_pair_t,
            top: 0 as *mut yaml_node_pair_t,
        };
        init
    };
    let mut index: i32 = 0;
    let mut tag: *mut yaml_char_t = (*event).data.mapping_start.tag;
    if !(if (((*(*parser).document).nodes.top)
        .offset_from((*(*parser).document).nodes.start) as i64)
        < (2147483647 as i32 - 1 as i32) as i64
    {
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if tag.is_null()
            || strcmp(tag as *mut i8, b"!\0" as *const u8 as *const i8) == 0 as i32
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:map\0" as *const u8 as *const i8 as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 1053349063681702383;
            } else {
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            1053349063681702383 => {}
            _ => {
                pairs.start = yaml_malloc(
                    (16 as i32 as u64)
                        .wrapping_mul(::core::mem::size_of::<yaml_node_pair_t>() as u64),
                ) as *mut yaml_node_pair_t;
                if !(if !(pairs.start).is_null() {
                    pairs.top = pairs.start;
                    pairs.end = (pairs.start).offset(16 as i32 as isize);
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0)
                {
                    memset(
                        &mut node as *mut yaml_node_t as *mut libc::c_void,
                        0 as i32,
                        ::core::mem::size_of::<yaml_node_t>() as u64,
                    );
                    node.type_0 = YAML_MAPPING_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.mapping.pairs.start = pairs.start;
                    node.data.mapping.pairs.end = pairs.end;
                    node.data.mapping.pairs.top = pairs.start;
                    node.data.mapping.style = (*event).data.mapping_start.style;
                    if !(if (*(*parser).document).nodes.top
                        != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &mut (*(*parser).document).nodes.start
                                as *mut *mut yaml_node_t as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.top as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                            &mut (*(*parser).document).nodes.end as *mut *mut yaml_node_t
                                as *mut *mut libc::c_void,
                        ) != 0
                    {
                        let fresh6 = (*(*parser).document).nodes.top;
                        (*(*parser).document).nodes.top = ((*(*parser).document)
                            .nodes
                            .top)
                            .offset(1);
                        *fresh6 = node;
                        1 as i32
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as i32
                    } == 0)
                    {
                        index = ((*(*parser).document).nodes.top)
                            .offset_from((*(*parser).document).nodes.start) as i64
                            as i32;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.mapping_start.anchor,
                        ) == 0
                        {
                            return 0 as i32;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as i32;
                        }
                        if if (((*ctx).top).offset_from((*ctx).start) as i64)
                            < (2147483647 as i32 - 1 as i32) as i64
                        {
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } == 0
                        {
                            return 0 as i32;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &mut (*ctx).start as *mut *mut i32
                                    as *mut *mut libc::c_void,
                                &mut (*ctx).top as *mut *mut i32 as *mut *mut libc::c_void,
                                &mut (*ctx).end as *mut *mut i32 as *mut *mut libc::c_void,
                            ) != 0
                        {
                            let fresh7 = (*ctx).top;
                            (*ctx).top = ((*ctx).top).offset(1);
                            *fresh7 = index;
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } == 0
                        {
                            return 0 as i32;
                        }
                        return 1 as i32;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_load_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> i32 {
    let mut index: i32 = 0;
    if ((*ctx).top).offset_from((*ctx).start) as i64 > 0 as i32 as i64 {} else {
        __assert_fail(
            b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            535 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[i8; 87],
            >(
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3318: {
        if ((*ctx).top).offset_from((*ctx).start) as i64 > 0 as i32 as i64 {} else {
            __assert_fail(
                b"((*ctx).top - (*ctx).start) > 0\0" as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                535 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[i8; 87],
                >(
                    b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    index = *((*ctx).top).offset(-(1 as i32 as isize));
    if (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize)).type_0
        as u32 == YAML_MAPPING_NODE as i32 as u32
    {} else {
        __assert_fail(
            b"parser->document->nodes.start[index-1].type == YAML_MAPPING_NODE\0"
                as *const u8 as *const i8,
            b"loader.c\0" as *const u8 as *const i8,
            538 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 87],
                &[i8; 87],
            >(
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3240: {
        if (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize))
            .type_0 as u32 == YAML_MAPPING_NODE as i32 as u32
        {} else {
            __assert_fail(
                b"parser->document->nodes.start[index-1].type == YAML_MAPPING_NODE\0"
                    as *const u8 as *const i8,
                b"loader.c\0" as *const u8 as *const i8,
                538 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 87],
                    &[i8; 87],
                >(
                    b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    (*((*(*parser).document).nodes.start).offset((index - 1 as i32) as isize))
        .end_mark = (*event).end_mark;
    (*ctx).top = ((*ctx).top).offset(-1);
    *(*ctx).top;
    return 1 as i32;
}