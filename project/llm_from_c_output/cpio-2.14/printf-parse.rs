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
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ArgType {
    None,
    Int,
    UInt,
    SChar,
    UChar,
    Short,
    UShort,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    IntFast8,
    UIntFast8,
    IntFast16,
    UIntFast16,
    IntFast32,
    UIntFast32,
    IntFast64,
    UIntFast64,
    Char,
    WideChar,
    String,
    WideString,
    Pointer,
    Double,
    LongDouble,
    CountSCharPointer,
    CountShortPointer,
    CountIntPointer,
    CountLongPointer,
    CountLongLongPointer,
    CountInt8Pointer,
    CountInt16Pointer,
    CountInt32Pointer,
    CountInt64Pointer,
    CountIntFast8Pointer,
    CountIntFast16Pointer,
    CountIntFast32Pointer,
    CountIntFast64Pointer,
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
}

impl Arguments {
    fn new() -> Self {
        Arguments {
            count: 0,
            args: Vec::new(),
        }
    }

    fn reserve(&mut self, new_capacity: usize) -> Result<(), ()> {
        if new_capacity > self.args.capacity() {
            self.args.reserve(new_capacity - self.args.capacity());
        }
        Ok(())
    }

    fn register_arg(&mut self, index: usize, arg_type: ArgType) -> Result<(), ()> {
        if index >= self.args.len() {
            self.args.resize(index + 1, Argument { arg_type: ArgType::None });
            self.count = self.args.len();
        }

        if self.args[index].arg_type == ArgType::None {
            self.args[index].arg_type = arg_type;
            Ok(())
        } else if self.args[index].arg_type == arg_type {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Directive<'a> {
    dir_start: *const c_char,
    dir_end: *const c_char,
    flags: i32,
    width_start: *const c_char,
    width_end: *const c_char,
    width_arg_index: usize,
    precision_start: *const c_char,
    precision_end: *const c_char,
    precision_arg_index: usize,
    conversion: c_char,
    arg_index: usize,
}

const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;

#[derive(Debug, Clone)]
struct Directives<'a> {
    count: usize,
    dir: Vec<Directive<'a>>,
    max_width_length: usize,
    max_precision_length: usize,
}

impl<'a> Directives<'a> {
    fn new() -> Self {
        Directives {
            count: 0,
            dir: Vec::with_capacity(N_DIRECT_ALLOC_DIRECTIVES),
            max_width_length: 0,
            max_precision_length: 0,
        }
    }

    fn reserve(&mut self, new_capacity: usize) -> Result<(), ()> {
        if new_capacity > self.dir.capacity() {
            self.dir.reserve(new_capacity - self.dir.capacity());
        }
        Ok(())
    }

