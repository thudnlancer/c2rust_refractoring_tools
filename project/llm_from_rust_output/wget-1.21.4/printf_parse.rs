use std::fmt;
use std::mem;
use std::ptr;
use std::slice;

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
    Float,
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
pub union ArgumentValue {
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
    pub int_fast16: i64,
    pub uint_fast16: u64,
    pub int_fast32: i64,
    pub uint_fast32: u64,
    pub int_fast64: i64,
    pub uint_fast64: u64,
    pub float: f32,
    pub double: f64,
    pub longdouble: f64, // Simplified from f128
    pub char: i32,
    pub wide_char: u32,
    pub string: *const i8,
    pub wide_string: *const i32,
    pub pointer: *mut (),
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
    pub count_int_fast16_pointer: *mut i64,
    pub count_int_fast32_pointer: *mut i64,
    pub count_int_fast64_pointer: *mut i64,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgumentValue,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone, Copy)]
pub struct CharDirective {
    pub dir_start: *const i8,
    pub dir_end: *const i8,
    pub flags: i32,
    pub width_start: *const i8,
    pub width_end: *const i8,
    pub width_arg_index: usize,
    pub precision_start: *const i8,
    pub precision_end: *const i8,
    pub precision_arg_index: usize,
    pub conversion: i8,
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
    format: &str,
    directives: &mut CharDirectives,
    args: &mut Arguments,
) -> Result<(), ()> {
    let mut cp = format.as_ptr();
    let mut arg_posn = 0;
    directives.count = 0;
    directives.directives.clear();
    args.count = 0;
    args.args.clear();

    while unsafe { *cp } != 0 {
        let c = unsafe { *cp };
        cp = unsafe { cp.add(1) };

        if c != b'%' as i8 {
            continue;
        }

        let mut arg_index = !0;
        let mut dp = CharDirective {
            dir_start: cp.wrapping_sub(1),
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

        // Parse argument index
        if unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
            let mut np = cp;
            while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                np = unsafe { np.add(1) };
            }
            if unsafe { *np } == b'$' as i8 {
                let mut n = 0;
                np = cp;
                while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                    n = xsum(
                        if n <= usize::MAX / 10 {
                            n * 10
                        } else {
                            usize::MAX
                        },
                        (unsafe { *np } - b'0' as i8) as usize,
                    );
                    np = unsafe { np.add(1) };
                }
                if n == 0 || n == usize::MAX {
                    return Err(());
                }
                arg_index = n - 1;
                cp = unsafe { np.add(1) };
            }
        }

        // Parse flags
        loop {
            match unsafe { *cp } {
                b'\'' => {
                    dp.flags |= 1;
                    cp = unsafe { cp.add(1) };
                }
                b'-' => {
                    dp.flags |= 2;
                    cp = unsafe { cp.add(1) };
                }
                b'+' => {
                    dp.flags |= 4;
                    cp = unsafe { cp.add(1) };
                }
                b' ' => {
                    dp.flags |= 8;
                    cp = unsafe { cp.add(1) };
                }
                b'#' => {
                    dp.flags |= 16;
                    cp = unsafe { cp.add(1) };
                }
                b'0' => {
                    dp.flags |= 32;
                    cp = unsafe { cp.add(1) };
                }
                b'I' => {
                    dp.flags |= 64;
                    cp = unsafe { cp.add(1) };
                }
                _ => break,
            }
        }

        // Parse width
        if unsafe { *cp } == b'*' as i8 {
            dp.width_start = cp;
            cp = unsafe { cp.add(1) };
            dp.width_end = cp;
            directives.max_width_length = directives.max_width_length.max(1);

            if unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
                let mut np = cp;
                while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                    np = unsafe { np.add(1) };
                }
                if unsafe { *np } == b'$' as i8 {
                    let mut n = 0;
                    np = cp;
                    while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                        n = xsum(
                            if n <= usize::MAX / 10 {
                                n * 10
                            } else {
                                usize::MAX
                            },
                            (unsafe { *np } - b'0' as i8) as usize,
                        );
                        np = unsafe { np.add(1) };
                    }
                    if n == 0 || n == usize::MAX {
                        return Err(());
                    }
                    dp.width_arg_index = n - 1;
                    cp = unsafe { np.add(1) };
                }
            }

            if dp.width_arg_index == !0 {
                dp.width_arg_index = arg_posn;
                arg_posn = arg_posn.checked_add(1).ok_or(())?;
                if dp.width_arg_index == !0 {
                    return Err(());
                }
            }

            let n = dp.width_arg_index;
            if n >= args.args.len() {
                args.args.resize_with(n + 1, || Argument {
                    type_: ArgType::None,
                    value: ArgumentValue { int: 0 },
                });
            }

            match args.args[n].type_ {
                ArgType::None => args.args[n].type_ = ArgType::Int,
                ArgType::Int => {}
                _ => return Err(()),
            }
        } else if unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
            dp.width_start = cp;
            while unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
                cp = unsafe { cp.add(1) };
            }
            dp.width_end = cp;
            let width_length = unsafe { dp.width_end.offset_from(dp.width_start) } as usize;
            directives.max_width_length = directives.max_width_length.max(width_length);
        }

        // Parse precision
        if unsafe { *cp } == b'.' as i8 {
            cp = unsafe { cp.add(1) };
            if unsafe { *cp } == b'*' as i8 {
                dp.precision_start = cp.wrapping_sub(1);
                cp = unsafe { cp.add(1) };
                dp.precision_end = cp;
                directives.max_precision_length = directives.max_precision_length.max(2);

                if unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
                    let mut np = cp;
                    while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                        np = unsafe { np.add(1) };
                    }
                    if unsafe { *np } == b'$' as i8 {
                        let mut n = 0;
                        np = cp;
                        while unsafe { *np } >= b'0' as i8 && unsafe { *np } <= b'9' as i8 {
                            n = xsum(
                                if n <= usize::MAX / 10 {
                                    n * 10
                                } else {
                                    usize::MAX
                                },
                                (unsafe { *np } - b'0' as i8) as usize,
                            );
                            np = unsafe { np.add(1) };
                        }
                        if n == 0 || n == usize::MAX {
                            return Err(());
                        }
                        dp.precision_arg_index = n - 1;
                        cp = unsafe { np.add(1) };
                    }
                }

                if dp.precision_arg_index == !0 {
                    dp.precision_arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(())?;
                    if dp.precision_arg_index == !0 {
                        return Err(());
                    }
                }

                let n = dp.precision_arg_index;
                if n >= args.args.len() {
                    args.args.resize_with(n + 1, || Argument {
                        type_: ArgType::None,
                        value: ArgumentValue { int: 0 },
                    });
                }

                match args.args[n].type_ {
                    ArgType::None => args.args[n].type_ = ArgType::Int,
                    ArgType::Int => {}
                    _ => return Err(()),
                }
            } else {
                dp.precision_start = cp.wrapping_sub(1);
                while unsafe { *cp } >= b'0' as i8 && unsafe { *cp } <= b'9' as i8 {
                    cp = unsafe { cp.add(1) };
                }
                dp.precision_end = cp;
                let precision_length =
                    unsafe { dp.precision_end.offset_from(dp.precision_start) } as usize;
                directives.max_precision_length =
                    directives.max_precision_length.max(precision_length);
            }
        }

        // Parse length modifier
        let mut signed_type = ArgType::Int;
        let mut unsigned_type = ArgType::Uint;
        let mut pointer_type = ArgType::CountIntPointer;
        let mut floatingpoint_type = ArgType::Double;

        match unsafe { *cp } {
            b'h' => {
                if unsafe { *cp.add(1) } == b'h' as i8 {
                    signed_type = ArgType::Schar;
                    unsigned_type = ArgType::Uchar;
                    pointer_type = ArgType::CountScharPointer;
                    cp = unsafe { cp.add(2) };
                } else {
                    signed_type = ArgType::Short;
                    unsigned_type = ArgType::Ushort;
                    pointer_type = ArgType::CountShortPointer;
                    cp = unsafe { cp.add(1) };
                }
            }
            b'l' => {
                if unsafe { *cp.add(1) } == b'l' as i8 {
                    signed_type = ArgType::Longlongint;
                    unsigned_type = ArgType::Ulonglongint;
                    pointer_type = ArgType::CountLonglongintPointer;
                    floatingpoint_type = ArgType::LongDouble;
                    cp = unsafe { cp.add(2) };
                } else {
                    signed_type = ArgType::Longint;
                    unsigned_type = ArgType::Ulongint;
                    pointer_type = ArgType::CountLongintPointer;
                    cp = unsafe { cp.add(1) };
                }
            }
            b'j' => {
                if mem::size_of::<i64>() > mem::size_of::<i64>() {
                    signed_type = ArgType::Longlongint;
                    unsigned_type = ArgType::Ulonglongint;
                    pointer_type = ArgType::CountLonglongintPointer;
                    floatingpoint_type = ArgType::LongDouble;
                } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                    signed_type = ArgType::Longint;
                    unsigned_type = ArgType::Ulongint;
                    pointer_type = ArgType::CountLongintPointer;
                }
                cp = unsafe { cp.add(1) };
            }
            b'z' | b'Z' => {
                if mem::size_of::<usize>() > mem::size_of::<i64>() {
                    signed_type = ArgType::Longlongint;
                    unsigned_type = ArgType::Ulonglongint;
                    pointer_type = ArgType::CountLonglongintPointer;
                    floatingpoint_type = ArgType::LongDouble;
                } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                    signed_type = ArgType::Longint;
                    unsigned_type = ArgType::Ulongint;
                    pointer_type = ArgType::CountLongintPointer;
                }
                cp = unsafe { cp.add(1) };
            }
            b't' => {
                if mem::size_of::<isize>() > mem::size_of::<i64>() {
                    signed_type = ArgType::Longlongint;
                    unsigned_type = ArgType::Ulonglongint;
                    pointer_type = ArgType::CountLonglongintPointer;
                    floatingpoint_type = ArgType::LongDouble;
                } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                    signed_type = ArgType::Longint;
                    unsigned_type = ArgType::Ulongint;
                    pointer_type = ArgType::CountLongintPointer;
                }
                cp = unsafe { cp.add(1) };
            }
            b'w' => {
                if unsafe { *cp.add(1) } == b'f' as i8 {
                    if unsafe { *cp.add(2) } == b'8' as i8 {
                        signed_type = ArgType::IntFast8;
                        unsigned_type = ArgType::UintFast8;
                        pointer_type = ArgType::CountIntFast8Pointer;
                        cp = unsafe { cp.add(3) };
                    } else if unsafe { *cp.add(2) } == b'1' as i8
                        && unsafe { *cp.add(3) } == b'6' as i8
                    {
                        signed_type = ArgType::IntFast16;
                        unsigned_type = ArgType::UintFast16;
                        pointer_type = ArgType::CountIntFast16Pointer;
                        cp = unsafe { cp.add(4) };
                    } else if unsafe { *cp.add(2) } == b'3' as i8
                        && unsafe { *cp.add(3) } == b'2' as i8
                    {
                        signed_type = ArgType::IntFast32;
                        unsigned_type = ArgType::UintFast32;
                        pointer_type = ArgType::CountIntFast32Pointer;
                        cp = unsafe { cp.add(4) };
                    } else if unsafe { *cp.add(2) } == b'6' as i8
                        && unsafe { *cp.add(3) } == b'4' as i8
                    {
                        signed_type = ArgType::IntFast64;
                        unsigned_type = ArgType::UintFast64;
                        pointer_type = ArgType::CountIntFast64Pointer;
                        cp = unsafe { cp.add(4) };
                    }
                } else if unsafe { *cp.add(1) } == b'8' as i8 {
                    signed_type = ArgType::Int8;
