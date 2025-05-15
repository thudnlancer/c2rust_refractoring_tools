/* Decomposed printf argument list.
   Copyright (C) 1999, 2002-2003, 2006-2007, 2011-2023 Free Software
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

use std::ffi::{c_char, c_void, CStr, CString};
use std::os::raw::{c_int, c_long, c_longlong, c_ulong, c_ulonglong};
use std::ptr;
use std::mem;
use std::fmt;

#[cfg(feature = "wchar")]
use widestring::{WideChar, WideCStr, WideCString};

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
    Int8,
    Uint8,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    IntFast8,
    UintFast8,
    IntFast16,
    UintFast16,
    IntFast32,
    UintFast32,
    IntFast64,
    UintFast64,
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
    CountInt8Pointer,
    CountInt16Pointer,
    CountInt32Pointer,
    CountInt64Pointer,
    CountIntFast8Pointer,
    CountIntFast16Pointer,
    CountIntFast32Pointer,
    CountIntFast64Pointer,
    #[cfg(feature = "unistdio")]
    U8String,
    #[cfg(feature = "unistdio")]
    U16String,
    #[cfg(feature = "unistdio")]
    U32String,
}

#[derive(Debug)]
pub union ArgumentValue {
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
    pub a_int8: i8,
    pub a_uint8: u8,
    pub a_int16: i16,
    pub a_uint16: u16,
    pub a_int32: i32,
    pub a_uint32: u32,
    pub a_int64: i64,
    pub a_uint64: u64,
    pub a_int_fast8: i8,
    pub a_uint_fast8: u8,
    pub a_int_fast16: i16,
    pub a_uint_fast16: u16,
    pub a_int_fast32: i32,
    pub a_uint_fast32: u32,
    pub a_int_fast64: i64,
    pub a_uint_fast64: u64,
    pub a_double: f64,
    pub a_longdouble: f64, // Note: Rust doesn't have long double, using f64
    pub a_char: i32,
    #[cfg(feature = "wchar")]
    pub a_wide_char: u32, // Assuming wchar_t is u32
    pub a_string: *const c_char,
    #[cfg(feature = "wchar")]
    pub a_wide_string: *const u16, // Assuming wchar_t is u16
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut i8,
    pub a_count_short_pointer: *mut i16,
    pub a_count_int_pointer: *mut i32,
    pub a_count_longint_pointer: *mut i64,
    pub a_count_longlongint_pointer: *mut i64,
    pub a_count_int8_pointer: *mut i8,
    pub a_count_int16_pointer: *mut i16,
    pub a_count_int32_pointer: *mut i32,
    pub a_count_int64_pointer: *mut i64,
    pub a_count_int_fast8_pointer: *mut i8,
    pub a_count_int_fast16_pointer: *mut i16,
    pub a_count_int_fast32_pointer: *mut i32,
    pub a_count_int_fast64_pointer: *mut i64,
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
    pub value: ArgumentValue,
}

const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
    pub direct_alloc_args: [Argument; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    pub fn new(count: usize) -> Self {
        Self {
            count,
            args: Vec::with_capacity(count),
            direct_alloc_args: unsafe { mem::zeroed() },
        }
    }
}

pub fn printf_fetchargs(args: &mut fmt::Arguments, a: &mut Arguments) -> Result<(), ()> {
    for i in 0..a.count {
        let ap = if i < N_DIRECT_ALLOC_ARGUMENTS {
            &mut a.direct_alloc_args[i]
        } else {
            &mut a.args[i - N_DIRECT_ALLOC_ARGUMENTS]
        };

        match ap.type_ {
            ArgType::Schar => unsafe { ap.value.a_schar = args.next().unwrap().to_i8().unwrap() },
            ArgType::Uchar => unsafe { ap.value.a_uchar = args.next().unwrap().to_u8().unwrap() },
            ArgType::Short => unsafe { ap.value.a_short = args.next().unwrap().to_i16().unwrap() },
            ArgType::Ushort => unsafe { ap.value.a_ushort = args.next().unwrap().to_u16().unwrap() },
            ArgType::Int => unsafe { ap.value.a_int = args.next().unwrap().to_i32().unwrap() },
            ArgType::Uint => unsafe { ap.value.a_uint = args.next().unwrap().to_u32().unwrap() },
            ArgType::Longint => unsafe { ap.value.a_longint = args.next().unwrap().to_i64().unwrap() },
            ArgType::Ulongint => unsafe { ap.value.a_ulongint = args.next().unwrap().to_u64().unwrap() },
            ArgType::Longlongint => unsafe { ap.value.a_longlongint = args.next().unwrap().to_i64().unwrap() },
            ArgType::Ulonglongint => unsafe { ap.value.a_ulonglongint = args.next().unwrap().to_u64().unwrap() },
            ArgType::Int8 => unsafe { ap.value.a_int8 = args.next().unwrap().to_i8().unwrap() },
            ArgType::Uint8 => unsafe { ap.value.a_uint8 = args.next().unwrap().to_u8().unwrap() },
            ArgType::Int16 => unsafe { ap.value.a_int16 = args.next().unwrap().to_i16().unwrap() },
            ArgType::Uint16 => unsafe { ap.value.a_uint16 = args.next().unwrap().to_u16().unwrap() },
            ArgType::Int32 => unsafe { ap.value.a_int32 = args.next().unwrap().to_i32().unwrap() },
            ArgType::Uint32 => unsafe { ap.value.a_uint32 = args.next().unwrap().to_u32().unwrap() },
            ArgType::Int64 => unsafe { ap.value.a_int64 = args.next().unwrap().to_i64().unwrap() },
            ArgType::Uint64 => unsafe { ap.value.a_uint64 = args.next().unwrap().to_u64().unwrap() },
            ArgType::IntFast8 => unsafe { ap.value.a_int_fast8 = args.next().unwrap().to_i8().unwrap() },
            ArgType::UintFast8 => unsafe { ap.value.a_uint_fast8 = args.next().unwrap().to_u8().unwrap() },
            ArgType::IntFast16 => unsafe { ap.value.a_int_fast16 = args.next().unwrap().to_i16().unwrap() },
            ArgType::UintFast16 => unsafe { ap.value.a_uint_fast16 = args.next().unwrap().to_u16().unwrap() },
            ArgType::IntFast32 => unsafe { ap.value.a_int_fast32 = args.next().unwrap().to_i32().unwrap() },
            ArgType::UintFast32 => unsafe { ap.value.a_uint_fast32 = args.next().unwrap().to_u32().unwrap() },
            ArgType::IntFast64 => unsafe { ap.value.a_int_fast64 = args.next().unwrap().to_i64().unwrap() },
            ArgType::UintFast64 => unsafe { ap.value.a_uint_fast64 = args.next().unwrap().to_u64().unwrap() },
            ArgType::Double => unsafe { ap.value.a_double = args.next().unwrap().to_f64().unwrap() },
            ArgType::LongDouble => unsafe { ap.value.a_longdouble = args.next().unwrap().to_f64().unwrap() },
            ArgType::Char => unsafe { ap.value.a_char = args.next().unwrap().to_i32().unwrap() },
            #[cfg(feature = "wchar")]
            ArgType::WideChar => unsafe { ap.value.a_wide_char = args.next().unwrap().to_u32().unwrap() },
            ArgType::String => {
                let s = args.next().unwrap().to_string();
                let c_str = CString::new(s).unwrap();
                unsafe { ap.value.a_string = c_str.into_raw() };
            },
            #[cfg(feature = "wchar")]
            ArgType::WideString => {
                let s = args.next().unwrap().to_string();
                let wide_str = WideCString::from_str(&s).unwrap();
                unsafe { ap.value.a_wide_string = wide_str.into_raw() };
            },
            ArgType::Pointer => unsafe { ap.value.a_pointer = args.next().unwrap().to_pointer().unwrap() },
            ArgType::CountScharPointer => unsafe { ap.value.a_count_schar_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i8 },
            ArgType::CountShortPointer => unsafe { ap.value.a_count_short_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i16 },
            ArgType::CountIntPointer => unsafe { ap.value.a_count_int_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i32 },
            ArgType::CountLongintPointer => unsafe { ap.value.a_count_longint_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i64 },
            ArgType::CountLonglongintPointer => unsafe { ap.value.a_count_longlongint_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i64 },
            ArgType::CountInt8Pointer => unsafe { ap.value.a_count_int8_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i8 },
            ArgType::CountInt16Pointer => unsafe { ap.value.a_count_int16_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i16 },
            ArgType::CountInt32Pointer => unsafe { ap.value.a_count_int32_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i32 },
            ArgType::CountInt64Pointer => unsafe { ap.value.a_count_int64_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i64 },
            ArgType::CountIntFast8Pointer => unsafe { ap.value.a_count_int_fast8_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i8 },
            ArgType::CountIntFast16Pointer => unsafe { ap.value.a_count_int_fast16_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i16 },
            ArgType::CountIntFast32Pointer => unsafe { ap.value.a_count_int_fast32_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i32 },
            ArgType::CountIntFast64Pointer => unsafe { ap.value.a_count_int_fast64_pointer = args.next().unwrap().to_pointer().unwrap() as *mut i64 },
            #[cfg(feature = "unistdio")]
            ArgType::U8String => {
                let s = args.next().unwrap().to_string();
                let bytes = s.into_bytes();
                let ptr = Box::into_raw(bytes.into_boxed_slice()) as *const u8;
                unsafe { ap.value.a_u8_string = ptr };
            },
            #[cfg(feature = "unistdio")]
            ArgType::U16String => {
                let s = args.next().unwrap().to_string();
                let bytes: Vec<u16> = s.encode_utf16().collect();
                let ptr = Box::into_raw(bytes.into_boxed_slice()) as *const u16;
                unsafe { ap.value.a_u16_string = ptr };
            },
            #[cfg(feature = "unistdio")]
            ArgType::U32String => {
                let s = args.next().unwrap().to_string();
                let bytes: Vec<u32> = s.chars().map(|c| c as u32).collect();
                let ptr = Box::into_raw(bytes.into_boxed_slice()) as *const u32;
                unsafe { ap.value.a_u32_string = ptr };
            },
            ArgType::None => return Err(()),
        }
    }
    Ok(())
}