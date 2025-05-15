use std::io::{self, Read, Seek};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort, c_schar, c_void};

pub type size_t = c_ulong;
pub type off_t = c_long;
pub type off64_t = c_long;

#[derive(Debug)]
pub struct FileBuffer {
    read_ptr: usize,
    read_end: usize,
    write_ptr: usize,
    write_base: usize,
    flags: i32,
    save_base: usize,
    save_end: usize,
}

impl FileBuffer {
    pub fn new() -> Self {
        FileBuffer {
            read_ptr: 0,
            read_end: 0,
            write_ptr: 0,
            write_base: 0,
            flags: 0,
            save_base: 0,
            save_end: 0,
        }
    }

    pub fn freadahead(&self) -> size_t {
        if self.write_ptr > self.write_base {
            return 0;
        }

        let read_available = (self.read_end - self.read_ptr) as c_long;
        let save_available = if self.flags & 0x100 != 0 {
            (self.save_end - self.save_base) as c_long
        } else {
            0
        };

        (read_available + save_available) as size_t
    }
}