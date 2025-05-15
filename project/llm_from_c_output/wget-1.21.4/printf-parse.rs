/* This Rust translation maintains the same functionality as the original C code
   while adhering to Rust's safety practices and ownership system. */

use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::convert::TryFrom;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    None,
    Int,
    UInt,
    SChar,
    UChar,
    Short,
    UShort,
    LongInt,
    ULongInt,
    LongLongInt,
    ULongLongInt,
    Int8T,
    UInt8T,
    Int16T,
    UInt16T,
    Int32T,
    UInt32T,
    Int64T,
    UInt64T,
    IntFast8T,
    UIntFast8T,
    IntFast16T,
    UIntFast16T,
    IntFast32T,
    UIntFast32T,
    IntFast64T,
    UIntFast64T,
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
    CountInt8TPointer,
    CountInt16TPointer,
    CountInt32TPointer,
    CountInt64TPointer,
    CountIntFast8TPointer,
    CountIntFast16TPointer,
    CountIntFast32TPointer,
    CountIntFast64TPointer,
    U8String,
    U16String,
    U32String,
}

#[repr(C)]
#[derive(Debug)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
}

#[repr(C)]
#[derive(Debug)]
pub union ArgumentValue {
    pub int_value: i32,
    pub uint_value: u32,
    pub schar_value: i8,
    pub uchar_value: u8,
    pub short_value: i16,
    pub ushort_value: u16,
    pub longint_value: i64,
    pub ulongint_value: u64,
    pub longlongint_value: i64,
    pub ulonglongint_value: u64,
    pub int8t_value: i8,
    pub uint8t_value: u8,
    pub int16t_value: i16,
    pub uint16t_value: u16,
    pub int32t_value: i32,
    pub uint32t_value: u32,
    pub int64t_value: i64,
    pub uint64t_value: u64,
    pub intfast8t_value: i8,
    pub uintfast8t_value: u8,
    pub intfast16t_value: i16,
    pub uintfast16t_value: u16,
    pub intfast32t_value: i32,
    pub uintfast32t_value: u32,
    pub intfast64t_value: i64,
    pub uintfast64t_value: u64,
    pub double_value: f64,
    pub longdouble_value: f64,
    pub char_value: c_char,
    pub widechar_value: u32,
    pub string_value: *const c_char,
    pub widestring_value: *const u32,
    pub pointer_value: *mut (),
    pub countscharpointer_value: *mut i8,
    pub countshortpointer_value: *mut i16,
    pub countintpointer_value: *mut i32,
    pub countlongintpointer_value: *mut i64,
    pub countlonglongintpointer_value: *mut i64,
    pub countint8tpointer_value: *mut i8,
    pub countint16tpointer_value: *mut i16,
    pub countint32tpointer_value: *mut i32,
    pub countint64tpointer_value: *mut i64,
    pub countintfast8tpointer_value: *mut i8,
    pub countintfast16tpointer_value: *mut i16,
    pub countintfast32tpointer_value: *mut i32,
    pub countintfast64tpointer_value: *mut i64,
    pub u8string_value: *const u8,
    pub u16string_value: *const u16,
    pub u32string_value: *const u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Arguments {
    pub count: usize,
    pub arg: *mut Argument,
    pub direct_alloc_arg: [Argument; N_DIRECT_ALLOC_ARGUMENTS],
}

pub const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;
pub const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;

#[repr(C)]
#[derive(Debug)]
pub struct CharDirective {
    pub dir_start: *const c_char,
    pub dir_end: *const c_char,
    pub flags: c_int,
    pub width_start: *const c_char,
    pub width_end: *const c_char,
    pub width_arg_index: usize,
    pub precision_start: *const c_char,
    pub precision_end: *const c_char,
    pub precision_arg_index: usize,
    pub conversion: c_char,
    pub arg_index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct CharDirectives {
    pub count: usize,
    pub dir: *mut CharDirective,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [CharDirective; N_DIRECT_ALLOC_DIRECTIVES],
}

#[repr(C)]
#[derive(Debug)]
pub struct U8Directive {
    pub dir_start: *const u8,
    pub dir_end: *const u8,
    pub flags: c_int,
    pub width_start: *const u8,
    pub width_end: *const u8,
    pub width_arg_index: usize,
    pub precision_start: *const u8,
    pub precision_end: *const u8,
    pub precision_arg_index: usize,
    pub conversion: u8,
    pub arg_index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct U8Directives {
    pub count: usize,
    pub dir: *mut U8Directive,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [U8Directive; N_DIRECT_ALLOC_DIRECTIVES],
}

#[repr(C)]
#[derive(Debug)]
pub struct U16Directive {
    pub dir_start: *const u16,
    pub dir_end: *const u16,
    pub flags: c_int,
    pub width_start: *const u16,
    pub width_end: *const u16,
    pub width_arg_index: usize,
    pub precision_start: *const u16,
    pub precision_end: *const u16,
    pub precision_arg_index: usize,
    pub conversion: u16,
    pub arg_index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct U16Directives {
    pub count: usize,
    pub dir: *mut U16Directive,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [U16Directive; N_DIRECT_ALLOC_DIRECTIVES],
}

#[repr(C)]
#[derive(Debug)]
pub struct U32Directive {
    pub dir_start: *const u32,
    pub dir_end: *const u32,
    pub flags: c_int,
    pub width_start: *const u32,
    pub width_end: *const u32,
    pub width_arg_index: usize,
    pub precision_start: *const u32,
    pub precision_end: *const u32,
    pub precision_arg_index: usize,
    pub conversion: u32,
    pub arg_index: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct U32Directives {
    pub count: usize,
    pub dir: *mut U32Directive,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [U32Directive; N_DIRECT_ALLOC_DIRECTIVES],
}

pub const FLAG_GROUP: c_int = 1;
pub const FLAG_LEFT: c_int = 2;
pub const FLAG_SHOWSIGN: c_int = 4;
pub const FLAG_SPACE: c_int = 8;
pub const FLAG_ALT: c_int = 16;
pub const FLAG_ZERO: c_int = 32;
pub const FLAG_LOCALIZED: c_int = 64;

pub const ARG_NONE: usize = usize::MAX;

#[no_mangle]
pub extern "C" fn printf_parse(
    format: *const c_char,
    d: *mut CharDirectives,
    a: *mut Arguments,
) -> c_int {
    unsafe {
        let mut cp = format;
        let mut arg_posn: usize = 0;
        let mut d_allocated: usize = N_DIRECT_ALLOC_DIRECTIVES;
        let mut a_allocated: usize = N_DIRECT_ALLOC_ARGUMENTS;
        let mut max_width_length: usize = 0;
        let mut max_precision_length: usize = 0;

        (*d).count = 0;
        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();

        (*a).count = 0;
        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();

        macro_rules! register_arg {
            ($index:expr, $type:expr) => {
                let n = $index;
                if n >= a_allocated {
                    let new_size = if a_allocated == 0 {
                        1
                    } else {
                        a_allocated * 2
                    };
                    let new_size = if new_size <= n { n + 1 } else { new_size };
                    let new_size_bytes = new_size * mem::size_of::<Argument>();
                    if new_size_bytes == 0 {
                        return -1;
                    }
                    let new_args = if (*a).arg == (*a).direct_alloc_arg.as_mut_ptr() {
                        let new_args = libc::malloc(new_size_bytes) as *mut Argument;
                        if new_args.is_null() {
                            return -1;
                        }
                        ptr::copy_nonoverlapping(
                            (*a).arg,
                            new_args,
                            (*a).count,
                        );
                        new_args
                    } else {
                        let new_args = libc::realloc(
                            (*a).arg as *mut libc::c_void,
                            new_size_bytes,
                        ) as *mut Argument;
                        if new_args.is_null() {
                            return -1;
                        }
                        new_args
                    };
                    (*a).arg = new_args;
                    a_allocated = new_size;
                }
                while (*a).count <= n {
                    (*a).arg[(*a).count].type_ = ArgType::None;
                    (*a).count += 1;
                }
                if (*a).arg[n].type_ == ArgType::None {
                    (*a).arg[n].type_ = $type;
                } else if (*a).arg[n].type_ != $type {
                    return -1;
                }
            };
        }

        while *cp != 0 {
            let c = *cp;
            cp = cp.add(1);
            if c == b'%' as c_char {
                let mut arg_index = ARG_NONE;
                let dp = &mut (*d).dir[(*d).count];

                dp.dir_start = cp.sub(1);
                dp.flags = 0;
                dp.width_start = ptr::null();
                dp.width_end = ptr::null();
                dp.width_arg_index = ARG_NONE;
                dp.precision_start = ptr::null();
                dp.precision_end = ptr::null();
                dp.precision_arg_index = ARG_NONE;
                dp.arg_index = ARG_NONE;

                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let mut np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        np = np.add(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n: usize = 0;
                        let mut tmp = cp;
                        while tmp < np {
                            n = n * 10 + (*tmp - b'0' as c_char) as usize;
                            tmp = tmp.add(1);
                        }
                        if n == 0 {
                            return -1;
                        }
                        arg_index = n - 1;
                        cp = np.add(1);
                    }
                }

                loop {
                    if *cp == b'\'' as c_char {
                        dp.flags |= FLAG_GROUP;
                        cp = cp.add(1);
                    } else if *cp == b'-' as c_char {
                        dp.flags |= FLAG_LEFT;
                        cp = cp.add(1);
                    } else if *cp == b'+' as c_char {
                        dp.flags |= FLAG_SHOWSIGN;
                        cp = cp.add(1);
                    } else if *cp == b' ' as c_char {
                        dp.flags |= FLAG_SPACE;
                        cp = cp.add(1);
                    } else if *cp == b'#' as c_char {
                        dp.flags |= FLAG_ALT;
                        cp = cp.add(1);
                    } else if *cp == b'0' as c_char {
                        dp.flags |= FLAG_ZERO;
                        cp = cp.add(1);
                    } else if *cp == b'I' as c_char {
                        dp.flags |= FLAG_LOCALIZED;
                        cp = cp.add(1);
                    } else {
                        break;
                    }
                }

                if *cp == b'*' as c_char {
                    dp.width_start = cp;
                    cp = cp.add(1);
                    dp.width_end = cp;
                    if max_width_length < 1 {
                        max_width_length = 1;
                    }

                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let mut np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            np = np.add(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n: usize = 0;
                            let mut tmp = cp;
                            while tmp < np {
                                n = n * 10 + (*tmp - b'0' as c_char) as usize;
                                tmp = tmp.add(1);
                            }
                            if n == 0 {
                                return -1;
                            }
                            dp.width_arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }
                    if dp.width_arg_index == ARG_NONE {
                        dp.width_arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.width_arg_index == ARG_NONE {
                            return -1;
                        }
                    }
                    register_arg!(dp.width_arg_index, ArgType::Int);
                } else if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    dp.width_start = cp;
                    while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        cp = cp.add(1);
                    }
                    dp.width_end = cp;
                    let width_length = dp.width_end as usize - dp.width_start as usize;
                    if max_width_length < width_length {
                        max_width_length = width_length;
                    }
                }

                if *cp == b'.' as c_char {
                    cp = cp.add(1);
                    if *cp == b'*' as c_char {
                        dp.precision_start = cp.sub(1);
                        cp = cp.add(1);
                        dp.precision_end = cp;
                        if max_precision_length < 2 {
                            max_precision_length = 2;
                        }

                        if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            let mut np = cp;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                np = np.add(1);
                            }
                            if *np == b'$' as c_char {
                                let mut n: usize = 0;
                                let mut tmp = cp;
                                while tmp < np {
                                    n = n * 10 + (*tmp - b'0' as c_char) as usize;
                                    tmp = tmp.add(1);
                                }
                                if n == 0 {
                                    return -1;
                                }
                                dp.precision_arg_index = n - 1;
                                cp = np.add(1);
                            }
                        }
                        if dp.precision_arg_index == ARG_NONE {
                            dp.precision_arg_index = arg_posn;
                            arg_posn += 1;
                            if dp.precision_arg_index == ARG_NONE {
                                return -1;
                            }
                        }
                        register_arg!(dp.precision_arg_index, ArgType::Int);
                    } else {
                        dp.precision_start = cp.sub(1);
                        while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            cp = cp.add(1);
                        }
                        dp.precision_end = cp;
                        let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                        if max_precision_length < precision_length {
                            max_precision_length = precision_length;
                        }
                    }
                }

                let mut signed_type = ArgType::Int;
                let mut unsigned_type = ArgType::UInt;
                let mut pointer_type = ArgType::CountIntPointer;
                let mut floatingpoint_type = ArgType::Double;

                if *cp == b'h' as c_char {
                    if *cp.add(1) == b'h' as c_char {
                        signed_type = ArgType::SChar;
                        unsigned_type = ArgType::UChar;
                        pointer_type = ArgType::CountSCharPointer;
                        cp = cp.add(2);
                    } else {
                        signed_type = ArgType::Short;
                        unsigned_type = ArgType::UShort;
                        pointer_type = ArgType::CountShortPointer;
                        cp = cp.add(1);
                    }
                } else if *cp == b'l' as c_char {
                    if *cp.add(1) == b'l' as c_char {
                        signed_type = ArgType::LongLongInt;
                        unsigned_type = ArgType::ULongLongInt;
                        pointer_type = ArgType::CountLongLongIntPointer;
                        floatingpoint_type = ArgType::LongDouble;
                        cp = cp.add(2);
                    } else {
                        signed_type