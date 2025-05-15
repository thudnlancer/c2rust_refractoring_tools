use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, Read, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum DmxError {
    IoError(io::Error),
    ParseError(String),
    InvalidCharacter(u8),
    UnexpectedEof,
    MissingFinalNewline,
    InvalidLineDesignator,
    FieldTooLong,
    TooManyFields,
    NonIntegerData,
}

pub struct DMX {
    fname: String,
    file: Box<dyn BufRead>,
    count: usize,
    current_char: Option<u8>,
    field: String,
    empty: bool,
    nonint: bool,
}

impl DMX {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, DmxError> {
        let fname = path.as_ref().to_string_lossy().into_owned();
        let file = File::open(path).map_err(DmxError::IoError)?;
        Ok(Self {
            fname,
            file: Box::new(io::BufReader::new(file)),
            count: 0,
            current_char: None,
            field: String::with_capacity(256),
            empty: false,
            nonint: false,
        })
    }

    pub fn error(&self, msg: &str) -> DmxError {
        DmxError::ParseError(format!("{}:{}: error: {}", self.fname, self.count, msg))
    }

    pub fn warning(&self, msg: &str) {
        eprintln!("{}:{}: warning: {}", self.fname, self.count, msg);
    }

    pub fn read_char(&mut self) -> Result<(), DmxError> {
        if let Some(b'\n') = self.current_char {
            self.count += 1;
        }

        let mut buf = [0u8; 1];
        match self.file.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0];
                if c != b'\n' {
                    if c.is_ascii_whitespace() {
                        self.current_char = Some(b' ');
                    } else if c.is_ascii_control() {
                        return Err(DmxError::InvalidCharacter(c));
                    } else {
                        self.current_char = Some(c);
                    }
                } else {
                    self.current_char = Some(c);
                }
                Ok(())
            }
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                if self.current_char == Some(b'\n') {
                    Err(DmxError::UnexpectedEof)
                } else {
                    self.warning("missing final end of line");
                    self.current_char = Some(b'\n');
                    Ok(())
                }
            }
            Err(e) => Err(DmxError::IoError(e)),
        }
    }

    pub fn read_designator(&mut self) -> Result<(), DmxError> {
        assert!(self.current_char == Some(b'\n'));
        self.read_char()?;

        loop {
            while self.current_char == Some(b' ') {
                self.read_char()?;
            }

            match self.current_char {
                Some(b'\n') => {
                    if !self.empty {
                        self.warning("empty line ignored");
                        self.empty = true;
                    }
                    self.read_char()?;
                }
                Some(b'c') => {
                    while self.current_char != Some(b'\n') {
                        self.read_char()?;
                    }
                    self.read_char()?;
                }
                _ => {
                    if let Some(c) = self.current_char {
                        self.field.clear();
                        self.field.push(c as char);
                        self.read_char()?;
                        if self.current_char != Some(b' ') && self.current_char != Some(b'\n') {
                            return Err(self.error("line designator missing or invalid"));
                        }
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn read_field(&mut self) -> Result<(), DmxError> {
        while self.current_char == Some(b' ') {
            self.read_char()?;
        }

        if self.current_char == Some(b'\n') {
            return Err(self.error("unexpected end of line"));
        }

        self.field.clear();
        while self.current_char != Some(b' ') && self.current_char != Some(b'\n') {
            if self.field.len() >= 255 {
                return Err(self.error(&format!("data field '{}...' too long", &self.field[..15])));
            }
            if let Some(c) = self.current_char {
                self.field.push(c as char);
                self.read_char()?;
            }
        }
        Ok(())
    }

    pub fn end_of_line(&mut self) -> Result<(), DmxError> {
        while self.current_char == Some(b' ') {
            self.read_char()?;
        }

        if self.current_char != Some(b'\n') {
            Err(self.error("too many data fields specified"))
        } else {
            Ok(())
        }
    }

    pub fn check_int(&mut self, num: f64) {
        if !self.nonint && num != num.floor() {
            self.warning("non-integer data detected");
            self.nonint = true;
        }
    }

    pub fn get_field(&self) -> &str {
        &self.field
    }
}