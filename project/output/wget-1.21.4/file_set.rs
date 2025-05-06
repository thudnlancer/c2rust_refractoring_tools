#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn triple_free(x: *mut libc::c_void);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type Hash_table = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct F_triple {
    pub name: *mut i8,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[no_mangle]
pub unsafe extern "C" fn record_file(
    mut ht: *mut Hash_table,
    mut file: *const i8,
    mut stats: *const stat,
) {
    let mut ent: *mut F_triple = 0 as *mut F_triple;
    if ht.is_null() {
        return;
    }
    ent = xmalloc(::core::mem::size_of::<F_triple>() as u64) as *mut F_triple;
    (*ent).name = xstrdup(file);
    (*ent).st_ino = (*stats).st_ino;
    (*ent).st_dev = (*stats).st_dev;
    let mut ent_from_table: *mut F_triple = hash_insert(ht, ent as *const libc::c_void)
        as *mut F_triple;
    if ent_from_table.is_null() {
        xalloc_die();
    }
    if ent_from_table != ent {
        triple_free(ent as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn seen_file(
    mut ht: *const Hash_table,
    mut file: *const i8,
    mut stats: *const stat,
) -> bool {
    let mut new_ent: F_triple = F_triple {
        name: 0 as *mut i8,
        st_ino: 0,
        st_dev: 0,
    };
    if ht.is_null() {
        return 0 as i32 != 0;
    }
    new_ent.name = file as *mut i8;
    new_ent.st_ino = (*stats).st_ino;
    new_ent.st_dev = (*stats).st_dev;
    return !(hash_lookup(ht, &mut new_ent as *mut F_triple as *const libc::c_void))
        .is_null();
}