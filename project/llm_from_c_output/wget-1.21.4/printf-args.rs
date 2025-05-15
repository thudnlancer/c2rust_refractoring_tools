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

use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

#[cfg(feature = "wchar")]
use widestring::WideCStr;

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
pub union ArgValue {
    pub schar: i8,
    pub uchar: u8,
    pub short: i16,
    pub ushort: u16,
    pub int: i32,
    pub uint: u32,
    pub longint: i64,
    pub ulongint: u64,
    pub longlongint: i64,
    pub ulonglongint: u64,
    pub int8: i8,
    pub uint8: u8,
    pub int16: i16,
    pub uint16: u16,
    pub int32: i32,
    pub uint32: u32,
    pub int64: i64,
    pub uint64: u64,
    pub int_fast8: i8,
    pub uint_fast8: u8,
    pub int_fast16: i16,
    pub uint_fast16: u16,
    pub int_fast32: i32,
    pub uint_fast32: u32,
    pub int_fast64: i64,
    pub uint_fast64: u64,
    pub double: f64,
    pub longdouble: f64,
    pub char: i32,
    #[cfg(feature = "wchar")]
    pub wide_char: u32,
    pub string: *const c_char,
    #[cfg(feature = "wchar")]
    pub wide_string: *const u16,
    pub pointer: *mut c_void,
    pub count_schar_pointer: *mut i8,
    pub count_short_pointer: *mut i16,
    pub count_int_pointer: *mut i32,
    pub count_longint_pointer: *mut i64,
    pub count_longlongint_pointer: *mut i64,
    pub count_int8_pointer: *mut i8,
    pub count_int16_pointer: *mut i16,
    pub count_int32_pointer: *mut i32,
    pub count_int64_pointer: *mut i64,
    pub count_int_fast8_pointer: *mut i8,
    pub count_int_fast16_pointer: *mut i16,
    pub count_int_fast32_pointer: *mut i32,
    pub count_int_fast64_pointer: *mut i64,
    #[cfg(feature = "unistdio")]
    pub u8_string: *const u8,
    #[cfg(feature = "unistdio")]
    pub u16_string: *const u16,
    #[cfg(feature = "unistdio")]
    pub u32_string: *const u32,
}

#[derive(Debug)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgValue,
}

pub const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

#[derive(Debug)]
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

