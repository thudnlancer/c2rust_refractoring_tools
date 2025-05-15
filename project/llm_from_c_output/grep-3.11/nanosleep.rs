/*
 * Provide a replacement for the POSIX nanosleep function.
 *
 * Copyright (C) 1999-2000, 2002, 2004-2023 Free Software Foundation, Inc.
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

use std::time::{Duration, Instant};
use std::thread;
use std::io::Error;
use std::os::windows::prelude::*;
use winapi::um::profileapi::*;
use winapi::um::synchapi::*;

const BILLION: u64 = 1_000_000_000;

#[cfg(target_os = "linux")]
pub fn nanosleep(requested_delay: &Duration, remaining_delay: Option<&mut Duration>) -> Result<(), Error> {
    if requested_delay.subsec_nanos() >= BILLION as u32 {
        return Err(Error::from_raw_os_error(libc::EINVAL));
    }

    // Verify that Duration can handle large sleeps
    const LIMIT_SECS: u64 = 24 * 24 * 60 * 60;
    let mut seconds = requested_delay.as_secs();
    let mut nanos = requested_delay.subsec_nanos();

    while seconds > LIMIT_SECS {
        let intermediate = Duration::new(LIMIT_SECS, nanos);
        if let Err(e) = thread::sleep(intermediate) {
            if let Some(remaining) = remaining_delay {
                *remaining = Duration::new(seconds, nanos);
            }
            return Err(e);
        }
        seconds -= LIMIT_SECS;
        nanos = 0;
    }

    thread::sleep(Duration::new(seconds, nanos));
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn nanosleep(requested_delay: &Duration, remaining_delay: Option<&mut Duration>) -> Result<(), Error> {
    if requested_delay.subsec_nanos() >= BILLION as u32 {
        return Err(Error::from_raw_os_error(libc::EINVAL));
    }

    static INIT: std::sync::Once = std::sync::Once::new();
    static mut TICKS_PER_NANOSECOND: f64 = 0.0;

    // For delays >= 1 second, use Sleep with millisecond precision
    if requested_delay.as_secs() > 0 {
        let total_millis = requested_delay.as_secs() * 1000 + u64::from(requested_delay.subsec_nanos() / 1_000_000);
        unsafe { Sleep(total_millis as u32); }
    } else {
        INIT.call_once(|| {
            unsafe {
                let mut freq = 0;
                if QueryPerformanceFrequency(&mut freq) != 0 {
                    TICKS_PER_NANOSECOND = freq as f64 / 1_000_000_000.0;
                }
            }
        });

        unsafe {
            if TICKS_PER_NANOSECOND > 0.0 {
                let sleep_millis = (requested_delay.subsec_nanos() / 1_000_000).saturating_sub(10);
                let wait_ticks = (requested_delay.subsec_nanos() as f64 * TICKS_PER_NANOSECOND) as i64;
                
                let mut before = 0;
                if QueryPerformanceCounter(&mut before) != 0 {
                    let wait_until = before + wait_ticks;
                    
                    if sleep_millis > 0 {
                        Sleep(sleep_millis);
                    }
                    
                    loop {
                        let mut after = 0;
                        if QueryPerformanceCounter(&mut after) == 0 {
                            break;
                        }
                        if after >= wait_until {
                            break;
                        }
                    }
                }
            } else {
                Sleep((requested_delay.subsec_nanos() / 1_000_000) as u32);
            }
        }
    }

    if let Some(remaining) = remaining_delay {
        *remaining = Duration::new(0, 0);
    }
    Ok(())
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
pub fn nanosleep(requested_delay: &Duration, _remaining_delay: Option<&mut Duration>) -> Result<(), Error> {
    thread::sleep(*requested_delay);
    Ok(())
}