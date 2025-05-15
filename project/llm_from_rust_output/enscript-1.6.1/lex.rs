use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum NodeType {
    Void = 0,
    String = 1,
    Regexp = 2,
    Integer = 3,
    Real = 4,
    Symbol = 5,
    Array = 6,
}

#[derive(Debug)]
pub struct Node {
    pub type_: NodeType,
    pub refcount: c_uint,
    pub linenum: c_uint,
    pub data: NodeData,
}

#[derive(Debug)]
pub enum NodeData {
    String { data: CString, len: c_uint },
    Regexp { data: CString, len: c_uint, flags: c_uint },
    Integer(i32),
    Real(f64),
    Symbol(CString),
    Array { elements: Vec<*mut Node>, len: c_uint, allocated: c_uint },
}

pub struct Lexer {
    buffer: Vec<u8>,
    pos: usize,
    linenum: c_uint,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            buffer: input.as_bytes().to_vec(),
            pos: 0,
            linenum: 1,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos >= self.buffer.len() {
            None
        } else {
            let c = self.buffer[self.pos] as char;
            self.pos += 1;
            if c == '\n' {
                self.linenum += 1;
            }
            Some(c)
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos >= self.buffer.len() {
            None
        } else {
            Some(self.buffer[self.pos] as char)
        }
    }

    fn eat_comment(&mut self) {
        while let Some(c) = self.next_char() {
            if c == '*' {
                if let Some('/') = self.peek_char() {
                    self.next_char();
                    return;
                }
            }
        }
        // Handle EOF in comment error
    }

    fn read_string(&mut self) -> Result<CString, &'static str> {
        let mut buf = Vec::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => break,
                '\\' => {
                    if let Some(esc) = self.next_char() {
                        let decoded = match esc {
                            'n' => '\n',
                            't' => '\t',
                            'v' => '\x0B',
                            'b' => '\x08',
                            'r' => '\r',
                            'f' => '\x0C',
                            'a' => '\x07',
                            '0'..='7' => {
                                // Handle octal escape
                                let mut val = esc.to_digit(8).unwrap();
                                for _ in 0..2 {
                                    if let Some(d) = self.peek_char().and_then(|c| c.to_digit(8)) {
                                        self.next_char();
                                        val = val * 8 + d;
                                    } else {
                                        break;
                                    }
                                }
                                char::from_u32(val).unwrap_or('?')
                            }
                            _ => esc,
                        };
                        buf.push(decoded as u8);
                    }
                }
                _ => buf.push(c as u8),
            }
        }
        CString::new(buf).map_err(|_| "Invalid string data")
    }

    fn read_regexp(&mut self, node: &mut Node) -> Result<(), &'static str> {
        let mut buf = Vec::new();
        while let Some(c) = self.next_char() {
            match c {
                '/' => break,
                '\\' => {
                    if let Some(esc) = self.next_char() {
                        let decoded = match esc {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            'f' => '\x0C',
                            '/' | '\\' => esc,
                            _ => {
                                // Handle other escapes
                                esc
                            }
                        };
                        buf.push(decoded as u8);
                    }
                }
                _ => buf.push(c as u8),
            }
        }

        // Read regex flags
        let mut flags = 0;
        while let Some(c) = self.peek_char() {
            match c {
                'i' => {
                    flags |= 1;
                    self.next_char();
                }
                _ => break,
            }
        }

        if let NodeData::Regexp { data, len, flags: regex_flags } = &mut node.data {
            *data = CString::new(buf).map_err(|_| "Invalid regex data")?;
            *len = buf.len() as c_uint;
            *regex_flags = flags;
            Ok(())
        } else {
            Err("Node is not a regexp type")
        }
    }

    pub fn lex(&mut self) -> Result<Token, &'static str> {
        while let Some(c) = self.peek_char() {
            match c {
                '/' => {
                    self.next_char();
                    if let Some('*') = self.peek_char() {
                        self.next_char();
                        self.eat_comment();
                        continue;
                    } else {
                        return Ok(Token::Slash);
                    }
                }
                '"' => {
                    self.next_char();
                    let string = self.read_string()?;
                    return Ok(Token::String(string));
                }
                '0'..='9' => {
                    let num_str = self.read_number()?;
                    if num_str.contains('.') {
                        let num = num_str.parse::<f64>().map_err(|_| "Invalid real number")?;
                        return Ok(Token::Real(num));
                    } else {
                        let num = num_str.parse::<i32>().map_err(|_| "Invalid integer")?;
                        return Ok(Token::Integer(num));
                    }
                }
                _ if c.is_whitespace() => {
                    self.next_char();
                    continue;
                }
                _ => {
                    // Handle other tokens
                    self.next_char();
                    return match c {
                        '+' => Ok(Token::Plus),
                        '-' => Ok(Token::Minus),
                        '*' => Ok(Token::Star),
                        // ... other single-character tokens
                        _ => {
                            if c.is_alphabetic() || c == '_' {
                                let ident = self.read_identifier(c);
                                Ok(Token::Identifier(ident))
                            } else {
                                Err("Unknown token")
                            }
                        }
                    };
                }
            }
        }
        Ok(Token::Eof)
    }

    fn read_number(&mut self) -> Result<String, &'static str> {
        let mut num = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_digit(10) || c == '.' {
                num.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Ok(num)
    }

    fn read_identifier(&mut self, first: char) -> String {
        let mut ident = String::new();
        ident.push(first);
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        ident
    }
}

#[derive(Debug)]
pub enum Token {
    Integer(i32),
    Real(f64),
    String(CString),
    Regexp(CString, c_uint),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    // ... other token variants
    Eof,
}

fn node_alloc(type_: NodeType) -> Node {
    Node {
        type_,
        refcount: 1,
        linenum: 1,
        data: match type_ {
            NodeType::String => NodeData::String { data: CString::new("").unwrap(), len: 0 },
            NodeType::Regexp => NodeData::Regexp { data: CString::new("").unwrap(), len: 0, flags: 0 },
            NodeType::Integer => NodeData::Integer(0),
            NodeType::Real => NodeData::Real(0.0),
            NodeType::Symbol => NodeData::Symbol(CString::new("").unwrap()),
            NodeType::Array => NodeData::Array { elements: Vec::new(), len: 0, allocated: 0 },
            NodeType::Void => NodeData::Integer(0), // Placeholder
        },
    }
}