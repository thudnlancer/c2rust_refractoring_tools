/* strtrim.rs (remove trailing spaces from string) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

/// Remove trailing spaces from a string
///
/// # Arguments
///
/// * `s` - A string slice that may contain trailing spaces
///
/// # Returns
///
/// A new String with trailing spaces removed
///
/// # Examples
///
/// ```
/// let trimmed = strtrim("Errare humanum est   ");
/// assert_eq!(trimmed, "Errare humanum est");
///
/// let trimmed = strtrim("      ");
/// assert_eq!(trimmed, "");
/// ```
pub fn strtrim(s: &str) -> String {
    let mut result = s.to_string();
    while result.ends_with(' ') {
        result.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strtrim() {
        assert_eq!(strtrim("Errare humanum est   "), "Errare humanum est");
        assert_eq!(strtrim("      "), "");
        assert_eq!(strtrim("no spaces"), "no spaces");
        assert_eq!(strtrim(""), "");
    }
}