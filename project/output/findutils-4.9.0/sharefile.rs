#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    pub type Hash_tuning_0;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn strdup(_: *const i8) -> *mut i8;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn set_cloexec_flag(desc: i32, value: bool) -> i32;
    fn fileno(__stream: *mut FILE) -> i32;
    fn rpl_fclose(stream: *mut FILE) -> i32;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn fopen_safer(_: *const i8, _: *const i8) -> *mut FILE;
    fn fatal_nontarget_file_error(errno_value: i32, name: *const i8) -> !;
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type sharefile_handle = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharefile {
    pub mode: *mut i8,
    pub table: *mut Hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SharefileEntry {
    pub device: dev_t,
    pub inode: ino_t,
    pub name: *mut i8,
    pub fp: *mut FILE,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    DefaultHashTableSize = 11,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::DefaultHashTableSize => 11,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            11 => C2RustUnnamed::DefaultHashTableSize,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
unsafe extern "C" fn entry_comparator(
    mut av: *const libc::c_void,
    mut bv: *const libc::c_void,
) -> bool {
    let mut a: *const SharefileEntry = av as *const SharefileEntry;
    let mut b: *const SharefileEntry = bv as *const SharefileEntry;
    return (*a).inode == (*b).inode && (*a).device == (*b).device;
}
unsafe extern "C" fn entry_free(mut pv: *mut libc::c_void) {
    let mut p: *mut SharefileEntry = pv as *mut SharefileEntry;
    if !((*p).fp).is_null() {
        if 0 as i32 != rpl_fclose((*p).fp) {
            fatal_nontarget_file_error(*__errno_location(), (*p).name);
        }
    }
    rpl_free((*p).name as *mut libc::c_void);
    rpl_free(p as *mut libc::c_void);
}
unsafe extern "C" fn entry_hashfunc(
    mut pv: *const libc::c_void,
    mut buckets: size_t,
) -> size_t {
    let mut p: *const SharefileEntry = pv as *const SharefileEntry;
    return ((*p).device ^ (*p).inode).wrapping_rem(buckets);
}
#[no_mangle]
pub unsafe extern "C" fn sharefile_init(mut mode: *const i8) -> sharefile_handle {
    let mut p: *mut sharefile = malloc(::core::mem::size_of::<sharefile>() as u64)
        as *mut sharefile;
    if !p.is_null() {
        (*p).mode = strdup(mode);
        if !((*p).mode).is_null() {
            (*p).table = hash_initialize(
                C2RustUnnamed::DefaultHashTableSize as i32 as size_t,
                0 as *const Hash_tuning,
                Some(
                    entry_hashfunc
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    entry_comparator
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                Some(entry_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            if !((*p).table).is_null() {
                return p as sharefile_handle
            } else {
                rpl_free((*p).mode as *mut libc::c_void);
                rpl_free(p as *mut libc::c_void);
            }
        } else {
            rpl_free(p as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn sharefile_destroy(mut pv: sharefile_handle) {
    let mut p: *mut sharefile = pv as *mut sharefile;
    rpl_free((*p).mode as *mut libc::c_void);
    hash_free((*p).table);
}
#[no_mangle]
pub unsafe extern "C" fn sharefile_fopen(
    mut h: sharefile_handle,
    mut filename: *const i8,
) -> *mut FILE {
    let mut p: *mut sharefile = h as *mut sharefile;
    let mut new_entry: *mut SharefileEntry = 0 as *mut SharefileEntry;
    new_entry = malloc(::core::mem::size_of::<SharefileEntry>() as u64)
        as *mut SharefileEntry;
    if new_entry.is_null() {
        return 0 as *mut FILE;
    }
    (*new_entry).name = strdup(filename);
    if ((*new_entry).name).is_null() {
        rpl_free(new_entry as *mut libc::c_void);
        return 0 as *mut FILE;
    }
    (*new_entry).fp = fopen_safer(filename, (*p).mode);
    if ((*new_entry).fp).is_null() {
        entry_free(new_entry as *mut libc::c_void);
        return 0 as *mut FILE;
    } else {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let fd: i32 = fileno((*new_entry).fp);
        if fd >= 0 as i32 {} else {
            __assert_fail(
                b"fd >= 0\0" as *const u8 as *const i8,
                b"sharefile.c\0" as *const u8 as *const i8,
                166 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 54],
                    &[i8; 54],
                >(b"FILE *sharefile_fopen(sharefile_handle, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_3690: {
            if fd >= 0 as i32 {} else {
                __assert_fail(
                    b"fd >= 0\0" as *const u8 as *const i8,
                    b"sharefile.c\0" as *const u8 as *const i8,
                    166 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 54],
                        &[i8; 54],
                    >(b"FILE *sharefile_fopen(sharefile_handle, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
        set_cloexec_flag(fd, 1 as i32 != 0);
        if fstat(fd, &mut st) < 0 as i32 {
            entry_free(new_entry as *mut libc::c_void);
            return 0 as *mut FILE;
        } else {
            let mut existing: *mut libc::c_void = 0 as *mut libc::c_void;
            (*new_entry).device = st.st_dev;
            (*new_entry).inode = st.st_ino;
            existing = hash_lookup((*p).table, new_entry as *const libc::c_void);
            if !existing.is_null() {
                entry_free(new_entry as *mut libc::c_void);
                return (*(existing as *const SharefileEntry)).fp;
            } else if !(hash_insert((*p).table, new_entry as *const libc::c_void))
                .is_null()
            {
                return (*new_entry).fp
            } else {
                let save_errno: i32 = *__errno_location();
                entry_free(new_entry as *mut libc::c_void);
                *__errno_location() = save_errno;
                return 0 as *mut FILE;
            }
        }
    };
}