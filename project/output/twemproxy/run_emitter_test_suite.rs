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
    static mut stdin: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn abort() -> !;
    fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32;
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: i32);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
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
    fn yaml_alias_event_initialize(
        event: *mut yaml_event_t,
        anchor: *const yaml_char_t,
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
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> i32;
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut FILE);
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: i32);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
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
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_7,
    pub document_start: C2RustUnnamed_5,
    pub document_end: C2RustUnnamed_4,
    pub alias: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub sequence_start: C2RustUnnamed_1,
    pub mapping_start: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub implicit: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_6,
    pub implicit: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub data: C2RustUnnamed_8,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub scalar: C2RustUnnamed_13,
    pub sequence: C2RustUnnamed_11,
    pub mapping: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pairs: C2RustUnnamed_10,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub items: C2RustUnnamed_12,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_15,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub start_implicit: i32,
    pub end_implicit: i32,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
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
    pub output: C2RustUnnamed_25,
    pub buffer: C2RustUnnamed_24,
    pub raw_buffer: C2RustUnnamed_23,
    pub encoding: yaml_encoding_t,
    pub canonical: i32,
    pub best_indent: i32,
    pub best_width: i32,
    pub unicode: i32,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_22,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_21,
    pub indents: C2RustUnnamed_20,
    pub tag_directives: C2RustUnnamed_19,
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
    pub anchor_data: C2RustUnnamed_18,
    pub tag_data: C2RustUnnamed_17,
    pub scalar_data: C2RustUnnamed_16,
    pub opened: i32,
    pub closed: i32,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: i32,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
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
pub struct C2RustUnnamed_17 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
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
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub string: C2RustUnnamed_26,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub buffer: *mut u8,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut current_block: u64;
    let mut input: *mut FILE = 0 as *mut FILE;
    let mut emitter: yaml_emitter_t = yaml_emitter_t {
        error: YAML_NO_ERROR,
        problem: 0 as *const i8,
        write_handler: None,
        write_handler_data: 0 as *mut libc::c_void,
        output: C2RustUnnamed_25 {
            string: C2RustUnnamed_26 {
                buffer: 0 as *mut u8,
                size: 0,
                size_written: 0 as *mut size_t,
            },
        },
        buffer: C2RustUnnamed_24 {
            start: 0 as *mut yaml_char_t,
            end: 0 as *mut yaml_char_t,
            pointer: 0 as *mut yaml_char_t,
            last: 0 as *mut yaml_char_t,
        },
        raw_buffer: C2RustUnnamed_23 {
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
        states: C2RustUnnamed_22 {
            start: 0 as *mut yaml_emitter_state_t,
            end: 0 as *mut yaml_emitter_state_t,
            top: 0 as *mut yaml_emitter_state_t,
        },
        state: YAML_EMIT_STREAM_START_STATE,
        events: C2RustUnnamed_21 {
            start: 0 as *mut yaml_event_t,
            end: 0 as *mut yaml_event_t,
            head: 0 as *mut yaml_event_t,
            tail: 0 as *mut yaml_event_t,
        },
        indents: C2RustUnnamed_20 {
            start: 0 as *mut i32,
            end: 0 as *mut i32,
            top: 0 as *mut i32,
        },
        tag_directives: C2RustUnnamed_19 {
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
        anchor_data: C2RustUnnamed_18 {
            anchor: 0 as *mut yaml_char_t,
            anchor_length: 0,
            alias: 0,
        },
        tag_data: C2RustUnnamed_17 {
            handle: 0 as *mut yaml_char_t,
            handle_length: 0,
            suffix: 0 as *mut yaml_char_t,
            suffix_length: 0,
        },
        scalar_data: C2RustUnnamed_16 {
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
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut version_directive: *mut yaml_version_directive_t = 0
        as *mut yaml_version_directive_t;
    let mut canonical: i32 = 0 as i32;
    let mut unicode: i32 = 0 as i32;
    let mut line: [i8; 1024] = [0; 1024];
    let mut foundfile: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut minor: i32 = 0 as i32;
    let mut flow: i32 = -(1 as i32);
    i = 1 as i32;
    while i < argc {
        if strncmp(
            *argv.offset(i as isize),
            b"--help\0" as *const u8 as *const i8,
            6 as i32 as u64,
        ) == 0 as i32
        {
            return usage(0 as i32);
        }
        if strncmp(
            *argv.offset(i as isize),
            b"-h\0" as *const u8 as *const i8,
            2 as i32 as u64,
        ) == 0 as i32
        {
            return usage(0 as i32);
        }
        if strncmp(
            *argv.offset(i as isize),
            b"--flow\0" as *const u8 as *const i8,
            6 as i32 as u64,
        ) == 0 as i32
        {
            if i + 1 as i32 == argc {
                return usage(1 as i32);
            }
            i += 1;
            i;
            if strncmp(
                *argv.offset(i as isize),
                b"keep\0" as *const u8 as *const i8,
                4 as i32 as u64,
            ) == 0 as i32
            {
                flow = 0 as i32;
            } else if strncmp(
                *argv.offset(i as isize),
                b"on\0" as *const u8 as *const i8,
                2 as i32 as u64,
            ) == 0 as i32
            {
                flow = 1 as i32;
            } else if strncmp(
                *argv.offset(i as isize),
                b"off\0" as *const u8 as *const i8,
                3 as i32 as u64,
            ) == 0 as i32
            {
                flow = -(1 as i32);
            } else {
                return usage(1 as i32)
            }
        } else if strncmp(
            *argv.offset(i as isize),
            b"--directive\0" as *const u8 as *const i8,
            11 as i32 as u64,
        ) == 0 as i32
        {
            if i + 1 as i32 == argc {
                return usage(1 as i32);
            }
            i += 1;
            i;
            if strncmp(
                *argv.offset(i as isize),
                b"1.1\0" as *const u8 as *const i8,
                3 as i32 as u64,
            ) == 0 as i32
            {
                minor = 1 as i32;
            } else if strncmp(
                *argv.offset(i as isize),
                b"1.2\0" as *const u8 as *const i8,
                3 as i32 as u64,
            ) == 0 as i32
            {
                minor = 2 as i32;
            } else {
                return usage(1 as i32)
            }
        } else if foundfile == 0 {
            input = fopen(*argv.offset(i as isize), b"rb\0" as *const u8 as *const i8);
            foundfile = 1 as i32;
        }
        i += 1;
        i;
    }
    if minor != 0 {
        version_directive = yaml_malloc(
            ::core::mem::size_of::<yaml_version_directive_t>() as u64,
        ) as *mut yaml_version_directive_t;
        (*version_directive).major = 1 as i32;
        (*version_directive).minor = minor;
    }
    if foundfile == 0 {
        input = stdin;
    }
    if !input.is_null() {} else {
        __assert_fail(
            b"input\0" as *const u8 as *const i8,
            b"run-emitter-test-suite.c\0" as *const u8 as *const i8,
            72 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 23],
                &[i8; 23],
            >(b"int main(int, char **)\0"))
                .as_ptr(),
        );
    }
    'c_4461: {
        if !input.is_null() {} else {
            __assert_fail(
                b"input\0" as *const u8 as *const i8,
                b"run-emitter-test-suite.c\0" as *const u8 as *const i8,
                72 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 23],
                    &[i8; 23],
                >(b"int main(int, char **)\0"))
                    .as_ptr(),
            );
        }
    };
    if yaml_emitter_initialize(&mut emitter) == 0 {
        fprintf(
            stderr,
            b"Could not initalize the emitter object\n\0" as *const u8 as *const i8,
        );
        return 1 as i32;
    }
    yaml_emitter_set_output_file(&mut emitter, stdout);
    yaml_emitter_set_canonical(&mut emitter, canonical);
    yaml_emitter_set_unicode(&mut emitter, unicode);
    loop {
        if !(get_line(input, line.as_mut_ptr()) != 0) {
            current_block = 14329534724295951598;
            break;
        }
        let mut ok: i32 = 0;
        let mut anchor: [i8; 256] = [0; 256];
        let mut tag: [i8; 256] = [0; 256];
        let mut implicit: i32 = 0;
        let mut style: i32 = 0;
        if strncmp(
            line.as_mut_ptr(),
            b"+STR\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            ok = yaml_stream_start_event_initialize(&mut event, YAML_UTF8_ENCODING);
        } else if strncmp(
            line.as_mut_ptr(),
            b"-STR\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            ok = yaml_stream_end_event_initialize(&mut event);
        } else if strncmp(
            line.as_mut_ptr(),
            b"+DOC\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            implicit = (strncmp(
                line.as_mut_ptr().offset(4 as i32 as isize),
                b" ---\0" as *const u8 as *const i8,
                4 as i32 as u64,
            ) != 0 as i32) as i32;
            ok = yaml_document_start_event_initialize(
                &mut event,
                version_directive,
                0 as *mut yaml_tag_directive_t,
                0 as *mut yaml_tag_directive_t,
                implicit,
            );
        } else if strncmp(
            line.as_mut_ptr(),
            b"-DOC\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            implicit = (strncmp(
                line.as_mut_ptr().offset(4 as i32 as isize),
                b" ...\0" as *const u8 as *const i8,
                4 as i32 as u64,
            ) != 0 as i32) as i32;
            ok = yaml_document_end_event_initialize(&mut event, implicit);
        } else if strncmp(
            line.as_mut_ptr(),
            b"+MAP\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            style = YAML_BLOCK_MAPPING_STYLE as i32;
            if flow == 1 as i32 {
                style = YAML_FLOW_MAPPING_STYLE as i32;
            } else if flow == 0 as i32
                && strncmp(
                    line.as_mut_ptr().offset(5 as i32 as isize),
                    b"{}\0" as *const u8 as *const i8,
                    2 as i32 as u64,
                ) == 0 as i32
            {
                style = YAML_FLOW_MAPPING_STYLE as i32;
            }
            ok = yaml_mapping_start_event_initialize(
                &mut event,
                get_anchor('&' as i32 as i8, line.as_mut_ptr(), anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as i32,
                style as yaml_mapping_style_t,
            );
        } else if strncmp(
            line.as_mut_ptr(),
            b"-MAP\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            ok = yaml_mapping_end_event_initialize(&mut event);
        } else if strncmp(
            line.as_mut_ptr(),
            b"+SEQ\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            style = YAML_BLOCK_SEQUENCE_STYLE as i32;
            if flow == 1 as i32 {
                style = YAML_FLOW_MAPPING_STYLE as i32;
            } else if flow == 0 as i32
                && strncmp(
                    line.as_mut_ptr().offset(5 as i32 as isize),
                    b"[]\0" as *const u8 as *const i8,
                    2 as i32 as u64,
                ) == 0 as i32
            {
                style = YAML_FLOW_SEQUENCE_STYLE as i32;
            }
            ok = yaml_sequence_start_event_initialize(
                &mut event,
                get_anchor('&' as i32 as i8, line.as_mut_ptr(), anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                0 as i32,
                style as yaml_sequence_style_t,
            );
        } else if strncmp(
            line.as_mut_ptr(),
            b"-SEQ\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            ok = yaml_sequence_end_event_initialize(&mut event);
        } else if strncmp(
            line.as_mut_ptr(),
            b"=VAL\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            let mut value: [i8; 1024] = [0; 1024];
            let mut style_0: i32 = 0;
            get_value(line.as_mut_ptr(), value.as_mut_ptr(), &mut style_0);
            implicit = (get_tag(line.as_mut_ptr(), tag.as_mut_ptr())
                == 0 as *mut libc::c_void as *mut i8) as i32;
            ok = yaml_scalar_event_initialize(
                &mut event,
                get_anchor('&' as i32 as i8, line.as_mut_ptr(), anchor.as_mut_ptr())
                    as *mut yaml_char_t,
                get_tag(line.as_mut_ptr(), tag.as_mut_ptr()) as *mut yaml_char_t,
                value.as_mut_ptr() as *mut yaml_char_t,
                -(1 as i32),
                implicit,
                implicit,
                style_0 as yaml_scalar_style_t,
            );
        } else if strncmp(
            line.as_mut_ptr(),
            b"=ALI\0" as *const u8 as *const i8,
            4 as i32 as u64,
        ) == 0 as i32
        {
            ok = yaml_alias_event_initialize(
                &mut event,
                get_anchor('*' as i32 as i8, line.as_mut_ptr(), anchor.as_mut_ptr())
                    as *mut yaml_char_t,
            );
        } else {
            fprintf(
                stderr,
                b"Unknown event: '%s'\n\0" as *const u8 as *const i8,
                line.as_mut_ptr(),
            );
            fflush(stdout);
            return 1 as i32;
        }
        if ok == 0 {
            current_block = 17552154457271152869;
            break;
        }
        if yaml_emitter_emit(&mut emitter, &mut event) == 0 {
            current_block = 15386843355306702351;
            break;
        }
    }
    match current_block {
        17552154457271152869 => {
            fprintf(
                stderr,
                b"Memory error: Not enough memory for creating an event\n\0" as *const u8
                    as *const i8,
            );
            yaml_emitter_delete(&mut emitter);
            return 1 as i32;
        }
        15386843355306702351 => {
            match emitter.error as u32 {
                1 => {
                    fprintf(
                        stderr,
                        b"Memory error: Not enough memory for emitting\n\0" as *const u8
                            as *const i8,
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
            yaml_emitter_delete(&mut emitter);
            return 1 as i32;
        }
        _ => {
            if fclose(input) == 0 {} else {
                __assert_fail(
                    b"!fclose(input)\0" as *const u8 as *const i8,
                    b"run-emitter-test-suite.c\0" as *const u8 as *const i8,
                    157 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 23],
                        &[i8; 23],
                    >(b"int main(int, char **)\0"))
                        .as_ptr(),
                );
            }
            'c_3822: {
                if fclose(input) == 0 {} else {
                    __assert_fail(
                        b"!fclose(input)\0" as *const u8 as *const i8,
                        b"run-emitter-test-suite.c\0" as *const u8 as *const i8,
                        157 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 23],
                            &[i8; 23],
                        >(b"int main(int, char **)\0"))
                            .as_ptr(),
                    );
                }
            };
            yaml_emitter_delete(&mut emitter);
            fflush(stdout);
            return 0 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_line(mut input: *mut FILE, mut line: *mut i8) -> i32 {
    let mut newline: *mut i8 = 0 as *mut i8;
    if (fgets(line, 1024 as i32 - 1 as i32, input)).is_null() {
        return 0 as i32;
    }
    newline = strchr(line, '\n' as i32);
    if newline.is_null() {
        fprintf(stderr, b"Line too long: '%s'\0" as *const u8 as *const i8, line);
        abort();
    }
    *newline = '\0' as i32 as i8;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn get_anchor(
    mut sigil: i8,
    mut line: *mut i8,
    mut anchor: *mut i8,
) -> *mut i8 {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    start = strchr(line, sigil as i32);
    if start.is_null() {
        return 0 as *mut i8;
    }
    start = start.offset(1);
    start;
    end = strchr(start, ' ' as i32);
    if end.is_null() {
        end = line.offset(strlen(line) as isize);
    }
    memcpy(
        anchor as *mut libc::c_void,
        start as *const libc::c_void,
        end.offset_from(start) as i64 as u64,
    );
    *anchor.offset(end.offset_from(start) as i64 as isize) = '\0' as i32 as i8;
    return anchor;
}
#[no_mangle]
pub unsafe extern "C" fn get_tag(mut line: *mut i8, mut tag: *mut i8) -> *mut i8 {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    start = strchr(line, '<' as i32);
    if start.is_null() {
        return 0 as *mut i8;
    }
    end = strchr(line, '>' as i32);
    if end.is_null() {
        return 0 as *mut i8;
    }
    memcpy(
        tag as *mut libc::c_void,
        start.offset(1 as i32 as isize) as *const libc::c_void,
        (end.offset_from(start) as i64 - 1 as i32 as i64) as u64,
    );
    *tag.offset((end.offset_from(start) as i64 - 1 as i32 as i64) as isize) = '\0' as i32
        as i8;
    return tag;
}
#[no_mangle]
pub unsafe extern "C" fn get_value(
    mut line: *mut i8,
    mut value: *mut i8,
    mut style: *mut i32,
) {
    let mut i: i32 = 0 as i32;
    let mut c: *mut i8 = 0 as *mut i8;
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = line.offset(strlen(line) as isize);
    let mut current_block_8: u64;
    c = line.offset(4 as i32 as isize);
    while c < end {
        if *c as i32 == ' ' as i32 {
            start = c.offset(1 as i32 as isize);
            if *start as i32 == ':' as i32 {
                *style = YAML_PLAIN_SCALAR_STYLE as i32;
                current_block_8 = 10599921512955367680;
            } else if *start as i32 == '\'' as i32 {
                *style = YAML_SINGLE_QUOTED_SCALAR_STYLE as i32;
                current_block_8 = 10599921512955367680;
            } else if *start as i32 == '"' as i32 {
                *style = YAML_DOUBLE_QUOTED_SCALAR_STYLE as i32;
                current_block_8 = 10599921512955367680;
            } else if *start as i32 == '|' as i32 {
                *style = YAML_LITERAL_SCALAR_STYLE as i32;
                current_block_8 = 10599921512955367680;
            } else if *start as i32 == '>' as i32 {
                *style = YAML_FOLDED_SCALAR_STYLE as i32;
                current_block_8 = 10599921512955367680;
            } else {
                start = 0 as *mut i8;
                current_block_8 = 8258075665625361029;
            }
            match current_block_8 {
                8258075665625361029 => {}
                _ => {
                    start = start.offset(1);
                    start;
                    break;
                }
            }
        }
        c = c.offset(1);
        c;
    }
    if start.is_null() {
        abort();
    }
    c = start;
    while c < end {
        if *c as i32 == '\\' as i32 {
            c = c.offset(1);
            if *c as i32 == '\\' as i32 {
                let fresh0 = i;
                i = i + 1;
                *value.offset(fresh0 as isize) = '\\' as i32 as i8;
            } else if *c as i32 == '0' as i32 {
                let fresh1 = i;
                i = i + 1;
                *value.offset(fresh1 as isize) = '\0' as i32 as i8;
            } else if *c as i32 == 'b' as i32 {
                let fresh2 = i;
                i = i + 1;
                *value.offset(fresh2 as isize) = '\u{8}' as i32 as i8;
            } else if *c as i32 == 'n' as i32 {
                let fresh3 = i;
                i = i + 1;
                *value.offset(fresh3 as isize) = '\n' as i32 as i8;
            } else if *c as i32 == 'r' as i32 {
                let fresh4 = i;
                i = i + 1;
                *value.offset(fresh4 as isize) = '\r' as i32 as i8;
            } else if *c as i32 == 't' as i32 {
                let fresh5 = i;
                i = i + 1;
                *value.offset(fresh5 as isize) = '\t' as i32 as i8;
            } else {
                abort();
            }
        } else {
            let fresh6 = i;
            i = i + 1;
            *value.offset(fresh6 as isize) = *c;
        }
        c = c.offset(1);
        c;
    }
    *value.offset(i as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn usage(mut ret: i32) -> i32 {
    fprintf(
        stderr,
        b"Usage: run-emitter-test-suite [--directive (1.1|1.2)] [--flow (on|off|keep)] [<input-file>]\n\0"
            as *const u8 as *const i8,
    );
    return ret;
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