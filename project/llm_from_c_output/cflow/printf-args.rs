/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Decomposed printf argument list.
   Copyright (C) 1999, 2002-2003, 2006-2007 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation,
   Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.  */

use std::ffi::{c_char, c_void, CStr, CString};
use std::os::raw::{c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
use std::ptr;

#[cfg(feature = "wchar")]
use widestring::WideCStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    None,
    Schar,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    LongInt,
    UlongInt,
    #[cfg(feature = "long_long")]
    LongLongInt,
    #[cfg(feature = "long_long")]
    UlongLongInt,
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
    CountLongIntPointer,
    #[cfg(feature = "long_long")]
    CountLongLongIntPointer,
    #[cfg(feature = "unistdio")]
    U8String,
    #[cfg(feature = "unistdio")]
    U16String,
    #[cfg(feature = "unistdio")]
    U32String,
}

#[derive(Debug)]
pub enum ArgumentValue {
    Schar(i8),
    Uchar(u8),
    Short(i16),
    Ushort(u16),
    Int(i32),
    Uint(u32),
    LongInt(i64),
    UlongInt(u64),
    #[cfg(feature = "long_long")]
    LongLongInt(i64),
    #[cfg(feature = "long_long")]
    UlongLongInt(u64),
    Double(f64),
    LongDouble(f64),
    Char(i32),
    #[cfg(feature = "wchar")]
    WideChar(u32),
    String(*const c_char),
    #[cfg(feature = "wchar")]
    WideString(*const u16),
    Pointer(*mut c_void),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongIntPointer(*mut i64),
    #[cfg(feature = "long_long")]
    CountLongLongIntPointer(*mut i64),
    #[cfg(feature = "unistdio")]
    U8String(*const u8),
    #[cfg(feature = "unistdio")]
    U16String(*const u16),
    #[cfg(feature = "unistdio")]
    U32String(*const u32),
}

#[derive(Debug)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
}

