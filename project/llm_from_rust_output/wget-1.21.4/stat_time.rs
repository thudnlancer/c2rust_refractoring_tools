use std::os::raw::{c_int, c_uint, c_ulong, c_long};

pub type DevT = c_ulong;
pub type UidT = c_uint;
pub type GidT = c_uint;
pub type InoT = c_ulong;
pub type ModeT = c_uint;
pub type NlinkT = c_ulong;
pub type OffT = c_long;
pub type TimeT = c_long;
pub type BlksizeT = c_long;
pub type BlkcntT = c_long;
pub type SyscallSlongT = c_long;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: SyscallSlongT,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stat {
    pub st_dev: DevT,
    pub st_ino: InoT,
    pub st_nlink: NlinkT,
    pub st_mode: ModeT,
    pub st_uid: UidT,
    pub st_gid: GidT,
    pub __pad0: c_int,
    pub st_rdev: DevT,
    pub st_size: OffT,
    pub st_blksize: BlksizeT,
    pub st_blocks: BlkcntT,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [SyscallSlongT; 3],
}

pub fn stat_time_normalize(result: c_int, st: &mut Stat) -> c_int {
    result
}

pub fn get_stat_birthtime(st: &Stat) -> Timespec {
    Timespec {
        tv_sec: -1,
        tv_nsec: -1,
    }
}

pub fn get_stat_mtime(st: &Stat) -> Timespec {
    st.st_mtim
}

pub fn get_stat_ctime(st: &Stat) -> Timespec {
    st.st_ctim
}

pub fn get_stat_atime(st: &Stat) -> Timespec {
    st.st_atim
}

pub fn get_stat_birthtime_ns(_st: &Stat) -> c_long {
    0
}

pub fn get_stat_mtime_ns(st: &Stat) -> c_long {
    st.st_mtim.tv_nsec
}

pub fn get_stat_ctime_ns(st: &Stat) -> c_long {
    st.st_ctim.tv_nsec
}

pub fn get_stat_atime_ns(st: &Stat) -> c_long {
    st.st_atim.tv_nsec
}