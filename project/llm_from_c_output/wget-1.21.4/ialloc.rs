/*
 * ialloc.rs -- malloc with idx_t rather than size_t
 *
 * Copyright 2021-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::num::NonZeroUsize;
use std::ptr::NonNull;

type Idx = isize; // Assuming idx_t is equivalent to isize

#[cold]
fn alloc_nomem() -> Option<NonNull<u8>> {
    std::io::Error::last_os_error().raw_os_error().and_then(|e| {
        if e == libc::ENOMEM as i32 {
            None
        } else {
            None
        }
    })
}

/// imalloc(size) is like malloc(size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns None.
pub fn imalloc(s: Idx) -> Option<NonNull<u8>> {
    if s <= 0 {
        return None;
    }
    let size = s as usize;
    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).ok()?;
    unsafe { NonNull::new(alloc(layout)) }
}

/// irealloc(ptr, size) is like realloc(ptr, size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns None.
pub fn irealloc(p: Option<NonNull<u8>>, s: Idx) -> Option<NonNull<u8>> {
    if s <= 0 {
        return None;
    }
    let size = s as usize;
    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>()).ok()?;
    
    match p {
        Some(ptr) => unsafe {
            NonNull::new(realloc(ptr.as_ptr(), layout, size))
        },
        None => unsafe { NonNull::new(alloc(layout)) },
    }
}

/// icalloc(num, size) is like calloc(num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns None.
pub fn icalloc(n: Idx, s: Idx) -> Option<NonNull<u8>> {
    if n <= 0 || s <= 0 {
        return None;
    }
    
    let num = n as usize;
    let size = s as usize;
    
    if let Some(nz_num) = NonZeroUsize::new(num) {
        if let Some(nz_size) = NonZeroUsize::new(size) {
            let total_size = nz_num.get().checked_mul(nz_size.get())?;
            let layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>()).ok()?;
            return unsafe { NonNull::new(alloc_zeroed(layout)) };
        }
    }
    None
}

/// ireallocarray(ptr, num, size) is like reallocarray(ptr, num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns None.
pub fn ireallocarray(p: Option<NonNull<u8>>, n: Idx, s: Idx) -> Option<NonNull<u8>> {
    if n <= 0 || s <= 0 {
        return None;
    }
    
    let num = n as usize;
    let size = s as usize;
    
    if let Some(nz_num) = NonZeroUsize::new(num) {
        if let Some(nz_size) = NonZeroUsize::new(size) {
            let total_size = nz_num.get().checked_mul(nz_size.get())?;
            let layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>()).ok()?;
            
            match p {
                Some(ptr) => unsafe {
                    NonNull::new(realloc(ptr.as_ptr(), layout, total_size))
                },
                None => unsafe { NonNull::new(alloc(layout)) },
            }
        } else {
            None
        }
    } else {
        None
    }
}

/*
 * malloc with idx_t rather than size_t
 *
 * Copyright 2021-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */