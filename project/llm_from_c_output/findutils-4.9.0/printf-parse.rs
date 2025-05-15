/*!
Parse printf format string.
*/

use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::num::NonZeroUsize;

// Flags
const FLAG_GROUP: u32 = 1;      // ' flag
const FLAG_LEFT: u32 = 2;       // - flag
const FLAG_SHOWSIGN: u32 = 4;   // + flag
const FLAG_SPACE: u32 = 8;      // space flag
const FLAG_ALT: u32 = 16;       // # flag
const FLAG_ZERO: u32 = 32;
#[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
const FLAG_LOCALIZED: u32 = 64; // I flag, uses localized digits

// arg_index value indicating that no argument is consumed.
const ARG_NONE: usize = !0;

// Number of directly allocated directives (no malloc() needed).
const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;
const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArgType {
    None,
    Int,
    LongInt,
    LongLongInt,
    Short,
    SChar,
    UInt,
    ULongInt,
    ULongLongInt,
    UShort,
    UChar,
    Double,
    LongDouble,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountIntPointer,
    CountShortPointer,
    CountSCharPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
    U8String,
    U16String,
    U32String,
}

#[derive(Debug, Clone)]
struct Argument {
    arg_type: ArgType,
}

#[derive(Debug, Clone)]
struct Arguments {
    count: usize,
    args: Vec<Argument>,
    direct_alloc_args: [Argument; N_DIRECT_ALLOC_ARGUMENTS],
}

impl Arguments {
    fn new() -> Self {
        Arguments {
            count: 0,
            args: Vec::new(),
            direct_alloc_args: [Argument { arg_type: ArgType::None }; N_DIRECT_ALLOC_ARGUMENTS],
        }
    }
}

#[derive(Debug, Clone)]
struct Directive {
    dir_start: *const c_char,
    dir_end: *const c_char,
    flags: u32,
    width_start: *const c_char,
    width_end: *const c_char,
    width_arg_index: usize,
    precision_start: *const c_char,
    precision_end: *const c_char,
    precision_arg_index: usize,
    conversion: c_char,
    arg_index: usize,
}

#[derive(Debug, Clone)]
struct Directives {
    count: usize,
    dirs: Vec<Directive>,
    max_width_length: usize,
    max_precision_length: usize,
    direct_alloc_dirs: [Directive; N_DIRECT_ALLOC_DIRECTIVES],
}

impl Directives {
    fn new() -> Self {
        Directives {
            count: 0,
            dirs: Vec::new(),
            max_width_length: 0,
            max_precision_length: 0,
            direct_alloc_dirs: [Directive {
                dir_start: ptr::null(),
                dir_end: ptr::null(),
                flags: 0,
                width_start: ptr::null(),
                width_end: ptr::null(),
                width_arg_index: ARG_NONE,
                precision_start: ptr::null(),
                precision_end: ptr::null(),
                precision_arg_index: ARG_NONE,
                conversion: 0,
                arg_index: ARG_NONE,
            }; N_DIRECT_ALLOC_DIRECTIVES],
        }
    }
}

