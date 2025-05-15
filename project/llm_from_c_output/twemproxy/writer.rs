use std::io::Write;

#[derive(Debug)]
pub enum YamlError {
    WriterError(String),
}

pub struct YamlEmitter<'a> {
    buffer: Buffer,
    raw_buffer: Buffer,
    encoding: Encoding,
    write_handler: Box<dyn FnMut(&[u8]) -> Result<(), std::io::Error> + 'a>,
    error: Option<YamlError>,
}

pub struct Buffer {
    start: *mut u8,
    pointer: *mut u8,
    last: *mut u8,
}

#[derive(PartialEq)]
pub enum Encoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

impl<'a> YamlEmitter<'a> {
    fn set_writer_error(&mut self, problem: &str) -> Result<(), YamlError> {
        self.error = Some(YamlError::WriterError(problem.to_string()));
        Err(YamlError::WriterError(problem.to_string()))
    }

    pub fn flush(&mut self) -> Result<(), YamlError> {
        assert!(self.write_handler.as_ref() as *const _ != std::ptr::null());
        assert!(self.encoding != Encoding::Utf8 || self.encoding != Encoding::Utf16Le || self.encoding != Encoding::Utf16Be);

        self.buffer.last = self.buffer.pointer;
        self.buffer.pointer = self.buffer.start;

        if self.buffer.start == self.buffer.last {
            return Ok(());
        }

        if self.encoding == Encoding::Utf8 {
            let data = unsafe {
                std::slice::from_raw_parts(
                    self.buffer.start,
                    self.buffer.last as usize - self.buffer.start as usize,
                )
            };
            if (self.write_handler)(data).is_ok() {
                self.buffer.last = self.buffer.start;
                self.buffer.pointer = self.buffer.start;
                Ok(())
            } else {
                self.set_writer_error("write error")
            }
        } else {
            let low = if self.encoding == Encoding::Utf16Le { 0 } else { 1 };
            let high = if self.encoding == Encoding::Utf16Le { 1 } else { 0 };

            let mut buffer_ptr = self.buffer.pointer;
            while buffer_ptr != self.buffer.last {
                let octet = unsafe { *buffer_ptr };
                let width = if (octet & 0x80) == 0x00 {
                    1
                } else if (octet & 0xE0) == 0xC0 {
                    2
                } else if (octet & 0xF0) == 0xE0 {
                    3
                } else if (octet & 0xF8) == 0xF0 {
                    4
                } else {
                    0
                };

                let mut value = if (octet & 0x80) == 0x00 {
                    octet & 0x7F
                } else if (octet & 0xE0) == 0xC0 {
                    octet & 0x1F
                } else if (octet & 0xF0) == 0xE0 {
                    octet & 0x0F
                } else if (octet & 0xF8) == 0xF0 {
                    octet & 0x07
                } else {
                    0
                };

                for k in 1..width {
                    let octet = unsafe { *buffer_ptr.add(k) };
                    value = (value << 6) + (octet & 0x3F);
                }

                buffer_ptr = unsafe { buffer_ptr.add(width) };

                if value < 0x10000 {
                    unsafe {
                        *self.raw_buffer.last.add(high) = (value >> 8) as u8;
                        *self.raw_buffer.last.add(low) = (value & 0xFF) as u8;
                        self.raw_buffer.last = self.raw_buffer.last.add(2);
                    }
                } else {
                    value -= 0x10000;
                    unsafe {
                        *self.raw_buffer.last.add(high) = (0xD8 + (value >> 18)) as u8;
                        *self.raw_buffer.last.add(low) = ((value >> 10) & 0xFF) as u8;
                        *self.raw_buffer.last.add(high + 2) = (0xDC + ((value >> 8) & 0xFF)) as u8;
                        *self.raw_buffer.last.add(low + 2) = (value & 0xFF) as u8;
                        self.raw_buffer.last = self.raw_buffer.last.add(4);
                    }
                }
            }

            let data = unsafe {
                std::slice::from_raw_parts(
                    self.raw_buffer.start,
                    self.raw_buffer.last as usize - self.raw_buffer.start as usize,
                )
            };
            if (self.write_handler)(data).is_ok() {
                self.buffer.last = self.buffer.start;
                self.buffer.pointer = self.buffer.start;
                self.raw_buffer.last = self.raw_buffer.start;
                self.raw_buffer.pointer = self.raw_buffer.start;
                Ok(())
            } else {
                self.set_writer_error("write error")
            }
        }
    }
}