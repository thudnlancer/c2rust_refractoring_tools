use ::libc;
extern "C" {
    fn getdtablesize() -> libc::c_int;
    fn gl_posix_spawn_file_actions_realloc(
        file_actions: *mut rpl_posix_spawn_file_actions_t,
    ) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
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
pub struct rpl_posix_spawn_file_actions_t {
    pub _allocated: libc::c_int,
    pub _used: libc::c_int,
    pub _actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
#[no_mangle]
pub unsafe extern "C" fn rpl_posix_spawn_file_actions_addclose(
    mut file_actions: *mut rpl_posix_spawn_file_actions_t,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut maxfd: libc::c_int = getdtablesize();
    if fd < 0 as libc::c_int || fd >= maxfd {
        return 9 as libc::c_int;
    }
    if (*file_actions)._used == (*file_actions)._allocated
        && gl_posix_spawn_file_actions_realloc(file_actions) != 0 as libc::c_int
    {
        return 12 as libc::c_int;
    }
    let mut rec: *mut __spawn_action = 0 as *mut __spawn_action;
    rec = &mut *((*file_actions)._actions).offset((*file_actions)._used as isize)
        as *mut __spawn_action;
    (*rec).tag = spawn_do_close;
    (*rec).action.open_action.fd = fd;
    (*file_actions)._used += 1;
    (*file_actions)._used;
    return 0 as libc::c_int;
}
