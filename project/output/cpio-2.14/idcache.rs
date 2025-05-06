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
    fn xstrdup(str: *const i8) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getgrnam(__name: *const i8) -> *mut group;
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
}
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct userid {
    pub id: C2RustUnnamed,
    pub name: *mut i8,
    pub next: *mut userid,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: uid_t,
    pub g: gid_t,
}
static mut user_alist: *mut userid = 0 as *const userid as *mut userid;
static mut nouser_alist: *mut userid = 0 as *const userid as *mut userid;
#[no_mangle]
pub unsafe extern "C" fn getuser(mut uid: uid_t) -> *mut i8 {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut pwent: *mut passwd = 0 as *mut passwd;
    tail = user_alist;
    while !tail.is_null() {
        if (*tail).id.u == uid {
            return (*tail).name;
        }
        tail = (*tail).next;
    }
    pwent = getpwuid(uid);
    tail = xmalloc(::core::mem::size_of::<userid>() as u64) as *mut userid;
    (*tail).id.u = uid;
    if pwent.is_null() {
        let mut nbuf: [i8; 21] = [0; 21];
        (*tail).name = xstrdup(umaxtostr(uid as uintmax_t, nbuf.as_mut_ptr()));
    } else {
        (*tail).name = xstrdup((*pwent).pw_name);
    }
    (*tail).next = user_alist;
    user_alist = tail;
    return (*tail).name;
}
#[no_mangle]
pub unsafe extern "C" fn getuidbyname(mut user: *mut i8) -> *mut uid_t {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut pwent: *mut passwd = 0 as *mut passwd;
    tail = user_alist;
    while !tail.is_null() {
        if *(*tail).name as i32 == *user as i32 && strcmp((*tail).name, user) == 0 {
            return &mut (*tail).id.u;
        }
        tail = (*tail).next;
    }
    tail = nouser_alist;
    while !tail.is_null() {
        if *(*tail).name as i32 == *user as i32 && strcmp((*tail).name, user) == 0 {
            return 0 as *mut uid_t;
        }
        tail = (*tail).next;
    }
    pwent = getpwnam(user);
    tail = xmalloc(::core::mem::size_of::<userid>() as u64) as *mut userid;
    (*tail).name = xstrdup(user);
    if !pwent.is_null() {
        (*tail).id.u = (*pwent).pw_uid;
        (*tail).next = user_alist;
        user_alist = tail;
        return &mut (*tail).id.u;
    }
    (*tail).next = nouser_alist;
    nouser_alist = tail;
    return 0 as *mut uid_t;
}
static mut group_alist: *mut userid = 0 as *const userid as *mut userid;
static mut nogroup_alist: *mut userid = 0 as *const userid as *mut userid;
#[no_mangle]
pub unsafe extern "C" fn getgroup(mut gid: gid_t) -> *mut i8 {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut grent: *mut group = 0 as *mut group;
    tail = group_alist;
    while !tail.is_null() {
        if (*tail).id.g == gid {
            return (*tail).name;
        }
        tail = (*tail).next;
    }
    grent = getgrgid(gid);
    tail = xmalloc(::core::mem::size_of::<userid>() as u64) as *mut userid;
    (*tail).id.g = gid;
    if grent.is_null() {
        let mut nbuf: [i8; 21] = [0; 21];
        (*tail).name = xstrdup(umaxtostr(gid as uintmax_t, nbuf.as_mut_ptr()));
    } else {
        (*tail).name = xstrdup((*grent).gr_name);
    }
    (*tail).next = group_alist;
    group_alist = tail;
    return (*tail).name;
}
#[no_mangle]
pub unsafe extern "C" fn getgidbyname(mut group: *mut i8) -> *mut gid_t {
    let mut tail: *mut userid = 0 as *mut userid;
    let mut grent: *mut group = 0 as *mut group;
    tail = group_alist;
    while !tail.is_null() {
        if *(*tail).name as i32 == *group as i32 && strcmp((*tail).name, group) == 0 {
            return &mut (*tail).id.g;
        }
        tail = (*tail).next;
    }
    tail = nogroup_alist;
    while !tail.is_null() {
        if *(*tail).name as i32 == *group as i32 && strcmp((*tail).name, group) == 0 {
            return 0 as *mut gid_t;
        }
        tail = (*tail).next;
    }
    grent = getgrnam(group);
    tail = xmalloc(::core::mem::size_of::<userid>() as u64) as *mut userid;
    (*tail).name = xstrdup(group);
    if !grent.is_null() {
        (*tail).id.g = (*grent).gr_gid;
        (*tail).next = group_alist;
        group_alist = tail;
        return &mut (*tail).id.g;
    }
    (*tail).next = nogroup_alist;
    nogroup_alist = tail;
    return 0 as *mut gid_t;
}