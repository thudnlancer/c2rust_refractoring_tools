/* Decomposed printf argument list.
   Copyright (C) 1999, 2002-2003, 2006-2007, 2011-2021 Free Software
   Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, see <https://www.gnu.org/licenses/>.  */

use std::os::raw::{c_char, c_void, c_int, c_long, c_longlong, c_uchar, c_ulong, c_ulonglong};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;

#[cfg(feature = "wchar")]
use widestring::{WideStr, WideString};

#[cfg(feature = "unistdio")]
use std::os::raw::{c_uint8_t, c_uint16_t, c_uint32_t};

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
    #[cfg(feature = "wchar")]
    WideChar,
    String,
    #[cfg(feature = "wchar")]
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

#[derive(Debug, Clone)]
pub enum ArgumentValue {
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
    #[cfg(feature = "wchar")]
    WideChar(u32),
    String(CString),
    #[cfg(feature = "wchar")]
    WideString(WideString),
    Pointer(*mut c_void),
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

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
}

pub const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
    pub direct_alloc_args: [Option<Argument>; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    pub fn new(count: usize) -> Self {
        Arguments {
            count,
            args: Vec::with_capacity(count),
            direct_alloc_args: Default::default(),
        }
    }

    pub fn fetch_args(&mut self, va_list: &mut va_list::VaList) -> Result<(), i32> {
        for i in 0..self.count {
            let arg = if i < N_DIRECT_ALLOC_ARGUMENTS {
                &mut self.direct_alloc_args[i]
            } else {
                if self.args.len() <= i - N_DIRECT_ALLOC_ARGUMENTS {
                    self.args.resize(i - N_DIRECT_ALLOC_ARGUMENTS + 1, Argument {
                        type_: ArgType::None,
                        value: ArgumentValue::Int(0),
                    });
                }
                &mut self.args[i - N_DIRECT_ALLOC_ARGUMENTS]
            };

            match arg.type_ {
                ArgType::Schar => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Schar,
                        value: ArgumentValue::Schar(val as i8),
                    };
                }
                ArgType::Uchar => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Uchar,
                        value: ArgumentValue::Uchar(val as u8),
                    };
                }
                ArgType::Short => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Short,
                        value: ArgumentValue::Short(val as i16),
                    };
                }
                ArgType::Ushort => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Ushort,
                        value: ArgumentValue::Ushort(val as u16),
                    };
                }
                ArgType::Int => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Int,
                        value: ArgumentValue::Int(val),
                    };
                }
                ArgType::Uint => {
                    let val = va_list.arg::<u32>();
                    *arg = Argument {
                        type_: ArgType::Uint,
                        value: ArgumentValue::Uint(val),
                    };
                }
                ArgType::Longint => {
                    let val = va_list.arg::<i64>();
                    *arg = Argument {
                        type_: ArgType::Longint,
                        value: ArgumentValue::Longint(val),
                    };
                }
                ArgType::Ulongint => {
                    let val = va_list.arg::<u64>();
                    *arg = Argument {
                        type_: ArgType::Ulongint,
                        value: ArgumentValue::Ulongint(val),
                    };
                }
                ArgType::Longlongint => {
                    let val = va_list.arg::<i64>();
                    *arg = Argument {
                        type_: ArgType::Longlongint,
                        value: ArgumentValue::Longlongint(val),
                    };
                }
                ArgType::Ulonglongint => {
                    let val = va_list.arg::<u64>();
                    *arg = Argument {
                        type_: ArgType::Ulonglongint,
                        value: ArgumentValue::Ulonglongint(val),
                    };
                }
                ArgType::Double => {
                    let val = va_list.arg::<f64>();
                    *arg = Argument {
                        type_: ArgType::Double,
                        value: ArgumentValue::Double(val),
                    };
                }
                ArgType::LongDouble => {
                    let val = va_list.arg::<f64>();
                    *arg = Argument {
                        type_: ArgType::LongDouble,
                        value: ArgumentValue::LongDouble(val),
                    };
                }
                ArgType::Char => {
                    let val = va_list.arg::<i32>();
                    *arg = Argument {
                        type_: ArgType::Char,
                        value: ArgumentValue::Char(val),
                    };
                }
                #[cfg(feature = "wchar")]
                ArgType::WideChar => {
                    let val = if mem::size_of::<wchar_t>() < mem::size_of::<i32>() {
                        va_list.arg::<i32>() as wchar_t
                    } else {
                        va_list.arg::<wchar_t>()
                    };
                    *arg = Argument {
                        type_: ArgType::WideChar,
                        value: ArgumentValue::WideChar(val),
                    };
                }
                ArgType::String => {
                    let ptr = va_list.arg::<*const c_char>();
                    let c_str = if ptr.is_null() {
                        CString::new("(NULL)").unwrap()
                    } else {
                        unsafe { CStr::from_ptr(ptr) }.to_owned()
                    };
                    *arg = Argument {
                        type_: ArgType::String,
                        value: ArgumentValue::String(c_str),
                    };
                }
                #[cfg(feature = "wchar")]
                ArgType::WideString => {
                    let ptr = va_list.arg::<*const wchar_t>();
                    let wide_str = if ptr.is_null() {
                        WideString::from_str("(NULL)")
                    } else {
                        unsafe { WideStr::from_ptr(ptr) }.to_owned()
                    };
                    *arg = Argument {
                        type_: ArgType::WideString,
                        value: ArgumentValue::WideString(wide_str),
                    };
                }
                ArgType::Pointer => {
                    let val = va_list.arg::<*mut c_void>();
                    *arg = Argument {
                        type_: ArgType::Pointer,
                        value: ArgumentValue::Pointer(val),
                    };
                }
                ArgType::CountScharPointer => {
                    let val = va_list.arg::<*mut i8>();
                    *arg = Argument {
                        type_: ArgType::CountScharPointer,
                        value: ArgumentValue::CountScharPointer(val),
                    };
                }
                ArgType::CountShortPointer => {
                    let val = va_list.arg::<*mut i16>();
                    *arg = Argument {
                        type_: ArgType::CountShortPointer,
                        value: ArgumentValue::CountShortPointer(val),
                    };
                }
                ArgType::CountIntPointer => {
                    let val = va_list.arg::<*mut i32>();
                    *arg = Argument {
                        type_: ArgType::CountIntPointer,
                        value: ArgumentValue::CountIntPointer(val),
                    };
                }
                ArgType::CountLongintPointer => {
                    let val = va_list.arg::<*mut i64>();
                    *arg = Argument {
                        type_: ArgType::CountLongintPointer,
                        value: ArgumentValue::CountLongintPointer(val),
                    };
                }
                ArgType::CountLonglongintPointer => {
                    let val = va_list.arg::<*mut i64>();
                    *arg = Argument {
                        type_: ArgType::CountLonglongintPointer,
                        value: ArgumentValue::CountLonglongintPointer(val),
                    };
                }
                #[cfg(feature = "unistdio")]
                ArgType::U8String => {
                    let ptr = va_list.arg::<*const u8>();
                    let vec = if ptr.is_null() {
                        b"(NULL)".to_vec()
                    } else {
                        unsafe { std::slice::from_raw_parts(ptr, libc::strlen(ptr as *const i8)) }.to_vec()
                    };
                    *arg = Argument {
                        type_: ArgType::U8String,
                        value: ArgumentValue::U8String(vec),
                    };
                }
                #[cfg(feature = "unistdio")]
                ArgType::U16String => {
                    let ptr = va_list.arg::<*const u16>();
                    let vec = if ptr.is_null() {
                        "(NULL)".encode_utf16().collect()
                    } else {
                        unsafe { std::slice::from_raw_parts(ptr, libc::wcslen(ptr as *const wchar_t)) }.to_vec()
                    };
                    *arg = Argument {
                        type_: ArgType::U16String,
                        value: ArgumentValue::U16String(vec),
                    };
                }
                #[cfg(feature = "unistdio")]
                ArgType::U32String => {
                    let ptr = va_list.arg::<*const u32>();
                    let vec = if ptr.is_null() {
                        "(NULL)".encode_utf32().collect()
                    } else {
                        unsafe { std::slice::from_raw_parts(ptr, libc::wcslen(ptr as *const wchar_t)) }.to_vec()
                    };
                    *arg = Argument {
                        type_: ArgType::U32String,
                        value: ArgumentValue::U32String(vec),
                    };
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}