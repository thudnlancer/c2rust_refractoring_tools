/*!
Parse printf format string.
Copyright (C) 1999, 2002-2003, 2005, 2007, 2010-2023 Free Software
Foundation, Inc.

This file is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as
published by the Free Software Foundation; either version 2.1 of the
License, or (at your option) any later version.

This file is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.

You should have received a copy of the GNU Lesser General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::num::{NonZeroUsize, TryFromIntError};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArgType {
    None = 0,
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
    Int8T,
    UInt8T,
    Int16T,
    UInt16T,
    Int32T,
    UInt32T,
    Int64T,
    UInt64T,
    IntFast8T,
    UIntFast8T,
    IntFast16T,
    UIntFast16T,
    IntFast32T,
    UIntFast32T,
    IntFast64T,
    UIntFast64T,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    CountSCharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
    CountInt8TPointer,
    CountInt16TPointer,
    CountInt32TPointer,
    CountInt64TPointer,
    CountIntFast8TPointer,
    CountIntFast16TPointer,
    CountIntFast32TPointer,
    CountIntFast64TPointer,
    Double,
    LongDouble,
    U8String,
    U16String,
    U32String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            arg: Vec::with_capacity(7),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for CharDirective {
    fn default() -> Self {
        CharDirective {
            dir_start: ptr::null(),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharDirectives {
    pub count: usize,
    pub dir: Vec<CharDirective>,
    pub max_width_length: usize,
    pub max_precision_length: usize,
}

impl Default for CharDirectives {
    fn default() -> Self {
        CharDirectives {
            count: 0,
            dir: Vec::with_capacity(7),
            max_width_length: 0,
            max_precision_length: 0,
        }
    }
}

pub const FLAG_GROUP: c_int = 1;
pub const FLAG_LEFT: c_int = 2;
pub const FLAG_SHOWSIGN: c_int = 4;
pub const FLAG_SPACE: c_int = 8;
pub const FLAG_ALT: c_int = 16;
pub const FLAG_ZERO: c_int = 32;
pub const FLAG_LOCALIZED: c_int = 64;

pub const ARG_NONE: usize = usize::MAX;

pub fn printf_parse(
    format: *const c_char,
    d: &mut CharDirectives,
    a: &mut Arguments,
) -> c_int {
    let mut cp = format;
    let mut arg_posn = 0usize;
    let mut max_width_length = 0usize;
    let mut max_precision_length = 0usize;

    d.count = 0;
    d.dir.clear();
    d.max_width_length = 0;
    d.max_precision_length = 0;

    a.count = 0;
    a.arg.clear();

    macro_rules! register_arg {
        ($index:expr, $type:expr) => {
            let n = $index;
            if n >= a.arg.capacity() {
                let new_cap = if a.arg.capacity() == 0 {
                    1
                } else {
                    a.arg.capacity() * 2
                };
                if new_cap <= n {
                    a.arg.reserve(n + 1 - a.arg.len());
                } else {
                    a.arg.reserve(new_cap - a.arg.len());
                }
            }
            while a.count <= n {
                a.arg.push(Argument { type_: ArgType::None });
                a.count += 1;
            }
            if a.arg[n].type_ == ArgType::None {
                a.arg[n].type_ = $type;
            } else if a.arg[n].type_ != $type {
                // Ambiguous type for positional argument
                return -1;
            }
        };
    }

    unsafe {
        while *cp != 0 {
            let mut c = *cp;
            cp = cp.add(1);

            if c == b'%' as c_char {
                let mut arg_index = ARG_NONE;
                let mut dp = CharDirective::default();
                dp.dir_start = cp.sub(1);

                // Parse flags
                loop {
                    c = *cp;
                    match c {
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
                        b'I' => {
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
                    max_width_length = max_width_length.max(1);

                    // Check for positional argument
                    if (*cp >= b'0' as c_char) && (*cp <= b'9' as c_char) {
                        let mut np = cp;
                        while (*np >= b'0' as c_char) && (*np <= b'9' as c_char) {
                            np = np.add(1);
                        }
                        if *np == b'$' as c_char {
                            let mut n = 0usize;
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
                        if arg_posn == ARG_NONE {
                            return -1;
                        }
                    }
                    register_arg!(dp.width_arg_index, ArgType::Int);
                } else if (*cp >= b'0' as c_char) && (*cp <= b'9' as c_char) {
                    dp.width_start = cp;
                    while (*cp >= b'0' as c_char) && (*cp <= b'9' as c_char) {
                        cp = cp.add(1);
                    }
                    dp.width_end = cp;
                    let width_length = dp.width_end as usize - dp.width_start as usize;
                    max_width_length = max_width_length.max(width_length);
                }

                // Parse precision
                if *cp == b'.' as c_char {
                    cp = cp.add(1);
                    if *cp == b'*' as c_char {
                        dp.precision_start = cp.sub(1);
                        cp = cp.add(1);
                        dp.precision_end = cp;
                        max_precision_length = max_precision_length.max(2);

                        // Check for positional argument
                        if (*cp >= b'0' as c_char) && (*cp <= b'9' as c_char) {
                            let mut np = cp;
                            while (*np >= b'0' as c_char) && (*np <= b'9' as c_char) {
                                np = np.add(1);
                            }
                            if *np == b'$' as c_char {
                                let mut n = 0usize;
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
                            if arg_posn == ARG_NONE {
                                return -1;
                            }
                        }
                        register_arg!(dp.precision_arg_index, ArgType::Int);
                    } else {
                        dp.precision_start = cp.sub(1);
                        while (*cp >= b'0' as c_char) && (*cp <= b'9' as c_char) {
                            cp = cp.add(1);
                        }
                        dp.precision_end = cp;
                        let precision_length = dp.precision_end as usize - dp.precision_start as usize;
                        max_precision_length = max_precision_length.max(precision_length);
                    }
                }

                // Parse length modifiers and conversion specifier
                let mut signed_type = ArgType::Int;
                let mut unsigned_type = ArgType::UInt;
                let mut pointer_type = ArgType::CountIntPointer;
                let mut floatingpoint_type = ArgType::Double;

                match *cp {
                    b'h' => {
                        if *cp.add(1) == b'h' {
                            signed_type = ArgType::SChar;
                            unsigned_type = ArgType::UChar;
                            pointer_type = ArgType::CountSCharPointer;
                            cp = cp.add(2);
                        } else {
                            signed_type = ArgType::Short;
                            unsigned_type = ArgType::UShort;
                            pointer_type = ArgType::CountShortPointer;
                            cp = cp.add(1);
                        }
                    }
                    b'l' => {
                        if *cp.add(1) == b'l' {
                            signed_type = ArgType::LongLongInt;
                            unsigned_type = ArgType::ULongLongInt;
                            pointer_type = ArgType::CountLongLongIntPointer;
                            floatingpoint_type = ArgType::LongDouble;
                            cp = cp.add(2);
                        } else {
                            signed_type = ArgType::LongInt;
                            unsigned_type = ArgType::ULongInt;
                            pointer_type = ArgType::CountLongIntPointer;
                            cp = cp.add(1);
                        }
                    }
                    b'j' => {
                        if mem::size_of::<i64>() > mem::size_of::<isize>() {
                            signed_type = ArgType::LongLongInt;
                            unsigned_type = ArgType::ULongLongInt;
                            pointer_type = ArgType::CountLongLongIntPointer;
                            floatingpoint_type = ArgType::LongDouble;
                        } else if mem::size_of::<i32>() > mem::size_of::<isize>() {
                            signed_type = ArgType::LongInt;
                            unsigned_type = ArgType::ULongInt;
                            pointer_type = ArgType::CountLongIntPointer;
                        }
                        cp = cp.add(1);
                    }
                    b'z' | b'Z' => {
                        if mem::size_of::<usize>() > mem::size_of::<u32>() {
                            signed_type = ArgType::LongLongInt;
                            unsigned_type = ArgType::ULongLongInt;
                            pointer_type = ArgType::CountLongLongIntPointer;
                            floatingpoint_type = ArgType::LongDouble;
                        } else if mem::size_of::<usize>() > mem::size_of::<u16>() {
                            signed_type = ArgType::LongInt;
                            unsigned_type = ArgType::ULongInt;
                            pointer_type = ArgType::CountLongIntPointer;
                        }
                        cp = cp.add(1);
                    }
                    b't' => {
                        if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            signed_type = ArgType::LongLongInt;
                            unsigned_type = ArgType::ULongLongInt;
                            pointer_type = ArgType::CountLongLongIntPointer;
                            floatingpoint_type = ArgType::LongDouble;
                        } else if mem::size_of::<isize>() > mem::size_of::<i16>() {
                            signed_type = ArgType::LongInt;
                            unsigned_type = ArgType::ULongInt;
                            pointer_type = ArgType::CountLongIntPointer;
                        }
                        cp = cp.add(1);
                    }
                    b'L' => {
                        signed_type = ArgType::LongLongInt;
                        unsigned_type = ArgType::ULongLongInt;
                        pointer_type = ArgType::CountLongLongIntPointer;
                        floatingpoint_type = ArgType::LongDouble;
                        cp = cp.add(1);
                    }
                    _ => {}
                }

                // Parse conversion specifier
                c = *cp;
                cp = cp.add(1);

                let type_ = match c {
                    b'd' | b'i' => signed_type,
                    b'b' | b'o' | b'u' | b'x' | b'X' | b'B' => unsigned_type,
                    b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => floatingpoint_type,
                    b'c' => {
                        if signed_type == ArgType::LongInt || signed_type == ArgType::LongLongInt {
                            ArgType::WideChar
                        } else {
                            ArgType::Char
                        }
                    }
                    b's' => {
                        if signed_type == ArgType::LongInt || signed_type == ArgType::LongLongInt {
                            ArgType::WideString
                        } else {
                            ArgType::String
                        }
                    }
                    b'p' => ArgType::Pointer,
                    b'n' => pointer_type,
                    b'%' => ArgType::None,
                    _ => return -1,
                };

                if type_ != ArgType::None {
                    dp.arg_index = arg_index;
                    if dp.arg_index == ARG_NONE {
                        dp.arg_index = arg_posn;
                        arg_posn += 1;
                        if arg_posn == ARG_NONE {
                            return -1;
                        }
                    }
                    register_arg!(dp.arg_index, type_);
                }

                dp.conversion = c;
                dp.dir_end = cp;

                d.dir.push(dp);
                d.count += 1;
            }
        }
    }

    d.max_width_length = max_width_length;
    d.max_precision_length = max_precision_length;
    0
}