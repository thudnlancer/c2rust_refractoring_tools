use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_schar, c_ushort};
use std::mem;

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
    WideChar,
    String,
    WideString,
    Pointer,
    CountScharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
    CountInt8Pointer,
    CountInt16Pointer,
    CountInt32Pointer,
    CountInt64Pointer,
    CountIntFast8Pointer,
    CountIntFast16Pointer,
    CountIntFast32Pointer,
    CountIntFast64Pointer,
}

#[derive(Debug, Clone)]
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
    Int8(i8),
    Uint8(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    IntFast8(i8),
    UintFast8(u8),
    IntFast16(i64),
    UintFast16(u64),
    IntFast32(i64),
    UintFast32(u64),
    IntFast64(i64),
    UintFast64(u64),
    Double(f64),
    LongDouble(f64), // Note: f128 not stable in Rust yet
    Char(i32),
    WideChar(u32),
    String(String),
    WideString(Vec<u32>),
    Pointer(usize),
    CountScharPointer(Option<Box<i8>>),
    CountShortPointer(Option<Box<i16>>),
    CountIntPointer(Option<Box<i32>>),
    CountLongIntPointer(Option<Box<i64>>),
    CountLongLongIntPointer(Option<Box<i64>>),
    CountInt8Pointer(Option<Box<i8>>),
    CountInt16Pointer(Option<Box<i16>>),
    CountInt32Pointer(Option<Box<i32>>),
    CountInt64Pointer(Option<Box<i64>>),
    CountIntFast8Pointer(Option<Box<i8>>),
    CountIntFast16Pointer(Option<Box<i64>>),
    CountIntFast32Pointer(Option<Box<i64>>),
    CountIntFast64Pointer(Option<Box<i64>>),
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub arg_type: ArgType,
    pub value: ArgumentValue,
}

