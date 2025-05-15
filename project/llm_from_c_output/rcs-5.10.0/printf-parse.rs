use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::os::raw::{c_char, c_int, c_void};

const FLAG_GROUP: c_int = 1;
const FLAG_LEFT: c_int = 2;
const FLAG_SHOWSIGN: c_int = 4;
const FLAG_SPACE: c_int = 8;
const FLAG_ALT: c_int = 16;
const FLAG_ZERO: c_int = 32;

const ARG_NONE: usize = !0;

const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;
const N_DIRECT_ALLOC_ARGUMENTS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgType {
    None,
    Int,
    LongInt,
    LongLongInt,
    UInt,
    ULongInt,
    ULongLongInt,
    SChar,
    UChar,
    Short,
    UShort,
    Double,
    LongDouble,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountIntPointer,
    CountShortPointer,
    CountScharPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
    U8String,
    U16String,
    U32String,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub type_: ArgType,
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub count: usize,
    pub arg: Vec<Argument>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            count: 0,
            arg: Vec::with_capacity(N_DIRECT_ALLOC_ARGUMENTS),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct CharDirectives {
    pub count: usize,
    pub dir: Vec<CharDirective>,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [CharDirective; N_DIRECT_ALLOC_DIRECTIVES],
}

impl Default for CharDirectives {
    fn default() -> Self {
        CharDirectives {
            count: 0,
            dir: Vec::new(),
            max_width_length: 0,
            max_precision_length: 0,
            direct_alloc_dir: unsafe { mem::zeroed() },
        }
    }
}

pub fn printf_parse(
    format: *const c_char,
    d: &mut CharDirectives,
    a: &mut Arguments,
) -> c_int {
    unsafe {
        let mut cp = format;
        let mut arg_posn = 0;
        let mut d_allocated = N_DIRECT_ALLOC_DIRECTIVES;
        let mut a_allocated = N_DIRECT_ALLOC_ARGUMENTS;
        let mut max_width_length = 0;
        let mut max_precision_length = 0;

        d.count = 0;
        d.dir = Vec::with_capacity(d_allocated);

        a.count = 0;
        a.arg = Vec::with_capacity(a_allocated);

        macro_rules! register_arg {
            ($index:expr, $type:expr) => {
                let n = $index;
                if n >= a_allocated {
                    a_allocated = if a_allocated == 0 {
                        1
                    } else {
                        a_allocated * 2
                    };
                    if a_allocated <= n {
                        a_allocated = n + 1;
                    }
                    a.arg.reserve(a_allocated);
                }
                while a.count <= n {
                    a.arg.push(Argument { type_: ArgType::None });
                    a.count += 1;
                }
                if a.arg[n].type_ == ArgType::None {
                    a.arg[n].type_ = $type;
                } else if a.arg[n].type_ != $type {
                    return -1;
                }
            };
        }

        while *cp != 0 {
            let c = *cp;
            cp = cp.offset(1);
            if c == b'%' as c_char {
                let mut arg_index = ARG_NONE;
                let dp = if d.count < d.direct_alloc_dir.len() {
                    &mut d.direct_alloc_dir[d.count]
                } else {
                    d.dir.push(CharDirective {
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
                    d.dir.last_mut().unwrap()
                };

                dp.dir_start = cp.offset(-1);
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
                        np = np.offset(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n = 0;
                        let mut tmp = cp;
                        while tmp < np {
                            n = n * 10 + (*tmp - b'0' as c_char) as usize;
                            tmp = tmp.offset(1);
                        }
                        if n == 0 {
                            return -1;
                        }
                        arg_index = n - 1;
                        cp = np.offset(1);
                    }
                }

                loop {
                    match *cp {
                        b'\'' => {
                            dp.flags |= FLAG_GROUP;
                            cp = cp.offset(1);
                        }
                        b'-' => {
                            dp.flags |= FLAG_LEFT;
                            cp = cp.offset(1);
                        }
                        b'+' => {
                            dp.flags |= FLAG_SHOWSIGN;
                            cp = cp.offset(1);
                        }
                        b' ' => {
                            dp.flags |= FLAG_SPACE;
                            cp = cp.offset(1);
                        }
                        b'#' => {
                            dp.flags |= FLAG_ALT;
                            cp = cp.offset(1);
                        }
                        b'0' => {
                            dp.flags |= FLAG_ZERO;
                            cp = cp.offset(1);
                        }
                        _ => break,
                    }
                }

                if *cp == b'*' as c_char {
                    dp.width_start = cp;
                    cp = cp.offset(1);
                    dp.width_end = cp;
                    if max_width_length < 1 {
                        max_width_length = 1;
                    }

                    if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        let mut np = cp;
                        while *np >= b'0' as c_char && *np <= b'9' as c_char {
                            np = np.offset(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n = 0;
                            let mut tmp = cp;
                            while tmp < np {
                                n = n * 10 + (*tmp - b'0' as c_char) as usize;
                                tmp = tmp.offset(1);
                            }
                            if n == 0 {
                                return -1;
                            }
                            dp.width_arg_index = n - 1;
                            cp = np.offset(1);
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
                        cp = cp.offset(1);
                    }
                    dp.width_end = cp;
                    let width_length = dp.width_end as usize - dp.width_start as usize;
                    if max_width_length < width_length {
                        max_width_length = width_length;
                    }
                }

                if *cp == b'.' as c_char {
                    cp = cp.offset(1);
                    if *cp == b'*' as c_char {
                        dp.precision_start = cp.offset(-1);
                        cp = cp.offset(1);
                        dp.precision_end = cp;
                        if max_precision_length < 2 {
                            max_precision_length = 2;
                        }

                        if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            let mut np = cp;
                            while *np >= b'0' as c_char && *np <= b'9' as c_char {
                                np = np.offset(1);
                            }
                            if *np == b'$' as c_char {
                                let mut n = 0;
                                let mut tmp = cp;
                                while tmp < np {
                                    n = n * 10 + (*tmp - b'0' as c_char) as usize;
                                    tmp = tmp.offset(1);
                                }
                                if n == 0 {
                                    return -1;
                                }
                                dp.precision_arg_index = n - 1;
                                cp = np.offset(1);
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
                        dp.precision_start = cp.offset(-1);
                        while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            cp = cp.offset(1);
                        }
                        dp.precision_end = cp;
                        let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                        if max_precision_length < precision_length {
                            max_precision_length = precision_length;
                        }
                    }
                }

                let mut flags = 0;
                loop {
                    match *cp {
                        b'h' => {
                            flags |= 1 << (flags & 1);
                            cp = cp.offset(1);
                        }
                        b'L' => {
                            flags |= 4;
                            cp = cp.offset(1);
                        }
                        b'l' => {
                            flags += 8;
                            cp = cp.offset(1);
                        }
                        b'j' => {
                            if mem::size_of::<i64>() > mem::size_of::<isize>() {
                                flags += 16;
                            } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = cp.offset(1);
                        }
                        b'z' | b'Z' => {
                            if mem::size_of::<usize>() > mem::size_of::<isize>() {
                                flags += 16;
                            } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = cp.offset(1);
                        }
                        b't' => {
                            if mem::size_of::<isize>() > mem::size_of::<isize>() {
                                flags += 16;
                            } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                                flags += 8;
                            }
                            cp = cp.offset(1);
                        }
                        _ => break,
                    }
                }

                let c = *cp;
                cp = cp.offset(1);
                let type_ = match c {
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
                            ArgType::WideChar
                        } else {
                            ArgType::Char
                        }
                    }
                    b's' => {
                        if flags >= 8 {
                            ArgType::WideString
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
                            ArgType::CountScharPointer
                        } else if (flags & 1) != 0 {
                            ArgType::CountShortPointer
                        } else {
                            ArgType::CountIntPointer
                        }
                    }
                    b'%' => ArgType::None,
                    _ => return -1,
                };

                if type_ != ArgType::None {
                    dp.arg_index = arg_index;
                    if dp.arg_index == ARG_NONE {
                        dp.arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.arg_index == ARG_NONE {
                            return -1;
                        }
                    }
                    register_arg!(dp.arg_index, type_);
                }
                dp.conversion = c;
                dp.dir_end = cp;

                d.count += 1;
                if d.count >= d_allocated {
                    d_allocated *= 2;
                    d.dir.reserve(d_allocated);
                }
            }
        }

        d.max_width_length = max_width_length;
        d.max_precision_length = max_precision_length;
        0
    }
}