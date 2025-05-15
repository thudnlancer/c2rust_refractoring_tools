// regextype.rs -- Decode the name of a regular expression syntax.

// Copyright (C) 2005-2022 Free Software Foundation, Inc.

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

// Written by James Youngman <jay@gnu.org>.

use regex_syntax::hir::Hir;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Findutils = 1,
    Generic = 2,
    All = 3, // CONTEXT_GENERIC | CONTEXT_FINDUTILS
}

#[derive(Debug)]
struct RegexTypeMap {
    name: &'static str,
    context: Context,
    option_val: i32,
}

const REGEX_MAP: [RegexTypeMap; 13] = [
    RegexTypeMap {
        name: "findutils-default",
        context: Context::Findutils,
        option_val: libc::RE_SYNTAX_EMACS | libc::RE_DOT_NEWLINE,
    },
    RegexTypeMap {
        name: "ed",
        context: Context::Generic,
        option_val: libc::RE_SYNTAX_ED,
    },
    RegexTypeMap {
        name: "emacs",
        context: Context::All,
        option_val: libc::RE_SYNTAX_EMACS,
    },
    RegexTypeMap {
        name: "gnu-awk",
        context: Context::All,
        option_val: libc::RE_SYNTAX_GNU_AWK,
    },
    RegexTypeMap {
        name: "grep",
        context: Context::All,
        option_val: libc::RE_SYNTAX_GREP,
    },
    RegexTypeMap {
        name: "posix-awk",
        context: Context::All,
        option_val: libc::RE_SYNTAX_POSIX_AWK,
    },
    RegexTypeMap {
        name: "awk",
        context: Context::All,
        option_val: libc::RE_SYNTAX_AWK,
    },
    RegexTypeMap {
        name: "posix-basic",
        context: Context::All,
        option_val: libc::RE_SYNTAX_POSIX_BASIC,
    },
    RegexTypeMap {
        name: "posix-egrep",
        context: Context::All,
        option_val: libc::RE_SYNTAX_POSIX_EGREP,
    },
    RegexTypeMap {
        name: "egrep",
        context: Context::All,
        option_val: libc::RE_SYNTAX_EGREP,
    },
    RegexTypeMap {
        name: "posix-extended",
        context: Context::All,
        option_val: libc::RE_SYNTAX_POSIX_EXTENDED,
    },
    RegexTypeMap {
        name: "posix-minimal-basic",
        context: Context::Generic,
        option_val: libc::RE_SYNTAX_POSIX_MINIMAL_BASIC,
    },
    RegexTypeMap {
        name: "sed",
        context: Context::Generic,
        option_val: libc::RE_SYNTAX_SED,
    },
];

pub fn get_regex_type(s: &str) -> Result<i32, String> {
    for entry in REGEX_MAP.iter() {
        if entry.name == s {
            return Ok(entry.option_val);
        }
    }

    let valid_types: Vec<&str> = REGEX_MAP.iter().map(|e| e.name).collect();
    Err(format!(
        "Unknown regular expression type {}; valid types are {}.",
        s,
        valid_types.join(", ")
    ))
}

pub fn get_regex_type_name(ix: usize) -> Option<&'static str> {
    REGEX_MAP.get(ix).map(|e| e.name)
}

pub fn get_regex_type_flags(ix: usize) -> Option<i32> {
    REGEX_MAP.get(ix).map(|e| e.option_val)
}

pub fn get_regex_type_context(ix: usize) -> Option<Context> {
    REGEX_MAP.get(ix).map(|e| e.context)
}

pub fn get_regex_type_synonym(ix: usize, context: Context) -> Option<usize> {
    if ix >= REGEX_MAP.len() {
        return None;
    }

    let flags = REGEX_MAP[ix].option_val;
    for i in 0..ix {
        if (REGEX_MAP[i].context as u32 & context as u32) == 0 {
            continue;
        } else if flags == REGEX_MAP[i].option_val {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_regex_type() {
        assert_eq!(
            get_regex_type("emacs").unwrap(),
            libc::RE_SYNTAX_EMACS
        );
        assert!(get_regex_type("invalid").is_err());
    }

    #[test]
    fn test_get_regex_type_name() {
        assert_eq!(get_regex_type_name(0), Some("findutils-default"));
        assert_eq!(get_regex_type_name(100), None);
    }

    #[test]
    fn test_get_regex_type_context() {
        assert_eq!(
            get_regex_type_context(0).unwrap() as u32,
            Context::Findutils as u32
        );
        assert_eq!(get_regex_type_context(100), None);
    }

    #[test]
    fn test_get_regex_type_synonym() {
        assert_eq!(get_regex_type_synonym(0, Context::Findutils), None);
    }
}