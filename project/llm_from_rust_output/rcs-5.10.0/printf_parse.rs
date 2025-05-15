use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::CStr;
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub union ArgumentData {
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
    pub longdouble: f64, // Simplified from f128
    pub char: c_int,
    pub wide_char: u32,
    pub string: *const c_char,
    pub wide_string: *const c_int,
    pub pointer: *mut c_void,
    pub count_schar_pointer: *mut i8,
    pub count_short_pointer: *mut i16,
    pub count_int_pointer: *mut i32,
    pub count_longint_pointer: *mut i64,
    pub count_longlongint_pointer: *mut i64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Argument {
    pub type_: ArgType,
    pub data: ArgumentData,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
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

#[derive(Debug)]
pub struct Arguments {
    count: usize,
    args: Vec<Argument>,
}

#[derive(Debug)]
pub struct CharDirectives {
    count: usize,
    dirs: Vec<CharDirective>,
    max_width_length: usize,
    max_precision_length: usize,
}

fn xsum(size1: usize, size2: usize) -> Option<usize> {
    size1.checked_add(size2).filter(|&sum| sum >= size1)
}

pub fn printf_parse(
    format: &CStr,
    directives: &mut CharDirectives,
    args: &mut Arguments,
) -> Result<(), ()> {
    let mut cp = format.as_ptr();
    let mut arg_posn = 0;
    directives.count = 0;
    directives.dirs.clear();
    directives.max_width_length = 0;
    directives.max_precision_length = 0;
    
    args.count = 0;
    args.args.clear();

    while unsafe { *cp != 0 } {
        unsafe {
            let c = *cp;
            cp = cp.add(1);

            if c != b'%' as c_char {
                continue;
            }

            let mut arg_index = usize::MAX;
            let mut dp = CharDirective {
                dir_start: cp.sub(1),
                dir_end: ptr::null(),
                flags: 0,
                width_start: ptr::null(),
                width_end: ptr::null(),
                width_arg_index: usize::MAX,
                precision_start: ptr::null(),
                precision_end: ptr::null(),
                precision_arg_index: usize::MAX,
                conversion: 0,
                arg_index: usize::MAX,
            };

            // Parse argument position if present
            if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                let np = cp;
                while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    cp = cp.add(1);
                }
                
                if *cp == b'$' as c_char {
                    let mut n = 0usize;
                    let mut np = np;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        n = match xsum(
                            if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                            (*np - b'0' as c_char) as usize
                        ) {
                            Some(val) => val,
                            None => return Err(()),
                        };
                        np = np.add(1);
                    }
                    
                    if n == 0 || n == usize::MAX {
                        return Err(());
                    }
                    
                    arg_index = n - 1;
                    cp = np.add(1);
                }
            }

            // Parse flags
            loop {
                match *cp {
                    b'\'' => { dp.flags |= 1; cp = cp.add(1); }
                    b'-' => { dp.flags |= 2; cp = cp.add(1); }
                    b'+' => { dp.flags |= 4; cp = cp.add(1); }
                    b' ' => { dp.flags |= 8; cp = cp.add(1); }
                    b'#' => { dp.flags |= 16; cp = cp.add(1); }
                    b'0' => { dp.flags |= 32; cp = cp.add(1); }
                    b'I' => { dp.flags |= 64; cp = cp.add(1); }
                    _ => break,
                }
            }

            // Parse width
            if *cp == b'*' as c_char {
                dp.width_start = cp;
                cp = cp.add(1);
                dp.width_end = cp;
                directives.max_width_length = directives.max_width_length.max(1);
                
                // Parse width argument position if present
                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let np = cp;
                    while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        cp = cp.add(1);
                    }
                    
                    if *cp == b'$' as c_char {
                        let mut n = 0usize;
                        let mut np = np;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            n = match xsum(
                                if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                                (*np - b'0' as c_char) as usize
                            ) {
                                Some(val) => val,
                                None => return Err(()),
                            };
                            np = np.add(1);
                        }
                        
                        if n == 0 || n == usize::MAX {
                            return Err(());
                        }
                        
                        dp.width_arg_index = n - 1;
                        cp = np.add(1);
                    }
                }
                
                if dp.width_arg_index == usize::MAX {
                    dp.width_arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(())?;
                }
                
                // Ensure argument vector is large enough
                let n = dp.width_arg_index;
                if n >= args.args.len() {
                    args.args.resize_with(n + 1, || Argument {
                        type_: ArgType::None,
                        data: ArgumentData { int: 0 },
                    });
                }
                
                match args.args[n].type_ {
                    ArgType::None => args.args[n].type_ = ArgType::Int,
                    ArgType::Int => (),
                    _ => return Err(()),
                }
            } else if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                dp.width_start = cp;
                while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    cp = cp.add(1);
                }
                dp.width_end = cp;
                let width_length = dp.width_end as usize - dp.width_start as usize;
                directives.max_width_length = directives.max_width_length.max(width_length);
            }

            // Parse precision
            if *cp == b'.' as c_char {
                cp = cp.add(1);
                
                if *cp == b'*' as c_char {
                    dp.precision_start = cp.sub(1);
                    cp = cp.add(1);
                    dp.precision_end = cp;
                    directives.max_precision_length = directives.max_precision_length.max(2);
                    
                    // Parse precision argument position if present
                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let np = cp;
                        while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            cp = cp.add(1);
                        }
                        
                        if *cp == b'$' as c_char {
                            let mut n = 0usize;
                            let mut np = np;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                n = match xsum(
                                    if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                                    (*np - b'0' as c_char) as usize
                                ) {
                                    Some(val) => val,
                                    None => return Err(()),
                                };
                                np = np.add(1);
                            }
                            
                            if n == 0 || n == usize::MAX {
                                return Err(());
                            }
                            
                            dp.precision_arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }
                    
                    if dp.precision_arg_index == usize::MAX {
                        dp.precision_arg_index = arg_posn;
                        arg_posn = arg_posn.checked_add(1).ok_or(())?;
                    }
                    
                    // Ensure argument vector is large enough
                    let n = dp.precision_arg_index;
                    if n >= args.args.len() {
                        args.args.resize_with(n + 1, || Argument {
                            type_: ArgType::None,
                            data: ArgumentData { int: 0 },
                        });
                    }
                    
                    match args.args[n].type_ {
                        ArgType::None => args.args[n].type_ = ArgType::Int,
                        ArgType::Int => (),
                        _ => return Err(()),
                    }
                } else {
                    dp.precision_start = cp.sub(1);
                    while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        cp = cp.add(1);
                    }
                    dp.precision_end = cp;
                    let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                    directives.max_precision_length = directives.max_precision_length.max(precision_length);
                }
            }

            // Parse length modifiers
            let mut flags = 0;
            loop {
                match *cp {
                    b'h' => { flags |= (1 << (flags & 1)); cp = cp.add(1); }
                    b'L' => { flags |= 4; cp = cp.add(1); }
                    b'l' => { flags += 8; cp = cp.add(1); }
                    b'j' => {
                        if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b'z' | b'Z' => {
                        if mem::size_of::<usize>() > mem::size_of::<i64>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b't' => {
                        if mem::size_of::<isize>() > mem::size_of::<i64>() {
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
            dp.conversion = c;
            dp.dir_end = cp;

            let type_ = match c {
                b'd' | b'i' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Longlongint
                    } else if flags >= 8 {
                        ArgType::Longint
                    } else if (flags & 2) != 0 {
                        ArgType::Schar
                    } else if (flags & 1) != 0 {
                        ArgType::Short
                    } else {
                        ArgType::Int
                    }
                }
                b'o' | b'u' | b'x' | b'X' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Ulonglongint
                    } else if flags >= 8 {
                        ArgType::Ulongint
                    } else if (flags & 2) != 0 {
                        ArgType::Uchar
                    } else if (flags & 1) != 0 {
                        ArgType::Ushort
                    } else {
                        ArgType::Uint
                    }
                }
                b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        ArgType::Longdouble
                    } else {
                        ArgType::Double
                    }
                }
                b'c' => {
                    if flags >= 8 {
                        ArgType::WideChar
                    } else {
                        ArgType::Char
                    }
                }
                b'C' => {
                    dp.conversion = b'c';
                    ArgType::WideChar
                }
                b's' => {
                    if flags >= 8 {
                        ArgType::WideString
                    } else {
                        ArgType::String
                    }
                }
                b'S' => {
                    dp.conversion = b's';
                    ArgType::WideString
                }
                b'p' => ArgType::Pointer,
                b'n' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        ArgType::CountLonglongintPointer
                    } else if flags >= 8 {
                        ArgType::CountLongintPointer
                    } else if (flags & 2) != 0 {
                        ArgType::CountScharPointer
                    } else if (flags & 1) != 0 {
                        ArgType::CountShortPointer
                    } else {
                        ArgType::CountIntPointer
                    }
                }
                b'%' => ArgType::None,
                _ => return Err(()),
            };

            if type_ != ArgType::None {
                dp.arg_index = arg_index;
                if dp.arg_index == usize::MAX {
                    dp.arg_index = arg_posn;
                    arg_posn = arg_posn.checked_add(1).ok_or(())?;
                }

                // Ensure argument vector is large enough
                let n = dp.arg_index;
                if n >= args.args.len() {
                    args.args.resize_with(n + 1, || Argument {
                        type_: ArgType::None,
                        data: ArgumentData { int: 0 },
                    });
                }

                match args.args[n].type_ {
                    ArgType::None => args.args[n].type_ = type_,
                    _ if args.args[n].type_ == type_ => (),
                    _ => return Err(()),
                }
            }

            directives.dirs.push(dp);
            directives.count += 1;
            args.count = args.args.len();
        }
    }

    Ok(())
}