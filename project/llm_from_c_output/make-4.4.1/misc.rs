/*
 * Process handling for Windows
 * Copyright (C) 1996-2023 Free Software Foundation, Inc.
 * This file is part of GNU Make.
 *
 * GNU Make is free software; you can redistribute it and/or modify it under the
 * terms of the GNU General Public License as published by the Free Software
 * Foundation; either version 3 of the License, or (at your option) any later
 * version.
 *
 * GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE.  See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::Ordering;
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr;

/*
 * Description:  Convert a NULL string terminated UNIX environment block to
 *              an environment block suitable for a windows32 system call
 *
 * Returns:  Ok(env_block) on success, Err(()) on failure
 *
 * Notes/Dependencies:  the environment block is sorted in case-insensitive
 *      order, is double-null terminated, and is a char *, not a char **
 */
fn compare(a: &&str, b: &&str) -> Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

pub fn arr2envblk(arr: &[&str]) -> Result<Vec<u16>, ()> {
    // Count non-null entries
    let arrcnt = arr.iter().take_while(|&&s| !s.is_empty()).count();

    // Calculate required size
    let size_needed = arr.iter()
        .take(arrcnt)
        .map(|s| s.len() + 1)
        .sum::<usize>() + 1;

    // Sort the environment variables case-insensitively
    let mut sorted_arr = arr.iter()
        .take(arrcnt)
        .collect::<Vec<_>>();
    sorted_arr.sort_by(|a, b| compare(a, b));

    // Convert to Windows environment block format
    let mut env_block = Vec::with_capacity(size_needed);
    for &var in sorted_arr.iter() {
        // Convert each string to UTF-16 (Windows wide string)
        let wide_str = OsStr::new(var)
            .encode_wide()
            .collect::<Vec<u16>>();
        
        // Append to environment block with null terminator
        env_block.extend(wide_str);
        env_block.push(0); // null terminator
    }
    env_block.push(0); // double null terminator

    Ok(env_block)
}