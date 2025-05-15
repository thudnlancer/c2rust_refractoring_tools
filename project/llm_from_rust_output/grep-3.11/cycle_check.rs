use std::os::raw::{c_ulong, c_uint, c_long};
use std::path::Path;
use std::fs;

type UIntMax = c_ulong;
type DevT = c_ulong;
type InoT = c_ulong;

#[derive(Debug, Copy, Clone)]
pub struct Timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub struct DevIno {
    pub st_ino: InoT,
    pub st_dev: DevT,
}

#[derive(Debug, Copy, Clone)]
pub struct CycleCheckState {
    pub dev_ino: DevIno,
    pub chdir_counter: UIntMax,
    pub magic: i32,
}

fn is_zero_or_power_of_two(i: UIntMax) -> bool {
    i & i.wrapping_sub(1) == 0
}

impl CycleCheckState {
    pub fn new() -> Self {
        CycleCheckState {
            dev_ino: DevIno {
                st_ino: 0,
                st_dev: 0,
            },
            chdir_counter: 0,
            magic: 9827862,
        }
    }

    pub fn check(&mut self, sb: &Stat) -> bool {
        assert_eq!(self.magic, 9827862, "Invalid magic number");

        if self.chdir_counter != 0 && (sb.st_ino == self.dev_ino.st_ino && sb.st_dev == self.dev_ino.st_dev) {
            return true;
        }

        self.chdir_counter = self.chdir_counter.wrapping_add(1);
        if is_zero_or_power_of_two(self.chdir_counter) {
            if self.chdir_counter == 0 {
                return true;
            }
            self.dev_ino.st_dev = sb.st_dev;
            self.dev_ino.st_ino = sb.st_ino;
        }

        false
    }
}

// Helper function to get file metadata as Stat
pub fn get_stat(path: &Path) -> Result<Stat, std::io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(Stat {
        st_dev: metadata.dev() as DevT,
        st_ino: metadata.ino() as InoT,
        st_nlink: metadata.nlink() as c_ulong,
        st_mode: metadata.mode() as c_uint,
        st_uid: metadata.uid() as c_uint,
        st_gid: metadata.gid() as c_uint,
        __pad0: 0,
        st_rdev: metadata.rdev() as DevT,
        st_size: metadata.len() as c_long,
        st_blksize: metadata.blksize() as c_long,
        st_blocks: metadata.blocks() as c_long,
        st_atim: Timespec {
            tv_sec: metadata.accessed()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as c_long,
            tv_nsec: 0,
        },
        st_mtim: Timespec {
            tv_sec: metadata.modified()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as c_long,
            tv_nsec: 0,
        },
        st_ctim: Timespec {
            tv_sec: metadata.created()?.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as c_long,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    })
}