/* strspx.rs (remove all spaces from string) */

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

/// Removes all spaces from the given string.
///
/// # Arguments
///
/// * `s` - The string to remove spaces from
///
/// # Returns
///
/// The string with all spaces removed
///
/// # Examples
///
/// ```
/// let s = String::from("   Errare   humanum   est   ");
/// assert_eq!(strspx(s), "Errarehumanumest");
///
/// let s = String::from("      ");
/// assert_eq!(strspx(s), "");
/// ```
pub fn strspx(mut s: String) -> String {
    s.retain(|c| c != ' ');
    s
}

/* eof */