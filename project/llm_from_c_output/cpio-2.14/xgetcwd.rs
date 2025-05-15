/*
 * This file is part of the Rust implementation of xgetcwd.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use std::io;

/// Return the current directory, newly allocated.
/// Upon an out-of-memory error, panic (equivalent to xalloc_die in C).
/// Upon any other type of error, return None with the error stored in io::Error.
pub fn xgetcwd() -> Option<String> {
    match env::current_dir() {
        Ok(path) => Some(path.to_string_lossy().into_owned()),
        Err(e) => {
            if e.kind() == io::ErrorKind::OutOfMemory {
                panic!("Out of memory while getting current directory");
            }
            None
        }
    }
}