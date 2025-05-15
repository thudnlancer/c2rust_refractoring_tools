// Report an dynamic array index out of bounds condition.
// Copyright (C) 2017-2023 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// The GNU C Library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// The GNU C Library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with the GNU C Library; if not, see
// <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "libc"))]
use std::process::abort;

pub fn libc_dynarray_at_failure(size: usize, index: usize) -> ! {
    #[cfg(feature = "libc")]
    {
        use std::process::abort;
        let msg = format!(
            "Fatal glibc error: array index {} not less than array length {}\n",
            index, size
        );
        eprintln!("{}", msg);
        abort();
    }
    
    #[cfg(not(feature = "libc"))]
    {
        abort();
    }
}