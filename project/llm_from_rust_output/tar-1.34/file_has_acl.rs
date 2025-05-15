use std::os::raw::{c_int, c_uint, c_ulong, c_long, c_char};
use std::path::Path;

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

pub fn file_has_acl(name: &Path, sb: &Stat) -> c_int {
    0
}