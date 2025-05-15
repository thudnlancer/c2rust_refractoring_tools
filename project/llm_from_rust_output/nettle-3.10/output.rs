use std::io::{self, Write};
use std::fmt;
use std::error::Error;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpMode {
    Canonical,
    Advanced,
    Transport,
}

#[derive(Debug)]
pub struct SexpOutput<'a> {
    writer: &'a mut dyn Write,
    line_width: u32,
    coding: Option<&'a NettleArmor>,
    coding_indent: u32,
    prefer_hex: bool,
    hash: Option<&'a NettleHash>,
    ctx: Option<Box<dyn std::any::Any>>,
    base64: Base64DecodeCtx,
    pos: u32,
    soft_newline: bool,
}

#[derive(Debug)]
pub struct NettleBuffer {
    contents: Vec<u8>,
    realloc_ctx: Option<Box<dyn std::any::Any>>,
    realloc: Option<fn(&mut dyn std::any::Any, *mut u8, usize) -> *mut u8>,
}

#[derive(Debug)]
pub struct NettleHash {
    name: &'static str,
    context_size: u32,
    digest_size: u32,
    block_size: u32,
    init: fn(&mut dyn std::any::Any),
    update: fn(&mut dyn std::any::Any, usize, &[u8]),
    digest: fn(&mut dyn std::any::Any, usize, &mut [u8]),
}

#[derive(Debug)]
pub struct NettleArmor {
    name: &'static str,
    encode_context_size: u32,
    decode_context_size: u32,
    encode_final_length: u32,
    encode_init: fn(&mut dyn std::any::Any),
    encode_length: fn(usize) -> usize,
    encode_update: fn(&mut dyn std::any::Any, &mut [u8], usize, &[u8]) -> usize,
    encode_final: fn(&mut dyn std::any::Any, &mut [u8]) -> usize,
    decode_init: fn(&mut dyn std::any::Any),
    decode_length: fn(usize) -> usize,
    decode_update: fn(&mut dyn std::any::Any, &mut usize, &mut [u8], usize, &[u8]) -> i32,
    decode_final: fn(&mut dyn std::any::Any) -> i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Base64DecodeCtx {
    table: &'static [i8],
    word: u16,
    bits: u8,
    padding: u8,
}

static NETTLE_BASE64: NettleArmor = NettleArmor {
    name: "base64",
    encode_context_size: 0,
    decode_context_size: 0,
    encode_final_length: 0,
    encode_init: |_| {},
    encode_length: |_| 0,
    encode_update: |_, _, _, _| 0,
    encode_final: |_, _| 0,
    decode_init: |_| {},
    decode_length: |_| 0,
    decode_update: |_, _, _, _, _| 0,
    decode_final: |_| 0,
};

static NETTLE_BASE16: NettleArmor = NettleArmor {
    name: "base16",
    encode_context_size: 0,
    decode_context_size: 0,
    encode_final_length: 0,
    encode_init: |_| {},
    encode_length: |_| 0,
    encode_update: |_, _, _, _| 0,
    encode_final: |_, _| 0,
    decode_init: |_| {},
    decode_length: |_| 0,
    decode_update: |_, _, _, _, _| 0,
    decode_final: |_| 0,
};

impl<'a> SexpOutput<'a> {
    pub fn new(writer: &'a mut dyn Write, width: u32, prefer_hex: bool) -> Self {
        SexpOutput {
            writer,
            line_width: width,
            coding: None,
            coding_indent: 0,
            prefer_hex,
            hash: None,
            ctx: None,
            base64: Base64DecodeCtx {
                table: &[],
                word: 0,
                bits: 0,
                padding: 0,
            },
            pos: 0,
            soft_newline: false,
        }
    }

    pub fn hash_init(&mut self, hash: &'a NettleHash, ctx: Box<dyn std::any::Any>) {
        self.hash = Some(hash);
        self.ctx = Some(ctx);
        (hash.init)(self.ctx.as_mut().unwrap());
    }

    fn put_raw_char(&mut self, c: u8) -> io::Result<()> {
        self.writer.write_all(&[c])?;
        self.pos += 1;
        self.soft_newline = false;
        Ok(())
    }

    pub fn put_newline(&mut self, indent: u32) -> io::Result<()> {
        if !self.soft_newline {
            self.put_raw_char(b'\n')?;
            self.pos = 0;
            for _ in 0..indent {
                self.put_raw_char(b' ')?;
            }
            self.pos = indent;
        }
        Ok(())
    }

    pub fn put_soft_newline(&mut self, indent: u32) -> io::Result<()> {
        self.put_newline(indent)?;
        self.soft_newline = true;
        Ok(())
    }

