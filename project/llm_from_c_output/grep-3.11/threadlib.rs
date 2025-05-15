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

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[cfg(any(
    target_os = "freebsd",
    target_os = "dragonfly"
))]
fn glthread_in_use() -> bool {
    use std::sync::Once;
    use std::thread_local;

    static TESTED: Once = Once::new();
    static RESULT: AtomicBool = AtomicBool::new(false);

    TESTED.call_once(|| {
        thread_local! {
            static KEY: std::thread::LocalKey<()> = std::thread::LocalKey::new(|| ());
        }

        KEY.with(|_| {
            RESULT.store(true, Ordering::Relaxed);
        });
    });

    RESULT.load(Ordering::Relaxed)
}

#[cfg(not(any(
    target_os = "freebsd",
    target_os = "dragonfly"
)))]
fn glthread_in_use() -> bool {
    use std::sync::Once;

    static TESTED: Once = Once::new();
    static RESULT: AtomicBool = AtomicBool::new(false);

    TESTED.call_once(|| {
        let handle = thread::spawn(|| ());

        match handle.join() {
            Ok(_) => RESULT.store(true, Ordering::Relaxed),
            Err(_) => RESULT.store(false, Ordering::Relaxed),
        }
    });

    RESULT.load(Ordering::Relaxed)
}

/* This declaration is solely to ensure that after preprocessing
   this file is never empty. */
struct Dummy;