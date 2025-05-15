/*
 * Case mapping for UTF-8 strings (locale dependent).
 * Copyright (C) 2009-2023 Free Software Foundation, Inc.
 * Written by Bruno Haible <bruno@clisp.org>, 2009.
 *
 * This file is free software.
 * It is dual-licensed under "the GNU LGPLv3+ or the GNU GPLv2+".
 * You can redistribute it and/or modify it under either
 *   - the terms of the GNU Lesser General Public License as published
 *     by the Free Software Foundation, either version 3, or (at your
 *     option) any later version, or
 *   - the terms of the GNU General Public License as published by the
 *     Free Software Foundation; either version 2, or (at your option)
 *     any later version, or
 *   - the same dual license "the GNU LGPLv3+ or the GNU GPLv2+".
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License and the GNU General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License and of the GNU General Public License along with this
 * program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::error::Error;
use std::fmt;
use std::str;

mod unicase {
    pub mod caseprop;
    pub mod special_casing;
}

mod context;
mod unictype;
mod uninorm;

#[derive(Debug)]
pub enum CaseMapError {
    InvalidUtf8,
    NormalizationFailed,
    Other(String),
}

impl fmt::Display for CaseMapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CaseMapError::InvalidUtf8 => write!(f, "Invalid UTF-8 sequence"),
            CaseMapError::NormalizationFailed => write!(f, "Unicode normalization failed"),
            CaseMapError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for CaseMapError {}

pub fn u8_casemap(input: &str, to_upper: bool) -> Result<String, CaseMapError> {
    // Validate UTF-8 input
    if !input.is_char_boundary(0) || !input.is_char_boundary(input.len()) {
        return Err(CaseMapError::InvalidUtf8);
    }

    // Normalize the input string
    let normalized = uninorm::normalize(input).map_err(|_| CaseMapError::NormalizationFailed)?;

    // Apply case mapping
    let result = if to_upper {
        normalized.to_uppercase()
    } else {
        normalized.to_lowercase()
    };

    Ok(result)
}