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
    fn unsetenv(__name: *const i8) -> i32;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> i32;
    fn seteuid(__uid: __uid_t) -> i32;
    fn setgid(__gid: __gid_t) -> i32;
    fn fcntl(__fd: i32, __cmd: i32, _: ...) -> i32;
}
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
#[no_mangle]
pub static mut noPrivileges: i32 = 0 as i32;
#[inline]
unsafe extern "C" fn print_privs(mut message: *const i8) {}
static mut rgid: gid_t = 0;
static mut egid: gid_t = 0;
static mut ruid: uid_t = 0;
static mut euid: uid_t = 0;
#[inline]
unsafe extern "C" fn Setuid(mut uid: uid_t) {
    if euid == 0 as i32 as u32 {
        seteuid(uid);
    } else {
        setuid(uid);
    };
}
#[no_mangle]
pub unsafe extern "C" fn reclaim_privs() {
    if noPrivileges != 0 {
        return;
    }
    setgid(egid);
    Setuid(euid);
    print_privs(
        b"after reclaim privs, both uids should be 0 \0" as *const u8 as *const i8,
    );
}
#[no_mangle]
pub unsafe extern "C" fn drop_privs() {
    Setuid(ruid);
    setgid(rgid);
    print_privs(
        b"after drop_privs, real should be 0, effective should not \0" as *const u8
            as *const i8,
    );
}
#[no_mangle]
pub unsafe extern "C" fn destroy_privs() {
    if euid == 0 as i32 as u32 {
        setuid(0 as i32 as __uid_t);
        setuid(ruid);
        seteuid(ruid);
    }
    drop_privs();
    print_privs(b"destroy_privs, no uid should be zero  \0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn get_real_uid() -> uid_t {
    return ruid;
}
#[no_mangle]
pub unsafe extern "C" fn init_privs() {
    euid = geteuid();
    ruid = getuid();
    egid = getegid();
    rgid = getgid();
    if euid != ruid {
        unsetenv(b"SOURCE_DATE_EPOCH\0" as *const u8 as *const i8);
    }
    if euid == 0 as i32 as u32 && ruid != 0 as i32 as u32 {
        setuid(0 as i32 as __uid_t);
    }
    drop_privs();
    print_privs(
        b"after init, real should be 0, effective should not \0" as *const u8
            as *const i8,
    );
}
#[no_mangle]
pub unsafe extern "C" fn closeExec(mut fd: i32) {
    fcntl(fd, 2 as i32, 1 as i32);
}