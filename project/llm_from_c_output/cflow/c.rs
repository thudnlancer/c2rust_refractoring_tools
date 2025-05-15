use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::string::String;
use std::vec::Vec;

struct Lexer {
    input: Box<dyn BufRead>,
    buffer: Vec<u8>,
    pos: usize,
    read_pos: usize,
    ch: u8,
    line_num: usize,
    filename: String,
    canonical_filename: String,
    hit_eof: bool,
    prev_token: Token,
    string_stk: Vec<u8>,
    debug: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Extern,
    Static,
    Typedef,
    Struct(String),
    Modifier(String),
    MemberOf(String),
    Op(String),
    Equals,
    String,
    Word(String),
    Identifier(String),
    Type(String),
    Qualifier(String),
    Lbrace0,
    Rbrace0,
    Char(char),
    Eof,
}

impl Lexer {
    fn new(input: Box<dyn BufRead>, filename: String) -> Self {
        let mut lexer = Lexer {
            input,
            buffer: Vec::new(),
            pos: 0,
            read_pos: 0,
            ch: 0,
            line_num: 1,
            filename: filename.clone(),
            canonical_filename: filename,
            hit_eof: false,
            prev_token: Token::Eof,
            string_stk: Vec::new(),
            debug: false,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.buffer.len() {
            self.buffer.clear();
            let bytes_read = self.input.read_until(b'\n', &mut self.buffer).unwrap();
            if bytes_read == 0 {
                self.ch = 0;
                return;
            }
            self.pos = 0;
            self.read_pos = 0;
        }
        self.ch = self.buffer[self.pos];
        self.pos += 1;
        self.read_pos += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            b'/' => {
                if self.peek_char() == b'/' {
                    self.skip_line_comment();
                    self.next_token()
                } else if self.peek_char() == b'*' {
                    self.skip_block_comment();
                    self.next_token()
                } else if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("/=".to_string())
                } else {
                    self.read_char();
                    Token::Op("/".to_string())
                }
            }
            b'%' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("%=".to_string())
                } else {
                    self.read_char();
                    Token::Op("%".to_string())
                }
            }
            b'+' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("+=".to_string())
                } else if self.peek_char() == b'+' {
                    self.read_char();
                    self.read_char();
                    Token::Op("++".to_string())
                } else {
                    self.read_char();
                    Token::Op("+".to_string())
                }
            }
            b'-' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("-=".to_string())
                } else if self.peek_char() == b'-' {
                    self.read_char();
                    self.read_char();
                    Token::Op("--".to_string())
                } else if self.peek_char() == b'>' {
                    self.read_char();
                    self.read_char();
                    Token::MemberOf("->".to_string())
                } else {
                    self.read_char();
                    Token::Op("-".to_string())
                }
            }
            b'<' => {
                if self.peek_char() == b'<' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        self.read_char();
                        Token::Op("<<=".to_string())
                    } else {
                        self.read_char();
                        Token::Op("<<".to_string())
                    }
                } else if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("<=".to_string())
                } else {
                    self.read_char();
                    Token::Op("<".to_string())
                }
            }
            b'>' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    if self.peek_char() == b'=' {
                        self.read_char();
                        self.read_char();
                        Token::Op(">>=".to_string())
                    } else {
                        self.read_char();
                        Token::Op(">>".to_string())
                    }
                } else if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op(">=".to_string())
                } else {
                    self.read_char();
                    Token::Op(">".to_string())
                }
            }
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    self.read_char();
                    Token::Op("&&".to_string())
                } else if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("&=".to_string())
                } else {
                    self.read_char();
                    Token::Op("&".to_string())
                }
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    self.read_char();
                    Token::Op("||".to_string())
                } else if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("|=".to_string())
                } else {
                    self.read_char();
                    Token::Op("|".to_string())
                }
            }
            b'^' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("^=".to_string())
                } else {
                    self.read_char();
                    Token::Op("^".to_string())
                }
            }
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("==".to_string())
                } else {
                    self.read_char();
                    Token::Equals
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("!=".to_string())
                } else {
                    self.read_char();
                    Token::Op("!".to_string())
                }
            }
            b'.' => {
                self.read_char();
                Token::MemberOf(".".to_string())
            }
            b'*' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    Token::Op("*=".to_string())
                } else {
                    self.read_char();
                    Token::Modifier("*".to_string())
                }
            }
            b'{' => {
                self.read_char();
                Token::Lbrace0
            }
            b'}' => {
                self.read_char();
                Token::Rbrace0
            }
            b'"' => {
                self.read_string()
            }
            b'\'' => {
                self.read_char_literal()
            }
            b'#' => {
                self.handle_preprocessor()
            }
            0 => {
                self.hit_eof = true;
                Token::Eof
            }
            _ => {
                if is_letter(self.ch) {
                    self.read_identifier()
                } else if is_digit(self.ch) {
                    self.read_number()
                } else {
                    let ch = self.ch;
                    self.read_char();
                    Token::Char(ch as char)
                }
            }
        };
        
        self.prev_token = token.clone();
        token
    }

    fn peek_char(&self) -> u8 {
        if self.read_pos >= self.buffer.len() {
            0
        } else {
            self.buffer[self.pos]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            if self.ch == b'\n' {
                self.line_num += 1;
            }
            self.read_char();
        }
    }

    fn skip_line_comment(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
        if self.ch == b'\n' {
            self.line_num += 1;
            self.read_char();
        }
    }

    fn skip_block_comment(&mut self) {
        self.read_char(); // skip *
        loop {
            self.read_char();
            if self.ch == b'*' && self.peek_char() == b'/' {
                self.read_char();
                self.read_char();
                break;
            }
            if self.ch == b'\n' {
                self.line_num += 1;
            }
            if self.ch == 0 {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut ident = Vec::new();
        while is_letter(self.ch) || is_digit(self.ch) {
            ident.push(self.ch);
            self.read_char();
        }
        let ident = String::from_utf8(ident).unwrap();

        match ident.as_str() {
            "extern" => Token::Extern,
            "static" => Token::Static,
            "typedef" => Token::Typedef,
            "struct" => Token::Struct("struct".to_string()),
            "union" => Token::Struct("union".to_string()),
            "enum" => Token::Struct("enum".to_string()),
            "char" | "double" | "float" | "int" | "void" => Token::Type(ident),
            "long" | "const" | "register" | "restrict" | "short" | "signed" | "unsigned" | "volatile" | "inline" => Token::Qualifier(ident),
            _ => {
                if self.prev_token != Token::Struct("".to_string()) {
                    if let Some(token_type) = KEYWORDS.get(&ident) {
                        return token_type.clone();
                    }
                }
                Token::Identifier(ident)
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut num = Vec::new();
        while is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        Token::Word(String::from_utf8(num).unwrap())
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // skip opening quote
        let mut s = Vec::new();
        while self.ch != b'"' && self.ch != 0 {
            if self.ch == b'\\' {
                self.read_char();
                s.push(self.read_escape());
            } else {
                s.push(self.ch);
                self.read_char();
            }
        }
        if self.ch == b'"' {
            self.read_char();
        }
        Token::String
    }

    fn read_char_literal(&mut self) -> Token {
        self.read_char(); // skip opening quote
        let ch = if self.ch == b'\\' {
            self.read_char();
            self.read_escape() as char
        } else {
            self.ch as char
        };
        self.read_char(); // skip closing quote
        Token::Char(ch)
    }

    fn read_escape(&mut self) -> u8 {
        match self.ch {
            b'a' => b'\x07',
            b'b' => b'\x08',
            b'f' => b'\x0c',
            b'n' => b'\n',
            b'r' => b'\r',
            b't' => b'\t',
            b'x' => {
                self.read_char();
                self.read_hex_digit()
            }
            b'0'..=b'7' => {
                self.read_octal_digit()
            }
            _ => self.ch,
        }
    }

    fn read_hex_digit(&mut self) -> u8 {
        let mut val = 0;
        for _ in 0..2 {
            if !is_hex_digit(self.ch) {
                break;
            }
            val = val * 16 + hex_value(self.ch);
            self.read_char();
        }
        val
    }

    fn read_octal_digit(&mut self) -> u8 {
        let mut val = 0;
        for _ in 0..3 {
            if !is_octal_digit(self.ch) {
                break;
            }
            val = val * 8 + (self.ch - b'0') as u8;
            self.read_char();
        }
        val
    }

    fn handle_preprocessor(&mut self) -> Token {
        let mut line = Vec::new();
        while self.ch != b'\n' && self.ch != 0 {
            line.push(self.ch);
            self.read_char();
        }
        if self.ch == b'\n' {
            self.line_num += 1;
            self.read_char();
        }
        
        let line = String::from_utf8(line).unwrap();
        if line.starts_with("line") {
            self.update_location(&line[4..]);
        }
        self.next_token()
    }

    fn update_location(&mut self, line: &str) {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 1 {
            if let Ok(num) = parts[0].parse::<usize>() {
                self.line_num = num;
            }
        }
        if parts.len() >= 2 && parts[1].starts_with('"') && parts[1].ends_with('"') {
            self.filename = parts[1][1..parts[1].len()-1].to_string();
        }
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_alphabetic() || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}

fn is_hex_digit(ch: u8) -> bool {
    ch.is_ascii_hexdigit()
}

fn is_octal_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'7'
}

fn hex_value(ch: u8) -> u8 {
    if ch.is_ascii_digit() {
        ch - b'0'
    } else {
        ch.to_ascii_lowercase() - b'a' + 10
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<String, Token> = {
        let mut m = HashMap::new();
        m.insert("break".to_string(), Token::Word("break".to_string()));
        m.insert("case".to_string(), Token::Word("case".to_string()));
        m.insert("continue".to_string(), Token::Word("continue".to_string()));
        m.insert("default".to_string(), Token::Word("default".to_string()));
        m.insert("do".to_string(), Token::Word("do".to_string()));
        m.insert("else".to_string(), Token::Word("else".to_string()));
        m.insert("for".to_string(), Token::Word("for".to_string()));
        m.insert("goto".to_string(), Token::Word("goto".to_string()));
        m.insert("if".to_string(), Token::Word("if".to_string()));
        m.insert("return".to_string(), Token::Word("return".to_string()));
        m.insert("sizeof".to_string(), Token::Word("sizeof".to_string()));
        m.insert("switch".to_string(), Token::Word("switch".to_string()));
        m.insert("while".to_string(), Token::Word("while".to_string()));
        m
    };
}

struct Preprocessor {
    command: String,
    options: Vec<String>,
}

impl Preprocessor {
    fn new(cmd: Option<String>) -> Self {
        Preprocessor {
            command: cmd.unwrap_or_else(|| "cpp".to_string()),
            options: Vec::new(),
        }
    }

    fn add_option(&mut self, opt: String) {
        self.options.push(opt);
    }

    fn preprocess(&self, filename: &str) -> io::Result<Box<dyn BufRead>> {
        let mut cmd = Command::new(&self.command);
        for opt in &self.options {
            cmd.arg(opt);
        }
        cmd.arg(filename);
        cmd.stdout(Stdio::piped());
        
        let child = cmd.spawn()?;
        Ok(Box::new(BufReader::new(child.stdout.unwrap())))
    }
}

fn source(filename: &str, preprocess: bool, pp: Option<&Preprocessor>) -> io::Result<Lexer> {
    let input: Box<dyn BufRead> = if preprocess {
        pp.unwrap().preprocess(filename)?
    } else {
        Box::new(BufReader::new(File::open(filename)?))
    };
    
    Ok(Lexer::new(input, filename.to_string()))
}