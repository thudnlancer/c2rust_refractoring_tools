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
    fn gl_posix_spawn_internal(
        pid: *mut pid_t,
        path: *const i8,
        file_actions: *const rpl_posix_spawn_file_actions_t,
        attrp: *const rpl_posix_spawnattr_t,
        argv: *const *const i8,
        envp: *const *const i8,
        use_path: i32,
    ) -> i32;
}
pub type __mode_t = u32;
pub type __pid_t = i32;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: i32,
}
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
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
    pub fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub path: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fd: i32,
    pub path: *mut i8,
    pub oflag: i32,
    pub mode: mode_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub fd: i32,
    pub newfd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub fd: i32,
}
pub type C2RustUnnamed_5 = u32;
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
    pub _policy: i32,
    pub __pad: [i32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: i32,
    pub _used: i32,
    pub _actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawn(
    mut pid: *mut pid_t,
    mut path: *const i8,
    mut file_actions: *const rpl_posix_spawn_file_actions_t,
    mut attrp: *const rpl_posix_spawnattr_t,
    mut argv: *const *mut i8,
    mut envp: *const *mut i8,
) -> i32 {
    return gl_posix_spawn_internal(
        pid,
        path,
        file_actions,
        attrp,
        argv as *const *const i8,
        envp as *const *const i8,
        0 as i32,
    );
}