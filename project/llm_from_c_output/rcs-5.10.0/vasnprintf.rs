use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_ulong, c_ulonglong};
use std::ptr;
use std::mem;
use std::fmt;
use std::io::{self, Write};
use std::slice;
use std::str;
use std::num::{NonZeroUsize, ParseIntError};
use std::error::Error;
use std::cmp;
use std::ops::{Add, Mul, Sub};
use libc::{localeconv, size_t, va_list};
use std::cell::RefCell;

#[derive(Debug)]
pub enum PrintfError {
    InvalidFormat,
    InvalidArgument,
    OutOfMemory,
    IoError(io::Error),
    Other(i32),
}

impl fmt::Display for PrintfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrintfError::InvalidFormat => write!(f, "Invalid format string"),
            PrintfError::InvalidArgument => write!(f, "Invalid argument"),
            PrintfError::OutOfMemory => write!(f, "Out of memory"),
            PrintfError::IoError(e) => write!(f, "I/O error: {}", e),
            PrintfError::Other(code) => write!(f, "Error code {}", code),
        }
    }
}

impl Error for PrintfError {}

impl From<io::Error> for PrintfError {
    fn from(err: io::Error) -> Self {
        PrintfError::IoError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArgType {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    Double,
    LongDouble,
    String,
    Pointer,
    WideChar,
    WideString,
}

union ArgValue {
    char_val: c_char,
    short_val: c_int,
    int_val: c_int,
    long_val: c_long,
    longlong_val: c_longlong,
    uchar_val: c_char,
    ushort_val: c_int,
    uint_val: c_int,
    ulong_val: c_ulong,
    ulonglong_val: c_ulonglong,
    double_val: f64,
    longdouble_val: f64, // Note: Rust doesn't have long double, using f64 as approximation
    string_val: *const c_char,
    pointer_val: *const (),
    widechar_val: u32, // Assuming wchar_t is 32-bit
    widestring_val: *const u32,
}

struct Argument {
    arg_type: ArgType,
    value: ArgValue,
}

struct Directive {
    flags: u32,
    width: Option<usize>,
    precision: Option<usize>,
    conversion: char,
    arg_index: usize,
}

const FLAG_LEFT: u32 = 1 << 0;
const FLAG_SHOWSIGN: u32 = 1 << 1;
const FLAG_SPACE: u32 = 1 << 2;
const FLAG_ALT: u32 = 1 << 3;
const FLAG_ZERO: u32 = 1 << 4;

fn parse_format_string(format: &str) -> Result<Vec<Directive>, PrintfError> {
    let mut directives = Vec::new();
    let mut chars = format.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c != '%' {
            continue;
        }
        
        let mut directive = Directive {
            flags: 0,
            width: None,
            precision: None,
            conversion: '\0',
            arg_index: 0,
        };
        
        // Parse flags
        loop {
            match chars.peek() {
                Some('-') => {
                    directive.flags |= FLAG_LEFT;
                    chars.next();
                }
                Some('+') => {
                    directive.flags |= FLAG_SHOWSIGN;
                    chars.next();
                }
                Some(' ') => {
                    directive.flags |= FLAG_SPACE;
                    chars.next();
                }
                Some('#') => {
                    directive.flags |= FLAG_ALT;
                    chars.next();
                }
                Some('0') => {
                    directive.flags |= FLAG_ZERO;
                    chars.next();
                }
                _ => break,
            }
        }
        
        // Parse width
        if let Some(&c) = chars.peek() {
            if c == '*' {
                chars.next();
                directive.width = Some(usize::MAX); // Special value for *
            } else if c.is_ascii_digit() {
                let mut width = 0;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        width = width * 10 + c.to_digit(10).unwrap() as usize;
                        chars.next();
                    } else {
                        break;
                    }
                }
                directive.width = Some(width);
            }
        }
        
        // Parse precision
        if let Some(&'.') = chars.peek() {
            chars.next();
            if let Some(&c) = chars.peek() {
                if c == '*' {
                    chars.next();
                    directive.precision = Some(usize::MAX); // Special value for *
                } else if c.is_ascii_digit() {
                    let mut precision = 0;
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            precision = precision * 10 + c.to_digit(10).unwrap() as usize;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    directive.precision = Some(precision);
                } else {
                    directive.precision = Some(0);
                }
            } else {
                return Err(PrintfError::InvalidFormat);
            }
        }
        
        // Parse length modifier and conversion specifier
        if let Some(&c) = chars.peek() {
            match c {
                'h' => {
                    chars.next();
                    if let Some(&'h') = chars.peek() {
                        chars.next();
                        // hh modifier
                    }
                }
                'l' => {
                    chars.next();
                    if let Some(&'l') = chars.peek() {
                        chars.next();
                        // ll modifier
                    }
                }
                'L' => {
                    chars.next();
                    // L modifier
                }
                'j' | 'z' | 't' => {
                    chars.next();
                    // j, z, or t modifier
                }
                _ => {}
            }
            
            if let Some(&conv) = chars.peek() {
                directive.conversion = conv;
                chars.next();
                
                // TODO: Handle argument index parsing ($)
                directive.arg_index = directives.len();
                
                directives.push(directive);
            } else {
                return Err(PrintfError::InvalidFormat);
            }
        } else {
            return Err(PrintfError::InvalidFormat);
        }
    }
    
    Ok(directives)
}

