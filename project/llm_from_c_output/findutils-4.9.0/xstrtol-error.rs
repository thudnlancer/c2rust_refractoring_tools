// Error reporting interface for xstrto* functions.

// Copyright (C) 1995-1996, 1998-1999, 2001-2004, 2006-2022 Free Software
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

use std::process;
use getopt::Opt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrToLError {
    Invalid,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Overflow,
}

/// Report an error for an invalid integer in an option argument.
///
/// `err` is the error code returned by one of the xstrto* functions.
///
/// Use `opt_idx` to decide whether to print the short option string "C"
/// or "-C" or a long option string derived from `long_option`. `opt_idx`
/// is -2 if the short option "C" was used, without any leading "-"; it
/// is -1 if the short option "-C" was used; otherwise it is an index
/// into `long_options`, which should have a name preceded by two '-'
/// characters.
///
/// `arg` is the option-argument containing the integer.
///
/// After reporting an error, exit with a failure status.
pub fn xstrtol_fatal(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Opt],
    arg: &str,
) -> ! {
    xstrtol_error(err, opt_idx, c, long_options, arg, exit_failure());
    process::abort();
}

fn exit_failure() -> i32 {
    1
}

/// Like `xstrtol_error`, except exit with a failure status.
fn xstrtol_error(
    err: StrToLError,
    opt_idx: i32,
    c: char,
    long_options: &[Opt],
    arg: &str,
    exit_status: i32,
) {
    let hyphens = "--";
    let (msgid, option) = match err {
        StrToLError::Invalid => (
            "invalid %s%s argument '%s'",
            if opt_idx < 0 {
                let mut option_buffer = ['\0'; 2];
                option_buffer[0] = c;
                option_buffer[1] = '\0';
                option_buffer.iter().collect()
            } else {
                long_options[opt_idx as usize].name.to_string()
            },
        ),
        StrToLError::InvalidSuffixChar | StrToLError::InvalidSuffixCharWithOverflow => {
            ("invalid suffix in %s%s argument '%s'", "")
        }
        StrToLError::Overflow => ("%s%s argument '%s' too large", ""),
    };

    let hyphens = if opt_idx < 0 {
        &hyphens[(-opt_idx) as usize..]
    } else {
        hyphens
    };

    eprintln!(msgid, hyphens, option, arg);
    process::exit(exit_status);
}