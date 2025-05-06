#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type __dirstream;
    pub type exclist;
    pub type directory;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn openat(__fd: i32, __file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn readlinkat(
        __fd: i32,
        __path: *const i8,
        __buf: *mut i8,
        __len: size_t,
    ) -> ssize_t;
    fn fsync(__fd: i32) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    static mut exit_status: i32;
    fn close_error(_: *const i8);
    fn open_error(_: *const i8);
    fn read_error(_: *const i8);
    fn readlink_error(_: *const i8);
    fn readlink_warn(_: *const i8);
    fn seek_error_details(_: *const i8, _: off_t);
    fn seek_warn(_: *const i8);
    fn stat_error(_: *const i8);
    fn stat_warn(_: *const i8);
    fn utime_error(_: *const i8);
    fn removed_prefixes_p() -> bool;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    static mut record_size: size_t;
    static mut atime_preserve_option: atime_preserve;
    static mut ignore_zeros_option: bool;
    static mut listed_incremental_option: *const i8;
    static mut verbose_option: i32;
    static mut archive: i32;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const i8;
    static mut open_read_flags: i32;
    static mut access_mode: access_mode;
    static mut stdlis: *mut FILE;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn current_block_ordinal() -> off_t;
    fn find_next_block() -> *mut block;
    fn flush_read();
    fn set_next_block_after(block: *mut block);
    fn mv_begin_read(st: *mut tar_stat_info);
    fn mv_end();
    fn mv_size_left(size: off_t);
    fn subfile_open(dir: *const tar_stat_info, file: *const i8, flags: i32) -> i32;
    fn quote_n_colon(n: i32, arg: *const i8) -> *const i8;
    fn blocking_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn skip_member();
    static mut chdir_fd: i32;
    static mut current_header: *mut block;
    fn off_from_header(buf: *const i8, size: size_t) -> off_t;
    fn deref_stat(name: *const i8, buf: *mut stat) -> i32;
    fn scan_directory(st: *mut tar_stat_info) -> *mut directory;
    fn directory_contents(dir: *mut directory) -> *const i8;
    fn file_removed_diag(
        name: *const i8,
        top_level: bool,
        diagfn: Option<unsafe extern "C" fn(*const i8) -> ()>,
    );
    fn stat_diag(name: *const i8);
    fn open_diag(name: *const i8);
    fn is_dumpdir(stat_info: *mut tar_stat_info) -> bool;
    fn set_file_atime(fd: i32, parentfd: i32, file: *const i8, atime: timespec) -> i32;
    fn print_header(st: *mut tar_stat_info, blk: *mut block, block_ordinal: off_t);
    fn read_directory_file();
    fn page_aligned_alloc(
        ptr: *mut *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    static mut current_format: archive_format;
    fn decode_header(
        header: *mut block,
        stat_info: *mut tar_stat_info,
        format_pointer: *mut archive_format,
        do_user_group: i32,
    );
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn clear_directory_table();
    fn tar_timespec_cmp(a: timespec, b: timespec) -> i32;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn set_exit_status(val: i32);
    fn sys_compare_links(link_data: *mut stat, stat_data: *mut stat) -> bool;
    fn sys_compare_uid(a: *mut stat, b: *mut stat) -> bool;
    fn sys_compare_gid(a: *mut stat, b: *mut stat) -> bool;
    fn sparse_diff_file(_: i32, st: *mut tar_stat_info) -> bool;
    fn transform_program_p() -> bool;
    static mut warning_option: i32;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    fn rmt_lseek__(_: i32, _: off_t, _: i32) -> off_t;
    fn rmt_ioctl__(_: i32, _: i32, _: *mut i8) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtop {
    pub mt_op: libc::c_short,
    pub mt_count: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_header {
    pub name: [i8; 100],
    pub mode: [i8; 8],
    pub uid: [i8; 8],
    pub gid: [i8; 8],
    pub size: [i8; 12],
    pub mtime: [i8; 12],
    pub chksum: [i8; 8],
    pub typeflag: i8,
    pub linkname: [i8; 100],
    pub magic: [i8; 6],
    pub version: [i8; 2],
    pub uname: [i8; 32],
    pub gname: [i8; 32],
    pub devmajor: [i8; 8],
    pub devminor: [i8; 8],
    pub prefix: [i8; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse {
    pub offset: [i8; 12],
    pub numbytes: [i8; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_header {
    pub sp: [sparse; 21],
    pub isextended: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldgnu_header {
    pub unused_pad1: [i8; 345],
    pub atime: [i8; 12],
    pub ctime: [i8; 12],
    pub offset: [i8; 12],
    pub longnames: [i8; 4],
    pub unused_pad2: i8,
    pub sp: [sparse; 4],
    pub isextended: i8,
    pub realsize: [i8; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_header {
    pub name: [i8; 100],
    pub mode: [i8; 8],
    pub uid: [i8; 8],
    pub gid: [i8; 8],
    pub size: [i8; 12],
    pub mtime: [i8; 12],
    pub chksum: [i8; 8],
    pub typeflag: i8,
    pub linkname: [i8; 100],
    pub magic: [i8; 6],
    pub version: [i8; 2],
    pub uname: [i8; 32],
    pub gname: [i8; 32],
    pub devmajor: [i8; 8],
    pub devminor: [i8; 8],
    pub prefix: [i8; 131],
    pub atime: [i8; 12],
    pub ctime: [i8; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_in_header {
    pub fill: [i8; 345],
    pub prefix: [i8; 1],
    pub fill2: i8,
    pub fill3: [i8; 8],
    pub isextended: i8,
    pub sp: [sparse; 4],
    pub realsize: [i8; 12],
    pub offset: [i8; 12],
    pub atime: [i8; 12],
    pub ctime: [i8; 12],
    pub mfill: [i8; 8],
    pub xmagic: [i8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_ext_header {
    pub sp: [sparse; 21],
    pub isextended: i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum archive_format {
    DEFAULT_FORMAT,
    V7_FORMAT,
    OLDGNU_FORMAT,
    USTAR_FORMAT,
    POSIX_FORMAT,
    STAR_FORMAT,
    GNU_FORMAT,
}
impl archive_format {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            archive_format::DEFAULT_FORMAT => 0,
            archive_format::V7_FORMAT => 1,
            archive_format::OLDGNU_FORMAT => 2,
            archive_format::USTAR_FORMAT => 3,
            archive_format::POSIX_FORMAT => 4,
            archive_format::STAR_FORMAT => 5,
            archive_format::GNU_FORMAT => 6,
        }
    }
    fn from_libc_c_uint(value: u32) -> archive_format {
        match value {
            0 => archive_format::DEFAULT_FORMAT,
            1 => archive_format::V7_FORMAT,
            2 => archive_format::OLDGNU_FORMAT,
            3 => archive_format::USTAR_FORMAT,
            4 => archive_format::POSIX_FORMAT,
            5 => archive_format::STAR_FORMAT,
            6 => archive_format::GNU_FORMAT,
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
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut i8,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut i8,
    pub xval_ptr: *mut i8,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut i8,
    pub file_name: *mut i8,
    pub had_trailing_slash: bool,
    pub link_name: *mut i8,
    pub uname: *mut i8,
    pub gname: *mut i8,
    pub cntx_name: *mut i8,
    pub acls_a_ptr: *mut i8,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut i8,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: u32,
    pub sparse_minor: u32,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut i8,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: i32,
    pub exclude_list: *mut exclist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union block {
    pub buffer: [i8; 512],
    pub header: posix_header,
    pub star_header: star_header,
    pub oldgnu_header: oldgnu_header,
    pub sparse_header: sparse_header,
    pub star_in_header: star_in_header,
    pub star_ext_header: star_ext_header,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum atime_preserve {
    no_atime_preserve,
    replace_atime_preserve,
    system_atime_preserve,
}
impl atime_preserve {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            atime_preserve::no_atime_preserve => 0,
            atime_preserve::replace_atime_preserve => 1,
            atime_preserve::system_atime_preserve => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> atime_preserve {
        match value {
            0 => atime_preserve::no_atime_preserve,
            1 => atime_preserve::replace_atime_preserve,
            2 => atime_preserve::system_atime_preserve,
            _ => panic!("Invalid value for atime_preserve: {}", value),
        }
    }
}
impl AddAssign<u32> for atime_preserve {
    fn add_assign(&mut self, rhs: u32) {
        *self = atime_preserve::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for atime_preserve {
    fn sub_assign(&mut self, rhs: u32) {
        *self = atime_preserve::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for atime_preserve {
    fn mul_assign(&mut self, rhs: u32) {
        *self = atime_preserve::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for atime_preserve {
    fn div_assign(&mut self, rhs: u32) {
        *self = atime_preserve::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for atime_preserve {
    fn rem_assign(&mut self, rhs: u32) {
        *self = atime_preserve::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for atime_preserve {
    type Output = atime_preserve;
    fn add(self, rhs: u32) -> atime_preserve {
        atime_preserve::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for atime_preserve {
    type Output = atime_preserve;
    fn sub(self, rhs: u32) -> atime_preserve {
        atime_preserve::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for atime_preserve {
    type Output = atime_preserve;
    fn mul(self, rhs: u32) -> atime_preserve {
        atime_preserve::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for atime_preserve {
    type Output = atime_preserve;
    fn div(self, rhs: u32) -> atime_preserve {
        atime_preserve::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for atime_preserve {
    type Output = atime_preserve;
    fn rem(self, rhs: u32) -> atime_preserve {
        atime_preserve::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum access_mode {
    ACCESS_READ,
    ACCESS_WRITE,
    ACCESS_UPDATE,
}
impl access_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            access_mode::ACCESS_READ => 0,
            access_mode::ACCESS_WRITE => 1,
            access_mode::ACCESS_UPDATE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> access_mode {
        match value {
            0 => access_mode::ACCESS_READ,
            1 => access_mode::ACCESS_WRITE,
            2 => access_mode::ACCESS_UPDATE,
            _ => panic!("Invalid value for access_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for access_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = access_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for access_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = access_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for access_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = access_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for access_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = access_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for access_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = access_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for access_mode {
    type Output = access_mode;
    fn add(self, rhs: u32) -> access_mode {
        access_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for access_mode {
    type Output = access_mode;
    fn sub(self, rhs: u32) -> access_mode {
        access_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for access_mode {
    type Output = access_mode;
    fn mul(self, rhs: u32) -> access_mode {
        access_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for access_mode {
    type Output = access_mode;
    fn div(self, rhs: u32) -> access_mode {
        access_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for access_mode {
    type Output = access_mode;
    fn rem(self, rhs: u32) -> access_mode {
        access_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    QUOTE_ARG,
    QUOTE_NAME,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::QUOTE_ARG => 0,
            C2RustUnnamed_2::QUOTE_NAME => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            0 => C2RustUnnamed_2::QUOTE_ARG,
            1 => C2RustUnnamed_2::QUOTE_NAME,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum read_header {
    HEADER_STILL_UNREAD,
    HEADER_SUCCESS,
    HEADER_SUCCESS_EXTENDED,
    HEADER_ZERO_BLOCK,
    HEADER_END_OF_FILE,
    HEADER_FAILURE,
}
impl read_header {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            read_header::HEADER_STILL_UNREAD => 0,
            read_header::HEADER_SUCCESS => 1,
            read_header::HEADER_SUCCESS_EXTENDED => 2,
            read_header::HEADER_ZERO_BLOCK => 3,
            read_header::HEADER_END_OF_FILE => 4,
            read_header::HEADER_FAILURE => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> read_header {
        match value {
            0 => read_header::HEADER_STILL_UNREAD,
            1 => read_header::HEADER_SUCCESS,
            2 => read_header::HEADER_SUCCESS_EXTENDED,
            3 => read_header::HEADER_ZERO_BLOCK,
            4 => read_header::HEADER_END_OF_FILE,
            5 => read_header::HEADER_FAILURE,
            _ => panic!("Invalid value for read_header: {}", value),
        }
    }
}
impl AddAssign<u32> for read_header {
    fn add_assign(&mut self, rhs: u32) {
        *self = read_header::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for read_header {
    fn sub_assign(&mut self, rhs: u32) {
        *self = read_header::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for read_header {
    fn mul_assign(&mut self, rhs: u32) {
        *self = read_header::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for read_header {
    fn div_assign(&mut self, rhs: u32) {
        *self = read_header::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for read_header {
    fn rem_assign(&mut self, rhs: u32) {
        *self = read_header::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for read_header {
    type Output = read_header;
    fn add(self, rhs: u32) -> read_header {
        read_header::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for read_header {
    type Output = read_header;
    fn sub(self, rhs: u32) -> read_header {
        read_header::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for read_header {
    type Output = read_header;
    fn mul(self, rhs: u32) -> read_header {
        read_header::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for read_header {
    type Output = read_header;
    fn div(self, rhs: u32) -> read_header {
        read_header::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for read_header {
    type Output = read_header;
    fn rem(self, rhs: u32) -> read_header {
        read_header::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum read_header_mode {
    read_header_auto,
    read_header_x_raw,
    read_header_x_global,
}
impl read_header_mode {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            read_header_mode::read_header_auto => 0,
            read_header_mode::read_header_x_raw => 1,
            read_header_mode::read_header_x_global => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> read_header_mode {
        match value {
            0 => read_header_mode::read_header_auto,
            1 => read_header_mode::read_header_x_raw,
            2 => read_header_mode::read_header_x_global,
            _ => panic!("Invalid value for read_header_mode: {}", value),
        }
    }
}
impl AddAssign<u32> for read_header_mode {
    fn add_assign(&mut self, rhs: u32) {
        *self = read_header_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for read_header_mode {
    fn sub_assign(&mut self, rhs: u32) {
        *self = read_header_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for read_header_mode {
    fn mul_assign(&mut self, rhs: u32) {
        *self = read_header_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for read_header_mode {
    fn div_assign(&mut self, rhs: u32) {
        *self = read_header_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for read_header_mode {
    fn rem_assign(&mut self, rhs: u32) {
        *self = read_header_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for read_header_mode {
    type Output = read_header_mode;
    fn add(self, rhs: u32) -> read_header_mode {
        read_header_mode::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for read_header_mode {
    type Output = read_header_mode;
    fn sub(self, rhs: u32) -> read_header_mode {
        read_header_mode::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for read_header_mode {
    type Output = read_header_mode;
    fn mul(self, rhs: u32) -> read_header_mode {
        read_header_mode::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for read_header_mode {
    type Output = read_header_mode;
    fn div(self, rhs: u32) -> read_header_mode {
        read_header_mode::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for read_header_mode {
    type Output = read_header_mode;
    fn rem(self, rhs: u32) -> read_header_mode {
        read_header_mode::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[no_mangle]
pub static mut now_verifying: bool = false;
static mut diff_handle: i32 = 0;
static mut diff_buffer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn diff_init() {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    diff_buffer = page_aligned_alloc(&mut ptr, record_size) as *mut i8;
    if !listed_incremental_option.is_null() {
        read_directory_file();
    }
}
#[no_mangle]
pub unsafe extern "C" fn report_difference(
    mut st: *mut tar_stat_info,
    mut fmt: *const i8,
    mut args: ...
) {
    if !fmt.is_null() {
        let mut ap: ::core::ffi::VaListImpl;
        fprintf(
            stdlis,
            b"%s: \0" as *const u8 as *const i8,
            quote_n_colon(C2RustUnnamed_2::QUOTE_NAME as i32, (*st).file_name),
        );
        ap = args.clone();
        vfprintf(stdlis, fmt, ap.as_va_list());
        fprintf(stdlis, b"\n\0" as *const u8 as *const i8);
    }
    set_exit_status(1 as i32);
}
unsafe extern "C" fn process_noop(mut size: size_t, mut data: *mut i8) -> i32 {
    return 1 as i32;
}
unsafe extern "C" fn process_rawdata(mut bytes: size_t, mut buffer: *mut i8) -> i32 {
    let mut status: size_t = blocking_read(
        diff_handle,
        diff_buffer as *mut libc::c_void,
        bytes,
    );
    if status != bytes {
        if status == -(1 as i32) as size_t {
            read_error(current_stat_info.file_name);
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                0 as *const i8,
            );
        } else {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcngettext(
                    0 as *const i8,
                    b"Could only read %lu of %lu byte\0" as *const u8 as *const i8,
                    b"Could only read %lu of %lu bytes\0" as *const u8 as *const i8,
                    bytes,
                    5 as i32,
                ),
                status,
                bytes,
            );
        }
        return 0 as i32;
    }
    if memcmp(buffer as *const libc::c_void, diff_buffer as *const libc::c_void, bytes)
        != 0
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Contents differ\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn read_and_process(
    mut st: *mut tar_stat_info,
    mut processor: Option<unsafe extern "C" fn(size_t, *mut i8) -> i32>,
) {
    let mut data_block: *mut block = 0 as *mut block;
    let mut data_size: size_t = 0;
    let mut size: off_t = (*st).stat.st_size;
    mv_begin_read(st);
    while size != 0 {
        data_block = find_next_block();
        if data_block.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Unexpected EOF in archive\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            exit_status = 2 as i32;
            return;
        }
        data_size = available_space_after(data_block);
        if data_size > size as u64 {
            data_size = size as size_t;
        }
        if (Some(processor.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(data_size, ((*data_block).buffer).as_mut_ptr()) == 0
        {
            processor = Some(
                process_noop as unsafe extern "C" fn(size_t, *mut i8) -> i32,
            );
        }
        set_next_block_after(
            ((*data_block).buffer)
                .as_mut_ptr()
                .offset(data_size as isize)
                .offset(-(1 as i32 as isize)) as *mut block,
        );
        size = (size as u64).wrapping_sub(data_size) as off_t as off_t;
        mv_size_left(size);
    }
    mv_end();
}
unsafe extern "C" fn get_stat_data(
    mut file_name: *const i8,
    mut stat_data: *mut stat,
) -> i32 {
    let mut status: i32 = deref_stat(file_name, stat_data);
    if status != 0 as i32 {
        if *__errno_location() == 2 as i32 {
            stat_warn(file_name);
        } else {
            stat_error(file_name);
        }
        report_difference(&mut current_stat_info as *mut tar_stat_info, 0 as *const i8);
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn diff_dir() {
    let mut stat_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32) {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"File type differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else if current_stat_info.stat.st_mode
        & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
            | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                | 0o100 as i32 >> 3 as i32 >> 3 as i32
                | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                    | 0o200 as i32 >> 3 as i32 >> 3 as i32
                    | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                        | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
        != stat_data.st_mode
            & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                    | 0o100 as i32 >> 3 as i32 >> 3 as i32
                    | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Mode differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
unsafe extern "C" fn diff_file() {
    let mut file_name: *const i8 = current_stat_info.file_name;
    let mut stat_data: stat = stat {
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
    if get_stat_data(file_name, &mut stat_data) == 0 {
        skip_member();
    } else if !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"File type differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        skip_member();
    } else {
        if current_stat_info.stat.st_mode
            & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                    | 0o100 as i32 >> 3 as i32 >> 3 as i32
                    | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
            != stat_data.st_mode
                & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                    | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                        | 0o100 as i32 >> 3 as i32 >> 3 as i32
                        | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                            | 0o200 as i32 >> 3 as i32 >> 3 as i32
                            | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const i8,
                    b"Mode differs\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if !sys_compare_uid(&mut stat_data, &mut current_stat_info.stat) {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const i8,
                    b"Uid differs\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if !sys_compare_gid(&mut stat_data, &mut current_stat_info.stat) {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const i8,
                    b"Gid differs\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if tar_timespec_cmp(get_stat_mtime(&mut stat_data), current_stat_info.mtime) != 0
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const i8,
                    b"Mod time differs\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        if (*current_header).header.typeflag as i32 != 'S' as i32
            && stat_data.st_size != current_stat_info.stat.st_size
        {
            report_difference(
                &mut current_stat_info as *mut tar_stat_info,
                dcgettext(
                    0 as *const i8,
                    b"Size differs\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            skip_member();
        } else {
            diff_handle = openat(chdir_fd, file_name, open_read_flags);
            if diff_handle < 0 as i32 {
                open_error(file_name);
                skip_member();
                report_difference(
                    &mut current_stat_info as *mut tar_stat_info,
                    0 as *const i8,
                );
            } else {
                let mut status: i32 = 0;
                if current_stat_info.is_sparse {
                    sparse_diff_file(diff_handle, &mut current_stat_info);
                } else {
                    read_and_process(
                        &mut current_stat_info,
                        Some(
                            process_rawdata
                                as unsafe extern "C" fn(size_t, *mut i8) -> i32,
                        ),
                    );
                }
                if atime_preserve_option as u32
                    == atime_preserve::replace_atime_preserve as i32 as u32
                    && stat_data.st_size != 0 as i32 as i64
                {
                    let mut atime: timespec = get_stat_atime(&mut stat_data);
                    if set_file_atime(diff_handle, chdir_fd, file_name, atime)
                        != 0 as i32
                    {
                        utime_error(file_name);
                    }
                }
                status = close(diff_handle);
                if status != 0 as i32 {
                    close_error(file_name);
                }
            }
        }
    };
}
unsafe extern "C" fn diff_link() {
    let mut file_data: stat = stat {
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
    let mut link_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut file_data) != 0
        && get_stat_data(current_stat_info.link_name, &mut link_data) != 0
        && !sys_compare_links(&mut file_data, &mut link_data)
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Not linked to %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote_n_colon(C2RustUnnamed_2::QUOTE_ARG as i32, current_stat_info.link_name),
        );
    }
}
unsafe extern "C" fn diff_symlink() {
    let mut buf: [i8; 1024] = [0; 1024];
    let mut len: size_t = strlen(current_stat_info.link_name);
    let mut linkbuf: *mut i8 = (if len < ::core::mem::size_of::<[i8; 1024]>() as u64 {
        buf.as_mut_ptr() as *mut libc::c_void
    } else {
        xmalloc(len.wrapping_add(1 as i32 as u64))
    }) as *mut i8;
    let mut status: ssize_t = readlinkat(
        chdir_fd,
        current_stat_info.file_name,
        linkbuf,
        len.wrapping_add(1 as i32 as u64),
    );
    if status < 0 as i32 as i64 {
        if *__errno_location() == 2 as i32 {
            readlink_warn(current_stat_info.file_name);
        } else {
            readlink_error(current_stat_info.file_name);
        }
        report_difference(&mut current_stat_info as *mut tar_stat_info, 0 as *const i8);
    } else if status as u64 != len
        || memcmp(
            current_stat_info.link_name as *const libc::c_void,
            linkbuf as *const libc::c_void,
            len,
        ) != 0 as i32
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Symlink differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if linkbuf != buf.as_mut_ptr() {
        rpl_free(linkbuf as *mut libc::c_void);
    }
}
unsafe extern "C" fn diff_special() {
    let mut stat_data: stat = stat {
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
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if if (*current_header).header.typeflag as i32 == '3' as i32 {
        !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32) as i32
    } else if (*current_header).header.typeflag as i32 == '4' as i32 {
        !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32) as i32
    } else {
        !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o10000 as i32 as u32) as i32
    } != 0
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"File type differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return;
    }
    if ((*current_header).header.typeflag as i32 == '3' as i32
        || (*current_header).header.typeflag as i32 == '4' as i32)
        && current_stat_info.stat.st_rdev != stat_data.st_rdev
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Device number differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return;
    }
    if current_stat_info.stat.st_mode
        & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
            | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                | 0o100 as i32 >> 3 as i32 >> 3 as i32
                | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                    | 0o200 as i32 >> 3 as i32 >> 3 as i32
                    | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                        | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
        != stat_data.st_mode
            & (0o4000 as i32 | 0o2000 as i32 | 0o1000 as i32
                | (0o100 as i32 | 0o100 as i32 >> 3 as i32
                    | 0o100 as i32 >> 3 as i32 >> 3 as i32
                    | (0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32)))) as u32
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Mode differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
}
unsafe extern "C" fn dumpdir_cmp(mut a: *const i8, mut b: *const i8) -> i32 {
    let mut len: size_t = 0;
    while *a != 0 {
        match *a as i32 {
            89 | 78 => {
                if (strchr(b"YN\0" as *const u8 as *const i8, *b as i32)).is_null() {
                    return 1 as i32;
                }
                if strcmp(a.offset(1 as i32 as isize), b.offset(1 as i32 as isize)) != 0
                {
                    return 1 as i32;
                }
                len = (strlen(a)).wrapping_add(1 as i32 as u64);
                a = a.offset(len as isize);
                b = b.offset(len as isize);
            }
            68 => {
                if strcmp(a, b) != 0 {
                    return 1 as i32;
                }
                len = (strlen(a)).wrapping_add(1 as i32 as u64);
                a = a.offset(len as isize);
                b = b.offset(len as isize);
            }
            82 | 84 | 88 => return *b as i32,
            _ => {}
        }
    }
    return *b as i32;
}
unsafe extern "C" fn diff_dumpdir(mut dir: *mut tar_stat_info) {
    let mut dumpdir_buffer: *const i8 = 0 as *const i8;
    if (*dir).fd == 0 as i32 {
        let mut diag: Option<unsafe extern "C" fn(*const i8) -> ()> = None;
        let mut fd: i32 = subfile_open(
            (*dir).parent,
            (*dir).orig_file_name,
            open_read_flags,
        );
        if fd < 0 as i32 {
            diag = Some(open_diag as unsafe extern "C" fn(*const i8) -> ());
        } else if fstat(fd, &mut (*dir).stat) != 0 {
            diag = Some(stat_diag as unsafe extern "C" fn(*const i8) -> ());
            close(fd);
        } else {
            (*dir).fd = fd;
        }
        if diag.is_some() {
            file_removed_diag((*dir).orig_file_name, 0 as i32 != 0, diag);
            return;
        }
    }
    dumpdir_buffer = directory_contents(scan_directory(dir));
    if !dumpdir_buffer.is_null() {
        if dumpdir_cmp((*dir).dumpdir, dumpdir_buffer) != 0 {
            report_difference(
                dir,
                dcgettext(
                    0 as *const i8,
                    b"Contents differ\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
    } else {
        read_and_process(
            dir,
            Some(process_noop as unsafe extern "C" fn(size_t, *mut i8) -> i32),
        );
    };
}
unsafe extern "C" fn diff_multivol() {
    let mut stat_data: stat = stat {
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
    let mut fd: i32 = 0;
    let mut status: i32 = 0;
    let mut offset: off_t = 0;
    if current_stat_info.had_trailing_slash {
        diff_dir();
        return;
    }
    if get_stat_data(current_stat_info.file_name, &mut stat_data) == 0 {
        return;
    }
    if !(stat_data.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"File type differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        skip_member();
        return;
    }
    offset = off_from_header(
        ((*current_header).oldgnu_header.offset).as_mut_ptr(),
        ::core::mem::size_of::<[i8; 12]>() as u64,
    );
    if offset < 0 as i32 as i64
        || (if (if ((if 1 as i32 != 0 {
            0 as i32 as i64
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                current_stat_info.stat.st_size
            }) + offset
        }) - 1 as i32 as i64) < 0 as i32 as i64
        {
            !(((((if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    current_stat_info.stat.st_size
                }) + offset
            }) + 1 as i32 as i64)
                << (::core::mem::size_of::<i64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as i64
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    current_stat_info.stat.st_size
                }) + offset
            }) + 0 as i32 as i64
        }) < 0 as i32 as i64
        {
            (if offset < 0 as i32 as i64 {
                (current_stat_info.stat.st_size
                    < (if ((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        !(((((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                current_stat_info.stat.st_size
                            }) + offset
                        }) + 1 as i32 as i64)
                            << (::core::mem::size_of::<i64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                            * 2 as i32 as i64 + 1 as i32 as i64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                current_stat_info.stat.st_size
                            }) + offset
                        }) + 0 as i32 as i64
                    }) - offset) as i32
            } else {
                ((if ((if 1 as i32 != 0 {
                    0 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        current_stat_info.stat.st_size
                    }) + offset
                }) - 1 as i32 as i64) < 0 as i32 as i64
                {
                    ((((if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) + 1 as i32 as i64)
                        << (::core::mem::size_of::<i64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                        * 2 as i32 as i64 + 1 as i32 as i64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as i64
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            current_stat_info.stat.st_size
                        }) + offset
                    }) - 1 as i32 as i64
                }) - offset < current_stat_info.stat.st_size) as i32
            })
        } else {
            (if current_stat_info.stat.st_size < 0 as i32 as i64 {
                (offset <= current_stat_info.stat.st_size + offset) as i32
            } else {
                (if offset < 0 as i32 as i64 {
                    (current_stat_info.stat.st_size
                        <= current_stat_info.stat.st_size + offset) as i32
                } else {
                    (current_stat_info.stat.st_size + offset < offset) as i32
                })
            })
        }) != 0 || stat_data.st_size != current_stat_info.stat.st_size + offset
    {
        report_difference(
            &mut current_stat_info as *mut tar_stat_info,
            dcgettext(
                0 as *const i8,
                b"Size differs\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        skip_member();
        return;
    }
    fd = openat(chdir_fd, current_stat_info.file_name, open_read_flags);
    if fd < 0 as i32 {
        open_error(current_stat_info.file_name);
        report_difference(&mut current_stat_info as *mut tar_stat_info, 0 as *const i8);
        skip_member();
        return;
    }
    if lseek(fd, offset, 0 as i32) < 0 as i32 as i64 {
        seek_error_details(current_stat_info.file_name, offset);
        report_difference(&mut current_stat_info as *mut tar_stat_info, 0 as *const i8);
    } else {
        read_and_process(
            &mut current_stat_info,
            Some(process_rawdata as unsafe extern "C" fn(size_t, *mut i8) -> i32),
        );
    }
    status = close(fd);
    if status != 0 as i32 {
        close_error(current_stat_info.file_name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn diff_archive() {
    set_next_block_after(current_header);
    if verbose_option != 0 {
        if now_verifying {
            fprintf(
                stdlis,
                dcgettext(
                    0 as *const i8,
                    b"Verify \0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
        }
        print_header(&mut current_stat_info, current_header, -(1 as i32) as off_t);
    }
    let mut current_block_22: u64;
    match (*current_header).header.typeflag as i32 {
        0 | 48 | 83 | 55 => {
            current_block_22 = 10529928424890701275;
        }
        49 => {
            diff_link();
            current_block_22 = 18386322304582297246;
        }
        50 => {
            diff_symlink();
            current_block_22 = 18386322304582297246;
        }
        51 | 52 | 54 => {
            diff_special();
            current_block_22 = 18386322304582297246;
        }
        68 | 53 => {
            if is_dumpdir(&mut current_stat_info) {
                diff_dumpdir(&mut current_stat_info);
            }
            diff_dir();
            current_block_22 = 18386322304582297246;
        }
        86 => {
            current_block_22 = 18386322304582297246;
        }
        77 => {
            diff_multivol();
            current_block_22 = 18386322304582297246;
        }
        _ => {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: Unknown file type '%c', diffed as normal file\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                quotearg_colon(current_stat_info.file_name),
                (*current_header).header.typeflag as i32,
            );
            exit_status = 2 as i32;
            current_block_22 = 10529928424890701275;
        }
    }
    match current_block_22 {
        10529928424890701275 => {
            if current_stat_info.had_trailing_slash {
                diff_dir();
            } else {
                diff_file();
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn verify_volume() {
    let mut may_fail: i32 = 0 as i32;
    if removed_prefixes_p() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Archive contains file names with leading prefixes removed.\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        may_fail = 1 as i32;
    }
    if transform_program_p() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Archive contains transformed file names.\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        may_fail = 1 as i32;
    }
    if may_fail != 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Verification may fail to locate original files.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    clear_directory_table();
    if diff_buffer.is_null() {
        diff_init();
    }
    fsync(archive);
    ioctl(
        archive,
        ((0 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
            | ((2 as i32) << 0 as i32 + 8 as i32) as u32
            | ((0x4b as i32) << 0 as i32) as u32
            | ((0 as i32) << 0 as i32 + 8 as i32 + 8 as i32) as u32) as u64,
    );
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    let mut status: i32 = 0;
    operation.mt_op = 2 as i32 as libc::c_short;
    operation.mt_count = 1 as i32;
    status = (if archive >= (1 as i32) << 30 as i32 {
        rmt_ioctl__(
            archive - ((1 as i32) << 30 as i32),
            (((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                | ((1 as i32) << 0 as i32) as u32) as u64
                | (::core::mem::size_of::<mtop>() as u64)
                    << 0 as i32 + 8 as i32 + 8 as i32) as i32,
            &mut operation as *mut mtop as *mut i8,
        )
    } else {
        ioctl(
            archive,
            ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                | ((1 as i32) << 0 as i32) as u32) as u64
                | (::core::mem::size_of::<mtop>() as u64)
                    << 0 as i32 + 8 as i32 + 8 as i32,
            &mut operation as *mut mtop as *mut i8,
        )
    });
    if status < 0 as i32 {
        if *__errno_location() != 5 as i32
            || {
                status = (if archive >= (1 as i32) << 30 as i32 {
                    rmt_ioctl__(
                        archive - ((1 as i32) << 30 as i32),
                        (((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                            | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                            | ((1 as i32) << 0 as i32) as u32) as u64
                            | (::core::mem::size_of::<mtop>() as u64)
                                << 0 as i32 + 8 as i32 + 8 as i32) as i32,
                        &mut operation as *mut mtop as *mut i8,
                    )
                } else {
                    ioctl(
                        archive,
                        ((1 as u32) << 0 as i32 + 8 as i32 + 8 as i32 + 14 as i32
                            | (('m' as i32) << 0 as i32 + 8 as i32) as u32
                            | ((1 as i32) << 0 as i32) as u32) as u64
                            | (::core::mem::size_of::<mtop>() as u64)
                                << 0 as i32 + 8 as i32 + 8 as i32,
                        &mut operation as *mut mtop as *mut i8,
                    )
                });
                status < 0 as i32
            }
        {
            if (if archive >= (1 as i32) << 30 as i32 {
                rmt_lseek__(
                    archive - ((1 as i32) << 30 as i32),
                    0 as i32 as off_t,
                    0 as i32,
                )
            } else {
                lseek(archive, 0 as i32 as off_t, 0 as i32)
            }) != 0 as i32 as i64
            {
                seek_warn(*archive_name_array.offset(0 as i32 as isize));
                return;
            }
        }
    }
    access_mode = access_mode::ACCESS_READ;
    now_verifying = 1 as i32 != 0;
    flush_read();
    loop {
        let mut status_0: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_mode::read_header_auto,
        );
        if status_0 as u32 == read_header::HEADER_FAILURE as i32 as u32 {
            let mut counter: i32 = 0 as i32;
            loop {
                counter += 1;
                counter;
                set_next_block_after(current_header);
                status_0 = read_header(
                    &mut current_header,
                    &mut current_stat_info,
                    read_header_mode::read_header_auto,
                );
                if !(status_0 as u32 == read_header::HEADER_FAILURE as i32 as u32) {
                    break;
                }
            }
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcngettext(
                    0 as *const i8,
                    b"VERIFY FAILURE: %d invalid header detected\0" as *const u8
                        as *const i8,
                    b"VERIFY FAILURE: %d invalid headers detected\0" as *const u8
                        as *const i8,
                    counter as u64,
                    5 as i32,
                ),
                counter,
            );
            exit_status = 2 as i32;
        }
        if status_0 as u32 == read_header::HEADER_END_OF_FILE as i32 as u32 {
            break;
        }
        if status_0 as u32 == read_header::HEADER_ZERO_BLOCK as i32 as u32 {
            set_next_block_after(current_header);
            if ignore_zeros_option {
                continue;
            }
            let mut buf: [i8; 21] = [0; 21];
            status_0 = read_header(
                &mut current_header,
                &mut current_stat_info,
                read_header_mode::read_header_auto,
            );
            if status_0 as u32 == read_header::HEADER_ZERO_BLOCK as i32 as u32 {
                break;
            }
            if warning_option & 0x1 as i32 != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"A lone zero block at %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    umaxtostr(current_block_ordinal() as uintmax_t, buf.as_mut_ptr()),
                );
            }
        } else {
            decode_header(
                current_header,
                &mut current_stat_info,
                &mut current_format,
                1 as i32,
            );
            diff_archive();
            tar_stat_destroy(&mut current_stat_info);
        }
    }
    access_mode = access_mode::ACCESS_WRITE;
    now_verifying = 0 as i32 != 0;
}