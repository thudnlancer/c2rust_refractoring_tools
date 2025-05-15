use std::time::SystemTime;
use std::io::{Read, Write, Seek, SeekFrom};
use std::convert::TryInto;

type MtOffT = i64;
type SizeT = usize;
type SSizeT = isize;

struct Stream {
    next: Box<dyn StreamOps>,
    refs: i32,
}

trait StreamOps: Read + Write + Seek {
    fn get_data(&mut self, date: &mut SystemTime, size: &mut MtOffT, type_: &mut i32, address: &mut u32) -> i32;
    fn pre_allocate(&mut self, size: MtOffT) -> i32;
    fn discard(&mut self) -> i32;
    fn flush(&mut self) -> std::io::Result<()>;
}

struct Filter {
    inner: Box<dyn StreamOps>,
    buffer: [u8; 4096],
    read_bytes: SizeT,
    buf_pos: SizeT,
    pending_nl: bool,
    eof: bool,
}

impl Read for Filter {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.eof {
            return Ok(0);
        }

        let mut bytes_read = 0;
        while bytes_read < buf.len() && !self.eof {
            let c = if self.pending_nl {
                self.pending_nl = false;
                b'\n'
            } else {
                if self.buf_pos == self.read_bytes {
                    let bytes = self.inner.read(&mut self.buffer)?;
                    if bytes == 0 {
                        self.eof = true;
                        b'\x1a'
                    } else {
                        self.read_bytes = bytes;
                        self.buf_pos = 0;
                        continue;
                    }
                } else {
                    let c = self.buffer[self.buf_pos];
                    self.buf_pos += 1;
                    if c == b'\n' {
                        self.pending_nl = true;
                        b'\r'
                    } else {
                        c
                    }
                }
            };
            buf[bytes_read] = c;
            bytes_read += 1;
        }
        Ok(bytes_read)
    }
}

impl StreamOps for Filter {
    fn get_data(&mut self, date: &mut SystemTime, size: &mut MtOffT, type_: &mut i32, address: &mut u32) -> i32 {
        self.inner.get_data(date, size, type_, address)
    }

    fn pre_allocate(&mut self, size: MtOffT) -> i32 {
        self.inner.pre_allocate(size)
    }

    fn discard(&mut self) -> i32 {
        self.inner.discard()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Write for Filter {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

impl Seek for Filter {
    fn seek(&mut self, _pos: SeekFrom) -> std::io::Result<u64> {
        unimplemented!()
    }
}

fn open_unix2dos(next: Box<dyn StreamOps>, _convert_charset: i32) -> Box<dyn StreamOps> {
    Box::new(Filter {
        inner: next,
        buffer: [0; 4096],
        read_bytes: 0,
        buf_pos: 0,
        pending_nl: false,
        eof: false,
    })
}