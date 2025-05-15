use std::mem;
use std::ptr;
use std::os::raw::{c_char, c_int, c_void, c_long, c_ulong, c_uchar, c_short, c_ushort, c_uint, c_longlong, c_ulonglong, c_float, c_double};
use std::ffi::CStr;
use std::num::NonZeroUsize;

type ptrdiff_t = c_long;
type size_t = c_ulong;
type wchar_t = c_int;
type wint_t = c_uint;
type arg_type = c_uint;

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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
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
    pub a_wide_char: wint_t,
    pub a_string: *const c_char,
    pub a_wide_string: *const wchar_t,
    pub a_pointer: *mut c_void,
    pub a_count_schar_pointer: *mut c_char,
    pub a_count_short_pointer: *mut c_short,
    pub a_count_int_pointer: *mut c_int,
    pub a_count_longint_pointer: *mut c_long,
    pub a_count_longlongint_pointer: *mut c_longlong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Argument {
    pub type_: ArgType,
    pub data: ArgumentData,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Arguments {
    pub count: size_t,
    pub arg: *mut Argument,
    pub direct_alloc_arg: [Argument; 7],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CharDirective {
    pub dir_start: *const c_char,
    pub dir_end: *const c_char,
    pub flags: c_int,
    pub width_start: *const c_char,
    pub width_end: *const c_char,
    pub width_arg_index: size_t,
    pub precision_start: *const c_char,
    pub precision_end: *const c_char,
    pub precision_arg_index: size_t,
    pub conversion: c_char,
    pub arg_index: size_t,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CharDirectives {
    pub count: size_t,
    pub dir: *mut CharDirective,
    pub max_width_length: size_t,
    pub max_precision_length: size_t,
    pub direct_alloc_dir: [CharDirective; 7],
}

type intmax_t = c_long;

fn xsum(size1: size_t, size2: size_t) -> size_t {
    size1.checked_add(size2).unwrap_or(size_t::MAX)
}

pub fn printf_parse(
    format: &CStr,
    d: &mut CharDirectives,
    a: &mut Arguments,
) -> c_int {
    let mut cp = format.as_ptr();
    let mut arg_posn: size_t = 0;
    let mut d_allocated = 7;
    let mut a_allocated = 7;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;

    d.count = 0;
    d.dir = d.direct_alloc_dir.as_mut_ptr();
    a.count = 0;
    a.arg = a.direct_alloc_arg.as_mut_ptr();

    while unsafe { *cp != 0 } {
        let c = unsafe { *cp };
        cp = unsafe { cp.offset(1) };

        if c != b'%' as c_char {
            continue;
        }

        let mut arg_index = !0;
        let dp = unsafe { &mut *d.dir.add(d.count) };

        dp.dir_start = unsafe { cp.offset(-1) };
        dp.flags = 0;
        dp.width_start = ptr::null();
        dp.width_end = ptr::null();
        dp.width_arg_index = !0;
        dp.precision_start = ptr::null();
        dp.precision_end = ptr::null();
        dp.precision_arg_index = !0;
        dp.arg_index = !0;

        if unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
            let mut np = cp;
            while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                np = unsafe { np.offset(1) };
            }

            if unsafe { *np == b'$' as c_char } {
                let mut n = 0;
                let mut np = cp;
                while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                    n = xsum(
                        if n <= size_t::MAX / 10 {
                            n * 10
                        } else {
                            size_t::MAX
                        },
                        (unsafe { *np } - b'0' as c_char) as size_t,
                    );
                    np = unsafe { np.offset(1) };
                }

                if n == 0 || n == size_t::MAX {
                    return -1;
                }

                arg_index = n - 1;
                cp = unsafe { np.offset(1) };
            }
        }

        loop {
            match unsafe { *cp } {
                b'\'' => {
                    dp.flags |= 1;
                    cp = unsafe { cp.offset(1) };
                }
                b'-' => {
                    dp.flags |= 2;
                    cp = unsafe { cp.offset(1) };
                }
                b'+' => {
                    dp.flags |= 4;
                    cp = unsafe { cp.offset(1) };
                }
                b' ' => {
                    dp.flags |= 8;
                    cp = unsafe { cp.offset(1) };
                }
                b'#' => {
                    dp.flags |= 16;
                    cp = unsafe { cp.offset(1) };
                }
                b'0' => {
                    dp.flags |= 32;
                    cp = unsafe { cp.offset(1) };
                }
                b'I' => {
                    dp.flags |= 64;
                    cp = unsafe { cp.offset(1) };
                }
                _ => break,
            }
        }

        if unsafe { *cp == b'*' as c_char } {
            dp.width_start = cp;
            cp = unsafe { cp.offset(1) };
            dp.width_end = cp;

            max_width_length = max_width_length.max(1);

            if unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
                let mut np = cp;
                while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                    np = unsafe { np.offset(1) };
                }

                if unsafe { *np == b'$' as c_char } {
                    let mut n = 0;
                    let mut np = cp;
                    while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                        n = xsum(
                            if n <= size_t::MAX / 10 {
                                n * 10
                            } else {
                                size_t::MAX
                            },
                            (unsafe { *np } - b'0' as c_char) as size_t,
                        );
                        np = unsafe { np.offset(1) };
                    }

                    if n == 0 || n == size_t::MAX {
                        return -1;
                    }

                    dp.width_arg_index = n - 1;
                    cp = unsafe { np.offset(1) };
                }
            }

            if dp.width_arg_index == !0 {
                arg_posn = arg_posn.wrapping_add(1);
                dp.width_arg_index = arg_posn;
                if dp.width_arg_index == !0 {
                    return -1;
                }
            }

            let n = dp.width_arg_index;
            if n >= a_allocated {
                a_allocated = if a_allocated <= size_t::MAX / 2 {
                    a_allocated * 2
                } else {
                    size_t::MAX
                };
                if a_allocated <= n {
                    a_allocated = xsum(n, 1);
                }

                let new_size = a_allocated.checked_mul(mem::size_of::<Argument>() as size_t)
                    .unwrap_or(size_t::MAX);
                if new_size == size_t::MAX {
                    return -1;
                }

                let new_arg = if a.arg != a.direct_alloc_arg.as_mut_ptr() {
                    unsafe {
                        libc::realloc(a.arg as *mut c_void, new_size) as *mut Argument
                    }
                } else {
                    unsafe {
                        libc::malloc(new_size) as *mut Argument
                    }
                };

                if new_arg.is_null() {
                    return -1;
                }

                if a.arg == a.direct_alloc_arg.as_mut_ptr() {
                    unsafe {
                        ptr::copy_nonoverlapping(
                            a.arg,
                            new_arg,
                            a.count as usize,
                        );
                    }
                }

                a.arg = new_arg;
            }

            while a.count <= n {
                unsafe {
                    (*a.arg.add(a.count)).type_ = ArgType::None;
                }
                a.count += 1;
            }

            let arg = unsafe { &mut *a.arg.add(n) };
            if arg.type_ == ArgType::None {
                arg.type_ = ArgType::Int;
            } else if arg.type_ != ArgType::Int {
                return -1;
            }
        } else if unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
            dp.width_start = cp;
            while unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
                cp = unsafe { cp.offset(1) };
            }
            dp.width_end = cp;

            let width_length = unsafe { dp.width_end.offset_from(dp.width_start) as size_t };
            max_width_length = max_width_length.max(width_length);
        }

        if unsafe { *cp == b'.' as c_char } {
            cp = unsafe { cp.offset(1) };

            if unsafe { *cp == b'*' as c_char } {
                dp.precision_start = unsafe { cp.offset(-1) };
                cp = unsafe { cp.offset(1) };
                dp.precision_end = cp;

                max_precision_length = max_precision_length.max(2);

                if unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
                    let mut np = cp;
                    while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                        np = unsafe { np.offset(1) };
                    }

                    if unsafe { *np == b'$' as c_char } {
                        let mut n = 0;
                        let mut np = cp;
                        while unsafe { *np >= b'0' as c_char && *np <= b'9' as c_char } {
                            n = xsum(
                                if n <= size_t::MAX / 10 {
                                    n * 10
                                } else {
                                    size_t::MAX
                                },
                                (unsafe { *np } - b'0' as c_char) as size_t,
                            );
                            np = unsafe { np.offset(1) };
                        }

                        if n == 0 || n == size_t::MAX {
                            return -1;
                        }

                        dp.precision_arg_index = n - 1;
                        cp = unsafe { np.offset(1) };
                    }
                }

                if dp.precision_arg_index == !0 {
                    arg_posn = arg_posn.wrapping_add(1);
                    dp.precision_arg_index = arg_posn;
                    if dp.precision_arg_index == !0 {
                        return -1;
                    }
                }

                let n = dp.precision_arg_index;
                if n >= a_allocated {
                    a_allocated = if a_allocated <= size_t::MAX / 2 {
                        a_allocated * 2
                    } else {
                        size_t::MAX
                    };
                    if a_allocated <= n {
                        a_allocated = xsum(n, 1);
                    }

                    let new_size = a_allocated.checked_mul(mem::size_of::<Argument>() as size_t)
                        .unwrap_or(size_t::MAX);
                    if new_size == size_t::MAX {
                        return -1;
                    }

                    let new_arg = if a.arg != a.direct_alloc_arg.as_mut_ptr() {
                        unsafe {
                            libc::realloc(a.arg as *mut c_void, new_size) as *mut Argument
                        }
                    } else {
                        unsafe {
                            libc::malloc(new_size) as *mut Argument
                        }
                    };

                    if new_arg.is_null() {
                        return -1;
                    }

                    if a.arg == a.direct_alloc_arg.as_mut_ptr() {
                        unsafe {
                            ptr::copy_nonoverlapping(
                                a.arg,
                                new_arg,
                                a.count as usize,
                            );
                        }
                    }

                    a.arg = new_arg;
                }

                while a.count <= n {
                    unsafe {
                        (*a.arg.add(a.count)).type_ = ArgType::None;
                    }
                    a.count += 1;
                }

                let arg = unsafe { &mut *a.arg.add(n) };
                if arg.type_ == ArgType::None {
                    arg.type_ = ArgType::Int;
                } else if arg.type_ != ArgType::Int {
                    return -1;
                }
            } else {
                dp.precision_start = unsafe { cp.offset(-1) };
                while unsafe { *cp >= b'0' as c_char && *cp <= b'9' as c_char } {
                    cp = unsafe { cp.offset(1) };
                }
                dp.precision_end = cp;

                let precision_length = unsafe { dp.precision_end.offset_from(dp.precision_start) as size_t };
                max_precision_length = max_precision_length.max(precision_length);
            }
        }

        let mut type_ = ArgType::None;
        let mut flags = 0;

        loop {
            match unsafe { *cp } {
                b'h' => {
                    flags |= 1 << (flags & 1);
                    cp = unsafe { cp.offset(1) };
                }
                b'L' => {
                    flags |= 4;
                    cp = unsafe { cp.offset(1) };
                }
                b'l' => {
                    flags += 8;
                    cp = unsafe { cp.offset(1) };
                }
                b'j' => {
                    if mem::size_of::<intmax_t>() > mem::size_of::<c_long>() {
                        flags += 16;
                    } else if mem::size_of::<intmax_t>() > mem::size_of::<c_int>() {
                        flags += 8;
                    }
                    cp = unsafe { cp.offset(1) };
                }
                b'z' | b'Z' => {
                    if mem::size_of::<size_t>() > mem::size_of::<c_long>() {
                        flags += 16;
                    } else if mem::size_of::<size_t>() > mem::size_of::<c_int>() {
                        flags += 8;
                    }
                    cp = unsafe { cp.offset(1) };
                }
                b't' => {
                    if mem::size_of::<ptrdiff_t>() > mem::size_of::<c_long>() {
                        flags += 16;
                    } else if mem::size_of::<ptrdiff_t>() > mem::size_of::<c_int>() {
                        flags += 8;
                    }
                    cp = unsafe { cp.offset(1) };
                }
                _ => break,
            }
        }

        let c = unsafe { *cp };
        cp = unsafe { cp.offset(1) };

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
           