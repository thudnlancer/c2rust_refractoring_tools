use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
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
    Longdouble(f64), // Simplified from f128
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
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    pub fn fetch_args(&mut self, mut args: std::ffi::VaList) -> Result<(), i32> {
        for arg in &mut self.args {
            match arg.type_ {
                ArgType::Schar => {
                    arg.value = ArgumentValue::Schar(args.arg::<c_int>() as i8);
                }
                ArgType::Uchar => {
                    arg.value = ArgumentValue::Uchar(args.arg::<c_int>() as u8);
                }
                ArgType::Short => {
                    arg.value = ArgumentValue::Short(args.arg::<c_int>() as i16);
                }
                ArgType::Ushort => {
                    arg.value = ArgumentValue::Ushort(args.arg::<c_int>() as u16);
                }
                ArgType::Int => {
                    arg.value = ArgumentValue::Int(args.arg::<c_int>());
                }
                ArgType::Uint => {
                    arg.value = ArgumentValue::Uint(args.arg::<c_uint>());
                }
                ArgType::Longint => {
                    arg.value = ArgumentValue::Longint(args.arg::<c_long>());
                }
                ArgType::Ulongint => {
                    arg.value = ArgumentValue::Ulongint(args.arg::<c_ulong>());
                }
                ArgType::Longlongint => {
                    arg.value = ArgumentValue::Longlongint(args.arg::<c_longlong>());
                }
                ArgType::Ulonglongint => {
                    arg.value = ArgumentValue::Ulonglongint(args.arg::<c_ulonglong>());
                }
                ArgType::Double => {
                    arg.value = ArgumentValue::Double(args.arg::<f64>());
                }
                ArgType::Longdouble => {
                    arg.value = ArgumentValue::Longdouble(args.arg::<f64>());
                }
                ArgType::Char => {
                    arg.value = ArgumentValue::Char(args.arg::<c_int>());
                }
                ArgType::WideChar => {
                    let wchar = if std::mem::size_of::<u32>() < std::mem::size_of::<c_int>() {
                        args.arg::<c_int>() as u32
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
                    arg.value = ArgumentValue::Pointer(args.arg::<*mut c_void>() as usize);
                }
                ArgType::CountScharPointer => {
                    let ptr = args.arg::<*mut c_schar>();
                    arg.value = if ptr.is_null() {
                        ArgumentValue::CountScharPointer(None)
                    } else {
                        ArgumentValue::CountScharPointer(Some(Box::new(unsafe { *ptr })))
                    };
                }
                ArgType::CountShortPointer => {
                    let ptr = args.arg::<*mut c_short>();
                    arg.value = if ptr.is_null() {
                        ArgumentValue::CountShortPointer(None)
                    } else {
                        ArgumentValue::CountShortPointer(Some(Box::new(unsafe { *ptr })))
                    };
                }
                ArgType::CountIntPointer => {
                    let ptr = args.arg::<*mut c_int>();
                    arg.value = if ptr.is_null() {
                        ArgumentValue::CountIntPointer(None)
                    } else {
                        ArgumentValue::CountIntPointer(Some(Box::new(unsafe { *ptr })))
                    };
                }
                ArgType::CountLongintPointer => {
                    let ptr = args.arg::<*mut c_long>();
                    arg.value = if ptr.is_null() {
                        ArgumentValue::CountLongintPointer(None)
                    } else {
                        ArgumentValue::CountLongintPointer(Some(Box::new(unsafe { *ptr })))
                    };
                }
                ArgType::CountLonglongintPointer => {
                    let ptr = args.arg::<*mut c_longlong>();
                    arg.value = if ptr.is_null() {
                        ArgumentValue::CountLonglongintPointer(None)
                    } else {
                        ArgumentValue::CountLonglongintPointer(Some(Box::new(unsafe { *ptr })))
                    };
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}