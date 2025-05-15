use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ArgType {
    None = 0,
    SChar = 1,
    UChar = 2,
    Short = 3,
    UShort = 4,
    Int = 5,
    UInt = 6,
    LongInt = 7,
    ULongInt = 8,
    LongLongInt = 9,
    ULongLongInt = 10,
    Double = 11,
    LongDouble = 12,
    Char = 13,
    WideChar = 14,
    String = 15,
    WideString = 16,
    Pointer = 17,
    CountSCharPointer = 18,
    CountShortPointer = 19,
    CountIntPointer = 20,
    CountLongIntPointer = 21,
    CountLongLongIntPointer = 22,
}

#[derive(Debug, Clone)]
pub enum ArgumentValue {
    SChar(i8),
    UChar(u8),
    Short(i16),
    UShort(u16),
    Int(i32),
    UInt(u32),
    LongInt(i64),
    ULongInt(u64),
    LongLongInt(i64),
    ULongLongInt(u64),
    Double(f64),
    LongDouble(f64), // Simplified from f128
    Char(i32),
    WideChar(u32),
    String(String),
    WideString(Vec<u32>),
    Pointer(*mut c_void),
    CountSCharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongIntPointer(*mut i64),
    CountLongLongIntPointer(*mut i64),
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub args: Vec<Argument>,
}

impl Arguments {
    pub fn new(capacity: usize) -> Self {
        Self {
            args: Vec::with_capacity(capacity),
        }
    }

    pub fn fetch_args(&mut self, mut args: std::ffi::VaList) -> Result<(), i32> {
        for arg in &mut self.args {
            match arg.type_ {
                ArgType::SChar => {
                    arg.value = ArgumentValue::SChar(args.arg::<c_int>() as i8);
                }
                ArgType::UChar => {
                    arg.value = ArgumentValue::UChar(args.arg::<c_int>() as u8);
                }
                ArgType::Short => {
                    arg.value = ArgumentValue::Short(args.arg::<c_int>() as i16);
                }
                ArgType::UShort => {
                    arg.value = ArgumentValue::UShort(args.arg::<c_int>() as u16);
                }
                ArgType::Int => {
                    arg.value = ArgumentValue::Int(args.arg::<c_int>());
                }
                ArgType::UInt => {
                    arg.value = ArgumentValue::UInt(args.arg::<c_uint>());
                }
                ArgType::LongInt => {
                    arg.value = ArgumentValue::LongInt(args.arg::<c_long>() as i64);
                }
                ArgType::ULongInt => {
                    arg.value = ArgumentValue::ULongInt(args.arg::<c_ulong>() as u64);
                }
                ArgType::LongLongInt => {
                    arg.value = ArgumentValue::LongLongInt(args.arg::<c_longlong>());
                }
                ArgType::ULongLongInt => {
                    arg.value = ArgumentValue::ULongLongInt(args.arg::<c_ulonglong>());
                }
                ArgType::Double => {
                    arg.value = ArgumentValue::Double(args.arg::<f64>());
                }
                ArgType::LongDouble => {
                    arg.value = ArgumentValue::LongDouble(args.arg::<f64>());
                }
                ArgType::Char => {
                    arg.value = ArgumentValue::Char(args.arg::<c_int>());
                }
                ArgType::WideChar => {
                    arg.value = ArgumentValue::WideChar(args.arg::<u32>());
                }
                ArgType::String => {
                    let ptr = args.arg::<*const c_char>();
                    arg.value = ArgumentValue::String(if ptr.is_null() {
                        "(NULL)".to_string()
                    } else {
                        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
                    });
                }
                ArgType::WideString => {
                    let ptr = args.arg::<*const u32>();
                    arg.value = ArgumentValue::WideString(if ptr.is_null() {
                        vec!['(' as u32, 'N' as u32, 'U' as u32, 'L' as u32, 'L' as u32, ')' as u32, 0]
                    } else {
                        let mut vec = Vec::new();
                        let mut i = 0;
                        loop {
                            let c = unsafe { *ptr.offset(i) };
                            if c == 0 {
                                break;
                            }
                            vec.push(c);
                            i += 1;
                        }
                        vec
                    });
                }
                ArgType::Pointer => {
                    arg.value = ArgumentValue::Pointer(args.arg::<*mut c_void>());
                }
                ArgType::CountSCharPointer => {
                    arg.value = ArgumentValue::CountSCharPointer(args.arg::<*mut c_schar>());
                }
                ArgType::CountShortPointer => {
                    arg.value = ArgumentValue::CountShortPointer(args.arg::<*mut c_short>());
                }
                ArgType::CountIntPointer => {
                    arg.value = ArgumentValue::CountIntPointer(args.arg::<*mut c_int>());
                }
                ArgType::CountLongIntPointer => {
                    arg.value = ArgumentValue::CountLongIntPointer(args.arg::<*mut c_long>());
                }
                ArgType::CountLongLongIntPointer => {
                    arg.value = ArgumentValue::CountLongLongIntPointer(args.arg::<*mut c_longlong>());
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}