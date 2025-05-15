/* Multithreading primitives.
   Copyright (C) 2005-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Bruno Haible <bruno@clisp.org>, 2005.  */

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use std::thread;

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly"
))]
pub fn glthread_in_use() -> bool {
    static TESTED: Once = Once::new();
    static RESULT: AtomicBool = AtomicBool::new(false);

    TESTED.call_once(|| {
        let result = std::thread::spawn(|| {
            let key = std::thread::LocalKey::new(|_| ());
            true
        }).join().is_ok();
        
        RESULT.store(result, Ordering::Relaxed);
    });

    RESULT.load(Ordering::Relaxed)
}

#[cfg(any(
    target_os = "solaris",
    target_os = "hpux"
))]
pub fn glthread_in_use() -> bool {
    static TESTED: Once = Once::new();
    static RESULT: AtomicBool = AtomicBool::new(false);

    TESTED.call_once(|| {
        let result = thread::spawn(|| ()).join().is_ok();
        RESULT.store(result, Ordering::Relaxed);
    });

    RESULT.load(Ordering::Relaxed)
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "solaris",
    target_os = "hpux"
)))]
pub fn glthread_in_use() -> bool {
    false
}

/* This declaration is solely to ensure that after preprocessing
   this file is never empty.  */
type Dummy = i32;