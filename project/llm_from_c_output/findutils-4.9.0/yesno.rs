/* yesno.rs -- read a yes/no response from stdin

   Copyright (C) 1990, 1998, 2001, 2003-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

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
        match stdin.lock().read_line(&mut line) {
            Ok(0) => false,
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                }
                matches!(rpmatch(&line), Some(true))
            }
            Err(_) => false,
        }
    }

    #[cfg(not(feature = "nls"))]
    {
        let mut stdin = io::stdin().lock();
        let mut buf = [0; 1];
        let yes = match stdin.read_exact(&mut buf) {
            Ok(_) => buf[0] == b'y' || buf[0] == b'Y',
            Err(_) => false,
        };
        
        // Consume remaining characters until newline or EOF
        let mut consume_buf = [0; 1024];
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
fn rpmatch(response: &str) -> Option<bool> {
    // This is a placeholder for actual rpmatch functionality
    // In a real implementation, this would interface with the system's rpmatch
    // or provide a Rust implementation of the same logic
    Some(response.starts_with('y') || response.starts_with('Y'))
}