pub fn printf_fetchargs(args: &mut std::ffi::VaList, a: &mut Arguments) -> Result<(), i32> {
    static NULL_STR: &[u8] = b"(NULL)\0";
    #[cfg(feature = "wchar")]
    static WIDE_NULL_STR: &[u16] = &['(' as u16, 'N' as u16, 'U' as u16, 'L' as u16, 'L' as u16, ')' as u16, 0];
    #[cfg(feature = "unistdio")]
    static U8_NULL_STR: &[u8] = b"(NULL)\0";
    #[cfg(feature = "unistdio")]
    static U16_NULL_STR: &[u16] = &['(' as u16, 'N' as u16, 'U' as u16, 'L' as u16, 'L' as u16, ')' as u16, 0];
    #[cfg(feature = "unistdio")]
    static U32_NULL_STR: &[u32] = &['(' as u32, 'N' as u32, 'U' as u32, 'L' as u32, 'L' as u32, ')' as u32, 0];

    for arg in a.args.iter_mut() {
        match arg.type_ {
            ArgType::Schar => unsafe { arg.value.schar = args.arg::<i32>() as i8 },
            ArgType::Uchar => unsafe { arg.value.uchar = args.arg::<u32>() as u8 },
            ArgType::Short => unsafe { arg.value.short = args.arg::<i32>() as i16 },
            ArgType::Ushort => unsafe { arg.value.ushort = args.arg::<u32>() as u16 },
            ArgType::Int => unsafe { arg.value.int = args.arg::<i32>() },
            ArgType::Uint => unsafe { arg.value.uint = args.arg::<u32>() },
            ArgType::Longint => unsafe { arg.value.longint = args.arg::<i64>() },
            ArgType::Ulongint => unsafe { arg.value.ulongint = args.arg::<u64>() },
            ArgType::Longlongint => unsafe { arg.value.longlongint = args.arg::<i64>() },
            ArgType::Ulonglongint => unsafe { arg.value.ulonglongint = args.arg::<u64>() },
            ArgType::Int8 => unsafe { arg.value.int8 = args.arg::<i8>() },
            ArgType::Uint8 => unsafe { arg.value.uint8 = args.arg::<u8>() },
            ArgType::Int16 => unsafe { arg.value.int16 = args.arg::<i16>() },
            ArgType::Uint16 => unsafe { arg.value.uint16 = args.arg::<u16>() },
            ArgType::Int32 => unsafe { arg.value.int32 = args.arg::<i32>() },
            ArgType::Uint32 => unsafe { arg.value.uint32 = args.arg::<u32>() },
            ArgType::Int64 => unsafe { arg.value.int64 = args.arg::<i64>() },
            ArgType::Uint64 => unsafe { arg.value.uint64 = args.arg::<u64>() },
            ArgType::IntFast8 => unsafe { arg.value.int_fast8 = args.arg::<i8>() },
            ArgType::UintFast8 => unsafe { arg.value.uint_fast8 = args.arg::<u8>() },
            ArgType::IntFast16 => unsafe { arg.value.int_fast16 = args.arg::<i16>() },
            ArgType::UintFast16 => unsafe { arg.value.uint_fast16 = args.arg::<u16>() },
            ArgType::IntFast32 => unsafe { arg.value.int_fast32 = args.arg::<i32>() },
            ArgType::UintFast32 => unsafe { arg.value.uint_fast32 = args.arg::<u32>() },
            ArgType::IntFast64 => unsafe { arg.value.int_fast64 = args.arg::<i64>() },
            ArgType::UintFast64 => unsafe { arg.value.uint_fast64 = args.arg::<u64>() },
            ArgType::Double => unsafe { arg.value.double = args.arg::<f64>() },
            ArgType::LongDouble => unsafe { arg.value.longdouble = args.arg::<f64>() },
            ArgType::Char => unsafe { arg.value.char = args.arg::<i32>() },
            #[cfg(feature = "wchar")]
            ArgType::WideChar => unsafe {
                arg.value.wide_char = args.arg::<u32>()
            },
            ArgType::String => unsafe {
                let ptr = args.arg::<*const c_char>();
                arg.value.string = if ptr.is_null() {
                    NULL_STR.as_ptr() as *const c_char
                } else {
                    ptr
                };
            },
            #[cfg(feature = "wchar")]
            ArgType::WideString => unsafe {
                let ptr = args.arg::<*const u16>();
                arg.value.wide_string = if ptr.is_null() {
                    WIDE_NULL_STR.as_ptr()
                } else {
                    ptr
                };
            },
            ArgType::Pointer => unsafe { arg.value.pointer = args.arg::<*mut c_void>() },
            ArgType::CountScharPointer => unsafe { arg.value.count_schar_pointer = args.arg::<*mut i8>() },
            ArgType::CountShortPointer => unsafe { arg.value.count_short_pointer = args.arg::<*mut i16>() },
            ArgType::CountIntPointer => unsafe { arg.value.count_int_pointer = args.arg::<*mut i32>() },
            ArgType::CountLongintPointer => unsafe { arg.value.count_longint_pointer = args.arg::<*mut i64>() },
            ArgType::CountLonglongintPointer => unsafe { arg.value.count_longlongint_pointer = args.arg::<*mut i64>() },
            ArgType::CountInt8Pointer => unsafe { arg.value.count_int8_pointer = args.arg::<*mut i8>() },
            ArgType::CountInt16Pointer => unsafe { arg.value.count_int16_pointer = args.arg::<*mut i16>() },
            ArgType::CountInt32Pointer => unsafe { arg.value.count_int32_pointer = args.arg::<*mut i32>() },
            ArgType::CountInt64Pointer => unsafe { arg.value.count_int64_pointer = args.arg::<*mut i64>() },
            ArgType::CountIntFast8Pointer => unsafe { arg.value.count_int_fast8_pointer = args.arg::<*mut i8>() },
            ArgType::CountIntFast16Pointer => unsafe { arg.value.count_int_fast16_pointer = args.arg::<*mut i16>() },
            ArgType::CountIntFast32Pointer => unsafe { arg.value.count_int_fast32_pointer = args.arg::<*mut i32>() },
            ArgType::CountIntFast64Pointer => unsafe { arg.value.count_int_fast64_pointer = args.arg::<*mut i64>() },
            #[cfg(feature = "unistdio")]
            ArgType::U8String => unsafe {
                let ptr = args.arg::<*const u8>();
                arg.value.u8_string = if ptr.is_null() {
                    U8_NULL_STR.as_ptr()
                } else {
                    ptr
                };
            },
            #[cfg(feature = "unistdio")]
            ArgType::U16String => unsafe {
                let ptr = args.arg::<*const u16>();
                arg.value.u16_string = if ptr.is_null() {
                    U16_NULL_STR.as_ptr()
                } else {
                    ptr
                };
            },
            #[cfg(feature = "unistdio")]
            ArgType::U32String => unsafe {
                let ptr = args.arg::<*const u32>();
                arg.value.u32_string = if ptr.is_null() {
                    U32_NULL_STR.as_ptr()
                } else {
                    ptr
                };
            },
            _ => return Err(-1),
        }
    }
    Ok(())
}