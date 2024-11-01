#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn gl_posix_spawn_internal(
        pid: *mut pid_t,
        path: *const libc::c_char,
        file_actions: *const rpl_posix_spawn_file_actions_t,
        attrp: *const rpl_posix_spawnattr_t,
        argv: *const *const libc::c_char,
        envp: *const *const libc::c_char,
        use_path: libc::c_int,
    ) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __spawn_action {
    pub tag: C2RustUnnamed_5,
    pub action: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub close_action: C2RustUnnamed_4,
    pub dup2_action: C2RustUnnamed_3,
    pub open_action: C2RustUnnamed_2,
    pub chdir_action: C2RustUnnamed_1,
    pub fchdir_action: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fd: libc::c_int,
    pub path: *mut libc::c_char,
    pub oflag: libc::c_int,
    pub mode: mode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fd: libc::c_int,
    pub newfd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub fd: libc::c_int,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const spawn_do_fchdir: C2RustUnnamed_5 = 4;
pub const spawn_do_chdir: C2RustUnnamed_5 = 3;
pub const spawn_do_open: C2RustUnnamed_5 = 2;
pub const spawn_do_dup2: C2RustUnnamed_5 = 1;
pub const spawn_do_close: C2RustUnnamed_5 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: libc::c_int,
    pub _used: libc::c_int,
    pub _actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawn(
    mut pid: *mut pid_t,
    mut path: *const libc::c_char,
    mut file_actions: *const rpl_posix_spawn_file_actions_t,
    mut attrp: *const rpl_posix_spawnattr_t,
    mut argv: *const *mut libc::c_char,
    mut envp: *const *mut libc::c_char,
) -> libc::c_int {
    return gl_posix_spawn_internal(
        pid,
        path,
        file_actions,
        attrp,
        argv as *const *const libc::c_char,
        envp as *const *const libc::c_char,
        0 as libc::c_int,
    );
}
