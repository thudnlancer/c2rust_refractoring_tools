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
const FLAG_GROUP: u8 = 1;       // ' flag
const FLAG_LEFT: u8 = 2;        // - flag
const FLAG_SHOWSIGN: u8 = 4;    // + flag
const FLAG_SPACE: u8 = 8;       // space flag
const FLAG_ALT: u8 = 16;        // # flag
const FLAG_ZERO: u8 = 32;
#[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
const FLAG_LOCALIZED: u8 = 64;  // I flag, uses localized digits

// arg_index value indicating that no argument is consumed.
const ARG_NONE: usize = usize::MAX;

// Number of directly allocated directives (no malloc() needed).
const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;

#[derive(Debug, Clone, Copy)]
pub enum ArgType {
    None,
    Int,
    UInt,
    LongInt,
    ULongInt,
    LongLongInt,
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
    CountSCharPointer,
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
    pub args: Vec<Argument>,
    pub direct_alloc_arg: [Argument; N_DIRECT_ALLOC_DIRECTIVES],
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            args: Vec::new(),
            direct_alloc_arg: [Argument { type_: ArgType::None }; N_DIRECT_ALLOC_DIRECTIVES],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub dir_start: *const c_char,
    pub dir_end: *const c_char,
    pub flags: u8,
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
pub struct Directives {
    pub count: usize,
    pub dir: Vec<Directive>,
    pub max_width_length: usize,
    pub max_precision_length: usize,
    pub direct_alloc_dir: [Directive; N_DIRECT_ALLOC_DIRECTIVES],
}

impl Default for Directives {
    fn default() -> Self {
        Directives {
            count: 0,
            dir: Vec::new(),
            max_width_length: 0,
            max_precision_length: 0,
            direct_alloc_dir: unsafe { mem::zeroed() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintfParser {
    directives: Directives,
    arguments: Arguments,
}

impl PrintfParser {
    pub fn new() -> Self {
        PrintfParser {
            directives: Directives::default(),
            arguments: Arguments::default(),
        }
    }

    pub fn parse(&mut self, format: &CStr) -> Result<(), String> {
        let mut cp = format.as_ptr();
        let mut arg_posn = 0;
        let mut d_allocated = N_DIRECT_ALLOC_DIRECTIVES;
        let mut a_allocated = N_DIRECT_ALLOC_DIRECTIVES;
        let mut max_width_length = 0;
        let mut max_precision_length = 0;

        self.directives.count = 0;
        self.directives.dir = Vec::new();
        self.directives.dir.extend_from_slice(&self.directives.direct_alloc_dir);

        self.arguments.args = Vec::new();
        self.arguments.args.extend_from_slice(&self.arguments.direct_alloc_arg);

        macro_rules! register_arg {
            ($index:expr, $type:expr) => {
                let n = $index;
                if n >= a_allocated {
                    a_allocated = if a_allocated.checked_mul(2).is_none() {
                        n.checked_add(1).ok_or("Argument index overflow")?
                    } else {
                        a_allocated * 2
                    };
                    
                    let new_size = a_allocated.checked_mul(mem::size_of::<Argument>())
                        .ok_or("Memory size overflow")?;
                    
                    let mut new_args = Vec::with_capacity(a_allocated);
                    new_args.extend_from_slice(&self.arguments.args);
                    while new_args.len() <= n {
                        new_args.push(Argument { type_: ArgType::None });
                    }
                    self.arguments.args = new_args;
                }
                
                if let ArgType::None = self.arguments.args[n].type_ {
                    self.arguments.args[n].type_ = $type;
                } else if self.arguments.args[n].type_ != $type {
                    return Err("Ambiguous type for positional argument".to_string());
                }
            };
        }

        unsafe {
            while *cp != 0 {
                let mut c = *cp;
                cp = cp.add(1);
                
                if c == b'%' as c_char {
                    let mut arg_index = ARG_NONE;
                    let dp = &mut self.directives.dir[self.directives.count];
                    
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
                                n = n.checked_mul(10)
                                    .and_then(|v| v.checked_add((*tmp - b'0' as c_char) as usize))
                                    .ok_or("Positional argument number overflow")?;
                                tmp = tmp.add(1);
                            }
                            if n == 0 {
                                return Err("Positional argument 0".to_string());
                            }
                            arg_index = n - 1;
                            cp = np.add(1);
                        }
                    }
                    
                    // Read the flags
                    loop {
                        match *cp {
                            b'\'' as c_char => {
                                dp.flags |= FLAG_GROUP;
                                cp = cp.add(1);
                            }
                            b'-' as c_char => {
                                dp.flags |= FLAG_LEFT;
                                cp = cp.add(1);
                            }
                            b'+' as c_char => {
                                dp.flags |= FLAG_SHOWSIGN;
                                cp = cp.add(1);
                            }
                            b' ' as c_char => {
                                dp.flags |= FLAG_SPACE;
                                cp = cp.add(1);
                            }
                            b'#' as c_char => {
                                dp.flags |= FLAG_ALT;
                                cp = cp.add(1);
                            }
                            b'0' as c_char => {
                                dp.flags |= FLAG_ZERO;
                                cp = cp.add(1);
                            }
                            #[cfg(all(target_os = "linux", not(target_env = "uclibc")))]
                            b'I' as c_char => {
                                dp.flags |= FLAG_LOCALIZED;
                                cp = cp.add(1);
                            }
                            _ => break,
                        }
                    }
                    
                    // Parse field width
                    if *cp == b'*' as c_char {
                        dp.width_start = cp;
                        cp = cp.add(1);
                        dp.width_end = cp;
                        max_width_length = std::cmp::max(max_width_length, 1);
                        
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
                                    n = n.checked_mul(10)
                                        .and_then(|v| v.checked_add((*tmp - b'0' as c_char) as usize))
                                        .ok_or("Positional argument number overflow")?;
                                    tmp = tmp.add(1);
                                }
                                if n == 0 {
                                    return Err("Positional argument 0".to_string());
                                }
                                dp.width_arg_index = n - 1;
                                cp = np.add(1);
                            }
                        }
                        
                        if dp.width_arg_index == ARG_NONE {
                            dp.width_arg_index = arg_posn;
                            arg_posn = arg_posn.checked_add(1)
                                .ok_or("Argument position overflow")?;
                        }
                        register_arg!(dp.width_arg_index, ArgType::Int);
                    } else if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                        dp.width_start = cp;
                        while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                            cp = cp.add(1);
                        }
                        dp.width_end = cp;
                        let width_length = dp.width_end as usize - dp.width_start as usize;
                        max_width_length = std::cmp::max(max_width_length, width_length);
                    }
                    