#[derive(Debug, Clone)]
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

    pub fn fetch_args(&mut self, va_args: &mut std::ffi::VaList) -> Result<(), i32> {
        for _ in 0..self.count {
            let arg = match self.args.last().map(|a| a.arg_type) {
                Some(ArgType::Schar) => {
                    Argument {
                        arg_type: ArgType::Schar,
                        value: ArgumentValue::Schar(va_args.arg::<i32>() as i8),
                    }
                }
                Some(ArgType::Uchar) => {
                    Argument {
                        arg_type: ArgType::Uchar,
                        value: ArgumentValue::Uchar(va_args.arg::<i32>() as u8),
                    }
                }
                Some(ArgType::Short) => {
                    Argument {
                        arg_type: ArgType::Short,
                        value: ArgumentValue::Short(va_args.arg::<i32>() as i16),
                    }
                }
                Some(ArgType::Ushort) => {
                    Argument {
                        arg_type: ArgType::Ushort,
                        value: ArgumentValue::Ushort(va_args.arg::<i32>() as u16),
                    }
                }
                Some(ArgType::Int) => {
                    Argument {
                        arg_type: ArgType::Int,
                        value: ArgumentValue::Int(va_args.arg::<i32>()),
                    }
                }
                Some(ArgType::Uint) => {
                    Argument {
                        arg_type: ArgType::Uint,
                        value: ArgumentValue::Uint(va_args.arg::<u32>()),
                    }
                }
                Some(ArgType::LongInt) => {
                    Argument {
                        arg_type: ArgType::LongInt,
                        value: ArgumentValue::LongInt(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::UlongInt) => {
                    Argument {
                        arg_type: ArgType::UlongInt,
                        value: ArgumentValue::UlongInt(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::LongLongInt) => {
                    Argument {
                        arg_type: ArgType::LongLongInt,
                        value: ArgumentValue::LongLongInt(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::UlongLongInt) => {
                    Argument {
                        arg_type: ArgType::UlongLongInt,
                        value: ArgumentValue::UlongLongInt(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::Int8) => {
                    Argument {
                        arg_type: ArgType::Int8,
                        value: ArgumentValue::Int8(va_args.arg::<i32>() as i8),
                    }
                }
                Some(ArgType::Uint8) => {
                    Argument {
                        arg_type: ArgType::Uint8,
                        value: ArgumentValue::Uint8(va_args.arg::<i32>() as u8),
                    }
                }
                Some(ArgType::Int16) => {
                    Argument {
                        arg_type: ArgType::Int16,
                        value: ArgumentValue::Int16(va_args.arg::<i32>() as i16),
                    }
                }
                Some(ArgType::Uint16) => {
                    Argument {
                        arg_type: ArgType::Uint16,
                        value: ArgumentValue::Uint16(va_args.arg::<i32>() as u16),
                    }
                }
                Some(ArgType::Int32) => {
                    Argument {
                        arg_type: ArgType::Int32,
                        value: ArgumentValue::Int32(va_args.arg::<i32>()),
                    }
                }
                Some(ArgType::Uint32) => {
                    Argument {
                        arg_type: ArgType::Uint32,
                        value: ArgumentValue::Uint32(va_args.arg::<u32>()),
                    }
                }
                Some(ArgType::Int64) => {
                    Argument {
                        arg_type: ArgType::Int64,
                        value: ArgumentValue::Int64(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::Uint64) => {
                    Argument {
                        arg_type: ArgType::Uint64,
                        value: ArgumentValue::Uint64(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::IntFast8) => {
                    Argument {
                        arg_type: ArgType::IntFast8,
                        value: ArgumentValue::IntFast8(va_args.arg::<i32>() as i8),
                    }
                }
                Some(ArgType::UintFast8) => {
                    Argument {
                        arg_type: ArgType::UintFast8,
                        value: ArgumentValue::UintFast8(va_args.arg::<i32>() as u8),
                    }
                }
                Some(ArgType::IntFast16) => {
                    Argument {
                        arg_type: ArgType::IntFast16,
                        value: ArgumentValue::IntFast16(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::UintFast16) => {
                    Argument {
                        arg_type: ArgType::UintFast16,
                        value: ArgumentValue::UintFast16(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::IntFast32) => {
                    Argument {
                        arg_type: ArgType::IntFast32,
                        value: ArgumentValue::IntFast32(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::UintFast32) => {
                    Argument {
                        arg_type: ArgType::UintFast32,
                        value: ArgumentValue::UintFast32(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::IntFast64) => {
                    Argument {
                        arg_type: ArgType::IntFast64,
                        value: ArgumentValue::IntFast64(va_args.arg::<i64>()),
                    }
                }
                Some(ArgType::UintFast64) => {
                    Argument {
                        arg_type: ArgType::UintFast64,
                        value: ArgumentValue::UintFast64(va_args.arg::<u64>()),
                    }
                }
                Some(ArgType::Double) => {
                    Argument {
                        arg_type: ArgType::Double,
                        value: ArgumentValue::Double(va_args.arg::<f64>()),
                    }
                }
                Some(ArgType::LongDouble) => {
                    Argument {
                        arg_type: ArgType::LongDouble,
                        value: ArgumentValue::LongDouble(va_args.arg::<f64>()),
                    }
                }
                Some(ArgType::Char) => {
                    Argument {
                        arg_type: ArgType::Char,
                        value: ArgumentValue::Char(va_args.arg::<i32>()),
                    }
                }
                Some(ArgType::WideChar) => {
                    let wchar = if mem::size_of::<u32>() < mem::size_of::<i32>() {
                        va_args.arg::<i32>() as u32
                    } else {
                        va_args.arg::<u32>()
                    };
                    Argument {
                        arg_type: ArgType::WideChar,
                        value: ArgumentValue::WideChar(wchar),
                    }
                }
                Some(ArgType::String) => {
                    let ptr = va_args.arg::<*const c_char>();
                    let s = if ptr.is_null() {
                        "(NULL)".to_string()
                    } else {
                        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
                    };
                    Argument {
                        arg_type: ArgType::String,
                        value: ArgumentValue::String(s),
                    }
                }
                Some(ArgType::WideString) => {
                    let ptr = va_args.arg::<*const u32>();
                    let ws = if ptr.is_null() {
                        vec!['(' as u32, 'N' as u32, 'U' as u32, 'L' as u32, 'L' as u32, ')' as u32, 0]
                    } else {
                        // Note: Proper wide string handling would require more complex logic
                        vec![]
                    };
                    Argument {
                        arg_type: ArgType::WideString,
                        value: ArgumentValue::WideString(ws),
                    }
                }
                Some(ArgType::Pointer) => {
                    Argument {
                        arg_type: ArgType::Pointer,
                        value: ArgumentValue::Pointer(va_args.arg::<*mut c_void>() as usize),
                    }
                }
                Some(ArgType::CountScharPointer) => {
                    Argument {
                        arg_type: ArgType::CountScharPointer,
                        value: ArgumentValue::CountScharPointer(None), // Simplified
                    }
                }
                // Other pointer types similarly simplified
                _ => return Err(-1),
            };
            self.args.push(arg);
        }
        Ok(())
    }
}