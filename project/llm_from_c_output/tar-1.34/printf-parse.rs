/*!
Parse printf format string.
Copyright (C) 1999, 2002-2003, 2005, 2007, 2010-2021 Free Software
Foundation, Inc.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, see <https://www.gnu.org/licenses/>.
*/

use std::convert::TryFrom;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

/// Flags
pub const FLAG_GROUP: u8 = 1;      // ' flag
pub const FLAG_LEFT: u8 = 2;       // - flag
pub const FLAG_SHOWSIGN: u8 = 4;   // + flag
pub const FLAG_SPACE: u8 = 8;      // space flag
pub const FLAG_ALT: u8 = 16;       // # flag
pub const FLAG_ZERO: u8 = 32;
#[cfg(all(glibc_2, not(uclibc)))]
pub const FLAG_LOCALIZED: u8 = 64; // I flag, uses localized digits

/// arg_index value indicating that no argument is consumed.
pub const ARG_NONE: usize = usize::MAX;

/// Number of directly allocated directives (no malloc() needed).
pub const N_DIRECT_ALLOC_DIRECTIVES: usize = 7;

/// Argument types
#[derive(Debug, Clone, Copy, PartialEq)]
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
    String,
    Pointer,
    CountIntPointer,
    CountShortPointer,
    CountSCharPointer,
    CountLongIntPointer,
    CountLongLongIntPointer,
    WideChar,
    WideString,
    #[cfg(feature = "unistdio")]
    U8String,
    #[cfg(feature = "unistdio")]
    U16String,
    #[cfg(feature = "unistdio")]
    U32String,
}

/// A parsed directive.
#[derive(Debug)]
pub struct Directive<'a> {
    pub dir_start: &'a str,
    pub dir_end: &'a str,
    pub flags: u8,
    pub width_start: Option<&'a str>,
    pub width_end: Option<&'a str>,
    pub width_arg_index: usize,
    pub precision_start: Option<&'a str>,
    pub precision_end: Option<&'a str>,
    pub precision_arg_index: usize,
    pub conversion: char,
    pub arg_index: usize,
}

/// A parsed format string.
#[derive(Debug)]
pub struct Directives<'a> {
    pub count: usize,
    pub dir: Vec<Directive<'a>>,
    pub max_width_length: usize,
    pub max_precision_length: usize,
}

/// Arguments structure
#[derive(Debug)]
pub struct Arguments {
    pub count: usize,
    pub arg: Vec<ArgType>,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            count: 0,
            arg: Vec::new(),
        }
    }
}

impl Arguments {
    fn register_arg(&mut self, index: usize, arg_type: ArgType) -> Result<(), ()> {
        if index >= self.arg.len() {
            self.arg.resize(index + 1, ArgType::None);
            self.count = self.arg.len();
        }
        
        match self.arg[index] {
            ArgType::None => {
                self.arg[index] = arg_type;
                Ok(())
            }
            existing if existing == arg_type => Ok(()),
            _ => Err(()),
        }
    }
}

