use libc::{c_int, c_short, c_ulong};

pub type Pid = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SchedParam {
    pub sched_priority: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sigset {
    pub val: [c_ulong; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PosixSpawnAttr {
    flags: c_short,
    pgrp: Pid,
    sd: Sigset,
    ss: Sigset,
    sp: SchedParam,
    policy: c_int,
    pad: [c_int; 16],
}

pub fn posix_spawnattr_destroy(attr: &mut PosixSpawnAttr) -> c_int {
    0
}