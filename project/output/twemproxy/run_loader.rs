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
    fn yaml_parser_load(
        parser: *mut yaml_parser_t,
        document: *mut yaml_document_t,
    ) -> i32;
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32;
    fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn printf(_: *const i8, _: ...) -> i32;
    fn yaml_document_delete(document: *mut yaml_document_t);
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
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub scalar: C2RustUnnamed_12,
    pub sequence: C2RustUnnamed_10,
    pub mapping: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pairs: C2RustUnnamed_9,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
    pub items: C2RustUnnamed_11,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_14,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_13,
    pub start_implicit: i32,
    pub end_implicit: i32,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub input: C2RustUnnamed_24,
    pub eof: i32,
    pub buffer: C2RustUnnamed_23,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_22,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: i32,
    pub stream_end_produced: i32,
    pub flow_level: i32,
    pub tokens: C2RustUnnamed_21,
    pub tokens_parsed: size_t,
    pub token_available: i32,
    pub indents: C2RustUnnamed_20,
    pub indent: i32,
    pub simple_key_allowed: i32,
    pub simple_keys: C2RustUnnamed_19,
    pub states: C2RustUnnamed_18,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_17,
    pub tag_directives: C2RustUnnamed_16,
    pub aliases: C2RustUnnamed_15,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub string: C2RustUnnamed_25,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *const u8,
    pub end: *const u8,
    pub current: *const u8,
}
pub type yaml_parser_t = yaml_parser_s;
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut number: i32 = 0;
    if argc < 2 as i32 {
        printf(
            b"Usage: %s file1.yaml ...\n\0" as *const u8 as *const i8,
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
            input: C2RustUnnamed_24 {
                string: C2RustUnnamed_25 {
                    start: 0 as *const u8,
                    end: 0 as *const u8,
                    current: 0 as *const u8,
                },
            },
            eof: 0,
            buffer: C2RustUnnamed_23 {
                start: 0 as *mut yaml_char_t,
                end: 0 as *mut yaml_char_t,
                pointer: 0 as *mut yaml_char_t,
                last: 0 as *mut yaml_char_t,
            },
            unread: 0,
            raw_buffer: C2RustUnnamed_22 {
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
            tokens: C2RustUnnamed_21 {
                start: 0 as *mut yaml_token_t,
                end: 0 as *mut yaml_token_t,
                head: 0 as *mut yaml_token_t,
                tail: 0 as *mut yaml_token_t,
            },
            tokens_parsed: 0,
            token_available: 0,
            indents: C2RustUnnamed_20 {
                start: 0 as *mut i32,
                end: 0 as *mut i32,
                top: 0 as *mut i32,
            },
            indent: 0,
            simple_key_allowed: 0,
            simple_keys: C2RustUnnamed_19 {
                start: 0 as *mut yaml_simple_key_t,
                end: 0 as *mut yaml_simple_key_t,
                top: 0 as *mut yaml_simple_key_t,
            },
            states: C2RustUnnamed_18 {
                start: 0 as *mut yaml_parser_state_t,
                end: 0 as *mut yaml_parser_state_t,
                top: 0 as *mut yaml_parser_state_t,
            },
            state: YAML_PARSE_STREAM_START_STATE,
            marks: C2RustUnnamed_17 {
                start: 0 as *mut yaml_mark_t,
                end: 0 as *mut yaml_mark_t,
                top: 0 as *mut yaml_mark_t,
            },
            tag_directives: C2RustUnnamed_16 {
                start: 0 as *mut yaml_tag_directive_t,
                end: 0 as *mut yaml_tag_directive_t,
                top: 0 as *mut yaml_tag_directive_t,
            },
            aliases: C2RustUnnamed_15 {
                start: 0 as *mut yaml_alias_data_t,
                end: 0 as *mut yaml_alias_data_t,
                top: 0 as *mut yaml_alias_data_t,
            },
            document: 0 as *mut yaml_document_t,
        };
        let mut document: yaml_document_t = yaml_document_t {
            nodes: C2RustUnnamed_14 {
                start: 0 as *mut yaml_node_t,
                end: 0 as *mut yaml_node_t,
                top: 0 as *mut yaml_node_t,
            },
            version_directive: 0 as *mut yaml_version_directive_t,
            tag_directives: C2RustUnnamed_13 {
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
        let mut done: i32 = 0 as i32;
        let mut count: i32 = 0 as i32;
        let mut error: i32 = 0 as i32;
        printf(
            b"[%d] Loading '%s': \0" as *const u8 as *const i8,
            number,
            *argv.offset(number as isize),
        );
        fflush(stdout);
        file = fopen(*argv.offset(number as isize), b"rb\0" as *const u8 as *const i8);
        if !file.is_null() {} else {
            __assert_fail(
                b"file\0" as *const u8 as *const i8,
                b"run-loader.c\0" as *const u8 as *const i8,
                34 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_3266: {
            if !file.is_null() {} else {
                __assert_fail(
                    b"file\0" as *const u8 as *const i8,
                    b"run-loader.c\0" as *const u8 as *const i8,
                    34 as i32 as u32,
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
                b"run-loader.c\0" as *const u8 as *const i8,
                36 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_3226: {
            if yaml_parser_initialize(&mut parser) != 0 {} else {
                __assert_fail(
                    b"yaml_parser_initialize(&parser)\0" as *const u8 as *const i8,
                    b"run-loader.c\0" as *const u8 as *const i8,
                    36 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        yaml_parser_set_input_file(&mut parser, file);
        while done == 0 {
            if yaml_parser_load(&mut parser, &mut document) == 0 {
                error = 1 as i32;
                break;
            } else {
                done = (yaml_document_get_root_node(&mut document)).is_null() as i32;
                yaml_document_delete(&mut document);
                if done == 0 {
                    count += 1;
                    count;
                }
            }
        }
        yaml_parser_delete(&mut parser);
        if fclose(file) == 0 {} else {
            __assert_fail(
                b"!fclose(file)\0" as *const u8 as *const i8,
                b"run-loader.c\0" as *const u8 as *const i8,
                56 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
        'c_3116: {
            if fclose(file) == 0 {} else {
                __assert_fail(
                    b"!fclose(file)\0" as *const u8 as *const i8,
                    b"run-loader.c\0" as *const u8 as *const i8,
                    56 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
        };
        printf(
            b"%s (%d documents)\n\0" as *const u8 as *const i8,
            if error != 0 {
                b"FAILURE\0" as *const u8 as *const i8
            } else {
                b"SUCCESS\0" as *const u8 as *const i8
            },
            count,
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