use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::cmp;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Int8 = 11,
    Uint8 = 12,
    Int16 = 13,
    Uint16 = 14,
    Int32 = 15,
    Uint32 = 16,
    Int64 = 17,
    Uint64 = 18,
    IntFast8 = 19,
    UintFast8 = 20,
    IntFast16 = 21,
    UintFast16 = 22,
    IntFast32 = 23,
    UintFast32 = 24,
    IntFast64 = 25,
    UintFast64 = 26,
    Double = 27,
    Longdouble = 28,
    Char = 29,
    WideChar = 30,
    String = 31,
    WideString = 32,
    Pointer = 33,
    CountScharPointer = 34,
    CountShortPointer = 35,
    CountIntPointer = 36,
    CountLongintPointer = 37,
    CountLonglongintPointer = 38,
    CountInt8Pointer = 39,
    CountInt16Pointer = 40,
    CountInt32Pointer = 41,
    CountInt64Pointer = 42,
    CountIntFast8Pointer = 43,
    CountIntFast16Pointer = 44,
    CountIntFast32Pointer = 45,
    CountIntFast64Pointer = 46,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
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
    Float(f32),
    Double(f64),
    Longdouble(f64), // Note: f128 not available in stable Rust
    Char(i32),
    WideChar(u32),
    String(*const c_char),
    WideString(*const i32),
    Pointer(*mut c_void),
    CountScharPointer(*mut i8),
    CountShortPointer(*mut i16),
    CountIntPointer(*mut i32),
    CountLongintPointer(*mut i64),
    CountLonglongintPointer(*mut i64),
    CountInt8Pointer(*mut i8),
    CountInt16Pointer(*mut i16),
    CountInt32Pointer(*mut i32),
    CountInt64Pointer(*mut i64),
    CountIntFast8Pointer(*mut i8),
    CountIntFast16Pointer(*mut i64),
    CountIntFast32Pointer(*mut i64),
    CountIntFast64Pointer(*mut i64),
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct CharDirective {
    pub dir_start: *const c_char,
    pub dir_end: *const c_char,
    pub flags: i32,
    pub width_start: *const c_char,
    pub width_end: *const c_char,
    pub width_arg_index: usize,
    pub precision_start: *const c_char,
    pub precision_end: *const c_char,
    pub precision_arg_index: usize,
    pub conversion: c_char,
    pub arg_index: usize,
}

#[derive(Debug, Clone)]
pub struct CharDirectives {
    pub count: usize,
    pub directives: Vec<CharDirective>,
    pub max_width_length: usize,
    pub max_precision_length: usize,
}

fn xsum(size1: usize, size2: usize) -> usize {
    size1.checked_add(size2).unwrap_or(usize::MAX)
}

