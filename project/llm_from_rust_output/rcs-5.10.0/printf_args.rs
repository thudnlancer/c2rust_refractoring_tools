use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort};
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
    Longint,
    Ulongint,
    Longlongint,
    Ulonglongint,
    Double,
    Longdouble,
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
    Pointer(usize),
    CountScharPointer(Box<i8>),
    CountShortPointer(Box<i16>),
    CountIntPointer(Box<i32>),
    CountLongintPointer(Box<i64>),
    CountLonglongintPointer(Box<i64>),
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
        Arguments { args: Vec::new() }
    }

    pub fn add_arg(&mut self, arg: Argument) {
        self.args.push(arg);
    }

    pub fn fetch_args(&mut self, mut args: std::ffi::VaList) -> Result<(), i32> {
        for arg in &mut self.args {
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
                ArgType::Longint => {
                    arg.value = ArgumentValue::Longint(unsafe { args.arg::<c_long>() });
                }
                ArgType::Ulongint => {
                    arg.value = ArgumentValue::Ulongint(unsafe { args.arg::<c_ulong>() });
                }
                ArgType::Longlongint => {
                    arg.value = ArgumentValue::Longlongint(unsafe { args.arg::<c_longlong>() });
                }
                ArgType::Ulonglongint => {
                    arg.value = ArgumentValue::Ulonglongint(unsafe { args.arg::<c_ulonglong>() });
                }
                ArgType::Double => {
                    arg.value = ArgumentValue::Double(unsafe { args.arg::<f64>() });
                }
                ArgType::Longdouble => {
                    arg.value = ArgumentValue::Longdouble(unsafe { args.arg::<f64>() });
                }
                ArgType::Char => {
                    arg.value = ArgumentValue::Char(unsafe { args.arg::<c_int>() });
                }
                ArgType::WideChar => {
                    let wchar = if mem::size_of::<u32>() < mem::size_of::<c_int>() {
                        unsafe { args.arg::<c_int>() } as u32
                    } else {
                        unsafe { args.arg::<u32>() }
                    };
                    arg.value = ArgumentValue::WideChar(wchar);
                }
                ArgType::String => {
                    let ptr = unsafe { args.arg::<*const c_char>() };
                    let str = if ptr.is_null() {
                        "(NULL)".to_string()
                    } else {
                        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
                    };
                    arg.value = ArgumentValue::String(str);
                }
                ArgType::WideString => {
                    let ptr = unsafe { args.arg::<*const u32>() };
                    let wide_str = if ptr.is_null() {
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
                    arg.value = ArgumentValue::WideString(wide_str);
                }
                ArgType::Pointer => {
                    arg.value = ArgumentValue::Pointer(unsafe { args.arg::<*mut c_void>() as usize });
                }
                ArgType::CountScharPointer => {
                    let ptr = unsafe { args.arg::<*mut c_schar>() };
                    arg.value = ArgumentValue::CountScharPointer(Box::new(unsafe { *ptr }));
                }
                ArgType::CountShortPointer => {
                    let ptr = unsafe { args.arg::<*mut c_short>() };
                    arg.value = ArgumentValue::CountShortPointer(Box::new(unsafe { *ptr }));
                }
                ArgType::CountIntPointer => {
                    let ptr = unsafe { args.arg::<*mut c_int>() };
                    arg.value = ArgumentValue::CountIntPointer(Box::new(unsafe { *ptr }));
                }
                ArgType::CountLongintPointer => {
                    let ptr = unsafe { args.arg::<*mut c_long>() };
                    arg.value = ArgumentValue::CountLongintPointer(Box::new(unsafe { *ptr }));
                }
                ArgType::CountLonglongintPointer => {
                    let ptr = unsafe { args.arg::<*mut c_longlong>() };
                    arg.value = ArgumentValue::CountLonglongintPointer(Box::new(unsafe { *ptr }));
                }
                _ => return Err(-1),
            }
        }
        Ok(())
    }
}