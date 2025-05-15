/* Decomposed printf argument list.
   Copyright (C) 1999, 2002-2003, 2006-2007, 2011-2022 Free Software
   Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
use std::ptr;

#[cfg(feature = "wchar")]
use widestring::WideCStr;

#[cfg(feature = "unistdio")]
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    LongLongInt,
    UlongLongInt,
    Double,
    LongDouble,
    Char,
    #[cfg(feature = "wint_t")]
    WideChar,
    String,
    #[cfg(feature = "wchar")]
    WideString,
    Pointer,
    CountScharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
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
    LongLongInt(i64),
    UlongLongInt(u64),
    Double(f64),
    LongDouble(f64), // Note: Rust doesn't have long double, using f64
    Char(i32),
    #[cfg(feature = "wint_t")]
    WideChar(u32),
    String(*const c_char),
    #[cfg(feature = "wchar")]
    WideString(*const u16),
    Pointer(*mut c_void),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongIntPointer(*mut i64),
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

pub const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
    pub direct_alloc_args: [Option<Argument>; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            args: Vec::with_capacity(count),
            direct_alloc_args: Default::default(),
        }
    }

    pub fn fetch_args(&mut self, args: &mut std::ffi::VaList) -> Result<(), i32> {
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
                Some(&mut self.args[i - N_DIRECT_ALLOC_ARGUMENTS])
            };

            if let Some(arg) = arg {
                match arg.type_ {
                    ArgType::Schar => {
                        arg.value = ArgumentValue::Schar(unsafe { args.arg::<c_int>() } as i8);
                    }
                    ArgType::Uchar => {
                        arg.value = ArgumentValue::Uchar(unsafe { args.arg::<c_int>() } as u8);
                    }
                    ArgType::Short => {
                        arg.value = ArgumentValue::Short(unsafe { args.arg::<c_int>() } as i16);
                    }
                    ArgType::Ushort => {
                        arg.value = ArgumentValue::Ushort(unsafe { args.arg::<c_int>() } as u16);
                    }
                    ArgType::Int => {
                        arg.value = ArgumentValue::Int(unsafe { args.arg::<c_int>() });
                    }
                    ArgType::Uint => {
                        arg.value = ArgumentValue::Uint(unsafe { args.arg::<c_uint>() });
                    }
                    ArgType::LongInt => {
                        arg.value = ArgumentValue::LongInt(unsafe { args.arg::<c_long>() } as i64);
                    }
                    ArgType::UlongInt => {
                        arg.value = ArgumentValue::UlongInt(unsafe { args.arg::<c_ulong>() } as u64);
                    }
                    ArgType::LongLongInt => {
                        arg.value = ArgumentValue::LongLongInt(unsafe { args.arg::<c_longlong>() } as i64);
                    }
                    ArgType::UlongLongInt => {
                        arg.value = ArgumentValue::UlongLongInt(unsafe { args.arg::<c_ulonglong>() } as u64);
                    }
                    ArgType::Double => {
                        arg.value = ArgumentValue::Double(unsafe { args.arg::<f64>() });
                    }
                    ArgType::LongDouble => {
                        arg.value = ArgumentValue::LongDouble(unsafe { args.arg::<f64>() });
                    }
                    ArgType::Char => {
                        arg.value = ArgumentValue::Char(unsafe { args.arg::<c_int>() });
                    }
                    #[cfg(feature = "wint_t")]
                    ArgType::WideChar => {
                        let val = unsafe { args.arg::<u32>() };
                        arg.value = ArgumentValue::WideChar(val);
                    }
                    ArgType::String => {
                        let ptr = unsafe { args.arg::<*const c_char>() };
                        arg.value = ArgumentValue::String(if ptr.is_null() {
                            b"(NULL)\0".as_ptr() as *const c_char
                        } else {
                            ptr
                        });
                    }
                    #[cfg(feature = "wchar")]
                    ArgType::WideString => {
                        let ptr = unsafe { args.arg::<*const u16>() };
                        arg.value = ArgumentValue::WideString(if ptr.is_null() {
                            &[b'(' as u16, b'N' as u16, b'U' as u16, b'L' as u16, b'L' as u16, b')' as u16, 0][0]
                        } else {
                            ptr
                        });
                    }
                    ArgType::Pointer => {
                        arg.value = ArgumentValue::Pointer(unsafe { args.arg::<*mut c_void>() });
                    }
                    ArgType::CountScharPointer => {
                        arg.value = ArgumentValue::CountScharPointer(unsafe { args.arg::<*mut i8>() });
                    }
                    ArgType::CountShortPointer => {
                        arg.value = ArgumentValue::CountShortPointer(unsafe { args.arg::<*mut i16>() });
                    }
                    ArgType::CountIntPointer => {
                        arg.value = ArgumentValue::CountIntPointer(unsafe { args.arg::<*mut i32>() });
                    }
                    ArgType::CountLongIntPointer => {
                        arg.value = ArgumentValue::CountLongIntPointer(unsafe { args.arg::<*mut i64>() });
                    }
                    ArgType::CountLongLongIntPointer => {
                        arg.value = ArgumentValue::CountLongLongIntPointer(unsafe { args.arg::<*mut i64>() });
                    }
                    #[cfg(feature = "unistdio")]
                    ArgType::U8String => {
                        let ptr = unsafe { args.arg::<*const u8>() };
                        arg.value = ArgumentValue::U8String(if ptr.is_null() {
                            b"(NULL)\0".as_ptr()
                        } else {
                            ptr
                        });
                    }
                    #[cfg(feature = "unistdio")]
                    ArgType::U16String => {
                        let ptr = unsafe { args.arg::<*const u16>() };
                        arg.value = ArgumentValue::U16String(if ptr.is_null() {
                            &[b'(' as u16, b'N' as u16, b'U' as u16, b'L' as u16, b'L' as u16, b')' as u16, 0][0]
                        } else {
                            ptr
                        });
                    }
                    #[cfg(feature = "unistdio")]
                    ArgType::U32String => {
                        let ptr = unsafe { args.arg::<*const u32>() };
                        arg.value = ArgumentValue::U32String(if ptr.is_null() {
                            &[b'(' as u32, b'N' as u32, b'U' as u32, b'L' as u32, b'L' as u32, b')' as u32, 0][0]
                        } else {
                            ptr
                        });
                    }
                    _ => return Err(-1),
                }
            }
        }
        Ok(())
    }
}