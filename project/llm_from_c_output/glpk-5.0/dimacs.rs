use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub struct DimacsError {
    message: String,
    line: usize,
    file: String,
}

impl fmt::Display for DimacsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}: error: {}",
            self.file, self.line, self.message
        )
    }
}

impl std::error::Error for DimacsError {}

#[derive(Debug)]
pub struct DimacsWarning {
    message: String,
    line: usize,
    file: String,
}

impl fmt::Display for DimacsWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}: warning: {}",
            self.file, self.line, self.message
        )
    }
}

pub struct DimacsReader {
    fname: String,
    reader: BufReader<File>,
    line_count: usize,
    current_char: Option<char>,
    field: String,
    empty_warned: bool,
    nonint_warned: bool,
}

impl DimacsReader {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(&path)?;
        let fname = path.as_ref().to_string_lossy().into_owned();
        let mut reader = BufReader::new(file);
        
        let mut first_char = None;
        let mut buf = [0; 1];
        if reader.read(&mut buf)? > 0 {
            first_char = Some(buf[0] as char);
        }
        
        Ok(Self {
            fname,
            reader,
            line_count: 1,
            current_char: first_char,
            field: String::with_capacity(255),
            empty_warned: false,
            nonint_warned: false,
        })
    }

    pub fn error(&self, message: &str) -> DimacsError {
        DimacsError {
            message: message.to_string(),
            line: self.line_count,
            file: self.fname.clone(),
        }
    }

    pub fn warning(&self, message: &str) -> DimacsWarning {
        DimacsWarning {
            message: message.to_string(),
            line: self.line_count,
            file: self.fname.clone(),
        }
    }

    pub fn read_char(&mut self) -> io::Result<()> {
        if self.current_char == Some('\n') {
            self.line_count += 1;
        }

        let mut buf = [0; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => {
                if self.current_char == Some('\n') {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        self.error("unexpected end of file").to_string(),
                    ));
                } else {
                    self.current_char = Some('\n');
                    return Ok(());
                }
            }
            Ok(_) => {
                let c = buf[0] as char;
                self.current_char = Some(match c {
                    '\n' => '\n',
                    c if c.is_ascii_whitespace() => ' ',
                    c if c.is_ascii_control() => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            self.error(&format!("invalid control character 0x{:02X}", c as u8))
                                .to_string(),
                        ));
                    }
                    _ => c,
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn read_designator(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if self.current_char != Some('\n') {
            return Err(Box::new(self.error("expected newline")));
        }

        self.read_char()?;
        loop {
            // Skip whitespace
            while self.current_char == Some(' ') {
                self.read_char()?;
            }

            match self.current_char {
                Some('\n') => {
                    if !self.empty_warned {
                        eprintln!("{}", self.warning("empty line ignored"));
                        self.empty_warned = true;
                    }
                    self.read_char()?;
                }
                Some('c') => {
                    // Skip comment line
                    while self.current_char != Some('\n') {
                        self.read_char()?;
                    }
                    self.read_char()?;
                }
                Some(c) => {
                    self.field.clear();
                    self.field.push(c);
                    self.read_char()?;
                    if self.current_char != Some(' ') && self.current_char != Some('\n') {
                        return Err(Box::new(self.error("line designator missing or invalid")));
                    }
                    break;
                }
                None => return Err(Box::new(self.error("unexpected end of file"))),
            }
        }

        Ok(self.field.clone())
    }

    pub fn read_field(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.field.clear();
        
        // Skip whitespace
        while self.current_char == Some(' ') {
            self.read_char()?;
        }

        if self.current_char == Some('\n') {
            return Err(Box::new(self.error("unexpected end of line")));
        }

        while let Some(c) = self.current_char {
            if c == ' ' || c == '\n' {
                break;
            }
            
            if self.field.len() >= 255 {
                return Err(Box::new(self.error(&format!(
                    "data field '{}...' too long",
                    &self.field[..15]
                ))));
            }
            
            self.field.push(c);
            self.read_char()?;
        }

        Ok(self.field.clone())
    }

    pub fn end_of_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while self.current_char == Some(' ') {
            self.read_char()?;
        }

        if self.current_char != Some('\n') {
            return Err(Box::new(self.error("too many data fields specified")));
        }

        Ok(())
    }

    pub fn check_int(&mut self, num: f64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.nonint_warned && num != num.floor() {
            eprintln!("{}", self.warning("non-integer data detected"));
            self.nonint_warned = true;
        }
        Ok(())
    }
}