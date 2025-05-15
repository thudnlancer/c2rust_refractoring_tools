/* str2num.rs (convert string to double) */

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

use std::str::FromStr;

/***********************************************************************
*  NAME
*
*  str2num - convert character string to value of double type
*
*  SYNOPSIS
*
*  fn str2num(str: &str, val: &mut f64) -> i32
*
*  DESCRIPTION
*
*  The routine str2num converts the character string str to a value of
*  double type and stores the value into location, which the parameter
*  val points to (in the case of error content of this location is not
*  changed).
*
*  RETURNS
*
*  The routine returns one of the following error codes:
*
*  0 - no error;
*  1 - value out of range;
*  2 - character string is syntactically incorrect. */

pub fn str2num(str: &str, val: &mut f64) -> i32 {
    let mut k = 0;
    let chars: Vec<char> = str.chars().collect();
    let len = chars.len();
    
    /* scan optional sign */
    if len > 0 && (chars[0] == '+' || chars[0] == '-') {
        k += 1;
    }
    
    /* check for decimal point */
    if k < len && chars[k] == '.' {
        k += 1;
        /* a digit should follow it */
        if k >= len || !chars[k].is_ascii_digit() {
            return 2;
        }
        k += 1;
        goto_frac();
    } else {
        /* integer part should start with a digit */
        if k >= len || !chars[k].is_ascii_digit() {
            return 2;
        }
        /* scan integer part */
        while k < len && chars[k].is_ascii_digit() {
            k += 1;
        }
        /* check for decimal point */
        if k < len && chars[k] == '.' {
            k += 1;
        }
    }
    
    fn goto_frac() {}
    /* scan optional fraction part */
    while k < len && chars[k].is_ascii_digit() {
        k += 1;
    }
    
    /* check for decimal exponent */
    if k < len && (chars[k] == 'E' || chars[k] == 'e') {
        k += 1;
        /* scan optional sign */
        if k < len && (chars[k] == '+' || chars[k] == '-') {
            k += 1;
        }
        /* a digit should follow E, E+ or E- */
        if k >= len || !chars[k].is_ascii_digit() {
            return 2;
        }
    }
    
    /* scan optional exponent part */
    while k < len && chars[k].is_ascii_digit() {
        k += 1;
    }
    
    /* check for terminator */
    if k != len {
        return 2;
    }
    
    /* perform conversion */
    let parsed_val = match f64::from_str(str) {
        Ok(v) => v,
        Err(_) => return 2,
    };
    
    /* check for overflow */
    if !parsed_val.is_finite() {
        return 1;
    }
    
    /* check for underflow */
    let val_result = if parsed_val.abs() < f64::MIN_POSITIVE {
        0.0
    } else {
        parsed_val
    };
    
    /* conversion has been done */
    *val = val_result;
    0
}

/* eof */