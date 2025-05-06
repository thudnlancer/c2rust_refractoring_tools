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
    pub type __dirstream;
    pub type exclist;
    pub type directory;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn truncate_warn(_: *const i8);
    fn seek_error_details(_: *const i8, _: off_t);
    static mut exit_status: i32;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn abort() -> !;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    static mut blocking_factor: i32;
    static mut record_size: size_t;
    static mut ignore_zeros_option: bool;
    static mut occurrence_option: uintmax_t;
    static mut archive: i32;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const i8;
    static mut write_archive_to_stdout: bool;
    static mut records_written: off_t;
    fn close_archive();
    fn flush_read();
    fn flush_write();
    fn flush_archive();
    fn open_archive(mode: access_mode);
    fn set_next_block_after(block: *mut block);
    static mut current_header: *mut block;
    static mut recent_long_name_blocks: size_t;
    static mut recent_long_link_blocks: size_t;
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn skip_member();
    fn sys_truncate(fd: i32) -> i32;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn name_scan(name: *const i8) -> *mut name;
    fn xheader_decode(stat: *mut tar_stat_info);
    fn name_gather();
    fn names_notfound();
    fn rmt_lseek__(_: i32, _: off_t, _: i32) -> off_t;
    fn rmt_ioctl__(_: i32, _: i32, _: *mut i8) -> i32;
    static mut record_start: *mut block;
    static mut record_end: *mut block;
    static mut current_block: *mut block;
    static mut recent_long_name: *mut block;
    static mut recent_long_link: *mut block;
    static mut records_read: off_t;
}
pub type __uintmax_t = u64;
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
pub type off_t = __off_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name {
    pub next: *mut name,
    pub prev: *mut name,
    pub name: *mut i8,
    pub length: size_t,
    pub matching_flags: i32,
    pub cmdline: bool,
    pub change_dir: i32,
    pub found_count: uintmax_t,
    pub directory: *mut directory,
    pub parent: *mut name,
    pub child: *mut name,
    pub sibling: *mut name,
    pub caname: *mut i8,
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
static mut new_record: *mut block = 0 as *const block as *mut block;
static mut new_blocks: i32 = 0;
static mut acting_as_filter: bool = false;
#[no_mangle]
pub static mut records_skipped: off_t = 0;
unsafe extern "C" fn move_archive(mut count: off_t) {
    if count == 0 as i32 as i64 {
        return;
    }
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    if if count < 0 as i32 as i64 {
        operation.mt_op = 4 as i32 as libc::c_short;
        operation.mt_count = -count as i32;
        (operation.mt_count as i64 == -count) as i32
    } else {
        operation.mt_op = 3 as i32 as libc::c_short;
        operation.mt_count = count as i32;
        (operation.mt_count as i64 == count) as i32
    } != 0
    {
        if 0 as i32
            <= (if archive >= (1 as i32) << 30 as i32 {
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
            })
        {
            return;
        }
        if *__errno_location() == 5 as i32
            && 0 as i32
                <= (if archive >= (1 as i32) << 30 as i32 {
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
                })
        {
            return;
        }
    }
    let mut position0: off_t = if archive >= (1 as i32) << 30 as i32 {
        rmt_lseek__(archive - ((1 as i32) << 30 as i32), 0 as i32 as off_t, 1 as i32)
    } else {
        lseek(archive, 0 as i32 as off_t, 1 as i32)
    };
    let mut increment: off_t = record_size.wrapping_mul(count as u64) as off_t;
    let mut position: off_t = position0 + increment;
    if (increment / count) as u64 != record_size
        || (position < position0) as i32 != (increment < 0 as i32 as i64) as i32
        || {
            position = (if position < 0 as i32 as i64 {
                0 as i32 as i64
            } else {
                position
            });
            (if archive >= (1 as i32) << 30 as i32 {
                rmt_lseek__(archive - ((1 as i32) << 30 as i32), position, 0 as i32)
            } else {
                lseek(archive, position, 0 as i32)
            }) != position
        }
    {
        seek_error_details(*archive_name_array.offset(0 as i32 as isize), position);
    }
}
unsafe extern "C" fn write_record(mut move_back_flag: i32) {
    let mut save_record: *mut block = record_start;
    record_start = new_record;
    if acting_as_filter {
        archive = 1 as i32;
        flush_write();
        archive = 0 as i32;
    } else {
        move_archive(records_written + records_skipped - records_read);
        flush_write();
    }
    record_start = save_record;
    if move_back_flag != 0 {
        if !acting_as_filter {
            move_archive(records_read - (records_written + records_skipped));
        }
    }
    new_blocks = 0 as i32;
}
unsafe extern "C" fn write_recent_blocks(mut h: *mut block, mut blocks: size_t) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < blocks {
        let fresh0 = new_blocks;
        new_blocks = new_blocks + 1;
        *new_record.offset(fresh0 as isize) = *h.offset(i as isize);
        if new_blocks == blocking_factor {
            write_record(1 as i32);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn write_recent_bytes(mut data: *mut i8, mut bytes: size_t) {
    let mut blocks: size_t = bytes.wrapping_div(512 as i32 as u64);
    let mut rest: size_t = bytes.wrapping_sub(blocks.wrapping_mul(512 as i32 as u64));
    write_recent_blocks(data as *mut block, blocks);
    memcpy(
        ((*new_record.offset(new_blocks as isize)).buffer).as_mut_ptr()
            as *mut libc::c_void,
        data.offset(blocks.wrapping_mul(512 as i32 as u64) as isize)
            as *const libc::c_void,
        rest,
    );
    if rest < 512 as i32 as u64 {
        memset(
            ((*new_record.offset(new_blocks as isize)).buffer)
                .as_mut_ptr()
                .offset(rest as isize) as *mut libc::c_void,
            0 as i32,
            (512 as i32 as u64).wrapping_sub(rest),
        );
    }
    new_blocks += 1;
    new_blocks;
    if new_blocks == blocking_factor {
        write_record(1 as i32);
    }
}
#[inline]
unsafe extern "C" fn flush_file() {
    let mut blocks_to_skip: off_t = 0;
    set_next_block_after(current_header);
    blocks_to_skip = (current_stat_info.stat.st_size + 512 as i32 as i64
        - 1 as i32 as i64) / 512 as i32 as i64;
    while record_end.offset_from(current_block) as i64 <= blocks_to_skip {
        blocks_to_skip -= record_end.offset_from(current_block) as i64;
        flush_archive();
    }
    current_block = current_block.offset(blocks_to_skip as isize);
}
#[no_mangle]
pub unsafe extern "C" fn delete_archive_members() {
    let mut logical_status: read_header = read_header::HEADER_STILL_UNREAD;
    let mut previous_status: read_header = read_header::HEADER_STILL_UNREAD;
    let mut name: *mut name = 0 as *mut name;
    let mut blocks_to_keep: off_t = 0 as i32 as off_t;
    let mut kept_blocks_in_record: i32 = 0;
    name_gather();
    open_archive(access_mode::ACCESS_UPDATE);
    acting_as_filter = strcmp(
        *archive_name_array.offset(0 as i32 as isize),
        b"-\0" as *const u8 as *const i8,
    ) == 0 as i32;
    loop {
        let mut status: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_mode::read_header_x_raw,
        );
        let mut current_block_28: u64;
        match status as u32 {
            0 => {
                abort();
            }
            1 => {
                name = name_scan(current_stat_info.file_name);
                if name.is_null() {
                    skip_member();
                    current_block_28 = 14136749492126903395;
                } else {
                    (*name).found_count = ((*name).found_count).wrapping_add(1);
                    (*name).found_count;
                    if if occurrence_option == 0 as i32 as u64 {
                        ((*name).found_count != 0 as i32 as u64) as i32
                    } else {
                        ((*name).found_count == occurrence_option) as i32
                    } == 0
                    {
                        skip_member();
                        current_block_28 = 14136749492126903395;
                    } else {
                        current_block_28 = 2910383867557031836;
                    }
                }
            }
            2 => {
                current_block_28 = 2910383867557031836;
            }
            3 => {
                if ignore_zeros_option {
                    set_next_block_after(current_header);
                    current_block_28 = 14136749492126903395;
                } else {
                    current_block_28 = 1117614026381764558;
                }
            }
            4 => {
                current_block_28 = 1117614026381764558;
            }
            5 => {
                set_next_block_after(current_header);
                let mut current_block_27: u64;
                match previous_status as u32 {
                    0 => {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"This does not look like a tar archive\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        current_block_27 = 17272740679596634909;
                    }
                    1 | 2 | 3 => {
                        current_block_27 = 17272740679596634909;
                    }
                    4 => {
                        abort();
                    }
                    5 | _ => {
                        current_block_27 = 18377268871191777778;
                    }
                }
                match current_block_27 {
                    17272740679596634909 => {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Skipping to next header\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                        );
                        exit_status = 2 as i32;
                    }
                    _ => {}
                }
                current_block_28 = 14136749492126903395;
            }
            _ => {
                current_block_28 = 14136749492126903395;
            }
        }
        match current_block_28 {
            2910383867557031836 => {
                logical_status = status;
            }
            1117614026381764558 => {
                logical_status = read_header::HEADER_END_OF_FILE;
            }
            _ => {}
        }
        previous_status = status;
        if !(logical_status as u32 == read_header::HEADER_STILL_UNREAD as i32 as u32) {
            break;
        }
    }
    records_skipped = records_read - 1 as i32 as i64;
    new_record = xmalloc(record_size) as *mut block;
    if logical_status as u32 == read_header::HEADER_SUCCESS as i32 as u32
        || logical_status as u32 == read_header::HEADER_SUCCESS_EXTENDED as i32 as u32
    {
        write_archive_to_stdout = 0 as i32 != 0;
        new_blocks = current_block.offset_from(record_start) as i64 as i32;
        if new_blocks != 0 {
            memcpy(
                new_record as *mut libc::c_void,
                record_start as *const libc::c_void,
                (new_blocks * 512 as i32) as u64,
            );
        }
        if logical_status as u32 == read_header::HEADER_SUCCESS as i32 as u32 {
            logical_status = read_header::HEADER_STILL_UNREAD;
            flush_file();
        }
        while logical_status as u32 != read_header::HEADER_END_OF_FILE as i32 as u32 {
            let mut status_0: read_header = read_header::HEADER_STILL_UNREAD;
            if current_block == record_end {
                flush_archive();
            }
            status_0 = read_header(
                &mut current_header,
                &mut current_stat_info,
                read_header_mode::read_header_auto,
            );
            let mut current_block_97: u64;
            match status_0 as u32 {
                0 | 2 => {
                    abort();
                }
                1 => {
                    xheader_decode(&mut current_stat_info);
                    name = name_scan(current_stat_info.file_name);
                    if !name.is_null() {
                        (*name).found_count = ((*name).found_count).wrapping_add(1);
                        (*name).found_count;
                        if if occurrence_option == 0 as i32 as u64 {
                            ((*name).found_count != 0 as i32 as u64) as i32
                        } else {
                            ((*name).found_count == occurrence_option) as i32
                        } != 0
                        {
                            flush_file();
                            current_block_97 = 5687667889785024198;
                        } else {
                            current_block_97 = 8151474771948790331;
                        }
                    } else {
                        current_block_97 = 8151474771948790331;
                    }
                    match current_block_97 {
                        5687667889785024198 => {}
                        _ => {
                            if current_stat_info.xhdr.size != 0 {
                                write_recent_bytes(
                                    current_stat_info.xhdr.buffer,
                                    current_stat_info.xhdr.size,
                                );
                            } else {
                                write_recent_blocks(
                                    recent_long_name,
                                    recent_long_name_blocks,
                                );
                                write_recent_blocks(
                                    recent_long_link,
                                    recent_long_link_blocks,
                                );
                            }
                            *new_record.offset(new_blocks as isize) = *current_header;
                            new_blocks += 1;
                            new_blocks;
                            blocks_to_keep = (current_stat_info.stat.st_size
                                + 512 as i32 as i64 - 1 as i32 as i64) / 512 as i32 as i64;
                            set_next_block_after(current_header);
                            if new_blocks == blocking_factor {
                                write_record(1 as i32);
                            }
                            kept_blocks_in_record = record_end.offset_from(current_block)
                                as i64 as i32;
                            if kept_blocks_in_record as i64 > blocks_to_keep {
                                kept_blocks_in_record = blocks_to_keep as i32;
                            }
                            while blocks_to_keep != 0 {
                                let mut count: i32 = 0;
                                if current_block == record_end {
                                    flush_read();
                                    current_block = record_start;
                                    kept_blocks_in_record = blocking_factor;
                                    if kept_blocks_in_record as i64 > blocks_to_keep {
                                        kept_blocks_in_record = blocks_to_keep as i32;
                                    }
                                }
                                count = kept_blocks_in_record;
                                if blocking_factor - new_blocks < count {
                                    count = blocking_factor - new_blocks;
                                }
                                if count == 0 {
                                    abort();
                                }
                                memcpy(
                                    new_record.offset(new_blocks as isize) as *mut libc::c_void,
                                    current_block as *const libc::c_void,
                                    (count * 512 as i32) as u64,
                                );
                                new_blocks += count;
                                current_block = current_block.offset(count as isize);
                                blocks_to_keep -= count as i64;
                                kept_blocks_in_record -= count;
                                if new_blocks == blocking_factor {
                                    write_record(1 as i32);
                                }
                            }
                        }
                    }
                }
                3 => {
                    if ignore_zeros_option {
                        set_next_block_after(current_header);
                    } else {
                        logical_status = read_header::HEADER_END_OF_FILE;
                    }
                }
                4 => {
                    logical_status = read_header::HEADER_END_OF_FILE;
                }
                5 => {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Deleting non-header from archive\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    exit_status = 2 as i32;
                    set_next_block_after(current_header);
                }
                _ => {
                    abort();
                }
            }
            tar_stat_destroy(&mut current_stat_info);
        }
        if logical_status as u32 == read_header::HEADER_END_OF_FILE as i32 as u32 {
            let mut total_zero_blocks: i32 = 0 as i32;
            loop {
                let mut zero_blocks: i32 = blocking_factor - new_blocks;
                memset(
                    new_record.offset(new_blocks as isize) as *mut libc::c_void,
                    0 as i32,
                    (512 as i32 * zero_blocks) as u64,
                );
                total_zero_blocks += zero_blocks;
                write_record((total_zero_blocks < 2 as i32) as i32);
                if !(total_zero_blocks < 2 as i32) {
                    break;
                }
            }
        }
        if !acting_as_filter && !(archive >= (1 as i32) << 30 as i32) {
            if sys_truncate(archive) != 0 {
                truncate_warn(*archive_name_array.offset(0 as i32 as isize));
            }
        }
    }
    rpl_free(new_record as *mut libc::c_void);
    close_archive();
    names_notfound();
}