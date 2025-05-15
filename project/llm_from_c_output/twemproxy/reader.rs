use std::mem;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
enum YamlEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

#[derive(Debug, Clone, Copy)]
enum YamlError {
    ReaderError {
        problem: &'static str,
        offset: usize,
        value: i32,
    },
}

struct YamlBuffer {
    start: usize,
    pointer: usize,
    last: usize,
    end: usize,
    data: Vec<u8>,
}

impl YamlBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            start: 0,
            pointer: 0,
            last: 0,
            end: capacity,
            data: vec![0; capacity],
        }
    }

    fn available(&self) -> usize {
        self.last - self.pointer
    }

    fn remaining(&self) -> usize {
        self.end - self.last
    }
}

struct YamlParser {
    encoding: Option<YamlEncoding>,
    error: Option<YamlError>,
    eof: bool,
    offset: usize,
    unread: usize,
    raw_buffer: YamlBuffer,
    buffer: YamlBuffer,
    read_handler: Box<dyn FnMut(&mut [u8]) -> Result<usize, &'static str>>,
}

impl YamlParser {
    fn new(
        read_handler: Box<dyn FnMut(&mut [u8]) -> Result<usize, &'static str>>,
        buffer_size: usize,
    ) -> Self {
        Self {
            encoding: None,
            error: None,
            eof: false,
            offset: 0,
            unread: 0,
            raw_buffer: YamlBuffer::new(buffer_size),
            buffer: YamlBuffer::new(buffer_size),
            read_handler,
        }
    }

    fn set_reader_error(
        &mut self,
        problem: &'static str,
        offset: usize,
        value: i32,
    ) -> Result<(), YamlError> {
        self.error = Some(YamlError::ReaderError {
            problem,
            offset,
            value,
        });
        Err(self.error.unwrap())
    }

    fn update_raw_buffer(&mut self) -> Result<(), YamlError> {
        if self.raw_buffer.start == self.raw_buffer.pointer
            && self.raw_buffer.last == self.raw_buffer.end
        {
            return Ok(());
        }

        if self.eof {
            return Ok(());
        }

        if self.raw_buffer.start < self.raw_buffer.pointer
            && self.raw_buffer.pointer < self.raw_buffer.last
        {
            let size = self.raw_buffer.last - self.raw_buffer.pointer;
            self.raw_buffer
                .data
                .copy_within(self.raw_buffer.pointer..self.raw_buffer.last, 0);
            self.raw_buffer.last = size;
            self.raw_buffer.pointer = 0;
        }

        let read_result = (self.read_handler)(
            &mut self.raw_buffer.data[self.raw_buffer.last..self.raw_buffer.end],
        );

        match read_result {
            Ok(size_read) => {
                self.raw_buffer.last += size_read;
                if size_read == 0 {
                    self.eof = true;
                }
                Ok(())
            }
            Err(problem) => self.set_reader_error(problem, self.offset, -1),
        }
    }

    fn determine_encoding(&mut self) -> Result<(), YamlError> {
        while !self.eof && self.raw_buffer.available() < 3 {
            self.update_raw_buffer()?;
        }

        if self.raw_buffer.available() >= 2 {
            let bom_utf16le = [0xFFu8, 0xFE];
            let bom_utf16be = [0xFEu8, 0xFF];
            let slice = &self.raw_buffer.data[self.raw_buffer.pointer..self.raw_buffer.pointer + 2];

            if slice == bom_utf16le {
                self.encoding = Some(YamlEncoding::Utf16Le);
                self.raw_buffer.pointer += 2;
                self.offset += 2;
                return Ok(());
            } else if slice == bom_utf16be {
                self.encoding = Some(YamlEncoding::Utf16Be);
                self.raw_buffer.pointer += 2;
                self.offset += 2;
                return Ok(());
            }
        }

        if self.raw_buffer.available() >= 3 {
            let bom_utf8 = [0xEFu8, 0xBB, 0xBF];
            let slice = &self.raw_buffer.data[self.raw_buffer.pointer..self.raw_buffer.pointer + 3];

            if slice == bom_utf8 {
                self.encoding = Some(YamlEncoding::Utf8);
                self.raw_buffer.pointer += 3;
                self.offset += 3;
                return Ok(());
            }
        }

        self.encoding = Some(YamlEncoding::Utf8);
        Ok(())
    }

