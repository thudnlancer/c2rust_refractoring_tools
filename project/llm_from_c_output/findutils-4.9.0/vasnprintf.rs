use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::string::String;
use std::vec::Vec;
use std::os::raw::{c_int, c_long, c_longlong, c_ulong, c_ulonglong};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::cmp;

#[derive(Debug)]
enum ArgType {
    Schar,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    LongInt,
    UlongInt,
    LongLongInt,
    UlongLongInt,
    Double,
    LongDouble,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountScharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
}

#[derive(Debug)]
struct Argument {
    type_: ArgType,
    value: ArgumentValue,
}

#[derive(Debug)]
enum ArgumentValue {
    Schar(i8),
    Uchar(u8),
    Short(i16),
    Ushort(u16),
    Int(i32),
    Uint(u32),
    LongInt(i64),
    UlongInt(u64),
    LongLongInt(i64),
    UlongLongInt(u64),
    Double(f64),
    LongDouble(f64),
    Char(char),
    WideChar(char),
    String(String),
    WideString(String),
    Pointer(*mut c_char),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongIntPointer(*mut i64),
    CountLongLongIntPointer(*mut i64),
}

#[derive(Debug)]
struct Directive {
    flags: u32,
    width: Option<usize>,
    precision: Option<usize>,
    conversion: char,
    arg_index: usize,
    width_arg_index: Option<usize>,
    precision_arg_index: Option<usize>,
}

const FLAG_LEFT: u32 = 1 << 0;
const FLAG_SHOWSIGN: u32 = 1 << 1;
const FLAG_SPACE: u32 = 1 << 2;
const FLAG_ALT: u32 = 1 << 3;
const FLAG_ZERO: u32 = 1 << 4;
const FLAG_GROUP: u32 = 1 << 5;

fn vasprintf(
    resultbuf: Option<&mut Vec<u8>>,
    format: &str,
    args: &[Argument],
) -> Result<Vec<u8>, io::Error> {
    let mut result = match resultbuf {
        Some(buf) => buf.clone(),
        None => Vec::new(),
    };

    let mut directives = Vec::new();
    parse_format_string(format, &mut directives)?;

    for directive in directives {
        match directive.conversion {
            '%' => {
                result.push(b'%');
            }
            'n' => {
                let count = result.len();
                match args[directive.arg_index].value {
                    ArgumentValue::CountScharPointer(ptr) => unsafe { *ptr = count as i8 },
                    ArgumentValue::CountShortPointer(ptr) => unsafe { *ptr = count as i16 },
                    ArgumentValue::CountIntPointer(ptr) => unsafe { *ptr = count as i32 },
                    ArgumentValue::CountLongIntPointer(ptr) => unsafe { *ptr = count as i64 },
                    ArgumentValue::CountLongLongIntPointer(ptr) => unsafe { *ptr = count as i64 },
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid type for %n")),
                }
            }
            _ => {
                let arg = &args[directive.arg_index];
                let mut s = format_arg(arg, &directive)?;
                if let Some(width) = directive.width {
                    let pad = width.saturating_sub(s.len());
                    if pad > 0 {
                        if directive.flags & FLAG_LEFT != 0 {
                            s.extend(vec![b' '; pad]);
                        } else if directive.flags & FLAG_ZERO != 0 {
                            let mut prefix = vec![b'0'; pad];
                            prefix.extend(s);
                            s = prefix;
                        } else {
                            let mut prefix = vec![b' '; pad];
                            prefix.extend(s);
                            s = prefix;
                        }
                    }
                }
                result.extend(s);
            }
        }
    }

    Ok(result)
}

fn parse_format_string(format: &str, directives: &mut Vec<Directive>) -> Result<(), io::Error> {
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut directive = Directive {
                flags: 0,
                width: None,
                precision: None,
                conversion: '\0',
                arg_index: 0,
                width_arg_index: None,
                precision_arg_index: None,
            };

            // Parse flags
            loop {
                match chars.peek() {
                    Some(&'-') => { directive.flags |= FLAG_LEFT; chars.next(); }
                    Some(&'+') => { directive.flags |= FLAG_SHOWSIGN; chars.next(); }
                    Some(&' ') => { directive.flags |= FLAG_SPACE; chars.next(); }
                    Some(&'#') => { directive.flags |= FLAG_ALT; chars.next(); }
                    Some(&'0') => { directive.flags |= FLAG_ZERO; chars.next(); }
                    Some(&'\'') => { directive.flags |= FLAG_GROUP; chars.next(); }
                    _ => break,
                }
            }

            // Parse width
            if let Some(&c) = chars.peek() {
                if c == '*' {
                    chars.next();
                    directive.width_arg_index = Some(0); // TODO: Handle arg index
                } else if c.is_digit(10) {
                    let mut width = 0;
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) {
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
                        directive.precision_arg_index = Some(0); // TODO: Handle arg index
                    } else if c.is_digit(10) {
                        let mut precision = 0;
                        while let Some(&c) = chars.peek() {
                            if c.is_digit(10) {
                                precision = precision * 10 + c.to_digit(10).unwrap() as usize;
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        directive.precision = Some(precision);
                    }
                }
            }

            // Parse length modifier
            let mut long_flag = false;
            let mut long_long_flag = false;
            if let Some(&c) = chars.peek() {
                match c {
                    'h' => { chars.next(); } // TODO: Handle short
                    'l' => {
                        chars.next();
                        if let Some(&'l') = chars.peek() {
                            chars.next();
                            long_long_flag = true;
                        } else {
                            long_flag = true;
                        }
                    }
                    'L' => { chars.next(); } // TODO: Handle long double
                    'j' | 'z' | 't' => { chars.next(); } // TODO: Handle intmax_t, size_t, ptrdiff_t
                    _ => {}
                }
            }

            // Parse conversion specifier
            if let Some(&c) = chars.next() {
                directive.conversion = c;
                directives.push(directive);
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Incomplete format specifier"));
            }
        } else {
            result.push(c as u8);
        }
    }

    Ok(())
}

fn format_arg(arg: &Argument, directive: &Directive) -> Result<Vec<u8>, io::Error> {
    match (&arg.value, directive.conversion) {
        (ArgumentValue::Schar(val), 'd' | 'i') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Uchar(val), 'u') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Short(val), 'd' | 'i') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Ushort(val), 'u') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Int(val), 'd' | 'i') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Uint(val), 'u') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::LongInt(val), 'd' | 'i') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::UlongInt(val), 'u') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::LongLongInt(val), 'd' | 'i') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::UlongLongInt(val), 'u') => Ok(format!("{}", val).into_bytes()),
        (ArgumentValue::Double(val), 'f' | 'F' | 'e' | 'E' | 'g' | 'G') => {
            let precision = directive.precision.unwrap_or(6);
            Ok(format!("{:.*}", precision, val).into_bytes())
        }
        (ArgumentValue::LongDouble(val), 'f' | 'F' | 'e' | 'E' | 'g' | 'G') => {
            let precision = directive.precision.unwrap_or(6);
            Ok(format!("{:.*}", precision, val).into_bytes())
        }
        (ArgumentValue::Char(val), 'c') => Ok(vec![*val as u8]),
        (ArgumentValue::WideChar(val), 'c') => Ok(val.to_string().into_bytes()),
        (ArgumentValue::String(val), 's') => Ok(val.as_bytes().to_vec()),
        (ArgumentValue::WideString(val), 's') => Ok(val.as_bytes().to_vec()),
        (ArgumentValue::Pointer(val), 'p') => Ok(format!("{:p}", val).into_bytes()),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid format specifier for argument type")),
    }
}