    pub fn put_char(&mut self, c: u8) -> io::Result<()> {
        if let Some(coding) = self.coding {
            let mut encoded = [0; 2];
            let done = (coding.encode_update)(
                &mut self.base64 as *mut _ as *mut dyn std::any::Any,
                &mut encoded,
                1,
                &[c],
            );
            assert!(done <= encoded.len());

            for i in 0..done {
                if self.line_width != 0
                    && self.pos >= self.line_width
                    && self.pos >= self.coding_indent + 10
                {
                    self.put_newline(self.coding_indent)?;
                }
                self.put_raw_char(encoded[i] as u8)?;
            }
        } else if let Some(hash) = self.hash {
            (hash.update)(self.ctx.as_mut().unwrap(), 1, &[c]);
        } else {
            self.put_raw_char(c)?;
        }
        Ok(())
    }

    pub fn put_data(&mut self, data: &[u8]) -> io::Result<()> {
        for &c in data {
            self.put_char(c)?;
        }
        Ok(())
    }

    fn put_length(&mut self, length: u32) -> io::Result<()> {
        let mut digit = 1;
        while digit * 10 <= length {
            digit *= 10;
        }
        while digit != 0 {
            self.put_char(b'0' + (length / digit) as u8)?;
            digit /= 10;
        }
        Ok(())
    }

    pub fn put_code_start(&mut self, coding: &'a NettleArmor) -> io::Result<()> {
        assert!(self.coding.is_none());
        self.coding_indent = self.pos;
        self.coding = Some(coding);
        (coding.encode_init)(&mut self.base64 as *mut _ as *mut dyn std::any::Any);
        Ok(())
    }

    pub fn put_code_end(&mut self) -> io::Result<()> {
        assert!(self.coding.is_some());
        let mut encoded = [0; 3];
        let done = (self.coding.unwrap().encode_final)(
            &mut self.base64 as *mut _ as *mut dyn std::any::Any,
            &mut encoded,
        );
        assert!(done <= encoded.len());
        self.coding = None;
        self.put_data(&encoded[..done])
    }

    pub fn put_string(&mut self, mode: SexpMode, string: &NettleBuffer) -> io::Result<()> {
        if string.contents.is_empty() {
            let prefix = match mode {
                SexpMode::Advanced => b"\"\"",
                _ => b"0:",
            };
            self.put_data(prefix)
        } else if mode == SexpMode::Advanced {
            let mut token = (string.contents[0] < b'0' || string.contents[0] > b'9') as i32;
            let mut quote_friendly = true;
            static ESCAPE_NAMES: [u8; 32] = [
                0, 0, 0, 0, 0, 0, 0, 0, b'b', b't', b'n', 0, b'f', b'r', 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ];

            for &c in &string.contents {
                if token != 0 && (c >= 0x80 || SEXP_TOKEN_CHARS[c as usize] == 0) {
                    token = 0;
                }
                if quote_friendly {
                    if c >= 0x7f {
                        quote_friendly = false;
                    } else if c < 0x20 && ESCAPE_NAMES[c as usize] == 0 {
                        quote_friendly = false;
                    }
                }
            }

            if token != 0 {
                self.put_data(&string.contents)
            } else if quote_friendly {
                self.put_char(b'"')?;
                for &c in &string.contents {
                    let mut escape = false;
                    let mut c = c;
                    assert!(c < 0x7f);
                    if c == b'\\' || c == b'"' {
                        escape = true;
                    } else if c < 0x20 {
                        escape = true;
                        c = ESCAPE_NAMES[c as usize];
                        assert!(c != 0);
                    }
                    if escape {
                        self.put_char(b'\\')?;
                    }
                    self.put_char(c)?;
                }
                self.put_char(b'"')
            } else {
                let (delimiter, coding) = if self.prefer_hex {
                    (b'#', &NETTLE_BASE16)
                } else {
                    (b'|', &NETTLE_BASE64)
                };
                self.put_char(delimiter)?;
                self.put_code_start(coding)?;
                self.put_data(&string.contents)?;
                self.put_code_end()?;
                self.put_char(delimiter)
            }
        } else {
            self.put_length(string.contents.len() as u32)?;
            self.put_char(b':')?;
            self.put_data(&string.contents)
        }
    }

    pub fn put_digest(&mut self) -> io::Result<()> {
        assert!(self.hash.is_some());
        let hash = self.hash.unwrap();
        let mut digest = vec![0; hash.digest_size as usize];
        (hash.digest)(
            self.ctx.as_mut().unwrap(),
            hash.digest_size as usize,
            &mut digest,
        );
        self.put_code_start(&NETTLE_BASE16)?;
        self.put_data(&digest)?;
        self.put_code_end()
    }
}

static SEXP_TOKEN_CHARS: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1,
];