// Error-checking interface to string-to-float conversion functions.

// Copyright (C) 1996, 1998, 2003-2004, 2006, 2009-2022 Free Software
// Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Jim Meyering.

use std::str::FromStr;
use std::error::Error;
use std::num::ParseFloatError;

/// An interface to a string-to-floating-point conversion that encapsulates
/// all the error checking one should usually perform.
/// Like `str::parse`, but stores the conversion in `result`,
/// and returns `Ok(true)` upon successful conversion.
pub fn xstrtod(str: &str, result: &mut f64) -> Result<bool, ParseFloatError> {
    match str.parse::<f64>() {
        Ok(val) => {
            *result = val;
            Ok(true)
        }
        Err(e) => {
            if e.kind() == std::num::FloatErrorKind::Empty ||
               e.kind() == std::num::FloatErrorKind::Invalid {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

/// Long double version of xstrtod (using f64 in Rust as closest equivalent)
pub fn xstrtold(str: &str, result: &mut f64) -> Result<bool, ParseFloatError> {
    xstrtod(str, result)
}

/// Extended version that also returns the terminator position
pub fn xstrtod_ext(str: &str, result: &mut f64) -> Result<(bool, usize), ParseFloatError> {
    let mut chars = str.chars().peekable();
    let mut pos = 0;
    
    // Skip whitespace
    while let Some(c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            pos += 1;
        } else {
            break;
        }
    }
    
    let num_str: String = chars.clone().collect();
    match num_str.parse::<f64>() {
        Ok(val) => {
            *result = val;
            
            // Calculate terminator position
            let mut term_pos = pos;
            while let Some(c) = chars.next() {
                if c.is_numeric() || c == '.' || c == '+' || c == '-' || c == 'e' || c == 'E' {
                    term_pos += 1;
                } else {
                    break;
                }
            }
            
            Ok((true, term_pos))
        }
        Err(e) => {
            if e.kind() == std::num::FloatErrorKind::Empty ||
               e.kind() == std::num::FloatErrorKind::Invalid {
                Ok((false, pos))
            } else {
                Err(e)
            }
        }
    }
}

/// Long double version of xstrtod_ext
pub fn xstrtold_ext(str: &str, result: &mut f64) -> Result<(bool, usize), ParseFloatError> {
    xstrtod_ext(str, result)
}