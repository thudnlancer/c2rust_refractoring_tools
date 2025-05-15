/*
 * Multithreading primitives.
 * Copyright (C) 2005-2023 Free Software Foundation, Inc.
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

/* Written by Bruno Haible <bruno@clisp.org>, 2005. */

// =========================================================================

#[cfg(any(feature = "posix_threads", feature = "isoc_and_posix_threads"))]
mod posix_threads {
    use std::sync::Once;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;

    #[cfg(target_os = "freebsd")]
    pub fn glthread_in_use() -> bool {
        static TESTED: Once = Once::new();
        static RESULT: AtomicBool = AtomicBool::new(false);

        TESTED.call_once(|| {
            let result = std::panic::catch_unwind(|| {
                let mut key = std::mem::MaybeUninit::uninit();
                unsafe {
                    let err = libc::pthread_key_create(key.as_mut_ptr(), None);
                    if err == libc::ENOSYS {
                        false
                    } else {
                        if err == 0 {
                            libc::pthread_key_delete(key.assume_init());
                        }
                        true
                    }
                }
            }).unwrap_or(false);
            RESULT.store(result, Ordering::Relaxed);
        });

        RESULT.load(Ordering::Relaxed)
    }

    #[cfg(not(target_os = "freebsd"))]
    pub fn glthread_in_use() -> bool {
        static TESTED: Once = Once::new();
        static RESULT: AtomicBool = AtomicBool::new(false);

        TESTED.call_once(|| {
            let result = thread::spawn(|| ()).is_ok();
            RESULT.store(result, Ordering::Relaxed);
        });

        RESULT.load(Ordering::Relaxed)
    }
}

// =========================================================================

// This declaration is solely to ensure that after preprocessing
// this file is never empty.
#[allow(dead_code)]
type Dummy = i32;