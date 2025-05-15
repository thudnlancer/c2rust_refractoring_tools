use std::os::raw::{c_ulong, c_uint, c_long, c_char};

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

fn ftypelet(bits: i64) -> char {
    match bits & 0o170000 {
        0o60000 => 'b',
        0o20000 => 'c',
        0o40000 => 'd',
        0o100000 => '-',
        0o10000 => 'p',
        0o120000 => 'l',
        0o140000 => 's',
        _ => '?',
    }
}

fn rwx(bits: u16, chars: &mut [char; 3]) {
    chars[0] = if bits as i32 & 0o400 != 0 { 'r' } else { '-' };
    chars[1] = if bits as i32 & 0o200 != 0 { 'w' } else { '-' };
    chars[2] = if bits as i32 & 0o100 != 0 { 'x' } else { '-' };
}

fn setst(bits: u16, chars: &mut [char; 10]) {
    if bits as i32 & 0o4000 != 0 {
        chars[3] = if chars[3] == 'x' { 's' } else { 'S' };
    }
    if bits as i32 & 0o2000 != 0 {
        chars[6] = if chars[6] == 'x' { 's' } else { 'S' };
    }
    if bits as i32 & 0o1000 != 0 {
        chars[9] = if chars[9] == 'x' { 't' } else { 'T' };
    }
}

pub fn mode_string(mode: u32, str: &mut [char; 11]) {
    str[0] = ftypelet(mode as i64);
    
    let mut user_perms = ['-'; 3];
    rwx(((mode & 0o700) << 0) as u16, &mut user_perms);
    str[1..4].copy_from_slice(&user_perms);
    
    let mut group_perms = ['-'; 3];
    rwx(((mode & 0o70) << 3) as u16, &mut group_perms);
    str[4..7].copy_from_slice(&group_perms);
    
    let mut other_perms = ['-'; 3];
    rwx(((mode & 0o7) << 6) as u16, &mut other_perms);
    str[7..10].copy_from_slice(&other_perms);
    
    setst(mode as u16, str);
}

pub fn filemodestring(statp: &Stat, str: &mut [char; 11]) {
    mode_string(statp.st_mode, str);
}