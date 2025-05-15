use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::ptr;
use libc::{c_int, c_uint, c_ulong, c_void, off_t};
use memmap2::MmapOptions;

type VmaIterateCallbackFn = Box<dyn FnMut(*mut c_void, c_ulong, c_ulong, c_uint) -> c_int>;

struct Rofile {
    position: usize,
    filled: usize,
    eof_seen: bool,
    buffer: Vec<u8>,
    auxmap: Option<memmap2::Mmap>,
    file: File,
}

impl Rofile {
    fn open(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        Ok(Self {
            position: 0,
            filled: 0,
            eof_seen: false,
            buffer: vec![0; 1], // stack_allocated_buffer
            auxmap: None,
            file,
        })
    }

    fn peek_char(&mut self) -> Option<u8> {
        if self.position >= self.filled {
            self.eof_seen = true;
            None
        } else {
            Some(self.buffer[self.position])
        }
    }

    fn get_char(&mut self) -> Option<u8> {
        let c = self.peek_char();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    fn scan_hex(&mut self) -> Option<c_ulong> {
        let mut value = 0;
        let mut num_digits = 0;

        while let Some(c) = self.peek_char() {
            let digit = match c {
                b'0'..=b'9' => (c - b'0') as c_ulong,
                b'A'..=b'F' => (c - b'A' + 10) as c_ulong,
                b'a'..=b'f' => (c - b'a' + 10) as c_ulong,
                _ => break,
            };
            self.get_char();
            value = (value << 4) + digit;
            num_digits += 1;
        }

        if num_digits > 0 { Some(value) } else { None }
    }
}

fn vma_iterate_proc(callback: &mut VmaIterateCallbackFn, data: *mut c_void) -> c_int {
    let mut rof = match Rofile::open("/proc/self/maps") {
        Ok(f) => f,
        Err(_) => return -1,
    };

    let auxmap_range = rof.auxmap.as_ref().map(|m| {
        let start = m.as_ptr() as usize as c_ulong;
        let end = start + m.len() as c_ulong;
        (start, end)
    });

    loop {
        let start = match rof.scan_hex() {
            Some(v) => v,
            None => break,
        };

        if rof.get_char() != Some(b'-') {
            break;
        }

        let end = match rof.scan_hex() {
            Some(v) => v,
            None => break,
        };

        // Skip until permissions
        while rof.get_char() == Some(b' ') {}

        let mut flags = 0;
        if rof.peek_char() == Some(b'r') {
            flags |= 1 << 0;
            rof.get_char();
        }
        if rof.peek_char() == Some(b'w') {
            flags |= 1 << 1;
            rof.get_char();
        }
        if rof.peek_char() == Some(b'x') {
            flags |= 1 << 2;
            rof.get_char();
        }

        // Skip to end of line
        while rof.get_char().is_some() && rof.peek_char() != Some(b'\n') {}

        if let Some((auxmap_start, auxmap_end)) = auxmap_range {
            if start <= auxmap_start && auxmap_end - 1 <= end - 1 {
                if start < auxmap_start {
                    if callback(data, start, auxmap_start, flags) != 0 {
                        break;
                    }
                }
                if auxmap_end - 1 < end - 1 {
                    if callback(data, auxmap_end, end, flags) != 0 {
                        break;
                    }
                }
                continue;
            }
        }

        if callback(data, start, end, flags) != 0 {
            break;
        }
    }

    0
}

fn vma_iterate_bsd(_callback: &mut VmaIterateCallbackFn, _data: *mut c_void) -> c_int {
    -1
}

pub fn vma_iterate(callback: VmaIterateCallbackFn, data: *mut c_void) -> c_int {
    let mut callback = callback;
    let retval = vma_iterate_proc(&mut callback, data);
    if retval == 0 {
        0
    } else {
        vma_iterate_bsd(&mut callback, data)
    }
}