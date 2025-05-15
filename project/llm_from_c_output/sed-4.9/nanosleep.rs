// Provide a replacement for the POSIX nanosleep function.

// Copyright (C) 1999-2000, 2002, 2004-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// written by Jim Meyering
// and Bruno Haible for the native Windows part

use std::time::{Duration, Instant};
use std::thread;
use std::io::{Error, ErrorKind};
use std::convert::TryInto;

const BILLION: u64 = 1_000_000_000;

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
    use winapi::um::synchapi::Sleep;
    use winapi::shared::minwindef::DWORD;
    use std::mem::MaybeUninit;
    use std::ptr;

    pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
        if requested_delay.subsec_nanos() >= BILLION as u32 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid nanoseconds value"));
        }

        // For delays >= 1s, use Sleep with millisecond precision
        if requested_delay.as_secs() > 0 {
            let total_millis = requested_delay.as_secs() * 1000 + u64::from(requested_delay.subsec_millis());
            unsafe { Sleep(total_millis as DWORD); }
            return Ok(());
        }

        // For subsecond delays, use high precision timing
        static mut INITIALIZED: bool = false;
        static mut TICKS_PER_NANOSECOND: f64 = 0.0;

        unsafe {
            if !INITIALIZED {
                let mut freq = MaybeUninit::uninit();
                if QueryPerformanceFrequency(freq.as_mut_ptr()) != 0 {
                    TICKS_PER_NANOSECOND = f64::from(freq.assume_init().QuadPart) / 1_000_000_000.0;
                }
                INITIALIZED = true;
            }

            if TICKS_PER_NANOSECOND > 0.0 {
                let wait_ticks = (requested_delay.subsec_nanos() as f64 * TICKS_PER_NANOSECOND) as i64;
                let mut before = MaybeUninit::uninit();
                if QueryPerformanceCounter(before.as_mut_ptr()) == 0 {
                    return Err(Error::last_os_error());
                }

                let wait_until = before.assume_init().QuadPart + wait_ticks;

                // Sleep for most of the duration (minus 10ms for safety)
                let sleep_millis = requested_delay.subsec_millis().saturating_sub(10);
                if sleep_millis > 0 {
                    Sleep(sleep_millis as DWORD);
                }

                // Busy-wait for remaining time
                loop {
                    let mut after = MaybeUninit::uninit();
                    if QueryPerformanceCounter(after.as_mut_ptr()) == 0 {
                        return Err(Error::last_os_error());
                    }
                    if after.assume_init().QuadPart >= wait_until {
                        break;
                    }
                }
                return Ok(());
            }
        }

        // Fallback to Sleep if high precision timing failed
        let total_millis = requested_delay.as_millis().try_into().unwrap_or(u32::MAX);
        unsafe { Sleep(total_millis as DWORD); }
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
mod unix {
    use super::*;
    use nix::sys::time::TimeValLike;
    use nix::sys::select::{pselect, FdSet};
    use nix::time::{TimeSpec, ClockId};

    pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
        if requested_delay.subsec_nanos() >= BILLION as u32 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid nanoseconds value"));
        }

        // Break down large sleeps into smaller chunks (24 days max)
        const LIMIT_SECS: u64 = 24 * 24 * 60 * 60;
        let mut remaining = *requested_delay;

        while remaining.as_secs() > LIMIT_SECS {
            let chunk = Duration::new(LIMIT_SECS, 0);
            thread::sleep(chunk);
            remaining = remaining.saturating_sub(chunk);
        }

        // Handle remaining time
        thread::sleep(remaining);
        Ok(())
    }
}

pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    {
        windows::nanosleep(requested_delay)
    }
    #[cfg(not(target_os = "windows"))]
    {
        unix::nanosleep(requested_delay)
    }
}