fn format_directive(
    directive: &Directive,
    args: &[Argument],
    output: &mut Vec<u8>,
) -> Result<(), PrintfError> {
    if directive.arg_index >= args.len() {
        return Err(PrintfError::InvalidArgument);
    }
    
    let arg = &args[directive.arg_index];
    
    match directive.conversion {
        'd' | 'i' => format_integer(arg, 10, false, directive, output),
        'u' => format_integer(arg, 10, true, directive, output),
        'o' => format_integer(arg, 8, true, directive, output),
        'x' => format_integer(arg, 16, true, directive, output),
        'X' => format_integer(arg, 16, true, directive, output),
        'f' | 'F' => format_float(arg, directive, output),
        'e' | 'E' => format_scientific(arg, directive, output),
        'g' | 'G' => format_general(arg, directive, output),
        'c' => format_char(arg, directive, output),
        's' => format_string(arg, directive, output),
        'p' => format_pointer(arg, directive, output),
        'n' => Ok(()), // Not supported in safe Rust
        '%' => output.push(b'%'),
        _ => Err(PrintfError::InvalidFormat),
    }
}

fn format_integer(
    arg: &Argument,
    radix: u32,
    unsigned: bool,
    directive: &Directive,
    output: &mut Vec<u8>,
) -> Result<(), PrintfError> {
    let value = match (arg.arg_type, unsigned) {
        (ArgType::Char, false) => arg.value.char_val as i64,
        (ArgType::Short, false) => arg.value.short_val as i64,
        (ArgType::Int, false) => arg.value.int_val as i64,
        (ArgType::Long, false) => arg.value.long_val as i64,
        (ArgType::LongLong, false) => arg.value.longlong_val,
        (ArgType::UChar, true) => arg.value.uchar_val as u8 as i64,
        (ArgType::UShort, true) => arg.value.ushort_val as u16 as i64,
        (ArgType::UInt, true) => arg.value.uint_val as u32 as i64,
        (ArgType::ULong, true) => arg.value.ulong_val as u64 as i64,
        (ArgType::ULongLong, true) => arg.value.ulonglong_val as i64,
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    let is_negative = value < 0 && !unsigned;
    let abs_value = if is_negative { -value } else { value } as u64;
    
    let prefix = match (directive.conversion, directive.flags & FLAG_ALT != 0) {
        ('o', true) => Some("0"),
        ('x', true) => Some("0x"),
        ('X', true) => Some("0X"),
        _ => None,
    };
    
    let mut digits = Vec::new();
    if abs_value == 0 {
        digits.push(b'0');
    } else {
        let mut n = abs_value;
        while n > 0 {
            let digit = (n % radix as u64) as u8;
            digits.push(if digit < 10 {
                b'0' + digit
            } else if directive.conversion.is_ascii_uppercase() {
                b'A' + digit - 10
            } else {
                b'a' + digit - 10
            });
            n /= radix as u64;
        }
        digits.reverse();
    }
    
    // Apply precision
    if let Some(precision) = directive.precision {
        if digits.len() < precision {
            let zeros = precision - digits.len();
            digits.splice(0..0, vec![b'0'; zeros]);
        }
    }
    
    // Calculate total length
    let sign_len = if is_negative {
        1
    } else if directive.flags & FLAG_SHOWSIGN != 0 {
        1
    } else if directive.flags & FLAG_SPACE != 0 {
        1
    } else {
        0
    };
    
    let prefix_len = prefix.map(|s| s.len()).unwrap_or(0);
    let digits_len = digits.len();
    let total_len = sign_len + prefix_len + digits_len;
    
    // Apply width
    if let Some(width) = directive.width {
        if total_len < width {
            let padding = width - total_len;
            if directive.flags & FLAG_LEFT != 0 {
                // Left-justified: pad after
                write_padded(output, sign_len, prefix_len, &digits, padding, false)?;
            } else if directive.flags & FLAG_ZERO != 0 && directive.precision.is_none() {
                // Zero-padded: pad between sign/prefix and digits
                write_padded(output, sign_len, prefix_len, &digits, padding, true)?;
            } else {
                // Right-justified: pad before
                for _ in 0..padding {
                    output.push(b' ');
                }
                write_padded(output, sign_len, prefix_len, &digits, 0, false)?;
            }
            return Ok(());
        }
    }
    
    write_padded(output, sign_len, prefix_len, &digits, 0, false)
}

fn write_padded(
    output: &mut Vec<u8>,
    sign_len: usize,
    prefix_len: usize,
    digits: &[u8],
    padding: usize,
    zero_pad: bool,
) -> Result<(), PrintfError> {
    // Write sign
    if sign_len > 0 {
        output.push(b'-');
    } else if sign_len > 0 {
        output.push(if sign_len == 1 { b'+' } else { b' ' });
    }
    
    // Write prefix
    if prefix_len > 0 {
        let prefix = if prefix_len == 1 { "0" } else { "0x" };
        output.extend_from_slice(prefix.as_bytes());
    }
    
    // Write padding
    if zero_pad {
        for _ in 0..padding {
            output.push(b'0');
        }
    }
    
    // Write digits
    output.extend_from_slice(digits);
    
    // Write space padding if left-justified
    if !zero_pad && padding > 0 {
        for _ in 0..padding {
            output.push(b' ');
        }
    }
    
    Ok(())
}

fn format_float(arg: &Argument, directive: &Directive, output: &mut Vec<u8>) -> Result<(), PrintfError> {
    let value = match arg.arg_type {
        ArgType::Double => arg.value.double_val,
        ArgType::LongDouble => arg.value.longdouble_val,
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    let precision = directive.precision.unwrap_or(6);
    let mut s = format!("{0:.1$}", value, precision);
    
    if directive.conversion == 'F' {
        s = s.to_uppercase();
    }
    
    output.extend_from_slice(s.as_bytes());
    Ok(())
}

fn format_scientific(arg: &Argument, directive: &Directive, output: &mut Vec<u8>) -> Result<(), PrintfError> {
    let value = match arg.arg_type {
        ArgType::Double => arg.value.double_val,
        ArgType::LongDouble => arg.value.longdouble_val,
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    let precision = directive.precision.unwrap_or(6);
    let mut s = format!("{0:.1$e}", value, precision);
    
    if directive.conversion.is_ascii_uppercase() {
        s = s.to_uppercase();
    }
    
    output.extend_from_slice(s.as_bytes());
    Ok(())
}

fn format_general(arg: &Argument, directive: &Directive, output: &mut Vec<u8>) -> Result<(), PrintfError> {
    let value = match arg.arg_type {
        ArgType::Double => arg.value.double_val,
        ArgType::LongDouble => arg.value.longdouble_val,
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    let precision = directive.precision.unwrap_or(6);
    let mut s = if (value.abs().log10().floor() as i32) < -4 || (value.abs().log10().floor() as i32) >= precision as i32 {
        format!("{0:.1$e}", value, precision)
    } else {
        format!("{0:.1$}", value, precision)
    };
    
    if directive.conversion.is_ascii_uppercase() {
        s = s.to_uppercase();
    }
    
    output.extend_from_slice(s.as_bytes());
    Ok(())
}

fn format_char(arg: &Argument, directive: &Directive, output: &mut Vec<u8>) -> Result<(), PrintfError> {
    let c = match arg.arg_type {
        ArgType::Char => arg.value.char_val as u8,
        ArgType::WideChar => {
            // Simple ASCII conversion for wide chars
            let wc = arg.value.widechar_val;
            if wc <= 0x7F {
                wc as u8
            } else {
                b'?'
            }
        }
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    if let Some(width) = directive.width {
        if width > 1 {
            let padding = width - 1;
            if directive.flags & FLAG_LEFT == 0 {
                for _ in 0..padding {
                    output.push(b' ');
                }
            }
            output.push(c);
            if directive.flags & FLAG_LEFT != 0 {
                for _ in 0..padding {
                    output.push(b' ');
                }
            }
            return Ok(());
        }
    }
    
    output.push(c);
    Ok(())
}

fn format_string(arg: &Argument, directive: &Directive, output: &mut Vec<u8>) -> Result<(), PrintfError> {
    let s = match arg.arg_type {
        ArgType::String => unsafe {
            CStr::from_ptr(arg.value.string_val).to_str().map_err(|_| PrintfError::InvalidArgument)?
        },
        ArgType::WideString => {
            // Simple ASCII conversion for wide strings
            let mut s = String::new();
            let mut ptr = arg.value.widestring_val;
            unsafe {
                while *ptr != 0 {
                    let wc = *ptr;
                    if wc <= 0x7F {
                        s.push(wc as u8 as char);
                    } else {
                        s.push('?');
                    }
                    ptr = ptr.add(1);
                }
            }
            s
        }
        _ => return Err(PrintfError::InvalidArgument),
    };
    
    let truncated = if let Some(precision) = directive.precision {
        if precision < s.len() {
            &s[..precision]
        } else {
            s
        }
    } else {
        s
    };
    
    if let Some(width) = directive.width {
        if width > truncated.len() {
            let padding = width - truncated.len();
            if directive.flags & FLAG_LEFT == 0 {
                for _ in 0..padding {
