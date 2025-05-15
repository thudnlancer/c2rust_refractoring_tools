use libc::{c_int, c_short, c_ulong};

pub type __pid_t = c_int;
pub type pid_t = __pid_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16],
}

pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: c_short,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: c_int,
    pub __pad: [c_int; 16],
}

const VALID_FLAGS: c_int = 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x40;

#[no_mangle]
pub extern "C" fn rpl_posix_spawnattr_setflags(
    attr: &mut rpl_posix_spawnattr_t,
    flags: c_short,
) -> c_int {
    if (flags as c_int) & !VALID_FLAGS != 0 {
        return 22;
    }
    attr._flags = flags;
    0
}