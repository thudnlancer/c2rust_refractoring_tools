use std::os::raw::{c_ulong, c_uint, c_long};
use std::path::Path;
use std::fs;

type UIntMax = c_ulong;
type DevT = c_ulong;
type InoT = c_ulong;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[derive(Debug, Clone, Copy)]
pub struct Stat {
    pub st_dev: DevT,
    pub st_ino: InoT,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: i32,
    pub st_rdev: DevT,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [c_long; 3],
}

#[derive(Debug, Clone, Copy)]
pub struct DevIno {
    pub st_ino: InoT,
    pub st_dev: DevT,
}

#[derive(Debug, Clone, Copy)]
pub struct CycleCheckState {
    pub dev_ino: DevIno,
    pub chdir_counter: UIntMax,
    pub magic: i32,
}

impl CycleCheckState {
    pub fn new() -> Self {
        Self {
            dev_ino: DevIno {
                st_ino: 0,
                st_dev: 0,
            },
            chdir_counter: 0,
            magic: 9827862,
        }
    }

    pub fn check_cycle(&mut self, sb: &Stat) -> bool {
        assert_eq!(self.magic, 9827862, "Invalid magic number");

        if self.chdir_counter != 0 
            && sb.st_ino == self.dev_ino.st_ino 
            && sb.st_dev == self.dev_ino.st_dev 
        {
            return true;
        }

        self.chdir_counter = self.chdir_counter.wrapping_add(1);
        if Self::is_zero_or_power_of_two(self.chdir_counter) {
            if self.chdir_counter == 0 {
                return true;
            }
            self.dev_ino.st_dev = sb.st_dev;
            self.dev_ino.st_ino = sb.st_ino;
        }

        false
    }

    fn is_zero_or_power_of_two(i: UIntMax) -> bool {
        i & i.wrapping_sub(1) == 0
    }
}