    fn update_buffer(&mut self, length: usize) -> Result<(), YamlError> {
        if self.eof && self.raw_buffer.pointer == self.raw_buffer.last {
            return Ok(());
        }

        if self.unread >= length {
            return Ok(());
        }

        if self.encoding.is_none() {
            self.determine_encoding()?;
        }

        if self.buffer.start < self.buffer.pointer && self.buffer.pointer < self.buffer.last {
            let size = self.buffer.last - self.buffer.pointer;
            self.buffer
                .data
                .copy_within(self.buffer.pointer..self.buffer.last, 0);
            self.buffer.pointer = 0;
            self.buffer.last = size;
        } else if self.buffer.pointer == self.buffer.last {
            self.buffer.pointer = 0;
            self.buffer.last = 0;
        }

        let mut first = true;
        while self.unread < length {
            if !first || self.raw_buffer.pointer == self.raw_buffer.last {
                self.update_raw_buffer()?;
            }
            first = false;

            while self.raw_buffer.pointer != self.raw_buffer.last {
                let raw_unread = self.raw_buffer.last - self.raw_buffer.pointer;
                let (value, width) = match self.encoding.unwrap() {
                    YamlEncoding::Utf8 => self.decode_utf8(raw_unread)?,
                    YamlEncoding::Utf16Le | YamlEncoding::Utf16Be => {
                        self.decode_utf16(raw_unread)?
                    }
                };

                if value == 0x09
                    || value == 0x0A
                    || value == 0x0D
                    || (value >= 0x20 && value <= 0x7E)
                    || (value == 0x85)
                    || (value >= 0xA0 && value <= 0xD7FF)
                    || (value >= 0xE000 && value <= 0xFFFD)
                    || (value >= 0x10000 && value <= 0x10FFFF)
                {
                    self.raw_buffer.pointer += width;
                    self.offset += width;

                    if value <= 0x7F {
                        self.buffer.data[self.buffer.last] = value as u8;
                        self.buffer.last += 1;
                    } else if value <= 0x7FF {
                        self.buffer.data[self.buffer.last] = (0xC0 + (value >> 6)) as u8;
                        self.buffer.data[self.buffer.last + 1] = (0x80 + (value & 0x3F)) as u8;
                        self.buffer.last += 2;
                    } else if value <= 0xFFFF {
                        self.buffer.data[self.buffer.last] = (0xE0 + (value >> 12)) as u8;
                        self.buffer.data[self.buffer.last + 1] =
                            (0x80 + ((value >> 6) & 0x3F)) as u8;
                        self.buffer.data[self.buffer.last + 2] = (0x80 + (value & 0x3F)) as u8;
                        self.buffer.last += 3;
                    } else {
                        self.buffer.data[self.buffer.last] = (0xF0 + (value >> 18)) as u8;
                        self.buffer.data[self.buffer.last + 1] =
                            (0x80 + ((value >> 12) & 0x3F)) as u8;
                        self.buffer.data[self.buffer.last + 2] =
                            (0x80 + ((value >> 6) & 0x3F)) as u8;
                        self.buffer.data[self.buffer.last + 3] = (0x80 + (value & 0x3F)) as u8;
                        self.buffer.last += 4;
                    }

                    self.unread += 1;
                } else {
                    return self.set_reader_error(
                        "control characters are not allowed",
                        self.offset,
                        value as i32,
                    );
                }
            }

            if self.eof {
                self.buffer.data[self.buffer.last] = 0;
                self.buffer.last += 1;
                self.unread += 1;
                return Ok(());
            }
        }

        if self.offset >= usize::MAX {
            return self.set_reader_error("input is too long", self.offset, -1);
        }

        Ok(())
    }

