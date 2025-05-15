use std::time::Duration;

pub type DevT = u64;
pub type UidT = u32;
pub type GidT = u32;
pub type InoT = u64;
pub type ModeT = u32;
pub type NlinkT = u64;
pub type OffT = i64;
pub type TimeT = i64;
pub type BlksizeT = i64;
pub type BlkcntT = i64;
pub type SyscallSlongT = i64;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: SyscallSlongT,
}

impl Timespec {
    pub fn new() -> Self {
        Timespec {
            tv_sec: -1,
            tv_nsec: -1,
        }
    }

    pub fn from_duration(d: Duration) -> Self {
        Timespec {
            tv_sec: d.as_secs() as TimeT,
            tv_nsec: d.subsec_nanos() as SyscallSlongT,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Stat {
    pub st_dev: DevT,
    pub st_ino: InoT,
    pub st_nlink: NlinkT,
    pub st_mode: ModeT,
    pub st_uid: UidT,
    pub st_gid: GidT,
    pub __pad0: i32,
    pub st_rdev: DevT,
    pub st_size: OffT,
    pub st_blksize: BlksizeT,
    pub st_blocks: BlkcntT,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [SyscallSlongT; 3],
}

impl Stat {
    pub fn stat_time_normalize(&mut self, result: i32) -> i32 {
        result
    }

    pub fn get_birthtime(&self) -> Timespec {
        Timespec::new()
    }

    pub fn get_mtime(&self) -> Timespec {
        self.st_mtim
    }

    pub fn get_ctime(&self) -> Timespec {
        self.st_ctim
    }

    pub fn get_atime_ns(&self) -> i64 {
        self.st_atim.tv_nsec
    }

    pub fn get_ctime_ns(&self) -> i64 {
        self.st_ctim.tv_nsec
    }

    pub fn get_mtime_ns(&self) -> i64 {
        self.st_mtim.tv_nsec
    }

    pub fn get_birthtime_ns(&self) -> i64 {
        0
    }

    pub fn get_atime(&self) -> Timespec {
        self.st_atim
    }
}