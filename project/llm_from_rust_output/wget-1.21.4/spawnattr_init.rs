use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub struct SchedParam {
    pub sched_priority: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Sigset {
    pub val: [u64; 16],
}

#[derive(Debug, Clone, Copy)]
pub struct PosixSpawnattr {
    pub flags: i16,
    pub pgrp: i32,
    pub sd: Sigset,
    pub ss: Sigset,
    pub sp: SchedParam,
    pub policy: i32,
    pub pad: [i32; 16],
}

impl Default for PosixSpawnattr {
    fn default() -> Self {
        PosixSpawnattr {
            flags: 0,
            pgrp: 0,
            sd: Sigset { val: [0; 16] },
            ss: Sigset { val: [0; 16] },
            sp: SchedParam { sched_priority: 0 },
            policy: 0,
            pad: [0; 16],
        }
    }
}

pub fn posix_spawnattr_init(attr: &mut PosixSpawnattr) -> i32 {
    *attr = PosixSpawnattr::default();
    0
}