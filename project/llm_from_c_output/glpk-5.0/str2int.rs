/* str2int.rs (convert string to int) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

/// Convert character string to value of i32 type
///
/// # Arguments
///
/// * `str` - The string to convert
///
/// # Returns
///
/// Returns `Ok(i32)` if conversion is successful, or one of the following errors:
///
/// * `Err(1)` - value out of range
/// * `Err(2)` - character string is syntactically incorrect
pub fn str2int(str: &str) -> Result<i32, i32> {
    let mut val = 0i32;
    let mut chars = str.chars().peekable();
    
    // scan optional sign
    let s = match chars.peek() {
        Some('+') => {
            chars.next();
            1
        }
        Some('-') => {
            chars.next();
            -1
        }
        _ => 1,
    };
    
    // check for the first digit
    let mut has_digits = false;
    while let Some(c) = chars.peek() {
        if !c.is_ascii_digit() {
            break;
        }
        has_digits = true;
        let d = c.to_digit(10).unwrap() as i32;
        
        if s > 0 {
            if val > i32::MAX / 10 {
                return Err(1);
            }
            val *= 10;
            if val > i32::MAX - d {
                return Err(1);
            }
            val += d;
        } else {
            if val < i32::MIN / 10 {
                return Err(1);
            }
            val *= 10;
            if val < i32::MIN + d {
                return Err(1);
            }
            val -= d;
        }
        chars.next();
    }
    
    // check we had at least one digit and no remaining characters
    if !has_digits || chars.next().is_some() {
        return Err(2);
    }
    
    Ok(val)
}

/* eof */