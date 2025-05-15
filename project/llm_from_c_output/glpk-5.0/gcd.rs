// gcd.rs (greatest common divisor)

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

/***********************************************************************
*  NAME
*
*  gcd - find greatest common divisor of two integers
*
*  SYNOPSIS
*
*  fn gcd(x: i32, y: i32) -> i32;
*
*  RETURNS
*
*  The routine gcd returns gcd(x, y), the greatest common divisor of
*  the two positive integers given.
*
*  ALGORITHM
*
*  The routine gcd is based on Euclid's algorithm.
*
*  REFERENCES
*
*  Don Knuth, The Art of Computer Programming, Vol.2: Seminumerical
*  Algorithms, 3rd Edition, Addison-Wesley, 1997. Section 4.5.2: The
*  Greatest Common Divisor, pp. 333-56. */

pub fn gcd(x: i32, y: i32) -> i32 {
    assert!(x > 0 && y > 0, "x and y must be positive integers");
    let mut x = x;
    let mut y = y;
    while y > 0 {
        let r = x % y;
        x = y;
        y = r;
    }
    x
}

/***********************************************************************
*  NAME
*
*  gcdn - find greatest common divisor of n integers
*
*  SYNOPSIS
*
*  fn gcdn(n: usize, x: &[i32]) -> i32;
*
*  RETURNS
*
*  The routine gcdn returns gcd(x[0], x[1], ..., x[n-1]), the greatest
*  common divisor of n positive integers given, n > 0.
*
*  BACKGROUND
*
*  The routine gcdn is based on the following identity:
*
*     gcd(x, y, z) = gcd(gcd(x, y), z).
*
*  REFERENCES
*
*  Don Knuth, The Art of Computer Programming, Vol.2: Seminumerical
*  Algorithms, 3rd Edition, Addison-Wesley, 1997. Section 4.5.2: The
*  Greatest Common Divisor, pp. 333-56. */

pub fn gcdn(n: usize, x: &[i32]) -> i32 {
    assert!(n > 0, "n must be greater than 0");
    assert!(x.len() >= n, "x must have at least n elements");
    
    let mut d = 0;
    for j in 0..n {
        assert!(x[j] > 0, "all elements must be positive integers");
        if j == 0 {
            d = x[0];
        } else {
            d = gcd(d, x[j]);
        }
        if d == 1 {
            break;
        }
    }
    d
}

/* eof */