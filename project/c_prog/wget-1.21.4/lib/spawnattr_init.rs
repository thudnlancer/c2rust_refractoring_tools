use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
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
    pub _policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawnattr_init(
    mut attr: *mut rpl_posix_spawnattr_t,
) -> libc::c_int {
    memset(
        attr as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<rpl_posix_spawnattr_t>() as libc::c_ulong,
    );
    return 0 as libc::c_int;
}
