use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_uchar, c_long, c_void};
use std::ptr;
use std::cmp::Ordering;
use std::fmt::{self, Write};

#[derive(Debug, Clone)]
pub struct String {
    len: u32,
    data: Option<Box<[u8]>>,
}

impl String {
    pub fn new() -> Self {
        String {
            len: 0,
            data: None,
        }
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn duplicate(&self) -> Result<Self, i32> {
        let data = match &self.data {
            Some(d) => {
                let mut new_data = vec![0u8; (self.len + 1) as usize];
                new_data[..self.len as usize].copy_from_slice(&d[..self.len as usize]);
                new_data[self.len as usize] = b'\0';
                Some(new_data.into_boxed_slice())
            }
            None => None,
        };

        Ok(String {
            len: self.len,
            data,
        })
    }

    pub fn copy(&mut self, src: &[u8]) -> Result<(), i32> {
        let mut data = vec![0u8; (src.len() + 1) as usize];
        data[..src.len()].copy_from_slice(src);
        data[src.len()] = b'\0';
        
        self.len = src.len() as u32;
        self.data = Some(data.into_boxed_slice());
        Ok(())
    }

    pub fn compare(&self, other: &String) -> i32 {
        match self.len.cmp(&other.len) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => {
                let self_slice = self.data.as_ref().map_or(&[], |d| &d[..self.len as usize]);
                let other_slice = other.data.as_ref().map_or(&[], |d| &d[..other.len as usize]);
                self_slice.cmp(other_slice) as i32
            }
        }
    }
}

const HEX: &[u8] = b"0123456789abcdef";

fn safe_utoa(base: u32, val: u64, buf: &mut [u8]) -> &[u8] {
    let mut val = val;
    let mut pos = buf.len() - 1;
    buf[pos] = b'\0';
    
    loop {
        pos -= 1;
        buf[pos] = HEX[(val % base as u64) as usize];
        val /= base as u64;
        if val == 0 {
            break;
        }
    }
    
    &buf[pos..]
}

fn safe_itoa(base: i32, mut val: i64, buf: &mut [u8]) -> &[u8] {
    let orig_len = buf.len();
    let is_neg = val < 0;
    let mut pos = orig_len - 1;
    buf[pos] = b'\0';
    
    if is_neg {
        val = -val;
    }

    if is_neg && base == 16 {
        val -= 1;
        for i in 0..16 {
            buf[pos - i - 1] = b'0';
        }
    }

    loop {
        pos -= 1;
        buf[pos] = HEX[(val % base as i64) as usize];
        val /= base as i64;
        if val == 0 {
            break;
        }
    }

    if is_neg && base == 10 {
        pos -= 1;
        buf[pos] = b'-';
    }

    if is_neg && base == 16 {
        pos = orig_len - 1;
        for i in 0..16 {
            match buf[pos - i - 1] {
                b'0' => buf[pos - i - 1] = b'f',
                b'1' => buf[pos - i - 1] = b'e',
                b'2' => buf[pos - i - 1] = b'd',
                b'3' => buf[pos - i - 1] = b'c',
                b'4' => buf[pos - i - 1] = b'b',
                b'5' => buf[pos - i - 1] = b'a',
                b'6' => buf[pos - i - 1] = b'9',
                b'7' => buf[pos - i - 1] = b'8',
                b'8' => buf[pos - i - 1] = b'7',
                b'9' => buf[pos - i - 1] = b'6',
                b'a' => buf[pos - i - 1] = b'5',
                b'b' => buf[pos - i - 1] = b'4',
                b'c' => buf[pos - i - 1] = b'3',
                b'd' => buf[pos - i - 1] = b'2',
                b'e' => buf[pos - i - 1] = b'1',
                b'f' => buf[pos - i - 1] = b'0',
                _ => {}
            }
        }
    }

    &buf[pos..]
}

pub fn safe_snprintf(to: &mut [u8], format: &str, args: &[&dyn fmt::Display]) -> usize {
    let mut pos = 0;
    let mut arg_idx = 0;
    let format_bytes = format.as_bytes();
    let mut i = 0;
    
    while i < format_bytes.len() && pos < to.len() - 1 {
        if format_bytes[i] != b'%' {
            to[pos] = format_bytes[i];
            pos += 1;
            i += 1;
            continue;
        }
        
        i += 1;
        if i >= format_bytes.len() {
            break;
        }
        
        let specifier = format_bytes[i];
        if arg_idx >= args.len() {
            break;
        }
        
        match specifier {
            b'd' | b'i' | b'u' | b'x' | b'p' => {
                let mut buf = [0u8; 22];
                let s = match specifier {
                    b'u' => {
                        let val = args[arg_idx].to_string().parse::<u64>().unwrap_or(0);
                        safe_utoa(if specifier == b'x' { 16 } else { 10 }, val, &mut buf)
                    }
                    _ => {
                        let val = args[arg_idx].to_string().parse::<i64>().unwrap_or(0);
                        safe_itoa(if specifier == b'x' { 16 } else { 10 }, val, &mut buf)
                    }
                };
                
                for &c in s {
                    if pos >= to.len() - 1 {
                        break;
                    }
                    to[pos] = c;
                    pos += 1;
                }
                arg_idx += 1;
            }
            b's' => {
                let s = args[arg_idx].to_string();
                for c in s.bytes() {
                    if pos >= to.len() - 1 {
                        break;
                    }
                    to[pos] = c;
                    pos += 1;
                }
                arg_idx += 1;
            }
            _ => {}
        }
        i += 1;
    }
    
    to[pos] = b'\0';
    pos
}