    fn push(&mut self, directive: Directive<'a>) -> Result<(), ()> {
        if self.dir.len() == self.dir.capacity() {
            let new_capacity = self.dir.capacity() * 2;
            self.reserve(new_capacity)?;
        }
        self.dir.push(directive);
        self.count += 1;
        Ok(())
    }
}

const FLAG_GROUP: i32 = 1;
const FLAG_LEFT: i32 = 2;
const FLAG_SHOWSIGN: i32 = 4;
const FLAG_SPACE: i32 = 8;
const FLAG_ALT: i32 = 16;
const FLAG_ZERO: i32 = 32;
const FLAG_LOCALIZED: i32 = 64;

const ARG_NONE: usize = usize::MAX;

fn printf_parse(format: &CStr, directives: &mut Directives, args: &mut Arguments) -> c_int {
    let mut cp = format.as_ptr();
    let mut arg_posn = 0;
    let mut max_width_length = 0;
    let mut max_precision_length = 0;

    directives.count = 0;
    directives.dir.clear();
    directives.max_width_length = 0;
    directives.max_precision_length = 0;

    args.count = 0;
    args.args.clear();

    unsafe {
        while *cp != 0 {
            let mut c = *cp;
            cp = cp.add(1);

            if c == b'%' as c_char {
                let mut arg_index = ARG_NONE;
                let mut dp = Directive {
                    dir_start: cp.sub(1),
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
                };

                // Test for positional argument
                if *cp >= b'0' as c_char && *cp <= b'9' as c_char {
                    let mut np = cp;
                    while *np >= b'0' as c_char && *np <= b'9' as c_char {
                        np = np.add(1);
                    }
                    if *np == b'$' as c_char {
                        let mut n = 0;
                        let mut digit_ptr = cp;
                        while digit_ptr < np {
                            n = n * 10 + (*digit_ptr - b'0' as c_char) as usize;
                            digit_ptr = digit_ptr.add(1);
                        }
                        if n == 0 {
                            return -1;
                        }
                        arg_index = n - 1;
                        cp = np.add(1);
                    }
                }

                // Read flags
                loop {
                    if *cp == b'\'' as c_char {
                        dp.flags |= FLAG_GROUP;
                        cp = cp.add(1);
                    } else if *cp == b'-' as c_char {
                        dp.flags |= FLAG_LEFT;
                        cp = cp.add(1);
                    } else if *cp == b'+' as c_char {
                        dp.flags |= FLAG_SHOWSIGN;
                        cp = cp.add(1);
                    } else if *cp == b' ' as c_char {
                        dp.flags |= FLAG_SPACE;
                        cp = cp.add(1);
                    } else if *cp == b'#' as c_char {
                        dp.flags |= FLAG_ALT;
                        cp = cp.add(1);
                    } else if *cp == b'0' as c_char {
                        dp.flags |= FLAG_ZERO;
                        cp = cp.add(1);
                    } else if *cp == b'I' as c_char {
                        dp.flags |= FLAG_LOCALIZED;
                        cp = cp.add(1);
                    } else {
                        break;
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
                            let mut digit_ptr = cp;
                            while digit_ptr < np {
                                n = n * 10 + (*digit_ptr - b'0' as c_char) as usize;
                                digit_ptr = digit_ptr.add(1);
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

                    if args.register_arg(dp.width_arg_index, ArgType::Int).is_err() {
                        return -1;
                    }
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
                                let mut digit_ptr = cp;
                                while digit_ptr < np {
                                    n = n * 10 + (*digit_ptr - b'0' as c_char) as usize;
                                    digit_ptr = digit_ptr.add(1);
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

                        if args.register_arg(dp.precision_arg_index, ArgType::Int).is_err() {
                            return -1;
                        }
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
                let mut signed_type = ArgType::Int;
                let mut unsigned_type = ArgType::UInt;
                let mut pointer_type = ArgType::CountIntPointer;
                let mut floatingpoint_type = ArgType::Double;

                if *cp == b'h' as c_char {
                    if *cp.add(1) == b'h' as c_char {
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
                } else if *cp == b'l' as c_char {
                    if *cp.add(1) == b'l' as c_char {
                        signed_type = ArgType::LongLong;
                        unsigned_type = ArgType::ULongLong;
                        pointer_type = ArgType::CountLongLongPointer;
                        floatingpoint_type = ArgType::LongDouble;
                        cp = cp.add(2);
                    } else {
                        signed_type = ArgType::Long;
                        unsigned_type = ArgType::ULong;
                        pointer_type = ArgType::CountLongPointer;
                        cp = cp.add(1);
                    }
                } else if *cp == b'j' as c_char {
                    if mem::size_of::<i64>() > mem::size_of::<isize>() {
                        signed_type = ArgType::LongLong;
                        unsigned_type = ArgType::ULongLong;
                        pointer_type = ArgType::CountLongLongPointer;
                        floatingpoint_type = ArgType::LongDouble;
                    } else if mem::size_of::<i32>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Long;
                        unsigned_type = ArgType::ULong;
                        pointer_type = ArgType::CountLongPointer;
                    }
                    cp = cp.add(1);
                } else if *cp == b'z' as c_char || *cp == b'Z' as c_char {
                    if mem::size_of::<usize>() > mem::size_of::<u32>() {
                        signed_type = ArgType::LongLong;
                        unsigned_type = ArgType::ULongLong;
                        pointer_type = ArgType::CountLongLongPointer;
                        floatingpoint_type = ArgType::LongDouble;
                    } else if mem::size_of::<usize>() > mem::size_of::<u32>() {
                        signed_type = ArgType::Long;
                        unsigned_type = ArgType::ULong;
                        pointer_type = ArgType::CountLongPointer;
                    }
                    cp = cp.add(1);
                } else if *cp == b't' as c_char {
                    if mem::size_of::<isize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::LongLong;
                        unsigned_type = ArgType::ULongLong;
                        pointer_type = ArgType::CountLongLongPointer;
                        floatingpoint_type = ArgType::LongDouble;
                    } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                        signed_type = ArgType::Long;
                        unsigned_type = ArgType::ULong;
                        pointer_type = ArgType::CountLongPointer;
                    }
                    cp = cp.add(1);
                } else if *cp == b'L' as c_char {
                    signed_type = ArgType::LongLong;
                    unsigned_type = ArgType::ULongLong;
                    pointer_type = ArgType::CountLongLongPointer;
                    floatingpoint_type = ArgType::LongDouble;
                    cp = cp.add(1);
                }

                // Read conversion character
                c = *cp;
                cp = cp.add(1);

                let arg_type = match c {
                    b'd' | b'i' => signed_type,
                    b'b' | b'o' | b'u' | b'x' | b'X' | b'B' => unsigned_type,
                    b'f' | b'F' | b'e' | b'E' | b'g' | b'G' | b'a' | b'A' => floatingpoint_type,
                    b'c' => {
                        if signed_type == ArgType::Long || signed_type == ArgType::LongLong {
                            ArgType::WideChar
                        } else {
                            ArgType::Char
                        }
                    }
                    b's' => {
                        if signed_type == ArgType::Long || signed_type == ArgType::LongLong {
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

                if arg_type != ArgType::None {
                    dp.arg_index = arg_index;
                    if dp.arg_index == ARG_NONE {
                        dp.arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.arg_index == ARG_NONE {
                            return -1;
                        }
                    }
                    if args.register_arg(dp.arg_index, arg_type).is_err() {
                        return -1;
                    }
                }

                dp.conversion = c;
                dp.dir_end = cp;

                if directives.push(dp).is_err() {
                    return -1;
                }
            }
        }
    }

    directives.max_width_length = max_width_length;
    directives.max_precision_length = max_precision_length;
    0
}