/*
 * extendbuf.rs -- Manage a dynamically-allocated buffer
 *
 * Copyright (C) 2004-2022 Free Software Foundation, Inc.
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

const SIZE_DEFAULT: usize = 16;

fn decide_size(current: usize, wanted: usize) -> usize {
    let mut newsize = if current == 0 { SIZE_DEFAULT } else { current };

    while newsize < wanted {
        match newsize.checked_mul(2) {
            Some(n) => newsize = n,
            None => return wanted,
        }
    }
    newsize
}

pub fn extendbuf(
    existing: Option<Vec<u8>>,
    wanted: usize,
    allocated: &mut usize,
) -> Result<Option<Vec<u8>>, std::io::Error> {
    assert!(wanted > 0);
    let newsize = decide_size(*allocated, wanted);

    let result = if *allocated == 0 {
        assert!(existing.is_none());
        *allocated = newsize;
        Some(Vec::with_capacity(newsize))
    } else {
        if newsize != *allocated {
            *allocated = newsize;
            let mut new_vec = existing.unwrap_or_default();
            new_vec.try_reserve_exact(newsize - new_vec.capacity())?;
            Some(new_vec)
        } else {
            existing
        }
    };

    Ok(result)
}

pub fn xextendbuf(
    existing: Option<Vec<u8>>,
    wanted: usize,
    allocated: &mut usize,
) -> Option<Vec<u8>> {
    extendbuf(existing, wanted, allocated).unwrap_or_else(|_| {
        std::process::exit(1);
    })
}

type Result<T> = std::result::Result<T, std::io::Error>;