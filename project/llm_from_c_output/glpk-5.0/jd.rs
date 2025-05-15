/* jd.rs (conversions between calendar date and Julian day number) */

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

/// Convert calendar date to Julian day number
///
/// # Arguments
/// * `d` - day (1-31)
/// * `m` - month (1-12)
/// * `y` - year (1-4000)
///
/// # Returns
/// Returns `Ok(j)` where `j` is the Julian day number if the date is valid,
/// or `Err(())` if the date is invalid.
///
/// # References
/// R. G. Tantzen, Algorithm 199: conversions between calendar date and
/// Julian day number, Communications of the ACM, vol. 6, no. 8, p. 444,
/// Aug. 1963.
pub fn jday(d: i32, m: i32, y: i32) -> Result<i32, ()> {
    if !(1 <= d && d <= 31 && 1 <= m && m <= 12 && 1 <= y && y <= 4000) {
        return Err(());
    }

    let (m, y) = if m >= 3 {
        (m - 3, y)
    } else {
        (m + 9, y - 1)
    };

    let c = y / 100;
    let ya = y - 100 * c;
    let j = (146097 * c) / 4 + (1461 * ya) / 4 + (153 * m + 2) / 5 + d + 1721119;

    let mut dd = 0;
    jdate(j, Some(&mut dd), None, None)?;
    if d != dd {
        return Err(());
    }

    Ok(j)
}

/// Convert Julian day number to calendar date
///
/// # Arguments
/// * `j` - Julian day number (1721426-3182395)
/// * `d` - optional mutable reference to store day
/// * `m` - optional mutable reference to store month
/// * `y` - optional mutable reference to store year
///
/// # Returns
/// Returns `Ok(())` if the conversion is successful, or `Err(())` if the
/// Julian day number is invalid.
///
/// # References
/// R. G. Tantzen, Algorithm 199: conversions between calendar date and
/// Julian day number, Communications of the ACM, vol. 6, no. 8, p. 444,
/// Aug. 1963.
pub fn jdate(j: i32, d: Option<&mut i32>, m: Option<&mut i32>, y: Option<&mut i32>) -> Result<(), ()> {
    if !(1721426 <= j && j <= 3182395) {
        return Err(());
    }

    let mut j = j - 1721119;
    let mut y = (4 * j - 1) / 146097;
    j = (4 * j - 1) % 146097;
    let mut d_val = j / 4;
    j = (4 * d_val + 3) / 1461;
    d_val = (4 * d_val + 3) % 1461;
    d_val = (d_val + 4) / 4;
    let mut m_val = (5 * d_val - 3) / 153;
    d_val = (5 * d_val - 3) % 153;
    d_val = (d_val + 5) / 5;
    y = 100 * y + j;

    if m_val <= 9 {
        m_val += 3;
    } else {
        m_val -= 9;
        y += 1;
    }

    if let Some(d_ref) = d {
        *d_ref = d_val;
    }
    if let Some(m_ref) = m {
        *m_ref = m_val;
    }
    if let Some(y_ref) = y {
        *y_ref = y;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jday_jdate() {
        let jbeg = jday(1, 1, 1).unwrap();
        let jend = jday(31, 12, 4000).unwrap();
        
        for j in jbeg..=jend {
            let mut d = 0;
            let mut m = 0;
            let mut y = 0;
            assert!(jdate(j, Some(&mut d), Some(&mut m), Some(&mut y)).is_ok());
            assert_eq!(jday(d, m, y).unwrap(), j);
        }
    }
}