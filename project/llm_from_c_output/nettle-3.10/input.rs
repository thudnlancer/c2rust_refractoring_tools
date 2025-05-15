use std::io::{self, Read, BufRead};
use std::fmt;
use std::error::Error;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpCharType {
    Normal,
    Eof,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpToken {
    String,
    ListStart,
    ListEnd,
    DisplayStart,
    DisplayEnd,
    TransportStart,
    CodingEnd,
    Comment,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SexpMode {
    Canonical,
    Advanced,
}

pub trait Armor {
    fn decode_init(&self, state: &mut ());
    fn decode_update(&self, state: &mut (), dst: &mut [u8], src: &[u8]) -> Result<usize, Box<dyn Error>>;
    fn decode_final(&self, state: &mut ()) -> Result<(), Box<dyn Error>>;
}

pub struct Base16;
pub struct Base64;

impl Armor for Base16 {
    fn decode_init(&self, _state: &mut ()) {}
    fn decode_update(&self, _state: &mut (), dst: &mut [u8], src: &[u8]) -> Result<usize, Box<dyn Error>> {
        // Base16 decode implementation
        Ok(0)
    }
    fn decode_final(&self, _state: &mut ()) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Armor for Base64 {
    fn decode_init(&self, _state: &mut ()) {}
    fn decode_update(&self, _state: &mut (), dst: &mut [u8], src: &[u8]) -> Result<usize, Box<dyn Error>> {
        // Base64 decode implementation
        Ok(0)
    }
    fn decode_final(&self, _state: &mut ()) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub struct SexpInput<R: Read> {
    reader: R,
    ctype: SexpCharType,
    c: u8,
    coding: Option<&'static dyn Armor>,
    state: (),
    terminator: u8,
    token: SexpToken,
}

impl<R: Read> SexpInput<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            ctype: SexpCharType::Normal,
            c: 0,
            coding: None,
            state: (),
            terminator: 0,
            token: SexpToken::Eof,
        }
    }

    pub fn get_char(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(coding) = self.coding {
            loop {
                self.get_raw_char()?;
                if self.ctype == SexpCharType::Eof {
                    return Err("Unexpected end of file in coded data".into());
                }

                if self.c == self.terminator {
                    self.ctype = SexpCharType::End;
                    return Ok(());
                }

                let mut dst = [0u8; 1];
                let src = [self.c];
                let done = coding.decode_update(&mut self.state, &mut dst, &src)?;

                if done == 1 {
                    self.c = dst[0];
                    return Ok(());
                }
            }
        } else {
            self.get_raw_char()
        }
    }

    fn get_raw_char(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = [0u8; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => {
                self.ctype = SexpCharType::Eof;
                Ok(())
            }
            Ok(_) => {
                self.ctype = SexpCharType::Normal;
                self.c = buf[0];
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn next_char(&mut self) -> Result<u8, Box<dyn Error>> {
        self.get_char()?;
        if self.ctype != SexpCharType::Normal {
            Err("Unexpected end of file".into())
        } else {
            Ok(self.c)
        }
    }

    fn push_char(&mut self, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        if self.ctype != SexpCharType::Normal {
            return Err("Expected normal character".into());
        }
        string.push(self.c);
        Ok(())
    }

    fn start_coding(&mut self, coding: &'static dyn Armor, terminator: u8) -> Result<(), Box<dyn Error>> {
        if self.coding.is_some() {
            return Err("Already in coding mode".into());
        }
        coding.decode_init(&mut self.state);
        self.coding = Some(coding);
        self.terminator = terminator;
        Ok(())
    }

    fn end_coding(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(coding) = self.coding {
            coding.decode_final(&mut self.state)?;
            self.coding = None;
            Ok(())
        } else {
            Err("Not in coding mode".into())
        }
    }

    fn get_quoted_char(&mut self) -> Result<bool, Box<dyn Error>> {
        self.next_char()?;

        match self.c {
            b'"' => Ok(false),
            b'\\' => {
                self.next_char()?;
                match self.c {
                    b'b' => { self.c = b'\b'; Ok(true) },
                    b't' => { self.c = b'\t'; Ok(true) },
                    b'n' => { self.c = b'\n'; Ok(true) },
                    b'f' => { self.c = b'\f'; Ok(true) },
                    b'r' => { self.c = b'\r'; Ok(true) },
                    b'\\' => { self.c = b'\\'; Ok(true) },
                    b'o' | b'x' => Err("Octal/hex escape not implemented".into()),
                    b'\n' => {
                        if self.next_char()? == b'\r' {
                            self.next_char()?;
                        }
                        Ok(true)
                    },
                    b'\r' => {
                        if self.next_char()? == b'\n' {
                            self.next_char()?;
                        }
                        Ok(true)
                    },
                    _ => Ok(true),
                }
            },
            _ => Ok(true),
        }
    }

    fn get_token_string(&mut self, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        if self.coding.is_some() || self.ctype != SexpCharType::Normal {
            return Err("Invalid state for token string".into());
        }

        if !is_token_char(self.c) {
            return Err("Invalid token character".into());
        }

        loop {
            self.push_char(string)?;
            self.get_char()?;
            if self.ctype != SexpCharType::Normal || !is_token_char(self.c) {
                break;
            }
        }

        if string.is_empty() {
            Err("Empty token string".into())
        } else {
            Ok(())
        }
    }

    fn get_string(&mut self, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        string.clear();
        self.token = SexpToken::String;

        match self.c {
            b'"' => {
                while self.get_quoted_char()? {
                    self.push_char(string)?;
                }
                self.get_char()
            },
            b'#' => {
                self.start_coding(&Base16, b'#')?;
                self.decode_string(string)
            },
            b'|' => {
                self.start_coding(&Base64, b'|')?;
                self.decode_string(string)
            },
            _ => {
                if !is_token_char(self.c) {
                    Err("Invalid string start character".into())
                } else {
                    self.get_token_string(string)
                }
            },
        }
    }

    fn decode_string(&mut self, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        loop {
            self.get_char()?;
            match self.ctype {
                SexpCharType::Normal => self.push_char(string)?,
                SexpCharType::Eof => return Err("Unexpected end of file in coded string".into()),
                SexpCharType::End => {
                    self.end_coding()?;
                    self.get_char()?;
                    return Ok(());
                },
            }
        }
    }

    fn get_string_length(&mut self, mode: SexpMode, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        string.clear();
        self.token = SexpToken::String;

        let mut length = (self.c - b'0') as usize;

        if length == 0 {
            self.next_char()?;
        } else {
            loop {
                self.next_char()?;
                if !self.c.is_ascii_digit() {
                    break;
                }
                length = length * 10 + (self.c - b'0') as usize;
            }
        }

        if self.c == b':' {
            for _ in 0..length {
                self.next_char()?;
                self.push_char(string)?;
            }
        } else if mode != SexpMode::Advanced {
            Err("Encountered advanced string in canonical mode".into())
        } else {
            match self.c {
                b'"' => {
                    for _ in 0..length {
                        if !self.get_quoted_char()? {
                            return Err("Unexpected end of string".into());
                        }
                        self.push_char(string)?;
                    }
                    if self.get_quoted_char()? {
                        return Err("Quoted string longer than expected".into());
                    }
                },
                b'#' => {
                    self.start_coding(&Base16, b'#')?;
                    for _ in 0..length {
                        self.next_char()?;
                        self.push_char(string)?;
                    }
                    self.get_char()?;
                    if self.ctype != SexpCharType::End {
                        return Err("Coded string too long".into());
                    }
                    self.end_coding()?;
                },
                b'|' => {
                    self.start_coding(&Base64, b'|')?;
                    for _ in 0..length {
                        self.next_char()?;
                        self.push_char(string)?;
                    }
                    self.get_char()?;
                    if self.ctype != SexpCharType::End {
                        return Err("Coded string too long".into());
                    }
                    self.end_coding()?;
                },
                _ => return Err("Invalid string".into()),
            }
        }

        self.get_char()
    }

    fn get_comment(&mut self, string: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        string.clear();
        self.token = SexpToken::Comment;

        loop {
            self.push_char(string)?;
            self.get_raw_char()?;
            if self.ctype != SexpCharType::Normal || self.c == b'\n' {
                break;
            }
        }

        Ok(())
    }

    pub fn get_token(&mut self, mode: SexpMode, string: &mut Vec<u8>) -> Result<SexpToken, Box<dyn Error>> {
        loop {
            match self.ctype {
                SexpCharType::Eof => {
                    self.token = SexpToken::Eof;
                    return Ok(self.token);
                },
                SexpCharType::End => {
                    self.token = SexpToken::CodingEnd;
                    self.end_coding()?;
                    self.get_char()?;
                    return Ok(self.token);
                },
                SexpCharType::Normal => match self.c {
                    b'0'..=b'9' => {
                        self.get_string_length(mode, string)?;
                        return Ok(self.token);
                    },
                    b'(' => {
                        self.token = SexpToken::ListStart;
                        self.get_char()?;
                        return Ok(self.token);
                    },
                    b')' => {
                        self.token = SexpToken::ListEnd;
                        self.get_char()?;
                        return Ok(self.token);
                    },
                    b'[' => {
                        self.token = SexpToken::DisplayStart;
                        self.get_char()?;
                        return Ok(self.token);
                    },
                    b']' => {
                        self.token = SexpToken::DisplayEnd;
                        self.get_char()?;
                        return Ok(self.token);
                    },
                    b'{' => {
                        if mode == SexpMode::Canonical {
                            return Err("Unexpected transport data in canonical mode".into());
                        }
                        self.start_coding(&Base64, b'}')?;
                        self.get_char()?;
                        self.token = SexpToken::TransportStart;
                        return Ok(self.token);
                    },
                    b' ' | b'\t' | b'\n' | b'\r' => {
                        if mode == SexpMode::Canonical {
                            return Err("Whitespace encountered in canonical mode".into());
                        }
                        self.get_char()?;
                    },
                    b';' => {
                        if mode == SexpMode::Canonical {
                            return Err("Comment encountered in canonical mode".into());
                        }
                        self.get_comment(string)?;
                        return Ok(self.token);
                    },
                    _ => {
                        if mode != SexpMode::Advanced {
                            return Err("Encountered advanced string in canonical mode".into());
                        }
                        self.get_string(string)?;
                        return Ok(self.token);
                    },
                },
            }
        }
    }
}

fn is_token_char(c: u8) -> bool {
    match c {
        b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b'#' | b'|' | b';' | b' ' | b'\t' | b'\n' | b'\r' => false,
        _ => !c.is_ascii_control(),
    }
}