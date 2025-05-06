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
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut i32;
    fn __pth_scheduler_drop();
}
pub type __pid_t = i32;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pth_atfork_st {
    pub prepare: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub parent: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub child: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
static mut pth_atfork_list: [pth_atfork_st; 128] = [pth_atfork_st {
    prepare: None,
    parent: None,
    child: None,
    arg: 0 as *const libc::c_void as *mut libc::c_void,
}; 128];
static mut pth_atfork_idx: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn pth_atfork_push(
    mut prepare: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut parent: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut child: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg: *mut libc::c_void,
) -> i32 {
    if pth_atfork_idx > 128 as i32 - 1 as i32 {
        *__errno_location() = 12 as i32;
        return 0 as i32;
    }
    pth_atfork_list[pth_atfork_idx as usize].prepare = prepare;
    pth_atfork_list[pth_atfork_idx as usize].parent = parent;
    pth_atfork_list[pth_atfork_idx as usize].child = child;
    pth_atfork_list[pth_atfork_idx as usize].arg = arg;
    pth_atfork_idx += 1;
    pth_atfork_idx;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_atfork_pop() -> i32 {
    if pth_atfork_idx <= 0 as i32 {
        return 0 as i32;
    }
    pth_atfork_idx -= 1;
    pth_atfork_idx;
    return (0 as i32 == 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn pth_fork() -> pid_t {
    let mut pid: pid_t = 0;
    let mut i: i32 = 0;
    i = pth_atfork_idx - 1 as i32;
    while i >= 0 as i32 {
        if (pth_atfork_list[i as usize].prepare).is_some() {
            (pth_atfork_list[i as usize].prepare)
                .expect("non-null function pointer")(pth_atfork_list[i as usize].arg);
        }
        i -= 1;
        i;
    }
    pid = fork();
    if pid == -(1 as i32) {
        return 0 as i32;
    }
    if pid != 0 as i32 {
        i = 0 as i32;
        while i <= pth_atfork_idx - 1 as i32 {
            if (pth_atfork_list[i as usize].parent).is_some() {
                (pth_atfork_list[i as usize].parent)
                    .expect(
                        "non-null function pointer",
                    )(pth_atfork_list[i as usize].arg);
            }
            i += 1;
            i;
        }
    } else {
        __pth_scheduler_drop();
        i = 0 as i32;
        while i <= pth_atfork_idx - 1 as i32 {
            if (pth_atfork_list[i as usize].child).is_some() {
                (pth_atfork_list[i as usize].child)
                    .expect(
                        "non-null function pointer",
                    )(pth_atfork_list[i as usize].arg);
            }
            i += 1;
            i;
        }
    }
    return pid;
}