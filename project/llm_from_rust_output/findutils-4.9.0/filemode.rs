use std::os::raw::{c_char, c_uint, c_ulong, c_long};

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

fn ftypelet(bits: ModeT) -> char {
    match bits & 0o170000 {
        0o100000 => '-',
        0o40000 => 'd',
        0o60000 => 'b',
        0o20000 => 'c',
        0o120000 => 'l',
        0o10000 => 'p',
        0o140000 => 's',
        _ => '?',
    }
}

pub fn strmode(mode: ModeT) -> [char; 12] {
    let mut result = [' '; 12];
    
    result[0] = ftypelet(mode);
    result[1] = if mode & 0o400 != 0 { 'r' } else { '-' };
    result[2] = if mode & 0o200 != 0 { 'w' } else { '-' };
    result[3] = match (mode & 0o4000, mode & 0o100) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'S',
        (_, _) => 's',
    };
    result[4] = if mode & 0o040 != 0 { 'r' } else { '-' };
    result[5] = if mode & 0o020 != 0 { 'w' } else { '-' };
    result[6] = match (mode & 0o2000, mode & 0o010) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'S',
        (_, _) => 's',
    };
    result[7] = if mode & 0o004 != 0 { 'r' } else { '-' };
    result[8] = if mode & 0o002 != 0 { 'w' } else { '-' };
    result[9] = match (mode & 0o1000, mode & 0o001) {
        (0, 0) => '-',
        (0, _) => 'x',
        (_, 0) => 'T',
        (_, _) => 't',
    };
    result[10] = ' ';
    result[11] = '\0';
    
    result
}

pub fn filemodestring(statp: &Stat) -> [char; 12] {
    let mut result = strmode(statp.st_mode);
    
    // These conditions appear to be always false in original code
    // (mode - mode will always be 0)
    // Keeping them for completeness but they'll never execute
    if statp.st_mode.wrapping_sub(statp.st_mode) != 0 {
        result[0] = 'F';
    } else if statp.st_mode.wrapping_sub(statp.st_mode) != 0 {
        result[0] = 'Q';
    } else if statp.st_mode.wrapping_sub(statp.st_mode) != 0 {
        result[0] = 'S';
    }
    
    result
}