#[derive(Debug)]
pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(count: usize) -> Self {
        Arguments {
            count,
            args: Vec::with_capacity(count),
        }
    }

    pub fn fetch_args(&mut self, args: &mut std::ffi::VaList) -> Result<(), i32> {
        for i in 0..self.count {
            let arg = if i < self.args.len() {
                &mut self.args[i]
            } else {
                self.args.push(Argument {
                    type_: ArgType::None,
                    value: ArgumentValue::Int(0),
                });
                &mut self.args[i]
            };

            match arg.type_ {
                ArgType::Schar => {
                    arg.value = ArgumentValue::Schar(unsafe { args.arg::<i32>() } as i8);
                }
                ArgType::Uchar => {
                    arg.value = ArgumentValue::Uchar(unsafe { args.arg::<i32>() } as u8);
                }
                ArgType::Short => {
                    arg.value = ArgumentValue::Short(unsafe { args.arg::<i32>() } as i16);
                }
                ArgType::Ushort => {
                    arg.value = ArgumentValue::Ushort(unsafe { args.arg::<i32>() } as u16);
                }
                ArgType::Int => {
                    arg.value = ArgumentValue::Int(unsafe { args.arg::<i32>() });
                }
                ArgType::Uint => {
                    arg.value = ArgumentValue::Uint(unsafe { args.arg::<u32>() });
                }
                ArgType::LongInt => {
                    arg.value = ArgumentValue::LongInt(unsafe { args.arg::<i64>() });
                }
                ArgType::UlongInt => {
                    arg.value = ArgumentValue::UlongInt(unsafe { args.arg::<u64>() });
                }
                #[cfg(feature = "long_long")]
                ArgType::LongLongInt => {
                    arg.value = ArgumentValue::LongLongInt(unsafe { args.arg::<i64>() });
                }
                #[cfg(feature = "long_long")]
                ArgType::UlongLongInt => {
                    arg.value = ArgumentValue::UlongLongInt(unsafe { args.arg::<u64>() });
                }
                ArgType::Double => {
                    arg.value = ArgumentValue::Double(unsafe { args.arg::<f64>() });
                }
                ArgType::LongDouble => {
                    arg.value = ArgumentValue::LongDouble(unsafe { args.arg::<f64>() });
                }
                ArgType::Char => {
                    arg.value = ArgumentValue::Char(unsafe { args.arg::<i32>() });
                }
                #[cfg(feature = "wchar")]
                ArgType::WideChar => {
                    arg.value = ArgumentValue::WideChar(unsafe { args.arg::<u32>() });
                }
                ArgType::String => {
                    let ptr = unsafe { args.arg::<*const c_char>() };
                    arg.value = ArgumentValue::String(if ptr.is_null() {
                        CString::new("(NULL)").unwrap().into_raw()
                    } else {
                        ptr
                    });
                }
                #[cfg(feature = "wchar")]
                ArgType::WideString => {
                    let ptr = unsafe { args.arg::<*const u16>() };
                    arg.value = ArgumentValue::WideString(if ptr.is_null() {
                        static WIDE_NULL_STRING: [u16; 7] = [
                            '(' as u16,
                            'N' as u16,
                            'U' as u16,
                            'L' as u16,
                            'L' as u16,
                            ')' as u16,
                            0,
                        ];
                        WIDE_NULL_STRING.as_ptr()
                    } else {
                        ptr
                    });
                }
                ArgType::Pointer => {
                    arg.value = ArgumentValue::Pointer(unsafe { args.arg::<*mut c_void>() });
                }
                ArgType::CountScharPointer => {
                    arg.value = ArgumentValue::CountScharPointer(unsafe {
                        args.arg::<*mut i8>()
                    });
                }
                ArgType::CountShortPointer => {
                    arg.value = ArgumentValue::CountShortPointer(unsafe {
                        args.arg::<*mut i16>()
                    });
                }
                ArgType::CountIntPointer => {
                    arg.value = ArgumentValue::CountIntPointer(unsafe {
                        args.arg::<*mut i32>()
                    });
                }
                ArgType::CountLongIntPointer => {
                    arg.value = ArgumentValue::CountLongIntPointer(unsafe {
                        args.arg::<*mut i64>()
                    });
                }
                #[cfg(feature = "long_long")]
                ArgType::CountLongLongIntPointer => {
                    arg.value = ArgumentValue::CountLongLongIntPointer(unsafe {
                        args.arg::<*mut i64>()
                    });
                }
                #[cfg(feature = "unistdio")]
                ArgType::U8String => {
                    let ptr = unsafe { args.arg::<*const u8>() };
                    arg.value = ArgumentValue::U8String(if ptr.is_null() {
                        static U8_NULL_STRING: [u8; 7] = [b'(', b'N', b'U', b'L', b'L', b')', 0];
                        U8_NULL_STRING.as_ptr()
                    } else {
                        ptr
                    });
                }
                #[cfg(feature = "unistdio")]
                ArgType::U16String => {
                    let ptr = unsafe { args.arg::<*const u16>() };
                    arg.value = ArgumentValue::U16String(if ptr.is_null() {
                        static U16_NULL_STRING: [u16; 7] = [
                            '(' as u16,
                            'N' as u16,
                            'U' as u16,
                            'L' as u16,
                            'L' as u16,
                            ')' as u16,
                            0,
                        ];
                        U16_NULL_STRING.as_ptr()
                    } else {
                        ptr
                    });
                }
                #[cfg(feature = "unistdio")]
                ArgType::U32String => {
                    let ptr = unsafe { args.arg::<*const u32>() };
                    arg.value = ArgumentValue::U32String(if ptr.is_null() {
                        static U32_NULL_STRING: [u32; 7] = [
                            '(' as u32,
                            'N' as u32,
                            'U' as u32,
                            'L' as u32,
                            'L' as u32,
                            ')' as u32,
                            0,
                        ];
                        U32_NULL_STRING.as_ptr()
                    } else {
                        ptr
                    });
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}