use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    None,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    LongInt,
    ULongInt,
    LongLongInt,
    ULongLongInt,
    Double,
    LongDouble,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountSCharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
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
    LongDouble(f64), // Note: f128 not stable in Rust yet
    Char(i32),
    WideChar(u32),
    String(String),
    WideString(Vec<u32>),
    Pointer(usize),
    CountSCharPointer(Option<Box<i8>>),
    CountShortPointer(Option<Box<i16>>),
    CountIntPointer(Option<Box<i32>>),
    CountLongIntPointer(Option<Box<i64>>),
    CountLongLongIntPointer(Option<Box<i64>>),
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

pub fn printf_fetchargs(mut args: impl Iterator<Item = ArgumentValue>) -> Result<Arguments, i32> {
    let mut collected_args = Vec::new();
    
    for arg in args {
        let type_ = match &arg {
            ArgumentValue::SChar(_) => ArgType::SChar,
            ArgumentValue::UChar(_) => ArgType::UChar,
            ArgumentValue::Short(_) => ArgType::Short,
            ArgumentValue::UShort(_) => ArgType::UShort,
            ArgumentValue::Int(_) => ArgType::Int,
            ArgumentValue::UInt(_) => ArgType::UInt,
            ArgumentValue::LongInt(_) => ArgType::LongInt,
            ArgumentValue::ULongInt(_) => ArgType::ULongInt,
            ArgumentValue::LongLongInt(_) => ArgType::LongLongInt,
            ArgumentValue::ULongLongInt(_) => ArgType::ULongLongInt,
            ArgumentValue::Double(_) => ArgType::Double,
            ArgumentValue::LongDouble(_) => ArgType::LongDouble,
            ArgumentValue::Char(_) => ArgType::Char,
            ArgumentValue::WideChar(_) => ArgType::WideChar,
            ArgumentValue::String(_) => ArgType::String,
            ArgumentValue::WideString(_) => ArgType::WideString,
            ArgumentValue::Pointer(_) => ArgType::Pointer,
            ArgumentValue::CountSCharPointer(_) => ArgType::CountSCharPointer,
            ArgumentValue::CountShortPointer(_) => ArgType::CountShortPointer,
            ArgumentValue::CountIntPointer(_) => ArgType::CountIntPointer,
            ArgumentValue::CountLongIntPointer(_) => ArgType::CountLongIntPointer,
            ArgumentValue::CountLongLongIntPointer(_) => ArgType::CountLongLongIntPointer,
        };
        
        collected_args.push(Argument { type_, value: arg });
    }
    
    Ok(Arguments { args: collected_args })
}

// Helper functions for creating argument values
impl ArgumentValue {
    pub fn from_string(s: &str) -> Self {
        ArgumentValue::String(s.to_string())
    }
    
    pub fn from_wide_string(s: &[u32]) -> Self {
        ArgumentValue::WideString(s.to_vec())
    }
    
    pub fn null_string() -> Self {
        ArgumentValue::String("(NULL)".to_string())
    }
    
    pub fn null_wide_string() -> Self {
        ArgumentValue::WideString(vec!['(' as u32, 'N' as u32, 'U' as u32, 'L' as u32, 'L' as u32, ')' as u32, 0])
    }
}