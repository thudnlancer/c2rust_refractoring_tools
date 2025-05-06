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
unsafe extern "C" fn yaml_emitter_set_writer_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const i8,
) -> i32 {
    (*emitter).error = YAML_WRITER_ERROR;
    (*emitter).problem = problem;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(mut emitter: *mut yaml_emitter_t) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = 0;
    if !emitter.is_null() {} else {
        __assert_fail(
            b"emitter\0" as *const u8 as *const i8,
            b"writer.c\0" as *const u8 as *const i8,
            36 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3710: {
        if !emitter.is_null() {} else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const i8,
                b"writer.c\0" as *const u8 as *const i8,
                36 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if ((*emitter).write_handler).is_some() {} else {
        __assert_fail(
            b"emitter->write_handler\0" as *const u8 as *const i8,
            b"writer.c\0" as *const u8 as *const i8,
            37 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3673: {
        if ((*emitter).write_handler).is_some() {} else {
            __assert_fail(
                b"emitter->write_handler\0" as *const u8 as *const i8,
                b"writer.c\0" as *const u8 as *const i8,
                37 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*emitter).encoding as u64 != 0 {} else {
        __assert_fail(
            b"emitter->encoding\0" as *const u8 as *const i8,
            b"writer.c\0" as *const u8 as *const i8,
            38 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3625: {
        if (*emitter).encoding as u64 != 0 {} else {
            __assert_fail(
                b"emitter->encoding\0" as *const u8 as *const i8,
                b"writer.c\0" as *const u8 as *const i8,
                38 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"int yaml_emitter_flush(yaml_emitter_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*emitter).buffer.last = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1 as i32;
    }
    if (*emitter).encoding as u32 == YAML_UTF8_ENCODING as i32 as u32 {
        if ((*emitter).write_handler)
            .expect(
                "non-null function pointer",
            )(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            ((*emitter).buffer.last).offset_from((*emitter).buffer.start) as i64
                as size_t,
        ) != 0
        {
            (*emitter).buffer.last = (*emitter).buffer.start;
            (*emitter).buffer.pointer = (*emitter).buffer.start;
            return 1 as i32;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const i8,
            )
        }
    }
    low = if (*emitter).encoding as u32 == YAML_UTF16LE_ENCODING as i32 as u32 {
        0 as i32
    } else {
        1 as i32
    };
    high = if (*emitter).encoding as u32 == YAML_UTF16LE_ENCODING as i32 as u32 {
        1 as i32
    } else {
        0 as i32
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: u8 = 0;
        let mut width: u32 = 0;
        let mut value: u32 = 0;
        let mut k: size_t = 0;
        octet = *((*emitter).buffer.pointer).offset(0 as i32 as isize);
        width = (if octet as i32 & 0x80 as i32 == 0 as i32 {
            1 as i32
        } else if octet as i32 & 0xe0 as i32 == 0xc0 as i32 {
            2 as i32
        } else if octet as i32 & 0xf0 as i32 == 0xe0 as i32 {
            3 as i32
        } else if octet as i32 & 0xf8 as i32 == 0xf0 as i32 {
            4 as i32
        } else {
            0 as i32
        }) as u32;
        value = (if octet as i32 & 0x80 as i32 == 0 as i32 {
            octet as i32 & 0x7f as i32
        } else if octet as i32 & 0xe0 as i32 == 0xc0 as i32 {
            octet as i32 & 0x1f as i32
        } else if octet as i32 & 0xf0 as i32 == 0xe0 as i32 {
            octet as i32 & 0xf as i32
        } else if octet as i32 & 0xf8 as i32 == 0xf0 as i32 {
            octet as i32 & 0x7 as i32
        } else {
            0 as i32
        }) as u32;
        k = 1 as i32 as size_t;
        while k < width as u64 {
            octet = *((*emitter).buffer.pointer).offset(k as isize);
            value = (value << 6 as i32)
                .wrapping_add((octet as i32 & 0x3f as i32) as u32);
            k = k.wrapping_add(1);
            k;
        }
        (*emitter).buffer.pointer = ((*emitter).buffer.pointer).offset(width as isize);
        if value < 0x10000 as i32 as u32 {
            *((*emitter).raw_buffer.last).offset(high as isize) = (value >> 8 as i32)
                as u8;
            *((*emitter).raw_buffer.last).offset(low as isize) = (value
                & 0xff as i32 as u32) as u8;
            (*emitter).raw_buffer.last = ((*emitter).raw_buffer.last)
                .offset(2 as i32 as isize);
        } else {
            value = value.wrapping_sub(0x10000 as i32 as u32);
            *((*emitter).raw_buffer.last).offset(high as isize) = (0xd8 as i32 as u32)
                .wrapping_add(value >> 18 as i32) as u8;
            *((*emitter).raw_buffer.last).offset(low as isize) = (value >> 10 as i32
                & 0xff as i32 as u32) as u8;
            *((*emitter).raw_buffer.last).offset((high + 2 as i32) as isize) = (0xdc
                as i32 as u32)
                .wrapping_add(value >> 8 as i32 & 0xff as i32 as u32) as u8;
            *((*emitter).raw_buffer.last).offset((low + 2 as i32) as isize) = (value
                & 0xff as i32 as u32) as u8;
            (*emitter).raw_buffer.last = ((*emitter).raw_buffer.last)
                .offset(4 as i32 as isize);
        }
    }
    if ((*emitter).write_handler)
        .expect(
            "non-null function pointer",
        )(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        ((*emitter).raw_buffer.last).offset_from((*emitter).raw_buffer.start) as i64
            as size_t,
    ) != 0
    {
        (*emitter).buffer.last = (*emitter).buffer.start;
        (*emitter).buffer.pointer = (*emitter).buffer.start;
        (*emitter).raw_buffer.last = (*emitter).raw_buffer.start;
        (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.start;
        return 1 as i32;
    } else {
        return yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const i8,
        )
    };
}