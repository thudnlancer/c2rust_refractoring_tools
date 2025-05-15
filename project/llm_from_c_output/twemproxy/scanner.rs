use std::mem;
use std::ptr;
use std::os::raw::{c_int, c_char};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::collections::VecDeque;
use std::str;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct yaml_mark_t {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_token_t {
    pub type_: yaml_token_type_t,
    pub data: yaml_token_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Debug)]
pub enum yaml_token_type_t {
    YAML_NO_TOKEN = 0,
    YAML_STREAM_START_TOKEN,
    YAML_STREAM_END_TOKEN,
    YAML_VERSION_DIRECTIVE_TOKEN,
    YAML_TAG_DIRECTIVE_TOKEN,
    YAML_DOCUMENT_START_TOKEN,
    YAML_DOCUMENT_END_TOKEN,
    YAML_BLOCK_SEQUENCE_START_TOKEN,
    YAML_BLOCK_MAPPING_START_TOKEN,
    YAML_BLOCK_END_TOKEN,
    YAML_FLOW_SEQUENCE_START_TOKEN,
    YAML_FLOW_MAPPING_START_TOKEN,
    YAML_FLOW_SEQUENCE_END_TOKEN,
    YAML_FLOW_MAPPING_END_TOKEN,
    YAML_BLOCK_ENTRY_TOKEN,
    YAML_FLOW_ENTRY_TOKEN,
    YAML_KEY_TOKEN,
    YAML_VALUE_TOKEN,
    YAML_ALIAS_TOKEN,
    YAML_ANCHOR_TOKEN,
    YAML_TAG_TOKEN,
    YAML_SCALAR_TOKEN,
}

