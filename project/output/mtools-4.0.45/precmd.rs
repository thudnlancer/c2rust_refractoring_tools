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
    fn exit(_: i32) -> !;
    fn perror(__s: *const i8);
    fn execl(__path: *const i8, __arg: *const i8, _: ...) -> i32;
    fn fork() -> __pid_t;
    fn wait(__stat_loc: *mut i32) -> __pid_t;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __pid_t = i32;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[no_mangle]
pub unsafe extern "C" fn precmd(mut dev: *mut device) {
    if dev.is_null() {
        return;
    }
    postcmd((*dev).precmd);
}
#[no_mangle]
pub unsafe extern "C" fn postcmd(mut cmd: *const i8) {
    let mut status: i32 = 0;
    let mut pid: pid_t = 0;
    if cmd.is_null() {
        return;
    }
    pid = fork();
    match pid {
        -1 => {
            perror(b"Could not fork\0" as *const u8 as *const i8);
            exit(1 as i32);
        }
        0 => {
            execl(
                b"/bin/sh\0" as *const u8 as *const i8,
                b"sh\0" as *const u8 as *const i8,
                b"-c\0" as *const u8 as *const i8,
                cmd,
                0 as *mut libc::c_void as *mut i8,
            );
        }
        _ => {
            wait(&mut status);
        }
    };
}