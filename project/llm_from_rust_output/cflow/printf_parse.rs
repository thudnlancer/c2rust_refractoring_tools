use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

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
pub union ArgValue {
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
    pub float: f32,
    pub double: f64,
    pub longdouble: f64, // f128 not available in stable Rust
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
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
    pub value: ArgValue,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub count: usize,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
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
    pub dirs: Vec<CharDirective>,
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
) -> Result<(), i32> {
    let mut cp = format.as_ptr();
    let mut arg_posn = 0;
    directives.count = 0;
    directives.dirs.clear();
    directives.max_width_length = 0;
    directives.max_precision_length = 0;

    args.count = 0;
    args.args.clear();

    unsafe {
        while *cp != 0 {
            let c = *cp;
            cp = cp.add(1);

            if c != b'%' as i8 {
                continue;
            }

            let mut arg_index = !0;
            let mut dir = CharDirective {
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

            // Parse argument index
            if *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                let mut np = cp;
                while *np >= b'0' as i8 && *np <= b'9' as i8 {
                    np = np.add(1);
                }
                if *np == b'$' as i8 {
                    let mut n = 0;
                    np = cp;
                    while *np >= b'0' as i8 && *np <= b'9' as i8 {
                        n = xsum(
                            if n <= usize::MAX / 10 {
                                n * 10
                            } else {
                                usize::MAX
                            },
                            (*np - b'0' as i8) as usize,
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

            // Parse flags
            loop {
                match *cp {
                    b'\'' as i8 => {
                        dir.flags |= 1;
                        cp = cp.add(1);
                    }
                    b'-' as i8 => {
                        dir.flags |= 2;
                        cp = cp.add(1);
                    }
                    b'+' as i8 => {
                        dir.flags |= 4;
                        cp = cp.add(1);
                    }
                    b' ' as i8 => {
                        dir.flags |= 8;
                        cp = cp.add(1);
                    }
                    b'#' as i8 => {
                        dir.flags |= 16;
                        cp = cp.add(1);
                    }
                    b'0' as i8 => {
                        dir.flags |= 32;
                        cp = cp.add(1);
                    }
                    _ => break,
                }
            }

            // Parse width
            if *cp == b'*' as i8 {
                dir.width_start = cp;
                cp = cp.add(1);
                dir.width_end = cp;
                directives.max_width_length = directives.max_width_length.max(1);

                if *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                    let mut np = cp;
                    while *np >= b'0' as i8 && *np <= b'9' as i8 {
                        np = np.add(1);
                    }
                    if *np == b'$' as i8 {
                        let mut n = 0;
                        np = cp;
                        while *np >= b'0' as i8 && *np <= b'9' as i8 {
                            n = xsum(
                                if n <= usize::MAX / 10 {
                                    n * 10
                                } else {
                                    usize::MAX
                                },
                                (*np - b'0' as i8) as usize,
                            );
                            np = np.add(1);
                        }
                        if n == 0 || n == usize::MAX {
                            return Err(22);
                        }
                        dir.width_arg_index = n - 1;
                        cp = np.add(1);
                    }
                }

                if dir.width_arg_index == !0 {
                    dir.width_arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(22)?;
                }

                let n = dir.width_arg_index;
                if n >= args.args.len() {
                    args.args.resize(n + 1, Argument {
                        type_: ArgType::None,
                        value: ArgValue { int: 0 },
                    });
                }

                match args.args[n].type_ {
                    ArgType::None => args.args[n].type_ = ArgType::Int,
                    ArgType::Int => {}
                    _ => return Err(22),
                }
            } else if *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                dir.width_start = cp;
                while *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                    cp = cp.add(1);
                }
                dir.width_end = cp;
                let width_length = dir.width_end as usize - dir.width_start as usize;
                directives.max_width_length = directives.max_width_length.max(width_length);
            }

            // Parse precision
            if *cp == b'.' as i8 {
                cp = cp.add(1);
                if *cp == b'*' as i8 {
                    dir.precision_start = cp.sub(1);
                    cp = cp.add(1);
                    dir.precision_end = cp;
                    directives.max_precision_length = directives.max_precision_length.max(2);

                    if *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                        let mut np = cp;
                        while *np >= b'0' as i8 && *np <= b'9' as i8 {
                            np = np.add(1);
                        }
                        if *np == b'$' as i8 {
                            let mut n = 0;
                            np = cp;
                            while *np >= b'0' as i8 && *np <= b'9' as i8 {
                                n = xsum(
                                    if n <= usize::MAX / 10 {
                                        n * 10
                                    } else {
                                        usize::MAX
                                    },
                                    (*np - b'0' as i8) as usize,
                                );
                                np = np.add(1);
                            }
                            if n == 0 || n == usize::MAX {
                                return Err(22);
                            }
                            dir.precision_arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }

                    if dir.precision_arg_index == !0 {
                        dir.precision_arg_index = arg_posn;
                        arg_posn = arg_posn.checked_add(1).ok_or(22)?;
                    }

                    let n = dir.precision_arg_index;
                    if n >= args.args.len() {
                        args.args.resize(n + 1, Argument {
                            type_: ArgType::None,
                            value: ArgValue { int: 0 },
                        });
                    }

                    match args.args[n].type_ {
                        ArgType::None => args.args[n].type_ = ArgType::Int,
                        ArgType::Int => {}
                        _ => return Err(22),
                    }
                } else {
                    dir.precision_start = cp.sub(1);
                    while *cp >= b'0' as i8 && *cp <= b'9' as i8 {
                        cp = cp.add(1);
                    }
                    dir.precision_end = cp;
                    let precision_length = dir.precision_end as usize - dir.precision_start as usize;
                    directives.max_precision_length = directives.max_precision_length.max(precision_length);
                }
            }

            // Parse length modifiers
            let mut type_ = ArgType::None;
            let mut flags = 0;
            loop {
                match *cp {
                    b'h' as i8 => {
                        flags |= (1 << (flags & 1));
                        cp = cp.add(1);
                    }
                    b'L' as i8 => {
                        flags |= 4;
                        cp = cp.add(1);
                    }
                    b'l' as i8 => {
                        flags += 8;
                        cp = cp.add(1);
                    }
                    b'j' as i8 => {
                        if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b'z' as i8 | b'Z' as i8 => {
                        if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b't' as i8 => {
                        if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    _ => break,
                }
            }

            // Parse conversion specifier
            let c = *cp;
            cp = cp.add(1);
            match c {
                b'd' | b'i' => {
                    type_ = if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Longlongint
                    } else if flags >= 8 {
                        ArgType::Longint
                    } else if (flags & 2) != 0 {
                        ArgType::Schar
                    } else if (flags & 1) != 0 {
                        ArgType::Short
                    } else {
                        ArgType::Int
                    };
                }
                b'o' | b'u' | b'x' | b'X' => {
                    type_ = if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Ulonglongint
                    } else if flags >= 8 {
                        ArgType::Ulongint
                    } else if (flags & 2) != 0 {
                        ArgType::Uchar
                    } else if (flags & 1) != 0 {
                        ArgType::Ushort
                    } else {
                        ArgType::Uint
                    };
                }
                b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => {
                    type_ = if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Longdouble
                    } else {
                        ArgType::Double
                    };
                }
                b'c' => {
                    type_ = if flags >= 8 {
                        ArgType::WideChar
                    } else {
                        ArgType::Char
                    };
                }
                b'C' => {
                    type_ = ArgType::WideChar;
                    dir.conversion = b'c' as i8;
                }
                b's' => {
                    type_ = if flags >= 8 {
                        ArgType::WideString
                    } else {
                        ArgType::String
                    };
                }
                b'S' => {
                    type_ = ArgType::WideString;
                    dir.conversion = b's' as i8;
                }
                b'p' => {
                    type_ = ArgType::Pointer;
                }
                b'n' => {
                    type_ = if flags >= 16 || (flags & 4) != 0 {
                        ArgType::CountLonglongintPointer
                    } else if flags >= 8 {
                        ArgType::CountLongintPointer
                    } else if (flags & 2) != 0 {
                        ArgType::CountScharPointer
                    } else if (flags & 1) != 0 {
                        ArgType::CountShortPointer
                    } else {
                        ArgType::CountIntPointer
                    };
                }
                b'%' => {
                    type_ = ArgType::None;
                }
                _ => return Err(22),
            }

            if type_ != ArgType::None {
                dir.arg_index = arg_index;
                if dir.arg_index == !0 {
                    dir.arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(22)?;
                }

                let n = dir.arg_index;
                if n >= args.args.len() {
                    args.args.resize(n + 1, Argument {
                        type_: ArgType::None,
                        value: ArgValue { int: 0 },
                    });
                }

                match args.args[n].type_ {
                    ArgType::None => args.args[n].type_ = type_,
                    _ if args.args[n].type_ == type_ => {}
                    _ => return Err(22),
                }
            }

            dir.conversion = c;
            dir.dir_end = cp;
            directives.count += 1;
            directives.dirs.push(dir);

            if directives.dirs.len() >= directives.dirs.capacity() {
                directives.dirs.reserve(directives.dirs.capacity());
            }
        }
    }

    Ok(())
}