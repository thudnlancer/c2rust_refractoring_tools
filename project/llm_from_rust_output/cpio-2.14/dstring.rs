use std::io::{self, Read};
use std::ptr;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

pub type size_t = usize;

#[derive(Default)]
pub struct DynamicString {
    size: size_t,
    idx: size_t,
    string: Vec<u8>,
}

impl DynamicString {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resize(&mut self, len: size_t) {
        while len + self.idx >= self.size {
            self.size = if self.size == 0 { 1 } else { self.size * 2 };
            self.string.resize(self.size, 0);
        }
    }

    pub fn reset(&mut self, len: size_t) {
        self.resize(len);
        self.idx = len;
    }

    pub fn fgetstr<R: Read>(&mut self, f: &mut R, eos: u8) -> io::Result<Option<&CStr>> {
        self.idx = 0;
        let mut next_ch = [0u8; 1];
        
        loop {
            match f.read_exact(&mut next_ch) {
                Ok(_) => {
                    if next_ch[0] == eos || next_ch[0] == 0xFF {
                        break;
                    }
                    self.resize(0);
                    self.string[self.idx] = next_ch[0];
                    self.idx += 1;
                }
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    if self.idx == 0 {
                        return Ok(None);
                    }
                    break;
                }
                Err(e) => return Err(e),
            }
        }

        self.resize(0);
        self.string[self.idx] = b'\0';
        Ok(Some(unsafe { CStr::from_ptr(self.string.as_ptr() as *const c_char) }))
    }

    pub fn append(&mut self, c: u8) {
        self.resize(0);
        self.string[self.idx] = c;
        if c != 0 {
            self.idx += 1;
            self.resize(0);
            self.string[self.idx] = 0;
        }
    }

    pub fn concat(&mut self, s: &CStr) {
        let bytes = s.to_bytes_with_nul();
        let len = bytes.len() - 1; // exclude null terminator
        self.resize(len);
        self.string[self.idx..self.idx + len].copy_from_slice(&bytes[..len]);
        self.idx += len;
        self.string[self.idx] = 0;
    }

    pub fn fgets<R: Read>(&mut self, f: &mut R) -> io::Result<Option<&CStr>> {
        self.fgetstr(f, b'\n')
    }

    pub fn fgetname<R: Read>(&mut self, f: &mut R) -> io::Result<Option<&CStr>> {
        self.fgetstr(f, 0)
    }

    pub fn endswith(&self, c: u8) -> bool {
        self.idx > 0 && self.string[self.idx - 1] == c
    }
}

impl Drop for DynamicString {
    fn drop(&mut self) {
        // Vec will automatically deallocate
    }
}