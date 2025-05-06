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
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
}
pub type __uint32_t = u32;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __off_t = i64;
pub type __time_t = i64;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpio_file_stat {
    pub c_magic: libc::c_ushort,
    pub c_ino: ino_t,
    pub c_mode: mode_t,
    pub c_uid: uid_t,
    pub c_gid: gid_t,
    pub c_nlink: size_t,
    pub c_mtime: time_t,
    pub c_filesize: off_t,
    pub c_dev_maj: u32,
    pub c_dev_min: u32,
    pub c_rdev_maj: u32,
    pub c_rdev_min: u32,
    pub c_namesize: size_t,
    pub c_chksum: uint32_t,
    pub c_name: *mut i8,
    pub c_name_buflen: size_t,
    pub c_tar_linkname: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferment {
    pub next: *mut deferment,
    pub header: cpio_file_stat,
}
#[no_mangle]
pub unsafe extern "C" fn create_deferment(
    mut file_hdr: *mut cpio_file_stat,
) -> *mut deferment {
    let mut d: *mut deferment = 0 as *mut deferment;
    d = xmalloc(::core::mem::size_of::<deferment>() as u64) as *mut deferment;
    (*d).header = *file_hdr;
    (*d).header.c_name = xmalloc(
        (strlen((*file_hdr).c_name)).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    strcpy((*d).header.c_name, (*file_hdr).c_name);
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn free_deferment(mut d: *mut deferment) {
    rpl_free((*d).header.c_name as *mut libc::c_void);
    rpl_free(d as *mut libc::c_void);
}