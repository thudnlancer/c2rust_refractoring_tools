/* -*- buffer-read-only: t -*- vi: set ro: */
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* xmalloc.rs -- malloc with out of memory checking

   Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995, 1996, 1997, 1998,
   1999, 2000, 2002, 2003, 2004, 2005, 2006, 2008-2009 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem;

/// 1 if calloc is known to be compatible with GNU calloc.
const HAVE_GNU_CALLOC: bool = cfg!(any(target_env = "gnu", feature = "gnu-calloc"));

/// Allocate N bytes of memory dynamically, with error checking.
pub fn xmalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let p = alloc(layout);
        if p.is_null() {
            xalloc_die();
        }
        p
    }
}

/// Change the size of an allocated block of memory P to N bytes,
/// with error checking.
pub fn xrealloc(p: *mut u8, n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    if p.is_null() {
        return xmalloc(n);
    }
    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let p = realloc(p, layout, n);
        if p.is_null() {
            xalloc_die();
        }
        p
    }
}

/// If P is null, allocate a block of at least *PN bytes; otherwise,
/// reallocate P so that it contains more than *PN bytes. *PN must be
/// nonzero unless P is null. Set *PN to the new block's size, and
/// return the pointer to the new block. *PN is never set to zero, and
/// the returned pointer is never null.
pub fn x2realloc(p: *mut u8, pn: &mut usize) -> *mut u8 {
    x2nrealloc(p, pn, 1)
}

/// Allocate S bytes of zeroed memory dynamically, with error checking.
pub fn xzalloc(s: usize) -> *mut u8 {
    if s == 0 {
        return null_mut();
    }
    let layout = Layout::from_size_align(s, mem::align_of::<u8>()).unwrap();
    unsafe {
        let p = alloc_zeroed(layout);
        if p.is_null() {
            xalloc_die();
        }
        p
    }
}

/// Allocate zeroed memory for N elements of S bytes, with error checking.
/// S must be nonzero.
pub fn xcalloc(n: usize, s: usize) -> *mut u8 {
    if s == 0 {
        return null_mut();
    }
    if !HAVE_GNU_CALLOC && xalloc_oversized(n, s) {
        xalloc_die();
    }
    let layout = Layout::from_size_align(n * s, mem::align_of::<u8>()).unwrap();
    unsafe {
        let p = alloc_zeroed(layout);
        if p.is_null() && (HAVE_GNU_CALLOC || n != 0) {
            xalloc_die();
        }
        p
    }
}

/// Clone an object P of size S, with error checking.
pub fn xmemdup(p: *const u8, s: usize) -> *mut u8 {
    let new_p = xmalloc(s);
    unsafe {
        copy_nonoverlapping(p, new_p, s);
    }
    new_p
}

/// Clone STRING.
pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len() + 1;
    let new_p = xmalloc(len);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), new_p, len - 1);
        *new_p.add(len - 1) = 0;
    }
    new_p
}

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

fn xalloc_oversized(n: usize, s: usize) -> bool {
    n > usize::MAX / s
}

fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8 {
    if *pn == 0 {
        *pn = 1;
        return xmalloc(s);
    }
    *pn = if *pn > usize::MAX / 2 / s {
        usize::MAX / s
    } else {
        *pn * 2
    };
    xrealloc(p, *pn * s)
}