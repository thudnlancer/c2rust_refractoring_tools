#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __pid_t = i32;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: libc::c_short,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: i32,
    pub __pad: [i32; 16],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawnattr_setflags(
    mut attr: *mut rpl_posix_spawnattr_t,
    mut flags: libc::c_short,
) -> i32 {
    if flags as i32
        & !(0x1 as i32 | 0x2 as i32 | 0x4 as i32 | 0x8 as i32 | 0x10 as i32 | 0x20 as i32
            | 0x40 as i32) != 0
    {
        return 22 as i32;
    }
    (*attr)._flags = flags;
    return 0 as i32;
}