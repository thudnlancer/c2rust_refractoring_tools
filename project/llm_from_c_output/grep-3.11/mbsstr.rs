/*
 * Searching in a string.
 * Copyright (C) 2005-2023 Free Software Foundation, Inc.
 * Written by Bruno Haible <bruno@clisp.org>, 2005.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::Ordering;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Knuth-Morris-Pratt algorithm for multibyte strings
fn knuth_morris_pratt_multibyte(
    haystack: &str,
    needle: &str,
) -> Result<Option<*const c_char>, &'static str> {
    let m = needle.chars().count();
    if m == 0 {
        return Ok(Some(haystack.as_ptr() as *const c_char));
    }

    // Collect needle characters
    let needle_chars: Vec<char> = needle.chars().collect();

    // Build KMP table
    let mut table = vec![0; m];
    table[0] = 0;
    let mut len = 0;
    let mut i = 1;

    while i < m {
        if needle_chars[i] == needle_chars[len] {
            len += 1;
            table[i] = len;
            i += 1;
        } else if len != 0 {
            len = table[len - 1];
        } else {
            table[i] = 0;
            i += 1;
        }
    }

    // Search using KMP algorithm
    let mut haystack_chars = haystack.chars();
    let mut needle_idx = 0;
    let mut result_ptr = None;

    while let Some(hc) = haystack_chars.next() {
        if hc == needle_chars[needle_idx] {
            needle_idx += 1;
            if needle_idx == m {
                // Found match
                let pos = haystack.len() - haystack_chars.as_str().len() - hc.len_utf8();
                result_ptr = Some(unsafe { haystack.as_ptr().add(pos) } as *const c_char);
                break;
            }
        } else {
            if needle_idx != 0 {
                needle_idx = table[needle_idx - 1];
                // Need to re-examine current character with new needle_idx
                haystack_chars = haystack.chars().skip(
                    haystack.len() - haystack_chars.as_str().len() - hc.len_utf8(),
                );
            }
        }
    }

    Ok(result_ptr)
}

/// Find the first occurrence of the character string needle in the character
/// string haystack. Return None if needle is not found in haystack.
pub fn mbsstr(haystack: &str, needle: &str) -> Option<*const c_char> {
    if needle.is_empty() {
        return Some(haystack.as_ptr() as *const c_char);
    }

    // For single-byte encodings (ASCII), use simpler search
    if haystack.is_ascii() && needle.is_ascii() {
        return haystack.find(needle).map(|pos| unsafe {
            haystack.as_ptr().add(pos) as *const c_char
        });
    }

    // Multi-byte case
    let mut try_kmp = true;
    let mut outer_loop_count = 0;
    let mut comparison_count = 0;
    let mut last_ccount = 0;
    let mut needle_iter = needle.chars();
    let mut needle_last_ccount = needle_iter.clone();

    let mut haystack_iter = haystack.chars();
    let first_needle_char = match needle_iter.next() {
        Some(c) => c,
        None => return Some(haystack.as_ptr() as *const c_char),
    };

    loop {
        let haystack_char = match haystack_iter.next() {
            Some(c) => c,
            None => return None,
        };

        // Check if we should try KMP
        if try_kmp
            && outer_loop_count >= 10
            && comparison_count >= 5 * outer_loop_count
        {
            // Advance needle_last_ccount
            let count = comparison_count - last_ccount;
            for _ in 0..count {
                if needle_last_ccount.next().is_none() {
                    break;
                }
            }
            last_ccount = comparison_count;

            if needle_last_ccount.clone().next().is_none() {
                // Try KMP
                match knuth_morris_pratt_multibyte(haystack, needle) {
                    Ok(result) => return result,
                    Err(_) => try_kmp = false,
                }
            }
        }

        outer_loop_count += 1;
        comparison_count += 1;

        if haystack_char == first_needle_char {
            let mut rhaystack_iter = haystack_iter.clone();
            let mut rneedle_iter = needle_iter.clone();

            loop {
                match (rneedle_iter.next(), rhaystack_iter.next()) {
                    (None, _) => {
                        // Found match
                        let pos = haystack.len()
                            - haystack_iter.as_str().len()
                            - haystack_char.len_utf8();
                        return Some(unsafe { haystack.as_ptr().add(pos) } as *const c_char);
                    }
                    (_, None) => return None, // No match
                    (Some(nc), Some(hc)) => {
                        comparison_count += 1;
                        if nc != hc {
                            break;
                        }
                    }
                }
            }
        }
    }
}