#[repr(C)]
#[derive(Debug)]
pub union yaml_token_data_t {
    pub stream_start: yaml_stream_start_data_t,
    pub alias: yaml_alias_data_t,
    pub anchor: yaml_anchor_data_t,
    pub tag: yaml_tag_data_t,
    pub scalar: yaml_scalar_data_t,
    pub version_directive: yaml_version_directive_data_t,
    pub tag_directive: yaml_tag_directive_data_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_stream_start_data_t {
    pub encoding: yaml_encoding_t,
}

#[repr(C)]
#[derive(Debug)]
pub enum yaml_encoding_t {
    YAML_ANY_ENCODING,
    YAML_UTF8_ENCODING,
    YAML_UTF16LE_ENCODING,
    YAML_UTF16BE_ENCODING,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_alias_data_t {
    pub value: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_anchor_data_t {
    pub value: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_tag_data_t {
    pub handle: *mut c_char,
    pub suffix: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_scalar_data_t {
    pub value: *mut c_char,
    pub length: usize,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Debug)]
pub enum yaml_scalar_style_t {
    YAML_ANY_SCALAR_STYLE,
    YAML_PLAIN_SCALAR_STYLE,
    YAML_SINGLE_QUOTED_SCALAR_STYLE,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE,
    YAML_LITERAL_SCALAR_STYLE,
    YAML_FOLDED_SCALAR_STYLE,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_version_directive_data_t {
    pub major: c_int,
    pub minor: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_tag_directive_data_t {
    pub handle: *mut c_char,
    pub prefix: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_parser_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub problem_offset: usize,
    pub problem_value: c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const c_char,
    pub context_mark: yaml_mark_t,
    pub buffer: yaml_buffer_t,
    pub raw_buffer: yaml_buffer_t,
    pub encoding: yaml_encoding_t,
    pub offset: usize,
    pub mark: yaml_mark_t,
    pub stream_start_produced: c_int,
    pub stream_end_produced: c_int,
    pub tokens: VecDeque<yaml_token_t>,
    pub tokens_parsed: usize,
    pub token_available: c_int,
    pub simple_keys: Vec<yaml_simple_key_t>,
    pub simple_key_allowed: c_int,
    pub flow_level: c_int,
    pub indent: isize,
    pub indents: Vec<isize>,
    pub document_start_produced: c_int,
    pub document_end_produced: c_int,
    pub last_was_ws: c_int,
}

#[repr(C)]
#[derive(Debug)]
pub enum yaml_error_type_t {
    YAML_NO_ERROR,
    YAML_MEMORY_ERROR,
    YAML_READER_ERROR,
    YAML_SCANNER_ERROR,
    YAML_PARSER_ERROR,
    YAML_COMPOSER_ERROR,
    YAML_WRITER_ERROR,
    YAML_EMITTER_ERROR,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_buffer_t {
    pub start: *mut c_char,
    pub pointer: *mut c_char,
    pub end: *mut c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_simple_key_t {
    pub possible: c_int,
    pub required: c_int,
    pub token_number: usize,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Debug)]
pub struct yaml_string_t {
    pub start: *mut c_char,
    pub pointer: *mut c_char,
    pub end: *mut c_char,
}

const INITIAL_STRING_SIZE: usize = 16;
const MAX_NUMBER_LENGTH: usize = 9;

impl yaml_parser_t {
    pub fn scan(&mut self, token: &mut yaml_token_t) -> c_int {
        unsafe {
            ptr::write_bytes(token, 0, 1);

            if self.stream_end_produced != 0 || self.error != yaml_error_type_t::YAML_NO_ERROR {
                return 1;
            }

            if self.token_available == 0 {
                if self.fetch_more_tokens() == 0 {
                    return 0;
                }
            }

            if let Some(t) = self.tokens.pop_front() {
                *token = t;
                self.token_available = 0;
                self.tokens_parsed += 1;

                if token.type_ == yaml_token_type_t::YAML_STREAM_END_TOKEN {
                    self.stream_end_produced = 1;
                }
                1
            } else {
                0
            }
        }
    }

    fn fetch_more_tokens(&mut self) -> c_int {
        loop {
            let need_more_tokens = if self.tokens.is_empty() {
                1
            } else {
                let mut need = 0;
                for sk in &self.simple_keys {
                    if sk.possible != 0 && sk.token_number == self.tokens_parsed {
                        need = 1;
                        break;
                    }
                }
                need
            };

            if need_more_tokens == 0 {
                break;
            }

            if self.fetch_next_token() == 0 {
                return 0;
            }
        }

        self.token_available = 1;
        1
    }

    fn fetch_next_token(&mut self) -> c_int {
        if self.cache(1) == 0 {
            return 0;
        }

        if self.stream_start_produced == 0 {
            return self.fetch_stream_start();
        }

        if self.scan_to_next_token() == 0 {
            return 0;
        }

        if self.stale_simple_keys() == 0 {
            return 0;
        }

        if self.unroll_indent(self.mark.column as isize) == 0 {
            return 0;
        }

        if self.cache(4) == 0 {
            return 0;
        }

        if self.buffer_is_empty() {
            return self.fetch_stream_end();
        }

        if self.mark.column == 0 && self.check('%') {
            return self.fetch_directive();
        }

        if self.mark.column == 0
            && self.check_at('-', 0)
            && self.check_at('-', 1)
            && self.check_at('-', 2)
            && self.is_blankz_at(3)
        {
            return self.fetch_document_indicator(yaml_token_type_t::YAML_DOCUMENT_START_TOKEN);
        }

        if self.mark.column == 0
            && self.check_at('.', 0)
            && self.check_at('.', 1)
            && self.check_at('.', 2)
            && self.is_blankz_at(3)
        {
            return self.fetch_document_indicator(yaml_token_type_t::YAML_DOCUMENT_END_TOKEN);
        }

        if self.check('[') {
            return self.fetch_flow_collection_start(yaml_token_type_t::YAML_FLOW_SEQUENCE_START_TOKEN);
        }

        if self.check('{') {
            return self.fetch_flow_collection_start(yaml_token_type_t::YAML_FLOW_MAPPING_START_TOKEN);
        }

        if self.check(']') {
            return self.fetch_flow_collection_end(yaml_token_type_t::YAML_FLOW_SEQUENCE_END_TOKEN);
        }

        if self.check('}') {
            return self.fetch_flow_collection_end(yaml_token_type_t::YAML_FLOW_MAPPING_END_TOKEN);
        }

        if self.check(',') {
            return self.fetch_flow_entry();
        }

        if self.check('-') && self.is_blankz_at(1) {
            return self.fetch_block_entry();
        }

        if self.check('?') && (self.flow_level != 0 || self.is_blankz_at(1)) {
            return self.fetch_key();
        }

        if self.check(':') && (self.flow_level != 0 || self.is_blankz_at(1)) {
            return self.fetch_value();
        }

        if self.check('*') {
            return self.fetch_anchor(yaml_token_type_t::YAML_ALIAS_TOKEN);
        }

        if self.check('&') {
            return self.fetch_anchor(yaml_token_type_t::YAML_ANCHOR_TOKEN);
        }

        if self.check('!') {
            return self.fetch_tag();
        }

        if self.check('|') && self.flow_level == 0 {
            return self.fetch_block_scalar(1);
        }

        if self.check('>') && self.flow_level == 0 {
            return self.fetch_block_scalar(0);
        }

        if self.check('\'') {
            return self.fetch_flow_scalar(1);
        }

        if self.check('"') {
            return self.fetch_flow_scalar(0);
        }

        if !(self.is_blankz()
            || self.check('-')
            || self.check('?')
            || self.check(':')
            || self.check(',')
            || self.check('[')
            || self.check(']')
            || self.check('{')
            || self.check('}')
            || self.check('#')
            || self.check('&')
            || self.check('*')
            || self.check('!')
            || self.check('|')
            || self.check('>')
            || self.check('\'')
            || self.check('"')
            || self.check('%')
            || self.check('@')
            || self.check('`'))
            || (self.check('-') && !self.is_blank_at(1))
            || (self.flow_level == 0
                && (self.check('?') || self.check(':'))
                && !self.is_blankz_at(1))
        {
            return self.fetch_plain_scalar();
        }

        self.set_scanner_error(
            "while scanning for the next token",
            self.mark,
            "found character that cannot start any token",
        )
    }

    fn set_scanner_error(&mut self, context: &str, context_mark: yaml_mark_t, problem: &str) -> c_int {
        self.error = yaml_error_type_t::YAML_SCANNER_ERROR;
        self.context = CString::new(context).unwrap().into_raw();
        self.context_mark = context_mark;
        self.problem = CString::new(problem).unwrap().into_raw();
        self.problem_mark = self.mark;
        0
    }

    fn cache(&mut self, length: usize) -> c_int {
        if self.buffer_unread() >= length {
            1
        } else {
            self.update_buffer(length)
        }
    }

    fn update_buffer(&mut self, length: usize) -> c_int {
        // Implementation depends on yaml_parser_update_buffer
        0
    }

    fn buffer_unread(&self) -> usize {
        (self.buffer.end as usize - self.buffer.pointer as usize) / mem::size_of::<c_char>()
    }

    fn check(&self, ch: c_char) -> bool {
        unsafe { *self.buffer.pointer == ch }
    }

    fn check_at(&self, ch: c_char, offset: usize) -> bool {
        unsafe { *self.buffer.pointer.offset(offset as isize) == ch }
    }

    fn is_blankz(&self) -> bool {
        self.is_blank() || self.is_breakz()
    }

    fn is_blankz_at(&self, offset: usize) -> bool {
        self.is_blank_at(offset) || self.is_breakz_at(offset)
    }

    fn is_blank(&self) -> bool {
        self.check(' ') || self.check('\t')
    }

    fn is_blank_at(&self, offset: usize) -> bool {
        self.check_at(' ', offset) || self.check_at('\t', offset)
    }

    fn is_breakz(&self) -> bool {
        self.is_break() || self.buffer_is_empty()
    }

    fn is_breakz_at(&self, offset: usize) -> bool {
        self.is_break_at(offset) || self.buffer_is_empty_at(offset)
    }

    fn is_break(&self) -> bool {
        self.check('\r') || self.check('\n')
    }

    fn is_break_at(&self, offset: usize) -> bool {
        self.check_at('\r', offset) || self.check_at('\n', offset)
    }

    fn buffer_is_empty(&self) -> bool {
        self.buffer.pointer == self.buffer.end
    }

    fn buffer_is_empty_at(&self, offset: usize) -> bool {
        unsafe { self.buffer.pointer.offset(offset as isize) == self.buffer.end }
    }

    fn skip(&mut self) {
        self.mark.index += 1;
        self.mark.column += 1;
        unsafe {
            self.buffer.pointer = self.buffer.pointer.offset(1);
        }
    }

    fn skip_line(&mut self) {
        if self.is_crlf() {
            self.mark.index += 2;
            self.mark.column = 0;
            self.mark.line += 1;
            unsafe {
                self.buffer.pointer = self.buffer.pointer.offset(2);
            }
        } else if self.is_break() {
            self.mark.index += 1;
            self.mark.column = 0;
            self.mark.line += 1;
            unsafe {
                self.buffer.pointer = self.buffer.pointer.offset(1);
            }
        }
    }

    fn is_crlf(&self) -> bool {
        self.check('\r') && self.check_at('\n', 1)
    }

    fn read(&mut self, string: &mut yaml_string_t) -> c_int {
        if self.string_extend(string) != 0 {
            unsafe {
                *string.pointer = *self.buffer.pointer;
                string.pointer = string.pointer.offset(1);
                self.mark.index += 1;
                self.mark.column += 1;
                self.buffer.pointer = self.buffer.pointer.offset(1);
            }
            1
        } else {
            0
        }
    }

    fn read_line(&mut self, string: &mut yaml_string_t) -> c_int {
        if self.string_extend(string) != 0 {
            unsafe {
                if self.check_at('\r', 0) && self.check_at('\n', 1) {
                    *string.pointer = '\n' as c_char;
                    string.pointer = string.pointer.offset(1);
                    self.buffer.pointer = self.buffer.pointer.offset(2);
                    self.mark.index += 2;
                    self.mark.column = 0;
                    self.mark.line += 1;
                } else if self.check_at('\r', 0) || self.check_at('\n', 0) {
                    *string.pointer = '\n' as c_char;
                    string.pointer = string.pointer.offset(1);
                    self.buffer.pointer = self.buffer.pointer.offset(1);
                    self.mark.index += 1;
                    self.mark.column = 0;
                    self.mark.line += 1;
                } else if self.check_at(0xC2, 0) && self.check_at(0x85, 1) {
                    *string.pointer = '\n' as c_char;
                    string.pointer = string.pointer.offset(1);
                    self.buffer.pointer = self.buffer.pointer.offset(2);
                    self.mark.index += 1;
                    self.mark.column = 0;
                    self.mark.line += 1;
                } else if self.check_at(0xE2, 0)
                    && self.check_at(0x80, 1)
                    && (self.check_at(0xA8, 2) || self.check_at(0xA9, 2))
                {
                    *string.pointer = *self.buffer.pointer;
                    string.pointer = string