pub fn printf_parse(
    format: *const c_char,
    d: &mut CharDirectives,
    a: &mut Arguments,
) -> Result<(), i32> {
    let mut cp = format;
    let mut arg_posn = 0;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;

    d.count = 0;
    d.directives.clear();
    a.count = 0;
    a.args.clear();

    unsafe {
        while *cp != 0 {
            let c = *cp;
            cp = cp.add(1);

            if c != b'%' as c_char {
                continue;
            }

            let mut arg_index = !0;
            let mut dp = CharDirective {
                dir_start: cp.sub(1),
                dir_end: ptr::null(),
                flags: 0,
                width_start: ptr::null(),
                width_end: ptr::null(),
                width_arg_index: !0,
                precision_start: ptr::null(),
                precision_end: ptr::null(),
                precision_arg_index: !0,
                conversion: 0,
                arg_index: !0,
            };

            if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                let mut np = cp;
                while *np >= b'0' as c_char && *np <= b'9' as c_char {
                    np = np.add(1);
                }
                if *np == b'$' as c_char {
                    let mut n = 0;
                    np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        n = xsum(
                            if n <= usize::MAX / 10 {
                                n * 10
                            } else {
                                usize::MAX
                            },
                            (*np - b'0' as c_char) as usize,
                        );
                        np = np.add(1);
                    }
                    if n == 0 || n == usize::MAX {
                        return Err(22);
                    }
                    arg_index = n - 1;
                    cp = np.add(1);
                }
            }

            loop {
                match *cp {
                    b'\'' => {
                        dp.flags |= 1;
                        cp = cp.add(1);
                    }
                    b'-' => {
                        dp.flags |= 2;
                        cp = cp.add(1);
                    }
                    b'+' => {
                        dp.flags |= 4;
                        cp = cp.add(1);
                    }
                    b' ' => {
                        dp.flags |= 8;
                        cp = cp.add(1);
                    }
                    b'#' => {
                        dp.flags |= 16;
                        cp = cp.add(1);
                    }
                    b'0' => {
                        dp.flags |= 32;
                        cp = cp.add(1);
                    }
                    b'I' => {
                        dp.flags |= 64;
                        cp = cp.add(1);
                    }
                    _ => break,
                }
            }

            if *cp == b'*' as c_char {
                dp.width_start = cp;
                cp = cp.add(1);
                dp.width_end = cp;
                max_width_length = cmp::max(max_width_length, 1);

                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let mut np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        np = np.add(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n = 0;
                        np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            n = xsum(
                                if n <= usize::MAX / 10 {
                                    n * 10
                                } else {
                                    usize::MAX
                                },
                                (*np - b'0' as c_char) as usize,
                            );
                            np = np.add(1);
                        }
                        if n == 0 || n == usize::MAX {
                            return Err(22);
                        }
                        dp.width_arg_index = n - 1;
                        cp = np.add(1);
                    }
                }

                if dp.width_arg_index == !0 {
                    dp.width_arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(22)?;
                }

                let n = dp.width_arg_index;
                if n >= a.args.len() {
                    a.args.resize(n + 1, Argument {
                        type_: ArgType::None,
                        value: ArgumentValue::Int(0),
                    });
                }

                match a.args[n].type_ {
                    ArgType::None => a.args[n].type_ = ArgType::Int,
                    ArgType::Int => {}
                    _ => return Err(22),
                }
            } else if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                dp.width_start = cp;
                while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    cp = cp.add(1);
                }
                dp.width_end = cp;
                let width_length = dp.width_end as usize - dp.width_start as usize;
                max_width_length = cmp::max(max_width_length, width_length);
            }

            if *cp == b'.' as c_char {
                cp = cp.add(1);
                if *cp == b'*' as c_char {
                    dp.precision_start = cp.sub(1);
                    cp = cp.add(1);
                    dp.precision_end = cp;
                    max_precision_length = cmp::max(max_precision_length, 2);

                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let mut np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            np = np.add(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n = 0;
                            np = cp;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                n = xsum(
                                    if n <= usize::MAX / 10 {
                                        n * 10
                                    } else {
                                        usize::MAX
                                    },
                                    (*np - b'0' as c_char) as usize,
                                );
                                np = np.add(1);
                            }
                            if n == 0 || n == usize::MAX {
                                return Err(22);
                            }
                            dp.precision_arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }

                    if dp.precision_arg_index == !0 {
                        dp.precision_arg_index = arg_posn;
                        arg_posn = arg_posn.checked_add(1).ok_or(22)?;
                    }

                    let n = dp.precision_arg_index;
                    if n >= a.args.len() {
                        a.args.resize(n + 1, Argument {
                            type_: ArgType::None,
                            value: ArgumentValue::Int(0),
                        });
                    }

                    match a.args[n].type_ {
                        ArgType::None => a.args[n].type_ = ArgType::Int,
                        ArgType::Int => {}
                        _ => return Err(22),
                    }
                } else {
                    dp.precision_start = cp.sub(1);
                    while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        cp = cp.add(1);
                    }
                    dp.precision_end = cp;
                    let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                    max_precision_length = cmp::max(max_precision_length, precision_length);
                }
            }

            let mut type_ = ArgType::None;
            let mut signed_type = ArgType::Int;
            let mut unsigned_type = ArgType::Uint;
            let mut pointer_type = ArgType::CountIntPointer;
            let mut floatingpoint_type = ArgType::Double;

            match *cp {
                b'h' => {
                    if *cp.add(1) == b'h' {
                        signed_type = ArgType::Schar;
                        unsigned_type = ArgType::Uchar;
                        pointer_type = ArgType::CountScharPointer;
                        cp = cp.add(2);
                    } else {
                        signed_type = ArgType::Short;
                        unsigned_type = ArgType::Ushort;
                        pointer_type = ArgType::CountShortPointer;
                        cp = cp.add(1);
                    }
                }
                b'l' => {
                    if *cp.add(1) == b'l' {
                        signed_type = ArgType::Longlongint;
                        unsigned_type = ArgType::Ulonglongint;
                        pointer_type = ArgType::CountLonglongintPointer;
                        floatingpoint_type = ArgType::Longdouble;
                        cp = cp.add(2);
                    } else {
                        signed_type = ArgType::Longint;
                        unsigned_type = ArgType::Ulongint;
                        pointer_type = ArgType::CountLongintPointer;
                        cp = cp.add(1);
                    }
                }
                b'j' => {
                    if mem::size_of::<i64>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longlongint;
                        unsigned_type = ArgType::Ulonglongint;
                        pointer_type = ArgType::CountLonglongintPointer;
                        floatingpoint_type = ArgType::Longdouble;
                    } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longint;
                        unsigned_type = ArgType::Ulongint;
                        pointer_type = ArgType::CountLongintPointer;
                    }
                    cp = cp.add(1);
                }
                b'z' | b'Z' => {
                    if mem::size_of::<usize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longlongint;
                        unsigned_type = ArgType::Ulonglongint;
                        pointer_type = ArgType::CountLonglongintPointer;
                        floatingpoint_type = ArgType::Longdouble;
                    } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longint;
                        unsigned_type = ArgType::Ulongint;
                        pointer_type = ArgType::CountLongintPointer;
                    }
                    cp = cp.add(1);
                }
                b't' => {
                    if mem::size_of::<isize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longlongint;
                        unsigned_type = ArgType::Ulonglongint;
                        pointer_type = ArgType::CountLonglongintPointer;
                        floatingpoint_type = ArgType::Longdouble;
                    } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Longint;
                        unsigned_type = ArgType::Ulongint;
                        pointer_type = ArgType::CountLongintPointer;
                    }
                    cp = cp.add(1);
                }
                b'w' => {
                    if *cp.add(1) == b'f' {
                        match *cp.add(2) {
                            b'8' => {
                                signed_type = ArgType::IntFast8;
                                unsigned_type = ArgType::UintFast8;
                                pointer_type = ArgType::CountIntFast8Pointer;
                                cp = cp.add(3);
                            }
                            b'1' if *cp.add(3) == b'6' => {
                                signed_type = ArgType::IntFast16;
                                unsigned_type = ArgType::UintFast16;
                                pointer_type = ArgType::CountIntFast16Pointer;
                                cp = cp.add(4);
                            }
                            b'3' if *cp.add(3) == b'2' => {
                                signed_type = ArgType::IntFast32;
                                unsigned_type = ArgType::UintFast32;
                                pointer_type = ArgType::CountIntFast32Pointer;
                                cp = cp.add(4);
                            }
                            b'6' if *cp.add(3) == b'4' => {
                                signed_type = ArgType::IntFast64;
                                unsigned_type = ArgType::UintFast64;
                                pointer_type = ArgType::CountIntFast64Pointer;
                                cp = cp.add(4);
                            }
                            _ => {}
                        }
                    } else {
                        match (*cp.add(1), *cp.add(2)) {
                            (b'8', _) => {
                                signed_type = ArgType::Int8;
                                unsigned_type = ArgType::Uint8;
                                pointer_type = ArgType::CountInt8Pointer;
                                cp = cp.add(2);
                            }
                            (b'1', b'6') => {
                                signed_type = ArgType::Int16;
                                unsigned_type = ArgType::Uint16;
                                pointer_type = ArgType::CountInt16Pointer;
                                cp = cp.add(3);
                            }
                            (b'3', b'2') => {
                                signed_type = ArgType::Int32;
                                unsigned_type = ArgType::Uint32;
                                pointer_type = ArgType::CountInt32Pointer;
                                cp = cp.add(3);
                            }
                            (b'6', b'4') => {
                                signed_type = ArgType::Int64;
                                unsigned_type = ArgType::Uint64;
                                pointer_type = ArgType::CountInt64Pointer;
                                cp = cp.add(3);
                            }
                            _ => {}
                        }
                    }
                }
                b'L' => {
                    signed_type = ArgType::Longlongint;
                    unsigned_type = ArgType::Ulonglongint;
                    pointer_type = ArgType::CountLonglongintPointer;
                    floatingpoint_type = ArgType::Longdouble;
                    cp = cp.add(1);
                }
                _ => {}
            }

            let c =