use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::num::NonZeroUsize;

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
#[derive(Clone, Copy)]
pub union ArgumentValue {
    pub a_schar: i8,
    pub a_uchar: u8,
    pub a_short: i16,
    pub a_ushort: u16,
    pub a_int: i32,
    pub a_uint: u32,
    pub a_longint: i64,
    pub a_ulongint: u64,
    pub a_longlongint: i64,
    pub a_ulonglongint: u64,
    pub a_float: f32,
    pub a_double: f64,
    pub a_longdouble: f64,
    pub a_char: i32,
    pub a_wide_char: u32,
    pub a_string: *const c_char,
    pub a_wide_string: *const i32,
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut i8,
    pub a_count_short_pointer: *mut i16,
    pub a_count_int_pointer: *mut i32,
    pub a_count_longint_pointer: *mut i64,
    pub a_count_longlongint_pointer: *mut i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Argument {
    pub type_: ArgType,
    pub a: ArgumentValue,
}

#[repr(C)]
#[derive(Clone, Copy)]
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
pub struct CharDirectives {
    pub count: usize,
    pub dir: *mut CharDirective,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [CharDirective; 7],
}

#[repr(C)]
pub struct Arguments {
    pub count: usize,
    pub arg: *mut Argument,
    pub direct_alloc_arg: [Argument; 7],
}

fn xsum(size1: usize, size2: usize) -> usize {
    size1.checked_add(size2).unwrap_or(usize::MAX)
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
            cp = cp.add(1);

            if c != b'%' as c_char {
                continue;
            }

            let mut arg_index = !0;
            let dp = &mut *(*d).dir.add((*d).count);

            dp.dir_start = cp.sub(1);
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
                        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                        return -1;
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

            if *cp == b'*' {
                dp.width_start = cp;
                cp = cp.add(1);
                dp.width_end = cp;
                max_width_length = max_width_length.max(1);

                if *cp >= b'0' && *cp <= b'9' {
                    let mut np = cp;
                    while *np >= b'0' && *np <= b'9' {
                        np = np.add(1);
                    }
                    if *np == b'$' {
                        let mut n = 0;
                        np = cp;
                        while *np >= b'0' && *np <= b'9' {
                            n = xsum(
                                if n <= usize::MAX / 10 {
                                    n * 10
                                } else {
                                    usize::MAX
                                },
                                (*np - b'0') as usize,
                            );
                            np = np.add(1);
                        }
                        if n == 0 || n == usize::MAX {
                            (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                            (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                            return -1;
                        }
                        dp.width_arg_index = n - 1;
                        cp = np.add(1);
                    }
                }

                if dp.width_arg_index == !0 {
                    dp.width_arg_index = arg_posn;
                    arg_posn = arg_posn.wrapping_add(1);
                    if dp.width_arg_index == !0 {
                        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                        return -1;
                    }
                }

                let n = dp.width_arg_index;
                if n >= a_allocated {
                    a_allocated = a_allocated.saturating_mul(2).max(n + 1);
                    let new_size = a_allocated.checked_mul(mem::size_of::<Argument>())
                        .unwrap_or(usize::MAX);
                    if new_size == usize::MAX {
                        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                        return -1;
                    }

                    let new_arg = if (*a).arg == (*a).direct_alloc_arg.as_mut_ptr() {
                        let new_ptr = libc::malloc(new_size) as *mut Argument;
                        if new_ptr.is_null() {
                            (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                            (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                            return -1;
                        }
                        ptr::copy_nonoverlapping(
                            (*a).arg,
                            new_ptr,
                            (*a).count,
                        );
                        new_ptr
                    } else {
                        let new_ptr = libc::realloc(
                            (*a).arg as *mut c_void,
                            new_size,
                        ) as *mut Argument;
                        if new_ptr.is_null() {
                            (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                            (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                            return -1;
                        }
                        new_ptr
                    };
                    (*a).arg = new_arg;
                }

                while (*a).count <= n {
                    let idx = (*a).count;
                    (*a).count += 1;
                    (*a).arg.add(idx).write(Argument {
                        type_: ArgType::None,
                        a: ArgumentValue { a_int: 0 },
                    });
                }

                let arg = &mut *(*a).arg.add(n);
                if arg.type_ == ArgType::None {
                    arg.type_ = ArgType::Int;
                } else if arg.type_ != ArgType::Int {
                    (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                    (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                    return -1;
                }
            } else if *cp >= b'0' && *cp <= b'9' {
                dp.width_start = cp;
                while *cp >= b'0' && *cp <= b'9' {
                    cp = cp.add(1);
                }
                dp.width_end = cp;
                let width_length = dp.width_end.offset_from(dp.width_start) as usize;
                max_width_length = max_width_length.max(width_length);
            }

            if *cp == b'.' {
                cp = cp.add(1);
                if *cp == b'*' {
                    dp.precision_start = cp.sub(1);
                    cp = cp.add(1);
                    dp.precision_end = cp;
                    max_precision_length = max_precision_length.max(2);

                    if *cp >= b'0' && *cp <= b'9' {
                        let mut np = cp;
                        while *np >= b'0' && *np <= b'9' {
                            np = np.add(1);
                        }
                        if *np == b'$' {
                            let mut n = 0;
                            np = cp;
                            while *np >= b'0' && *np <= b'9' {
                                n = xsum(
                                    if n <= usize::MAX / 10 {
                                        n * 10
                                    } else {
                                        usize::MAX
                                    },
                                    (*np - b'0') as usize,
                                );
                                np = np.add(1);
                            }
                            if n == 0 || n == usize::MAX {
                                (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                                (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                                return -1;
                            }
                            dp.precision_arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }

                    if dp.precision_arg_index == !0 {
                        dp.precision_arg_index = arg_posn;
                        arg_posn = arg_posn.wrapping_add(1);
                        if dp.precision_arg_index == !0 {
                            (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                            (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                            return -1;
                        }
                    }

                    let n = dp.precision_arg_index;
                    if n >= a_allocated {
                        a_allocated = a_allocated.saturating_mul(2).max(n + 1);
                        let new_size = a_allocated.checked_mul(mem::size_of::<Argument>())
                            .unwrap_or(usize::MAX);
                        if new_size == usize::MAX {
                            (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                            (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                            return -1;
                        }

                        let new_arg = if (*a).arg == (*a).direct_alloc_arg.as_mut_ptr() {
                            let new_ptr = libc::malloc(new_size) as *mut Argument;
                            if new_ptr.is_null() {
                                (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                                (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                                return -1;
                            }
                            ptr::copy_nonoverlapping(
                                (*a).arg,
                                new_ptr,
                                (*a).count,
                            );
                            new_ptr
                        } else {
                            let new_ptr = libc::realloc(
                                (*a).arg as *mut c_void,
                                new_size,
                            ) as *mut Argument;
                            if new_ptr.is_null() {
                                (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                                (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                                return -1;
                            }
                            new_ptr
                        };
                        (*a).arg = new_arg;
                    }

                    while (*a).count <= n {
                        let idx = (*a).count;
                        (*a).count += 1;
                        (*a).arg.add(idx).write(Argument {
                            type_: ArgType::None,
                            a: ArgumentValue { a_int: 0 },
                        });
                    }

                    let arg = &mut *(*a).arg.add(n);
                    if arg.type_ == ArgType::None {
                        arg.type_ = ArgType::Int;
                    } else if arg.type_ != ArgType::Int {
                        (*a).arg = (*a).direct_alloc_arg.as_mut_ptr();
                        (*d).dir = (*d).direct_alloc_dir.as_mut_ptr();
                        return -1;
                    }
                } else {
                    dp.precision_start = cp.sub(1);
                    while *cp >= b'0' && *cp <= b'9' {
                        cp = cp.add(1);
                    }
                    dp.precision_end = cp;
                    let precision_length = dp.precision_end.offset_from(dp.precision_start) as usize;
                    max_precision_length = max_precision_length.max(precision_length);
                }
            }

            let mut flags = 0;
            loop {
                match *cp {
                    b'h' => {
                        flags |= (1 << (flags & 1));
                        cp = cp.add(1);
                    }
                    b'L' => {
                        flags |= 4;
                        cp = cp.add(1);
                    }
                    b'l' => {
                        flags += 8;
                        cp = cp.add(1);
                    }
                    b'j' => {
                        if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b'z' | b'Z' => {
                        if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = cp.add(1);
                    }
                    b't' => {
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

            let c = *cp;
            cp = cp.add(1);

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
                    if flags >= 16 || (