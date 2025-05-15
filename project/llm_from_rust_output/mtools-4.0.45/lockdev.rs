use std::os::unix::io::RawFd;
use std::thread;
use std::time::Duration;
use nix::sys::signal::{self, SigHandler, Signal};
use nix::fcntl::{flock, FlockArg};
use nix::errno::Errno;

#[derive(Debug, Clone, Copy)]
pub struct Device {
    pub name: Option<String>,
    pub drive: char,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: u16,
    pub sectors: u16,
    pub hidden: u32,
    pub offset: i64,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: u8,
    pub use_2m: u32,
    pub precmd: Option<String>,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: Option<String>,
    pub tot_sectors: u32,
    pub sector_size: u16,
    pub postcmd: Option<String>,
    pub cfg_filename: Option<String>,
}

static MT_LOCK_TIMEOUT: u32 = 10;

fn alrm(_: i32) {}

pub fn lock_dev(fd: RawFd, mode: i32, dev: Option<&mut Device>) -> Result<(), i32> {
    if let Some(dev) = dev {
        if dev.misc_flags & 0x4 != 0 {
            return Ok(());
        }
    }

    let mut retries = 0;
    loop {
        let old_handler = signal::signal(Signal::SIGALRM, SigHandler::Handler(alrm))
            .map_err(|_| -1)?;

        let old_alarm = unsafe { libc::alarm(MT_LOCK_TIMEOUT) };

        let lock_arg = if mode != 0 {
            FlockArg::LockExclusiveNonblock
        } else {
            FlockArg::LockSharedNonblock
        };

        let result = flock(fd, lock_arg);

        signal::signal(Signal::SIGALRM, old_handler).map_err(|_| -1)?;
        unsafe { libc::alarm(old_alarm) };

        match result {
            Ok(_) => return Ok(()),
            Err(e) => {
                if e == Errno::EINTR {
                    return Err(1);
                }
                if e != Errno::EWOULDBLOCK && e != Errno::EAGAIN && e != Errno::EINTR {
                    return Err(-1);
                }
            }
        }

        if retries < MT_LOCK_TIMEOUT * 10 {
            thread::sleep(Duration::from_millis(100));
            retries += 1;
        } else {
            return Err(1);
        }
    }
}