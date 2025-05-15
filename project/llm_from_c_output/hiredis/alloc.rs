/*
 * Copyright (c) 2020, Michael Grunder <michael dot grunder at gmail dot com>
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright notice,
 *     this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *   * Neither the name of Redis nor the names of its contributors may be used
 *     to endorse or promote products derived from this software without
 *     specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};
use std::ffi::{CString, CStr};
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct HiredisAllocFuncs {
    pub malloc_fn: fn(usize) -> *mut c_void,
    pub calloc_fn: fn(usize, usize) -> *mut c_void,
    pub realloc_fn: fn(*mut c_void, usize) -> *mut c_void,
    pub strdup_fn: fn(*const i8) -> *mut i8,
    pub free_fn: fn(*mut c_void),
}

impl Default for HiredisAllocFuncs {
    fn default() -> Self {
        HiredisAllocFuncs {
            malloc_fn: default_malloc,
            calloc_fn: default_calloc,
            realloc_fn: default_realloc,
            strdup_fn: default_strdup,
            free_fn: default_free,
        }
    }
}

pub static mut HIREDIS_ALLOC_FNS: HiredisAllocFuncs = HiredisAllocFuncs {
    malloc_fn: default_malloc,
    calloc_fn: default_calloc,
    realloc_fn: default_realloc,
    strdup_fn: default_strdup,
    free_fn: default_free,
};

fn default_malloc(size: usize) -> *mut c_void {
    unsafe { alloc(mem::size_of::<u8>() * size) as *mut c_void }
}

fn default_calloc(nmemb: usize, size: usize) -> *mut c_void {
    if size > 0 && nmemb > usize::MAX / size {
        return ptr::null_mut();
    }
    unsafe { alloc_zeroed(mem::size_of::<u8>() * size * nmemb) as *mut c_void }
}

fn default_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe { realloc(ptr as *mut u8, mem::size_of::<u8>() * size) as *mut c_void }
}

fn default_strdup(str: *const i8) -> *mut i8 {
    unsafe {
        let c_str = CStr::from_ptr(str);
        match CString::new(c_str.to_bytes()) {
            Ok(s) => s.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    }
}

fn default_free(ptr: *mut c_void) {
    unsafe {
        dealloc(ptr as *mut u8, mem::size_of::<u8>());
    }
}

pub fn hiredis_set_allocators(override_fns: HiredisAllocFuncs) -> HiredisAllocFuncs {
    unsafe {
        let orig = HIREDIS_ALLOC_FNS;
        HIREDIS_ALLOC_FNS = override_fns;
        orig
    }
}

pub fn hiredis_reset_allocators() {
    unsafe {
        HIREDIS_ALLOC_FNS = HiredisAllocFuncs::default();
    }
}

pub fn hi_malloc(size: usize) -> *mut c_void {
    unsafe { (HIREDIS_ALLOC_FNS.malloc_fn)(size) }
}

pub fn hi_calloc(nmemb: usize, size: usize) -> *mut c_void {
    if size > 0 && nmemb > usize::MAX / size {
        return ptr::null_mut();
    }
    unsafe { (HIREDIS_ALLOC_FNS.calloc_fn)(nmemb, size) }
}

pub fn hi_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe { (HIREDIS_ALLOC_FNS.realloc_fn)(ptr, size) }
}

pub fn hi_strdup(str: *const i8) -> *mut i8 {
    unsafe { (HIREDIS_ALLOC_FNS.strdup_fn)(str) }
}

pub fn hi_free(ptr: *mut c_void) {
    unsafe { (HIREDIS_ALLOC_FNS.free_fn)(ptr) }
}