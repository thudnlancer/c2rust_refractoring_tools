use std::ffi::{CString, NulError};
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_char;
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ArgcvError {
    MemoryAllocation,
    InvalidInput,
    Utf8Error(std::str::Utf8Error),
    NulError(NulError),
}

impl fmt::Display for ArgcvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ArgcvError::MemoryAllocation => write!(f, "Memory allocation failed"),
            ArgcvError::InvalidInput => write!(f, "Invalid input"),
            ArgcvError::Utf8Error(ref e) => e.fmt(f),
            ArgcvError::NulError(ref e) => e.fmt(f),
        }
    }
}

impl Error for ArgcvError {}

impl From<std::str::Utf8Error> for ArgcvError {
    fn from(err: std::str::Utf8Error) -> ArgcvError {
        ArgcvError::Utf8Error(err)
    }
}

impl From<NulError> for ArgcvError {
    fn from(err: NulError) -> ArgcvError {
        ArgcvError::NulError(err)
    }
}

const QUOTE_TRANSTAB: &[u8] = b"\\\\a\ab\bf\fn\nr\rt\t";

fn is_ws(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

fn is_delim(c: char, delim: &str) -> bool {
    delim.contains(c)
}

fn argcv_unquote_char(c: char) -> char {
    let mut p = QUOTE_TRANSTAB;
    while p.len() >= 2 {
        if p[0] as char == c {
            return p[1] as char;
        }
        p = &p[2..];
    }
    c
}

fn argcv_quote_char(c: char) -> Option<char> {
    let mut p = &QUOTE_TRANSTAB[QUOTE_TRANSTAB.len()-2..];
    while p.as_ptr() > QUOTE_TRANSTAB.as_ptr() {
        if p[1] as char == c {
            return Some(p[0] as char);
        }
        p = &p[..p.len()-2];
    }
    None
}

fn to_num(c: char) -> Option<u32> {
    if c.is_ascii_digit() {
        Some(c.to_digit(10).unwrap())
    } else if c.is_ascii_hexdigit() {
        Some(c.to_digit(16).unwrap())
    } else {
        None
    }
}

fn xtonum(src: &[u8], base: u32, cnt: usize) -> Option<(u32, usize)> {
    let mut val = 0;
    let mut i = 0;
    while i < cnt && i < src.len() {
        let c = src[i] as char;
        if let Some(n) = to_num(c) {
            if n >= base {
                break;
            }
            val = val * base + n;
            i += 1;
        } else {
            break;
        }
    }
    if i > 0 {
        Some((val, i))
    } else {
        None
    }
}

pub fn argcv_quoted_length(str: &str) -> (usize, bool) {
    let mut len = 0;
    let mut quote = false;
    for c in str.chars() {
        if c == ' ' {
            len += 1;
            quote = true;
        } else if c == '"' || c == '\'' {
            len += 2;
            quote = true;
        } else if c != '\t' && c != '\\' && c.is_ascii() && c.is_ascii_graphic() {
            len += 1;
        } else if argcv_quote_char(c).is_some() {
            len += 2;
        } else {
            len += 4;
        }
    }
    (len, quote)
}

pub fn argcv_unquote_copy(dst: &mut String, src: &str) {
    let mut chars = src.chars().peekable();
    let mut expect_delim = None;
    
    while let Some(c) = chars.next() {
        match c {
            '\'' | '"' => {
                if expect_delim.is_none() {
                    let delim = c;
                    let mut p = chars.clone();
                    while let Some(&next) = p.peek() {
                        if next == delim {
                            expect_delim = Some(delim);
                            break;
                        } else if next == '\\' {
                            p.next();
                        }
                        p.next();
                    }
                    if expect_delim.is_some() {
                        continue;
                    } else {
                        dst.push(c);
                    }
                } else if Some(c) == expect_delim {
                    expect_delim = None;
                } else {
                    dst.push(c);
                }
            }
            '\\' => {
                if let Some(&next) = chars.peek() {
                    match next {
                        'x' | 'X' => {
                            chars.next();
                            let hex_digits: Vec<_> = chars.by_ref().take(2).collect();
                            if hex_digits.len() == 2 {
                                let hex_str: String = hex_digits.iter().collect();
                                if let Ok(val) = u8::from_str_radix(&hex_str, 16) {
                                    dst.push(val as char);
                                    continue;
                                }
                            }
                            dst.push('\\');
                            dst.push(next);
                            dst.extend(hex_digits);
                        }
                        '0'..='7' => {
                            let oct_digits: Vec<_> = chars.by_ref().take(3).collect();
                            let oct_str: String = oct_digits.iter().collect();
                            if let Ok(val) = u8::from_str_radix(&oct_str, 8) {
                                dst.push(val as char);
                                continue;
                            }
                            dst.push('\\');
                            dst.push(next);
                            dst.extend(&oct_digits[1..]);
                        }
                        _ => {
                            dst.push(argcv_unquote_char(next));
                            chars.next();
                        }
                    }
                } else {
                    dst.push('\\');
                }
            }
            _ => dst.push(c),
        }
    }
}

pub fn argcv_quote_copy(dst: &mut String, src: &str) {
    for c in src.chars() {
        match c {
            '"' | '\'' => {
                dst.push('\\');
                dst.push(c);
            }
            c if c != '\t' && c != '\\' && c.is_ascii() && c.is_ascii_graphic() => {
                dst.push(c);
            }
            c => {
                if let Some(esc) = argcv_quote_char(c) {
                    dst.push('\\');
                    dst.push(esc);
                } else {
                    let val = c as u32;
                    dst.push_str(&format!("\\{:03o}", val));
                }
            }
        }
    }
}

pub fn argcv_get(command: &str, delim: &str, cmnt: Option<&str>) -> Result<Vec<String>, ArgcvError> {
    let len = command.len();
    let mut args = Vec::new();
    let mut save = 0;

    while save < len {
        let (start, end, new_save) = argcv_scan(command, delim, cmnt, save)?;
        save = new_save;
        
        if start > end {
            continue;
        }

        let mut arg = String::new();
        let arg_str = &command[start..=end];
        if (arg_str.starts_with('"') && arg_str.ends_with('"')) || 
           (arg_str.starts_with('\'') && arg_str.ends_with('\'')) {
            argcv_unquote_copy(&mut arg, &arg_str[1..arg_str.len()-1]);
        } else {
            argcv_unquote_copy(&mut arg, arg_str);
        }
        args.push(arg);
    }

    Ok(args)
}

fn argcv_scan(command: &str, delim: &str, cmnt: Option<&str>, save: usize) -> Result<(usize, usize, usize), ArgcvError> {
    let len = command.len();
    let mut i = save;
    let mut start;
    let mut end;

    loop {
        i = save;

        if i >= len {
            return Ok((0, 0, i + 1));
        }

        // Skip initial whitespace
        while i < len && is_ws(command.chars().nth(i).unwrap()) {
            i += 1;
        }
        start = i;

        if !is_delim(command.chars().nth(i).unwrap(), delim) {
            while i < len {
                let c = command.chars().nth(i).unwrap();
                if c == '\\' {
                    i += 1;
                    if i == len {
                        break;
                    }
                    i += 1;
                    continue;
                }
                
                if c == '\'' || c == '"' {
                    let mut j = i + 1;
                    while j < len && command.chars().nth(j).unwrap() != c {
                        if command.chars().nth(j).unwrap() == '\\' {
                            j += 1;
                        }
                        j += 1;
                    }
                    if j < len {
                        i = j + 1;
                    } else {
                        i += 1;
                    }
                } else if is_ws(c) || is_delim(c, delim) {
                    break;
                } else {
                    i += 1;
                }
            }
            if i > 0 {
                i -= 1;
            }
        }

        end = i;
        save = i + 1;

        // Handle comments
        if save <= len {
            if let Some(cmnt) = cmnt {
                if cmnt.contains(command.chars().nth(start).unwrap()) {
                    i = save;
                    while i < len && command.chars().nth(i).unwrap() != '\n' {
                        i += 1;
                    }
                    save = i;
                    continue;
                }
            }
        }
        break;
    }

    Ok((start, end, save))
}

pub fn argcv_string(args: &[String]) -> Result<String, ArgcvError> {
    let mut buffer = String::new();
    
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            buffer.push(' ');
        }
        
        let (_, needs_quote) = argcv_quoted_length(arg);
        if needs_quote {
            buffer.push('"');
            argcv_quote_copy(&mut buffer, arg);
            buffer.push('"');
        } else {
            argcv_quote_copy(&mut buffer, arg);
        }
    }

    while buffer.ends_with(' ') {
        buffer.pop();
    }

    Ok(buffer)
}