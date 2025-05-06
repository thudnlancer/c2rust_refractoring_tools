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
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type hash_table;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn rpl_free(_: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn fchmod(__fd: i32, __mode: __mode_t) -> i32;
    fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chown(__file: *const i8, __owner: __uid_t, __group: __gid_t) -> i32;
    fn fchown(__fd: i32, __owner: __uid_t, __group: __gid_t) -> i32;
    fn chdir(__path: *const i8) -> i32;
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn dir_name(file: *const i8) -> *mut i8;
    fn strip_trailing_slashes(file: *mut i8) -> bool;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, ps: *mut size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn dcngettext(
        __domainname: *const i8,
        __msgid1: *const i8,
        __msgid2: *const i8,
        __n: u64,
        __category: i32,
    ) -> *mut i8;
    fn chmod_error_details(name: *const i8, mode: mode_t);
    fn chown_error_details(name: *const i8, uid: uid_t, gid: gid_t);
    fn mkdir_error(_: *const i8);
    fn open_error(_: *const i8);
    fn stat_error(_: *const i8);
    fn utime_error(_: *const i8);
    fn safer_name_suffix(
        file_name: *const i8,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut i8;
    fn ds_free(string: *mut dynamic_string);
    fn ds_fgets(f: *mut FILE, s: *mut dynamic_string) -> *mut i8;
    static mut io_block_size: i32;
    static mut create_dir_flag: i32;
    static mut retain_time_flag: i32;
    static mut crc_i_flag: i32;
    static mut append_flag: i32;
    static mut swapping_bytes: i32;
    static mut swapping_halfwords: i32;
    static mut set_owner_flag: i32;
    static mut set_owner: uid_t;
    static mut set_group_flag: i32;
    static mut set_group: gid_t;
    static mut no_chown_flag: i32;
    static mut sparse_flag: i32;
    static mut only_verify_crc_flag: i32;
    static mut warn_option: u32;
    static mut newdir_umask: mode_t;
    static mut renumber_inodes_option: i32;
    static mut ignore_devno_option: i32;
    static mut to_stdout_option: bool;
    static mut last_header_start: off_t;
    static mut new_media_message: *mut i8;
    static mut new_media_message_with_number: *mut i8;
    static mut new_media_message_after_number: *mut i8;
    static mut archive_name: *mut i8;
    static mut rsh_command_option: *mut i8;
    static mut crc: uint32_t;
    static mut input_buffer: *mut i8;
    static mut output_buffer: *mut i8;
    static mut in_buff: *mut i8;
    static mut out_buff: *mut i8;
    static mut input_buffer_size: size_t;
    static mut input_size: size_t;
    static mut output_size: size_t;
    static mut input_bytes: off_t;
    static mut output_bytes: off_t;
    static mut input_is_special: i8;
    static mut output_is_special: i8;
    static mut xstat: Option<unsafe extern "C" fn() -> i32>;
    static mut copy_function: Option<unsafe extern "C" fn() -> ()>;
    static mut change_directory_option: *mut i8;
    fn swab_array(arg: *mut i8, count: i32);
    fn process_copy_in();
    fn make_path(
        argpath: *const i8,
        owner: uid_t,
        group: gid_t,
        verbose_fmt_string: *const i8,
    ) -> i32;
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn full_write(fd: i32, buf: *const libc::c_void, count: size_t) -> size_t;
    static mut rmt_dev_name__: *const i8;
    fn rmt_open__(_: *const i8, _: i32, _: i32, _: *const i8) -> i32;
    fn rmt_close__(_: i32) -> i32;
    fn rmt_read__(_: i32, _: *mut i8, _: size_t) -> size_t;
    fn rmt_write__(_: i32, _: *mut i8, _: size_t) -> size_t;
    fn rmt_ioctl__(_: i32, _: u64, _: *mut libc::c_void) -> i32;
    static mut force_local_option: bool;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn fdutimens(_: i32, _: *const i8, _: *const timespec) -> i32;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
}
pub type __uint32_t = u32;
pub type __uintmax_t = u64;
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
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
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
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
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
pub struct dynamic_string {
    pub ds_size: size_t,
    pub ds_idx: size_t,
    pub ds_string: *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum archive_format {
    arf_unknown,
    arf_binary,
    arf_oldascii,
    arf_newascii,
    arf_crcascii,
    arf_tar,
    arf_ustar,
    arf_hpoldascii,
    arf_hpbinary,
}
impl archive_format {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            archive_format::arf_unknown => 0,
            archive_format::arf_binary => 1,
            archive_format::arf_oldascii => 2,
            archive_format::arf_newascii => 3,
            archive_format::arf_crcascii => 4,
            archive_format::arf_tar => 5,
            archive_format::arf_ustar => 6,
            archive_format::arf_hpoldascii => 7,
            archive_format::arf_hpbinary => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> archive_format {
        match value {
            0 => archive_format::arf_unknown,
            1 => archive_format::arf_binary,
            2 => archive_format::arf_oldascii,
            3 => archive_format::arf_newascii,
            4 => archive_format::arf_crcascii,
            5 => archive_format::arf_tar,
            6 => archive_format::arf_ustar,
            7 => archive_format::arf_hpoldascii,
            8 => archive_format::arf_hpbinary,
            _ => panic!("Invalid value for archive_format: {}", value),
        }
    }
}
impl AddAssign<u32> for archive_format {
    fn add_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for archive_format {
    fn sub_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for archive_format {
    fn mul_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for archive_format {
    fn div_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for archive_format {
    fn rem_assign(&mut self, rhs: u32) {
        *self = archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for archive_format {
    type Output = archive_format;
    fn add(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for archive_format {
    type Output = archive_format;
    fn sub(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for archive_format {
    type Output = archive_format;
    fn mul(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for archive_format {
    type Output = archive_format;
    fn div(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for archive_format {
    type Output = archive_format;
    fn rem(self, rhs: u32) -> archive_format {
        archive_format::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    in_zeros = 1,
    not_in_zeros = 2,
    begin = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::in_zeros => 1,
            C2RustUnnamed::not_in_zeros => 2,
            C2RustUnnamed::begin => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1 => C2RustUnnamed::in_zeros,
            2 => C2RustUnnamed::not_in_zeros,
            0 => C2RustUnnamed::begin,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inode_val {
    pub inode: ino_t,
    pub major_num: u64,
    pub minor_num: u64,
    pub trans_inode: ino_t,
    pub file_name: *mut i8,
}
pub type Hash_table = hash_table;
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delayed_set_stat {
    pub next: *mut delayed_set_stat,
    pub stat: cpio_file_stat,
    pub invert_permissions: mode_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn gnu_dev_makedev(mut __major: u32, mut __minor: u32) -> __dev_t {
    let mut __dev: __dev_t = 0;
    __dev = ((__major & 0xfff as u32) as __dev_t) << 8 as i32;
    __dev |= ((__major & 0xfffff000 as u32) as __dev_t) << 32 as i32;
    __dev |= ((__minor & 0xff as u32) as __dev_t) << 0 as i32;
    __dev |= ((__minor & 0xffffff00 as u32) as __dev_t) << 12 as i32;
    return __dev;
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> i32 {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as i32 as i64 != 0 {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = ((*__fp)._IO_read_ptr).offset(1);
        *(fresh0 as *mut u8) as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn tape_empty_output_buffer(mut out_des: i32) {
    let mut bytes_written: i32 = 0;
    bytes_written = (if out_des >= (1 as i32) << 30 as i32 {
        rmt_write__(out_des - ((1 as i32) << 30 as i32), output_buffer, output_size)
    } else {
        full_write(out_des, output_buffer as *const libc::c_void, output_size)
    }) as i32;
    if bytes_written as u64 != output_size {
        let mut rest_bytes_written: i32 = 0;
        let mut rest_output_size: i32 = 0;
        if output_is_special as i32 != 0
            && (bytes_written >= 0 as i32
                || (*__errno_location() == 28 as i32 || *__errno_location() == 5 as i32
                    || *__errno_location() == 6 as i32))
        {
            get_next_reel(out_des);
            if bytes_written > 0 as i32 {
                rest_output_size = output_size.wrapping_sub(bytes_written as u64) as i32;
            } else {
                rest_output_size = output_size as i32;
            }
            rest_bytes_written = (if out_des >= (1 as i32) << 30 as i32 {
                rmt_write__(
                    out_des - ((1 as i32) << 30 as i32),
                    output_buffer,
                    rest_output_size as size_t,
                )
            } else {
                full_write(
                    out_des,
                    output_buffer as *const libc::c_void,
                    rest_output_size as size_t,
                )
            }) as i32;
            if rest_bytes_written != rest_output_size {
                error(
                    2 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"write error\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
        } else {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"write error\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    output_bytes = (output_bytes as u64).wrapping_add(output_size) as off_t as off_t;
    out_buff = output_buffer;
    output_size = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn disk_empty_output_buffer(mut out_des: i32, mut flush: bool) {
    let mut bytes_written: ssize_t = 0;
    if swapping_halfwords != 0 || swapping_bytes != 0 {
        if swapping_halfwords != 0 {
            let mut complete_words: i32 = 0;
            complete_words = output_size.wrapping_div(4 as i32 as u64) as i32;
            swahw_array(output_buffer, complete_words);
            if swapping_bytes != 0 {
                swab_array(output_buffer, 2 as i32 * complete_words);
            }
        } else {
            let mut complete_halfwords: i32 = 0;
            complete_halfwords = output_size.wrapping_div(2 as i32 as u64) as i32;
            swab_array(output_buffer, complete_halfwords);
        }
    }
    if sparse_flag != 0 {
        bytes_written = sparse_write(out_des, output_buffer, output_size, flush);
    } else {
        bytes_written = write(
            out_des,
            output_buffer as *const libc::c_void,
            output_size,
        );
    }
    if bytes_written as u64 != output_size {
        if bytes_written == -(1 as i32) as i64 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"write error\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        } else {
            error(
                2 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"write error: partial write\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    }
    output_bytes = (output_bytes as u64).wrapping_add(output_size) as off_t as off_t;
    out_buff = output_buffer;
    output_size = 0 as i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn swahw_array(mut ptr: *mut i8, mut count: i32) {
    let mut tmp: i8 = 0;
    while count > 0 as i32 {
        tmp = *ptr;
        *ptr = *ptr.offset(2 as i32 as isize);
        *ptr.offset(2 as i32 as isize) = tmp;
        ptr = ptr.offset(1);
        ptr;
        tmp = *ptr;
        *ptr = *ptr.offset(2 as i32 as isize);
        *ptr.offset(2 as i32 as isize) = tmp;
        ptr = ptr.offset(3 as i32 as isize);
        count -= 1;
        count;
    }
}
unsafe extern "C" fn tape_fill_input_buffer(mut in_des: i32, mut num_bytes: i32) {
    in_buff = input_buffer;
    num_bytes = if num_bytes < io_block_size { num_bytes } else { io_block_size };
    input_size = if in_des >= (1 as i32) << 30 as i32 {
        rmt_read__(in_des - ((1 as i32) << 30 as i32), input_buffer, num_bytes as size_t)
    } else {
        safe_read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
    };
    if input_size == 0 as i32 as u64 && input_is_special as i32 != 0 {
        get_next_reel(in_des);
        input_size = if in_des >= (1 as i32) << 30 as i32 {
            rmt_read__(
                in_des - ((1 as i32) << 30 as i32),
                input_buffer,
                num_bytes as size_t,
            )
        } else {
            safe_read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
        };
    }
    if input_size == -(1 as i32) as size_t {
        error(
            2 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"read error\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if input_size == 0 as i32 as u64 {
        error(
            2 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"premature end of file\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    input_bytes = (input_bytes as u64).wrapping_add(input_size) as off_t as off_t;
}
unsafe extern "C" fn disk_fill_input_buffer(
    mut in_des: i32,
    mut num_bytes: off_t,
) -> i32 {
    in_buff = input_buffer;
    num_bytes = if num_bytes < 512 as i32 as i64 {
        num_bytes
    } else {
        512 as i32 as i64
    };
    input_size = read(in_des, input_buffer as *mut libc::c_void, num_bytes as size_t)
        as size_t;
    if input_size == -(1 as i32) as size_t {
        input_size = 0 as i32 as size_t;
        return -(1 as i32);
    } else if input_size == 0 as i32 as u64 {
        return 1 as i32
    }
    input_bytes = (input_bytes as u64).wrapping_add(input_size) as off_t as off_t;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn tape_buffered_write(
    mut in_buf: *mut i8,
    mut out_des: i32,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as i32 as i64 {
        space_left = (io_block_size as u64).wrapping_sub(output_size) as off_t;
        if space_left == 0 as i32 as i64 {
            tape_empty_output_buffer(out_des);
        } else {
            if bytes_left < space_left {
                space_left = bytes_left;
            }
            memcpy(
                out_buff as *mut libc::c_void,
                in_buf as *const libc::c_void,
                space_left as u32 as u64,
            );
            out_buff = out_buff.offset(space_left as isize);
            output_size = (output_size as u64).wrapping_add(space_left as u64) as size_t
                as size_t;
            in_buf = in_buf.offset(space_left as isize);
            bytes_left -= space_left;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn disk_buffered_write(
    mut in_buf: *mut i8,
    mut out_des: i32,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as i32 as i64 {
        space_left = (512 as i32 as u64).wrapping_sub(output_size) as off_t;
        if space_left == 0 as i32 as i64 {
            disk_empty_output_buffer(out_des, 0 as i32 != 0);
        } else {
            if bytes_left < space_left {
                space_left = bytes_left;
            }
            memcpy(
                out_buff as *mut libc::c_void,
                in_buf as *const libc::c_void,
                space_left as u32 as u64,
            );
            out_buff = out_buff.offset(space_left as isize);
            output_size = (output_size as u64).wrapping_add(space_left as u64) as size_t
                as size_t;
            in_buf = in_buf.offset(space_left as isize);
            bytes_left -= space_left;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tape_buffered_read(
    mut in_buf: *mut i8,
    mut in_des: i32,
    mut num_bytes: off_t,
) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as i32 as i64 {
        if input_size == 0 as i32 as u64 {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        if (bytes_left as u64) < input_size {
            space_left = bytes_left;
        } else {
            space_left = input_size as off_t;
        }
        memcpy(
            in_buf as *mut libc::c_void,
            in_buff as *const libc::c_void,
            space_left as u32 as u64,
        );
        in_buff = in_buff.offset(space_left as isize);
        in_buf = in_buf.offset(space_left as isize);
        input_size = (input_size as u64).wrapping_sub(space_left as u64) as size_t
            as size_t;
        bytes_left -= space_left;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tape_buffered_peek(
    mut peek_buf: *mut i8,
    mut in_des: i32,
    mut num_bytes: i32,
) -> i32 {
    let mut tmp_input_size: i64 = 0;
    let mut got_bytes: i64 = 0;
    let mut append_buf: *mut i8 = 0 as *mut i8;
    while input_size < num_bytes as u64 {
        append_buf = in_buff.offset(input_size as isize);
        if append_buf.offset_from(input_buffer) as i64 as u64 >= input_buffer_size {
            let mut half: i32 = 0;
            half = input_buffer_size.wrapping_div(2 as i32 as u64) as i32;
            memmove(
                input_buffer as *mut libc::c_void,
                input_buffer.offset(half as isize) as *const libc::c_void,
                half as u64,
            );
            in_buff = in_buff.offset(-(half as isize));
            append_buf = append_buf.offset(-(half as isize));
        }
        tmp_input_size = (if in_des >= (1 as i32) << 30 as i32 {
            rmt_read__(
                in_des - ((1 as i32) << 30 as i32),
                append_buf,
                io_block_size as size_t,
            )
        } else {
            safe_read(in_des, append_buf as *mut libc::c_void, io_block_size as size_t)
        }) as i64;
        if tmp_input_size == 0 as i32 as i64 {
            if !(input_is_special != 0) {
                break;
            }
            get_next_reel(in_des);
            tmp_input_size = (if in_des >= (1 as i32) << 30 as i32 {
                rmt_read__(
                    in_des - ((1 as i32) << 30 as i32),
                    append_buf,
                    io_block_size as size_t,
                )
            } else {
                safe_read(
                    in_des,
                    append_buf as *mut libc::c_void,
                    io_block_size as size_t,
                )
            }) as i64;
        }
        if tmp_input_size < 0 as i32 as i64 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"read error\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        input_bytes += tmp_input_size;
        input_size = (input_size as u64).wrapping_add(tmp_input_size as u64) as size_t
            as size_t;
    }
    if num_bytes as u64 <= input_size {
        got_bytes = num_bytes as i64;
    } else {
        got_bytes = input_size as i64;
    }
    memcpy(
        peek_buf as *mut libc::c_void,
        in_buff as *const libc::c_void,
        got_bytes as u32 as u64,
    );
    return got_bytes as i32;
}
#[no_mangle]
pub unsafe extern "C" fn tape_toss_input(mut in_des: i32, mut num_bytes: off_t) {
    let mut bytes_left: off_t = num_bytes;
    let mut space_left: off_t = 0;
    while bytes_left > 0 as i32 as i64 {
        if input_size == 0 as i32 as u64 {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        if (bytes_left as u64) < input_size {
            space_left = bytes_left;
        } else {
            space_left = input_size as off_t;
        }
        if crc_i_flag != 0 && only_verify_crc_flag != 0 {
            let mut k: i32 = 0;
            k = 0 as i32;
            while (k as i64) < space_left {
                crc = (crc as u32)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as i32 & 0xff as i32) as u32,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        in_buff = in_buff.offset(space_left as isize);
        input_size = (input_size as u64).wrapping_sub(space_left as u64) as size_t
            as size_t;
        bytes_left -= space_left;
    }
}
#[no_mangle]
pub unsafe extern "C" fn write_nuls_to_file(
    mut num_bytes: off_t,
    mut out_des: i32,
    mut writer: Option<unsafe extern "C" fn(*mut i8, i32, off_t) -> ()>,
) {
    let mut blocks: off_t = 0;
    let mut extra_bytes: off_t = 0;
    let mut i: off_t = 0;
    static mut zeros_512: [i8; 512] = [0; 512];
    blocks = (num_bytes as u64).wrapping_div(::core::mem::size_of::<[i8; 512]>() as u64)
        as off_t;
    extra_bytes = (num_bytes as u64)
        .wrapping_rem(::core::mem::size_of::<[i8; 512]>() as u64) as off_t;
    i = 0 as i32 as off_t;
    while i < blocks {
        writer
            .expect(
                "non-null function pointer",
            )(
            zeros_512.as_mut_ptr(),
            out_des,
            ::core::mem::size_of::<[i8; 512]>() as u64 as off_t,
        );
        i += 1;
        i;
    }
    if extra_bytes != 0 {
        writer
            .expect(
                "non-null function pointer",
            )(zeros_512.as_mut_ptr(), out_des, extra_bytes);
    }
}
#[no_mangle]
pub unsafe extern "C" fn copy_files_tape_to_disk(
    mut in_des: i32,
    mut out_des: i32,
    mut num_bytes: off_t,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    while num_bytes > 0 as i32 as i64 {
        if input_size == 0 as i32 as u64 {
            tape_fill_input_buffer(in_des, io_block_size);
        }
        size = (if input_size < num_bytes as u64 {
            input_size
        } else {
            num_bytes as u64
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as i32 as off_t;
            while k < size {
                crc = (crc as u32)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as i32 & 0xff as i32) as u32,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        disk_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as u64).wrapping_sub(size as u64) as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn copy_files_disk_to_tape(
    mut in_des: i32,
    mut out_des: i32,
    mut num_bytes: off_t,
    mut filename: *mut i8,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    let mut rc: i32 = 0;
    let mut original_num_bytes: off_t = 0;
    original_num_bytes = num_bytes;
    while num_bytes > 0 as i32 as i64 {
        if input_size == 0 as i32 as u64 {
            rc = disk_fill_input_buffer(
                in_des,
                if num_bytes < 512 as i32 as i64 { num_bytes } else { 512 as i32 as i64 },
            );
            if rc != 0 {
                if rc > 0 as i32 {
                    let mut buf: [i8; 21] = [0; 21];
                    error(
                        0 as i32,
                        0 as i32,
                        dcngettext(
                            0 as *const i8,
                            b"File %s shrunk by %s byte, padding with zeros\0"
                                as *const u8 as *const i8,
                            b"File %s shrunk by %s bytes, padding with zeros\0"
                                as *const u8 as *const i8,
                            num_bytes as u64,
                            5 as i32,
                        ),
                        filename,
                        umaxtostr(num_bytes as uintmax_t, buf.as_mut_ptr()),
                    );
                } else {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Read error at byte %lld in file %s, padding with zeros\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (original_num_bytes - num_bytes) as libc::c_longlong,
                        filename,
                    );
                }
                write_nuls_to_file(
                    num_bytes,
                    out_des,
                    Some(
                        tape_buffered_write
                            as unsafe extern "C" fn(*mut i8, i32, off_t) -> (),
                    ),
                );
                break;
            }
        }
        size = (if input_size < num_bytes as u64 {
            input_size
        } else {
            num_bytes as u64
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as i32 as off_t;
            while k < size {
                crc = (crc as u32)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as i32 & 0xff as i32) as u32,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        tape_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as u64).wrapping_sub(size as u64) as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn copy_files_disk_to_disk(
    mut in_des: i32,
    mut out_des: i32,
    mut num_bytes: off_t,
    mut filename: *mut i8,
) {
    let mut size: off_t = 0;
    let mut k: off_t = 0;
    let mut original_num_bytes: off_t = 0;
    let mut rc: i32 = 0;
    original_num_bytes = num_bytes;
    while num_bytes > 0 as i32 as i64 {
        if input_size == 0 as i32 as u64 {
            rc = disk_fill_input_buffer(in_des, num_bytes);
            if rc != 0 {
                if rc > 0 as i32 {
                    let mut buf: [i8; 21] = [0; 21];
                    error(
                        0 as i32,
                        0 as i32,
                        dcngettext(
                            0 as *const i8,
                            b"File %s shrunk by %s byte, padding with zeros\0"
                                as *const u8 as *const i8,
                            b"File %s shrunk by %s bytes, padding with zeros\0"
                                as *const u8 as *const i8,
                            num_bytes as u64,
                            5 as i32,
                        ),
                        filename,
                        umaxtostr(num_bytes as uintmax_t, buf.as_mut_ptr()),
                    );
                } else {
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Read error at byte %lld in file %s, padding with zeros\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        (original_num_bytes - num_bytes) as libc::c_longlong,
                        filename,
                    );
                }
                write_nuls_to_file(
                    num_bytes,
                    out_des,
                    Some(
                        disk_buffered_write
                            as unsafe extern "C" fn(*mut i8, i32, off_t) -> (),
                    ),
                );
                break;
            }
        }
        size = (if input_size < num_bytes as u64 {
            input_size
        } else {
            num_bytes as u64
        }) as off_t;
        if crc_i_flag != 0 {
            k = 0 as i32 as off_t;
            while k < size {
                crc = (crc as u32)
                    .wrapping_add(
                        (*in_buff.offset(k as isize) as i32 & 0xff as i32) as u32,
                    ) as uint32_t as uint32_t;
                k += 1;
                k;
            }
        }
        disk_buffered_write(in_buff, out_des, size);
        num_bytes -= size;
        input_size = (input_size as u64).wrapping_sub(size as u64) as size_t as size_t;
        in_buff = in_buff.offset(size as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn warn_if_file_changed(
    mut file_name: *mut i8,
    mut old_file_size: off_t,
    mut old_file_mtime: time_t,
) {
    let mut new_file_stat: stat = stat {
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
    if ::core::mem::transmute::<
        _,
        fn(_, _) -> i32,
    >(
        (Some(xstat.expect("non-null function pointer")))
            .expect("non-null function pointer"),
    )(file_name, &mut new_file_stat) < 0 as i32
    {
        stat_error(file_name);
        return;
    }
    if new_file_stat.st_size > old_file_size {
        error(
            0 as i32,
            0 as i32,
            dcngettext(
                0 as *const i8,
                b"File %s grew, %lu new byte not copied\0" as *const u8 as *const i8,
                b"File %s grew, %lu new bytes not copied\0" as *const u8 as *const i8,
                (new_file_stat.st_size - old_file_size) as u64,
                5 as i32,
            ),
            file_name,
            (new_file_stat.st_size - old_file_size) as uintmax_t,
        );
    } else if new_file_stat.st_mtim.tv_sec != old_file_mtime {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"File %s was modified while being copied\0" as *const u8 as *const i8,
                5 as i32,
            ),
            file_name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_all_directories(mut name: *const i8) {
    let mut dir: *mut i8 = 0 as *mut i8;
    dir = dir_name(name);
    if dir.is_null() {
        error(
            2 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"virtual memory exhausted\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if *dir.offset(0 as i32 as isize) as i32 != '.' as i32
        || *dir.offset(1 as i32 as isize) as i32 != '\0' as i32
    {
        let mut fmt: *const i8 = 0 as *const i8;
        if warn_option & 0x2 as i32 as u32 != 0 {
            fmt = dcgettext(
                0 as *const i8,
                b"Creating intermediate directory `%s'\0" as *const u8 as *const i8,
                5 as i32,
            );
        } else {
            fmt = 0 as *const i8;
        }
        make_path(dir, -(1 as i32) as uid_t, -(1 as i32) as gid_t, fmt);
    }
    rpl_free(dir as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn prepare_append(mut out_file_des: i32) {
    let mut start_of_header: off_t = 0;
    let mut start_of_block: off_t = 0;
    let mut useful_bytes_in_block: size_t = 0;
    let mut tmp_buf: *mut i8 = 0 as *mut i8;
    start_of_header = last_header_start;
    useful_bytes_in_block = (start_of_header % io_block_size as i64) as size_t;
    start_of_block = (start_of_header as u64).wrapping_sub(useful_bytes_in_block)
        as off_t;
    if lseek(out_file_des, start_of_block, 0 as i32) < 0 as i32 as i64 {
        error(
            2 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot seek on output\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if useful_bytes_in_block > 0 as i32 as u64 {
        tmp_buf = xmalloc(useful_bytes_in_block) as *mut i8;
        read(out_file_des, tmp_buf as *mut libc::c_void, useful_bytes_in_block);
        if lseek(out_file_des, start_of_block, 0 as i32) < 0 as i32 as i64 {
            error(
                2 as i32,
                *__errno_location(),
                dcgettext(
                    0 as *const i8,
                    b"cannot seek on output\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        tape_buffered_write(tmp_buf, out_file_des, useful_bytes_in_block as off_t);
        rpl_free(tmp_buf as *mut libc::c_void);
    }
    input_size = 0 as i32 as size_t;
    input_bytes = 0 as i32 as off_t;
    in_buff = input_buffer;
}
static mut hash_table: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn inode_val_hasher(
    mut val: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut ival: *const inode_val = val as *const inode_val;
    return ((*ival).inode).wrapping_rem(n_buckets);
}
unsafe extern "C" fn inode_val_compare(
    mut val1: *const libc::c_void,
    mut val2: *const libc::c_void,
) -> bool {
    let mut ival1: *const inode_val = val1 as *const inode_val;
    let mut ival2: *const inode_val = val2 as *const inode_val;
    return (*ival1).inode == (*ival2).inode && (*ival1).major_num == (*ival2).major_num
        && (*ival1).minor_num == (*ival2).minor_num;
}
unsafe extern "C" fn find_inode_val(
    mut node_num: ino_t,
    mut major_num: u64,
    mut minor_num: u64,
) -> *mut inode_val {
    let mut sample: inode_val = inode_val {
        inode: 0,
        major_num: 0,
        minor_num: 0,
        trans_inode: 0,
        file_name: 0 as *mut i8,
    };
    if hash_table.is_null() {
        return 0 as *mut inode_val;
    }
    sample.inode = node_num;
    sample.major_num = major_num;
    sample.minor_num = minor_num;
    return hash_lookup(hash_table, &mut sample as *mut inode_val as *const libc::c_void)
        as *mut inode_val;
}
#[no_mangle]
pub unsafe extern "C" fn find_inode_file(
    mut node_num: ino_t,
    mut major_num: u64,
    mut minor_num: u64,
) -> *mut i8 {
    let mut ival: *mut inode_val = find_inode_val(node_num, major_num, minor_num);
    return if !ival.is_null() { (*ival).file_name } else { 0 as *mut i8 };
}
static mut next_inode: ino_t = 0;
#[no_mangle]
pub unsafe extern "C" fn add_inode(
    mut node_num: ino_t,
    mut file_name: *mut i8,
    mut major_num: u64,
    mut minor_num: u64,
) -> *mut inode_val {
    let mut temp: *mut inode_val = 0 as *mut inode_val;
    let mut e: *mut inode_val = 0 as *mut inode_val;
    temp = xmalloc(::core::mem::size_of::<inode_val>() as u64) as *mut inode_val;
    (*temp).inode = node_num;
    (*temp).major_num = major_num;
    (*temp).minor_num = minor_num;
    (*temp).file_name = if !file_name.is_null() {
        xstrdup(file_name)
    } else {
        0 as *mut i8
    };
    if renumber_inodes_option != 0 {
        let fresh1 = next_inode;
        next_inode = next_inode.wrapping_add(1);
        (*temp).trans_inode = fresh1;
    } else {
        (*temp).trans_inode = (*temp).inode;
    }
    if !((!hash_table.is_null()
        || {
            hash_table = hash_initialize(
                0 as i32 as size_t,
                0 as *const Hash_tuning,
                Some(
                    inode_val_hasher
                        as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
                ),
                Some(
                    inode_val_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> bool,
                ),
                None,
            );
            !hash_table.is_null()
        })
        && {
            e = hash_insert(hash_table, temp as *const libc::c_void) as *mut inode_val;
            !e.is_null()
        })
    {
        xalloc_die();
    }
    return e;
}
unsafe extern "C" fn get_inode_and_dev(mut hdr: *mut cpio_file_stat, mut st: *mut stat) {
    if renumber_inodes_option != 0 {
        if (*st).st_nlink > 1 as i32 as u64 {
            let mut ival: *mut inode_val = find_inode_val(
                (*st).st_ino,
                gnu_dev_major((*st).st_dev) as u64,
                gnu_dev_minor((*st).st_dev) as u64,
            );
            if ival.is_null() {
                ival = add_inode(
                    (*st).st_ino,
                    0 as *mut i8,
                    gnu_dev_major((*st).st_dev) as u64,
                    gnu_dev_minor((*st).st_dev) as u64,
                );
            }
            (*hdr).c_ino = (*ival).trans_inode;
        } else {
            let fresh2 = next_inode;
            next_inode = next_inode.wrapping_add(1);
            (*hdr).c_ino = fresh2;
        }
    } else {
        (*hdr).c_ino = (*st).st_ino;
    }
    if ignore_devno_option != 0 {
        (*hdr).c_dev_maj = 0 as i32 as u32;
        (*hdr).c_dev_min = 0 as i32 as u32;
    } else {
        (*hdr).c_dev_maj = gnu_dev_major((*st).st_dev);
        (*hdr).c_dev_min = gnu_dev_minor((*st).st_dev);
    };
}
#[no_mangle]
pub unsafe extern "C" fn open_archive(mut file: *mut i8) -> i32 {
    let mut fd: i32 = 0;
    let mut copy_in: Option<unsafe extern "C" fn() -> ()> = None;
    copy_in = ::core::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        Option<unsafe extern "C" fn() -> ()>,
    >(Some(process_copy_in as unsafe extern "C" fn() -> ()));
    if copy_function == copy_in {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as i64 as u64,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0 as i32 | 0 as i32,
                (1 as i32) << 30 as i32,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0 as i32 | 0 as i32,
                0o200 as i32 | 0o200 as i32 >> 3 as i32
                    | 0o200 as i32 >> 3 as i32 >> 3 as i32
                    | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                        | 0o400 as i32 >> 3 as i32 >> 3 as i32),
            )
        };
    } else if append_flag == 0 {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as i64 as u64,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0o1 as i32 | 0o100 as i32 | 0o1000 as i32 | 0 as i32,
                (1 as i32) << 30 as i32,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0o1 as i32 | 0o100 as i32 | 0o1000 as i32 | 0 as i32,
                0o200 as i32 | 0o200 as i32 >> 3 as i32
                    | 0o200 as i32 >> 3 as i32 >> 3 as i32
                    | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                        | 0o400 as i32 >> 3 as i32 >> 3 as i32),
            )
        };
    } else {
        fd = if !force_local_option
            && {
                rmt_dev_name__ = strchr(file, ':' as i32);
                !rmt_dev_name__.is_null()
            } && rmt_dev_name__ > file
            && (memchr(
                file as *const libc::c_void,
                '/' as i32,
                rmt_dev_name__.offset_from(file) as i64 as u64,
            ))
                .is_null()
        {
            rmt_open__(
                file,
                0o2 as i32 | 0 as i32,
                (1 as i32) << 30 as i32,
                rsh_command_option,
            )
        } else {
            open(
                file,
                0o2 as i32 | 0 as i32,
                0o200 as i32 | 0o200 as i32 >> 3 as i32
                    | 0o200 as i32 >> 3 as i32 >> 3 as i32
                    | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                        | 0o400 as i32 >> 3 as i32 >> 3 as i32),
            )
        };
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn tape_offline(mut tape_des: i32) {
    let mut control: mtop = mtop { mt_op: 0, mt_count: 0 };
    control.mt_op = 7 as i32 as libc::c_short;
    control.mt_count = 1 as i32;
    if tape_des >= (1 as i32) << 30 as i32 {
        rmt_ioctl__(
            tape_des - ((1 as i32) << 30 as i32),
            ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                | ((1 as i32) << 0 as i32) as u32) as u64
                | (::core::mem::size_of::<mtop>() as u64)
                    << 0 as i32 + 8 as i32 + 8 as i32,
            &mut control as *mut mtop as *mut i8 as *mut libc::c_void,
        );
    } else {
        ioctl(
            tape_des,
            ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                | ((1 as i32) << 0 as i32) as u32) as u64
                | (::core::mem::size_of::<mtop>() as u64)
                    << 0 as i32 + 8 as i32 + 8 as i32,
            &mut control as *mut mtop as *mut i8,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_next_reel(mut tape_des: i32) {
    static mut reel_number: i32 = 1 as i32;
    let mut tty_in: *mut FILE = 0 as *mut FILE;
    let mut tty_out: *mut FILE = 0 as *mut FILE;
    let mut old_tape_des: i32 = 0;
    let mut next_archive_name: *mut i8 = 0 as *mut i8;
    let mut new_name: dynamic_string = {
        let mut init = dynamic_string {
            ds_size: 0 as i32 as size_t,
            ds_idx: 0 as i32 as size_t,
            ds_string: 0 as *mut i8,
        };
        init
    };
    let mut str_res: *mut i8 = 0 as *mut i8;
    tty_in = fopen(
        b"/dev/tty\0" as *const u8 as *const i8,
        b"r\0" as *const u8 as *const i8,
    );
    if tty_in.is_null() {
        error(2 as i32, *__errno_location(), b"/dev/tty\0" as *const u8 as *const i8);
    }
    tty_out = fopen(
        b"/dev/tty\0" as *const u8 as *const i8,
        b"w\0" as *const u8 as *const i8,
    );
    if tty_out.is_null() {
        error(2 as i32, *__errno_location(), b"/dev/tty\0" as *const u8 as *const i8);
    }
    old_tape_des = tape_des;
    tape_offline(tape_des);
    if tape_des >= (1 as i32) << 30 as i32 {
        rmt_close__(tape_des - ((1 as i32) << 30 as i32));
    } else {
        close(tape_des);
    };
    reel_number += 1;
    reel_number;
    if !new_media_message.is_null() {
        fprintf(tty_out, b"%s\0" as *const u8 as *const i8, new_media_message);
    } else if !new_media_message_with_number.is_null() {
        fprintf(
            tty_out,
            b"%s%d%s\0" as *const u8 as *const i8,
            new_media_message_with_number,
            reel_number,
            new_media_message_after_number,
        );
    } else if !archive_name.is_null() {
        fprintf(
            tty_out,
            dcgettext(
                0 as *const i8,
                b"Found end of tape.  Load next tape and press RETURN. \0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    } else {
        fprintf(
            tty_out,
            dcgettext(
                0 as *const i8,
                b"Found end of tape.  To continue, type device/file name when ready.\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    fflush_unlocked(tty_out);
    if !archive_name.is_null() {
        let mut c: i32 = 0;
        loop {
            c = getc_unlocked(tty_in);
            if !(c != -(1 as i32) && c != '\n' as i32) {
                break;
            }
        }
        tape_des = open_archive(archive_name);
        if tape_des == -(1 as i32) {
            open_error(archive_name);
        }
    } else {
        loop {
            if tape_des < 0 as i32 {
                fprintf(
                    tty_out,
                    dcgettext(
                        0 as *const i8,
                        b"To continue, type device/file name when ready.\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                fflush_unlocked(tty_out);
            }
            str_res = ds_fgets(tty_in, &mut new_name);
            if str_res.is_null()
                || *str_res.offset(0 as i32 as isize) as i32 == '\0' as i32
            {
                exit(2 as i32);
            }
            next_archive_name = str_res;
            tape_des = open_archive(next_archive_name);
            if tape_des == -(1 as i32) {
                open_error(next_archive_name);
            }
            if !(tape_des < 0 as i32) {
                break;
            }
        }
    }
    if tape_des != old_tape_des {
        error(
            2 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"internal error: tape descriptor changed from %d to %d\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            old_tape_des,
            tape_des,
        );
    }
    ds_free(&mut new_name);
    fclose(tty_in);
    fclose(tty_out);
}
#[no_mangle]
pub unsafe extern "C" fn set_new_media_message(mut message: *mut i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut prev_was_percent: i32 = 0;
    p = message;
    prev_was_percent = 0 as i32;
    while *p as i32 != '\0' as i32 {
        if *p as i32 == 'd' as i32 && prev_was_percent != 0 {
            break;
        }
        prev_was_percent = (*p as i32 == '%' as i32) as i32;
        p = p.offset(1);
        p;
    }
    if *p as i32 == '\0' as i32 {
        new_media_message = xstrdup(message);
    } else {
        let mut length: i32 = (p.offset_from(message) as i64 - 1 as i32 as i64) as i32;
        new_media_message_with_number = xmalloc((length + 1 as i32) as size_t)
            as *mut i8;
        strncpy(new_media_message_with_number, message, length as u64);
        *new_media_message_with_number.offset(length as isize) = '\0' as i32 as i8;
        length = strlen(p.offset(1 as i32 as isize)) as i32;
        new_media_message_after_number = xmalloc((length + 1 as i32) as size_t)
            as *mut i8;
        strcpy(new_media_message_after_number, p.offset(1 as i32 as isize));
    };
}
unsafe extern "C" fn buf_all_zeros(mut buf: *mut i8, mut bufsize: i32) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < bufsize {
        let fresh3 = buf;
        buf = buf.offset(1);
        if *fresh3 as i32 != '\0' as i32 {
            return 0 as i32;
        }
        i += 1;
        i;
    }
    return 1 as i32;
}
unsafe extern "C" fn sparse_write(
    mut fildes: i32,
    mut buf: *mut i8,
    mut nbytes: size_t,
    mut flush: bool,
) -> ssize_t {
    let mut nwritten: size_t = 0 as i32 as size_t;
    let mut n: ssize_t = 0;
    let mut start_ptr: *mut i8 = buf;
    static mut delayed_seek_count: off_t = 0 as i32 as off_t;
    let mut seek_count: off_t = 0 as i32 as off_t;
    let mut state: C2RustUnnamed = C2RustUnnamed::from_libc_c_uint(
        (if delayed_seek_count != 0 {
            C2RustUnnamed::in_zeros as i32
        } else {
            C2RustUnnamed::begin as i32
        }) as u32,
    );
    while nbytes != 0 {
        let mut rest: size_t = nbytes;
        if rest < 512 as i32 as u64 {
            state = C2RustUnnamed::not_in_zeros;
        } else if buf_all_zeros(buf, rest as i32) != 0 {
            if state as u32 == C2RustUnnamed::not_in_zeros as i32 as u32 {
                let mut bytes: ssize_t = (buf.offset_from(start_ptr) as i64 as u64)
                    .wrapping_add(rest) as ssize_t;
                n = write(fildes, start_ptr as *const libc::c_void, bytes as size_t);
                if n == -(1 as i32) as i64 {
                    return -(1 as i32) as ssize_t;
                }
                nwritten = (nwritten as u64).wrapping_add(n as u64) as size_t as size_t;
                if n < bytes {
                    return nwritten.wrapping_add(seek_count as u64) as ssize_t;
                }
                start_ptr = 0 as *mut i8;
            } else {
                seek_count = (seek_count as u64).wrapping_add(rest) as off_t as off_t;
            }
            state = C2RustUnnamed::in_zeros;
        } else {
            seek_count += delayed_seek_count;
            if lseek(fildes, seek_count, 1 as i32) == -(1 as i32) as i64 {
                return -(1 as i32) as ssize_t;
            }
            seek_count = 0 as i32 as off_t;
            delayed_seek_count = seek_count;
            state = C2RustUnnamed::not_in_zeros;
            start_ptr = buf;
        }
        buf = buf.offset(rest as isize);
        nbytes = (nbytes as u64).wrapping_sub(rest) as size_t as size_t;
    }
    if state as u32 != C2RustUnnamed::in_zeros as i32 as u32 {
        seek_count += delayed_seek_count;
        if seek_count != 0 && lseek(fildes, seek_count, 1 as i32) == -(1 as i32) as i64 {
            return -(1 as i32) as ssize_t;
        }
        seek_count = 0 as i32 as off_t;
        delayed_seek_count = seek_count;
        n = write(
            fildes,
            start_ptr as *const libc::c_void,
            buf.offset_from(start_ptr) as i64 as size_t,
        );
        if n == -(1 as i32) as i64 {
            return n;
        }
        nwritten = (nwritten as u64).wrapping_add(n as u64) as size_t as size_t;
    }
    delayed_seek_count += seek_count;
    if flush as i32 != 0 && delayed_seek_count != 0 {
        if lseek(fildes, delayed_seek_count - 1 as i32 as i64, 1 as i32)
            == -(1 as i32) as i64
        {
            return -(1 as i32) as ssize_t;
        }
        n = write(
            fildes,
            b"\0" as *const u8 as *const i8 as *const libc::c_void,
            1 as i32 as size_t,
        );
        if n != 1 as i32 as i64 {
            return n;
        }
        delayed_seek_count = 0 as i32 as off_t;
    }
    return nwritten.wrapping_add(seek_count as u64) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn stat_to_cpio(mut hdr: *mut cpio_file_stat, mut st: *mut stat) {
    get_inode_and_dev(hdr, st);
    (*hdr).c_mode = (*st).st_mode & 0o7777 as i32 as u32;
    if (*st).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
        (*hdr).c_mode |= 0o100000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
        (*hdr).c_mode |= 0o40000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32 {
        (*hdr).c_mode |= 0o60000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32 {
        (*hdr).c_mode |= 0o20000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o10000 as i32 as u32 {
        (*hdr).c_mode |= 0o10000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
        (*hdr).c_mode |= 0o120000 as i32 as u32;
    } else if (*st).st_mode & 0o170000 as i32 as u32 == 0o140000 as i32 as u32 {
        (*hdr).c_mode |= 0o140000 as i32 as u32;
    }
    (*hdr).c_nlink = (*st).st_nlink;
    (*hdr).c_uid = if set_owner_flag != 0 { set_owner } else { (*st).st_uid };
    (*hdr).c_gid = if set_group_flag != 0 { set_group } else { (*st).st_gid };
    if (*st).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        || (*st).st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32
    {
        (*hdr).c_rdev_maj = gnu_dev_major((*st).st_rdev);
        (*hdr).c_rdev_min = gnu_dev_minor((*st).st_rdev);
    } else {
        (*hdr).c_rdev_maj = 0 as i32 as u32;
        (*hdr).c_rdev_min = 0 as i32 as u32;
    }
    (*hdr).c_mtime = (*st).st_mtim.tv_sec;
    (*hdr).c_filesize = (*st).st_size;
    (*hdr).c_chksum = 0 as i32 as uint32_t;
    (*hdr).c_tar_linkname = 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn cpio_to_stat(mut st: *mut stat, mut hdr: *mut cpio_file_stat) {
    memset(st as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<stat>() as u64);
    (*st).st_dev = gnu_dev_makedev((*hdr).c_dev_maj, (*hdr).c_dev_min);
    (*st).st_ino = (*hdr).c_ino;
    (*st).st_mode = (*hdr).c_mode & 0o777 as i32 as u32;
    if (*hdr).c_mode & 0o100000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o100000 as i32 as u32;
    } else if (*hdr).c_mode & 0o40000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o40000 as i32 as u32;
    } else if (*hdr).c_mode & 0o60000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o60000 as i32 as u32;
    } else if (*hdr).c_mode & 0o20000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o20000 as i32 as u32;
    } else if (*hdr).c_mode & 0o120000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o120000 as i32 as u32;
    } else if (*hdr).c_mode & 0o140000 as i32 as u32 != 0 {
        (*st).st_mode |= 0o140000 as i32 as u32;
    }
    (*st).st_nlink = (*hdr).c_nlink;
    (*st).st_uid = if set_owner_flag != 0 { set_owner } else { (*hdr).c_uid };
    (*st).st_gid = if set_group_flag != 0 { set_group } else { (*hdr).c_gid };
    (*st).st_rdev = gnu_dev_makedev((*hdr).c_rdev_maj, (*hdr).c_rdev_min);
    (*st).st_mtim.tv_sec = (*hdr).c_mtime;
    (*st).st_size = (*hdr).c_filesize;
}
#[no_mangle]
pub unsafe extern "C" fn fchown_or_chown(
    mut fd: i32,
    mut name: *const i8,
    mut uid: uid_t,
    mut gid: uid_t,
) -> i32 {
    if 1 as i32 != 0 && fd != -(1 as i32) {
        return fchown(fd, uid, gid)
    } else {
        return chown(name, uid, gid)
    };
}
#[no_mangle]
pub unsafe extern "C" fn fchmod_or_chmod(
    mut fd: i32,
    mut name: *const i8,
    mut mode: mode_t,
) -> i32 {
    if 1 as i32 != 0 && fd != -(1 as i32) {
        return fchmod(fd, mode)
    } else {
        return chmod(name, mode)
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_perms(mut fd: i32, mut header: *mut cpio_file_stat) {
    if no_chown_flag == 0 {
        let mut uid: uid_t = if set_owner_flag != 0 {
            set_owner
        } else {
            (*header).c_uid
        };
        let mut gid: gid_t = if set_group_flag != 0 {
            set_group
        } else {
            (*header).c_gid
        };
        if fchown_or_chown(fd, (*header).c_name, uid, gid) < 0 as i32
            && *__errno_location() != 1 as i32
        {
            chown_error_details((*header).c_name, uid, gid);
        }
    }
    if fchmod_or_chmod(fd, (*header).c_name, (*header).c_mode) < 0 as i32 {
        chmod_error_details((*header).c_name, (*header).c_mode);
    }
    if retain_time_flag != 0 {
        set_file_times(
            fd,
            (*header).c_name,
            (*header).c_mtime as u64,
            (*header).c_mtime as u64,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn set_file_times(
    mut fd: i32,
    mut name: *const i8,
    mut atime: u64,
    mut mtime: u64,
) {
    let mut ts: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    memset(
        &mut ts as *mut [timespec; 2] as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<[timespec; 2]>() as u64,
    );
    ts[0 as i32 as usize].tv_sec = atime as __time_t;
    ts[1 as i32 as usize].tv_sec = mtime as __time_t;
    if fdutimens(fd, name, ts.as_mut_ptr() as *const timespec) < 0 as i32
        && *__errno_location() != 30 as i32
    {
        utime_error(name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpio_realloc_c_name(
    mut file_hdr: *mut cpio_file_stat,
    mut len: size_t,
) {
    while (*file_hdr).c_name_buflen < len {
        (*file_hdr).c_name = x2realloc(
            (*file_hdr).c_name as *mut libc::c_void,
            &mut (*file_hdr).c_name_buflen,
        ) as *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cpio_set_c_name(
    mut file_hdr: *mut cpio_file_stat,
    mut name: *mut i8,
) {
    let mut len: size_t = (strlen(name)).wrapping_add(1 as i32 as u64);
    cpio_realloc_c_name(file_hdr, len);
    (*file_hdr).c_namesize = len;
    memmove((*file_hdr).c_name as *mut libc::c_void, name as *const libc::c_void, len);
}
#[no_mangle]
pub unsafe extern "C" fn cpio_safer_name_suffix(
    mut name: *mut i8,
    mut link_target: bool,
    mut absolute_names: bool,
    mut strip_leading_dots: bool,
) {
    let mut p: *mut i8 = safer_name_suffix(name, link_target, absolute_names);
    if strip_leading_dots as i32 != 0
        && strcmp(p, b"./\0" as *const u8 as *const i8) != 0
    {
        while *p as i32 == '.' as i32
            && *p.offset(1 as i32 as isize) as i32 == '/' as i32
        {
            p = p.offset(1);
            p;
            while *p as i32 == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    if p != name {
        memmove(
            name as *mut libc::c_void,
            p as *const libc::c_void,
            (strlen(p)).wrapping_add(1 as i32 as u64),
        );
    }
}
static mut delayed_set_stat_head: *mut delayed_set_stat = 0 as *const delayed_set_stat
    as *mut delayed_set_stat;
#[no_mangle]
pub unsafe extern "C" fn delay_cpio_set_stat(
    mut file_stat: *mut cpio_file_stat,
    mut invert_permissions: mode_t,
) {
    let mut file_name_len: size_t = strlen((*file_stat).c_name);
    let mut data: *mut delayed_set_stat = xmalloc(
        (::core::mem::size_of::<delayed_set_stat>() as u64)
            .wrapping_add(file_name_len)
            .wrapping_add(1 as i32 as u64),
    ) as *mut delayed_set_stat;
    (*data).next = delayed_set_stat_head;
    memcpy(
        &mut (*data).stat as *mut cpio_file_stat as *mut libc::c_void,
        file_stat as *const libc::c_void,
        ::core::mem::size_of::<cpio_file_stat>() as u64,
    );
    (*data).stat.c_name = data.offset(1 as i32 as isize) as *mut i8;
    strcpy((*data).stat.c_name, (*file_stat).c_name);
    (*data).invert_permissions = invert_permissions;
    delayed_set_stat_head = data;
}
#[no_mangle]
pub unsafe extern "C" fn delay_set_stat(
    mut file_name: *const i8,
    mut st: *mut stat,
    mut invert_permissions: mode_t,
) {
    let mut fs: cpio_file_stat = cpio_file_stat {
        c_magic: 0,
        c_ino: 0,
        c_mode: 0,
        c_uid: 0,
        c_gid: 0,
        c_nlink: 0,
        c_mtime: 0,
        c_filesize: 0,
        c_dev_maj: 0,
        c_dev_min: 0,
        c_rdev_maj: 0,
        c_rdev_min: 0,
        c_namesize: 0,
        c_chksum: 0,
        c_name: 0 as *mut i8,
        c_name_buflen: 0,
        c_tar_linkname: 0 as *const i8,
    };
    stat_to_cpio(&mut fs, st);
    fs.c_name = file_name as *mut i8;
    delay_cpio_set_stat(&mut fs, invert_permissions);
}
#[no_mangle]
pub unsafe extern "C" fn repair_inter_delayed_set_stat(
    mut dir_stat_info: *mut stat,
) -> i32 {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
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
        if stat((*data).stat.c_name, &mut st) != 0 as i32 {
            stat_error((*data).stat.c_name);
            return -(1 as i32);
        }
        if st.st_dev == (*dir_stat_info).st_dev && st.st_ino == (*dir_stat_info).st_ino {
            stat_to_cpio(&mut (*data).stat, dir_stat_info);
            (*data).invert_permissions = ((*dir_stat_info).st_mode ^ st.st_mode)
                & (0o100 as i32 | 0o100 as i32 >> 3 as i32
                    | 0o100 as i32 >> 3 as i32 >> 3 as i32
                    | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32))) as u32
                & !newdir_umask;
            return 0 as i32;
        }
        data = (*data).next;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn repair_delayed_set_stat(
    mut file_hdr: *mut cpio_file_stat,
) -> i32 {
    let mut data: *mut delayed_set_stat = 0 as *mut delayed_set_stat;
    data = delayed_set_stat_head;
    while !data.is_null() {
        if strcmp((*file_hdr).c_name, (*data).stat.c_name) == 0 as i32 {
            (*data).invert_permissions = 0 as i32 as mode_t;
            memcpy(
                &mut (*data).stat as *mut cpio_file_stat as *mut libc::c_void,
                file_hdr as *const libc::c_void,
                88 as u64,
            );
            return 0 as i32;
        }
        data = (*data).next;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn apply_delayed_set_stat() {
    while !delayed_set_stat_head.is_null() {
        let mut data: *mut delayed_set_stat = delayed_set_stat_head;
        if (*data).invert_permissions != 0 {
            (*data).stat.c_mode ^= (*data).invert_permissions;
        }
        set_perms(-(1 as i32), &mut (*data).stat);
        delayed_set_stat_head = (*data).next;
        rpl_free(data as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpio_mkdir(
    mut file_hdr: *mut cpio_file_stat,
    mut setstat_delayed: *mut i32,
) -> i32 {
    let mut rc: i32 = 0;
    let mut mode: mode_t = (*file_hdr).c_mode;
    if (*file_hdr).c_mode & 0o200 as i32 as u32 == 0 {
        rc = mkdir((*file_hdr).c_name, mode | 0o200 as i32 as u32);
        if rc == 0 as i32 {
            delay_cpio_set_stat(file_hdr, 0 as i32 as mode_t);
            *setstat_delayed = 1 as i32;
        }
    } else {
        rc = mkdir((*file_hdr).c_name, mode);
        *setstat_delayed = 0 as i32;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn cpio_create_dir(
    mut file_hdr: *mut cpio_file_stat,
    mut existing_dir: i32,
) -> i32 {
    let mut res: i32 = 0;
    let mut setstat_delayed: i32 = 0 as i32;
    if to_stdout_option {
        return 0 as i32;
    }
    strip_trailing_slashes((*file_hdr).c_name);
    if *((*file_hdr).c_name).offset(0 as i32 as isize) as i32 == '.' as i32
        && *((*file_hdr).c_name).offset(1 as i32 as isize) as i32 == '\0' as i32
    {
        return 0 as i32;
    }
    if existing_dir == 0 {
        res = cpio_mkdir(file_hdr, &mut setstat_delayed);
    } else {
        res = 0 as i32;
    }
    if res < 0 as i32 && create_dir_flag != 0 {
        create_all_directories((*file_hdr).c_name);
        res = cpio_mkdir(file_hdr, &mut setstat_delayed);
    }
    if res < 0 as i32 {
        let mut file_stat: stat = stat {
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
        if *__errno_location() != 17 as i32 {
            mkdir_error((*file_hdr).c_name);
            return -(1 as i32);
        }
        if lstat((*file_hdr).c_name, &mut file_stat) != 0 {
            stat_error((*file_hdr).c_name);
            return -(1 as i32);
        }
        if !(file_stat.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is not a directory\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_colon((*file_hdr).c_name),
            );
            return -(1 as i32);
        }
    }
    if setstat_delayed == 0 && repair_delayed_set_stat(file_hdr) != 0 {
        set_perms(-(1 as i32), file_hdr);
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn change_dir() {
    if !change_directory_option.is_null() && chdir(change_directory_option) != 0 {
        if *__errno_location() == 2 as i32 && create_dir_flag != 0 {
            if make_path(
                change_directory_option,
                -(1 as i32) as uid_t,
                -(1 as i32) as gid_t,
                if warn_option & 0x2 as i32 as u32 != 0 {
                    dcgettext(
                        0 as *const i8,
                        b"Creating directory `%s'\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                } else {
                    0 as *mut i8
                },
            ) != 0
            {
                exit(2 as i32);
            }
            if chdir(change_directory_option) == 0 as i32 {
                return;
            }
        }
        error(
            2 as i32,
            *__errno_location(),
            dcgettext(
                0 as *const i8,
                b"cannot change to directory `%s'\0" as *const u8 as *const i8,
                5 as i32,
            ),
            change_directory_option,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn arf_stores_inode_p(mut arf: archive_format) -> i32 {
    match arf as u32 {
        5 | 6 => return 0 as i32,
        _ => {}
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn cpio_file_stat_init(mut file_hdr: *mut cpio_file_stat) {
    memset(
        file_hdr as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<cpio_file_stat>() as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cpio_file_stat_free(mut file_hdr: *mut cpio_file_stat) {
    rpl_free((*file_hdr).c_name as *mut libc::c_void);
    cpio_file_stat_init(file_hdr);
}