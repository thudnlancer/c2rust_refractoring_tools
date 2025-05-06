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
    pub type namebuf;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn openat(__fd: i32, __file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn close(__fd: i32) -> i32;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
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
    fn read_fatal_details(_: *const i8, _: off_t, _: size_t) -> !;
    fn stat_error(_: *const i8);
    fn fatal_exit() -> !;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    static mut subcommand_option: subcommand;
    static mut archive_format: archive_format;
    static mut interactive_option: bool;
    static mut current_stat_info: tar_stat_info;
    fn available_space_after(pointer: *mut block) -> size_t;
    fn close_archive();
    fn find_next_block() -> *mut block;
    fn open_archive(mode: access_mode);
    fn reset_eof();
    fn set_next_block_after(block: *mut block);
    fn dump_file(parent: *mut tar_stat_info, name: *const i8, fullname: *const i8);
    fn write_eot();
    static mut current_header: *mut block;
    static mut current_format: archive_format;
    fn decode_header(
        header: *mut block,
        stat_info: *mut tar_stat_info,
        format_pointer: *mut archive_format,
        do_user_group: i32,
    );
    fn transform_stat_info(typeflag: i32, stat_info: *mut tar_stat_info);
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn skip_member();
    fn tar_savedir(name: *const i8, must_exist: i32) -> *mut i8;
    fn namebuf_create(dir: *const i8) -> namebuf_t;
    fn namebuf_free(buf: namebuf_t);
    fn namebuf_name(buf: namebuf_t, name: *const i8) -> *mut i8;
    fn deref_stat(name: *const i8, buf: *mut stat) -> i32;
    static mut chdir_fd: i32;
    fn chdir_do(dir: i32);
    fn name_gather();
    fn addname(
        string: *const i8,
        change_dir: i32,
        cmdline: bool,
        parent: *mut name,
    ) -> *mut name;
    fn remname(name: *mut name);
    fn names_notfound();
    fn name_scan(name: *const i8) -> *mut name;
    fn name_from_list() -> *const name;
    fn confirm(message_action: *const i8, name: *const i8) -> i32;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn tar_timespec_cmp(a: timespec, b: timespec) -> i32;
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn finish_deferred_unlinks();
    fn excluded_name(name: *const i8, st: *mut tar_stat_info) -> bool;
    fn xheader_forbid_global();
    static mut current_block: *mut block;
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
pub enum subcommand {
    UNKNOWN_SUBCOMMAND,
    APPEND_SUBCOMMAND,
    CAT_SUBCOMMAND,
    CREATE_SUBCOMMAND,
    DELETE_SUBCOMMAND,
    DIFF_SUBCOMMAND,
    EXTRACT_SUBCOMMAND,
    LIST_SUBCOMMAND,
    UPDATE_SUBCOMMAND,
    TEST_LABEL_SUBCOMMAND,
}
impl subcommand {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            subcommand::UNKNOWN_SUBCOMMAND => 0,
            subcommand::APPEND_SUBCOMMAND => 1,
            subcommand::CAT_SUBCOMMAND => 2,
            subcommand::CREATE_SUBCOMMAND => 3,
            subcommand::DELETE_SUBCOMMAND => 4,
            subcommand::DIFF_SUBCOMMAND => 5,
            subcommand::EXTRACT_SUBCOMMAND => 6,
            subcommand::LIST_SUBCOMMAND => 7,
            subcommand::UPDATE_SUBCOMMAND => 8,
            subcommand::TEST_LABEL_SUBCOMMAND => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> subcommand {
        match value {
            0 => subcommand::UNKNOWN_SUBCOMMAND,
            1 => subcommand::APPEND_SUBCOMMAND,
            2 => subcommand::CAT_SUBCOMMAND,
            3 => subcommand::CREATE_SUBCOMMAND,
            4 => subcommand::DELETE_SUBCOMMAND,
            5 => subcommand::DIFF_SUBCOMMAND,
            6 => subcommand::EXTRACT_SUBCOMMAND,
            7 => subcommand::LIST_SUBCOMMAND,
            8 => subcommand::UPDATE_SUBCOMMAND,
            9 => subcommand::TEST_LABEL_SUBCOMMAND,
            _ => panic!("Invalid value for subcommand: {}", value),
        }
    }
}
impl AddAssign<u32> for subcommand {
    fn add_assign(&mut self, rhs: u32) {
        *self = subcommand::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for subcommand {
    fn sub_assign(&mut self, rhs: u32) {
        *self = subcommand::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for subcommand {
    fn mul_assign(&mut self, rhs: u32) {
        *self = subcommand::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for subcommand {
    fn div_assign(&mut self, rhs: u32) {
        *self = subcommand::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for subcommand {
    fn rem_assign(&mut self, rhs: u32) {
        *self = subcommand::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for subcommand {
    type Output = subcommand;
    fn add(self, rhs: u32) -> subcommand {
        subcommand::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for subcommand {
    type Output = subcommand;
    fn sub(self, rhs: u32) -> subcommand {
        subcommand::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for subcommand {
    type Output = subcommand;
    fn mul(self, rhs: u32) -> subcommand {
        subcommand::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for subcommand {
    type Output = subcommand;
    fn div(self, rhs: u32) -> subcommand {
        subcommand::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for subcommand {
    type Output = subcommand;
    fn rem(self, rhs: u32) -> subcommand {
        subcommand::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub type namebuf_t = *mut namebuf;
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[no_mangle]
pub static mut time_to_start_writing: bool = false;
#[no_mangle]
pub static mut output_start: *mut i8 = 0 as *const i8 as *mut i8;
unsafe extern "C" fn append_file(mut file_name: *mut i8) {
    let mut handle: i32 = openat(chdir_fd, file_name, 0 as i32 | 0 as i32);
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
    if handle < 0 as i32 {
        open_error(file_name);
        return;
    }
    if fstat(handle, &mut stat_data) != 0 as i32 {
        stat_error(file_name);
    } else {
        let mut bytes_left: off_t = stat_data.st_size;
        while bytes_left > 0 as i32 as i64 {
            let mut start: *mut block = find_next_block();
            let mut buffer_size: size_t = available_space_after(start);
            let mut status: size_t = 0;
            let mut buf: [i8; 21] = [0; 21];
            if (bytes_left as u64) < buffer_size {
                buffer_size = bytes_left as size_t;
                status = buffer_size.wrapping_rem(512 as i32 as u64);
                if status != 0 {
                    memset(
                        ((*start).buffer).as_mut_ptr().offset(bytes_left as isize)
                            as *mut libc::c_void,
                        0 as i32,
                        (512 as i32 as u64).wrapping_sub(status),
                    );
                }
            }
            status = safe_read(
                handle,
                ((*start).buffer).as_mut_ptr() as *mut libc::c_void,
                buffer_size,
            );
            if status == -(1 as i32) as size_t {
                read_fatal_details(
                    file_name,
                    stat_data.st_size - bytes_left,
                    buffer_size,
                );
            }
            if status == 0 as i32 as u64 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcngettext(
                        0 as *const i8,
                        b"%s: File shrank by %s byte\0" as *const u8 as *const i8,
                        b"%s: File shrank by %s bytes\0" as *const u8 as *const i8,
                        bytes_left as u64,
                        5 as i32,
                    ),
                    quotearg_colon(file_name),
                    umaxtostr(bytes_left as uintmax_t, buf.as_mut_ptr()),
                );
                fatal_exit();
            }
            bytes_left = (bytes_left as u64).wrapping_sub(status) as off_t as off_t;
            set_next_block_after(
                start
                    .offset(
                        status
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_div(512 as i32 as u64) as isize,
                    ),
            );
        }
    }
    if close(handle) != 0 as i32 {
        close_error(file_name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn update_archive() {
    let mut previous_status: read_header = read_header::HEADER_STILL_UNREAD;
    let mut found_end: bool = 0 as i32 != 0;
    name_gather();
    open_archive(access_mode::ACCESS_UPDATE);
    xheader_forbid_global();
    while !found_end {
        let mut status: read_header = read_header(
            &mut current_header,
            &mut current_stat_info,
            read_header_mode::read_header_auto,
        );
        match status as u32 {
            0 | 2 => {
                abort();
            }
            1 => {
                let mut name: *mut name = 0 as *mut name;
                decode_header(
                    current_header,
                    &mut current_stat_info,
                    &mut current_format,
                    0 as i32,
                );
                transform_stat_info(
                    (*current_header).header.typeflag as i32,
                    &mut current_stat_info,
                );
                archive_format = current_format;
                if subcommand_option as u32
                    == subcommand::UPDATE_SUBCOMMAND as i32 as u32
                    && {
                        name = name_scan(current_stat_info.file_name);
                        !name.is_null()
                    }
                {
                    let mut s: stat = stat {
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
                    chdir_do((*name).change_dir);
                    if deref_stat(current_stat_info.file_name, &mut s) == 0 as i32 {
                        if s.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
                            let mut p: *mut i8 = 0 as *mut i8;
                            let mut dirp: *mut i8 = tar_savedir((*name).name, 1 as i32);
                            if !dirp.is_null() {
                                let mut nbuf: namebuf_t = namebuf_create((*name).name);
                                p = dirp;
                                while *p != 0 {
                                    addname(
                                        namebuf_name(nbuf, p),
                                        (*name).change_dir,
                                        0 as i32 != 0,
                                        0 as *mut name,
                                    );
                                    p = p
                                        .offset((strlen(p)).wrapping_add(1 as i32 as u64) as isize);
                                }
                                namebuf_free(nbuf);
                                rpl_free(dirp as *mut libc::c_void);
                                remname(name);
                            }
                        } else if tar_timespec_cmp(
                            get_stat_mtime(&mut s),
                            current_stat_info.mtime,
                        ) <= 0 as i32
                        {
                            remname(name);
                        }
                    }
                }
                skip_member();
            }
            3 => {
                current_block = current_header;
                found_end = 1 as i32 != 0;
            }
            4 => {
                found_end = 1 as i32 != 0;
            }
            5 => {
                set_next_block_after(current_header);
                let mut current_block_43: u64;
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
                        current_block_43 = 17360013617472061805;
                    }
                    1 | 3 => {
                        current_block_43 = 17360013617472061805;
                    }
                    4 | 2 => {
                        abort();
                    }
                    5 | _ => {
                        current_block_43 = 15512526488502093901;
                    }
                }
                match current_block_43 {
                    17360013617472061805 => {
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
            }
            _ => {}
        }
        tar_stat_destroy(&mut current_stat_info);
        previous_status = status;
    }
    reset_eof();
    time_to_start_writing = 1 as i32 != 0;
    output_start = ((*current_block).buffer).as_mut_ptr();
    let mut p_0: *const name = 0 as *const name;
    loop {
        p_0 = name_from_list();
        if p_0.is_null() {
            break;
        }
        let mut file_name: *mut i8 = (*p_0).name;
        if excluded_name(file_name, 0 as *mut tar_stat_info) {
            continue;
        }
        if interactive_option as i32 != 0
            && confirm(b"add\0" as *const u8 as *const i8, file_name) == 0
        {
            continue;
        }
        if subcommand_option as u32 == subcommand::CAT_SUBCOMMAND as i32 as u32 {
            append_file(file_name);
        } else {
            dump_file(0 as *mut tar_stat_info, file_name, file_name);
        }
    }
    write_eot();
    close_archive();
    finish_deferred_unlinks();
    names_notfound();
}