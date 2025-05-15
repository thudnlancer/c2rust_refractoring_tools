//! Decomposed printf argument list.
//! 
//! This module provides functionality for handling printf-style argument lists in a type-safe manner.

use std::os::raw::{c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;

#[cfg(feature = "wchar")]
use widestring::WideCString;

/// Argument types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    None,
    Schar,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    Longint,
    Ulongint,
    Longlongint,
    Ulonglongint,
    Double,
    LongDouble,
    Char,
    #[cfg(feature = "wint_t")]
    WideChar,
    String,
    #[cfg(feature = "wchar_t")]
    WideString,
    Pointer,
    CountScharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongintPointer,
    CountLonglongintPointer,
    #[cfg(feature = "unistdio")]
    U8String,
    #[cfg(feature = "unistdio")]
    U16String,
    #[cfg(feature = "unistdio")]
    U32String,
}

/// Polymorphic argument
#[derive(Debug)]
pub enum Argument {
    Schar(i8),
    Uchar(u8),
    Short(i16),
    Ushort(u16),
    Int(i32),
    Uint(u32),
    Longint(i64),
    Ulongint(u64),
    Longlongint(i64),
    Ulonglongint(u64),
    Double(f64),
    LongDouble(f64),
    Char(i32),
    #[cfg(feature = "wint_t")]
    WideChar(u32),
    String(String),
    #[cfg(feature = "wchar_t")]
    WideString(String),
    Pointer(*mut std::ffi::c_void),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongintPointer(*mut i64),
    CountLonglongintPointer(*mut i64),
    #[cfg(feature = "unistdio")]
    U8String(Vec<u8>),
    #[cfg(feature = "unistdio")]
    U16String(Vec<u16>),
    #[cfg(feature = "unistdio")]
    U32String(Vec<u32>),
}

impl Argument {
    pub fn get_type(&self) -> ArgType {
        match self {
            Argument::Schar(_) => ArgType::Schar,
            Argument::Uchar(_) => ArgType::Uchar,
            Argument::Short(_) => ArgType::Short,
            Argument::Ushort(_) => ArgType::Ushort,
            Argument::Int(_) => ArgType::Int,
            Argument::Uint(_) => ArgType::Uint,
            Argument::Longint(_) => ArgType::Longint,
            Argument::Ulongint(_) => ArgType::Ulongint,
            Argument::Longlongint(_) => ArgType::Longlongint,
            Argument::Ulonglongint(_) => ArgType::Ulonglongint,
            Argument::Double(_) => ArgType::Double,
            Argument::LongDouble(_) => ArgType::LongDouble,
            Argument::Char(_) => ArgType::Char,
            #[cfg(feature = "wint_t")]
            Argument::WideChar(_) => ArgType::WideChar,
            Argument::String(_) => ArgType::String,
            #[cfg(feature = "wchar_t")]
            Argument::WideString(_) => ArgType::WideString,
            Argument::Pointer(_) => ArgType::Pointer,
            Argument::CountScharPointer(_) => ArgType::CountScharPointer,
            Argument::CountShortPointer(_) => ArgType::CountShortPointer,
            Argument::CountIntPointer(_) => ArgType::CountIntPointer,
            Argument::CountLongintPointer(_) => ArgType::CountLongintPointer,
            Argument::CountLonglongintPointer(_) => ArgType::CountLonglongintPointer,
            #[cfg(feature = "unistdio")]
            Argument::U8String(_) => ArgType::U8String,
            #[cfg(feature = "unistdio")]
            Argument::U16String(_) => ArgType::U16String,
            #[cfg(feature = "unistdio")]
            Argument::U32String(_) => ArgType::U32String,
        }
    }
}

/// Number of directly allocated arguments (no heap allocation needed)
const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

/// Collection of arguments
pub struct Arguments {
    count: usize,
    args: Vec<Argument>,
    direct_args: [Option<Argument>; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    /// Create a new Arguments collection
    pub fn new() -> Self {
        Self {
            count: 0,
            args: Vec::new(),
            direct_args: Default::default(),
        }
    }

    /// Add an argument to the collection
    pub fn add_arg(&mut self, arg: Argument) {
        if self.count < N_DIRECT_ALLOC_ARGUMENTS {
            self.direct_args[self.count] = Some(arg);
        } else {
            self.args.push(arg);
        }
        self.count += 1;
    }

    /// Get an argument by index
    pub fn get_arg(&self, index: usize) -> Option<&Argument> {
        if index < N_DIRECT_ALLOC_ARGUMENTS {
            self.direct_args[index].as_ref()
        } else if index < self.count {
            self.args.get(index - N_DIRECT_ALLOC_ARGUMENTS)
        } else {
            None
        }
    }

    /// Get the number of arguments
    pub fn count(&self) -> usize {
        self.count
    }
}

/// Fetch arguments from a va_list-like structure
pub fn printf_fetchargs(args: &mut dyn Iterator<Item = Argument>, a: &mut Arguments) -> Result<(), String> {
    for arg in args {
        match arg {
            Argument::Schar(_)
            | Argument::Uchar(_)
            | Argument::Short(_)
            | Argument::Ushort(_)
            | Argument::Int(_)
            | Argument::Uint(_)
            | Argument::Longint(_)
            | Argument::Ulongint(_)
            | Argument::Longlongint(_)
            | Argument::Ulonglongint(_)
            | Argument::Double(_)
            | Argument::LongDouble(_)
            | Argument::Char(_)
            | Argument::Pointer(_)
            | Argument::CountScharPointer(_)
            | Argument::CountShortPointer(_)
            | Argument::CountIntPointer(_)
            | Argument::CountLongintPointer(_)
            | Argument::CountLonglongintPointer(_) => {
                a.add_arg(arg);
            }
            #[cfg(feature = "wint_t")]
            Argument::WideChar(_) => {
                a.add_arg(arg);
            }
            Argument::String(s) => {
                let s = if s.is_empty() { "(NULL)".to_string() } else { s };
                a.add_arg(Argument::String(s));
            }
            #[cfg(feature = "wchar_t")]
            Argument::WideString(s) => {
                let s = if s.is_empty() { "(NULL)".to_string() } else { s };
                a.add_arg(Argument::WideString(s));
            }
            #[cfg(feature = "unistdio")]
            Argument::U8String(v) => {
                let v = if v.is_empty() { b"(NULL)".to_vec() } else { v };
                a.add_arg(Argument::U8String(v));
            }
            #[cfg(feature = "unistdio")]
            Argument::U16String(v) => {
                let v = if v.is_empty() { "(NULL)".encode_utf16().collect() } else { v };
                a.add_arg(Argument::U16String(v));
            }
            #[cfg(feature = "unistdio")]
            Argument::U32String(v) => {
                let v = if v.is_empty() { "(NULL)".encode_utf32().collect() } else { v };
                a.add_arg(Argument::U32String(v));
            }
        }
    }
    Ok(())
}