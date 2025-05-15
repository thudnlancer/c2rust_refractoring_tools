use std::ffi::CString;
use std::ptr;
use std::process;
use libc::{c_int, c_char, c_void, c_uchar, c_long, c_ulong, c_ushort, c_schar};
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_version_directive_t {
    pub major: c_int,
    pub minor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_pair_t {
    pub key: c_int,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_simple_key_t {
    pub possible: c_int,
    pub required: c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_alias_data_t {
    pub anchor: *mut yaml_char_t,
    pub index: c_int,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_anchors_t {
    pub references: c_int,
    pub anchor: c_int,
    pub serialized: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_scalar_data_t {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: c_int,
    pub flow_plain_allowed: c_int,
    pub block_plain_allowed: c_int,
    pub single_quoted_allowed: c_int,
    pub block_allowed: c_int,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_tag_data_t {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_anchor_data_t {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_document_t {
    pub nodes: yaml_node_stack_t,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: yaml_tag_directives_t,
    pub start_implicit: c_int,
    pub end_implicit: c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_parser_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub problem_offset: size_t,
    pub problem_value: c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut c_void,
    pub input: yaml_input_t,
    pub eof: c_int,
    pub buffer: yaml_buffer_t,
    pub unread: size_t,
    pub raw_buffer: yaml_raw_buffer_t,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: c_int,
    pub stream_end_produced: c_int,
    pub flow_level: c_int,
    pub tokens: yaml_token_queue_t,
    pub tokens_parsed: size_t,
    pub token_available: c_int,
    pub indents: yaml_indent_stack_t,
    pub indent: c_int,
    pub simple_key_allowed: c_int,
    pub simple_keys: yaml_simple_key_stack_t,
    pub states: yaml_parser_state_stack_t,
    pub state: yaml_parser_state_t,
    pub marks: yaml_mark_stack_t,
    pub tag_directives: yaml_tag_directives_stack_t,
    pub aliases: yaml_alias_data_stack_t,
    pub document: *mut yaml_document_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut c_void,
    pub output: yaml_output_t,
    pub buffer: yaml_buffer_t,
    pub raw_buffer: yaml_raw_buffer_t,
    pub encoding: yaml_encoding_t,
    pub canonical: c_int,
    pub best_indent: c_int,
    pub best_width: c_int,
    pub unicode: c_int,
    pub line_break: yaml_break_t,
    pub states: yaml_emitter_state_stack_t,
    pub state: yaml_emitter_state_t,
    pub events: yaml_event_queue_t,
    pub indents: yaml_indent_stack_t,
    pub tag_directives: yaml_tag_directives_stack_t,
    pub indent: c_int,
    pub flow_level: c_int,
    pub root_context: c_int,
    pub sequence_context: c_int,
    pub mapping_context: c_int,
    pub simple_key_context: c_int,
    pub line: c_int,
    pub column: c_int,
    pub whitespace: c_int,
    pub indention: c_int,
    pub open_ended: c_int,
    pub anchor_data: yaml_anchor_data_t,
    pub tag_data: yaml_tag_data_t,
    pub scalar_data: yaml_scalar_data_t,
    pub opened: c_int,
    pub closed: c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: c_int,
    pub document: *mut yaml_document_t,
}

pub type size_t = c_ulong;
pub type yaml_char_t = c_uchar;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_encoding_t {
    YAML_ANY_ENCODING = 0,
    YAML_UTF8_ENCODING = 1,
    YAML_UTF16LE_ENCODING = 2,
    YAML_UTF16BE_ENCODING = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_break_t {
    YAML_ANY_BREAK = 0,
    YAML_CR_BREAK = 1,
    YAML_LN_BREAK = 2,
    YAML_CRLN_BREAK = 3,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_error_type_t {
    YAML_NO_ERROR = 0,
    YAML_MEMORY_ERROR = 1,
    YAML_READER_ERROR = 2,
    YAML_SCANNER_ERROR = 3,
    YAML_PARSER_ERROR = 4,
    YAML_COMPOSER_ERROR = 5,
    YAML_WRITER_ERROR = 6,
    YAML_EMITTER_ERROR = 7,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_scalar_style_t {
    YAML_ANY_SCALAR_STYLE = 0,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_FOLDED_SCALAR_STYLE = 5,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_sequence_style_t {
    YAML_ANY_SEQUENCE_STYLE = 0,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_FLOW_SEQUENCE_STYLE = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_mapping_style_t {
    YAML_ANY_MAPPING_STYLE = 0,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_FLOW_MAPPING_STYLE = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_parser_state_t {
    YAML_PARSE_STREAM_START_STATE = 0,
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE = 1,
    YAML_PARSE_DOCUMENT_START_STATE = 2,
    YAML_PARSE_DOCUMENT_CONTENT_STATE = 3,
    YAML_PARSE_DOCUMENT_END_STATE = 4,
    YAML_PARSE_BLOCK_NODE_STATE = 5,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE = 6,
    YAML_PARSE_FLOW_NODE_STATE = 7,
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE = 8,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE = 9,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE = 10,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE = 11,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE = 12,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE = 13,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE = 14,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE = 15,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE = 16,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE = 17,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE = 18,
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE = 19,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE = 20,
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE = 21,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE = 22,
    YAML_PARSE_END_STATE = 23,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum yaml_emitter_state_t {
    YAML_EMIT_STREAM_START_STATE = 0,
    YAML_EMIT_FIRST_DOCUMENT_START_STATE = 1,
    YAML_EMIT_DOCUMENT_START_STATE = 2,
    YAML_EMIT_DOCUMENT_CONTENT_STATE = 3,
    YAML_EMIT_DOCUMENT_END_STATE = 4,
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE = 5,
    YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE = 6,
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE = 7,
    YAML_EMIT_FLOW_MAPPING_KEY_STATE = 8,
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE = 9,
    YAML_EMIT_FLOW_MAPPING_VALUE_STATE = 10,
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE = 11,
    YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE = 12,
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE = 13,
    YAML_EMIT_BLOCK_MAPPING_KEY_STATE = 14,
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE = 15,
    YAML_EMIT_BLOCK_MAPPING_VALUE_STATE = 16,
    YAML_EMIT_END_STATE = 17,
}

pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut c_void,
    *mut c_uchar,
    size_t,
    *mut size_t,
) -> c_int;

pub type yaml_write_handler_t = unsafe extern "C" fn(
    *mut c_void,
    *mut c_uchar,
    size_t,
) -> c_int;

extern "C" {
    fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int;
    fn yaml_parser_delete(parser: *mut yaml_parser_t);
    fn yaml_parser_set_input_file(parser: *mut yaml_parser_t, file: *mut FILE);
    fn yaml_parser_load(parser: *mut yaml_parser_t, document: *mut yaml_document_t) -> c_int;
    fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> c_int;
    fn yaml_emitter_delete(emitter: *mut yaml_emitter_t);
    fn yaml_emitter_set_output_file(emitter: *mut yaml_emitter_t, file: *mut FILE);
    fn yaml_emitter_set_canonical(emitter: *mut yaml_emitter_t, canonical: c_int);
    fn yaml_emitter_set_unicode(emitter: *mut yaml_emitter_t, unicode: c_int);
    fn yaml_emitter_dump(emitter: *mut yaml_emitter_t, document: *mut yaml_document_t) -> c_int;
    fn yaml_document_get_root_node(document: *mut yaml_document_t) -> *mut yaml_node_t;
}

fn main() {
    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();
    let mut argv: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
    argv.push(ptr::null());

    unsafe {
        process::exit(main_0(
            (argv.len() - 1) as c_int,
            argv.as_mut_ptr() as *mut *mut c_char,
        ));
    }
}

unsafe fn main_0(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut help = 0;
    let mut canonical = 0;
    let mut unicode = 0;
    let mut done = 0;
    
    let mut parser: yaml_parser_t = mem::zeroed();
    let mut emitter: yaml_emitter_t = mem::zeroed();
    let mut document: yaml_document_t = mem::zeroed();

    // Parse command line arguments
    for i in 1..argc {
        let arg = CStr::from_ptr(*argv.offset(i as isize)).to_str().unwrap();
        match arg {
            "-h" | "--help" => help = 1,
            "-c" | "--canonical" => canonical = 1,
            "-u" | "--unicode" => unicode = 1,
            _ => {
                eprintln!("Unrecognized option: {}\nTry `{} --help` for more information.", 
                    arg, CStr::from_ptr(*argv).to_str().unwrap());
                return 1;
            }
        }
    }

    if help != 0 {
        println!("Usage: {} [--canonical] [--unicode] <input >output\n\
                 or\n{} -h | --help\n\n\
                 Reformat a YAML stream\n\n\
                 Options:\n\
                 -h, --help\t\tdisplay this help and exit\n\
                 -c, --canonical\toutput in the canonical YAML format\n\
                 -u, --unicode\t\toutput unescaped non-ASCII characters",
                 CStr::from_ptr(*argv).to_str().unwrap(),
                 CStr::from_ptr(*argv).to_str().unwrap());
        return 0;
    }

    // Initialize parser and emitter
    if yaml_parser_initialize(&mut parser) == 0 || yaml_emitter_initialize(&mut emitter) == 0 {
        eprintln!("Failed to initialize YAML parser/emitter");
        return 1;
    }

    // Set up input/output
    yaml_parser_set_input_file(&mut parser, libc::stdin);
    yaml_emitter_set_output_file(&mut emitter, libc::stdout);
    yaml_emitter_set_canonical(&mut emitter, canonical);
    yaml_emitter_set_unicode(&mut emitter, unicode);

    // Process documents
    while done == 0 {
        if yaml_parser_load(&mut parser, &mut document) == 0 {
            break;
        }

        if yaml_document_get_root_node(&mut document).is_null() {
            done = 1;
        }

        if yaml_emitter_dump(&mut emitter, &mut document) == 0 {
            break;
        }
    }

    // Clean up
    yaml_parser_delete(&mut parser);
    yaml_emitter_delete(&mut emitter);

    // Check for errors
    if parser.error != yaml_error_type_t::YAML_NO_ERROR {
        print_parser_error(&parser);
        1
    } else if emitter.error != yaml_error_type_t::YAML_NO_ERROR {
        print_emitter_error(&emitter);
        1
    } else {
        0
    }
}

unsafe fn print_parser_error(parser: &yaml_parser_t) {
    match parser.error {
        yaml_error_type_t::YAML_MEMORY_ERROR => eprintln!("Memory error: Not enough memory for parsing"),
        yaml_error_type_t::YAML_READER_ERROR => {
            if parser.problem_value != -1 {
                eprintln!("Reader error: {}: #{:X} at {}", 
                    CStr::from_ptr(parser.problem).