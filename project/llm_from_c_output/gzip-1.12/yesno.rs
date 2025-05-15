// yesno.rs -- read a yes/no response from stdin
//
// Copyright (C) 1990, 1998, 2001, 2003-2022 Free Software Foundation, Inc.
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

use std::io::{self, BufRead};

/// Return true if we read an affirmative line from standard input.
///
/// Since this function uses stdin, it is suggested that the caller not
/// use STDIN_FILENO directly, and also that the line
/// atexit(close_stdin) be added to main().
pub fn yesno() -> bool {
    #[cfg(feature = "nls")]
    {
        let stdin = io::stdin();
        let mut line = String::new();
        let yes = match stdin.lock().read_line(&mut line) {
            Ok(0) => false,
            Ok(_) => {
                let trimmed = line.trim_end();
                rpmatch::rpmatch(trimmed).unwrap_or(false)
            }
            Err(_) => false,
        };
        yes
    }

    #[cfg(not(feature = "nls"))]
    {
        let mut stdin = io::stdin();
        let mut buf = [0u8; 1];
        let yes = match stdin.read_exact(&mut buf) {
            Ok(_) => {
                let c = buf[0] as char;
                c == 'y' || c == 'Y'
            }
            Err(_) => false,
        };
        
        // Consume remaining characters until newline or EOF
        let mut consume_buf = [0u8; 1024];
        loop {
            match stdin.read(&mut consume_buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    if consume_buf[..n].contains(&b'\n') {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        
        yes
    }
}

#[cfg(feature = "nls")]
mod rpmatch {
    use regex::Regex;

    pub fn rpmatch(response: &str) -> Result<bool, regex::Error> {
        let re = Regex::new(r"^[yY]")?;
        Ok(re.is_match(response))
    }
}