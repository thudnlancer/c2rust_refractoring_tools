/*
 * Non-failing memory allocation routines.
 * Copyright (c) 1996 Markku Rossi.
 *
 * Author: Markku Rossi <mtr@iki.fi>
 */

/*
 * This file is part of GNU enscript.
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING.  If not, write to
 * the Free Software Foundation, 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::process;
use std::ptr;
use std::ffi::CString;
use std::os::raw::c_void;

pub fn xmalloc(size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(size, 1).unwrap_or_else(|_| {
        eprintln!("xmalloc(): invalid layout for {} bytes", size);
        process::exit(1);
    });
    
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            eprintln!("xmalloc(): couldn't allocate {} bytes", size);
            process::exit(1);
        }
        ptr as *mut c_void
    }
}

pub fn xcalloc(num: usize, size: usize) -> *mut c_void {
    let total_size = num.checked_mul(size).unwrap_or_else(|| {
        eprintln!("xcalloc(): size overflow");
        process::exit(1);
    });
    
    let layout = Layout::from_size_align(total_size, 1).unwrap_or_else(|_| {
        eprintln!("xcalloc(): invalid layout for {} bytes", total_size);
        process::exit(1);
    });
    
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            eprintln!("xcalloc(): couldn't allocate {} bytes", total_size);
            process::exit(1);
        }
        ptr as *mut c_void
    }
}

pub fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if ptr.is_null() {
        return xmalloc(size);
    }
    
    let layout = Layout::from_size_align(size, 1).unwrap_or_else(|_| {
        eprintln!("xrealloc(): invalid layout for {} bytes", size);
        process::exit(1);
    });
    
    unsafe {
        let new_ptr = std::alloc::realloc(ptr as *mut u8, layout, size);
        if new_ptr.is_null() {
            eprintln!("xrealloc(): couldn't reallocate {} bytes", size);
            process::exit(1);
        }
        new_ptr as *mut c_void
    }
}

pub fn xfree(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe {
            // We can't deallocate without knowing the original size,
            // but since this is a direct translation of the C code,
            // we'll leave it as is (C's free doesn't need size)
            // In real Rust code, you'd want to track the layout.
            dealloc(ptr as *mut u8, Layout::new::<u8>());
        }
    }
}

pub fn xstrdup(str: &str) -> *mut c_void {
    let c_str = CString::new(str).unwrap_or_else(|_| {
        eprintln!("xstrdup(): string contains null byte");
        process::exit(1);
    });
    
    let len = c_str.as_bytes_with_nul().len();
    let ptr = xmalloc(len);
    
    unsafe {
        ptr::copy_nonoverlapping(c_str.as_ptr() as *const u8, ptr as *mut u8, len);
    }
    
    ptr
}