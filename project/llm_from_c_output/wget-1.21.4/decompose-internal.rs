// Decomposition of Unicode strings.
// Copyright (C) 2009-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2009.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// A Unicode character together with its canonical combining class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ucs4WithCcc {
    pub code: u32,
    pub ccc: u8, // range 0..255
}

/// Variant of uc_decomposition that does not produce the 'tag'.
pub fn uc_compat_decomposition(uc: u32, decomposition: &mut [u32]) -> Result<usize, &'static str> {
    // Implementation should be provided based on actual decomposition logic
    Err("Not implemented")
}

/// Stable-sort an array of `Ucs4WithCcc` in place using merge sort.
pub fn gl_uninorm_decompose_merge_sort_inplace(
    src: &mut [Ucs4WithCcc],
    tmp: &mut [Ucs4WithCcc],
) -> Result<(), &'static str> {
    if src.len() != tmp.len() {
        return Err("Source and temporary buffers must have the same length");
    }
    
    merge_sort(src, tmp);
    Ok(())
}

fn merge_sort(arr: &mut [Ucs4WithCcc], tmp: &mut [Ucs4WithCcc]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid], &mut tmp[..mid]);
    merge_sort(&mut arr[mid..], &mut tmp[mid..]);

    merge(arr, mid, tmp);
}

fn merge(arr: &mut [Ucs4WithCcc], mid: usize, tmp: &mut [Ucs4WithCcc]) {
    let len = arr.len();
    tmp[..len].copy_from_slice(arr);

    let mut i = 0;
    let mut j = mid;
    let mut k = 0;

    while i < mid && j < len {
        if tmp[i].ccc <= tmp[j].ccc {
            arr[k] = tmp[i];
            i += 1;
        } else {
            arr[k] = tmp[j];
            j += 1;
        }
        k += 1;
    }

    if i < mid {
        arr[k..].copy_from_slice(&tmp[i..mid]);
    }
}