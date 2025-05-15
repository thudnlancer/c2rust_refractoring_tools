use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_long, c_uchar, c_schar, c_ushort, c_short};
use std::os::raw::{c_longlong, c_ulonglong};
use std::ptr;

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
    WideChar,
    String,
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
    LongDouble(f64), // Simplified for example
    Char(i32),
    WideChar(u32),
    String(String),
    WideString(Vec<u32>),
    Pointer(usize),
    CountScharPointer(Option<Box<i8>>),
    CountShortPointer(Option<Box<i16>>),
    CountIntPointer(Option<Box<i32>>),
    CountLongintPointer(Option<Box<i64>>),
    CountLonglongintPointer(Option<Box<i64>>),
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
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(count: usize) -> Self {
        Arguments {
            args: Vec::with_capacity(count),
        }
    }

    pub fn fetch_args(&mut self, mut args: std::ffi::VaList) -> Result<(), i32> {
        for _ in 0..self.args.capacity() {
            let arg = match self.args.last().map(|a| a.arg_type) {
                Some(ArgType::Schar) => Argument {
                    arg_type: ArgType::Schar,
                    value: ArgumentValue::Schar(args.arg::<i32>() as i8),
                },
                Some(ArgType::Uchar) => Argument {
                    arg_type: ArgType::Uchar,
                    value: ArgumentValue::Uchar(args.arg::<i32>() as u8),
                },
                Some(ArgType::Short) => Argument {
                    arg_type: ArgType::Short,
                    value: ArgumentValue::Short(args.arg::<i32>() as i16),
                },
                Some(ArgType::Ushort) => Argument {
                    arg_type: ArgType::Ushort,
                    value: ArgumentValue::Ushort(args.arg::<i32>() as u16),
                },
                Some(ArgType::Int) => Argument {
                    arg_type: ArgType::Int,
                    value: ArgumentValue::Int(args.arg::<i32>()),
                },
                Some(ArgType::Uint) => Argument {
                    arg_type: ArgType::Uint,
                    value: ArgumentValue::Uint(args.arg::<u32>()),
                },
                Some(ArgType::Longint) => Argument {
                    arg_type: ArgType::Longint,
                    value: ArgumentValue::Longint(args.arg::<i64>()),
                },
                Some(ArgType::Ulongint) => Argument {
                    arg_type: ArgType::Ulongint,
                    value: ArgumentValue::Ulongint(args.arg::<u64>()),
                },
                Some(ArgType::Longlongint) => Argument {
                    arg_type: ArgType::Longlongint,
                    value: ArgumentValue::Longlongint(args.arg::<i64>()),
                },
                Some(ArgType::Ulonglongint) => Argument {
                    arg_type: ArgType::Ulonglongint,
                    value: ArgumentValue::Ulonglongint(args.arg::<u64>()),
                },
                Some(ArgType::Int8) => Argument {
                    arg_type: ArgType::Int8,
                    value: ArgumentValue::Int8(args.arg::<i32>() as i8),
                },
                Some(ArgType::Uint8) => Argument {
                    arg_type: ArgType::Uint8,
                    value: ArgumentValue::Uint8(args.arg::<i32>() as u8),
                },
                Some(ArgType::Int16) => Argument {
                    arg_type: ArgType::Int16,
                    value: ArgumentValue::Int16(args.arg::<i32>() as i16),
                },
                Some(ArgType::Uint16) => Argument {
                    arg_type: ArgType::Uint16,
                    value: ArgumentValue::Uint16(args.arg::<i32>() as u16),
                },
                Some(ArgType::Int32) => Argument {
                    arg_type: ArgType::Int32,
                    value: ArgumentValue::Int32(args.arg::<i32>()),
                },
                Some(ArgType::Uint32) => Argument {
                    arg_type: ArgType::Uint32,
                    value: ArgumentValue::Uint32(args.arg::<u32>()),
                },
                Some(ArgType::Int64) => Argument {
                    arg_type: ArgType::Int64,
                    value: ArgumentValue::Int64(args.arg::<i64>()),
                },
                Some(ArgType::Uint64) => Argument {
                    arg_type: ArgType::Uint64,
                    value: ArgumentValue::Uint64(args.arg::<u64>()),
                },
                Some(ArgType::IntFast8) => Argument {
                    arg_type: ArgType::IntFast8,
                    value: ArgumentValue::IntFast8(args.arg::<i32>() as i8),
                },
                Some(ArgType::UintFast8) => Argument {
                    arg_type: ArgType::UintFast8,
                    value: ArgumentValue::UintFast8(args.arg::<i32>() as u8),
                },
                Some(ArgType::IntFast16) => Argument {
                    arg_type: ArgType::IntFast16,
                    value: ArgumentValue::IntFast16(args.arg::<i64>()),
                },
                Some(ArgType::UintFast16) => Argument {
                    arg_type: ArgType::UintFast16,
                    value: ArgumentValue::UintFast16(args.arg::<u64>()),
                },
                Some(ArgType::IntFast32) => Argument {
                    arg_type: ArgType::IntFast32,
                    value: ArgumentValue::IntFast32(args.arg::<i64>()),
                },
                Some(ArgType::UintFast32) => Argument {
                    arg_type: ArgType::UintFast32,
                    value: ArgumentValue::UintFast32(args.arg::<u64>()),
                },
                Some(ArgType::IntFast64) => Argument {
                    arg_type: ArgType::IntFast64,
                    value: ArgumentValue::IntFast64(args.arg::<i64>()),
                },
                Some(ArgType::UintFast64) => Argument {
                    arg_type: ArgType::UintFast64,
                    value: ArgumentValue::UintFast64(args.arg::<u64>()),
                },
                Some(ArgType::Double) => Argument {
                    arg_type: ArgType::Double,
                    value: ArgumentValue::Double(args.arg::<f64>()),
                },
                Some(ArgType::LongDouble) => Argument {
                    arg_type: ArgType::LongDouble,
                    value: ArgumentValue::LongDouble(args.arg::<f64>()),
                },
                Some(ArgType::Char) => Argument {
                    arg_type: ArgType::Char,
                    value: ArgumentValue::Char(args.arg::<i32>()),
                },
                Some(ArgType::WideChar) => Argument {
                    arg_type: ArgType::WideChar,
                    value: ArgumentValue::WideChar(args.arg::<u32>()),
                },
                Some(ArgType::String) => {
                    let ptr = args.arg::<*const c_char>();
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
                    let ptr = args.arg::<*const u32>();
                    let s = if ptr.is_null() {
                        vec!['(' as u32, 'N' as u32, 'U' as u32, 'L' as u32, 'L' as u32, ')' as u32, 0]
                    } else {
                        let mut v = Vec::new();
                        let mut i = 0;
                        loop {
                            let c = unsafe { *ptr.offset(i) };
                            if c == 0 {
                                break;
                            }
                            v.push(c);
                            i += 1;
                        }
                        v
                    };
                    Argument {
                        arg_type: ArgType::WideString,
                        value: ArgumentValue::WideString(s),
                    }
                }
                Some(ArgType::Pointer) => Argument {
                    arg_type: ArgType::Pointer,
                    value: ArgumentValue::Pointer(args.arg::<*mut c_void>() as usize),
                },
                _ => return Err(-1),
            };
            self.args.push(arg);
        }
        Ok(())
    }
}