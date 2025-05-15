use std::io::{self, Write};

pub type SizeT = usize;
pub type YamlCharT = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YamlEncoding {
    Any,
    Utf8,
    Utf16Le,
    Utf16Be,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YamlErrorType {
    NoError,
    MemoryError,
    ReaderError,
    ScannerError,
    ParserError,
    ComposerError,
    WriterError,
    EmitterError,
}

#[derive(Debug, Clone, Copy)]
pub struct YamlMark {
    pub index: SizeT,
    pub line: SizeT,
    pub column: SizeT,
}

pub struct YamlEmitter {
    error: YamlErrorType,
    problem: Option<String>,
    write_handler: Box<dyn FnMut(&[u8]) -> io::Result<()>>,
    encoding: YamlEncoding,
    buffer: Vec<YamlCharT>,
    raw_buffer: Vec<u8>,
}

impl YamlEmitter {
    pub fn new<W: 'static + Write>(writer: W, encoding: YamlEncoding) -> Self {
        YamlEmitter {
            error: YamlErrorType::NoError,
            problem: None,
            write_handler: Box::new(move |data| writer.write_all(data)),
            encoding,
            buffer: Vec::new(),
            raw_buffer: Vec::new(),
        }
    }

    fn set_writer_error(&mut self, problem: &str) -> bool {
        self.error = YamlErrorType::WriterError;
        self.problem = Some(problem.to_string());
        false
    }

    pub fn flush(&mut self) -> bool {
        if self.buffer.is_empty() {
            return true;
        }

        match self.encoding {
            YamlEncoding::Utf8 => {
                if (self.write_handler)(&self.buffer).is_err() {
                    return self.set_writer_error("write error");
                }
            }
            YamlEncoding::Utf16Le | YamlEncoding::Utf16Be => {
                let (low, high) = match self.encoding {
                    YamlEncoding::Utf16Le => (0, 1),
                    YamlEncoding::Utf16Be => (1, 0),
                    _ => unreachable!(),
                };

                let mut i = 0;
                while i < self.buffer.len() {
                    let (width, value) = self.decode_utf8(i);
                    i += width;

                    if value < 0x10000 {
                        self.raw_buffer.push((value >> 8) as u8);
                        self.raw_buffer.push((value & 0xFF) as u8);
                    } else {
                        let value = value - 0x10000;
                        self.raw_buffer.push((0xD8 + (value >> 18)) as u8);
                        self.raw_buffer.push((value >> 10 & 0xFF) as u8);
                        self.raw_buffer.push((0xDC + (value >> 8 & 0xFF)) as u8);
                        self.raw_buffer.push((value & 0xFF) as u8);
                    }
                }

                if (self.write_handler)(&self.raw_buffer).is_err() {
                    return self.set_writer_error("write error");
                }
            }
            _ => return self.set_writer_error("unsupported encoding"),
        }

        self.buffer.clear();
        self.raw_buffer.clear();
        true
    }

    fn decode_utf8(&self, pos: usize) -> (usize, u32) {
        let octet = self.buffer[pos];
        let (width, value) = match octet {
            _ if octet & 0x80 == 0 => (1, octet as u32),
            _ if octet & 0xE0 == 0xC0 => (2, (octet & 0x1F) as u32),
            _ if octet & 0xF0 == 0xE0 => (3, (octet & 0x0F) as u32),
            _ if octet & 0xF8 == 0xF0 => (4, (octet & 0x07) as u32),
            _ => (0, 0),
        };

        let mut value = value;
        for i in 1..width {
            value = (value << 6) | (self.buffer[pos + i] & 0x3F) as u32;
        }

        (width, value)
    }
}