                    // Parse precision
                    if *cp == b'.' as c_char {
                        cp = cp.add(1);
                        if *cp == b'*' as c_char {
                            dp.precision_start = cp.sub(1);
                            cp = cp.add(1);
                            dp.precision_end = cp;
                            max_precision_length = std::cmp::max(max_precision_length, 2);
                            
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
                                        n = n.checked_mul(10)
                                            .and_then(|v| v.checked_add((*tmp - b'0' as c_char) as usize))
                                            .ok_or("Positional argument number overflow")?;
                                        tmp = tmp.add(1);
                                    }
                                    if n == 0 {
                                        return Err("Positional argument 0".to_string());
                                    }
                                    dp.precision_arg_index = n - 1;
                                    cp = np.add(1);
                                }
                            }
                            
                            if dp.precision_arg_index == ARG_NONE {
                                dp.precision_arg_index = arg_posn;
                                arg_posn = arg_posn.checked_add(1)
                                    .ok_or("Argument position overflow")?;
                            }
                            register_arg!(dp.precision_arg_index, ArgType::Int);
                        } else {
                            dp.precision_start = cp.sub(1);
                            while *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                                cp = cp.add(1);
                            }
                            dp.precision_end = cp;
                            let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                            max_precision_length = std::cmp::max(max_precision_length, precision_length);
                        }
                    }
                    
                    // Parse argument type/size specifiers
                    let mut type_ = ArgType::None;
                    let mut flags = 0u8;
                    
                    loop {
                        match *cp {
                            b'h' as c_char => {
                                flags |= 1 << (flags & 1);
                                cp = cp.add(1);
                            }
                            b'L' as c_char => {
                                flags |= 4;
                                cp = cp.add(1);
                            }
                            b'l' as c_char => {
                                flags += 8;
                                cp = cp.add(1);
                            }
                            b'j' as c_char => {
                                if mem::size_of::<i64>() > mem::size_of::<isize>() {
                                    flags += 16;
                                } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                                    flags += 8;
                                }
                                cp = cp.add(1);
                            }
                            b'z' as c_char | b'Z' as c_char => {
                                if mem::size_of::<usize>() > mem::size_of::<isize>() {
                                    flags += 16;
                                } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                                    flags += 8;
                                }
                                cp = cp.add(1);
                            }
                            b't' as c_char => {
                                if mem::size_of::<isize>() > mem::size_of::<isize>() {
                                    flags += 16;
                                } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                                    flags += 8;
                                }
                                cp = cp.add(1);
                            }
                            #[cfg(target_os = "macos")]
                            b'q' as c_char => {
                                if 8 > mem::size_of::<isize>() {
                                    flags += 16;
                                } else {
                                    flags += 8;
                                }
                                cp = cp.add(1);
                            }
                            #[cfg(target_os = "windows")]
                            b'I' as c_char if *cp.add(1) == b'6' as c_char && *cp.add(2) == b'4' as c_char => {
                                if 8 > mem::size_of::<isize>() {
                                    flags += 16;
                                } else {
                                    flags += 8;
                                }
                                cp = cp.add(3);
                            }
                            _ => break,
                        }
                    }
                    
                    // Read conversion character
                    c = *cp;
                    cp = cp.add(1);
                    
                    type_ = match c {
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
                                ArgType::CountSCharPointer
                            } else if (flags & 1) != 0 {
                                ArgType::CountShortPointer
                            } else {
                                ArgType::CountIntPointer
                            }
                        }
                        b'%' => ArgType::None,
                        _ => return Err(format!("Unknown conversion character: {}", c as char)),
                    };
                    
                    if type_ != ArgType::None {
                        dp.arg_index = arg_index;
                        if dp.arg_index == ARG_NONE {
                            dp.arg_index = arg_posn;
                            arg_posn = arg_posn.checked_add(1)
                                .ok_or("Argument position overflow")?;
                        }
                        register_arg!(dp.arg_index, type_);
                    }
                    
                    dp.conversion = c;
                    dp.dir_end = cp;
                    
                    self.directives.count += 1;
                    if self.directives.count >= d_allocated {
                        d_allocated = if d_allocated.checked_mul(2).is_none() {
                            self.directives.count.checked_add(1)
                                .ok_or("Directives count overflow")?
                        } else {
                            d_allocated * 2
                        };
                        
                        let new_size = d_allocated.checked_mul(mem::size_of::<Directive>())
                            .ok_or("Memory size overflow")?;
                        
                        let mut new_dir = Vec::with_capacity(d_allocated);
                        new_dir.extend_from_slice