/// Parse printf format string
pub fn printf_parse(format: &str, directives: &mut Directives, args: &mut Arguments) -> Result<(), ()> {
    let mut cp = format;
    let mut arg_posn = 0;
    directives.count = 0;
    directives.dir.clear();
    directives.max_width_length = 0;
    directives.max_precision_length = 0;
    
    args.count = 0;
    args.arg.clear();

    while !cp.is_empty() {
        let c = cp.chars().next().unwrap();
        cp = &cp[c.len_utf8()..];
        
        if c == '%' {
            let mut arg_index = ARG_NONE;
            let dir_start = &cp[..cp.len() - 1]; // -1 because we advanced past %
            
            // Initialize the next directive
            let mut dp = Directive {
                dir_start,
                dir_end: "",
                flags: 0,
                width_start: None,
                width_end: None,
                width_arg_index: ARG_NONE,
                precision_start: None,
                precision_end: None,
                precision_arg_index: ARG_NONE,
                conversion: '\0',
                arg_index: ARG_NONE,
            };

            // Test for positional argument
            if let Some(ch) = cp.chars().next() {
                if ch.is_ascii_digit() {
                    let mut np = cp;
                    let mut n = 0;
                    
                    while let Some(ch) = np.chars().next() {
                        if ch.is_ascii_digit() {
                            n = n * 10 + (ch as usize - '0' as usize);
                            np = &np[ch.len_utf8()..];
                        } else {
                            break;
                        }
                    }
                    
                    if np.chars().next() == Some('$') {
                        if n == 0 {
                            return Err(());
                        }
                        arg_index = n - 1;
                        cp = &np[1..];
                    }
                }
            }

            // Read the flags
            loop {
                match cp.chars().next() {
                    Some('\'') => {
                        dp.flags |= FLAG_GROUP;
                        cp = &cp[1..];
                    }
                    Some('-') => {
                        dp.flags |= FLAG_LEFT;
                        cp = &cp[1..];
                    }
                    Some('+') => {
                        dp.flags |= FLAG_SHOWSIGN;
                        cp = &cp[1..];
                    }
                    Some(' ') => {
                        dp.flags |= FLAG_SPACE;
                        cp = &cp[1..];
                    }
                    Some('#') => {
                        dp.flags |= FLAG_ALT;
                        cp = &cp[1..];
                    }
                    Some('0') => {
                        dp.flags |= FLAG_ZERO;
                        cp = &cp[1..];
                    }
                    #[cfg(all(glibc_2, not(uclibc)))]
                    Some('I') => {
                        dp.flags |= FLAG_LOCALIZED;
                        cp = &cp[1..];
                    }
                    _ => break,
                }
            }

            // Parse the field width
            if let Some('*') = cp.chars().next() {
                dp.width_start = Some(cp);
                cp = &cp[1..];
                dp.width_end = Some(cp);
                directives.max_width_length = directives.max_width_length.max(1);

                // Test for positional argument
                if let Some(ch) = cp.chars().next() {
                    if ch.is_ascii_digit() {
                        let mut np = cp;
                        let mut n = 0;
                        
                        while let Some(ch) = np.chars().next() {
                            if ch.is_ascii_digit() {
                                n = n * 10 + (ch as usize - '0' as usize);
                                np = &np[ch.len_utf8()..];
                            } else {
                                break;
                            }
                        }
                        
                        if np.chars().next() == Some('$') {
                            if n == 0 {
                                return Err(());
                            }
                            dp.width_arg_index = n - 1;
                            cp = &np[1..];
                        }
                    }
                }
                
                if dp.width_arg_index == ARG_NONE {
                    dp.width_arg_index = arg_posn;
                    arg_posn += 1;
                    if dp.width_arg_index == ARG_NONE {
                        return Err(());
                    }
                }
                
                args.register_arg(dp.width_arg_index, ArgType::Int)?;
            } else if let Some(ch) = cp.chars().next() {
                if ch.is_ascii_digit() {
                    let start = cp;
                    let mut len = 0;
                    
                    while let Some(ch) = cp.chars().next() {
                        if ch.is_ascii_digit() {
                            len += 1;
                            cp = &cp[ch.len_utf8()..];
                        } else {
                            break;
                        }
                    }
                    
                    dp.width_start = Some(start);
                    dp.width_end = Some(cp);
                    directives.max_width_length = directives.max_width_length.max(len);
                }
            }

            // Parse the precision
            if let Some('.') = cp.chars().next() {
                cp = &cp[1..];
                
                if let Some('*') = cp.chars().next() {
                    dp.precision_start = Some(&cp[..1]);
                    cp = &cp[1..];
                    dp.precision_end = Some(cp);
                    directives.max_precision_length = directives.max_precision_length.max(2);

                    // Test for positional argument
                    if let Some(ch) = cp.chars().next() {
                        if ch.is_ascii_digit() {
                            let mut np = cp;
                            let mut n = 0;
                            
                            while let Some(ch) = np.chars().next() {
                                if ch.is_ascii_digit() {
                                    n = n * 10 + (ch as usize - '0' as usize);
                                    np = &np[ch.len_utf8()..];
                                } else {
                                    break;
                                }
                            }
                            
                            if np.chars().next() == Some('$') {
                                if n == 0 {
                                    return Err(());
                                }
                                dp.precision_arg_index = n - 1;
                                cp = &np[1..];
                            }
                        }
                    }
                    
                    if dp.precision_arg_index == ARG_NONE {
                        dp.precision_arg_index = arg_posn;
                        arg_posn += 1;
                        if dp.precision_arg_index == ARG_NONE {
                            return Err(());
                        }
                    }
                    
                    args.register_arg(dp.precision_arg_index, ArgType::Int)?;
                } else {
                    let start = &cp[..1];
                    let mut len = 1;
                    
                    while let Some(ch) = cp.chars().next() {
                        if ch.is_ascii_digit() {
                            len += 1;
                            cp = &cp[ch.len_utf8()..];
                        } else {
                            break;
                        }
                    }
                    
                    dp.precision_start = Some(start);
                    dp.precision_end = Some(cp);
                    directives.max_precision_length = directives.max_precision_length.max(len);
                }
            }

            // Parse argument type/size specifiers
            let mut flags = 0u8;
            loop {
                match cp.chars().next() {
                    Some('h') => {
                        flags |= 1 << (flags & 1);
                        cp = &cp[1..];
                    }
                    Some('L') => {
                        flags |= 4;
                        cp = &cp[1..];
                    }
                    Some('l') => {
                        flags += 8;
                        cp = &cp[1..];
                    }
                    Some('j') => {
                        if mem::size_of::<i64>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else if mem::size_of::<i64>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = &cp[1..];
                    }
                    Some('z') | Some('Z') => {
                        if mem::size_of::<usize>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else if mem::size_of::<usize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = &cp[1..];
                    }
                    Some('t') => {
                        if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            flags += 16;
                        } else if mem::size_of::<isize>() > mem::size_of::<i32>() {
                            flags += 8;
                        }
                        cp = &cp[1..];
                    }
                    #[cfg(target_os = "macos")]
                    Some('q') => {
                        if mem::size_of::<i64>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else {
                            flags += 8;
                        }
                        cp = &cp[1..];
                    }
                    #[cfg(target_os = "windows")]
                    Some('I') if cp.len() >= 3 && &cp[1..3] == "64" => {
                        if mem::size_of::<i64>() > mem::size_of::<isize>() {
                            flags += 16;
                        } else {
                            flags += 8;
                        }
                        cp = &cp[3..];
                    }
                    _ => break,
                }
            }

            // Read the conversion character
            let c = cp.chars().next().ok_or(())?;
            cp = &cp[c.len_utf8()..];
            
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
                        #[cfg(feature = "wchar")]
                        ArgType::WideChar,
                        #[cfg(not(feature = "wchar"))]
                        return Err(()),
                    } else {
                        ArgType::Char
                    }
                }
                #[cfg(feature = "wchar")]
                'C' => {
                    ArgType::WideChar
                }
                's' => {
                    if flags >= 8 {
                        #[cfg(feature = "wchar")]
                        ArgType::WideString,
                        #[cfg(not(feature = "wchar"))]
                        return Err(()),
                    } else {
                        ArgType::String
                    }
                }
                #[cfg(feature = "wchar")]
                'S' => {
                    ArgType::WideString
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
                #[cfg(feature = "unistdio")]
                'U' => {
                    if flags >= 16 {
                        ArgType::U32String
                    } else if flags >= 8 {
                        ArgType::U16String
                    } else {
                        ArgType::U8String
                    }
                }
                '%' => ArgType::None,
                _ => return Err(()),
            };

            if arg_type != ArgType::None {
                dp.arg_index = arg_index;
                if dp.arg_index == ARG_NONE {
                    dp.arg_index = arg_posn;
                    arg_posn += 1;
                    if dp.arg_index == ARG_NONE {
                        return Err(());
                    }
                }
                args.register_arg(dp.arg_index, arg_type)?;
            }
            
            dp.conversion = c;
            dp.dir_end = cp;
            directives.dir.push(dp);
            directives.count += 1;
        }
    }

    Ok(())
}