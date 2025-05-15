/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Parse printf format string.
   Copyright (C) 1999, 2002-2003, 2005, 2007 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation,
   Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.  */

use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use std::num::Wrapping;

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
    args: Vec<Argument>,
}

impl Arguments {
    fn new() -> Self {
        Arguments { args: Vec::new() }
    }

    fn register_arg(&mut self, index: usize, arg_type: ArgType) -> Result<(), ()> {
        while self.args.len() <= index {
            self.args.push(Argument { arg_type: ArgType::None });
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
struct Flags {
    group: bool,
    left: bool,
    showsign: bool,
    space: bool,
    alt: bool,
    zero: bool,
}

impl Flags {
    fn new() -> Self {
        Flags {
            group: false,
            left: false,
            showsign: false,
            space: false,
            alt: false,
            zero: false,
        }
    }

    fn from_bits(bits: u32) -> Self {
        Flags {
            group: (bits & 1) != 0,
            left: (bits & 2) != 0,
            showsign: (bits & 4) != 0,
            space: (bits & 8) != 0,
            alt: (bits & 16) != 0,
            zero: (bits & 32) != 0,
        }
    }

    fn to_bits(&self) -> u32 {
        let mut bits = 0;
        if self.group { bits |= 1; }
        if self.left { bits |= 2; }
        if self.showsign { bits |= 4; }
        if self.space { bits |= 8; }
        if self.alt { bits |= 16; }
        if self.zero { bits |= 32; }
        bits
    }
}

#[derive(Debug, Clone)]
struct Directive {
    dir_start: usize,
    dir_end: usize,
    flags: Flags,
    width_start: Option<usize>,
    width_end: Option<usize>,
    width_arg_index: Option<usize>,
    precision_start: Option<usize>,
    precision_end: Option<usize>,
    precision_arg_index: Option<usize>,
    conversion: char,
    arg_index: Option<usize>,
}

impl Directive {
    fn new() -> Self {
        Directive {
            dir_start: 0,
            dir_end: 0,
            flags: Flags::new(),
            width_start: None,
            width_end: None,
            width_arg_index: None,
            precision_start: None,
            precision_end: None,
            precision_arg_index: None,
            conversion: '\0',
            arg_index: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Directives {
    count: usize,
    dirs: Vec<Directive>,
    max_width_length: usize,
    max_precision_length: usize,
}

impl Directives {
    fn new() -> Self {
        Directives {
            count: 0,
            dirs: vec![Directive::new()],
            max_width_length: 0,
            max_precision_length: 0,
        }
    }
}

fn printf_parse(format: &str, directives: &mut Directives, args: &mut Arguments) -> Result<(), ()> {
    let mut cp = 0;
    let mut arg_posn = Wrapping(0);
    let mut d_allocated = 1;
    directives.dirs = vec![Directive::new(); d_allocated];
    
    let mut max_width_length = 0;
    let mut max_precision_length = 0;

    let format_chars: Vec<char> = format.chars().collect();
    
    while cp < format_chars.len() {
        let c = format_chars[cp];
        cp += 1;
        
        if c == '%' {
            let mut arg_index = None;
            let dp = &mut directives.dirs[directives.count];
            
            dp.dir_start = cp - 1;
            dp.flags = Flags::new();
            dp.width_start = None;
            dp.width_end = None;
            dp.width_arg_index = None;
            dp.precision_start = None;
            dp.precision_end = None;
            dp.precision_arg_index = None;
            dp.arg_index = None;
            
            // Test for positional argument
            if cp < format_chars.len() && format_chars[cp].is_digit(10) {
                let mut np = cp;
                while np < format_chars.len() && format_chars[np].is_digit(10) {
                    np += 1;
                }
                
                if np < format_chars.len() && format_chars[np] == '$' {
                    let mut n = 0;
                    for i in cp..np {
                        n = n * 10 + format_chars[i].to_digit(10).unwrap() as usize;
                    }
                    if n == 0 {
                        return Err(());
                    }
                    arg_index = Some(n - 1);
                    cp = np + 1;
                }
            }
            
            // Read flags
            loop {
                if cp >= format_chars.len() {
                    break;
                }
                
                match format_chars[cp] {
                    '\'' => {
                        dp.flags.group = true;
                        cp += 1;
                    }
                    '-' => {
                        dp.flags.left = true;
                        cp += 1;
                    }
                    '+' => {
                        dp.flags.showsign = true;
                        cp += 1;
                    }
                    ' ' => {
                        dp.flags.space = true;
                        cp += 1;
                    }
                    '#' => {
                        dp.flags.alt = true;
                        cp += 1;
                    }
                    '0' => {
                        dp.flags.zero = true;
                        cp += 1;
                    }
                    _ => break,
                }
            }
            
            // Parse field width
            if cp < format_chars.len() && format_chars[cp] == '*' {
                dp.width_start = Some(cp);
                cp += 1;
                dp.width_end = Some(cp);
                max_width_length = max_width_length.max(1);
                
                // Test for positional argument
                if cp < format_chars.len() && format_chars[cp].is_digit(10) {
                    let mut np = cp;
                    while np < format_chars.len() && format_chars[np].is_digit(10) {
                        np += 1;
                    }
                    
                    if np < format_chars.len() && format_chars[np] == '$' {
                        let mut n = 0;
                        for i in cp..np {
                            n = n * 10 + format_chars[i].to_digit(10).unwrap() as usize;
                        }
                        if n == 0 {
                            return Err(());
                        }
                        dp.width_arg_index = Some(n - 1);
                        cp = np + 1;
                    }
                }
                
                if dp.width_arg_index.is_none() {
                    dp.width_arg_index = Some(arg_posn.0);
                    arg_posn += Wrapping(1);
                    if dp.width_arg_index == Some(usize::MAX) {
                        return Err(());
                    }
                }
                
                args.register_arg(dp.width_arg_index.unwrap(), ArgType::Int)?;
            } else if cp < format_chars.len() && format_chars[cp].is_digit(10) {
                dp.width_start = Some(cp);
                while cp < format_chars.len() && format_chars[cp].is_digit(10) {
                    cp += 1;
                }
                dp.width_end = Some(cp);
                let width_length = dp.width_end.unwrap() - dp.width_start.unwrap();
                max_width_length = max_width_length.max(width_length);
            }
            
            // Parse precision
            if cp < format_chars.len() && format_chars[cp] == '.' {
                cp += 1;
                if cp < format_chars.len() && format_chars[cp] == '*' {
                    dp.precision_start = Some(cp - 1);
                    cp += 1;
                    dp.precision_end = Some(cp);
                    max_precision_length = max_precision_length.max(2);
                    
                    // Test for positional argument
                    if cp < format_chars.len() && format_chars[cp].is_digit(10) {
                        let mut np = cp;
                        while np < format_chars.len() && format_chars[np].is_digit(10) {
                            np += 1;
                        }
                        
                        if np < format_chars.len() && format_chars[np] == '$' {
                            let mut n = 0;
                            for i in cp..np {
                                n = n * 10 + format_chars[i].to_digit(10).unwrap() as usize;
                            }
                            if n == 0 {
                                return Err(());
                            }
                            dp.precision_arg_index = Some(n - 1);
                            cp = np + 1;
                        }
                    }
                    
                    if dp.precision_arg_index.is_none() {
                        dp.precision_arg_index = Some(arg_posn.0);
                        arg_posn += Wrapping(1);
                        if dp.precision_arg_index == Some(usize::MAX) {
                            return Err(());
                        }
                    }
                    
                    args.register_arg(dp.precision_arg_index.unwrap(), ArgType::Int)?;
                } else {
                    dp.precision_start = Some(cp - 1);
                    while cp < format_chars.len() && format_chars[cp].is_digit(10) {
                        cp += 1;
                    }
                    dp.precision_end = Some(cp);
                    let precision_length = dp.precision_end.unwrap() - dp.precision_start.unwrap();
                    max_precision_length = max_precision_length.max(precision_length);
                }
            }
            
            // Parse argument type/size specifiers
            let mut flags = 0u32;
            loop {
                if cp >= format_chars.len() {
                    break;
                }
                
                match format_chars[cp] {
                    'h' => {
                        flags |= 1 << (flags & 1);
                        cp += 1;
                    }
                    'L' => {
                        flags |= 4;
                        cp += 1;
                    }
                    'l' => {
                        flags += 8;
                        cp += 1;
                    }
                    'j' => {
                        if mem::size_of::<i64>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp += 1;
                    }
                    'z' | 'Z' => {
                        if mem::size_of::<usize>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp += 1;
                    }
                    't' => {
                        if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp += 1;
                    }
                    _ => break,
                }
            }
            
            if cp >= format_chars.len() {
                return Err(());
            }
            
            let c = format_chars[cp];
            cp += 1;
            
            let arg_type = match c {
                'd' | 'i' => {
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
                'o' | 'u' | 'x' | 'X' => {
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
                'f' | 'F' | 'e' | 'E' | 'g' | 'G' | 'a' | 'A' => {
                    if flags >= 16 || (flags & 4) != 0 {
                        ArgType::LongDouble
                    } else {
                        ArgType::Double
                    }
                }
                'c' => {
                    if flags >= 8 {
                        ArgType::WideChar
                    } else {
                        ArgType::Char
                    }
                }
                's' => {
                    if flags >= 8 {
                        ArgType::WideString
                    } else {
                        ArgType::String
                    }
                }
                'p' => ArgType::Pointer,
                'n' => {
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
                '%' => ArgType::None,
                _ => return Err(()),
            };
            
            if arg_type != ArgType::None {
                dp.arg_index = arg_index;
                if dp.arg_index.is_none() {
                    dp.arg_index = Some(arg_posn.0);
                    arg_posn += Wrapping(1);
                    if dp.arg_index == Some(usize::MAX) {
                        return Err(());
                    }
                }
                args.register_arg(dp.arg_index.unwrap(), arg_type)?;
            }
            
            dp.conversion = c;
            dp.dir_end = cp;
            
            directives.count += 1;
            if directives.count >= d_allocated {
                d_allocated *= 2;
                directives.dirs.resize(d_allocated, Directive::new());
            }
        }
    }
    
    directives.max_width_length = max_width_length;
    directives.max_precision_length = max_precision_length;
    Ok(())
}