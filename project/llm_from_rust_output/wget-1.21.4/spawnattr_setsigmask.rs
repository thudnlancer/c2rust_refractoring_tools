use std::mem;

pub type pid_t = i32;

#[derive(Copy, Clone)]
pub struct sched_param {
    pub sched_priority: i32,
}

#[derive(Copy, Clone)]
pub struct sigset_t {
    pub __val: [u64; 16],
}

#[derive(Copy, Clone)]
pub struct rpl_posix_spawnattr_t {
    pub _flags: i16,
    pub _pgrp: pid_t,
    pub _sd: sigset_t,
    pub _ss: sigset_t,
    pub _sp: sched_param,
    pub _policy: i32,
    pub __pad: [i32; 16],
}

pub fn rpl_posix_spawnattr_setsigmask(
    attr: &mut rpl_posix_spawnattr_t,
    sigmask: &sigset_t,
) -> i32 {
    attr._ss = *sigmask;
    0
}