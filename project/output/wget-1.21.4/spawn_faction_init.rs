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
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
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
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: i32,
    pub _used: i32,
    pub _actions: *mut __spawn_action,
    pub __pad: [i32; 16],
}
#[no_mangle]
pub unsafe extern "C" fn gl_posix_spawn_file_actions_realloc(
    mut file_actions: *mut rpl_posix_spawn_file_actions_t,
) -> i32 {
    let mut newalloc: i32 = (*file_actions)._allocated + 8 as i32;
    let mut newmem: *mut libc::c_void = realloc(
        (*file_actions)._actions as *mut libc::c_void,
        (newalloc as u64).wrapping_mul(::core::mem::size_of::<__spawn_action>() as u64),
    );
    if newmem.is_null() {
        return 12 as i32;
    }
    (*file_actions)._actions = newmem as *mut __spawn_action;
    (*file_actions)._allocated = newalloc;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawn_file_actions_init(
    mut file_actions: *mut rpl_posix_spawn_file_actions_t,
) -> i32 {
    memset(
        file_actions as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<rpl_posix_spawn_file_actions_t>() as u64,
    );
    return 0 as i32;
}