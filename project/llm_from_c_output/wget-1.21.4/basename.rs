// basename.rs -- return the last element in a file name

// Copyright (C) 1990, 1998-2001, 2003-2006, 2009-2023 Free Software
// Foundation, Inc.

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

use std::path::{Component, Path};

/// Returns the base name of a file path
pub fn base_name(name: &str) -> String {
    let path = Path::new(name);
    let base = path.components().last().unwrap_or(Component::RootDir);
    
    let (base_str, needs_dotslash) = match base {
        Component::Normal(s) => {
            let s = s.to_str().unwrap_or("");
            let needs_dotslash = if cfg!(windows) {
                // On Windows, check if the base has a drive prefix
                s.len() >= 2 && s.chars().nth(1) == Some(':')
            } else {
                false
            };
            (s, needs_dotslash)
        },
        _ => (name, false),
    };

    let length = base_str.len();
    let slash_adjusted_length = if length > 0 {
        // Collapse trailing slashes into one
        let mut adjusted = length;
        while adjusted > 0 && base_str.chars().nth(adjusted - 1).unwrap() == '/' {
            adjusted -= 1;
        }
        adjusted + 1
    } else {
        length
    };

    let mut result = String::with_capacity(if needs_dotslash { 2 + slash_adjusted_length } else { slash_adjusted_length });
    
    if needs_dotslash {
        result.push_str("./");
    }

    result.push_str(&base_str[..slash_adjusted_length]);
    result
}