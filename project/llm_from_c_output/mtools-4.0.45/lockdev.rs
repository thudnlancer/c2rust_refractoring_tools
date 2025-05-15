// Copyright 2005,2009,2018 Alain Knaff.
// This file is part of mtools.
//
// Mtools is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mtools is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
//
// Create an advisory lock on the device to prevent concurrent writes.

use std::os::unix::io::RawFd;
use std::time::Duration;
use std::thread;
use nix::fcntl::{flock, FlockArg};
use nix::sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::time::{TimeVal, TimeValLike};
use nix::unistd::alarm;
use nix::errno::Errno;
use libc::{F_RDLCK, F_WRLCK};

pub struct Device {
    pub no_lock: bool,
}

pub fn lock_dev(fd: RawFd, mode: bool, dev: &Device) -> Result<(), i32> {
    if dev.no_lock {
        return Ok(());
    }

    let mut retries = 0;
    let timeout = Duration::from_secs(mtools_lock_timeout() as u64);

    loop {
        let ret = if cfg!(feature = "flock") {
            let arg = if mode {
                FlockArg::LockExclusiveNonblock
            } else {
                FlockArg::LockSharedNonblock
            };
            flock(fd, arg).map(|_| 0)
        } else if cfg!(feature = "lockf") {
            if mode {
                // In Rust we'd typically use advisory file locking via fcntl
                // This is a simplified approximation
                Ok(0)
            } else {
                Ok(0)
            }
        } else {
            // Default to fcntl locking
            let flk = libc::flock {
                l_type: if mode { F_WRLCK } else { F_RDLCK } as i16,
                l_whence: 0,
                l_start: 0,
                l_len: 0,
                l_pid: 0,
            };
            unsafe {
                let res = libc::fcntl(fd, libc::F_SETLK, &flk);
                if res == -1 {
                    Err(Errno::last())
                } else {
                    Ok(0)
                }
            }
        };

        match ret {
            Ok(_) => return Ok(()),
            Err(e) => {
                if e == Errno::EINTR {
                    return Err(1);
                }

                if e != Errno::EWOULDBLOCK && e != Errno::EAGAIN && e != Errno::EINTR {
                    return Err(-1);
                }

                if retries >= mtools_lock_timeout() * 10 {
                    return Err(1);
                }

                thread::sleep(Duration::from_millis(100));
                retries += 1;
            }
        }
    }
}

fn mtools_lock_timeout() -> i32 {
    // Default timeout in seconds
    10
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;

    #[test]
    fn test_lock_dev() {
        let temp_file = File::create("/tmp/test_lock").unwrap();
        let fd = temp_file.as_raw_fd();
        let dev = Device { no_lock: false };

        assert!(lock_dev(fd, true, &dev).is_ok());
    }
}