/*
 * POSIX compatible signal blocking for threads.
 * Copyright (C) 2011-2022 Free Software Foundation, Inc.
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

use std::io;
use std::thread;
use std::time::Duration;
use nix::sys::signal::{self, SigSet, SigmaskHow};

#[cfg(not(target_os = "linux"))]
use nix::errno::Errno;

pub fn pthread_sigmask(
    how: SigmaskHow,
    new_mask: Option<&SigSet>,
    old_mask: Option<&mut SigSet>,
) -> Result<(), Errno> {
    #[cfg(not(target_os = "linux"))]
    {
        if cfg!(PTHREAD_SIGMASK_INEFFECTIVE) {
            let mut omask = SigSet::empty();
            omask.add(signal::SIGILL);
            let omask_copy = omask.clone();

            let result = signal::pthread_sigmask(how, new_mask, Some(&mut omask));

            if result.is_ok() {
                if omask == omask_copy && signal::pthread_sigmask(SigmaskHow::SIG_SETMASK, Some(&omask_copy), None).is_ok() {
                    // pthread_sigmask is ineffective, use sigprocmask
                    return signal::sigprocmask(how, new_mask, old_mask);
                }

                if let Some(old_mask) = old_mask {
                    *old_mask = omask;
                }
            }

            if cfg!(PTHREAD_SIGMASK_FAILS_WITH_ERRNO) && result == Err(Errno::UnknownErrno) {
                return Err(Errno::last());
            }

            if cfg!(PTHREAD_SIGMASK_UNBLOCK_BUG) && result.is_ok() && new_mask.is_some() && (how == SigmaskHow::SIG_UNBLOCK || how == SigmaskHow::SIG_SETMASK) {
                thread::sleep(Duration::from_micros(1));
            }

            result
        } else {
            signal::pthread_sigmask(how, new_mask, old_mask)
        }
    }

    #[cfg(target_os = "linux"))]
    {
        signal::pthread_sigmask(how, new_mask, old_mask)
    }
}