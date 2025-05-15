/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;
use std::str;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String {
    len: u32,
    data: Vec<u8>,
}

impl String {
    pub fn new() -> Self {
        String {
            len: 0,
            data: Vec::new(),
        }
    }

    pub fn from_text(text: &str) -> Self {
        String {
            len: text.len() as u32,
            data: text.as_bytes().to_vec(),
        }
    }

    pub fn from_raw(raw: &[u8]) -> Self {
        String {
            len: raw.len() as u32,
            data: raw.to_vec(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn duplicate(&self) -> Result<Self, std::io::Error> {
        Ok(String {
            len: self.len,
            data: self.data.clone(),
        })
    }

    pub fn copy(src: &[u8]) -> Result<Self, std::io::Error> {
        Ok(String {
            len: src.len() as u32,
            data: src.to_vec(),
        })
    }

    pub fn compare(&self, other: &String) -> Ordering {
        if self.len != other.len {
            return self.len.cmp(&other.len);
        }
        self.data.cmp(&other.data)
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.data).unwrap_or("")
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Default for String {
    fn default() -> Self {
        String::new()
    }
}

impl Deref for String {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn safe_vsnprintf(
    to: &mut [u8],
    format: &str,
    args: std::fmt::Arguments,
) -> Result<usize, std::fmt::Error> {
    let mut writer = Writer::new(to);
    fmt::write(&mut writer, args)?;
    Ok(writer.bytes_written())
}

struct Writer<'a> {
    buffer: &'a mut [u8],
    pos: usize,
}

impl<'a> Writer<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Writer { buffer, pos: 0 }
    }

    fn bytes_written(&self) -> usize {
        self.pos
    }
}

impl<'a> fmt::Write for Writer<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.buffer.len() - self.pos;
        let to_copy = std::cmp::min(remaining, bytes.len());
        if to_copy > 0 {
            self.buffer[self.pos..self.pos + to_copy].copy_from_slice(&bytes[..to_copy]);
            self.pos += to_copy;
        }
        Ok(())
    }
}

pub fn safe_snprintf(
    to: &mut [u8],
    format: &str,
    args: std::fmt::Arguments,
) -> Result<usize, std::fmt::Error> {
    safe_vsnprintf(to, format, args)
}

const HEX: &[u8] = b"0123456789abcdef";

fn safe_utoa(base: u32, val: u64, buf: &mut [u8]) -> &[u8] {
    let mut pos = buf.len() - 1;
    buf[pos] = 0;
    pos -= 1;
    let mut val = val;
    loop {
        buf[pos] = HEX[(val % base as u64) as usize];
        val /= base as u64;
        if val == 0 {
            break;
        }
        pos -= 1;
    }
    &buf[pos + 1..]
}

fn safe_itoa(base: u32, val: i64, buf: &mut [u8]) -> &[u8] {
    let mut pos = buf.len() - 1;
    buf[pos] = 0;
    pos -= 1;
    let is_neg = val < 0;
    let mut val = if is_neg { -val } else { val };

    if is_neg && base == 16 {
        val -= 1;
        for i in 0..16 {
            buf[pos - i] = b'0';
        }
    }

    loop {
        buf[pos] = HEX[(val % base as i64) as usize];
        val /= base as i64;
        if val == 0 {
            break;
        }
        pos -= 1;
    }

    if is_neg && base == 10 {
        pos -= 1;
        buf[pos] = b'-';
    }

    if is_neg && base == 16 {
        pos = buf.len() - 2;
        for i in 0..16 {
            buf[pos - i] = match buf[pos - i] {
                b'0' => b'f',
                b'1' => b'e',
                b'2' => b'd',
                b'3' => b'c',
                b'4' => b'b',
                b'5' => b'a',
                b'6' => b'9',
                b'7' => b'8',
                b'8' => b'7',
                b'9' => b'6',
                b'a' => b'5',
                b'b' => b'4',
                b'c' => b'3',
                b'd' => b'2',
                b'e' => b'1',
                b'f' => b'0',
                _ => buf[pos - i],
            };
        }
    }

    &buf[pos..]
}