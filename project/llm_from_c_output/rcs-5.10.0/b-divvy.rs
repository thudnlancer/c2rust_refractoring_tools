/*
 * b-divvy.rs --- dynamic memory management
 *
 * Copyright (C) 2010-2020 Thien-Thi Nguyen
 *
 * This file is part of GNU RCS.
 *
 * GNU RCS is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * GNU RCS is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty
 * of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::fmt;
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Divvy {
    name: &'static str,
    space: Vec<u8>,
    first: Option<*mut u8>,
    count: usize,
}

pub static mut PLEXUS: Option<Box<Divvy>> = None;
pub static mut SINGLE: Option<Box<Divvy>> = None;

pub fn make_space(name: &'static str) -> Box<Divvy> {
    let mut divvy = Box::new(Divvy {
        name,
        space: Vec::new(),
        first: None,
        count: 0,
    });

    divvy.first = Some(divvy.space.as_mut_ptr());
    divvy
}

pub fn alloc(divvy: &mut Divvy, len: usize) -> &mut [u8] {
    divvy.count += 1;
    let old_len = divvy.space.len();
    divvy.space.resize(old_len + len, 0);
    &mut divvy.space[old_len..]
}

pub fn zlloc(divvy: &mut Divvy, len: usize) -> &mut [u8] {
    let mem = alloc(divvy, len);
    for byte in mem.iter_mut() {
        *byte = 0;
    }
    mem
}

pub fn intern(divvy: &mut Divvy, s: &str, len: usize) -> &str {
    divvy.count += 1;
    let old_len = divvy.space.len();
    divvy.space.extend_from_slice(s.as_bytes());
    str::from_utf8(&divvy.space[old_len..old_len + len]).unwrap()
}

pub fn brush_off(divvy: &mut Divvy, ptr: *mut u8) {
    divvy.count -= 1;
    // In Rust, we don't manually free memory - it's handled by Vec
}

pub fn forget(divvy: &mut Divvy) {
    divvy.space.clear();
    divvy.count = 0;
    divvy.first = Some(divvy.space.as_mut_ptr());
}

pub fn accf(divvy: &mut Divvy, fmt: fmt::Arguments) {
    use std::fmt::Write;
    let _ = write!(&mut divvy.space, "{}", fmt);
}

pub fn accumulate_nbytes(divvy: &mut Divvy, start: &[u8], count: usize) {
    divvy.space.extend_from_slice(&start[..count]);
}

pub fn accumulate_byte(divvy: &mut Divvy, c: u8) {
    divvy.space.push(c);
}

pub fn accumulate_range(divvy: &mut Divvy, beg: &[u8], end: &[u8]) {
    let len = end.as_ptr() as usize - beg.as_ptr() as usize;
    divvy.space.extend_from_slice(&beg[..len]);
}

pub fn accs(divvy: &mut Divvy, string: &str) {
    divvy.space.extend_from_slice(string.as_bytes());
}

pub fn finish_string(divvy: &mut Divvy, result_len: &mut usize) -> &str {
    *result_len = divvy.space.len();
    divvy.space.push(b'\0');
    let s = unsafe { str::from_utf8_unchecked(&divvy.space) };
    let result = &s[..*result_len];
    divvy.space.clear();
    result
}

pub fn pointer_array(divvy: &mut Divvy, count: usize) -> &mut [*mut u8] {
    let len = count * mem::size_of::<*mut u8>();
    let mem = alloc(divvy, len);
    unsafe {
        slice::from_raw_parts_mut(mem.as_mut_ptr() as *mut *mut u8, count)
    }
}

pub fn close_space(divvy: Box<Divvy>) {
    // Memory is automatically freed when Box goes out of scope
}

// Idioms
pub fn plexus() -> &'static mut Divvy {
    unsafe { PLEXUS.as_deref_mut().unwrap() }
}

pub fn single() -> &'static mut Divvy {
    unsafe { SINGLE.as_deref_mut().unwrap() }
}

pub fn zalloc<T>(n: usize) -> &'static mut [T] {
    let len = n * mem::size_of::<T>();
    let bytes = zlloc(plexus(), len);
    unsafe {
        slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut T, n)
    }
}

pub fn falloc<T>() -> &'static mut T {
    let len = mem::size_of::<T>();
    let bytes = alloc(single(), len);
    unsafe { &mut *(bytes.as_mut_ptr() as *mut T) }
}

pub fn fzalloc<T>() -> &'static mut T {
    let len = mem::size_of::<T>();
    let bytes = zlloc(single(), len);
    unsafe { &mut *(bytes.as_mut_ptr() as *mut T) }
}

pub fn shaccr(beg: &[u8], end: &[u8]) {
    accumulate_range(plexus(), beg, end);
}

pub fn shstr(result_len: &mut usize) -> &'static str {
    finish_string(plexus(), result_len)
}

pub fn shsnip(result_len: &mut usize, beg: &[u8], end: &[u8]) -> &'static str {
    shaccr(beg, end);
    shstr(result_len)
}