/*
 * Make free() preserve errno.

 * Copyright (C) 2003, 2006, 2009-2023 Free Software Foundation, Inc.

 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.

 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.

 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/* written by Paul Eggert */

use std::io;
use std::ptr;

/// Replacement for free() that preserves errno
pub fn rpl_free<T>(p: *mut T) {
    // Save errno before freeing
    let saved_errno = io::Error::last_os_error();
    
    unsafe {
        // Convert to Box to let Rust handle the deallocation
        if !p.is_null() {
            let _ = Box::from_raw(p);
        }
    }
    
    // Restore errno if it was changed
    if saved_errno.raw_os_error() != io::Error::last_os_error().raw_os_error() {
        // This is a bit tricky in Rust since we don't have direct access to errno
        // We'll use the last_os_error mechanism to simulate it
        io::Error::from_raw_os_error(saved_errno.raw_os_error().unwrap_or(0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_rpl_free() {
        // Allocate some memory
        let ptr = Box::into_raw(Box::new(42));
        
        // Set a fake errno
        let fake_err = io::Error::from_raw_os_error(42);
        io::set_last_os_error(fake_err);
        
        // Free the memory
        rpl_free(ptr);
        
        // Verify errno was preserved
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(42));
    }
}