use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::string::String;
use std::vec::Vec;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::num::{NonZeroUsize, Wrapping};
use std::cmp;
use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Shl, Shr};
use std::convert::{TryFrom, TryInto};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::iter;
use std::marker::PhantomData;
use std::mem::{size_of, align_of};
use std::ops::{Deref, DerefMut};
use std::ptr::{null, null_mut};
use std::slice::{from_raw_parts, from_raw_parts_mut};
use std::str::{from_utf8, from_utf8_unchecked};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct VasprintfError {
    kind: VasprintfErrorKind,
}

#[derive(Debug)]
enum VasprintfErrorKind {
    InvalidFormat,
    InvalidArgument,
    OutOfMemory,
    IoError(io::Error),
    EncodingError,
    Overflow,
}

impl Display for VasprintfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            VasprintfErrorKind::InvalidFormat => write!(f, "invalid format string"),
            VasprintfErrorKind::InvalidArgument => write!(f, "invalid argument"),
            VasprintfErrorKind::OutOfMemory => write!(f, "out of memory"),
            VasprintfErrorKind::IoError(e) => write!(f, "I/O error: {}", e),
            VasprintfErrorKind::EncodingError => write!(f, "encoding error"),
            VasprintfErrorKind::Overflow => write!(f, "overflow"),
        }
    }
}

impl Error for VasprintfError {}

type Result<T> = std::result::Result<T, VasprintfError>;

struct FormatDirective {
    flags: u32,
    width: Option<usize>,
    precision: Option<usize>,
    conversion: char,
    arg_index: usize,
}

struct FormatParser<'a> {
    s: &'a str,
    pos: usize,
    args_count: usize,
}

impl<'a> FormatParser<'a> {
    fn new(s: &'a str) -> Self {
        FormatParser {
            s,
            pos: 0,
            args_count: 0,
        }
    }

    fn parse(&mut self) -> Result<Vec<FormatDirective>> {
        let mut directives = Vec::new();
        
        while let Some(c) = self.peek() {
            if c != '%' {
                self.advance();
                continue;
            }
            
            self.advance(); // skip '%'
            
            if self.peek() == Some('%') {
                self.advance();
                continue;
            }
            
            let directive = self.parse_directive()?;
            directives.push(directive);
        }
        
        Ok(directives)
    }
    
    fn parse_directive(&mut self) -> Result<FormatDirective> {
        let flags = self.parse_flags();
        let width = self.parse_width()?;
        let precision = self.parse_precision()?;
        let conversion = self.parse_conversion()?;
        
        let arg_index = self.args_count;
        self.args_count += 1;
        
        Ok(FormatDirective {
            flags,
            width,
            precision,
            conversion,
            arg_index,
        })
    }
    
    fn parse_flags(&mut self) -> u32 {
        let mut flags = 0;
        
        loop {
            match self.peek() {
                Some('#') => flags |= FLAG_ALT,
                Some('0') => flags |= FLAG_ZERO,
                Some('-') => flags |= FLAG_LEFT,
                Some(' ') => flags |= FLAG_SPACE,
                Some('+') => flags |= FLAG_SHOWSIGN,
                Some('\'') => flags |= FLAG_GROUP,
                _ => break,
            }
            self.advance();
        }
        
        flags
    }
    
    fn parse_width(&mut self) -> Result<Option<usize>> {
        if self.peek() == Some('*') {
            self.advance();
            Ok(None)
        } else {
            let mut width = 0;
            let mut has_width = false;
            
            while let Some(c) = self.peek() {
                if !c.is_ascii_digit() {
                    break;
                }
                
                width = width * 10 + (c as usize - '0' as usize);
                has_width = true;
                self.advance();
            }
            
            Ok(if has_width { Some(width) } else { None })
        }
    }
    
    fn parse_precision(&mut self) -> Result<Option<usize>> {
        if self.peek() != Some('.') {
            return Ok(None);
        }
        
        self.advance();
        
        if self.peek() == Some('*') {
            self.advance();
            Ok(None)
        } else {
            let mut precision = 0;
            let mut has_precision = false;
            
            while let Some(c) = self.peek() {
                if !c.is_ascii_digit() {
                    break;
                }
                
                precision = precision * 10 + (c as usize - '0' as usize);
                has_precision = true;
                self.advance();
            }
            
            Ok(if has_precision { Some(precision) } else { None })
        }
    }
    
    fn parse_conversion(&mut self) -> Result<char> {
        self.peek()
            .ok_or(VasprintfError { kind: VasprintfErrorKind::InvalidFormat })
            .and_then(|c| {
                self.advance();
                Ok(c)
            })
    }
    
    fn peek(&self) -> Option<char> {
        self.s[self.pos..].chars().next()
    }
    
    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }
}

const FLAG_ALT: u32 = 1 << 0;
const FLAG_ZERO: u32 = 1 << 1;
const FLAG_LEFT: u32 = 1 << 2;
const FLAG_SPACE: u32 = 1 << 3;
const FLAG_SHOWSIGN: u32 = 1 << 4;
const FLAG_GROUP: u32 = 1 << 5;

fn vasprintf(
    resultbuf: Option<&mut [u8]>,
    format: &str,
    args: &[&dyn fmt::Display],
) -> Result<Vec<u8>> {
    let mut parser = FormatParser::new(format);
    let directives = parser.parse()?;
    
    let mut output = Vec::new();
    let mut arg_iter = args.iter();
    
    let mut pos = 0;
    for directive in directives {
        // Copy literal text before directive
        if pos < parser.pos {
            output.extend_from_slice(&format.as_bytes()[pos..parser.pos]);
            pos = parser.pos;
        }
        
        // Handle directive
        let arg = arg_iter.next().ok_or(VasprintfError {
            kind: VasprintfErrorKind::InvalidArgument,
        })?;
        
        let mut buffer = Vec::new();
        write!(buffer, "{}", arg).map_err(|e| VasprintfError {
            kind: VasprintfErrorKind::IoError(e),
        })?;
        
        // Apply width and precision
        let formatted = apply_formatting(&buffer, &directive)?;
        output.extend_from_slice(&formatted);
    }
    
    // Copy remaining literal text
    if pos < format.len() {
        output.extend_from_slice(&format.as_bytes()[pos..]);
    }
    
    Ok(output)
}

fn apply_formatting(
    s: &[u8],
    directive: &FormatDirective,
) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    
    // Apply precision
    let s = if let Some(precision) = directive.precision {
        if precision < s.len() {
            &s[..precision]
        } else {
            s
        }
    } else {
        s
    };
    
    // Apply width
    if let Some(width) = directive.width {
        if s.len() < width {
            let padding = width - s.len();
            
            if directive.flags & FLAG_LEFT != 0 {
                result.extend_from_slice(s);
                result.extend(std::iter::repeat(b' ').take(padding));
            } else {
                let pad_char = if directive.flags & FLAG_ZERO != 0 {
                    b'0'
                } else {
                    b' '
                };
                
                result.extend(std::iter::repeat(pad_char).take(padding));
                result.extend_from_slice(s);
            }
        } else {
            result.extend_from_slice(s);
        }
    } else {
        result.extend_from_slice(s);
    }
    
    Ok(result)
}

struct VaList {
    args: Vec<Box<dyn fmt::Display>>,
}

impl VaList {
    fn new() -> Self {
        VaList { args: Vec::new() }
    }
    
    fn add<T: fmt::Display + 'static>(&mut self, arg: T) {
        self.args.push(Box::new(arg));
    }
    
    fn as_slice(&self) -> &[&dyn fmt::Display] {
        unsafe {
            std::mem::transmute::<&[Box<dyn fmt::Display>], &[&dyn fmt::Display]>(
                &self.args[..],
            )
        }
    }
}

fn vasnprintf(
    resultbuf: Option<&mut [u8]>,
    length: &mut usize,
    format: &str,
    args: &VaList,
) -> Result<Vec<u8>> {
    let result = vasprintf(resultbuf, format, args.as_slice())?;
    *length = result.len();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vasprintf() {
        let mut va = VaList::new();
        va.add(42);
        va.add("test");
        
        let mut len = 0;
        let result = vasnprintf(None, &mut len, "Number: %d, String: %s", &va).unwrap();
        assert_eq!(result, b"Number: 42, String: test");
        assert_eq!(len, result.len());
    }
}