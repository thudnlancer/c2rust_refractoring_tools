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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn yaml_token_delete(token: *mut yaml_token_t);
    fn yaml_queue_extend(
        start: *mut *mut libc::c_void,
        head: *mut *mut libc::c_void,
        tail: *mut *mut libc::c_void,
        end: *mut *mut libc::c_void,
    ) -> i32;
    fn yaml_free(ptr: *mut libc::c_void);
    fn yaml_parser_update_buffer(parser: *mut yaml_parser_t, length: size_t) -> i32;
    fn yaml_string_extend(
        start: *mut *mut yaml_char_t,
        pointer: *mut *mut yaml_char_t,
        end: *mut *mut yaml_char_t,
    ) -> i32;
    fn yaml_string_join(
        a_start: *mut *mut yaml_char_t,
        a_pointer: *mut *mut yaml_char_t,
        a_end: *mut *mut yaml_char_t,
        b_start: *mut *mut yaml_char_t,
        b_pointer: *mut *mut yaml_char_t,
        b_end: *mut *mut yaml_char_t,
    ) -> i32;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
pub type ptrdiff_t = i64;
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> i32 {
    if !parser.is_null() {} else {
        __assert_fail(
            b"parser\0" as *const u8 as *const i8,
            b"scanner.c\0" as *const u8 as *const i8,
            745 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[i8; 54],
            >(b"int yaml_parser_scan(yaml_parser_t *, yaml_token_t *)\0"))
                .as_ptr(),
        );
    }
    'c_39769: {
        if !parser.is_null() {} else {
            __assert_fail(
                b"parser\0" as *const u8 as *const i8,
                b"scanner.c\0" as *const u8 as *const i8,
                745 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"int yaml_parser_scan(yaml_parser_t *, yaml_token_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !token.is_null() {} else {
        __assert_fail(
            b"token\0" as *const u8 as *const i8,
            b"scanner.c\0" as *const u8 as *const i8,
            746 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 54],
                &[i8; 54],
            >(b"int yaml_parser_scan(yaml_parser_t *, yaml_token_t *)\0"))
                .as_ptr(),
        );
    }
    'c_39727: {
        if !token.is_null() {} else {
            __assert_fail(
                b"token\0" as *const u8 as *const i8,
                b"scanner.c\0" as *const u8 as *const i8,
                746 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"int yaml_parser_scan(yaml_parser_t *, yaml_token_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        token as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as u32 != 0 {
        return 1 as i32;
    }
    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0 as i32;
        }
    }
    let fresh0 = (*parser).tokens.head;
    (*parser).tokens.head = ((*parser).tokens.head).offset(1);
    *token = *fresh0;
    (*parser).token_available = 0 as i32;
    (*parser).tokens_parsed = ((*parser).tokens_parsed).wrapping_add(1);
    (*parser).tokens_parsed;
    if (*token).type_0 as u32 == YAML_STREAM_END_TOKEN as i32 as u32 {
        (*parser).stream_end_produced = 1 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_set_scanner_error(
    mut parser: *mut yaml_parser_t,
    mut context: *const i8,
    mut context_mark: yaml_mark_t,
    mut problem: *const i8,
) -> i32 {
    (*parser).error = YAML_SCANNER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = (*parser).mark;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_fetch_more_tokens(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut need_more_tokens: i32 = 0;
    loop {
        need_more_tokens = 0 as i32;
        if (*parser).tokens.head == (*parser).tokens.tail {
            need_more_tokens = 1 as i32;
        } else {
            let mut simple_key: *mut yaml_simple_key_t = 0 as *mut yaml_simple_key_t;
            if yaml_parser_stale_simple_keys(parser) == 0 {
                return 0 as i32;
            }
            simple_key = (*parser).simple_keys.start;
            while simple_key != (*parser).simple_keys.top {
                if (*simple_key).possible != 0
                    && (*simple_key).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1 as i32;
                    break;
                } else {
                    simple_key = simple_key.offset(1);
                    simple_key;
                }
            }
        }
        if need_more_tokens == 0 {
            break;
        }
        if yaml_parser_fetch_next_token(parser) == 0 {
            return 0 as i32;
        }
    }
    (*parser).token_available = 1 as i32;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_next_token(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    if if (*parser).unread >= 1 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
    } == 0
    {
        return 0 as i32;
    }
    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }
    if yaml_parser_scan_to_next_token(parser) == 0 {
        return 0 as i32;
    }
    if yaml_parser_stale_simple_keys(parser) == 0 {
        return 0 as i32;
    }
    if yaml_parser_unroll_indent(parser, (*parser).mark.column as ptrdiff_t) == 0 {
        return 0 as i32;
    }
    if if (*parser).unread >= 4 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 4 as i32 as size_t)
    } == 0
    {
        return 0 as i32;
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '\0' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_stream_end(parser);
    }
    if (*parser).mark.column == 0 as i32 as u64
        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '%' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_directive(parser);
    }
    if (*parser).mark.column == 0 as i32 as u64
        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '-' as i32 as yaml_char_t as i32
        && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
            == '-' as i32 as yaml_char_t as i32
        && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
            == '-' as i32 as yaml_char_t as i32
        && (*((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
            == ' ' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                == '\t' as i32 as yaml_char_t as i32
            || (*((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -62i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -123i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 2 as i32) as isize)
                        as i32 == -88i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 2 as i32) as isize)
                        as i32 == -87i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == '\0' as i32 as yaml_char_t as i32))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }
    if (*parser).mark.column == 0 as i32 as u64
        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '.' as i32 as yaml_char_t as i32
        && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
            == '.' as i32 as yaml_char_t as i32
        && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
            == '.' as i32 as yaml_char_t as i32
        && (*((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
            == ' ' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                == '\t' as i32 as yaml_char_t as i32
            || (*((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -62i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -123i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 2 as i32) as isize)
                        as i32 == -88i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((3 as i32 + 2 as i32) as isize)
                        as i32 == -87i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(3 as i32 as isize) as i32
                    == '\0' as i32 as yaml_char_t as i32))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '[' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_collection_start(
            parser,
            YAML_FLOW_SEQUENCE_START_TOKEN,
        );
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '{' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_collection_start(
            parser,
            YAML_FLOW_MAPPING_START_TOKEN,
        );
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == ']' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_collection_end(
            parser,
            YAML_FLOW_SEQUENCE_END_TOKEN,
        );
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '}' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_collection_end(
            parser,
            YAML_FLOW_MAPPING_END_TOKEN,
        );
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == ',' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '-' as i32 as yaml_char_t as i32
        && (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
            == ' ' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == '\t' as i32 as yaml_char_t as i32
            || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == -62i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((1 as i32 + 1 as i32) as isize)
                        as i32 == -123i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((1 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((1 as i32 + 2 as i32) as isize)
                        as i32 == -88i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((1 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((1 as i32 + 2 as i32) as isize)
                        as i32 == -87i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\0' as i32 as yaml_char_t as i32))
    {
        return yaml_parser_fetch_block_entry(parser);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '?' as i32 as yaml_char_t as i32
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == ' ' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\t' as i32 as yaml_char_t as i32
                || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\r' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\n' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -62i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -123i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -88i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -87i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\0' as i32 as yaml_char_t as i32)))
    {
        return yaml_parser_fetch_key(parser);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == ':' as i32 as yaml_char_t as i32
        && ((*parser).flow_level != 0
            || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == ' ' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\t' as i32 as yaml_char_t as i32
                || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\r' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\n' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -62i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -123i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -88i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -87i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\0' as i32 as yaml_char_t as i32)))
    {
        return yaml_parser_fetch_value(parser);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '*' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '&' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '!' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_tag(parser);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '|' as i32 as yaml_char_t as i32 && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 1 as i32);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '>' as i32 as yaml_char_t as i32 && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 0 as i32);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '\'' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_scalar(parser, 1 as i32);
    }
    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '"' as i32 as yaml_char_t as i32
    {
        return yaml_parser_fetch_flow_scalar(parser, 0 as i32);
    }
    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == ' ' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\t' as i32 as yaml_char_t as i32
        || (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\r' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\n' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -62i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -123i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -88i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -87i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\0' as i32 as yaml_char_t as i32)
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '-' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '?' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == ':' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == ',' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '[' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == ']' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '{' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '}' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '#' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '&' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '*' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '!' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '|' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '>' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\'' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '"' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '%' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '@' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '`' as i32 as yaml_char_t as i32)
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '-' as i32 as yaml_char_t as i32
            && !(*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == ' ' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\t' as i32 as yaml_char_t as i32)
        || (*parser).flow_level == 0
            && (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '?' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == ':' as i32 as yaml_char_t as i32)
            && !(*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                == ' ' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\t' as i32 as yaml_char_t as i32
                || (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\r' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\n' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -62i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -123i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -88i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == -30i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 1 as i32) as isize) as i32
                            == -128i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer)
                            .offset((1 as i32 + 2 as i32) as isize) as i32
                            == -87i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        == '\0' as i32 as yaml_char_t as i32))
    {
        return yaml_parser_fetch_plain_scalar(parser);
    }
    return yaml_parser_set_scanner_error(
        parser,
        b"while scanning for the next token\0" as *const u8 as *const i8,
        (*parser).mark,
        b"found character that cannot start any token\0" as *const u8 as *const i8,
    );
}
unsafe extern "C" fn yaml_parser_stale_simple_keys(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut simple_key: *mut yaml_simple_key_t = 0 as *mut yaml_simple_key_t;
    simple_key = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || ((*simple_key).mark.index).wrapping_add(1024 as i32 as u64)
                    < (*parser).mark.index)
        {
            if (*simple_key).required != 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a simple key\0" as *const u8 as *const i8,
                    (*simple_key).mark,
                    b"could not find expected ':'\0" as *const u8 as *const i8,
                );
            }
            (*simple_key).possible = 0 as i32;
        }
        simple_key = simple_key.offset(1);
        simple_key;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_save_simple_key(mut parser: *mut yaml_parser_t) -> i32 {
    let mut required: i32 = ((*parser).flow_level == 0
        && (*parser).indent as i64 == (*parser).mark.column as ptrdiff_t) as i32;
    if (*parser).simple_key_allowed != 0 {
        let mut simple_key: yaml_simple_key_t = yaml_simple_key_t {
            possible: 0,
            required: 0,
            token_number: 0,
            mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
        };
        simple_key.possible = 1 as i32;
        simple_key.required = required;
        simple_key.token_number = ((*parser).tokens_parsed)
            .wrapping_add(
                ((*parser).tokens.tail).offset_from((*parser).tokens.head) as i64 as u64,
            );
        simple_key.mark = (*parser).mark;
        if yaml_parser_remove_simple_key(parser) == 0 {
            return 0 as i32;
        }
        *((*parser).simple_keys.top).offset(-(1 as i32 as isize)) = simple_key;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_remove_simple_key(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut simple_key: *mut yaml_simple_key_t = ((*parser).simple_keys.top)
        .offset(-(1 as i32 as isize));
    if (*simple_key).possible != 0 {
        if (*simple_key).required != 0 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a simple key\0" as *const u8 as *const i8,
                (*simple_key).mark,
                b"could not find expected ':'\0" as *const u8 as *const i8,
            );
        }
    }
    (*simple_key).possible = 0 as i32;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_increase_flow_level(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut empty_simple_key: yaml_simple_key_t = {
        let mut init = yaml_simple_key_s {
            possible: 0 as i32,
            required: 0 as i32,
            token_number: 0 as i32 as size_t,
            mark: {
                let mut init = yaml_mark_s {
                    index: 0 as i32 as size_t,
                    line: 0 as i32 as size_t,
                    column: 0 as i32 as size_t,
                };
                init
            },
        };
        init
    };
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            &mut (*parser).simple_keys.start as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
            &mut (*parser).simple_keys.top as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
            &mut (*parser).simple_keys.end as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh1 = (*parser).simple_keys.top;
        (*parser).simple_keys.top = ((*parser).simple_keys.top).offset(1);
        *fresh1 = empty_simple_key;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        return 0 as i32;
    }
    if (*parser).flow_level == 2147483647 as i32 {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0 as i32;
    }
    (*parser).flow_level += 1;
    (*parser).flow_level;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_decrease_flow_level(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    if (*parser).flow_level != 0 {
        (*parser).flow_level -= 1;
        (*parser).flow_level;
        (*parser).simple_keys.top = ((*parser).simple_keys.top).offset(-1);
        *(*parser).simple_keys.top;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_roll_indent(
    mut parser: *mut yaml_parser_t,
    mut column: ptrdiff_t,
    mut number: ptrdiff_t,
    mut type_0: yaml_token_type_t,
    mut mark: yaml_mark_t,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level != 0 {
        return 1 as i32;
    }
    if ((*parser).indent as i64) < column {
        if if (*parser).indents.top != (*parser).indents.end
            || yaml_stack_extend(
                &mut (*parser).indents.start as *mut *mut i32 as *mut *mut libc::c_void,
                &mut (*parser).indents.top as *mut *mut i32 as *mut *mut libc::c_void,
                &mut (*parser).indents.end as *mut *mut i32 as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh2 = (*parser).indents.top;
            (*parser).indents.top = ((*parser).indents.top).offset(1);
            *fresh2 = (*parser).indent;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        if column > 2147483647 as i32 as i64 {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0 as i32;
        }
        (*parser).indent = column as i32;
        memset(
            &mut token as *mut yaml_token_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_token_t>() as u64,
        );
        token.type_0 = type_0;
        token.start_mark = mark;
        token.end_mark = mark;
        if number == -(1 as i32) as i64 {
            if if (*parser).tokens.tail != (*parser).tokens.end
                || yaml_queue_extend(
                    &mut (*parser).tokens.start as *mut *mut yaml_token_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).tokens.head as *mut *mut yaml_token_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                        as *mut *mut libc::c_void,
                    &mut (*parser).tokens.end as *mut *mut yaml_token_t
                        as *mut *mut libc::c_void,
                ) != 0
            {
                let fresh3 = (*parser).tokens.tail;
                (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
                *fresh3 = token;
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0
            {
                return 0 as i32;
            }
        } else if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &mut (*parser).tokens.start as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.head as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.end as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .offset(
                        (number as u64).wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .offset(1 as i32 as isize) as *mut libc::c_void,
                ((*parser).tokens.head)
                    .offset(
                        (number as u64).wrapping_sub((*parser).tokens_parsed) as isize,
                    ) as *const libc::c_void,
                (((*parser).tokens.tail).offset_from((*parser).tokens.head) as i64
                    as u64)
                    .wrapping_sub((number as u64).wrapping_sub((*parser).tokens_parsed))
                    .wrapping_mul(::core::mem::size_of::<yaml_token_t>() as u64),
            );
            *((*parser).tokens.head)
                .offset(
                    (number as u64).wrapping_sub((*parser).tokens_parsed) as isize,
                ) = token;
            (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
            (*parser).tokens.tail;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_unroll_indent(
    mut parser: *mut yaml_parser_t,
    mut column: ptrdiff_t,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level != 0 {
        return 1 as i32;
    }
    while (*parser).indent as i64 > column {
        memset(
            &mut token as *mut yaml_token_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_token_t>() as u64,
        );
        token.type_0 = YAML_BLOCK_END_TOKEN;
        token.start_mark = (*parser).mark;
        token.end_mark = (*parser).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &mut (*parser).tokens.start as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.head as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.end as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            let fresh4 = (*parser).tokens.tail;
            (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
            *fresh4 = token;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        (*parser).indents.top = ((*parser).indents.top).offset(-1);
        (*parser).indent = *(*parser).indents.top;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_stream_start(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut simple_key: yaml_simple_key_t = {
        let mut init = yaml_simple_key_s {
            possible: 0 as i32,
            required: 0 as i32,
            token_number: 0 as i32 as size_t,
            mark: {
                let mut init = yaml_mark_s {
                    index: 0 as i32 as size_t,
                    line: 0 as i32 as size_t,
                    column: 0 as i32 as size_t,
                };
                init
            },
        };
        init
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    (*parser).indent = -(1 as i32);
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            &mut (*parser).simple_keys.start as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
            &mut (*parser).simple_keys.top as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
            &mut (*parser).simple_keys.end as *mut *mut yaml_simple_key_t
                as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh5 = (*parser).simple_keys.top;
        (*parser).simple_keys.top = ((*parser).simple_keys.top).offset(1);
        *fresh5 = simple_key;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 1 as i32;
    (*parser).stream_start_produced = 1 as i32;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_STREAM_START_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    token.data.stream_start.encoding = (*parser).encoding;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh6 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh6 = token;
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
unsafe extern "C" fn yaml_parser_fetch_stream_end(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).mark.column != 0 as i32 as u64 {
        (*parser).mark.column = 0 as i32 as size_t;
        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
        (*parser).mark.line;
    }
    if yaml_parser_unroll_indent(parser, -(1 as i32) as ptrdiff_t) == 0 {
        return 0 as i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_STREAM_END_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh7 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh7 = token;
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
unsafe extern "C" fn yaml_parser_fetch_directive(mut parser: *mut yaml_parser_t) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_unroll_indent(parser, -(1 as i32) as ptrdiff_t) == 0 {
        return 0 as i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    if yaml_parser_scan_directive(parser, &mut token) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh8 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh8 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_document_indicator(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_unroll_indent(parser, -(1 as i32) as ptrdiff_t) == 0 {
        return 0 as i32;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh9 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh9 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_collection_start(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as i32;
    }
    if yaml_parser_increase_flow_level(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 1 as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh10 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh10 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_collection_end(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    if yaml_parser_decrease_flow_level(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh11 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh11 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_entry(
    mut parser: *mut yaml_parser_t,
) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 1 as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_FLOW_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh12 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh12 = token;
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
unsafe extern "C" fn yaml_parser_fetch_block_entry(
    mut parser: *mut yaml_parser_t,
) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                0 as *const i8,
                (*parser).mark,
                b"block sequence entries are not allowed in this context\0" as *const u8
                    as *const i8,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -(1 as i32) as ptrdiff_t,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as i32;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 1 as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_BLOCK_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh13 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh13 = token;
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
unsafe extern "C" fn yaml_parser_fetch_key(mut parser: *mut yaml_parser_t) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                0 as *const i8,
                (*parser).mark,
                b"mapping keys are not allowed in this context\0" as *const u8
                    as *const i8,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -(1 as i32) as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as i32;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as i32;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_KEY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh14 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh14 = token;
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
unsafe extern "C" fn yaml_parser_fetch_value(mut parser: *mut yaml_parser_t) -> i32 {
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
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    let mut simple_key: *mut yaml_simple_key_t = ((*parser).simple_keys.top)
        .offset(-(1 as i32 as isize));
    if (*simple_key).possible != 0 {
        memset(
            &mut token as *mut yaml_token_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<yaml_token_t>() as u64,
        );
        token.type_0 = YAML_KEY_TOKEN;
        token.start_mark = (*simple_key).mark;
        token.end_mark = (*simple_key).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &mut (*parser).tokens.start as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.head as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
                &mut (*parser).tokens.end as *mut *mut yaml_token_t
                    as *mut *mut libc::c_void,
            ) != 0
        {
            memmove(
                ((*parser).tokens.head)
                    .offset(
                        ((*simple_key).token_number)
                            .wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .offset(1 as i32 as isize) as *mut libc::c_void,
                ((*parser).tokens.head)
                    .offset(
                        ((*simple_key).token_number)
                            .wrapping_sub((*parser).tokens_parsed) as isize,
                    ) as *const libc::c_void,
                (((*parser).tokens.tail).offset_from((*parser).tokens.head) as i64
                    as u64)
                    .wrapping_sub(
                        ((*simple_key).token_number)
                            .wrapping_sub((*parser).tokens_parsed),
                    )
                    .wrapping_mul(::core::mem::size_of::<yaml_token_t>() as u64),
            );
            *((*parser).tokens.head)
                .offset(
                    ((*simple_key).token_number).wrapping_sub((*parser).tokens_parsed)
                        as isize,
                ) = token;
            (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
            (*parser).tokens.tail;
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as ptrdiff_t,
            (*simple_key).token_number as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0
        {
            return 0 as i32;
        }
        (*simple_key).possible = 0 as i32;
        (*parser).simple_key_allowed = 0 as i32;
    } else {
        if (*parser).flow_level == 0 {
            if (*parser).simple_key_allowed == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    0 as *const i8,
                    (*parser).mark,
                    b"mapping values are not allowed in this context\0" as *const u8
                        as *const i8,
                );
            }
            if yaml_parser_roll_indent(
                parser,
                (*parser).mark.column as ptrdiff_t,
                -(1 as i32) as ptrdiff_t,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0
            {
                return 0 as i32;
            }
        }
        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as i32;
    }
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    end_mark = (*parser).mark;
    memset(
        &mut token as *mut yaml_token_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_token_t>() as u64,
    );
    token.type_0 = YAML_VALUE_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh15 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh15 = token;
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
unsafe extern "C" fn yaml_parser_fetch_anchor(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    if yaml_parser_scan_anchor(parser, &mut token, type_0) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh16 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh16 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_tag(mut parser: *mut yaml_parser_t) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    if yaml_parser_scan_tag(parser, &mut token) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh17 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh17 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut literal: i32,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 1 as i32;
    if yaml_parser_scan_block_scalar(parser, &mut token, literal) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh18 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh18 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut single: i32,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    if yaml_parser_scan_flow_scalar(parser, &mut token, single) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh19 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh19 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_fetch_plain_scalar(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as i32;
    }
    (*parser).simple_key_allowed = 0 as i32;
    if yaml_parser_scan_plain_scalar(parser, &mut token) == 0 {
        return 0 as i32;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &mut (*parser).tokens.start as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.head as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.tail as *mut *mut yaml_token_t
                as *mut *mut libc::c_void,
            &mut (*parser).tokens.end as *mut *mut yaml_token_t as *mut *mut libc::c_void,
        ) != 0
    {
        let fresh20 = (*parser).tokens.tail;
        (*parser).tokens.tail = ((*parser).tokens.tail).offset(1);
        *fresh20 = token;
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        yaml_token_delete(&mut token);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_to_next_token(
    mut parser: *mut yaml_parser_t,
) -> i32 {
    loop {
        if if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        if (*parser).mark.column == 0 as i32 as u64
            && (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -17i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -69i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -65i32 as yaml_char_t as i32)
        {
            (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
            (*parser).mark.index;
            (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
            (*parser).mark.column;
            (*parser).unread = ((*parser).unread).wrapping_sub(1);
            (*parser).unread;
            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                .offset(
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0x80 as i32 == 0 as i32
                    {
                        1 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xe0 as i32 == 0xc0 as i32
                        {
                            2 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf0 as i32 == 0xe0 as i32
                            {
                                3 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                {
                                    4 as i32
                                } else {
                                    0 as i32
                                })
                            })
                        })
                    }) as isize,
                );
        }
        if if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        while *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == ' ' as i32 as yaml_char_t as i32
            || ((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '\t' as i32 as yaml_char_t as i32
        {
            (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
            (*parser).mark.index;
            (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
            (*parser).mark.column;
            (*parser).unread = ((*parser).unread).wrapping_sub(1);
            (*parser).unread;
            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                .offset(
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0x80 as i32 == 0 as i32
                    {
                        1 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xe0 as i32 == 0xc0 as i32
                        {
                            2 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf0 as i32 == 0xe0 as i32
                            {
                                3 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                {
                                    4 as i32
                                } else {
                                    0 as i32
                                })
                            })
                        })
                    }) as isize,
                );
            if if (*parser).unread >= 1 as i32 as u64 {
                1 as i32
            } else {
                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
            } == 0
            {
                return 0 as i32;
            }
        }
        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '#' as i32 as yaml_char_t as i32
        {
            while !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -62i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -123i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                        as i32 == -88i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                        as i32 == -87i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '\0' as i32 as yaml_char_t as i32)
            {
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                (*parser).mark.column;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0x80 as i32 == 0 as i32
                        {
                            1 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xe0 as i32 == 0xc0 as i32
                            {
                                2 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                {
                                    3 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                    {
                                        4 as i32
                                    } else {
                                        0 as i32
                                    })
                                })
                            })
                        }) as isize,
                    );
                if if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0
                {
                    return 0 as i32;
                }
            }
        }
        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\r' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\n' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -62i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -123i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -88i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -87i32 as yaml_char_t as i32)
        {
            break;
        }
        if if (*parser).unread >= 2 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 2 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\r' as i32 as yaml_char_t as i32
            && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize) as i32
                == '\n' as i32 as yaml_char_t as i32
        {
            (*parser).mark.index = ((*parser).mark.index as u64)
                .wrapping_add(2 as i32 as u64) as size_t as size_t;
            (*parser).mark.column = 0 as i32 as size_t;
            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
            (*parser).mark.line;
            (*parser).unread = ((*parser).unread as u64).wrapping_sub(2 as i32 as u64)
                as size_t as size_t;
            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                .offset(2 as i32 as isize);
        } else {
            if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -62i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -123i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                        as i32 == -88i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == -30i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                        as i32 == -128i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                        as i32 == -87i32 as yaml_char_t as i32
            {
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = 0 as i32 as size_t;
                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                (*parser).mark.line;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0x80 as i32 == 0 as i32
                        {
                            1 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xe0 as i32 == 0xc0 as i32
                            {
                                2 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                {
                                    3 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                    {
                                        4 as i32
                                    } else {
                                        0 as i32
                                    })
                                })
                            })
                        }) as isize,
                    );
            } else {};
        };
        if (*parser).flow_level == 0 {
            (*parser).simple_key_allowed = 1 as i32;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_directive(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> i32 {
    let mut current_block: u64;
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
    let mut name: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let mut handle: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut prefix: *mut yaml_char_t = 0 as *mut yaml_char_t;
    start_mark = (*parser).mark;
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    if !(yaml_parser_scan_directive_name(parser, start_mark, &mut name) == 0) {
        if strcmp(name as *mut i8, b"YAML\0" as *const u8 as *const i8) == 0 as i32 {
            if yaml_parser_scan_version_directive_value(
                parser,
                start_mark,
                &mut major,
                &mut minor,
            ) == 0
            {
                current_block = 6293364208900336400;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<yaml_token_t>() as u64,
                );
                (*token).type_0 = YAML_VERSION_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.version_directive.major = major;
                (*token).data.version_directive.minor = minor;
                current_block = 1841672684692190573;
            }
        } else if strcmp(name as *mut i8, b"TAG\0" as *const u8 as *const i8) == 0 as i32
        {
            if yaml_parser_scan_tag_directive_value(
                parser,
                start_mark,
                &mut handle,
                &mut prefix,
            ) == 0
            {
                current_block = 6293364208900336400;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<yaml_token_t>() as u64,
                );
                (*token).type_0 = YAML_TAG_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.tag_directive.handle = handle;
                (*token).data.tag_directive.prefix = prefix;
                current_block = 1841672684692190573;
            }
        } else {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0" as *const u8 as *const i8,
                start_mark,
                b"found unknown directive name\0" as *const u8 as *const i8,
            );
            current_block = 6293364208900336400;
        }
        match current_block {
            6293364208900336400 => {}
            _ => {
                if !(if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32)
                        {
                            current_block = 7149356873433890176;
                            break;
                        }
                        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                        (*parser).mark.index;
                        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                        (*parser).mark.column;
                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                        (*parser).unread;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                            .offset(
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0x80 as i32 == 0 as i32
                                {
                                    1 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                    {
                                        2 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                        {
                                            3 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                            {
                                                4 as i32
                                            } else {
                                                0 as i32
                                            })
                                        })
                                    })
                                }) as isize,
                            );
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 6293364208900336400;
                            break;
                        }
                    }
                    match current_block {
                        6293364208900336400 => {}
                        _ => {
                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '#' as i32 as yaml_char_t as i32
                            {
                                loop {
                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\r' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\n' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -62i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -123i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -30i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -128i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                == -88i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -30i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -128i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                == -87i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\0' as i32 as yaml_char_t as i32
                                    {
                                        current_block = 5601891728916014340;
                                        break;
                                    }
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                    if if (*parser).unread >= 1 as i32 as u64 {
                                        1 as i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                    } == 0
                                    {
                                        current_block = 6293364208900336400;
                                        break;
                                    }
                                }
                            } else {
                                current_block = 5601891728916014340;
                            }
                            match current_block {
                                6293364208900336400 => {}
                                _ => {
                                    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\r' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\n' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -62i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -123i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -30i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -128i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                == -88i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == -30i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == -128i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                == -87i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\0' as i32 as yaml_char_t as i32)
                                    {
                                        yaml_parser_set_scanner_error(
                                            parser,
                                            b"while scanning a directive\0" as *const u8 as *const i8,
                                            start_mark,
                                            b"did not find expected comment or line break\0"
                                                as *const u8 as *const i8,
                                        );
                                    } else {
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == -62i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                    == -123i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                                    == -88i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                                    == -87i32 as yaml_char_t as i32
                                        {
                                            if if (*parser).unread >= 2 as i32 as u64 {
                                                1 as i32
                                            } else {
                                                yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                            } == 0
                                            {
                                                current_block = 6293364208900336400;
                                            } else {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                                        == '\n' as i32 as yaml_char_t as i32
                                                {
                                                    (*parser).mark.index = ((*parser).mark.index as u64)
                                                        .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                    (*parser).mark.line;
                                                    (*parser).unread = ((*parser).unread as u64)
                                                        .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(2 as i32 as isize);
                                                } else {
                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\r' as i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == '\n' as i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -62i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -123i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -30i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -128i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                == -88i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -30i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -128i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                == -87i32 as yaml_char_t as i32
                                                    {
                                                        (*parser).mark.index = ((*parser).mark.index)
                                                            .wrapping_add(1);
                                                        (*parser).mark.index;
                                                        (*parser).mark.column = 0 as i32 as size_t;
                                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                        (*parser).mark.line;
                                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                        (*parser).unread;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(
                                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                    as i32 & 0x80 as i32 == 0 as i32
                                                                {
                                                                    1 as i32
                                                                } else {
                                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                                                    {
                                                                        2 as i32
                                                                    } else {
                                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                                                        {
                                                                            3 as i32
                                                                        } else {
                                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                                                            {
                                                                                4 as i32
                                                                            } else {
                                                                                0 as i32
                                                                            })
                                                                        })
                                                                    })
                                                                }) as isize,
                                                            );
                                                    } else {};
                                                };
                                                current_block = 2232869372362427478;
                                            }
                                        } else {
                                            current_block = 2232869372362427478;
                                        }
                                        match current_block {
                                            6293364208900336400 => {}
                                            _ => {
                                                yaml_free(name as *mut libc::c_void);
                                                return 1 as i32;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(prefix as *mut libc::c_void);
    yaml_free(handle as *mut libc::c_void);
    yaml_free(name as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_directive_name(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut name: *mut *mut yaml_char_t,
) -> i32 {
    let mut current_block: u64;
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if !(if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    >= '0' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        <= '9' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        >= 'A' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= 'Z' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        >= 'a' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= 'z' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '_' as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '-' as i32)
                {
                    current_block = 6873731126896040597;
                    break;
                }
                if if if (string.pointer).offset(5 as i32 as isize) < string.end
                    || yaml_string_extend(
                        &mut string.start,
                        &mut string.pointer,
                        &mut string.end,
                    ) != 0
                {
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } != 0
                {
                    if *(*parser).buffer.pointer as i32 & 0x80 as i32 == 0 as i32 {
                        let fresh21 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                        let fresh22 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        *fresh22 = *fresh21;
                    } else {
                        if *(*parser).buffer.pointer as i32 & 0xe0 as i32 == 0xc0 as i32
                        {
                            let fresh23 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh24 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh24 = *fresh23;
                            let fresh25 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh26 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh26 = *fresh25;
                        } else {
                            if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                == 0xe0 as i32
                            {
                                let fresh27 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh28 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh28 = *fresh27;
                                let fresh29 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh30 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh30 = *fresh29;
                                let fresh31 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh32 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh32 = *fresh31;
                            } else {
                                if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                    == 0xf0 as i32
                                {
                                    let fresh33 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh34 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh34 = *fresh33;
                                    let fresh35 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh36 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh36 = *fresh35;
                                    let fresh37 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh38 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh38 = *fresh37;
                                    let fresh39 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh40 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh40 = *fresh39;
                                } else {};
                            };
                        };
                    };
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                    (*parser).mark.column;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                    1 as i32
                } else {
                    0 as i32
                } == 0
                {
                    current_block = 6492702884463112210;
                    break;
                }
                if if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0
                {
                    current_block = 6492702884463112210;
                    break;
                }
            }
            match current_block {
                6492702884463112210 => {}
                _ => {
                    if string.start == string.pointer {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8 as *const i8,
                            start_mark,
                            b"could not find expected directive name\0" as *const u8
                                as *const i8,
                        );
                    } else if !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                        as i32 == ' ' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\t' as i32 as yaml_char_t as i32
                        || (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\r' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\n' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -62i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -123i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -30i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -128i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                    == -88i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -30i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -128i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                    == -87i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\0' as i32 as yaml_char_t as i32))
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8 as *const i8,
                            start_mark,
                            b"found unexpected non-alphabetical character\0" as *const u8
                                as *const i8,
                        );
                    } else {
                        *name = string.start;
                        return 1 as i32;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_version_directive_value(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut major: *mut i32,
    mut minor: *mut i32,
) -> i32 {
    if if (*parser).unread >= 1 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
    } == 0
    {
        return 0 as i32;
    }
    while *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == ' ' as i32 as yaml_char_t as i32
        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\t' as i32 as yaml_char_t as i32
    {
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        if if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
    }
    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0 as i32;
    }
    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        == '.' as i32 as yaml_char_t as i32)
    {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const i8,
            start_mark,
            b"did not find expected digit or '.' character\0" as *const u8 as *const i8,
        );
    }
    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
    (*parser).mark.index;
    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
    (*parser).mark.column;
    (*parser).unread = ((*parser).unread).wrapping_sub(1);
    (*parser).unread;
    (*parser).buffer.pointer = ((*parser).buffer.pointer)
        .offset(
            (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                & 0x80 as i32 == 0 as i32
            {
                1 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0xe0 as i32 == 0xc0 as i32
                {
                    2 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xf0 as i32 == 0xe0 as i32
                    {
                        3 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf8 as i32 == 0xf0 as i32
                        {
                            4 as i32
                        } else {
                            0 as i32
                        })
                    })
                })
            }) as isize,
        );
    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_version_directive_number(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut number: *mut i32,
) -> i32 {
    let mut value: i32 = 0 as i32;
    let mut length: size_t = 0 as i32 as size_t;
    if if (*parser).unread >= 1 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
    } == 0
    {
        return 0 as i32;
    }
    while *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
        >= '0' as i32 as yaml_char_t as i32
        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            <= '9' as i32 as yaml_char_t as i32
    {
        length = length.wrapping_add(1);
        if length > 9 as i32 as u64 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0" as *const u8 as *const i8,
                start_mark,
                b"found extremely long version number\0" as *const u8 as *const i8,
            );
        }
        value = value * 10 as i32
            + (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                - '0' as i32 as yaml_char_t as i32);
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        if if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
    }
    if length == 0 {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const i8,
            start_mark,
            b"did not find expected version number\0" as *const u8 as *const i8,
        );
    }
    *number = value;
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_tag_directive_value(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut handle: *mut *mut yaml_char_t,
    mut prefix: *mut *mut yaml_char_t,
) -> i32 {
    let mut current_block: u64;
    let mut handle_value: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut prefix_value: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if if (*parser).unread >= 1 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
    } == 0
    {
        current_block = 3362899909330607350;
    } else {
        current_block = 6239978542346980191;
    }
    '_error: loop {
        match current_block {
            3362899909330607350 => {
                yaml_free(handle_value as *mut libc::c_void);
                yaml_free(prefix_value as *mut libc::c_void);
                return 0 as i32;
            }
            _ => {
                if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == ' ' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '\t' as i32 as yaml_char_t as i32
                {
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                    (*parser).mark.column;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                        .offset(
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0x80 as i32 == 0 as i32
                            {
                                1 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                {
                                    2 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                    {
                                        3 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                        {
                                            4 as i32
                                        } else {
                                            0 as i32
                                        })
                                    })
                                })
                            }) as isize,
                        );
                    if if (*parser).unread >= 1 as i32 as u64 {
                        1 as i32
                    } else {
                        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                    } == 0
                    {
                        current_block = 3362899909330607350;
                    } else {
                        current_block = 6239978542346980191;
                    }
                } else {
                    if yaml_parser_scan_tag_handle(
                        parser,
                        1 as i32,
                        start_mark,
                        &mut handle_value,
                    ) == 0
                    {
                        current_block = 3362899909330607350;
                        continue;
                    }
                    if if (*parser).unread >= 1 as i32 as u64 {
                        1 as i32
                    } else {
                        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                    } == 0
                    {
                        current_block = 3362899909330607350;
                        continue;
                    }
                    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == ' ' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\t' as i32 as yaml_char_t as i32)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a %TAG directive\0" as *const u8
                                as *const i8,
                            start_mark,
                            b"did not find expected whitespace\0" as *const u8
                                as *const i8,
                        );
                        current_block = 3362899909330607350;
                    } else {
                        while *((*parser).buffer.pointer).offset(0 as i32 as isize)
                            as i32 == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                        {
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = ((*parser).mark.column)
                                .wrapping_add(1);
                            (*parser).mark.column;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0x80 as i32 == 0 as i32
                                    {
                                        1 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xe0 as i32 == 0xc0 as i32
                                        {
                                            2 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xf0 as i32 == 0xe0 as i32
                                            {
                                                3 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                                {
                                                    4 as i32
                                                } else {
                                                    0 as i32
                                                })
                                            })
                                        })
                                    }) as isize,
                                );
                            if if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0
                            {
                                current_block = 3362899909330607350;
                                continue '_error;
                            }
                        }
                        if yaml_parser_scan_tag_uri(
                            parser,
                            1 as i32,
                            1 as i32,
                            0 as *mut yaml_char_t,
                            start_mark,
                            &mut prefix_value,
                        ) == 0
                        {
                            current_block = 3362899909330607350;
                            continue;
                        }
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 3362899909330607350;
                            continue;
                        }
                        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                            || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\r' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -62i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -123i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -87i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\0' as i32 as yaml_char_t as i32))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a %TAG directive\0" as *const u8
                                    as *const i8,
                                start_mark,
                                b"did not find expected whitespace or line break\0"
                                    as *const u8 as *const i8,
                            );
                            current_block = 3362899909330607350;
                        } else {
                            *handle = handle_value;
                            *prefix = prefix_value;
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yaml_parser_scan_anchor(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut type_0: yaml_token_type_t,
) -> i32 {
    let mut current_block: u64;
    let mut length: i32 = 0 as i32;
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
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        start_mark = (*parser).mark;
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        if !(if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0)
        {
            loop {
                if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    >= '0' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        <= '9' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        >= 'A' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= 'Z' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        >= 'a' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= 'z' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '_' as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '-' as i32)
                {
                    current_block = 17216689946888361452;
                    break;
                }
                if if if (string.pointer).offset(5 as i32 as isize) < string.end
                    || yaml_string_extend(
                        &mut string.start,
                        &mut string.pointer,
                        &mut string.end,
                    ) != 0
                {
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } != 0
                {
                    if *(*parser).buffer.pointer as i32 & 0x80 as i32 == 0 as i32 {
                        let fresh41 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                        let fresh42 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        *fresh42 = *fresh41;
                    } else {
                        if *(*parser).buffer.pointer as i32 & 0xe0 as i32 == 0xc0 as i32
                        {
                            let fresh43 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh44 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh44 = *fresh43;
                            let fresh45 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh46 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh46 = *fresh45;
                        } else {
                            if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                == 0xe0 as i32
                            {
                                let fresh47 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh48 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh48 = *fresh47;
                                let fresh49 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh50 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh50 = *fresh49;
                                let fresh51 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh52 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh52 = *fresh51;
                            } else {
                                if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                    == 0xf0 as i32
                                {
                                    let fresh53 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh54 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh54 = *fresh53;
                                    let fresh55 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh56 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh56 = *fresh55;
                                    let fresh57 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh58 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh58 = *fresh57;
                                    let fresh59 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh60 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh60 = *fresh59;
                                } else {};
                            };
                        };
                    };
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                    (*parser).mark.column;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                    1 as i32
                } else {
                    0 as i32
                } == 0
                {
                    current_block = 2600150587426073677;
                    break;
                }
                if if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0
                {
                    current_block = 2600150587426073677;
                    break;
                }
                length += 1;
                length;
            }
            match current_block {
                2600150587426073677 => {}
                _ => {
                    end_mark = (*parser).mark;
                    if length == 0
                        || !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                            || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\r' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -62i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -123i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -87i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\0' as i32 as yaml_char_t as i32)
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '?' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ':' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ',' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ']' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '}' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '%' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '@' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '`' as i32 as yaml_char_t as i32)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            if type_0 as u32 == YAML_ANCHOR_TOKEN as i32 as u32 {
                                b"while scanning an anchor\0" as *const u8 as *const i8
                            } else {
                                b"while scanning an alias\0" as *const u8 as *const i8
                            },
                            start_mark,
                            b"did not find expected alphabetic or numeric character\0"
                                as *const u8 as *const i8,
                        );
                    } else {
                        if type_0 as u32 == YAML_ANCHOR_TOKEN as i32 as u32 {
                            memset(
                                token as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_token_t>() as u64,
                            );
                            (*token).type_0 = YAML_ANCHOR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.anchor.value = string.start;
                        } else {
                            memset(
                                token as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_token_t>() as u64,
                            );
                            (*token).type_0 = YAML_ALIAS_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.alias.value = string.start;
                        }
                        return 1 as i32;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_tag(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> i32 {
    let mut current_block: u64;
    let mut handle: *mut yaml_char_t = 0 as *mut yaml_char_t;
    let mut suffix: *mut yaml_char_t = 0 as *mut yaml_char_t;
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
    start_mark = (*parser).mark;
    if !(if (*parser).unread >= 2 as i32 as u64 {
        1 as i32
    } else {
        yaml_parser_update_buffer(parser, 2 as i32 as size_t)
    } == 0)
    {
        if *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
            == '<' as i32 as yaml_char_t as i32
        {
            handle = yaml_malloc(1 as i32 as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 2052849929536367530;
            } else {
                *handle.offset(0 as i32 as isize) = '\0' as i32 as yaml_char_t;
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                (*parser).mark.column;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0x80 as i32 == 0 as i32
                        {
                            1 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xe0 as i32 == 0xc0 as i32
                            {
                                2 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                {
                                    3 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                    {
                                        4 as i32
                                    } else {
                                        0 as i32
                                    })
                                })
                            })
                        }) as isize,
                    );
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                (*parser).mark.column;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0x80 as i32 == 0 as i32
                        {
                            1 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xe0 as i32 == 0xc0 as i32
                            {
                                2 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                {
                                    3 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                    {
                                        4 as i32
                                    } else {
                                        0 as i32
                                    })
                                })
                            })
                        }) as isize,
                    );
                if yaml_parser_scan_tag_uri(
                    parser,
                    1 as i32,
                    0 as i32,
                    0 as *mut yaml_char_t,
                    start_mark,
                    &mut suffix,
                ) == 0
                {
                    current_block = 2052849929536367530;
                } else if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '>' as i32 as yaml_char_t as i32)
                {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a tag\0" as *const u8 as *const i8,
                        start_mark,
                        b"did not find the expected '>'\0" as *const u8 as *const i8,
                    );
                    current_block = 2052849929536367530;
                } else {
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                    (*parser).mark.column;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                        .offset(
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0x80 as i32 == 0 as i32
                            {
                                1 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                {
                                    2 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                    {
                                        3 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                        {
                                            4 as i32
                                        } else {
                                            0 as i32
                                        })
                                    })
                                })
                            }) as isize,
                        );
                    current_block = 10652014663920648156;
                }
            }
        } else if yaml_parser_scan_tag_handle(parser, 0 as i32, start_mark, &mut handle)
            == 0
        {
            current_block = 2052849929536367530;
        } else if *handle.offset(0 as i32 as isize) as i32 == '!' as i32
            && *handle.offset(1 as i32 as isize) as i32 != '\0' as i32
            && *handle
                .offset(
                    (strlen(handle as *mut i8)).wrapping_sub(1 as i32 as u64) as isize,
                ) as i32 == '!' as i32
        {
            if yaml_parser_scan_tag_uri(
                parser,
                0 as i32,
                0 as i32,
                0 as *mut yaml_char_t,
                start_mark,
                &mut suffix,
            ) == 0
            {
                current_block = 2052849929536367530;
            } else {
                current_block = 10652014663920648156;
            }
        } else if yaml_parser_scan_tag_uri(
            parser,
            0 as i32,
            0 as i32,
            handle,
            start_mark,
            &mut suffix,
        ) == 0
        {
            current_block = 2052849929536367530;
        } else {
            yaml_free(handle as *mut libc::c_void);
            handle = yaml_malloc(2 as i32 as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 2052849929536367530;
            } else {
                *handle.offset(0 as i32 as isize) = '!' as i32 as yaml_char_t;
                *handle.offset(1 as i32 as isize) = '\0' as i32 as yaml_char_t;
                if *suffix.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                    let mut tmp: *mut yaml_char_t = handle;
                    handle = suffix;
                    suffix = tmp;
                }
                current_block = 10652014663920648156;
            }
        }
        match current_block {
            2052849929536367530 => {}
            _ => {
                if !(if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0)
                {
                    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == ' ' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\t' as i32 as yaml_char_t as i32
                        || (*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\r' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\n' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -62i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -123i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -30i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -128i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                    == -88i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == -30i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                    == -128i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer)
                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                    == -87i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\0' as i32 as yaml_char_t as i32))
                    {
                        if (*parser).flow_level == 0
                            || !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ',' as i32 as yaml_char_t as i32)
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a tag\0" as *const u8 as *const i8,
                                start_mark,
                                b"did not find expected whitespace or line break\0"
                                    as *const u8 as *const i8,
                            );
                            current_block = 2052849929536367530;
                        } else {
                            current_block = 14648156034262866959;
                        }
                    } else {
                        current_block = 14648156034262866959;
                    }
                    match current_block {
                        2052849929536367530 => {}
                        _ => {
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_token_t>() as u64,
                            );
                            (*token).type_0 = YAML_TAG_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.tag.handle = handle;
                            (*token).data.tag.suffix = suffix;
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(handle as *mut libc::c_void);
    yaml_free(suffix as *mut libc::c_void);
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_tag_handle(
    mut parser: *mut yaml_parser_t,
    mut directive: i32,
    mut start_mark: yaml_mark_t,
    mut handle: *mut *mut yaml_char_t,
) -> i32 {
    let mut current_block: u64;
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        if !(if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0)
        {
            if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '!' as i32 as yaml_char_t as i32)
            {
                yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while scanning a tag directive\0" as *const u8 as *const i8
                    } else {
                        b"while scanning a tag\0" as *const u8 as *const i8
                    },
                    start_mark,
                    b"did not find expected '!'\0" as *const u8 as *const i8,
                );
            } else if !(if if (string.pointer).offset(5 as i32 as isize) < string.end
                || yaml_string_extend(
                    &mut string.start,
                    &mut string.pointer,
                    &mut string.end,
                ) != 0
            {
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } != 0
            {
                if *(*parser).buffer.pointer as i32 & 0x80 as i32 == 0 as i32 {
                    let fresh61 = (*parser).buffer.pointer;
                    (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                    let fresh62 = string.pointer;
                    string.pointer = (string.pointer).offset(1);
                    *fresh62 = *fresh61;
                } else {
                    if *(*parser).buffer.pointer as i32 & 0xe0 as i32 == 0xc0 as i32 {
                        let fresh63 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                        let fresh64 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        *fresh64 = *fresh63;
                        let fresh65 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                        let fresh66 = string.pointer;
                        string.pointer = (string.pointer).offset(1);
                        *fresh66 = *fresh65;
                    } else {
                        if *(*parser).buffer.pointer as i32 & 0xf0 as i32 == 0xe0 as i32
                        {
                            let fresh67 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh68 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh68 = *fresh67;
                            let fresh69 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh70 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh70 = *fresh69;
                            let fresh71 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh72 = string.pointer;
                            string.pointer = (string.pointer).offset(1);
                            *fresh72 = *fresh71;
                        } else {
                            if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                == 0xf0 as i32
                            {
                                let fresh73 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh74 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh74 = *fresh73;
                                let fresh75 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh76 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh76 = *fresh75;
                                let fresh77 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh78 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh78 = *fresh77;
                                let fresh79 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh80 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh80 = *fresh79;
                            } else {};
                        };
                    };
                };
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                (*parser).mark.column;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                1 as i32
            } else {
                0 as i32
            } == 0)
            {
                if !(if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0)
                {
                    loop {
                        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            >= '0' as i32 as yaml_char_t as i32
                            && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 <= '9' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 >= 'A' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 <= 'Z' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 >= 'a' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 <= 'z' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '_' as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '-' as i32)
                        {
                            current_block = 3640593987805443782;
                            break;
                        }
                        if if if (string.pointer).offset(5 as i32 as isize) < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } != 0
                        {
                            if *(*parser).buffer.pointer as i32 & 0x80 as i32 == 0 as i32
                            {
                                let fresh81 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh82 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh82 = *fresh81;
                            } else {
                                if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                    == 0xc0 as i32
                                {
                                    let fresh83 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh84 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh84 = *fresh83;
                                    let fresh85 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh86 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh86 = *fresh85;
                                } else {
                                    if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                        == 0xe0 as i32
                                    {
                                        let fresh87 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh88 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh88 = *fresh87;
                                        let fresh89 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh90 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh90 = *fresh89;
                                        let fresh91 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh92 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh92 = *fresh91;
                                    } else {
                                        if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                            == 0xf0 as i32
                                        {
                                            let fresh93 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh94 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh94 = *fresh93;
                                            let fresh95 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh96 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh96 = *fresh95;
                                            let fresh97 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh98 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh98 = *fresh97;
                                            let fresh99 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh100 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh100 = *fresh99;
                                        } else {};
                                    };
                                };
                            };
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = ((*parser).mark.column)
                                .wrapping_add(1);
                            (*parser).mark.column;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                            1 as i32
                        } else {
                            0 as i32
                        } == 0
                        {
                            current_block = 14923755357020359028;
                            break;
                        }
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 14923755357020359028;
                            break;
                        }
                    }
                    match current_block {
                        14923755357020359028 => {}
                        _ => {
                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '!' as i32 as yaml_char_t as i32
                            {
                                if if if (string.pointer).offset(5 as i32 as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                    ) != 0
                                {
                                    1 as i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as i32
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                        == 0 as i32
                                    {
                                        let fresh101 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh102 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh102 = *fresh101;
                                    } else {
                                        if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                            == 0xc0 as i32
                                        {
                                            let fresh103 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh104 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh104 = *fresh103;
                                            let fresh105 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh106 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh106 = *fresh105;
                                        } else {
                                            if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                == 0xe0 as i32
                                            {
                                                let fresh107 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh108 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh108 = *fresh107;
                                                let fresh109 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh110 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh110 = *fresh109;
                                                let fresh111 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh112 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh112 = *fresh111;
                                            } else {
                                                if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                    == 0xf0 as i32
                                                {
                                                    let fresh113 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh114 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh114 = *fresh113;
                                                    let fresh115 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh116 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh116 = *fresh115;
                                                    let fresh117 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh118 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh118 = *fresh117;
                                                    let fresh119 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh120 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh120 = *fresh119;
                                                } else {};
                                            };
                                        };
                                    };
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    1 as i32
                                } else {
                                    0 as i32
                                } == 0
                                {
                                    current_block = 14923755357020359028;
                                } else {
                                    current_block = 17860125682698302841;
                                }
                            } else if directive != 0
                                && !(*(string.start).offset(0 as i32 as isize) as i32
                                    == '!' as i32
                                    && *(string.start).offset(1 as i32 as isize) as i32
                                        == '\0' as i32)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while parsing a tag directive\0" as *const u8
                                        as *const i8,
                                    start_mark,
                                    b"did not find expected '!'\0" as *const u8 as *const i8,
                                );
                                current_block = 14923755357020359028;
                            } else {
                                current_block = 17860125682698302841;
                            }
                            match current_block {
                                14923755357020359028 => {}
                                _ => {
                                    *handle = string.start;
                                    return 1 as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_tag_uri(
    mut parser: *mut yaml_parser_t,
    mut uri_char: i32,
    mut directive: i32,
    mut head: *mut yaml_char_t,
    mut start_mark: yaml_mark_t,
    mut uri: *mut *mut yaml_char_t,
) -> i32 {
    let mut current_block: u64;
    let mut length: size_t = if !head.is_null() {
        strlen(head as *mut i8)
    } else {
        0 as i32 as u64
    };
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0
    {
        current_block = 9229959591424135505;
    } else {
        current_block = 6239978542346980191;
    }
    '_error: loop {
        match current_block {
            9229959591424135505 => {
                yaml_free(string.start as *mut libc::c_void);
                string.end = 0 as *mut yaml_char_t;
                string.pointer = string.end;
                string.start = string.pointer;
                return 0 as i32;
            }
            _ => {
                if (string.end).offset_from(string.start) as i64 as size_t <= length {
                    if !(yaml_string_extend(
                        &mut string.start,
                        &mut string.pointer,
                        &mut string.end,
                    ) == 0)
                    {
                        current_block = 6239978542346980191;
                        continue;
                    }
                    (*parser).error = YAML_MEMORY_ERROR;
                    current_block = 9229959591424135505;
                } else {
                    if length > 1 as i32 as u64 {
                        memcpy(
                            string.start as *mut libc::c_void,
                            head.offset(1 as i32 as isize) as *const libc::c_void,
                            length.wrapping_sub(1 as i32 as u64),
                        );
                        string.pointer = (string.pointer)
                            .offset(length.wrapping_sub(1 as i32 as u64) as isize);
                    }
                    if if (*parser).unread >= 1 as i32 as u64 {
                        1 as i32
                    } else {
                        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                    } == 0
                    {
                        current_block = 9229959591424135505;
                        continue;
                    }
                    while *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        >= '0' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= '9' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            >= 'A' as i32 as yaml_char_t as i32
                            && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 <= 'Z' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            >= 'a' as i32 as yaml_char_t as i32
                            && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 <= 'z' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '_' as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '-' as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ';' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '/' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '?' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ':' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '@' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '&' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '=' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '+' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '$' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '.' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '%' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '!' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '~' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '*' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '\'' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '(' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ')' as i32 as yaml_char_t as i32
                        || uri_char != 0
                            && (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ',' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '[' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == ']' as i32 as yaml_char_t as i32)
                    {
                        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '%' as i32 as yaml_char_t as i32
                        {
                            if if (string.pointer).offset(5 as i32 as isize) < string.end
                                || yaml_string_extend(
                                    &mut string.start,
                                    &mut string.pointer,
                                    &mut string.end,
                                ) != 0
                            {
                                1 as i32
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0 as i32
                            } == 0
                            {
                                current_block = 9229959591424135505;
                                continue '_error;
                            }
                            if yaml_parser_scan_uri_escapes(
                                parser,
                                directive,
                                start_mark,
                                &mut string,
                            ) == 0
                            {
                                current_block = 9229959591424135505;
                                continue '_error;
                            }
                        } else if if if (string.pointer).offset(5 as i32 as isize)
                            < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } != 0
                        {
                            if *(*parser).buffer.pointer as i32 & 0x80 as i32 == 0 as i32
                            {
                                let fresh121 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(1);
                                let fresh122 = string.pointer;
                                string.pointer = (string.pointer).offset(1);
                                *fresh122 = *fresh121;
                            } else {
                                if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                    == 0xc0 as i32
                                {
                                    let fresh123 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh124 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh124 = *fresh123;
                                    let fresh125 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(1);
                                    let fresh126 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh126 = *fresh125;
                                } else {
                                    if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                        == 0xe0 as i32
                                    {
                                        let fresh127 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh128 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh128 = *fresh127;
                                        let fresh129 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh130 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh130 = *fresh129;
                                        let fresh131 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh132 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh132 = *fresh131;
                                    } else {
                                        if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                            == 0xf0 as i32
                                        {
                                            let fresh133 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh134 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh134 = *fresh133;
                                            let fresh135 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh136 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh136 = *fresh135;
                                            let fresh137 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh138 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh138 = *fresh137;
                                            let fresh139 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh140 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh140 = *fresh139;
                                        } else {};
                                    };
                                };
                            };
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = ((*parser).mark.column)
                                .wrapping_add(1);
                            (*parser).mark.column;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                            1 as i32
                        } else {
                            0 as i32
                        } == 0
                        {
                            current_block = 9229959591424135505;
                            continue '_error;
                        }
                        length = length.wrapping_add(1);
                        length;
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 9229959591424135505;
                            continue '_error;
                        }
                    }
                    if length == 0 {
                        if if (string.pointer).offset(5 as i32 as isize) < string.end
                            || yaml_string_extend(
                                &mut string.start,
                                &mut string.pointer,
                                &mut string.end,
                            ) != 0
                        {
                            1 as i32
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as i32
                        } == 0
                        {
                            current_block = 9229959591424135505;
                            continue;
                        }
                        yaml_parser_set_scanner_error(
                            parser,
                            if directive != 0 {
                                b"while parsing a %TAG directive\0" as *const u8
                                    as *const i8
                            } else {
                                b"while parsing a tag\0" as *const u8 as *const i8
                            },
                            start_mark,
                            b"did not find expected tag URI\0" as *const u8 as *const i8,
                        );
                        current_block = 9229959591424135505;
                    } else {
                        *uri = string.start;
                        return 1 as i32;
                    }
                }
            }
        }
    };
}
unsafe extern "C" fn yaml_parser_scan_uri_escapes(
    mut parser: *mut yaml_parser_t,
    mut directive: i32,
    mut start_mark: yaml_mark_t,
    mut string: *mut yaml_string_t,
) -> i32 {
    let mut width: i32 = 0 as i32;
    loop {
        let mut octet: u8 = 0 as i32 as u8;
        if if (*parser).unread >= 3 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 3 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '%' as i32 as yaml_char_t as i32
            && (*((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                >= '0' as i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    <= '9' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    >= 'A' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        <= 'F' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    >= 'a' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                        <= 'f' as i32 as yaml_char_t as i32)
            && (*((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                >= '0' as i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    <= '9' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    >= 'A' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                        <= 'F' as i32 as yaml_char_t as i32
                || *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    >= 'a' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                        <= 'f' as i32 as yaml_char_t as i32))
        {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const i8
                } else {
                    b"while parsing a tag\0" as *const u8 as *const i8
                },
                start_mark,
                b"did not find URI escaped octet\0" as *const u8 as *const i8,
            );
        }
        octet = (((if *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
            >= 'A' as i32 as yaml_char_t as i32
            && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                <= 'F' as i32 as yaml_char_t as i32
        {
            *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                - 'A' as i32 as yaml_char_t as i32 + 10 as i32
        } else {
            (if *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                >= 'a' as i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    <= 'f' as i32 as yaml_char_t as i32
            {
                *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    - 'a' as i32 as yaml_char_t as i32 + 10 as i32
            } else {
                *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    - '0' as i32 as yaml_char_t as i32
            })
        }) << 4 as i32)
            + (if *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                >= 'A' as i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    <= 'F' as i32 as yaml_char_t as i32
            {
                *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    - 'A' as i32 as yaml_char_t as i32 + 10 as i32
            } else {
                (if *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                    >= 'a' as i32 as yaml_char_t as i32
                    && *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                        <= 'f' as i32 as yaml_char_t as i32
                {
                    *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                        - 'a' as i32 as yaml_char_t as i32 + 10 as i32
                } else {
                    *((*parser).buffer.pointer).offset(2 as i32 as isize) as i32
                        - '0' as i32 as yaml_char_t as i32
                })
            })) as u8;
        if width == 0 {
            width = if octet as i32 & 0x80 as i32 == 0 as i32 {
                1 as i32
            } else if octet as i32 & 0xe0 as i32 == 0xc0 as i32 {
                2 as i32
            } else if octet as i32 & 0xf0 as i32 == 0xe0 as i32 {
                3 as i32
            } else if octet as i32 & 0xf8 as i32 == 0xf0 as i32 {
                4 as i32
            } else {
                0 as i32
            };
            if width == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while parsing a %TAG directive\0" as *const u8 as *const i8
                    } else {
                        b"while parsing a tag\0" as *const u8 as *const i8
                    },
                    start_mark,
                    b"found an incorrect leading UTF-8 octet\0" as *const u8 as *const i8,
                );
            }
        } else if octet as i32 & 0xc0 as i32 != 0x80 as i32 {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const i8
                } else {
                    b"while parsing a tag\0" as *const u8 as *const i8
                },
                start_mark,
                b"found an incorrect trailing UTF-8 octet\0" as *const u8 as *const i8,
            )
        }
        let fresh141 = (*string).pointer;
        (*string).pointer = ((*string).pointer).offset(1);
        *fresh141 = octet;
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
        (*parser).mark.index;
        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
        (*parser).mark.column;
        (*parser).unread = ((*parser).unread).wrapping_sub(1);
        (*parser).unread;
        (*parser).buffer.pointer = ((*parser).buffer.pointer)
            .offset(
                (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    & 0x80 as i32 == 0 as i32
                {
                    1 as i32
                } else {
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0xe0 as i32 == 0xc0 as i32
                    {
                        2 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xf0 as i32 == 0xe0 as i32
                        {
                            3 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf8 as i32 == 0xf0 as i32
                            {
                                4 as i32
                            } else {
                                0 as i32
                            })
                        })
                    })
                }) as isize,
            );
        width -= 1;
        if !(width != 0) {
            break;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut literal: i32,
) -> i32 {
    let mut current_block: u64;
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
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut leading_break: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut trailing_breaks: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut chomping: i32 = 0 as i32;
    let mut increment: i32 = 0 as i32;
    let mut indent: i32 = 0 as i32;
    let mut leading_blank: i32 = 0 as i32;
    let mut trailing_blank: i32 = 0 as i32;
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).offset(16 as i32 as isize);
            memset(leading_break.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).offset(16 as i32 as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as i32,
                    16 as i32 as u64,
                );
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0)
            {
                start_mark = (*parser).mark;
                (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                (*parser).mark.index;
                (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                (*parser).mark.column;
                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                (*parser).unread;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0x80 as i32 == 0 as i32
                        {
                            1 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xe0 as i32 == 0xc0 as i32
                            {
                                2 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                {
                                    3 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                    {
                                        4 as i32
                                    } else {
                                        0 as i32
                                    })
                                })
                            })
                        }) as isize,
                    );
                if !(if (*parser).unread >= 1 as i32 as u64 {
                    1 as i32
                } else {
                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                } == 0)
                {
                    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '+' as i32 as yaml_char_t as i32
                        || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '-' as i32 as yaml_char_t as i32
                    {
                        chomping = if *((*parser).buffer.pointer)
                            .offset(0 as i32 as isize) as i32
                            == '+' as i32 as yaml_char_t as i32
                        {
                            1 as i32
                        } else {
                            -(1 as i32)
                        };
                        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                        (*parser).mark.index;
                        (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                        (*parser).mark.column;
                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                        (*parser).unread;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                            .offset(
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0x80 as i32 == 0 as i32
                                {
                                    1 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                    {
                                        2 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                        {
                                            3 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                            {
                                                4 as i32
                                            } else {
                                                0 as i32
                                            })
                                        })
                                    })
                                }) as isize,
                            );
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 17909933110308592907;
                        } else if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                            as i32 >= '0' as i32 as yaml_char_t as i32
                            && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 <= '9' as i32 as yaml_char_t as i32
                        {
                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '0' as i32 as yaml_char_t as i32
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a block scalar\0" as *const u8
                                        as *const i8,
                                    start_mark,
                                    b"found an indentation indicator equal to 0\0" as *const u8
                                        as *const i8,
                                );
                                current_block = 17909933110308592907;
                            } else {
                                increment = *((*parser).buffer.pointer)
                                    .offset(0 as i32 as isize) as i32
                                    - '0' as i32 as yaml_char_t as i32;
                                (*parser).mark.index = ((*parser).mark.index)
                                    .wrapping_add(1);
                                (*parser).mark.index;
                                (*parser).mark.column = ((*parser).mark.column)
                                    .wrapping_add(1);
                                (*parser).mark.column;
                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                (*parser).unread;
                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                    .offset(
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0x80 as i32 == 0 as i32
                                        {
                                            1 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xe0 as i32 == 0xc0 as i32
                                            {
                                                2 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                                {
                                                    3 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                                    {
                                                        4 as i32
                                                    } else {
                                                        0 as i32
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                current_block = 6669252993407410313;
                            }
                        } else {
                            current_block = 6669252993407410313;
                        }
                    } else if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                        as i32 >= '0' as i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            <= '9' as i32 as yaml_char_t as i32
                    {
                        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '0' as i32 as yaml_char_t as i32
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a block scalar\0" as *const u8
                                    as *const i8,
                                start_mark,
                                b"found an indentation indicator equal to 0\0" as *const u8
                                    as *const i8,
                            );
                            current_block = 17909933110308592907;
                        } else {
                            increment = *((*parser).buffer.pointer)
                                .offset(0 as i32 as isize) as i32
                                - '0' as i32 as yaml_char_t as i32;
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = ((*parser).mark.column)
                                .wrapping_add(1);
                            (*parser).mark.column;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0x80 as i32 == 0 as i32
                                    {
                                        1 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xe0 as i32 == 0xc0 as i32
                                        {
                                            2 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xf0 as i32 == 0xe0 as i32
                                            {
                                                3 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                                {
                                                    4 as i32
                                                } else {
                                                    0 as i32
                                                })
                                            })
                                        })
                                    }) as isize,
                                );
                            if if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0
                            {
                                current_block = 17909933110308592907;
                            } else {
                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '+' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '-' as i32 as yaml_char_t as i32
                                {
                                    chomping = if *((*parser).buffer.pointer)
                                        .offset(0 as i32 as isize) as i32
                                        == '+' as i32 as yaml_char_t as i32
                                    {
                                        1 as i32
                                    } else {
                                        -(1 as i32)
                                    };
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                }
                                current_block = 6669252993407410313;
                            }
                        }
                    } else {
                        current_block = 6669252993407410313;
                    }
                    match current_block {
                        17909933110308592907 => {}
                        _ => {
                            if !(if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0)
                            {
                                loop {
                                    if !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == ' ' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\t' as i32 as yaml_char_t as i32)
                                    {
                                        current_block = 11932355480408055363;
                                        break;
                                    }
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                    if if (*parser).unread >= 1 as i32 as u64 {
                                        1 as i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                    } == 0
                                    {
                                        current_block = 17909933110308592907;
                                        break;
                                    }
                                }
                                match current_block {
                                    17909933110308592907 => {}
                                    _ => {
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '#' as i32 as yaml_char_t as i32
                                        {
                                            loop {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -62i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -123i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -128i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                            == -88i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -128i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                            == -87i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\0' as i32 as yaml_char_t as i32
                                                {
                                                    current_block = 9520865839495247062;
                                                    break;
                                                }
                                                (*parser).mark.index = ((*parser).mark.index)
                                                    .wrapping_add(1);
                                                (*parser).mark.index;
                                                (*parser).mark.column = ((*parser).mark.column)
                                                    .wrapping_add(1);
                                                (*parser).mark.column;
                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                (*parser).unread;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0x80 as i32 == 0 as i32
                                                        {
                                                            1 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xe0 as i32 == 0xc0 as i32
                                                            {
                                                                2 as i32
                                                            } else {
                                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                                                {
                                                                    3 as i32
                                                                } else {
                                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                                                    {
                                                                        4 as i32
                                                                    } else {
                                                                        0 as i32
                                                                    })
                                                                })
                                                            })
                                                        }) as isize,
                                                    );
                                                if if (*parser).unread >= 1 as i32 as u64 {
                                                    1 as i32
                                                } else {
                                                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                                } == 0
                                                {
                                                    current_block = 17909933110308592907;
                                                    break;
                                                }
                                            }
                                        } else {
                                            current_block = 9520865839495247062;
                                        }
                                        match current_block {
                                            17909933110308592907 => {}
                                            _ => {
                                                if !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -62i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -123i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -128i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                            == -88i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                            == -128i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer)
                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                            == -87i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\0' as i32 as yaml_char_t as i32)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while scanning a block scalar\0" as *const u8
                                                            as *const i8,
                                                        start_mark,
                                                        b"did not find expected comment or line break\0"
                                                            as *const u8 as *const i8,
                                                    );
                                                } else {
                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\r' as i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == '\n' as i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -62i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -123i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -30i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -128i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                == -88i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -30i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                == -128i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer)
                                                                .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                == -87i32 as yaml_char_t as i32
                                                    {
                                                        if if (*parser).unread >= 2 as i32 as u64 {
                                                            1 as i32
                                                        } else {
                                                            yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                                        } == 0
                                                        {
                                                            current_block = 17909933110308592907;
                                                        } else {
                                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                                && *((*parser).buffer.pointer)
                                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                    == '\n' as i32 as yaml_char_t as i32
                                                            {
                                                                (*parser).mark.index = ((*parser).mark.index as u64)
                                                                    .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                                                (*parser).mark.column = 0 as i32 as size_t;
                                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                (*parser).mark.line;
                                                                (*parser).unread = ((*parser).unread as u64)
                                                                    .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                    .offset(2 as i32 as isize);
                                                            } else {
                                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 == -62i32 as yaml_char_t as i32
                                                                        && *((*parser).buffer.pointer)
                                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                            == -123i32 as yaml_char_t as i32
                                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 == -30i32 as yaml_char_t as i32
                                                                        && *((*parser).buffer.pointer)
                                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                            == -128i32 as yaml_char_t as i32
                                                                        && *((*parser).buffer.pointer)
                                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                            == -88i32 as yaml_char_t as i32
                                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 == -30i32 as yaml_char_t as i32
                                                                        && *((*parser).buffer.pointer)
                                                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                            == -128i32 as yaml_char_t as i32
                                                                        && *((*parser).buffer.pointer)
                                                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                            == -87i32 as yaml_char_t as i32
                                                                {
                                                                    (*parser).mark.index = ((*parser).mark.index)
                                                                        .wrapping_add(1);
                                                                    (*parser).mark.index;
                                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                    (*parser).mark.line;
                                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                                    (*parser).unread;
                                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                        .offset(
                                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 & 0x80 as i32 == 0 as i32
                                                                            {
                                                                                1 as i32
                                                                            } else {
                                                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                                                {
                                                                                    2 as i32
                                                                                } else {
                                                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                                                    {
                                                                                        3 as i32
                                                                                    } else {
                                                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                                                        {
                                                                                            4 as i32
                                                                                        } else {
                                                                                            0 as i32
                                                                                        })
                                                                                    })
                                                                                })
                                                                            }) as isize,
                                                                        );
                                                                } else {};
                                                            };
                                                            current_block = 11743904203796629665;
                                                        }
                                                    } else {
                                                        current_block = 11743904203796629665;
                                                    }
                                                    match current_block {
                                                        17909933110308592907 => {}
                                                        _ => {
                                                            end_mark = (*parser).mark;
                                                            if increment != 0 {
                                                                indent = if (*parser).indent >= 0 as i32 {
                                                                    (*parser).indent + increment
                                                                } else {
                                                                    increment
                                                                };
                                                            }
                                                            if !(yaml_parser_scan_block_scalar_breaks(
                                                                parser,
                                                                &mut indent,
                                                                &mut trailing_breaks,
                                                                start_mark,
                                                                &mut end_mark,
                                                            ) == 0)
                                                            {
                                                                if !(if (*parser).unread >= 1 as i32 as u64 {
                                                                    1 as i32
                                                                } else {
                                                                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                                                } == 0)
                                                                {
                                                                    's_226: loop {
                                                                        if !((*parser).mark.column as i32 == indent
                                                                            && !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\0' as i32 as yaml_char_t as i32))
                                                                        {
                                                                            current_block = 5807581744382915773;
                                                                            break;
                                                                        }
                                                                        trailing_blank = (*((*parser).buffer.pointer)
                                                                            .offset(0 as i32 as isize) as i32
                                                                            == ' ' as i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\t' as i32 as yaml_char_t as i32) as i32;
                                                                        if literal == 0
                                                                            && *leading_break.start as i32 == '\n' as i32
                                                                            && leading_blank == 0 && trailing_blank == 0
                                                                        {
                                                                            if *trailing_breaks.start as i32 == '\0' as i32 {
                                                                                if if (string.pointer).offset(5 as i32 as isize)
                                                                                    < string.end
                                                                                    || yaml_string_extend(
                                                                                        &mut string.start,
                                                                                        &mut string.pointer,
                                                                                        &mut string.end,
                                                                                    ) != 0
                                                                                {
                                                                                    1 as i32
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as i32
                                                                                } == 0
                                                                                {
                                                                                    current_block = 17909933110308592907;
                                                                                    break;
                                                                                }
                                                                                let fresh142 = string.pointer;
                                                                                string.pointer = (string.pointer).offset(1);
                                                                                *fresh142 = ' ' as i32 as yaml_char_t;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0 as i32,
                                                                                (leading_break.end).offset_from(leading_break.start) as i64
                                                                                    as u64,
                                                                            );
                                                                        } else {
                                                                            if if yaml_string_join(
                                                                                &mut string.start,
                                                                                &mut string.pointer,
                                                                                &mut string.end,
                                                                                &mut leading_break.start,
                                                                                &mut leading_break.pointer,
                                                                                &mut leading_break.end,
                                                                            ) != 0
                                                                            {
                                                                                leading_break.pointer = leading_break.start;
                                                                                1 as i32
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as i32
                                                                            } == 0
                                                                            {
                                                                                current_block = 17909933110308592907;
                                                                                break;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut libc::c_void,
                                                                                0 as i32,
                                                                                (leading_break.end).offset_from(leading_break.start) as i64
                                                                                    as u64,
                                                                            );
                                                                        }
                                                                        if if yaml_string_join(
                                                                            &mut string.start,
                                                                            &mut string.pointer,
                                                                            &mut string.end,
                                                                            &mut trailing_breaks.start,
                                                                            &mut trailing_breaks.pointer,
                                                                            &mut trailing_breaks.end,
                                                                        ) != 0
                                                                        {
                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                            1 as i32
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as i32
                                                                        } == 0
                                                                        {
                                                                            current_block = 17909933110308592907;
                                                                            break;
                                                                        }
                                                                        trailing_breaks.pointer = trailing_breaks.start;
                                                                        memset(
                                                                            trailing_breaks.start as *mut libc::c_void,
                                                                            0 as i32,
                                                                            (trailing_breaks.end).offset_from(trailing_breaks.start)
                                                                                as i64 as u64,
                                                                        );
                                                                        leading_blank = (*((*parser).buffer.pointer)
                                                                            .offset(0 as i32 as isize) as i32
                                                                            == ' ' as i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\t' as i32 as yaml_char_t as i32) as i32;
                                                                        while !(*((*parser).buffer.pointer)
                                                                            .offset(0 as i32 as isize) as i32
                                                                            == '\r' as i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == -62i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer)
                                                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                                    == -123i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == -30i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer)
                                                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                                    == -128i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer)
                                                                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                                    == -88i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == -30i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer)
                                                                                    .offset((0 as i32 + 1 as i32) as isize) as i32
                                                                                    == -128i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer)
                                                                                    .offset((0 as i32 + 2 as i32) as isize) as i32
                                                                                    == -87i32 as yaml_char_t as i32
                                                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\0' as i32 as yaml_char_t as i32)
                                                                        {
                                                                            if if if (string.pointer).offset(5 as i32 as isize)
                                                                                < string.end
                                                                                || yaml_string_extend(
                                                                                    &mut string.start,
                                                                                    &mut string.pointer,
                                                                                    &mut string.end,
                                                                                ) != 0
                                                                            {
                                                                                1 as i32
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as i32
                                                                            } != 0
                                                                            {
                                                                                if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                                                                    == 0 as i32
                                                                                {
                                                                                    let fresh143 = (*parser).buffer.pointer;
                                                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                        .offset(1);
                                                                                    let fresh144 = string.pointer;
                                                                                    string.pointer = (string.pointer).offset(1);
                                                                                    *fresh144 = *fresh143;
                                                                                } else {
                                                                                    if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                                                                        == 0xc0 as i32
                                                                                    {
                                                                                        let fresh145 = (*parser).buffer.pointer;
                                                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                            .offset(1);
                                                                                        let fresh146 = string.pointer;
                                                                                        string.pointer = (string.pointer).offset(1);
                                                                                        *fresh146 = *fresh145;
                                                                                        let fresh147 = (*parser).buffer.pointer;
                                                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                            .offset(1);
                                                                                        let fresh148 = string.pointer;
                                                                                        string.pointer = (string.pointer).offset(1);
                                                                                        *fresh148 = *fresh147;
                                                                                    } else {
                                                                                        if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                                                            == 0xe0 as i32
                                                                                        {
                                                                                            let fresh149 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh150 = string.pointer;
                                                                                            string.pointer = (string.pointer).offset(1);
                                                                                            *fresh150 = *fresh149;
                                                                                            let fresh151 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh152 = string.pointer;
                                                                                            string.pointer = (string.pointer).offset(1);
                                                                                            *fresh152 = *fresh151;
                                                                                            let fresh153 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh154 = string.pointer;
                                                                                            string.pointer = (string.pointer).offset(1);
                                                                                            *fresh154 = *fresh153;
                                                                                        } else {
                                                                                            if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                                                                == 0xf0 as i32
                                                                                            {
                                                                                                let fresh155 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                    .offset(1);
                                                                                                let fresh156 = string.pointer;
                                                                                                string.pointer = (string.pointer).offset(1);
                                                                                                *fresh156 = *fresh155;
                                                                                                let fresh157 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                    .offset(1);
                                                                                                let fresh158 = string.pointer;
                                                                                                string.pointer = (string.pointer).offset(1);
                                                                                                *fresh158 = *fresh157;
                                                                                                let fresh159 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                    .offset(1);
                                                                                                let fresh160 = string.pointer;
                                                                                                string.pointer = (string.pointer).offset(1);
                                                                                                *fresh160 = *fresh159;
                                                                                                let fresh161 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                    .offset(1);
                                                                                                let fresh162 = string.pointer;
                                                                                                string.pointer = (string.pointer).offset(1);
                                                                                                *fresh162 = *fresh161;
                                                                                            } else {};
                                                                                        };
                                                                                    };
                                                                                };
                                                                                (*parser).mark.index = ((*parser).mark.index)
                                                                                    .wrapping_add(1);
                                                                                (*parser).mark.index;
                                                                                (*parser).mark.column = ((*parser).mark.column)
                                                                                    .wrapping_add(1);
                                                                                (*parser).mark.column;
                                                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                                                (*parser).unread;
                                                                                1 as i32
                                                                            } else {
                                                                                0 as i32
                                                                            } == 0
                                                                            {
                                                                                current_block = 17909933110308592907;
                                                                                break 's_226;
                                                                            }
                                                                            if if (*parser).unread >= 1 as i32 as u64 {
                                                                                1 as i32
                                                                            } else {
                                                                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                                                            } == 0
                                                                            {
                                                                                current_block = 17909933110308592907;
                                                                                break 's_226;
                                                                            }
                                                                        }
                                                                        if if (*parser).unread >= 2 as i32 as u64 {
                                                                            1 as i32
                                                                        } else {
                                                                            yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                                                        } == 0
                                                                        {
                                                                            current_block = 17909933110308592907;
                                                                            break;
                                                                        }
                                                                        if if if (leading_break.pointer).offset(5 as i32 as isize)
                                                                            < leading_break.end
                                                                            || yaml_string_extend(
                                                                                &mut leading_break.start,
                                                                                &mut leading_break.pointer,
                                                                                &mut leading_break.end,
                                                                            ) != 0
                                                                        {
                                                                            1 as i32
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as i32
                                                                        } != 0
                                                                        {
                                                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                                                && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                                                            {
                                                                                let fresh163 = leading_break.pointer;
                                                                                leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                *fresh163 = '\n' as i32 as yaml_char_t;
                                                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                    .offset(2 as i32 as isize);
                                                                                (*parser).mark.index = ((*parser).mark.index as u64)
                                                                                    .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                                                                (*parser).mark.column = 0 as i32 as size_t;
                                                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                                (*parser).mark.line;
                                                                                (*parser).unread = ((*parser).unread as u64)
                                                                                    .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                                                            } else {
                                                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                                                                {
                                                                                    let fresh164 = leading_break.pointer;
                                                                                    leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                    *fresh164 = '\n' as i32 as yaml_char_t;
                                                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                        .offset(1);
                                                                                    (*parser).buffer.pointer;
                                                                                    (*parser).mark.index = ((*parser).mark.index)
                                                                                        .wrapping_add(1);
                                                                                    (*parser).mark.index;
                                                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                                    (*parser).mark.line;
                                                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                                                    (*parser).unread;
                                                                                } else {
                                                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                        as i32 == -62i32 as yaml_char_t as i32
                                                                                        && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                                                            as i32 == -123i32 as yaml_char_t as i32
                                                                                    {
                                                                                        let fresh165 = leading_break.pointer;
                                                                                        leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                        *fresh165 = '\n' as i32 as yaml_char_t;
                                                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                            .offset(2 as i32 as isize);
                                                                                        (*parser).mark.index = ((*parser).mark.index)
                                                                                            .wrapping_add(1);
                                                                                        (*parser).mark.index;
                                                                                        (*parser).mark.column = 0 as i32 as size_t;
                                                                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                                        (*parser).mark.line;
                                                                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                                                        (*parser).unread;
                                                                                    } else {
                                                                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                                            as i32 == -30i32 as yaml_char_t as i32
                                                                                            && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                                                                as i32 == -128i32 as yaml_char_t as i32
                                                                                            && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                                                as i32 == -88i32 as yaml_char_t as i32
                                                                                                || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                                                    as i32 == -87i32 as yaml_char_t as i32)
                                                                                        {
                                                                                            let fresh166 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh167 = leading_break.pointer;
                                                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                            *fresh167 = *fresh166;
                                                                                            let fresh168 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh169 = leading_break.pointer;
                                                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                            *fresh169 = *fresh168;
                                                                                            let fresh170 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                                                .offset(1);
                                                                                            let fresh171 = leading_break.pointer;
                                                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                                                            *fresh171 = *fresh170;
                                                                                            (*parser).mark.index = ((*parser).mark.index)
                                                                                                .wrapping_add(1);
                                                                                            (*parser).mark.index;
                                                                                            (*parser).mark.column = 0 as i32 as size_t;
                                                                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                                                            (*parser).mark.line;
                                                                                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                                                            (*parser).unread;
                                                                                        } else {};
                                                                                    };
                                                                                };
                                                                            };
                                                                            1 as i32
                                                                        } else {
                                                                            0 as i32
                                                                        } == 0
                                                                        {
                                                                            current_block = 17909933110308592907;
                                                                            break;
                                                                        }
                                                                        if yaml_parser_scan_block_scalar_breaks(
                                                                            parser,
                                                                            &mut indent,
                                                                            &mut trailing_breaks,
                                                                            start_mark,
                                                                            &mut end_mark,
                                                                        ) == 0
                                                                        {
                                                                            current_block = 17909933110308592907;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        17909933110308592907 => {}
                                                                        _ => {
                                                                            if chomping != -(1 as i32) {
                                                                                if if yaml_string_join(
                                                                                    &mut string.start,
                                                                                    &mut string.pointer,
                                                                                    &mut string.end,
                                                                                    &mut leading_break.start,
                                                                                    &mut leading_break.pointer,
                                                                                    &mut leading_break.end,
                                                                                ) != 0
                                                                                {
                                                                                    leading_break.pointer = leading_break.start;
                                                                                    1 as i32
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as i32
                                                                                } == 0
                                                                                {
                                                                                    current_block = 17909933110308592907;
                                                                                } else {
                                                                                    current_block = 981995395831942902;
                                                                                }
                                                                            } else {
                                                                                current_block = 981995395831942902;
                                                                            }
                                                                            match current_block {
                                                                                17909933110308592907 => {}
                                                                                _ => {
                                                                                    if chomping == 1 as i32 {
                                                                                        if if yaml_string_join(
                                                                                            &mut string.start,
                                                                                            &mut string.pointer,
                                                                                            &mut string.end,
                                                                                            &mut trailing_breaks.start,
                                                                                            &mut trailing_breaks.pointer,
                                                                                            &mut trailing_breaks.end,
                                                                                        ) != 0
                                                                                        {
                                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                                            1 as i32
                                                                                        } else {
                                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                                            0 as i32
                                                                                        } == 0
                                                                                        {
                                                                                            current_block = 17909933110308592907;
                                                                                        } else {
                                                                                            current_block = 16779030619667747692;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 16779030619667747692;
                                                                                    }
                                                                                    match current_block {
                                                                                        17909933110308592907 => {}
                                                                                        _ => {
                                                                                            memset(
                                                                                                token as *mut libc::c_void,
                                                                                                0 as i32,
                                                                                                ::core::mem::size_of::<yaml_token_t>() as u64,
                                                                                            );
                                                                                            (*token).type_0 = YAML_SCALAR_TOKEN;
                                                                                            (*token).start_mark = start_mark;
                                                                                            (*token).end_mark = end_mark;
                                                                                            (*token).data.scalar.value = string.start;
                                                                                            (*token).data.scalar.length = (string.pointer)
                                                                                                .offset_from(string.start) as i64 as size_t;
                                                                                            (*token).data.scalar.style = (if literal != 0 {
                                                                                                YAML_LITERAL_SCALAR_STYLE as i32
                                                                                            } else {
                                                                                                YAML_FOLDED_SCALAR_STYLE as i32
                                                                                            }) as yaml_scalar_style_t;
                                                                                            yaml_free(leading_break.start as *mut libc::c_void);
                                                                                            leading_break.end = 0 as *mut yaml_char_t;
                                                                                            leading_break.pointer = leading_break.end;
                                                                                            leading_break.start = leading_break.pointer;
                                                                                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                                                                                            trailing_breaks.end = 0 as *mut yaml_char_t;
                                                                                            trailing_breaks.pointer = trailing_breaks.end;
                                                                                            trailing_breaks.start = trailing_breaks.pointer;
                                                                                            return 1 as i32;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = 0 as *mut yaml_char_t;
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = 0 as *mut yaml_char_t;
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_block_scalar_breaks(
    mut parser: *mut yaml_parser_t,
    mut indent: *mut i32,
    mut breaks: *mut yaml_string_t,
    mut start_mark: yaml_mark_t,
    mut end_mark: *mut yaml_mark_t,
) -> i32 {
    let mut max_indent: i32 = 0 as i32;
    *end_mark = (*parser).mark;
    loop {
        if if (*parser).unread >= 1 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        while (*indent == 0 || ((*parser).mark.column as i32) < *indent)
            && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == ' ' as i32 as yaml_char_t as i32
        {
            (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
            (*parser).mark.index;
            (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
            (*parser).mark.column;
            (*parser).unread = ((*parser).unread).wrapping_sub(1);
            (*parser).unread;
            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                .offset(
                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        & 0x80 as i32 == 0 as i32
                    {
                        1 as i32
                    } else {
                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            & 0xe0 as i32 == 0xc0 as i32
                        {
                            2 as i32
                        } else {
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0xf0 as i32 == 0xe0 as i32
                            {
                                3 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                {
                                    4 as i32
                                } else {
                                    0 as i32
                                })
                            })
                        })
                    }) as isize,
                );
            if if (*parser).unread >= 1 as i32 as u64 {
                1 as i32
            } else {
                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
            } == 0
            {
                return 0 as i32;
            }
        }
        if (*parser).mark.column as i32 > max_indent {
            max_indent = (*parser).mark.column as i32;
        }
        if (*indent == 0 || ((*parser).mark.column as i32) < *indent)
            && *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\t' as i32 as yaml_char_t as i32
        {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0" as *const u8 as *const i8,
                start_mark,
                b"found a tab character where an indentation space is expected\0"
                    as *const u8 as *const i8,
            );
        }
        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
            == '\r' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\n' as i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -62i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -123i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -88i32 as yaml_char_t as i32
            || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == -30i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 1 as i32) as isize)
                    as i32 == -128i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset((0 as i32 + 2 as i32) as isize)
                    as i32 == -87i32 as yaml_char_t as i32)
        {
            break;
        }
        if if (*parser).unread >= 2 as i32 as u64 {
            1 as i32
        } else {
            yaml_parser_update_buffer(parser, 2 as i32 as size_t)
        } == 0
        {
            return 0 as i32;
        }
        if if if ((*breaks).pointer).offset(5 as i32 as isize) < (*breaks).end
            || yaml_string_extend(
                &mut (*breaks).start,
                &mut (*breaks).pointer,
                &mut (*breaks).end,
            ) != 0
        {
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } != 0
        {
            if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                == '\r' as i32 as yaml_char_t as i32
                && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                    == '\n' as i32 as yaml_char_t as i32
            {
                let fresh172 = (*breaks).pointer;
                (*breaks).pointer = ((*breaks).pointer).offset(1);
                *fresh172 = '\n' as i32 as yaml_char_t;
                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                    .offset(2 as i32 as isize);
                (*parser).mark.index = ((*parser).mark.index as u64)
                    .wrapping_add(2 as i32 as u64) as size_t as size_t;
                (*parser).mark.column = 0 as i32 as size_t;
                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                (*parser).mark.line;
                (*parser).unread = ((*parser).unread as u64)
                    .wrapping_sub(2 as i32 as u64) as size_t as size_t;
            } else {
                if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                    == '\r' as i32 as yaml_char_t as i32
                    || *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == '\n' as i32 as yaml_char_t as i32
                {
                    let fresh173 = (*breaks).pointer;
                    (*breaks).pointer = ((*breaks).pointer).offset(1);
                    *fresh173 = '\n' as i32 as yaml_char_t;
                    (*parser).buffer.pointer = ((*parser).buffer.pointer).offset(1);
                    (*parser).buffer.pointer;
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = 0 as i32 as size_t;
                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                    (*parser).mark.line;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                } else {
                    if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                        == -62i32 as yaml_char_t as i32
                        && *((*parser).buffer.pointer).offset(1 as i32 as isize) as i32
                            == -123i32 as yaml_char_t as i32
                    {
                        let fresh174 = (*breaks).pointer;
                        (*breaks).pointer = ((*breaks).pointer).offset(1);
                        *fresh174 = '\n' as i32 as yaml_char_t;
                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                            .offset(2 as i32 as isize);
                        (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                        (*parser).mark.index;
                        (*parser).mark.column = 0 as i32 as size_t;
                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                        (*parser).mark.line;
                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                        (*parser).unread;
                    } else {
                        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == -30i32 as yaml_char_t as i32
                            && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                as i32 == -128i32 as yaml_char_t as i32
                            && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                as i32 == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                    as i32 == -87i32 as yaml_char_t as i32)
                        {
                            let fresh175 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh176 = (*breaks).pointer;
                            (*breaks).pointer = ((*breaks).pointer).offset(1);
                            *fresh176 = *fresh175;
                            let fresh177 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh178 = (*breaks).pointer;
                            (*breaks).pointer = ((*breaks).pointer).offset(1);
                            *fresh178 = *fresh177;
                            let fresh179 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(1);
                            let fresh180 = (*breaks).pointer;
                            (*breaks).pointer = ((*breaks).pointer).offset(1);
                            *fresh180 = *fresh179;
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = 0 as i32 as size_t;
                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                            (*parser).mark.line;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                        } else {};
                    };
                };
            };
            1 as i32
        } else {
            0 as i32
        } == 0
        {
            return 0 as i32;
        }
        *end_mark = (*parser).mark;
    }
    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent + 1 as i32 {
            *indent = (*parser).indent + 1 as i32;
        }
        if *indent < 1 as i32 {
            *indent = 1 as i32;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_parser_scan_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut single: i32,
) -> i32 {
    let mut current_block: u64;
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
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut leading_break: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut trailing_breaks: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut whitespaces: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut leading_blanks: i32 = 0;
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).offset(16 as i32 as isize);
            memset(leading_break.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).offset(16 as i32 as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as i32,
                    16 as i32 as u64,
                );
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
                if !(if !(whitespaces.start).is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = (whitespaces.start).offset(16 as i32 as isize);
                    memset(
                        whitespaces.start as *mut libc::c_void,
                        0 as i32,
                        16 as i32 as u64,
                    );
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0)
                {
                    start_mark = (*parser).mark;
                    (*parser).mark.index = ((*parser).mark.index).wrapping_add(1);
                    (*parser).mark.index;
                    (*parser).mark.column = ((*parser).mark.column).wrapping_add(1);
                    (*parser).mark.column;
                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                    (*parser).unread;
                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                        .offset(
                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 & 0x80 as i32 == 0 as i32
                            {
                                1 as i32
                            } else {
                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                {
                                    2 as i32
                                } else {
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                    {
                                        3 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                        {
                                            4 as i32
                                        } else {
                                            0 as i32
                                        })
                                    })
                                })
                            }) as isize,
                        );
                    's_44: loop {
                        if if (*parser).unread >= 4 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 4 as i32 as size_t)
                        } == 0
                        {
                            current_block = 3273537353013554702;
                            break;
                        }
                        if (*parser).mark.column == 0 as i32 as u64
                            && (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '-' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                    as i32 == '-' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                    as i32 == '-' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '.' as i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '.' as i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                        as i32 == '.' as i32 as yaml_char_t as i32)
                            && (*((*parser).buffer.pointer).offset(3 as i32 as isize)
                                as i32 == ' ' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                    as i32 == '\t' as i32 as yaml_char_t as i32
                                || (*((*parser).buffer.pointer).offset(3 as i32 as isize)
                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -62i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -123i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 2 as i32) as isize) as i32
                                            == -88i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 2 as i32) as isize) as i32
                                            == -87i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == '\0' as i32 as yaml_char_t as i32))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const i8,
                                start_mark,
                                b"found unexpected document indicator\0" as *const u8
                                    as *const i8,
                            );
                            current_block = 3273537353013554702;
                            break;
                        } else if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                            as i32 == '\0' as i32 as yaml_char_t as i32
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const i8,
                                start_mark,
                                b"found unexpected end of stream\0" as *const u8
                                    as *const i8,
                            );
                            current_block = 3273537353013554702;
                            break;
                        } else {
                            if if (*parser).unread >= 2 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                            } == 0
                            {
                                current_block = 3273537353013554702;
                                break;
                            }
                            leading_blanks = 0 as i32;
                            while !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ' ' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\t' as i32 as yaml_char_t as i32
                                || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -62i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -123i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                            == -88i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                            == -87i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\0' as i32 as yaml_char_t as i32))
                            {
                                if single != 0
                                    && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\'' as i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '\'' as i32 as yaml_char_t as i32
                                {
                                    if if (string.pointer).offset(5 as i32 as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                        ) != 0
                                    {
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break 's_44;
                                    }
                                    let fresh181 = string.pointer;
                                    string.pointer = (string.pointer).offset(1);
                                    *fresh181 = '\'' as i32 as yaml_char_t;
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                } else {
                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32
                                        == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                            as yaml_char_t as i32
                                    {
                                        break;
                                    }
                                    if single == 0
                                        && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\\' as i32 as yaml_char_t as i32
                                        && (*((*parser).buffer.pointer).offset(1 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -62i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -123i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 2 as i32) as isize) as i32
                                                    == -88i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 2 as i32) as isize) as i32
                                                    == -87i32 as yaml_char_t as i32)
                                    {
                                        if if (*parser).unread >= 3 as i32 as u64 {
                                            1 as i32
                                        } else {
                                            yaml_parser_update_buffer(parser, 3 as i32 as size_t)
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break 's_44;
                                        }
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0x80 as i32 == 0 as i32
                                                {
                                                    1 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                                    {
                                                        2 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                                        {
                                                            3 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                                            {
                                                                4 as i32
                                                            } else {
                                                                0 as i32
                                                            })
                                                        })
                                                    })
                                                }) as isize,
                                            );
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer)
                                                .offset((0 as i32 + 1 as i32) as isize) as i32
                                                == '\n' as i32 as yaml_char_t as i32
                                        {
                                            (*parser).mark.index = ((*parser).mark.index as u64)
                                                .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                            (*parser).mark.column = 0 as i32 as size_t;
                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                            (*parser).mark.line;
                                            (*parser).unread = ((*parser).unread as u64)
                                                .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(2 as i32 as isize);
                                        } else {
                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -62i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                                        == -123i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -30i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                                        == -128i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                                        == -88i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -30i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                                        == -128i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer)
                                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                                        == -87i32 as yaml_char_t as i32
                                            {
                                                (*parser).mark.index = ((*parser).mark.index)
                                                    .wrapping_add(1);
                                                (*parser).mark.index;
                                                (*parser).mark.column = 0 as i32 as size_t;
                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                (*parser).mark.line;
                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                (*parser).unread;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0x80 as i32 == 0 as i32
                                                        {
                                                            1 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xe0 as i32 == 0xc0 as i32
                                                            {
                                                                2 as i32
                                                            } else {
                                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                    as i32 & 0xf0 as i32 == 0xe0 as i32
                                                                {
                                                                    3 as i32
                                                                } else {
                                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 & 0xf8 as i32 == 0xf0 as i32
                                                                    {
                                                                        4 as i32
                                                                    } else {
                                                                        0 as i32
                                                                    })
                                                                })
                                                            })
                                                        }) as isize,
                                                    );
                                            } else {};
                                        };
                                        leading_blanks = 1 as i32;
                                        break;
                                    } else if single == 0
                                        && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\\' as i32 as yaml_char_t as i32
                                    {
                                        let mut code_length: size_t = 0 as i32 as size_t;
                                        if if (string.pointer).offset(5 as i32 as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                            ) != 0
                                        {
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break 's_44;
                                        }
                                        match *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                            as i32
                                        {
                                            48 => {
                                                let fresh182 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh182 = '\0' as i32 as yaml_char_t;
                                            }
                                            97 => {
                                                let fresh183 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh183 = '\u{7}' as i32 as yaml_char_t;
                                            }
                                            98 => {
                                                let fresh184 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh184 = '\u{8}' as i32 as yaml_char_t;
                                            }
                                            116 | 9 => {
                                                let fresh185 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh185 = '\t' as i32 as yaml_char_t;
                                            }
                                            110 => {
                                                let fresh186 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh186 = '\n' as i32 as yaml_char_t;
                                            }
                                            118 => {
                                                let fresh187 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh187 = '\u{b}' as i32 as yaml_char_t;
                                            }
                                            102 => {
                                                let fresh188 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh188 = '\u{c}' as i32 as yaml_char_t;
                                            }
                                            114 => {
                                                let fresh189 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh189 = '\r' as i32 as yaml_char_t;
                                            }
                                            101 => {
                                                let fresh190 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh190 = '\u{1b}' as i32 as yaml_char_t;
                                            }
                                            32 => {
                                                let fresh191 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh191 = ' ' as i32 as yaml_char_t;
                                            }
                                            34 => {
                                                let fresh192 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh192 = '"' as i32 as yaml_char_t;
                                            }
                                            47 => {
                                                let fresh193 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh193 = '/' as i32 as yaml_char_t;
                                            }
                                            92 => {
                                                let fresh194 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh194 = '\\' as i32 as yaml_char_t;
                                            }
                                            78 => {
                                                let fresh195 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh195 = -62i32 as yaml_char_t;
                                                let fresh196 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh196 = -123i32 as yaml_char_t;
                                            }
                                            95 => {
                                                let fresh197 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh197 = -62i32 as yaml_char_t;
                                                let fresh198 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh198 = -96i32 as yaml_char_t;
                                            }
                                            76 => {
                                                let fresh199 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh199 = -30i32 as yaml_char_t;
                                                let fresh200 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh200 = -128i32 as yaml_char_t;
                                                let fresh201 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh201 = -88i32 as yaml_char_t;
                                            }
                                            80 => {
                                                let fresh202 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh202 = -30i32 as yaml_char_t;
                                                let fresh203 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh203 = -128i32 as yaml_char_t;
                                                let fresh204 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh204 = -87i32 as yaml_char_t;
                                            }
                                            120 => {
                                                code_length = 2 as i32 as size_t;
                                            }
                                            117 => {
                                                code_length = 4 as i32 as size_t;
                                            }
                                            85 => {
                                                code_length = 8 as i32 as size_t;
                                            }
                                            _ => {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const i8,
                                                    start_mark,
                                                    b"found unknown escape character\0" as *const u8
                                                        as *const i8,
                                                );
                                                current_block = 3273537353013554702;
                                                break 's_44;
                                            }
                                        }
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0x80 as i32 == 0 as i32
                                                {
                                                    1 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                                    {
                                                        2 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                                        {
                                                            3 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                                            {
                                                                4 as i32
                                                            } else {
                                                                0 as i32
                                                            })
                                                        })
                                                    })
                                                }) as isize,
                                            );
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0x80 as i32 == 0 as i32
                                                {
                                                    1 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                                    {
                                                        2 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                                        {
                                                            3 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                                            {
                                                                4 as i32
                                                            } else {
                                                                0 as i32
                                                            })
                                                        })
                                                    })
                                                }) as isize,
                                            );
                                        if code_length != 0 {
                                            let mut value: u32 = 0 as i32 as u32;
                                            let mut k: size_t = 0;
                                            if if (*parser).unread >= code_length {
                                                1 as i32
                                            } else {
                                                yaml_parser_update_buffer(parser, code_length)
                                            } == 0
                                            {
                                                current_block = 3273537353013554702;
                                                break 's_44;
                                            }
                                            k = 0 as i32 as size_t;
                                            while k < code_length {
                                                if !(*((*parser).buffer.pointer).offset(k as isize) as i32
                                                    >= '0' as i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer).offset(k as isize) as i32
                                                        <= '9' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(k as isize) as i32
                                                        >= 'A' as i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer).offset(k as isize) as i32
                                                            <= 'F' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(k as isize) as i32
                                                        >= 'a' as i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer).offset(k as isize) as i32
                                                            <= 'f' as i32 as yaml_char_t as i32)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while parsing a quoted scalar\0" as *const u8
                                                            as *const i8,
                                                        start_mark,
                                                        b"did not find expected hexdecimal number\0" as *const u8
                                                            as *const i8,
                                                    );
                                                    current_block = 3273537353013554702;
                                                    break 's_44;
                                                } else {
                                                    value = (value << 4 as i32)
                                                        .wrapping_add(
                                                            (if *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                >= 'A' as i32 as yaml_char_t as i32
                                                                && *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                    <= 'F' as i32 as yaml_char_t as i32
                                                            {
                                                                *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                    - 'A' as i32 as yaml_char_t as i32 + 10 as i32
                                                            } else {
                                                                (if *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                    >= 'a' as i32 as yaml_char_t as i32
                                                                    && *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                        <= 'f' as i32 as yaml_char_t as i32
                                                                {
                                                                    *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                        - 'a' as i32 as yaml_char_t as i32 + 10 as i32
                                                                } else {
                                                                    *((*parser).buffer.pointer).offset(k as isize) as i32
                                                                        - '0' as i32 as yaml_char_t as i32
                                                                })
                                                            }) as u32,
                                                        );
                                                    k = k.wrapping_add(1);
                                                    k;
                                                }
                                            }
                                            if value >= 0xd800 as i32 as u32
                                                && value <= 0xdfff as i32 as u32
                                                || value > 0x10ffff as i32 as u32
                                            {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const i8,
                                                    start_mark,
                                                    b"found invalid Unicode character escape code\0"
                                                        as *const u8 as *const i8,
                                                );
                                                current_block = 3273537353013554702;
                                                break 's_44;
                                            } else {
                                                if value <= 0x7f as i32 as u32 {
                                                    let fresh205 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh205 = value as yaml_char_t;
                                                } else if value <= 0x7ff as i32 as u32 {
                                                    let fresh206 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh206 = (0xc0 as i32 as u32)
                                                        .wrapping_add(value >> 6 as i32) as yaml_char_t;
                                                    let fresh207 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh207 = (0x80 as i32 as u32)
                                                        .wrapping_add(value & 0x3f as i32 as u32) as yaml_char_t;
                                                } else if value <= 0xffff as i32 as u32 {
                                                    let fresh208 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh208 = (0xe0 as i32 as u32)
                                                        .wrapping_add(value >> 12 as i32) as yaml_char_t;
                                                    let fresh209 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh209 = (0x80 as i32 as u32)
                                                        .wrapping_add(value >> 6 as i32 & 0x3f as i32 as u32)
                                                        as yaml_char_t;
                                                    let fresh210 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh210 = (0x80 as i32 as u32)
                                                        .wrapping_add(value & 0x3f as i32 as u32) as yaml_char_t;
                                                } else {
                                                    let fresh211 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh211 = (0xf0 as i32 as u32)
                                                        .wrapping_add(value >> 18 as i32) as yaml_char_t;
                                                    let fresh212 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh212 = (0x80 as i32 as u32)
                                                        .wrapping_add(value >> 12 as i32 & 0x3f as i32 as u32)
                                                        as yaml_char_t;
                                                    let fresh213 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh213 = (0x80 as i32 as u32)
                                                        .wrapping_add(value >> 6 as i32 & 0x3f as i32 as u32)
                                                        as yaml_char_t;
                                                    let fresh214 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh214 = (0x80 as i32 as u32)
                                                        .wrapping_add(value & 0x3f as i32 as u32) as yaml_char_t;
                                                }
                                                k = 0 as i32 as size_t;
                                                while k < code_length {
                                                    (*parser).mark.index = ((*parser).mark.index)
                                                        .wrapping_add(1);
                                                    (*parser).mark.index;
                                                    (*parser).mark.column = ((*parser).mark.column)
                                                        .wrapping_add(1);
                                                    (*parser).mark.column;
                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                    (*parser).unread;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0x80 as i32 == 0 as i32
                                                            {
                                                                1 as i32
                                                            } else {
                                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                                {
                                                                    2 as i32
                                                                } else {
                                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                                    {
                                                                        3 as i32
                                                                    } else {
                                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                                        {
                                                                            4 as i32
                                                                        } else {
                                                                            0 as i32
                                                                        })
                                                                    })
                                                                })
                                                            }) as isize,
                                                        );
                                                    k = k.wrapping_add(1);
                                                    k;
                                                }
                                            }
                                        }
                                    } else if if if (string.pointer).offset(5 as i32 as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                        ) != 0
                                    {
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                            == 0 as i32
                                        {
                                            let fresh215 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh216 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh216 = *fresh215;
                                        } else {
                                            if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                                == 0xc0 as i32
                                            {
                                                let fresh217 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh218 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh218 = *fresh217;
                                                let fresh219 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh220 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh220 = *fresh219;
                                            } else {
                                                if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                    == 0xe0 as i32
                                                {
                                                    let fresh221 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh222 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh222 = *fresh221;
                                                    let fresh223 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh224 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh224 = *fresh223;
                                                    let fresh225 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh226 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh226 = *fresh225;
                                                } else {
                                                    if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                        == 0xf0 as i32
                                                    {
                                                        let fresh227 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh228 = string.pointer;
                                                        string.pointer = (string.pointer).offset(1);
                                                        *fresh228 = *fresh227;
                                                        let fresh229 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh230 = string.pointer;
                                                        string.pointer = (string.pointer).offset(1);
                                                        *fresh230 = *fresh229;
                                                        let fresh231 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh232 = string.pointer;
                                                        string.pointer = (string.pointer).offset(1);
                                                        *fresh232 = *fresh231;
                                                        let fresh233 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh234 = string.pointer;
                                                        string.pointer = (string.pointer).offset(1);
                                                        *fresh234 = *fresh233;
                                                    } else {};
                                                };
                                            };
                                        };
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        1 as i32
                                    } else {
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break 's_44;
                                    }
                                }
                                if if (*parser).unread >= 2 as i32 as u64 {
                                    1 as i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                } == 0
                                {
                                    current_block = 3273537353013554702;
                                    break 's_44;
                                }
                            }
                            if if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0
                            {
                                current_block = 3273537353013554702;
                                break;
                            }
                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32
                                == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                    as yaml_char_t as i32
                            {
                                current_block = 10468276026569382870;
                                break;
                            }
                            if if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0
                            {
                                current_block = 3273537353013554702;
                                break;
                            }
                            while *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ' ' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\t' as i32 as yaml_char_t as i32
                                || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -62i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -123i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                            == -88i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((0 as i32 + 2 as i32) as isize) as i32
                                            == -87i32 as yaml_char_t as i32)
                            {
                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == ' ' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\t' as i32 as yaml_char_t as i32
                                {
                                    if leading_blanks == 0 {
                                        if if if (whitespaces.pointer).offset(5 as i32 as isize)
                                            < whitespaces.end
                                            || yaml_string_extend(
                                                &mut whitespaces.start,
                                                &mut whitespaces.pointer,
                                                &mut whitespaces.end,
                                            ) != 0
                                        {
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } != 0
                                        {
                                            if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                                == 0 as i32
                                            {
                                                let fresh235 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh236 = whitespaces.pointer;
                                                whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                *fresh236 = *fresh235;
                                            } else {
                                                if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                                    == 0xc0 as i32
                                                {
                                                    let fresh237 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh238 = whitespaces.pointer;
                                                    whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                    *fresh238 = *fresh237;
                                                    let fresh239 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh240 = whitespaces.pointer;
                                                    whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                    *fresh240 = *fresh239;
                                                } else {
                                                    if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                        == 0xe0 as i32
                                                    {
                                                        let fresh241 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh242 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh242 = *fresh241;
                                                        let fresh243 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh244 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh244 = *fresh243;
                                                        let fresh245 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh246 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh246 = *fresh245;
                                                    } else {
                                                        if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                            == 0xf0 as i32
                                                        {
                                                            let fresh247 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh248 = whitespaces.pointer;
                                                            whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                            *fresh248 = *fresh247;
                                                            let fresh249 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh250 = whitespaces.pointer;
                                                            whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                            *fresh250 = *fresh249;
                                                            let fresh251 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh252 = whitespaces.pointer;
                                                            whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                            *fresh252 = *fresh251;
                                                            let fresh253 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh254 = whitespaces.pointer;
                                                            whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                            *fresh254 = *fresh253;
                                                        } else {};
                                                    };
                                                };
                                            };
                                            (*parser).mark.index = ((*parser).mark.index)
                                                .wrapping_add(1);
                                            (*parser).mark.index;
                                            (*parser).mark.column = ((*parser).mark.column)
                                                .wrapping_add(1);
                                            (*parser).mark.column;
                                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                            (*parser).unread;
                                            1 as i32
                                        } else {
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break 's_44;
                                        }
                                    } else {
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0x80 as i32 == 0 as i32
                                                {
                                                    1 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xe0 as i32 == 0xc0 as i32
                                                    {
                                                        2 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf0 as i32 == 0xe0 as i32
                                                        {
                                                            3 as i32
                                                        } else {
                                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                                as i32 & 0xf8 as i32 == 0xf0 as i32
                                                            {
                                                                4 as i32
                                                            } else {
                                                                0 as i32
                                                            })
                                                        })
                                                    })
                                                }) as isize,
                                            );
                                    }
                                } else {
                                    if if (*parser).unread >= 2 as i32 as u64 {
                                        1 as i32
                                    } else {
                                        yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break 's_44;
                                    }
                                    if leading_blanks == 0 {
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0 as i32,
                                            (whitespaces.end).offset_from(whitespaces.start) as i64
                                                as u64,
                                        );
                                        if if if (leading_break.pointer).offset(5 as i32 as isize)
                                            < leading_break.end
                                            || yaml_string_extend(
                                                &mut leading_break.start,
                                                &mut leading_break.pointer,
                                                &mut leading_break.end,
                                            ) != 0
                                        {
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } != 0
                                        {
                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                            {
                                                let fresh255 = leading_break.pointer;
                                                leading_break.pointer = (leading_break.pointer).offset(1);
                                                *fresh255 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(2 as i32 as isize);
                                                (*parser).mark.index = ((*parser).mark.index as u64)
                                                    .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                                (*parser).mark.column = 0 as i32 as size_t;
                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                (*parser).mark.line;
                                                (*parser).unread = ((*parser).unread as u64)
                                                    .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                            } else {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                                    || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                                {
                                                    let fresh256 = leading_break.pointer;
                                                    leading_break.pointer = (leading_break.pointer).offset(1);
                                                    *fresh256 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    (*parser).buffer.pointer;
                                                    (*parser).mark.index = ((*parser).mark.index)
                                                        .wrapping_add(1);
                                                    (*parser).mark.index;
                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                    (*parser).mark.line;
                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                    (*parser).unread;
                                                } else {
                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -62i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                            as i32 == -123i32 as yaml_char_t as i32
                                                    {
                                                        let fresh257 = leading_break.pointer;
                                                        leading_break.pointer = (leading_break.pointer).offset(1);
                                                        *fresh257 = '\n' as i32 as yaml_char_t;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(2 as i32 as isize);
                                                        (*parser).mark.index = ((*parser).mark.index)
                                                            .wrapping_add(1);
                                                        (*parser).mark.index;
                                                        (*parser).mark.column = 0 as i32 as size_t;
                                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                        (*parser).mark.line;
                                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                        (*parser).unread;
                                                    } else {
                                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 == -30i32 as yaml_char_t as i32
                                                            && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                                as i32 == -128i32 as yaml_char_t as i32
                                                            && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                as i32 == -88i32 as yaml_char_t as i32
                                                                || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                    as i32 == -87i32 as yaml_char_t as i32)
                                                        {
                                                            let fresh258 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh259 = leading_break.pointer;
                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                            *fresh259 = *fresh258;
                                                            let fresh260 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh261 = leading_break.pointer;
                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                            *fresh261 = *fresh260;
                                                            let fresh262 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                                .offset(1);
                                                            let fresh263 = leading_break.pointer;
                                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                                            *fresh263 = *fresh262;
                                                            (*parser).mark.index = ((*parser).mark.index)
                                                                .wrapping_add(1);
                                                            (*parser).mark.index;
                                                            (*parser).mark.column = 0 as i32 as size_t;
                                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                            (*parser).mark.line;
                                                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                            (*parser).unread;
                                                        } else {};
                                                    };
                                                };
                                            };
                                            1 as i32
                                        } else {
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break 's_44;
                                        }
                                        leading_blanks = 1 as i32;
                                    } else if if if (trailing_breaks.pointer)
                                        .offset(5 as i32 as isize) < trailing_breaks.end
                                        || yaml_string_extend(
                                            &mut trailing_breaks.start,
                                            &mut trailing_breaks.pointer,
                                            &mut trailing_breaks.end,
                                        ) != 0
                                    {
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                        {
                                            let fresh264 = trailing_breaks.pointer;
                                            trailing_breaks.pointer = (trailing_breaks.pointer)
                                                .offset(1);
                                            *fresh264 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(2 as i32 as isize);
                                            (*parser).mark.index = ((*parser).mark.index as u64)
                                                .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                            (*parser).mark.column = 0 as i32 as size_t;
                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                            (*parser).mark.line;
                                            (*parser).unread = ((*parser).unread as u64)
                                                .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                        } else {
                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                            {
                                                let fresh265 = trailing_breaks.pointer;
                                                trailing_breaks.pointer = (trailing_breaks.pointer)
                                                    .offset(1);
                                                *fresh265 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                (*parser).buffer.pointer;
                                                (*parser).mark.index = ((*parser).mark.index)
                                                    .wrapping_add(1);
                                                (*parser).mark.index;
                                                (*parser).mark.column = 0 as i32 as size_t;
                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                (*parser).mark.line;
                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                (*parser).unread;
                                            } else {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -62i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                        as i32 == -123i32 as yaml_char_t as i32
                                                {
                                                    let fresh266 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer = (trailing_breaks.pointer)
                                                        .offset(1);
                                                    *fresh266 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(2 as i32 as isize);
                                                    (*parser).mark.index = ((*parser).mark.index)
                                                        .wrapping_add(1);
                                                    (*parser).mark.index;
                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                    (*parser).mark.line;
                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                    (*parser).unread;
                                                } else {
                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                            as i32 == -128i32 as yaml_char_t as i32
                                                        && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                            as i32 == -88i32 as yaml_char_t as i32
                                                            || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                as i32 == -87i32 as yaml_char_t as i32)
                                                    {
                                                        let fresh267 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh268 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer = (trailing_breaks.pointer)
                                                            .offset(1);
                                                        *fresh268 = *fresh267;
                                                        let fresh269 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh270 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer = (trailing_breaks.pointer)
                                                            .offset(1);
                                                        *fresh270 = *fresh269;
                                                        let fresh271 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh272 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer = (trailing_breaks.pointer)
                                                            .offset(1);
                                                        *fresh272 = *fresh271;
                                                        (*parser).mark.index = ((*parser).mark.index)
                                                            .wrapping_add(1);
                                                        (*parser).mark.index;
                                                        (*parser).mark.column = 0 as i32 as size_t;
                                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                        (*parser).mark.line;
                                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                        (*parser).unread;
                                                    } else {};
                                                };
                                            };
                                        };
                                        1 as i32
                                    } else {
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break 's_44;
                                    }
                                }
                                if if (*parser).unread >= 1 as i32 as u64 {
                                    1 as i32
                                } else {
                                    yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                                } == 0
                                {
                                    current_block = 3273537353013554702;
                                    break 's_44;
                                }
                            }
                            if leading_blanks != 0 {
                                if *(leading_break.start).offset(0 as i32 as isize) as i32
                                    == '\n' as i32
                                {
                                    if *(trailing_breaks.start).offset(0 as i32 as isize) as i32
                                        == '\0' as i32
                                    {
                                        if if (string.pointer).offset(5 as i32 as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                            ) != 0
                                        {
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break;
                                        }
                                        let fresh273 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh273 = ' ' as i32 as yaml_char_t;
                                    } else {
                                        if if yaml_string_join(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                            &mut trailing_breaks.start,
                                            &mut trailing_breaks.pointer,
                                            &mut trailing_breaks.end,
                                        ) != 0
                                        {
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 3273537353013554702;
                                            break;
                                        }
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        memset(
                                            trailing_breaks.start as *mut libc::c_void,
                                            0 as i32,
                                            (trailing_breaks.end).offset_from(trailing_breaks.start)
                                                as i64 as u64,
                                        );
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0 as i32,
                                        (leading_break.end).offset_from(leading_break.start) as i64
                                            as u64,
                                    );
                                } else {
                                    if if yaml_string_join(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                        &mut leading_break.start,
                                        &mut leading_break.pointer,
                                        &mut leading_break.end,
                                    ) != 0
                                    {
                                        leading_break.pointer = leading_break.start;
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break;
                                    }
                                    if if yaml_string_join(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                        &mut trailing_breaks.start,
                                        &mut trailing_breaks.pointer,
                                        &mut trailing_breaks.end,
                                    ) != 0
                                    {
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 3273537353013554702;
                                        break;
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut libc::c_void,
                                        0 as i32,
                                        (leading_break.end).offset_from(leading_break.start) as i64
                                            as u64,
                                    );
                                    trailing_breaks.pointer = trailing_breaks.start;
                                    memset(
                                        trailing_breaks.start as *mut libc::c_void,
                                        0 as i32,
                                        (trailing_breaks.end).offset_from(trailing_breaks.start)
                                            as i64 as u64,
                                    );
                                }
                            } else {
                                if if yaml_string_join(
                                    &mut string.start,
                                    &mut string.pointer,
                                    &mut string.end,
                                    &mut whitespaces.start,
                                    &mut whitespaces.pointer,
                                    &mut whitespaces.end,
                                ) != 0
                                {
                                    whitespaces.pointer = whitespaces.start;
                                    1 as i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as i32
                                } == 0
                                {
                                    current_block = 3273537353013554702;
                                    break;
                                }
                                whitespaces.pointer = whitespaces.start;
                                memset(
                                    whitespaces.start as *mut libc::c_void,
                                    0 as i32,
                                    (whitespaces.end).offset_from(whitespaces.start) as i64
                                        as u64,
                                );
                            }
                        }
                    }
                    match current_block {
                        3273537353013554702 => {}
                        _ => {
                            (*parser).mark.index = ((*parser).mark.index)
                                .wrapping_add(1);
                            (*parser).mark.index;
                            (*parser).mark.column = ((*parser).mark.column)
                                .wrapping_add(1);
                            (*parser).mark.column;
                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                            (*parser).unread;
                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                .offset(
                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 & 0x80 as i32 == 0 as i32
                                    {
                                        1 as i32
                                    } else {
                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 & 0xe0 as i32 == 0xc0 as i32
                                        {
                                            2 as i32
                                        } else {
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0xf0 as i32 == 0xe0 as i32
                                            {
                                                3 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xf8 as i32 == 0xf0 as i32
                                                {
                                                    4 as i32
                                                } else {
                                                    0 as i32
                                                })
                                            })
                                        })
                                    }) as isize,
                                );
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_token_t>() as u64,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.scalar.value = string.start;
                            (*token).data.scalar.length = (string.pointer)
                                .offset_from(string.start) as i64 as size_t;
                            (*token).data.scalar.style = (if single != 0 {
                                YAML_SINGLE_QUOTED_SCALAR_STYLE as i32
                            } else {
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE as i32
                            }) as yaml_scalar_style_t;
                            yaml_free(leading_break.start as *mut libc::c_void);
                            leading_break.end = 0 as *mut yaml_char_t;
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                            trailing_breaks.end = 0 as *mut yaml_char_t;
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut libc::c_void);
                            whitespaces.end = 0 as *mut yaml_char_t;
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = 0 as *mut yaml_char_t;
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = 0 as *mut yaml_char_t;
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut libc::c_void);
    whitespaces.end = 0 as *mut yaml_char_t;
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    return 0 as i32;
}
unsafe extern "C" fn yaml_parser_scan_plain_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> i32 {
    let mut current_block: u64;
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
    let mut string: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut leading_break: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut trailing_breaks: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut whitespaces: yaml_string_t = {
        let mut init = yaml_string_t {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
        };
        init
    };
    let mut leading_blanks: i32 = 0 as i32;
    let mut indent: i32 = (*parser).indent + 1 as i32;
    string.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
    if !(if !(string.start).is_null() {
        string.pointer = string.start;
        string.end = (string.start).offset(16 as i32 as isize);
        memset(string.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
        1 as i32
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as i32
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
        if !(if !(leading_break.start).is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = (leading_break.start).offset(16 as i32 as isize);
            memset(leading_break.start as *mut libc::c_void, 0 as i32, 16 as i32 as u64);
            1 as i32
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as i32
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
            if !(if !(trailing_breaks.start).is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = (trailing_breaks.start).offset(16 as i32 as isize);
                memset(
                    trailing_breaks.start as *mut libc::c_void,
                    0 as i32,
                    16 as i32 as u64,
                );
                1 as i32
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as i32
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as i32 as size_t) as *mut yaml_char_t;
                if !(if !(whitespaces.start).is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = (whitespaces.start).offset(16 as i32 as isize);
                    memset(
                        whitespaces.start as *mut libc::c_void,
                        0 as i32,
                        16 as i32 as u64,
                    );
                    1 as i32
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as i32
                } == 0)
                {
                    end_mark = (*parser).mark;
                    start_mark = end_mark;
                    's_43: loop {
                        if if (*parser).unread >= 4 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 4 as i32 as size_t)
                        } == 0
                        {
                            current_block = 10918313897556652803;
                            break;
                        }
                        if (*parser).mark.column == 0 as i32 as u64
                            && (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '-' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                    as i32 == '-' as i32 as yaml_char_t as i32
                                && *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                    as i32 == '-' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '.' as i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '.' as i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                        as i32 == '.' as i32 as yaml_char_t as i32)
                            && (*((*parser).buffer.pointer).offset(3 as i32 as isize)
                                as i32 == ' ' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                    as i32 == '\t' as i32 as yaml_char_t as i32
                                || (*((*parser).buffer.pointer).offset(3 as i32 as isize)
                                    as i32 == '\r' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == '\n' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -62i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -123i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 2 as i32) as isize) as i32
                                            == -88i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == -30i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 1 as i32) as isize) as i32
                                            == -128i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer)
                                            .offset((3 as i32 + 2 as i32) as isize) as i32
                                            == -87i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(3 as i32 as isize)
                                        as i32 == '\0' as i32 as yaml_char_t as i32))
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        if *((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == '#' as i32 as yaml_char_t as i32
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        while !(*((*parser).buffer.pointer).offset(0 as i32 as isize)
                            as i32 == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                            || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\r' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -62i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -123i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -87i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\0' as i32 as yaml_char_t as i32))
                        {
                            if (*parser).flow_level != 0
                                && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == ':' as i32 as yaml_char_t as i32
                                && (*((*parser).buffer.pointer).offset(1 as i32 as isize)
                                    as i32 == ',' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '?' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '[' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == ']' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '{' as i32 as yaml_char_t as i32
                                    || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == '}' as i32 as yaml_char_t as i32)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a plain scalar\0" as *const u8
                                        as *const i8,
                                    start_mark,
                                    b"found unexpected ':'\0" as *const u8 as *const i8,
                                );
                                current_block = 10918313897556652803;
                                break 's_43;
                            } else {
                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == ':' as i32 as yaml_char_t as i32
                                    && (*((*parser).buffer.pointer).offset(1 as i32 as isize)
                                        as i32 == ' ' as i32 as yaml_char_t as i32
                                        || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                            as i32 == '\t' as i32 as yaml_char_t as i32
                                        || (*((*parser).buffer.pointer).offset(1 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -62i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -123i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 2 as i32) as isize) as i32
                                                    == -88i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == -30i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 1 as i32) as isize) as i32
                                                    == -128i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer)
                                                    .offset((1 as i32 + 2 as i32) as isize) as i32
                                                    == -87i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == '\0' as i32 as yaml_char_t as i32))
                                    || (*parser).flow_level != 0
                                        && (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == ',' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '[' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == ']' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '{' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '}' as i32 as yaml_char_t as i32)
                                {
                                    break;
                                }
                                if leading_blanks != 0
                                    || whitespaces.start != whitespaces.pointer
                                {
                                    if leading_blanks != 0 {
                                        if *(leading_break.start).offset(0 as i32 as isize) as i32
                                            == '\n' as i32
                                        {
                                            if *(trailing_breaks.start).offset(0 as i32 as isize) as i32
                                                == '\0' as i32
                                            {
                                                if if (string.pointer).offset(5 as i32 as isize)
                                                    < string.end
                                                    || yaml_string_extend(
                                                        &mut string.start,
                                                        &mut string.pointer,
                                                        &mut string.end,
                                                    ) != 0
                                                {
                                                    1 as i32
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as i32
                                                } == 0
                                                {
                                                    current_block = 10918313897556652803;
                                                    break 's_43;
                                                }
                                                let fresh274 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh274 = ' ' as i32 as yaml_char_t;
                                            } else {
                                                if if yaml_string_join(
                                                    &mut string.start,
                                                    &mut string.pointer,
                                                    &mut string.end,
                                                    &mut trailing_breaks.start,
                                                    &mut trailing_breaks.pointer,
                                                    &mut trailing_breaks.end,
                                                ) != 0
                                                {
                                                    trailing_breaks.pointer = trailing_breaks.start;
                                                    1 as i32
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as i32
                                                } == 0
                                                {
                                                    current_block = 10918313897556652803;
                                                    break 's_43;
                                                }
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                memset(
                                                    trailing_breaks.start as *mut libc::c_void,
                                                    0 as i32,
                                                    (trailing_breaks.end).offset_from(trailing_breaks.start)
                                                        as i64 as u64,
                                                );
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0 as i32,
                                                (leading_break.end).offset_from(leading_break.start) as i64
                                                    as u64,
                                            );
                                        } else {
                                            if if yaml_string_join(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                                &mut leading_break.start,
                                                &mut leading_break.pointer,
                                                &mut leading_break.end,
                                            ) != 0
                                            {
                                                leading_break.pointer = leading_break.start;
                                                1 as i32
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as i32
                                            } == 0
                                            {
                                                current_block = 10918313897556652803;
                                                break 's_43;
                                            }
                                            if if yaml_string_join(
                                                &mut string.start,
                                                &mut string.pointer,
                                                &mut string.end,
                                                &mut trailing_breaks.start,
                                                &mut trailing_breaks.pointer,
                                                &mut trailing_breaks.end,
                                            ) != 0
                                            {
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                1 as i32
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as i32
                                            } == 0
                                            {
                                                current_block = 10918313897556652803;
                                                break 's_43;
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut libc::c_void,
                                                0 as i32,
                                                (leading_break.end).offset_from(leading_break.start) as i64
                                                    as u64,
                                            );
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            memset(
                                                trailing_breaks.start as *mut libc::c_void,
                                                0 as i32,
                                                (trailing_breaks.end).offset_from(trailing_breaks.start)
                                                    as i64 as u64,
                                            );
                                        }
                                        leading_blanks = 0 as i32;
                                    } else {
                                        if if yaml_string_join(
                                            &mut string.start,
                                            &mut string.pointer,
                                            &mut string.end,
                                            &mut whitespaces.start,
                                            &mut whitespaces.pointer,
                                            &mut whitespaces.end,
                                        ) != 0
                                        {
                                            whitespaces.pointer = whitespaces.start;
                                            1 as i32
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as i32
                                        } == 0
                                        {
                                            current_block = 10918313897556652803;
                                            break 's_43;
                                        }
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut libc::c_void,
                                            0 as i32,
                                            (whitespaces.end).offset_from(whitespaces.start) as i64
                                                as u64,
                                        );
                                    }
                                }
                                if if if (string.pointer).offset(5 as i32 as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &mut string.start,
                                        &mut string.pointer,
                                        &mut string.end,
                                    ) != 0
                                {
                                    1 as i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as i32
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                        == 0 as i32
                                    {
                                        let fresh275 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(1);
                                        let fresh276 = string.pointer;
                                        string.pointer = (string.pointer).offset(1);
                                        *fresh276 = *fresh275;
                                    } else {
                                        if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                            == 0xc0 as i32
                                        {
                                            let fresh277 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh278 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh278 = *fresh277;
                                            let fresh279 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh280 = string.pointer;
                                            string.pointer = (string.pointer).offset(1);
                                            *fresh280 = *fresh279;
                                        } else {
                                            if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                == 0xe0 as i32
                                            {
                                                let fresh281 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh282 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh282 = *fresh281;
                                                let fresh283 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh284 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh284 = *fresh283;
                                                let fresh285 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh286 = string.pointer;
                                                string.pointer = (string.pointer).offset(1);
                                                *fresh286 = *fresh285;
                                            } else {
                                                if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                    == 0xf0 as i32
                                                {
                                                    let fresh287 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh288 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh288 = *fresh287;
                                                    let fresh289 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh290 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh290 = *fresh289;
                                                    let fresh291 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh292 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh292 = *fresh291;
                                                    let fresh293 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh294 = string.pointer;
                                                    string.pointer = (string.pointer).offset(1);
                                                    *fresh294 = *fresh293;
                                                } else {};
                                            };
                                        };
                                    };
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    1 as i32
                                } else {
                                    0 as i32
                                } == 0
                                {
                                    current_block = 10918313897556652803;
                                    break 's_43;
                                }
                                end_mark = (*parser).mark;
                                if if (*parser).unread >= 2 as i32 as u64 {
                                    1 as i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                } == 0
                                {
                                    current_block = 10918313897556652803;
                                    break 's_43;
                                }
                            }
                        }
                        if !(*((*parser).buffer.pointer).offset(0 as i32 as isize) as i32
                            == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                            || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\r' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -62i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -123i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -87i32 as yaml_char_t as i32))
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        if if (*parser).unread >= 1 as i32 as u64 {
                            1 as i32
                        } else {
                            yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                        } == 0
                        {
                            current_block = 10918313897556652803;
                            break;
                        }
                        while *((*parser).buffer.pointer).offset(0 as i32 as isize)
                            as i32 == ' ' as i32 as yaml_char_t as i32
                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\t' as i32 as yaml_char_t as i32
                            || (*((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == '\r' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -62i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -123i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -88i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == -30i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 1 as i32) as isize) as i32
                                        == -128i32 as yaml_char_t as i32
                                    && *((*parser).buffer.pointer)
                                        .offset((0 as i32 + 2 as i32) as isize) as i32
                                        == -87i32 as yaml_char_t as i32)
                        {
                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                as i32 == ' ' as i32 as yaml_char_t as i32
                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                    as i32 == '\t' as i32 as yaml_char_t as i32
                            {
                                if leading_blanks != 0
                                    && ((*parser).mark.column as i32) < indent
                                    && *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\t' as i32 as yaml_char_t as i32
                                {
                                    yaml_parser_set_scanner_error(
                                        parser,
                                        b"while scanning a plain scalar\0" as *const u8
                                            as *const i8,
                                        start_mark,
                                        b"found a tab character that violates indentation\0"
                                            as *const u8 as *const i8,
                                    );
                                    current_block = 10918313897556652803;
                                    break 's_43;
                                } else if leading_blanks == 0 {
                                    if if if (whitespaces.pointer).offset(5 as i32 as isize)
                                        < whitespaces.end
                                        || yaml_string_extend(
                                            &mut whitespaces.start,
                                            &mut whitespaces.pointer,
                                            &mut whitespaces.end,
                                        ) != 0
                                    {
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as i32 & 0x80 as i32
                                            == 0 as i32
                                        {
                                            let fresh295 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            let fresh296 = whitespaces.pointer;
                                            whitespaces.pointer = (whitespaces.pointer).offset(1);
                                            *fresh296 = *fresh295;
                                        } else {
                                            if *(*parser).buffer.pointer as i32 & 0xe0 as i32
                                                == 0xc0 as i32
                                            {
                                                let fresh297 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh298 = whitespaces.pointer;
                                                whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                *fresh298 = *fresh297;
                                                let fresh299 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                let fresh300 = whitespaces.pointer;
                                                whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                *fresh300 = *fresh299;
                                            } else {
                                                if *(*parser).buffer.pointer as i32 & 0xf0 as i32
                                                    == 0xe0 as i32
                                                {
                                                    let fresh301 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh302 = whitespaces.pointer;
                                                    whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                    *fresh302 = *fresh301;
                                                    let fresh303 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh304 = whitespaces.pointer;
                                                    whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                    *fresh304 = *fresh303;
                                                    let fresh305 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh306 = whitespaces.pointer;
                                                    whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                    *fresh306 = *fresh305;
                                                } else {
                                                    if *(*parser).buffer.pointer as i32 & 0xf8 as i32
                                                        == 0xf0 as i32
                                                    {
                                                        let fresh307 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh308 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh308 = *fresh307;
                                                        let fresh309 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh310 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh310 = *fresh309;
                                                        let fresh311 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh312 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh312 = *fresh311;
                                                        let fresh313 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh314 = whitespaces.pointer;
                                                        whitespaces.pointer = (whitespaces.pointer).offset(1);
                                                        *fresh314 = *fresh313;
                                                    } else {};
                                                };
                                            };
                                        };
                                        (*parser).mark.index = ((*parser).mark.index)
                                            .wrapping_add(1);
                                        (*parser).mark.index;
                                        (*parser).mark.column = ((*parser).mark.column)
                                            .wrapping_add(1);
                                        (*parser).mark.column;
                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                        (*parser).unread;
                                        1 as i32
                                    } else {
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 10918313897556652803;
                                        break 's_43;
                                    }
                                } else {
                                    (*parser).mark.index = ((*parser).mark.index)
                                        .wrapping_add(1);
                                    (*parser).mark.index;
                                    (*parser).mark.column = ((*parser).mark.column)
                                        .wrapping_add(1);
                                    (*parser).mark.column;
                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                    (*parser).unread;
                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                        .offset(
                                            (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 & 0x80 as i32 == 0 as i32
                                            {
                                                1 as i32
                                            } else {
                                                (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 & 0xe0 as i32 == 0xc0 as i32
                                                {
                                                    2 as i32
                                                } else {
                                                    (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 & 0xf0 as i32 == 0xe0 as i32
                                                    {
                                                        3 as i32
                                                    } else {
                                                        (if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                            as i32 & 0xf8 as i32 == 0xf0 as i32
                                                        {
                                                            4 as i32
                                                        } else {
                                                            0 as i32
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                }
                            } else {
                                if if (*parser).unread >= 2 as i32 as u64 {
                                    1 as i32
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as i32 as size_t)
                                } == 0
                                {
                                    current_block = 10918313897556652803;
                                    break 's_43;
                                }
                                if leading_blanks == 0 {
                                    whitespaces.pointer = whitespaces.start;
                                    memset(
                                        whitespaces.start as *mut libc::c_void,
                                        0 as i32,
                                        (whitespaces.end).offset_from(whitespaces.start) as i64
                                            as u64,
                                    );
                                    if if if (leading_break.pointer).offset(5 as i32 as isize)
                                        < leading_break.end
                                        || yaml_string_extend(
                                            &mut leading_break.start,
                                            &mut leading_break.pointer,
                                            &mut leading_break.end,
                                        ) != 0
                                    {
                                        1 as i32
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as i32
                                    } != 0
                                    {
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                        {
                                            let fresh315 = leading_break.pointer;
                                            leading_break.pointer = (leading_break.pointer).offset(1);
                                            *fresh315 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(2 as i32 as isize);
                                            (*parser).mark.index = ((*parser).mark.index as u64)
                                                .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                            (*parser).mark.column = 0 as i32 as size_t;
                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                            (*parser).mark.line;
                                            (*parser).unread = ((*parser).unread as u64)
                                                .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                        } else {
                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\r' as i32 as yaml_char_t as i32
                                                || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == '\n' as i32 as yaml_char_t as i32
                                            {
                                                let fresh316 = leading_break.pointer;
                                                leading_break.pointer = (leading_break.pointer).offset(1);
                                                *fresh316 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(1);
                                                (*parser).buffer.pointer;
                                                (*parser).mark.index = ((*parser).mark.index)
                                                    .wrapping_add(1);
                                                (*parser).mark.index;
                                                (*parser).mark.column = 0 as i32 as size_t;
                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                (*parser).mark.line;
                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                (*parser).unread;
                                            } else {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -62i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                        as i32 == -123i32 as yaml_char_t as i32
                                                {
                                                    let fresh317 = leading_break.pointer;
                                                    leading_break.pointer = (leading_break.pointer).offset(1);
                                                    *fresh317 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(2 as i32 as isize);
                                                    (*parser).mark.index = ((*parser).mark.index)
                                                        .wrapping_add(1);
                                                    (*parser).mark.index;
                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                    (*parser).mark.line;
                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                    (*parser).unread;
                                                } else {
                                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                        as i32 == -30i32 as yaml_char_t as i32
                                                        && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                            as i32 == -128i32 as yaml_char_t as i32
                                                        && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                            as i32 == -88i32 as yaml_char_t as i32
                                                            || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                                as i32 == -87i32 as yaml_char_t as i32)
                                                    {
                                                        let fresh318 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh319 = leading_break.pointer;
                                                        leading_break.pointer = (leading_break.pointer).offset(1);
                                                        *fresh319 = *fresh318;
                                                        let fresh320 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh321 = leading_break.pointer;
                                                        leading_break.pointer = (leading_break.pointer).offset(1);
                                                        *fresh321 = *fresh320;
                                                        let fresh322 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                            .offset(1);
                                                        let fresh323 = leading_break.pointer;
                                                        leading_break.pointer = (leading_break.pointer).offset(1);
                                                        *fresh323 = *fresh322;
                                                        (*parser).mark.index = ((*parser).mark.index)
                                                            .wrapping_add(1);
                                                        (*parser).mark.index;
                                                        (*parser).mark.column = 0 as i32 as size_t;
                                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                        (*parser).mark.line;
                                                        (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                        (*parser).unread;
                                                    } else {};
                                                };
                                            };
                                        };
                                        1 as i32
                                    } else {
                                        0 as i32
                                    } == 0
                                    {
                                        current_block = 10918313897556652803;
                                        break 's_43;
                                    }
                                    leading_blanks = 1 as i32;
                                } else if if if (trailing_breaks.pointer)
                                    .offset(5 as i32 as isize) < trailing_breaks.end
                                    || yaml_string_extend(
                                        &mut trailing_breaks.start,
                                        &mut trailing_breaks.pointer,
                                        &mut trailing_breaks.end,
                                    ) != 0
                                {
                                    1 as i32
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as i32
                                } != 0
                                {
                                    if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                        as i32 == '\r' as i32 as yaml_char_t as i32
                                        && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                            as i32 == '\n' as i32 as yaml_char_t as i32
                                    {
                                        let fresh324 = trailing_breaks.pointer;
                                        trailing_breaks.pointer = (trailing_breaks.pointer)
                                            .offset(1);
                                        *fresh324 = '\n' as i32 as yaml_char_t;
                                        (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                            .offset(2 as i32 as isize);
                                        (*parser).mark.index = ((*parser).mark.index as u64)
                                            .wrapping_add(2 as i32 as u64) as size_t as size_t;
                                        (*parser).mark.column = 0 as i32 as size_t;
                                        (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                        (*parser).mark.line;
                                        (*parser).unread = ((*parser).unread as u64)
                                            .wrapping_sub(2 as i32 as u64) as size_t as size_t;
                                    } else {
                                        if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                            as i32 == '\r' as i32 as yaml_char_t as i32
                                            || *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == '\n' as i32 as yaml_char_t as i32
                                        {
                                            let fresh325 = trailing_breaks.pointer;
                                            trailing_breaks.pointer = (trailing_breaks.pointer)
                                                .offset(1);
                                            *fresh325 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                .offset(1);
                                            (*parser).buffer.pointer;
                                            (*parser).mark.index = ((*parser).mark.index)
                                                .wrapping_add(1);
                                            (*parser).mark.index;
                                            (*parser).mark.column = 0 as i32 as size_t;
                                            (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                            (*parser).mark.line;
                                            (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                            (*parser).unread;
                                        } else {
                                            if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                as i32 == -62i32 as yaml_char_t as i32
                                                && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                    as i32 == -123i32 as yaml_char_t as i32
                                            {
                                                let fresh326 = trailing_breaks.pointer;
                                                trailing_breaks.pointer = (trailing_breaks.pointer)
                                                    .offset(1);
                                                *fresh326 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                    .offset(2 as i32 as isize);
                                                (*parser).mark.index = ((*parser).mark.index)
                                                    .wrapping_add(1);
                                                (*parser).mark.index;
                                                (*parser).mark.column = 0 as i32 as size_t;
                                                (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                (*parser).mark.line;
                                                (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                (*parser).unread;
                                            } else {
                                                if *((*parser).buffer.pointer).offset(0 as i32 as isize)
                                                    as i32 == -30i32 as yaml_char_t as i32
                                                    && *((*parser).buffer.pointer).offset(1 as i32 as isize)
                                                        as i32 == -128i32 as yaml_char_t as i32
                                                    && (*((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                        as i32 == -88i32 as yaml_char_t as i32
                                                        || *((*parser).buffer.pointer).offset(2 as i32 as isize)
                                                            as i32 == -87i32 as yaml_char_t as i32)
                                                {
                                                    let fresh327 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh328 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer = (trailing_breaks.pointer)
                                                        .offset(1);
                                                    *fresh328 = *fresh327;
                                                    let fresh329 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh330 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer = (trailing_breaks.pointer)
                                                        .offset(1);
                                                    *fresh330 = *fresh329;
                                                    let fresh331 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer = ((*parser).buffer.pointer)
                                                        .offset(1);
                                                    let fresh332 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer = (trailing_breaks.pointer)
                                                        .offset(1);
                                                    *fresh332 = *fresh331;
                                                    (*parser).mark.index = ((*parser).mark.index)
                                                        .wrapping_add(1);
                                                    (*parser).mark.index;
                                                    (*parser).mark.column = 0 as i32 as size_t;
                                                    (*parser).mark.line = ((*parser).mark.line).wrapping_add(1);
                                                    (*parser).mark.line;
                                                    (*parser).unread = ((*parser).unread).wrapping_sub(1);
                                                    (*parser).unread;
                                                } else {};
                                            };
                                        };
                                    };
                                    1 as i32
                                } else {
                                    0 as i32
                                } == 0
                                {
                                    current_block = 10918313897556652803;
                                    break 's_43;
                                }
                            }
                            if if (*parser).unread >= 1 as i32 as u64 {
                                1 as i32
                            } else {
                                yaml_parser_update_buffer(parser, 1 as i32 as size_t)
                            } == 0
                            {
                                current_block = 10918313897556652803;
                                break 's_43;
                            }
                        }
                        if (*parser).flow_level == 0
                            && ((*parser).mark.column as i32) < indent
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                    }
                    match current_block {
                        10918313897556652803 => {}
                        _ => {
                            memset(
                                token as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_token_t>() as u64,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.scalar.value = string.start;
                            (*token).data.scalar.length = (string.pointer)
                                .offset_from(string.start) as i64 as size_t;
                            (*token).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                            if leading_blanks != 0 {
                                (*parser).simple_key_allowed = 1 as i32;
                            }
                            yaml_free(leading_break.start as *mut libc::c_void);
                            leading_break.end = 0 as *mut yaml_char_t;
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut libc::c_void);
                            trailing_breaks.end = 0 as *mut yaml_char_t;
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut libc::c_void);
                            whitespaces.end = 0 as *mut yaml_char_t;
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1 as i32;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut libc::c_void);
    string.end = 0 as *mut yaml_char_t;
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut libc::c_void);
    leading_break.end = 0 as *mut yaml_char_t;
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut libc::c_void);
    trailing_breaks.end = 0 as *mut yaml_char_t;
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut libc::c_void);
    whitespaces.end = 0 as *mut yaml_char_t;
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    return 0 as i32;
}