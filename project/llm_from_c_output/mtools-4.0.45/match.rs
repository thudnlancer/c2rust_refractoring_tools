/*
 *  Copyright 1986-1992 Emmet P. Gray.
 *  Copyright 1996-1998,2001,2002,2008,2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Do shell-style pattern matching for '?', '\', '[..]', and '*' wildcards.
 * Returns true if match, false if not.
 */

use std::char;

fn casecmp(a: char, b: char) -> bool {
    a.to_uppercase().eq(b.to_uppercase())
}

fn exactcmp(a: char, b: char) -> bool {
    a == b
}

fn is_in_range(ch: char, pattern: &[char], index: &mut usize, reverse: &mut bool) -> bool {
    if pattern[*index] == '^' {
        *reverse = true;
        *index += 1;
    } else {
        *reverse = false;
    }

    let mut found = false;
    while *index < pattern.len() && pattern[*index] != ']' {
        let first = pattern[*index];
        *index += 1;

        if *index < pattern.len() && pattern[*index] == '-' {
            *index += 1;
            if *index >= pattern.len() {
                // Malformed pattern, range not closed
                return false;
            }

            let last = pattern[*index];
            if last == ']' {
                // Last "-" in range designates itself
                if ch == first || ch == '-' {
                    found = true;
                }
                break;
            }
            *index += 1;

            // A proper range
            if ch >= first && ch <= last {
                found = true;
            }
        } else {
            // Just one character
            if ch == first {
                found = true;
            }
        }
    }

    found ^ *reverse
}

fn parse_range(
    pattern: &[char],
    p_index: &mut usize,
    s_char: char,
    out: &mut Option<&mut Vec<char>>,
    compfn: fn(char, char) -> bool,
) -> bool {
    let mut reverse = false;
    let mut p0 = *p_index;
    let mut p1 = *p_index;

    if let Some(ref mut out_vec) = out {
        out_vec.push(s_char);
    }

    if is_in_range(s_char, pattern, p_index, &mut reverse) {
        return !reverse;
    }

    if compfn == exactcmp {
        return reverse;
    }

    let lower_char = s_char.to_lowercase().next().unwrap();
    if is_in_range(lower_char, pattern, &mut p0, &mut reverse) {
        if let Some(ref mut out_vec) = out {
            out_vec.pop();
            out_vec.push(lower_char);
        }
        return !reverse;
    }

    let upper_char = s_char.to_uppercase().next().unwrap();
    if is_in_range(upper_char, pattern, &mut p1, &mut reverse) {
        if let Some(ref mut out_vec) = out {
            out_vec.pop();
            out_vec.push(upper_char);
        }
        return !reverse;
    }

    reverse
}

fn mt_match(
    s: &[char],
    p: &[char],
    out: &mut Option<&mut Vec<char>>,
    case: bool,
    length: usize,
    compfn: fn(char, char) -> bool,
) -> bool {
    let mut s_index = 0;
    let mut p_index = 0;
    let mut remaining_length = length;

    while p_index < p.len() && remaining_length > 0 {
        match p[p_index] {
            '?' => {
                if s_index >= s.len() {
                    return false;
                }
                if let Some(ref mut out_vec) = out {
                    out_vec.push(s[s_index]);
                }
                s_index += 1;
            }
            '*' => {
                while p_index < p.len() && p[p_index] == '*' && remaining_length > 0 {
                    p_index += 1;
                    remaining_length -= 1;
                }

                // Search for next char in pattern
                let mut temp_s_index = s_index;
                while temp_s_index < s.len() {
                    if mt_match(
                        &s[temp_s_index..],
                        &p[p_index..],
                        out,
                        case,
                        remaining_length,
                        compfn,
                    ) {
                        return true;
                    }
                    if let Some(ref mut out_vec) = out {
                        out_vec.push(s[temp_s_index]);
                    }
                    temp_s_index += 1;
                }
                continue;
            }
            '[' => {
                p_index += 1;
                remaining_length -= 1;
                if s_index >= s.len() {
                    return false;
                }
                if !parse_range(
                    p,
                    &mut p_index,
                    s[s_index],
                    out,
                    compfn,
                ) {
                    return false;
                }
                s_index += 1;
            }
            '\\' => {
                p_index += 1;
                remaining_length -= 1;
                if p_index >= p.len() {
                    return false;
                }
                // Fall through to default case
                if !compfn(s[s_index], p[p_index]) {
                    return false;
                }
                if let Some(ref mut out_vec) = out {
                    out_vec.push(p[p_index]);
                }
            }
            _ => {
                if !compfn(s[s_index], p[p_index]) {
                    return false;
                }
                if let Some(ref mut out_vec) = out {
                    out_vec.push(p[p_index]);
                }
            }
        }
        p_index += 1;
        remaining_length -= 1;
        s_index += 1;
    }

    if let Some(ref mut out_vec) = out {
        out_vec.push('\0');
    }

    // String ended prematurely?
    if s_index < s.len() {
        false
    } else {
        true
    }
}

pub fn match_pattern(
    s: &str,
    p: &str,
    out: Option<&mut Vec<char>>,
    case: bool,
    length: usize,
) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();
    let compfn = if case { casecmp } else { casecmp /* exactcmp */ };

    mt_match(&s_chars, &p_chars, &mut out, case, length, compfn)
}