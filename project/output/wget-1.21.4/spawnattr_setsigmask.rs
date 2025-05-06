#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
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
pub unsafe extern "C" fn rpl_posix_spawnattr_setsigmask(
    mut attr: *mut rpl_posix_spawnattr_t,
    mut sigmask: *const sigset_t,
) -> i32 {
    memcpy(
        &mut (*attr)._ss as *mut sigset_t as *mut libc::c_void,
        sigmask as *const libc::c_void,
        ::core::mem::size_of::<sigset_t>() as u64,
    );
    return 0 as i32;
}