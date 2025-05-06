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
    fn rpl_free(ptr: *mut libc::c_void);
    fn strdup(_: *const i8) -> *mut i8;
    fn getdtablesize() -> i32;
    fn gl_posix_spawn_file_actions_realloc(
        file_actions: *mut rpl_posix_spawn_file_actions_t,
    ) -> i32;
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
pub unsafe extern "C" fn rpl_posix_spawn_file_actions_addopen(
    mut file_actions: *mut rpl_posix_spawn_file_actions_t,
    mut fd: i32,
    mut path: *const i8,
    mut oflag: i32,
    mut mode: mode_t,
) -> i32 {
    let mut maxfd: i32 = getdtablesize();
    if fd < 0 as i32 || fd >= maxfd {
        return 9 as i32;
    }
    let mut path_copy: *mut i8 = strdup(path);
    if path_copy.is_null() {
        return 12 as i32;
    }
    if (*file_actions)._used == (*file_actions)._allocated
        && gl_posix_spawn_file_actions_realloc(file_actions) != 0 as i32
    {
        rpl_free(path_copy as *mut libc::c_void);
        return 12 as i32;
    }
    let mut rec: *mut __spawn_action = 0 as *mut __spawn_action;
    rec = &mut *((*file_actions)._actions).offset((*file_actions)._used as isize)
        as *mut __spawn_action;
    (*rec).tag = spawn_do_open;
    (*rec).action.open_action.fd = fd;
    (*rec).action.open_action.path = path_copy;
    (*rec).action.open_action.oflag = oflag;
    (*rec).action.open_action.mode = mode;
    (*file_actions)._used += 1;
    (*file_actions)._used;
    return 0 as i32;
}