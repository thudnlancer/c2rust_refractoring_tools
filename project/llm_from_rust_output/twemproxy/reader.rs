use std::cmp::Ordering;
use std::mem;
use std::ptr;
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum YamlEncoding {
    Any = 0,
    Utf8 = 1,
    Utf16Le = 2,
    Utf16Be = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum YamlErrorType {
    NoError = 0,
    MemoryError = 1,
    ReaderError = 2,
    ScannerError = 3,
    ParserError = 4,
    ComposerError = 5,
    WriterError = 6,
    EmitterError = 7,
}

#[derive(Debug, Clone, Copy)]
pub struct YamlMark {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

type YamlChar = u8;

#[derive(Debug, Clone)]
pub struct YamlParser {
    error: YamlErrorType,
    problem: String,
    problem_offset: usize,
    problem_value: i32,
    problem_mark: YamlMark,
    context: String,
    context_mark: YamlMark,
    read_handler: Option<Box<dyn FnMut(&mut [YamlChar]) -> Result<usize, ()>>>,
    eof: bool,
    raw_buffer: Vec<YamlChar>,
    buffer: Vec<YamlChar>,
    unread: usize,
    encoding: YamlEncoding,
    offset: usize,
    mark: YamlMark,
    stream_start_produced: bool,
    stream_end_produced: bool,
}

impl YamlParser {
    fn set_reader_error(&mut self, problem: &str, offset: usize, value: i32) -> Result<(), ()> {
        self.error = YamlErrorType::ReaderError;
        self.problem = problem.to_string();
        self.problem_offset = offset;
        self.problem_value = value;
        Err(())
    }

    fn determine_encoding(&mut self) -> Result<(), ()> {
        while !self.eof && self.raw_buffer.len() < 3 {
            self.update_raw_buffer()?;
        }

        if self.raw_buffer.len() >= 2 {
            if self.raw_buffer.starts_with(&[0xFF, 0xFE]) {
                self.encoding = YamlEncoding::Utf16Le;
                self.raw_buffer.drain(0..2);
                self.offset += 2;
            } else if self.raw_buffer.starts_with(&[0xFE, 0xFF]) {
                self.encoding = YamlEncoding::Utf16Be;
                self.raw_buffer.drain(0..2);
                self.offset += 2;
            } else if self.raw_buffer.len() >= 3 && self.raw_buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
                self.encoding = YamlEncoding::Utf8;
                self.raw_buffer.drain(0..3);
                self.offset += 3;
            } else {
                self.encoding = YamlEncoding::Utf8;
            }
        } else {
            self.encoding = YamlEncoding::Utf8;
        }

        Ok(())
    }

    fn update_raw_buffer(&mut self) -> Result<(), ()> {
        if self.raw_buffer.is_empty() && self.eof {
            return Ok(());
        }

        if self.eof {
            return Ok(());
        }

        if let Some(handler) = &mut self.read_handler {
            let mut buf = vec![0; 4096];
            match handler(&mut buf) {
                Ok(size) => {
                    if size == 0 {
                        self.eof = true;
                    } else {
                        self.raw_buffer.extend_from_slice(&buf[..size]);
                    }
                    Ok(())
                }
                Err(_) => self.set_reader_error("input error", self.offset, -1),
            }
        } else {
            self.set_reader_error("no read handler", self.offset, -1)
        }
    }

    pub fn update_buffer(&mut self, length: usize) -> Result<(), ()> {
        if self.eof && self.raw_buffer.is_empty() {
            return Ok(());
        }

        if self.unread >= length {
            return Ok(());
        }

        if self.encoding == YamlEncoding::Any {
            self.determine_encoding()?;
        }

        // Handle buffer shifting
        if !self.buffer.is_empty() {
            let remaining = self.buffer.len() - self.unread;
            if remaining > 0 {
                self.buffer.copy_within(self.unread.., 0);
                self.buffer.truncate(remaining);
            } else {
                self.buffer.clear();
            }
        }

        while self.unread < length {
            if self.raw_buffer.is_empty() {
                self.update_raw_buffer()?;
            }

            let mut processed = 0;
            while processed < self.raw_buffer.len() {
                let (value, width, incomplete) = match self.encoding {
                    YamlEncoding::Utf8 => self.process_utf8()?,
                    YamlEncoding::Utf16Le | YamlEncoding::Utf16Be => self.process_utf16()?,
                    _ => return self.set_reader_error("unsupported encoding", self.offset, -1),
                };

                if incomplete {
                    break;
                }

                processed += width;
                self.encode_utf8(value)?;
                self.unread += 1;
            }

            self.raw_buffer.drain(0..processed);

            if self.eof {
                self.buffer.push(0);
                self.unread += 1;
                return Ok(());
            }
        }

        if self.offset >= usize::MAX / 2 {
            return self.set_reader_error("input is too long", self.offset, -1);
        }

        Ok(())
    }

    fn process_utf8(&mut self) -> Result<(u32, usize, bool), ()> {
        if self.raw_buffer.is_empty() {
            return Ok((0, 0, true));
        }

        let octet = self.raw_buffer[0];
        let width = match octet {
            _ if octet & 0x80 == 0 => 1,
            _ if octet & 0xE0 == 0xC0 => 2,
            _ if octet & 0xF0 == 0xE0 => 3,
            _ if octet & 0xF8 == 0xF0 => 4,
            _ => return self.set_reader_error("invalid leading UTF-8 octet", self.offset, octet as i32),
        };

        if self.raw_buffer.len() < width {
            if self.eof {
                return self.set_reader_error("incomplete UTF-8 octet sequence", self.offset, -1);
            }
            return Ok((0, 0, true));
        }

        let mut value = match width {
            1 => (octet & 0x7F) as u32,
            2 => (octet & 0x1F) as u32,
            3 => (octet & 0x0F) as u32,
            4 => (octet & 0x07) as u32,
            _ => unreachable!(),
        };

        for i in 1..width {
            let octet = self.raw_buffer[i];
            if octet & 0xC0 != 0x80 {
                return self.set_reader_error(
                    "invalid trailing UTF-8 octet",
                    self.offset + i,
                    octet as i32,
                );
            }
            value = (value << 6) | (octet & 0x3F) as u32;
        }

        // Validate the value
        if !(width == 1 && value <= 0x7F
            || width == 2 && value >= 0x80
            || width == 3 && value >= 0x800
            || width == 4 && value >= 0x10000)
        {
            return self.set_reader_error(
                "invalid length of a UTF-8 sequence",
                self.offset,
                -1,
            );
        }

        if (0xD800..=0xDFFF).contains(&value) || value > 0x10FFFF {
            return self.set_reader_error(
                "invalid Unicode character",
                self.offset,
                value as i32,
            );
        }

        Ok((value, width, false))
    }

    fn process_utf16(&mut self) -> Result<(u32, usize, bool), ()> {
        if self.raw_buffer.len() < 2 {
            if self.eof {
                return self.set_reader_error("incomplete UTF-16 character", self.offset, -1);
            }
            return Ok((0, 0, true));
        }

        let (low, high) = match self.encoding {
            YamlEncoding::Utf16Le => (0, 1),
            YamlEncoding::Utf16Be => (1, 0),
            _ => unreachable!(),
        };

        let value = (self.raw_buffer[low] as u32) | ((self.raw_buffer[high] as u32) << 8;

        if (0xDC00..=0xDFFF).contains(&value) {
            return self.set_reader_error(
                "unexpected low surrogate area",
                self.offset,
                value as i32,
            );
        }

        if (0xD800..=0xDBFF).contains(&value) {
            if self.raw_buffer.len() < 4 {
                if self.eof {
                    return self.set_reader_error(
                        "incomplete UTF-16 surrogate pair",
                        self.offset,
                        -1,
                    );
                }
                return Ok((0, 0, true));
            }

            let value2 = (self.raw_buffer[low + 2] as u32) | ((self.raw_buffer[high + 2] as u32) << 8);
            if !(0xDC00..=0xDFFF).contains(&value2) {
                return self.set_reader_error(
                    "expected low surrogate area",
                    self.offset + 2,
                    value2 as i32,
                );
            }

            let value = 0x10000 + ((value & 0x3FF) << 10) + (value2 & 0x3FF);
            Ok((value, 4, false))
        } else {
            Ok((value, 2, false))
        }
    }

    fn encode_utf8(&mut self, value: u32) -> Result<(), ()> {
        if !(value == 0x9
            || value == 0xA
            || value == 0xD
            || (0x20..=0x7E).contains(&value)
            || value == 0x85
            || (0xA0..=0xD7FF).contains(&value)
            || (0xE000..=0xFFFD).contains(&value)
            || (0x10000..=0x10FFFF).contains(&value))
        {
            return self.set_reader_error(
                "control characters are not allowed",
                self.offset,
                value as i32,
            );
        }

        match value {
            0..=0x7F => {
                self.buffer.push(value as u8);
            }
            0x80..=0x7FF => {
                self.buffer.push(0xC0 | ((value >> 6) as u8));
                self.buffer.push(0x80 | ((value & 0x3F) as u8));
            }
            0x800..=0xFFFF => {
                self.buffer.push(0xE0 | ((value >> 12) as u8));
                self.buffer.push(0x80 | (((value >> 6) & 0x3F) as u8));
                self.buffer.push(0x80 | ((value & 0x3F) as u8));
            }
            0x10000..=0x10FFFF => {
                self.buffer.push(0xF0 | ((value >> 18) as u8));
                self.buffer.push(0x80 | (((value >> 12) & 0x3F) as u8));
                self.buffer.push(0x80 | (((value >> 6) & 0x3F) as u8));
                self.buffer.push(0x80 | ((value & 0x3F) as u8));
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}