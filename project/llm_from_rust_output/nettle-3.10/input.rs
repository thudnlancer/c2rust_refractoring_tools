use std::io::{self, Read, Error, ErrorKind};
use std::mem;
use std::ptr;
use std::slice;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_uchar, c_void};
use std::cell::RefCell;
use std::rc::Rc;

type size_t = usize;
type uint8_t = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SexpMode {
    Canonical = 0,
    Advanced = 1,
    Transport = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SexpToken {
    String = 0,
    Display = 1,
    Comment = 2,
    ListStart = 3,
    ListEnd = 4,
    Eof = 5,
    DisplayStart = 6,
    DisplayEnd = 7,
    TransportStart = 8,
    CodingEnd = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum SexpCharType {
    Normal = 0,
    EofChar = 1,
    EndChar = 2,
}

struct NettleBuffer {
    contents: Vec<u8>,
    realloc_ctx: *mut c_void,
    realloc: Option<extern "C" fn(*mut c_void, *mut c_void, size_t) -> *mut c_void>,
}

impl NettleBuffer {
    fn new() -> Self {
        NettleBuffer {
            contents: Vec::new(),
            realloc_ctx: ptr::null_mut(),
            realloc: None,
        }
    }

    fn reset(&mut self) {
        self.contents.clear();
    }

    fn grow(&mut self, length: size_t) -> io::Result<()> {
        self.contents.reserve(length);
        Ok(())
    }

    fn push_char(&mut self, c: u8) -> io::Result<()> {
        self.contents.push(c);
        Ok(())
    }
}

struct Base16DecodeCtx {
    word: u8,
    bits: u8,
}

struct Base64DecodeCtx {
    table: *const i8,
    word: u16,
    bits: u8,
    padding: u8,
}

union DecodeState {
    base64: Base64DecodeCtx,
    hex: Base16DecodeCtx,
}

struct NettleArmor {
    name: &'static str,
    encode_context_size: u32,
    decode_context_size: u32,
    encode_final_length: u32,
    encode_init: Option<extern "C" fn(*mut c_void)>,
    encode_length: Option<extern "C" fn(size_t) -> size_t>,
    encode_update: Option<extern "C" fn(*mut c_void, *mut c_char, size_t, *const uint8_t) -> size_t>,
    encode_final: Option<extern "C" fn(*mut c_void, *mut c_char) -> size_t>,
    decode_init: Option<extern "C" fn(*mut c_void)>,
    decode_length: Option<extern "C" fn(size_t) -> size_t>,
    decode_update: Option<extern "C" fn(*mut c_void, *mut size_t, *mut uint8_t, size_t, *const c_char) -> c_int>,
    decode_final: Option<extern "C" fn(*mut c_void) -> c_int>,
}

static NETTLE_BASE16: NettleArmor = NettleArmor {
    name: "base16",
    encode_context_size: 0,
    decode_context_size: mem::size_of::<Base16DecodeCtx>() as u32,
    encode_final_length: 0,
    encode_init: None,
    encode_length: None,
    encode_update: None,
    encode_final: None,
    decode_init: None,
    decode_length: None,
    decode_update: None,
    decode_final: None,
};

static NETTLE_BASE64: NettleArmor = NettleArmor {
    name: "base64",
    encode_context_size: 0,
    decode_context_size: mem::size_of::<Base64DecodeCtx>() as u32,
    encode_final_length: 0,
    encode_init: None,
    encode_length: None,
    encode_update: None,
    encode_final: None,
    decode_init: None,
    decode_length: None,
    decode_update: None,
    decode_final: None,
};

struct SexpInput<R: Read> {
    reader: R,
    ctype: SexpCharType,
    c: u8,
    coding: Option<&'static NettleArmor>,
    state: DecodeState,
    terminator: u8,
    token: SexpToken,
}

impl<R: Read> SexpInput<R> {
    fn new(reader: R) -> Self {
        SexpInput {
            reader,
            ctype: SexpCharType::Normal,
            c: 0,
            coding: None,
            state: DecodeState {
                base64: Base64DecodeCtx {
                    table: ptr::null(),
                    word: 0,
                    bits: 0,
                    padding: 0,
                },
            },
            terminator: 0,
            token: SexpToken::Eof,
        }
    }

    fn init(&mut self) {
        self.coding = None;
    }

    fn get_raw_char(&mut self) -> io::Result<()> {
        let mut buf = [0u8; 1];
        match self.reader.read_exact(&mut buf) {
            Ok(_) => {
                self.ctype = SexpCharType::Normal;
                self.c = buf[0];
                Ok(())
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                self.ctype = SexpCharType::EofChar;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn get_char(&mut self) -> io::Result<()> {
        if let Some(coding) = self.coding {
            loop {
                self.get_raw_char()?;
                if self.ctype == SexpCharType::EofChar {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Unexpected end of file in coded data",
                    ));
                }
                if self.c == self.terminator {
                    self.ctype = SexpCharType::EndChar;
                    return Ok(());
                }

                let mut done = 1;
                let decode_update = coding.decode_update.unwrap();
                let result = unsafe {
                    decode_update(
                        &mut self.state as *mut _ as *mut c_void,
                        &mut done,
                        &mut self.c,
                        1,
                        &self.c as *const _ as *const c_char,
                    )
                };

                if result == 0 {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid coded data"));
                }
                if done != 0 {
                    return Ok(());
                }
            }
        } else {
            self.get_raw_char()
        }
    }

    fn next_char(&mut self) -> io::Result<u8> {
        self.get_char()?;
        if self.ctype != SexpCharType::Normal {
            Err(Error::new(ErrorKind::InvalidData, "Unexpected end of file"))
        } else {
            Ok(self.c)
        }
    }

    fn push_char(&mut self, buffer: &mut NettleBuffer) -> io::Result<()> {
        if self.ctype != SexpCharType::Normal {
            panic!("input->ctype == SEXP_NORMAL_CHAR");
        }
        buffer.push_char(self.c)
    }

    fn start_coding(
        &mut self,
        coding: &'static NettleArmor,
        terminator: u8,
    ) -> io::Result<()> {
        if self.coding.is_some() {
            panic!("!input->coding");
        }
        self.coding = Some(coding);
        if let Some(decode_init) = coding.decode_init {
            unsafe { decode_init(&mut self.state as *mut _ as *mut c_void) };
        }
        self.terminator = terminator;
        Ok(())
    }

    fn end_coding(&mut self) -> io::Result<()> {
        if self.coding.is_none() {
            panic!("input->coding");
        }
        let coding = self.coding.unwrap();
        if let Some(decode_final) = coding.decode_final {
            if unsafe { decode_final(&mut self.state as *mut _ as *mut c_void) } == 0 {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid coded data"));
            }
        }
        self.coding = None;
        Ok(())
    }

    fn get_quoted_char(&mut self) -> io::Result<bool> {
        let c = self.next_char()?;
        match c {
            b'"' => Ok(false),
            b'\\' => {
                let c = self.next_char()?;
                match c {
                    b'b' => {
                        self.c = b'\x08';
                        Ok(true)
                    }
                    b't' => {
                        self.c = b'\t';
                        Ok(true)
                    }
                    b'n' => {
                        self.c = b'\n';
                        Ok(true)
                    }
                    b'f' => {
                        self.c = b'\x0c';
                        Ok(true)
                    }
                    b'r' => {
                        self.c = b'\r';
                        Ok(true)
                    }
                    b'\\' => {
                        self.c = b'\\';
                        Ok(true)
                    }
                    b'o' | b'x' => {
                        panic!("Octal/hex escapes not implemented");
                    }
                    b'\n' => {
                        if self.next_char()? == b'\r' {
                            let _ = self.next_char()?;
                        }
                        Ok(false)
                    }
                    b'\r' => {
                        if self.next_char()? == b'\n' {
                            let _ = self.next_char()?;
                        }
                        Ok(false)
                    }
                    _ => Ok(true),
                }
            }
            _ => Ok(true),
        }
    }

    fn get_token_string(&mut self, buffer: &mut NettleBuffer) -> io::Result<()> {
        if self.coding.is_some() {
            panic!("!input->coding");
        }
        if self.ctype != SexpCharType::Normal {
            panic!("input->ctype == SEXP_NORMAL_CHAR");
        }

        // TODO: Implement sexp_token_chars check
        if !(self.c < 0x80 /* && sexp_token_chars[self.c as usize] != 0 */) {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid token"));
        }

        loop {
            self.push_char(buffer)?;
            self.get_char()?;
            if self.ctype != SexpCharType::Normal
                || !(self.c < 0x80 /* && sexp_token_chars[self.c as usize] != 0 */)
            {
                break;
            }
        }

        if buffer.contents.is_empty() {
            panic!("string->size");
        }

        Ok(())
    }

    fn get_string(&mut self, buffer: &mut NettleBuffer) -> io::Result<()> {
        buffer.reset();
        self.token = SexpToken::String;

        match self.c {
            b'"' => {
                while self.get_quoted_char()? {
                    self.push_char(buffer)?;
                }
                self.get_char()?;
            }
            b'#' => {
                self.start_coding(&NETTLE_BASE16, b'#')?;
                loop {
                    self.get_char()?;
                    match self.ctype {
                        SexpCharType::Normal => {
                            self.push_char(buffer)?;
                        }
                        SexpCharType::EofChar => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Unexpected end of file in coded string",
                            ));
                        }
                        SexpCharType::EndChar => {
                            self.end_coding()?;
                            self.get_char()?;
                            return Ok(());
                        }
                    }
                }
            }
            b'|' => {
                self.start_coding(&NETTLE_BASE64, b'|')?;
                loop {
                    self.get_char()?;
                    match self.ctype {
                        SexpCharType::Normal => {
                            self.push_char(buffer)?;
                        }
                        SexpCharType::EofChar => {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Unexpected end of file in coded string",
                            ));
                        }
                        SexpCharType::EndChar => {
                            self.end_coding()?;
                            self.get_char()?;
                            return Ok(());
                        }
                    }
                }
            }
            _ => {
                self.get_token_string(buffer)?;
            }
        }

        Ok(())
    }

    fn get_string_length(
        &mut self,
        mode: SexpMode,
        buffer: &mut NettleBuffer,
    ) -> io::Result<()> {
        buffer.reset();
        self.token = SexpToken::String;

        let mut length = (self.c - b'0') as u32;
        if length == 0 {
            self.next_char()?;
        } else {
            if length >= 10 {
                panic!("length < 10");
            }

            loop {
                let c = self.next_char()?;
                if !c.is_ascii_digit() {
                    break;
                }
                length = length * 10 + (c - b'0') as u32;
            }
        }

        if self.c == b':' {
            while length > 0 {
                self.next_char()?;
                self.push_char(buffer)?;
                length -= 1;
            }
        } else if mode != SexpMode::Advanced {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Encountered advanced string in canonical mode",
            ));
        } else {
            match self.c {
                b'"' => {
                    while length > 0 {
                        if !self.get_quoted_char()? {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Unexpected end of string",
                            ));
                        }
                        self.push_char(buffer)?;
                        length -= 1;
                    }
                    if self.get_quoted_char()? {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Quoted string longer than expected",
                        ));
                    }
                }
                b'#' => {
                    self.start_coding(&NETTLE_BASE16, b'#')?;
                    while length > 0 {
                        self.next_char()?;
                        self.push_char(buffer)?;
                        length -= 1;
                    }
                    self.get_char()?;
                    if self.ctype != SexpCharType::EndChar {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Coded string too long",
                        ));
                    }
                    self.end_coding()?;
                }
                b'|' => {
                    self.start_coding(&NETTLE_BASE64, b'|')?;
                    while length > 0 {
                        self.next_char()?;
                        self.push_char(buffer)?;
                        length -= 1;
                    }
                    self.get_char()?;
                    if self.ctype != SexpCharType::EndChar {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "Coded string too long",
                        ));
                    }
                    self.end_coding()?;
                }
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, "Invalid string"));
                }
            }
        }

        self.get_char()?;
        Ok(())
    }

    fn get_comment(&mut self, buffer: &mut NettleBuffer) -> io::Result<()> {
        buffer.reset();
        if self.ctype != SexpCharType::Normal {
            panic!("input->ctype == SEXP_NORMAL_CHAR");
        }
        if self.c != b';' {
            panic!("input->c == ';'");
        }

        loop {
            self.push_char(buffer)?;
            self.get_raw_char()?;
            if self.ctype != SexpCharType::Normal || self.c == b'\n' {
                break;
            }
        }

        self.token = SexpToken::Comment;
        Ok(())
    }

    fn get_token(
        &mut self,
        mode: SexpMode,
        buffer: &mut NettleBuffer,
    ) -> io::Result<SexpToken> {
        loop {
            match self.ctype {
                SexpCharType::EofChar => {
                    self.token = SexpToken::Eof;
                    return Ok(self.token);
                }
                SexpCharType::EndChar => {
                    self.token = SexpToken::CodingEnd;
                    self.end_coding()?;
                    self.get_char()?;
                    return Ok(self.token);
                }
                SexpCharType::Normal => {
                    match self.c {
                        b'0'..=b'9' => {
                            self.get_string_length(mode, buffer)?;
                            return Ok(self.token);
                        }
                        b'(' => {
                            self.token = SexpToken::ListStart;
                            self.get_char()?;
                            return Ok(self.token);
                        }
                        b')' => {
                            self.token = SexpToken::ListEnd;
                            self.get_char()?;
                            return Ok(self.token);
                        }
                        b'[' => {
                            self.token = SexpToken::DisplayStart;
                            self.get_char()?;
                            return Ok(self.token);
                        }
                        b']' => {
                            self.token = SexpToken::DisplayEnd;
                            self.get_char()?;
                            return Ok(self.token);
                        }
                        b'{' => {
                            if mode == SexpMode::Canonical {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "Unexpected transport data in canonical mode",
                                ));
                            }
                            self.start_coding(&NETTLE_BASE64, b'}')?;
                            self.get_char()?;
                            self.token = SexpToken::TransportStart;
                            return Ok(self.token);
                        }
                        b' ' | b'\t' | b'\n' | b'\r' => {
                            if mode == SexpMode::Canonical {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "Whitespace encountered in canonical mode",
