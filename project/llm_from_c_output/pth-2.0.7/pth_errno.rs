/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_errno.c: Pth errno support
*/
                             /* Steinbach's Guideline for Systems Programming:
                                ``Never test for an error condition
                                  you don't know how to handle.''            */

use std::io;
use std::sync::atomic::{AtomicI32, Ordering};

thread_local! {
    static PTH_ERRNO_STORAGE: AtomicI32 = AtomicI32::new(0);
    static PTH_ERRNO_FLAG: AtomicI32 = AtomicI32::new(0);
}

macro_rules! pth_shield {
    ($block:block) => {{
        PTH_ERRNO_STORAGE.with(|storage| {
            PTH_ERRNO_FLAG.with(|flag| {
                let old_errno = io::Error::last_os_error().raw_os_error().unwrap_or(0);
                storage.store(old_errno, Ordering::SeqCst);
                flag.store(1, Ordering::SeqCst);
                
                let result = $block;
                
                if flag.load(Ordering::SeqCst) != 0 {
                    let stored_errno = storage.load(Ordering::SeqCst);
                    if stored_errno != 0 {
                        io::Error::from_raw_os_error(stored_errno);
                    }
                    flag.store(0, Ordering::SeqCst);
                }
                
                result
            })
        })
    }};
}

#[cfg(debug_assertions)]
macro_rules! pth_error {
    ($return_val:expr, $errno_val:expr) => {{
        let err = io::Error::from_raw_os_error($errno_val);
        eprintln!(
            "return 0x{:x} with errno {}(\"{}\")",
            $return_val as usize,
            err.raw_os_error().unwrap_or(0),
            err
        );
        $return_val
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! pth_error {
    ($return_val:expr, $errno_val:expr) => {{
        let _ = io::Error::from_raw_os_error($errno_val);
        $return_val
    }};
}