fn printf_parse(format: *const c_char, d: &mut Directives, a: &mut Arguments) -> c_int {
    let mut cp = format;
    let mut arg_posn = 0;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;

    d.count = 0;
    a.count = 0;

    unsafe {
        while *cp != 0 {
            let c = *cp;
            cp = cp.add(1);

            if c == b'%' as c_char {
                let mut arg_index = ARG_NONE;
                let dp = if d.count < N_DIRECT_ALLOC_DIRECTIVES {
                    &mut d.direct_alloc_dirs[d.count]
                } else {
                    if d.dirs.len() <= d.count - N_DIRECT_ALLOC_DIRECTIVES {
                        d.dirs.resize(d.count - N_DIRECT_ALLOC_DIRECTIVES + 1, Directive {
                            dir_start: ptr::null(),
                            dir_end: ptr::null(),
                            flags: 0,
                            width_start: ptr::null(),
                            width_end: ptr::null(),
                            width_arg_index: ARG_NONE,
                            precision_start: ptr::null(),
                            precision_end: ptr::null(),
                            precision_arg_index: ARG_NONE,
                            conversion: 0,
                            arg_index: ARG_NONE,
                        });
                    }
                    &mut d.dirs[d.count - N_DIRECT_ALLOC_DIRECTIVES]
                };

                // Initialize the next directive
                dp.dir_start = cp.sub(1);
                dp.flags = 0;
                dp.width_start = ptr::null();
                dp.width_end = ptr::null();
                dp.width_arg_index = ARG_NONE;
                dp.precision_start = ptr::null();
                dp.precision_end = ptr::null();
                dp.precision_arg_index = ARG_NONE;
                dp.arg_index = ARG_NONE;

                // Test for positional argument
                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let mut np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        np = np.add(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n = 0;
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

                // Read the flags
                loop {
                    match *cp {
                        b'\'' => {
                            dp.flags |= FLAG_GROUP;
                            cp = cp.add(1);
                        }
                        b'-' => {
                            dp.flags |= FLAG_LEFT;
                            cp = cp.add(1);
                        }
                        b'+' => {
                            dp.flags |= FLAG_SHOWSIGN;
                            cp = cp.add(1);
                        }
                        b' ' => {
                            dp.flags |= FLAG_SPACE;
                            cp = cp.add(1);
                        }
                        b'#' => {
                            dp.flags |= FLAG_ALT;
                            cp = cp.add(1);
                        }
                        b'0' => {
                            dp.flags |= FLAG_ZERO;
                            cp = cp.add(1);
                        }
                        #[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
                        b'I' => {
                            dp.flags |= FLAG_LOCALIZED;
                            cp = cp.add(1);
                        }
                        _ => break,
                    }
                }

                // Parse the field width
                if *cp == b'*' as c_char {
                    dp.width_start = cp;
                    cp = cp.add(1);
                    dp.width_end = cp;
                    if max_width_length < 1 {
                        max_width_length = 1;
                    }

                    // Test for positional argument
                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let mut np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            np = np.add(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n = 0;
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
                    register_arg(a, dp.width_arg_index, ArgType::Int)?;
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

                // Parse the precision
                if *cp == b'.' as c_char {
                    cp = cp.add(1);
                    if *cp == b'*' as c_char {
                        dp.precision_start = cp.sub(1);
                        cp = cp.add(1);
                        dp.precision_end = cp;
                        if max_precision_length < 2 {
                            max_precision_length = 2;
                        }

                        // Test for positional argument
                        if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            let mut np = cp;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                np = np.add(1);
                            }
                            if *np == b'$' as c_char {
                                let mut n = 0;
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
                        register_arg(a, dp.precision_arg_index, ArgType::Int)?;
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

                // Parse argument type/size specifiers
                let (conversion, arg_type) = parse_conversion(&mut cp)?;
                dp.conversion = conversion;

                if arg_type != ArgType::None {
                    dp.arg_index = arg_index;
                    if dp.arg_index == ARG_NONE {
                        dp.arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.arg_index == ARG_NONE {
                            return -1;
                        }
                    }
                    register_arg(a, dp.arg_index, arg_type)?;
                }

                dp.dir_end = cp;
                d.count += 1;
            }
        }
    }

    d.max_width_length = max_width_length;
    d.max_precision_length = max_precision_length;
    0
}

fn register_arg(a: &mut Arguments, index: usize, arg_type: ArgType) -> Result<(), c_int> {
    if index >= a.args.len() + N_DIRECT_ALLOC_ARGUMENTS {
        let new_size = if a.args.is_empty() {
            index + 1
        } else {
            (a.args.len() + N_DIRECT_ALLOC_ARGUMENTS).max(index + 1)
        };
        a.args.resize(new_size - N_DIRECT_ALLOC_ARGUMENTS, Argument { arg_type: ArgType::None });
    }

    let arg = if index < N_DIRECT_ALLOC_ARGUMENTS {
        &mut a.direct_alloc_args[index]
    } else {
        &mut a.args[index - N_DIRECT_ALLOC_ARGUMENTS]
    };

    if arg.arg_type == ArgType::None {
        arg.arg_type = arg_type;
    } else if arg.arg_type != arg_type {
        return Err(-1);
    }

    Ok(())
}

fn parse_conversion(cp: &mut *const c_char) -> Result<(c_char, ArgType), c_int> {
    let mut flags = 0;

    unsafe {
        loop {
            match **cp {
                b'h' => {
                    flags |= 1 << (flags & 1);
                    *cp = cp.add(1);
                }
                b'L' => {
                    flags |= 4;
                    *cp = cp.add(1);
                }
                b'l' => {
                    flags += 8;
                    *cp = cp.add(1);
                }
                b'j' => {
                    if mem::size_of::<i64>() > mem::size_of::<isize>() {
                        flags += 16;
                    } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                        flags += 8;
                    }
                    *cp = cp.add(1);
                }
                b'z' | b'Z' => {
                    if mem::size_of::<usize>() > mem::size_of::<isize>() {
                        flags += 16;
                    } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                        flags += 8;
                    }
                    *cp = cp.add(1);
                }
                b't' => {
                    if mem::size_of::<isize>() > mem::size_of::<i32>() {
                        flags += 8;
                    }
                    *cp = cp.add(1);
                }
                _ => break,
            }
        }

        let c = **cp;
        *cp = cp.add(1);

        let arg_type = match c {
            b'd' | b'i' => {
                if flags >= 16 || (flags & 4) != 0 {
                    ArgType::LongLongInt
                } else if flags >= 8 {
                    ArgType::LongInt
                } else if (flags & 2) != 0 {
                    ArgType::SChar
                } else if (flags & 1) != 0 {
                    ArgType::Short
                } else {
                    ArgType::Int
                }
            }
            b'o' | b'u' | b'x' | b'X' => {
                if flags >= 16 || (flags & 4) != 0 {
                    ArgType::ULongLongInt
                } else if flags >= 8 {
                    ArgType::ULongInt
                } else if (flags & 2) != 0 {
                    ArgType::UChar
                } else if (flags & 1) != 0 {
                    ArgType::UShort
                } else {
                    ArgType::UInt
                }
            }
            b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => {
                if flags >= 16 || (flags & 4) != 0 {
                    ArgType::LongDouble
                } else {
                    ArgType::Double
                }
            }
            b'c' => {
                if flags >= 8 {
                    #[cfg(feature = "wchar")]
                    { ArgType::WideChar }
                    #[cfg(not(feature = "wchar"))]
                    { return Err(-1); }
                } else {
                    ArgType::Char
                }
            }
            b's' => {
                if flags >= 8 {
                    #[cfg(feature = "wchar")]
                    { ArgType::WideString }
                    #[cfg(not(feature = "wchar"))]
                    { return Err(-1); }
                } else {
                    ArgType::String
                }
            }
            b'p' => ArgType::Pointer,
            b'n' => {
                if flags >= 16 || (flags & 4) != 0 {
                    ArgType::CountLongLongIntPointer
                } else if flags >= 8 {
                    ArgType::CountLongIntPointer
                } else if (flags & 2) != 0 {
                    ArgType::CountSCharPointer
                } else if (flags & 1) != 0 {
                    ArgType::CountShortPointer
                } else {
                    ArgType::CountIntPointer
                }
            }
            b'%' => ArgType::None,
            _ => return Err(-1),
        };

        Ok((c, arg_type))
    }
}