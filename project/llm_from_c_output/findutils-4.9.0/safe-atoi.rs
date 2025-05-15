// safe-atoi.rs -- checked string-to-int conversion.
// Copyright (C) 2010-2022 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::num::ParseIntError;
use std::str::FromStr;
use std::i32;
use std::i64;
use std::process;

#[derive(Debug, Clone, Copy)]
pub enum QuotingStyle {
    // Define your quoting styles here as needed
    // This should match the C enum
}

pub fn safe_atoi(s: &str, style: QuotingStyle) -> Result<i32, String> {
    match s.parse::<i64>() {
        Ok(lval) => {
            if lval > i32::MAX as i64 || lval < i32::MIN as i64 {
                Err(format!("Number out of range for i32: {}", s))
            } else {
                Ok(lval as i32)
            }
        }
        Err(e) => match e.kind() {
            std::num::IntErrorKind::Empty => {
                Err(format!("Expected an integer: {}", s))
            }
            std::num::IntErrorKind::InvalidDigit => {
                // Check if there's a suffix
                if s.chars().next().is_some() {
                    let (num_part, suffix) = s.split_at(
                        s.find(|c: char| !c.is_digit(10)
                            .unwrap_or_else(|| s.len())
                    );
                    if num_part.is_empty() {
                        Err(format!("Expected an integer: {}", s))
                    } else {
                        Err(format!("Unexpected suffix {} on {}", suffix, s))
                    }
                } else {
                    Err(format!("Expected an integer: {}", s))
                }
            }
            std::num::IntErrorKind::PosOverflow => {
                Err(format!("Number too large: {}", s))
            }
            std::num::IntErrorKind::NegOverflow => {
                Err(format!("Number too small: {}", s))
            }
            _ => Err(format!("Invalid integer: {}", s)),
        },
    }
}

// Wrapper that exits on error, similar to the C version's die()
pub fn safe_atoi_or_die(s: &str, style: QuotingStyle) -> i32 {
    match safe_atoi(s, style) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}