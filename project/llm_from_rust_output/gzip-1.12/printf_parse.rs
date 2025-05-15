use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int, c_void, c_long, c_ulong, c_uchar, c_short, c_ushort, c_uint, c_longlong, c_ulonglong, c_float, c_double};
use std::ffi::CStr;

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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub union ArgumentData {
    pub a_schar: c_char,
    pub a_uchar: c_uchar,
    pub a_short: c_short,
    pub a_ushort: c_ushort,
    pub a_int: c_int,
    pub a_uint: c_uint,
    pub a_longint: c_long,
    pub a_ulongint: c_ulong,
    pub a_longlongint: c_longlong,
    pub a_ulonglongint: c_ulonglong,
    pub a_float: c_float,
    pub a_double: c_double,
    pub a_longdouble: f128::f128,
    pub a_char: c_int,
    pub a_wide_char: u32,
    pub a_string: *const c_char,
    pub a_wide_string: *const c_int,
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut c_char,
    pub a_count_short_pointer: *mut c_short,
    pub a_count_int_pointer: *mut c_int,
    pub a_count_longint_pointer: *mut c_long,
    pub a_count_longlongint_pointer: *mut c_longlong,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Argument {
    pub type_: ArgType,
    pub a: ArgumentData,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Arguments {
    pub count: usize,
    pub arg: *mut Argument,
    pub direct_alloc_arg: [Argument; 7],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct CharDirectives {
    pub count: usize,
    pub dir: *mut CharDirective,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [CharDirective; 7],
}

fn xsum(size1: usize, size2: usize) -> usize {
    let sum = size1.wrapping_add(size2);
    if sum >= size1 { sum } else { usize::MAX }
}

pub fn printf_parse(
    format: *const c_char,
    d: *mut CharDirectives,
    a: *mut Arguments,
) -> c_int {
    unsafe {
        let mut cp = format;
        let mut arg_posn = 0;
        let mut d_allocated = 7;
        let mut a_allocated = 7;
        let mut max_width_length = 0;
        let mut max_precision_length = 0;

        (*d).count = 0;
        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
        (*a).count = 0;
        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();

        while *cp != 0 {
            let c = *cp;
            cp = cp.offset(1);

            if c != b'%' as c_char {
                continue;
            }

            let mut arg_index = !0;
            let dp = &mut *(*d).dir.offset((*d).count as isize);

            dp.dir_start = cp.offset(-1);
            dp.flags = 0;
            dp.width_start = ptr::null();
            dp.width_end = ptr::null();
            dp.width_arg_index = !0;
            dp.precision_start = ptr::null();
            dp.precision_end = ptr::null();
            dp.precision_arg_index = !0;
            dp.arg_index = !0;

            if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                let mut np = cp;
                while *np >= b'0' as c_char && *np <= b'9' as c_char {
                    np = np.offset(1);
                }
                if *np == b'$' as c_char {
                    let mut n = 0;
                    np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        n = xsum(
                            if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                            (*np - b'0' as c_char) as usize,
                        );
                        np = np.offset(1);
                    }
                    if n == 0 || n == usize::MAX {
                        break;
                    }
                    arg_index = n - 1;
                    cp = np.offset(1);
                }
            }

            loop {
                match *cp {
                    b'\'' => { dp.flags |= 1; cp = cp.offset(1); }
                    b'-' => { dp.flags |= 2; cp = cp.offset(1); }
                    b'+' => { dp.flags |= 4; cp = cp.offset(1); }
                    b' ' => { dp.flags |= 8; cp = cp.offset(1); }
                    b'#' => { dp.flags |= 16; cp = cp.offset(1); }
                    b'0' => { dp.flags |= 32; cp = cp.offset(1); }
                    b'I' => { dp.flags |= 64; cp = cp.offset(1); }
                    _ => break,
                }
            }

            if *cp == b'*' as c_char {
                dp.width_start = cp;
                cp = cp.offset(1);
                dp.width_end = cp;
                max_width_length = max_width_length.max(1);

                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let mut np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        np = np.offset(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n = 0;
                        np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            n = xsum(
                                if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                                (*np - b'0' as c_char) as usize,
                            );
                            np = np.offset(1);
                        }
                        if n == 0 || n == usize::MAX {
                            break;
                        }
                        dp.width_arg_index = n - 1;
                        cp = np.offset(1);
                    }
                }

                if dp.width_arg_index == !0 {
                    arg_posn += 1;
                    dp.width_arg_index = arg_posn;
                    if dp.width_arg_index == !0 {
                        break;
                    }
                }

                let n = dp.width_arg_index;
                if n >= a_allocated {
                    a_allocated = if a_allocated <= usize::MAX / 2 {
                        a_allocated * 2
                    } else {
                        usize::MAX
                    };
                    if a_allocated <= n {
                        a_allocated = xsum(n, 1);
                    }
                    let memory_size = a_allocated * mem::size_of::<Argument>();
                    if memory_size == usize::MAX {
                        break;
                    }
                    let memory = if (*a).arg != (*a).direct_alloc_arg.as_mut_ptr() {
                        libc::realloc((*a).arg as *mut c_void, memory_size)
                    } else {
                        libc::malloc(memory_size)
                    } as *mut Argument;
                    if memory.is_null() {
                        break;
                    }
                    if (*a).arg == (*a).direct_alloc_arg.as_mut_ptr() {
                        ptr::copy_nonoverlapping(
                            (*a).arg,
                            memory,
                            (*a).count,
                        );
                    }
                    (*a).arg = memory;
                }

                while (*a).count <= n {
                    (*a).arg.offset((*a).count as isize).write(Argument {
                        type_: ArgType::None,
                        a: ArgumentData { a_int: 0 },
                    });
                    (*a).count += 1;
                }

                let arg = &mut *(*a).arg.offset(n as isize);
                if arg.type_ == ArgType::None {
                    arg.type_ = ArgType::Int;
                } else if arg.type_ != ArgType::Int {
                    break;
                }
            } else if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                dp.width_start = cp;
                while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    cp = cp.offset(1);
                }
                dp.width_end = cp;
                let width_length = dp.width_end.offset_from(dp.width_start) as usize;
                max_width_length = max_width_length.max(width_length);
            }

            if *cp == b'.' as c_char {
                cp = cp.offset(1);
                if *cp == b'*' as c_char {
                    dp.precision_start = cp.offset(-1);
                    cp = cp.offset(1);
                    dp.precision_end = cp;
                    max_precision_length = max_precision_length.max(2);

                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let mut np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            np = np.offset(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n = 0;
                            np = cp;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                n = xsum(
                                    if n <= usize::MAX / 10 { n * 10 } else { usize::MAX },
                                    (*np - b'0' as c_char) as usize,
                                );
                                np = np.offset(1);
                            }
                            if n == 0 || n == usize::MAX {
                                break;
                            }
                            dp.precision_arg_index = n - 1;
                            cp = np.offset(1);
                        }
                    }

                    if dp.precision_arg_index == !0 {
                        arg_posn += 1;
                        dp.precision_arg_index = arg_posn;
                        if dp.precision_arg_index == !0 {
                            break;
                        }
                    }

                    let n = dp.precision_arg_index;
                    if n >= a_allocated {
                        a_allocated = if a_allocated <= usize::MAX / 2 {
                            a_allocated * 2
                        } else {
                            usize::MAX
                        };
                        if a_allocated <= n {
                            a_allocated = xsum(n, 1);
                        }
                        let memory_size = a_allocated * mem::size_of::<Argument>();
                        if memory_size == usize::MAX {
                            break;
                        }
                        let memory = if (*a).arg != (*a).direct_alloc_arg.as_mut_ptr() {
                            libc::realloc((*a).arg as *mut c_void, memory_size)
                        } else {
                            libc::malloc(memory_size)
                        } as *mut Argument;
                        if memory.is_null() {
                            break;
                        }
                        if (*a).arg == (*a).direct_alloc_arg.as_mut_ptr() {
                            ptr::copy_nonoverlapping(
                                (*a).arg,
                                memory,
                                (*a).count,
                            );
                        }
                        (*a).arg = memory;
                    }

                    while (*a).count <= n {
                        (*a).arg.offset((*a).count as isize).write(Argument {
                            type_: ArgType::None,
                            a: ArgumentData { a_int: 0 },
                        });
                        (*a).count += 1;
                    }

                    let arg = &mut *(*a).arg.offset(n as isize);
                    if arg.type_ == ArgType::None {
                        arg.type_ = ArgType::Int;
                    } else if arg.type_ != ArgType::Int {
                        break;
                    }
                } else {
                    dp.precision_start = cp.offset(-1);
                    while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        cp = cp.offset(1);
                    }
                    dp.precision_end = cp;
                    let precision_length = dp.precision_end.offset_from(dp.precision_start) as usize;
                    max_precision_length = max_precision_length.max(precision_length);
                }
            }

            let mut type_ = ArgType::None;
            let mut flags = 0;

            loop {
                match *cp {
                    b'h' => { flags |= (1 << (flags & 1)); cp = cp.offset(1); }
                    b'L' => { flags |= 4; cp = cp.offset(1); }
                    b'l' => { flags += 8; cp = cp.offset(1); }
                    b'j' => {
                        if mem::size_of::<i64>() > mem::size_of::<c_long>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<c_int>() {
                            flags += 8;
                        }
                        cp = cp.offset(1);
                    }
                    b'z' | b'Z' => {
                        if mem::size_of::<usize>() > mem::size_of::<c_long>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<c_int>() {
                            flags += 8;
                        }
                        cp = cp.offset(1);
                    }
                    b't' => {
                        if mem::size_of::<isize>() > mem::size_of::<c_long>() {
                            flags += 16;
                        } else if mem::size_of::<isize>() > mem::size_of::<c_int>() {
                            flags += 8;
                        }
                        cp = cp.offset(1);
                    }
                    _ => break,
                }
            }

            let c = *cp;
            cp = cp.offset(1);

            match c {
                b'd' | b'i' => {
                    if flags >= 16 || flags & 4 != 0 {
                        type_ = ArgType::Longlongint;
                    } else if flags >= 8 {
                        type_ = ArgType::Longint;
                    } else if flags & 2 != 0 {
                        type_ = ArgType::Schar;
                    } else if flags & 1 != 0 {
                        type_ = ArgType::Short;
                    } else {
                        type_ = ArgType::Int;
                    }
                }
                b'o' | b'u' | b'x' | b'X' => {
                    if flags >= 16 || flags & 4 != 0 {
                        type_ = ArgType::Ulonglongint;
                    } else if flags >= 8 {
                        type_ = ArgType::Ulongint;
                    } else if flags & 2 != 0 {
                        type_ = ArgType::Uchar;
                    } else if flags & 1 != 0 {
                        type_ = ArgType::Ushort;
                    } else {
                        type_ = ArgType::Uint;
                    }
                }
                b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => {
                    if flags >= 16 || flags & 4 != 0 {
                        type_ = ArgType::Longdouble;
                    } else {
                        type_ = ArgType::Double;
                    }
                }
                b'c' => {
                    if flags >= 8 {
                        type_ = ArgType::WideChar;
                    } else {
                        type_ = ArgType::Char;
                    }
                }
                b'C' => {
                    type_ = ArgType::WideChar;
                }
                b's' => {
                    if flags >= 8 {
                        type_ = ArgType::WideString;
                    } else {
                        type_ = ArgType::String;
                    }
                }
                b'S' => {
                    type_ = ArgType::WideString;
                }
                b'p' => {
                    type_ = ArgType::Pointer;
                }
                b'n' => {
                    if flags >= 16 || flags & 4 != 0 {
                        type_ = ArgType::CountLonglongintPointer;
                    } else if flags >= 8 {
                        type_ = ArgType::CountLongintPointer;
                    } else if flags & 2 != 0 {
                        type_ = ArgType::CountScharPointer;
                    } else if flags & 1 != 0 {
                        type_ = ArgType::CountShortPointer;
                    } else {
                        type_ = ArgType::CountIntPointer;
                    }
                }
                b'%' => {
                    type_ =