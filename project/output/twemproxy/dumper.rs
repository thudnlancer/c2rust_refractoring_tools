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
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn yaml_malloc(size: size_t) -> *mut libc::c_void;
    fn yaml_free(ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32;
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
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(mut emitter: *mut yaml_emitter_t) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            67 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[i8; 40],
            >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3177: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                67 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[i8; 40],
                >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*emitter).opened == 0 {} else {
        __assert_fail(
            b"!emitter->opened\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            68 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[i8; 40],
            >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3128: {
        if (*emitter).opened == 0 {} else {
            __assert_fail(
                b"!emitter->opened\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                68 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[i8; 40],
                >(b"int yaml_emitter_open(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_STREAM_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.stream_start.encoding = YAML_ANY_ENCODING;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    (*emitter).opened = 1 as i32;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(mut emitter: *mut yaml_emitter_t) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            91 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3334: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                91 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*emitter).opened != 0 {} else {
        __assert_fail(
            b"emitter->opened\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            92 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3296: {
        if (*emitter).opened != 0 {} else {
            __assert_fail(
                b"emitter->opened\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                92 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"int yaml_emitter_close(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*emitter).closed != 0 {
        return 1 as i32;
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_STREAM_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    (*emitter).closed = 1 as i32;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    mut emitter: *mut yaml_emitter_t,
    mut document: *mut yaml_document_t,
) -> i32 {
    let mut current_block: u64;
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            117 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5195: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                117 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if !document.is_null() {} else {
        __assert_fail(
            b"document\0" as *const u8 as *const i8,
            b"dumper.c\0" as *const u8 as *const i8,
            118 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 59],
                &[i8; 59],
            >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                .as_ptr(),
        );
    }
    'c_5163: {
        if !document.is_null() {} else {
            __assert_fail(
                b"document\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                118 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 59],
                    &[i8; 59],
                >(b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter).document = document;
    if (*emitter).opened == 0 {
        if yaml_emitter_open(emitter) == 0 {
            current_block = 4342828966645461001;
        } else {
            current_block = 7502529970979898288;
        }
    } else {
        current_block = 7502529970979898288;
    }
    match current_block {
        7502529970979898288 => {
            if (*document).nodes.start == (*document).nodes.top {
                if !(yaml_emitter_close(emitter) == 0) {
                    yaml_emitter_delete_document_and_anchors(emitter);
                    return 1 as i32;
                }
            } else {
                if (*emitter).opened != 0 {} else {
                    __assert_fail(
                        b"emitter->opened\0" as *const u8 as *const i8,
                        b"dumper.c\0" as *const u8 as *const i8,
                        132 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 59],
                            &[i8; 59],
                        >(
                            b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_5073: {
                    if (*emitter).opened != 0 {} else {
                        __assert_fail(
                            b"emitter->opened\0" as *const u8 as *const i8,
                            b"dumper.c\0" as *const u8 as *const i8,
                            132 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 59],
                                &[i8; 59],
                            >(
                                b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                (*emitter).anchors = yaml_malloc(
                    (::core::mem::size_of::<yaml_anchors_t>() as u64)
                        .wrapping_mul(
                            ((*document).nodes.top).offset_from((*document).nodes.start)
                                as i64 as u64,
                        ),
                ) as *mut yaml_anchors_t;
                if !((*emitter).anchors).is_null() {
                    memset(
                        (*emitter).anchors as *mut libc::c_void,
                        0 as i32,
                        (::core::mem::size_of::<yaml_anchors_t>() as u64)
                            .wrapping_mul(
                                ((*document).nodes.top).offset_from((*document).nodes.start)
                                    as i64 as u64,
                            ),
                    );
                    memset(
                        &mut event as *mut yaml_event_t as *mut libc::c_void,
                        0 as i32,
                        ::core::mem::size_of::<yaml_event_t>() as u64,
                    );
                    event.type_0 = YAML_DOCUMENT_START_EVENT;
                    event.start_mark = mark;
                    event.end_mark = mark;
                    event.data.document_start.version_directive = (*document)
                        .version_directive;
                    event.data.document_start.tag_directives.start = (*document)
                        .tag_directives
                        .start;
                    event.data.document_start.tag_directives.end = (*document)
                        .tag_directives
                        .end;
                    event.data.document_start.implicit = (*document).start_implicit;
                    if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                        yaml_emitter_anchor_node(emitter, 1 as i32);
                        if !(yaml_emitter_dump_node(emitter, 1 as i32) == 0) {
                            memset(
                                &mut event as *mut yaml_event_t as *mut libc::c_void,
                                0 as i32,
                                ::core::mem::size_of::<yaml_event_t>() as u64,
                            );
                            event.type_0 = YAML_DOCUMENT_END_EVENT;
                            event.start_mark = mark;
                            event.end_mark = mark;
                            event.data.document_end.implicit = (*document).end_implicit;
                            if !(yaml_emitter_emit(emitter, &mut event) == 0) {
                                yaml_emitter_delete_document_and_anchors(emitter);
                                return 1 as i32;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_emitter_delete_document_and_anchors(emitter);
    return 0 as i32;
}
unsafe extern "C" fn yaml_emitter_delete_document_and_anchors(
    mut emitter: *mut yaml_emitter_t,
) {
    let mut index: i32 = 0;
    if ((*emitter).anchors).is_null() {
        yaml_document_delete((*emitter).document);
        (*emitter).document = 0 as *mut yaml_document_t;
        return;
    }
    index = 0 as i32;
    while ((*(*emitter).document).nodes.start).offset(index as isize)
        < (*(*emitter).document).nodes.top
    {
        let mut node: yaml_node_t = *((*(*emitter).document).nodes.start)
            .offset(index as isize);
        if (*((*emitter).anchors).offset(index as isize)).serialized == 0 {
            yaml_free(node.tag as *mut libc::c_void);
            if node.type_0 as u32 == YAML_SCALAR_NODE as i32 as u32 {
                yaml_free(node.data.scalar.value as *mut libc::c_void);
            }
        }
        if node.type_0 as u32 == YAML_SEQUENCE_NODE as i32 as u32 {
            yaml_free(node.data.sequence.items.start as *mut libc::c_void);
            node.data.sequence.items.end = 0 as *mut yaml_node_item_t;
            node.data.sequence.items.top = node.data.sequence.items.end;
            node.data.sequence.items.start = node.data.sequence.items.top;
        }
        if node.type_0 as u32 == YAML_MAPPING_NODE as i32 as u32 {
            yaml_free(node.data.mapping.pairs.start as *mut libc::c_void);
            node.data.mapping.pairs.end = 0 as *mut yaml_node_pair_t;
            node.data.mapping.pairs.top = node.data.mapping.pairs.end;
            node.data.mapping.pairs.start = node.data.mapping.pairs.top;
        }
        index += 1;
        index;
    }
    yaml_free((*(*emitter).document).nodes.start as *mut libc::c_void);
    (*(*emitter).document).nodes.end = 0 as *mut yaml_node_t;
    (*(*emitter).document).nodes.top = (*(*emitter).document).nodes.end;
    (*(*emitter).document).nodes.start = (*(*emitter).document).nodes.top;
    yaml_free((*emitter).anchors as *mut libc::c_void);
    (*emitter).anchors = 0 as *mut yaml_anchors_t;
    (*emitter).last_anchor_id = 0 as i32;
    (*emitter).document = 0 as *mut yaml_document_t;
}
unsafe extern "C" fn yaml_emitter_anchor_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: i32,
) {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .offset(index as isize)
        .offset(-(1 as i32 as isize));
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    let ref mut fresh0 = (*((*emitter).anchors).offset((index - 1 as i32) as isize))
        .references;
    *fresh0 += 1;
    *fresh0;
    if (*((*emitter).anchors).offset((index - 1 as i32) as isize)).references == 1 as i32
    {
        match (*node).type_0 as u32 {
            2 => {
                item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.offset(1);
                    item;
                }
            }
            3 => {
                pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.offset(1);
                    pair;
                }
            }
            _ => {}
        }
    } else if (*((*emitter).anchors).offset((index - 1 as i32) as isize)).references
        == 2 as i32
    {
        (*emitter).last_anchor_id += 1;
        (*((*emitter).anchors).offset((index - 1 as i32) as isize)).anchor = (*emitter)
            .last_anchor_id;
    }
}
unsafe extern "C" fn yaml_emitter_generate_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor_id: i32,
) -> *mut yaml_char_t {
    let mut anchor: *mut yaml_char_t = yaml_malloc(16 as i32 as size_t)
        as *mut yaml_char_t;
    if anchor.is_null() {
        return 0 as *mut yaml_char_t;
    }
    sprintf(anchor as *mut i8, b"id%03d\0" as *const u8 as *const i8, anchor_id);
    return anchor;
}
unsafe extern "C" fn yaml_emitter_dump_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: i32,
) -> i32 {
    let mut node: *mut yaml_node_t = ((*(*emitter).document).nodes.start)
        .offset(index as isize)
        .offset(-(1 as i32 as isize));
    let mut anchor_id: i32 = (*((*emitter).anchors).offset((index - 1 as i32) as isize))
        .anchor;
    let mut anchor: *mut yaml_char_t = 0 as *mut yaml_char_t;
    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() {
            return 0 as i32;
        }
    }
    if (*((*emitter).anchors).offset((index - 1 as i32) as isize)).serialized != 0 {
        return yaml_emitter_dump_alias(emitter, anchor);
    }
    (*((*emitter).anchors).offset((index - 1 as i32) as isize)).serialized = 1 as i32;
    match (*node).type_0 as u32 {
        1 => return yaml_emitter_dump_scalar(emitter, node, anchor),
        2 => return yaml_emitter_dump_sequence(emitter, node, anchor),
        3 => return yaml_emitter_dump_mapping(emitter, node, anchor),
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const i8,
                b"dumper.c\0" as *const u8 as *const i8,
                289 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"int yaml_emitter_dump_node(yaml_emitter_t *, int)\0"))
                    .as_ptr(),
            );
            'c_3788: {
                __assert_fail(
                    b"0\0" as *const u8 as *const i8,
                    b"dumper.c\0" as *const u8 as *const i8,
                    289 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"int yaml_emitter_dump_node(yaml_emitter_t *, int)\0"))
                        .as_ptr(),
                );
            };
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn yaml_emitter_dump_alias(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_ALIAS_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.alias.anchor = anchor;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    let mut plain_implicit: i32 = (strcmp(
        (*node).tag as *mut i8,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const i8,
    ) == 0 as i32) as i32;
    let mut quoted_implicit: i32 = (strcmp(
        (*node).tag as *mut i8,
        b"tag:yaml.org,2002:str\0" as *const u8 as *const i8,
    ) == 0 as i32) as i32;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_SCALAR_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.scalar.anchor = anchor;
    event.data.scalar.tag = (*node).tag;
    event.data.scalar.value = (*node).data.scalar.value;
    event.data.scalar.length = (*node).data.scalar.length;
    event.data.scalar.plain_implicit = plain_implicit;
    event.data.scalar.quoted_implicit = quoted_implicit;
    event.data.scalar.style = (*node).data.scalar.style;
    return yaml_emitter_emit(emitter, &mut event);
}
unsafe extern "C" fn yaml_emitter_dump_sequence(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    let mut implicit: i32 = (strcmp(
        (*node).tag as *mut i8,
        b"tag:yaml.org,2002:seq\0" as *const u8 as *const i8,
    ) == 0 as i32) as i32;
    let mut item: *mut yaml_node_item_t = 0 as *mut yaml_node_item_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_SEQUENCE_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.sequence_start.anchor = anchor;
    event.data.sequence_start.tag = (*node).tag;
    event.data.sequence_start.implicit = implicit;
    event.data.sequence_start.style = (*node).data.sequence.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 {
            return 0 as i32;
        }
        item = item.offset(1);
        item;
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_SEQUENCE_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn yaml_emitter_dump_mapping(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> i32 {
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
    let mut mark: yaml_mark_t = {
        let mut init = yaml_mark_s {
            index: 0 as i32 as size_t,
            line: 0 as i32 as size_t,
            column: 0 as i32 as size_t,
        };
        init
    };
    let mut implicit: i32 = (strcmp(
        (*node).tag as *mut i8,
        b"tag:yaml.org,2002:map\0" as *const u8 as *const i8,
    ) == 0 as i32) as i32;
    let mut pair: *mut yaml_node_pair_t = 0 as *mut yaml_node_pair_t;
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_MAPPING_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.mapping_start.anchor = anchor;
    event.data.mapping_start.tag = (*node).tag;
    event.data.mapping_start.implicit = implicit;
    event.data.mapping_start.style = (*node).data.mapping.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 {
            return 0 as i32;
        }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 {
            return 0 as i32;
        }
        pair = pair.offset(1);
        pair;
    }
    memset(
        &mut event as *mut yaml_event_t as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<yaml_event_t>() as u64,
    );
    event.type_0 = YAML_MAPPING_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0 as i32;
    }
    return 1 as i32;
}