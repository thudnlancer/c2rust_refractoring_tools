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

use std::os::raw::{c_char, c_void, c_int, c_long, c_longlong, c_uchar, c_short, c_ushort, c_uint, c_ulong, c_ulonglong};
use std::ffi::{CStr, CString};
use std::ptr;
use std::mem;

#[cfg(feature = "wchar")]
use widestring::{WideChar, WideCString};

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
    LongInt,
    UlongInt,
    LongLongInt,
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
    CountLongLongIntPointer,
    #[cfg(feature = "unistdio")]
    U8String,
    #[cfg(feature = "unistdio")]
    U16String,
    #[cfg(feature = "unistdio")]
    U32String,
}

#[derive(Debug)]
pub union ArgumentData {
    pub a_schar: i8,
    pub a_uchar: u8,
    pub a_short: i16,
    pub a_ushort: u16,
    pub a_int: i32,
    pub a_uint: u32,
    pub a_longint: i64,
    pub a_ulongint: u64,
    pub a_longlongint: i64,
    pub a_ulonglongint: u64,
    pub a_float: f32,
    pub a_double: f64,
    pub a_longdouble: f64, // Note: Rust doesn't have long double, using f64
    pub a_char: i32,
    #[cfg(feature = "wchar")]
    pub a_wide_char: u32, // Assuming wchar_t is u32
    pub a_string: *const c_char,
    #[cfg(feature = "wchar")]
    pub a_wide_string: *const u32, // Assuming wchar_t is u32
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut i8,
    pub a_count_short_pointer: *mut i16,
    pub a_count_int_pointer: *mut i32,
    pub a_count_longint_pointer: *mut i64,
    pub a_count_longlongint_pointer: *mut i64,
    #[cfg(feature = "unistdio")]
    pub a_u8_string: *const u8,
    #[cfg(feature = "unistdio")]
    pub a_u16_string: *const u16,
    #[cfg(feature = "unistdio")]
    pub a_u32_string: *const u32,
}

#[derive(Debug)]
pub struct Argument {
    pub type_: ArgType,
    pub data: ArgumentData,
}

const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
    pub direct_alloc_args: [Argument; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            count: 0,
            args: Vec::new(),
            direct_alloc_args: unsafe { mem::zeroed() },
        }
    }

    pub fn fetch_args(&mut self, args: &mut std::ffi::VaList) -> Result<(), i32> {
        for i in 0..self.count {
            let ap = if i < N_DIRECT_ALLOC_ARGUMENTS {
                &mut self.direct_alloc_args[i]
            } else {
                &mut self.args[i - N_DIRECT_ALLOC_ARGUMENTS]
            };

            match ap.type_ {
                ArgType::Schar => unsafe { ap.data.a_schar = args.arg::<i32>() as i8 },
                ArgType::Uchar => unsafe { ap.data.a_uchar = args.arg::<i32>() as u8 },
                ArgType::Short => unsafe { ap.data.a_short = args.arg::<i32>() as i16 },
                ArgType::Ushort => unsafe { ap.data.a_ushort = args.arg::<i32>() as u16 },
                ArgType::Int => unsafe { ap.data.a_int = args.arg::<i32>() },
                ArgType::Uint => unsafe { ap.data.a_uint = args.arg::<u32>() },
                ArgType::LongInt => unsafe { ap.data.a_longint = args.arg::<i64>() },
                ArgType::UlongInt => unsafe { ap.data.a_ulongint = args.arg::<u64>() },
                ArgType::LongLongInt => unsafe { ap.data.a_longlongint = args.arg::<i64>() },
                ArgType::UlongLongInt => unsafe { ap.data.a_ulonglongint = args.arg::<u64>() },
                ArgType::Double => unsafe { ap.data.a_double = args.arg::<f64>() },
                ArgType::LongDouble => unsafe { ap.data.a_longdouble = args.arg::<f64>() },
                ArgType::Char => unsafe { ap.data.a_char = args.arg::<i32>() },
                #[cfg(feature = "wchar")]
                ArgType::WideChar => unsafe {
                    ap.data.a_wide_char = args.arg::<u32>()
                },
                ArgType::String => unsafe {
                    ap.data.a_string = args.arg::<*const c_char>();
                    if ap.data.a_string.is_null() {
                        ap.data.a_string = CString::new("(NULL)").unwrap().into_raw();
                    }
                },
                #[cfg(feature = "wchar")]
                ArgType::WideString => unsafe {
                    ap.data.a_wide_string = args.arg::<*const u32>();
                    if ap.data.a_wide_string.is_null() {
                        static WIDE_NULL_STRING: [u32; 7] = [
                            '(' as u32,
                            'N' as u32,
                            'U' as u32,
                            'L' as u32,
                            'L' as u32,
                            ')' as u32,
                            0
                        ];
                        ap.data.a_wide_string = WIDE_NULL_STRING.as_ptr();
                    }
                },
                ArgType::Pointer => unsafe {
                    ap.data.a_pointer = args.arg::<*mut c_void>();
                },
                ArgType::CountScharPointer => unsafe {
                    ap.data.a_count_schar_pointer = args.arg::<*mut i8>();
                },
                ArgType::CountShortPointer => unsafe {
                    ap.data.a_count_short_pointer = args.arg::<*mut i16>();
                },
                ArgType::CountIntPointer => unsafe {
                    ap.data.a_count_int_pointer = args.arg::<*mut i32>();
                },
                ArgType::CountLongIntPointer => unsafe {
                    ap.data.a_count_longint_pointer = args.arg::<*mut i64>();
                },
                ArgType::CountLongLongIntPointer => unsafe {
                    ap.data.a_count_longlongint_pointer = args.arg::<*mut i64>();
                },
                #[cfg(feature = "unistdio")]
                ArgType::U8String => unsafe {
                    ap.data.a_u8_string = args.arg::<*const u8>();
                    if ap.data.a_u8_string.is_null() {
                        static U8_NULL_STRING: [u8; 7] = [b'(', b'N', b'U', b'L', b'L', b')', 0];
                        ap.data.a_u8_string = U8_NULL_STRING.as_ptr();
                    }
                },
                #[cfg(feature = "unistdio")]
                ArgType::U16String => unsafe {
                    ap.data.a_u16_string = args.arg::<*const u16>();
                    if ap.data.a_u16_string.is_null() {
                        static U16_NULL_STRING: [u16; 7] = [
                            '(' as u16,
                            'N' as u16,
                            'U' as u16,
                            'L' as u16,
                            'L' as u16,
                            ')' as u16,
                            0
                        ];
                        ap.data.a_u16_string = U16_NULL_STRING.as_ptr();
                    }
                },
                #[cfg(feature = "unistdio")]
                ArgType::U32String => unsafe {
                    ap.data.a_u32_string = args.arg::<*const u32>();
                    if ap.data.a_u32_string.is_null() {
                        static U32_NULL_STRING: [u32; 7] = [
                            '(' as u32,
                            'N' as u32,
                            'U' as u32,
                            'L' as u32,
                            'L' as u32,
                            ')' as u32,
                            0
                        ];
                        ap.data.a_u32_string = U32_NULL_STRING.as_ptr();
                    }
                },
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}