use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_long, c_ulonglong, c_longlong, c_schar, c_uchar, c_short, c_ushort};
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ArgType {
    None = 0,
    Schar = 1,
    Uchar = 2,
    Short = 3,
    Ushort = 4,
    Int = 5,
    Uint = 6,
    Longint = 7,
    Ulongint = 8,
    Longlongint = 9,
    Ulonglongint = 10,
    Double = 11,
    Longdouble = 12,
    Char = 13,
    WideChar = 14,
    String = 15,
    WideString = 16,
    Pointer = 17,
    CountScharPointer = 18,
    CountShortPointer = 19,
    CountIntPointer = 20,
    CountLongintPointer = 21,
    CountLonglongintPointer = 22,
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
    Longdouble(f64), // Note: f128 not stable in Rust yet
    Char(i32),
    WideChar(u32),
    String(String),
    WideString(Vec<u32>),
    Pointer(*mut c_void),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongintPointer(*mut i64),
    CountLonglongintPointer(*mut i64),
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
    pub fn new(capacity: usize) -> Self {
        Self {
            args: Vec::with_capacity(capacity),
        }
    }

    pub fn fetch_args(&mut self, mut args: std::ffi::VaList) -> Result<(), i32> {
        for arg in &mut self.args {
            match arg.arg_type {
                ArgType::Schar => {
                    arg.value = ArgumentValue::Schar(args.arg::<i32>() as i8);
                }
                ArgType::Uchar => {
                    arg.value = ArgumentValue::Uchar(args.arg::<i32>() as u8);
                }
                ArgType::Short => {
                    arg.value = ArgumentValue::Short(args.arg::<i32>() as i16);
                }
                ArgType::Ushort => {
                    arg.value = ArgumentValue::Ushort(args.arg::<i32>() as u16);
                }
                ArgType::Int => {
                    arg.value = ArgumentValue::Int(args.arg::<i32>());
                }
                ArgType::Uint => {
                    arg.value = ArgumentValue::Uint(args.arg::<u32>());
                }
                ArgType::Longint => {
                    arg.value = ArgumentValue::Longint(args.arg::<i64>());
                }
                ArgType::Ulongint => {
                    arg.value = ArgumentValue::Ulongint(args.arg::<u64>());
                }
                ArgType::Longlongint => {
                    arg.value = ArgumentValue::Longlongint(args.arg::<i64>());
                }
                ArgType::Ulonglongint => {
                    arg.value = ArgumentValue::Ulonglongint(args.arg::<u64>());
                }
                ArgType::Double => {
                    arg.value = ArgumentValue::Double(args.arg::<f64>());
                }
                ArgType::Longdouble => {
                    arg.value = ArgumentValue::Longdouble(args.arg::<f64>());
                }
                ArgType::Char => {
                    arg.value = ArgumentValue::Char(args.arg::<i32>());
                }
                ArgType::WideChar => {
                    let wchar = if std::mem::size_of::<u32>() < std::mem::size_of::<i32>() {
                        args.arg::<i32>() as u32
                    } else {
                        args.arg::<u32>()
                    };
                    arg.value = ArgumentValue::WideChar(wchar);
                }
                ArgType::String => {
                    let ptr = args.arg::<*const c_char>();
                    let s = if ptr.is_null() {
                        "(NULL)".to_string()
                    } else {
                        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
                    };
                    arg.value = ArgumentValue::String(s);
                }
                ArgType::WideString => {
                    let ptr = args.arg::<*const u32>();
                    let ws = if ptr.is_null() {
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
                    };
                    arg.value = ArgumentValue::WideString(ws);
                }
                ArgType::Pointer => {
                    arg.value = ArgumentValue::Pointer(args.arg::<*mut c_void>());
                }
                ArgType::CountScharPointer => {
                    arg.value = ArgumentValue::CountScharPointer(args.arg::<*mut i8>());
                }
                ArgType::CountShortPointer => {
                    arg.value = ArgumentValue::CountShortPointer(args.arg::<*mut i16>());
                }
                ArgType::CountIntPointer => {
                    arg.value = ArgumentValue::CountIntPointer(args.arg::<*mut i32>());
                }
                ArgType::CountLongintPointer => {
                    arg.value = ArgumentValue::CountLongintPointer(args.arg::<*mut i64>());
                }
                ArgType::CountLonglongintPointer => {
                    arg.value = ArgumentValue::CountLonglongintPointer(args.arg::<*mut i64>());
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}