    fn decode_utf8(&mut self, raw_unread: usize) -> Result<(u32, usize), YamlError> {
        let octet = self.raw_buffer.data[self.raw_buffer.pointer];
        let width = match octet {
            _ if (octet & 0x80) == 0x00 => 1,
            _ if (octet & 0xE0) == 0xC0 => 2,
            _ if (octet & 0xF0) == 0xE0 => 3,
            _ if (octet & 0xF8) == 0xF0 => 4,
            _ => {
                return self.set_reader_error(
                    "invalid leading UTF-8 octet",
                    self.offset,
                    octet as i32,
                );
            }
        };

        if width > raw_unread {
            if self.eof {
                return self.set_reader_error(
                    "incomplete UTF-8 octet sequence",
                    self.offset,
                    -1,
                );
            }
            return Ok((0, 0));
        }

        let mut value = match width {
            1 => (octet & 0x7F) as u32,
            2 => (octet & 0x1F) as u32,
            3 => (octet & 0x0F) as u32,
            4 => (octet & 0x07) as u32,
            _ => unreachable!(),
        };

        for k in 1..width {
            let octet = self.raw_buffer.data[self.raw_buffer.pointer + k];
            if (octet & 0xC0) != 0x80 {
                return self.set_reader_error(
                    "invalid trailing UTF-8 octet",
                    self.offset + k,
                    octet as i32,
                );
            }
            value = (value << 6) + (octet & 0x3F) as u32;
        }

        if !((width == 1)
            || (width == 2 && value >= 0x80)
            || (width == 3 && value >= 0x800)
            || (width == 4 && value >= 0x10000))
        {
            return self.set_reader_error(
                "invalid length of a UTF-8 sequence",
                self.offset,
                -1,
            );
        }

        if (value >= 0xD800 && value <= 0xDFFF) || value > 0x10FFFF {
            return self.set_reader_error("invalid Unicode character", self.offset, value as i32);
        }

        Ok((value, width))
    }

    fn decode_utf16(&mut self, raw_unread: usize) -> Result<(u32, usize), YamlError> {
        let (low, high) = match self.encoding.unwrap() {
            YamlEncoding::Utf16Le => (0, 1),
            YamlEncoding::Utf16Be => (1, 0),
            _ => unreachable!(),
        };

        if raw_unread < 2 {
            if self.eof {
                return self.set_reader_error(
                    "incomplete UTF-16 character",
                    self.offset,
                    -1,
                );
            }
            return Ok((0, 0));
        }

        let value = self.raw_buffer.data[self.raw_buffer.pointer + low] as u32
            + ((self.raw_buffer.data[self.raw_buffer.pointer + high] as u32) << 8);

        if (value & 0xFC00) == 0xDC00 {
            return self.set_reader_error(
                "unexpected low surrogate area",
                self.offset,
                value as i32,
            );
        }

        if (value & 0xFC00) == 0xD800 {
            if raw_unread < 4 {
                if self.eof {
                    return self.set_reader_error(
                        "incomplete UTF-16 surrogate pair",
                        self.offset,
                        -1,
                    );
                }
                return Ok((0, 0));
            }

            let value2 = self.raw_buffer.data[self.raw_buffer.pointer + low + 2] as u32
                + ((self.raw_buffer.data[self.raw_buffer.pointer + high + 2] as u32) << 8);

            if (value2 & 0xFC00) != 0xDC00 {
                return self.set_reader_error(
                    "expected low surrogate area",
                    self.offset + 2,
                    value2 as i32,
                );
            }

            let value = 0x10000 + ((value & 0x3FF) << 10) + (value2 & 0x3FF);
            Ok((value, 4))
        } else {
            Ok((value, 2))
        }
    }
}