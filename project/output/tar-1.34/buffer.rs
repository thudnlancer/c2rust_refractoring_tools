use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    static mut stdin: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush_unlocked(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn fscanf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn creat(__file: *const i8, __mode: mode_t) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn getuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn isatty(__fd: i32) -> i32;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __overflow(_: *mut _IO_FILE, _: i32) -> i32;
    fn strip_trailing_slashes(file: *mut i8) -> bool;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
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
    fn umaxtostr(_: uintmax_t, _: *mut i8) -> *mut i8;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    static mut exit_status: i32;
    fn close_error(_: *const i8);
    fn open_error(_: *const i8);
    fn open_fatal(_: *const i8) -> !;
    fn open_warn(_: *const i8);
    fn read_error(_: *const i8);
    fn write_error(_: *const i8);
    fn write_error_details(_: *const i8, _: size_t, _: size_t);
    fn fatal_exit() -> !;
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
    fn fnmatch(__pattern: *const i8, __name: *const i8, __flags: i32) -> i32;
    fn human_readable(
        _: uintmax_t,
        _: *mut i8,
        _: i32,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut i8;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    static mut subcommand_option: subcommand;
    static mut archive_format: archive_format;
    static mut blocking_factor: i32;
    static mut record_size: size_t;
    static mut backup_option: bool;
    static mut use_compress_program_option: *const i8;
    static mut info_script_option: *const i8;
    static mut multi_volume_option: bool;
    static mut read_full_records_option: bool;
    static mut rsh_command_option: *const i8;
    static mut tape_length_option: tarlong;
    static mut totals_option: bool;
    static mut restrict_option: bool;
    static mut verbose_option: i32;
    static mut verify_option: bool;
    static mut volno_file_option: *const i8;
    static mut volume_label_option: *const i8;
    static mut archive: i32;
    static mut dev_null_output: bool;
    static mut start_time: timespec;
    static mut volume_start_time: timespec;
    static mut last_stat_time: timespec;
    static mut current_stat_info: tar_stat_info;
    static mut archive_name_array: *mut *const i8;
    static mut archive_names: size_t;
    static mut archive_name_cursor: *mut *const i8;
    static mut index_file_name: *const i8;
    static mut seek_option: i32;
    static mut seekable_archive: bool;
    fn start_header(st: *mut tar_stat_info) -> *mut block;
    fn finish_header(st: *mut tar_stat_info, header: *mut block, block_ordinal: off_t);
    fn simple_finish_header(header: *mut block);
    fn write_extended(
        global: bool,
        st: *mut tar_stat_info,
        old_header: *mut block,
    ) -> *mut block;
    fn off_to_chars(off: off_t, buf: *mut i8, size: size_t) -> bool;
    fn time_to_chars(t: time_t, buf: *mut i8, size: size_t) -> bool;
    static mut now_verifying: bool;
    fn verify_volume();
    static mut current_header: *mut block;
    fn off_from_header(buf: *const i8, size: size_t) -> off_t;
    fn uintmax_from_header(buf: *const i8, size: size_t) -> uintmax_t;
    fn read_header(
        return_block: *mut *mut block,
        info: *mut tar_stat_info,
        m: read_header_mode,
    ) -> read_header;
    fn tar_checksum(header: *mut block, silent: bool) -> read_header;
    fn assign_string(dest: *mut *mut i8, src: *const i8);
    fn assign_string_n(string: *mut *mut i8, value: *const i8, n: size_t);
    fn sys_detect_dev_null_output();
    fn undo_last_backup();
    fn sys_get_archive_stat() -> bool;
    fn maybe_backup_file(file_name: *const i8, this_is_the_archive: bool) -> bool;
    fn sys_save_archive_dev_ino();
    fn sys_child_open_for_uncompress() -> pid_t;
    fn set_compression_program_by_suffix(name: *const i8, defprog: *const i8);
    fn tar_stat_init(st: *mut tar_stat_info);
    fn xheader_read(xhdr: *mut xheader, header: *mut block, size: off_t);
    fn xheader_decode(stat: *mut tar_stat_info);
    fn xheader_store(
        keyword: *const i8,
        st: *mut tar_stat_info,
        data: *const libc::c_void,
    );
    fn page_aligned_alloc(
        ptr: *mut *mut libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn xheader_format_name(st: *mut tar_stat_info, fmt: *const i8, n: size_t) -> *mut i8;
    static mut output_start: *mut i8;
    fn sys_spawn_shell();
    fn sys_exec_info_script(archive_name: *mut *const i8, volume_number: i32) -> i32;
    fn sys_write_archive_buffer() -> size_t;
    fn checkpoint_run(do_write: bool);
    fn stat_diag(name: *const i8);
    static mut warning_option: i32;
    fn sys_wait_for_child(_: pid_t, _: bool);
    fn sys_child_open_for_compress() -> pid_t;
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn xheader_write_global(xhdr: *mut xheader);
    fn gettime(_: *mut timespec);
    fn safe_read(fd: i32, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn quote(arg: *const i8) -> *const i8;
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    static mut rmt_dev_name__: *const i8;
    fn rmt_open__(_: *const i8, _: i32, _: i32, _: *const i8) -> i32;
    fn rmt_close__(_: i32) -> i32;
    fn rmt_read__(_: i32, _: *mut i8, _: size_t) -> size_t;
    fn rmt_lseek__(_: i32, _: off_t, _: i32) -> off_t;
    fn rmt_ioctl__(_: i32, _: i32, _: *mut i8) -> i32;
    static mut force_local_option: bool;
    static mut records_skipped: off_t;
    static mut time_to_start_writing: bool;
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
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    human_ceiling = 0,
    human_round_to_nearest = 1,
    human_floor = 2,
    human_group_digits = 4,
    human_suppress_point_zero = 8,
    human_autoscale = 16,
    human_base_1024 = 32,
    human_space_before_unit = 64,
    human_SI = 128,
    human_B = 256,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::human_ceiling => 0,
            C2RustUnnamed_3::human_round_to_nearest => 1,
            C2RustUnnamed_3::human_floor => 2,
            C2RustUnnamed_3::human_group_digits => 4,
            C2RustUnnamed_3::human_suppress_point_zero => 8,
            C2RustUnnamed_3::human_autoscale => 16,
            C2RustUnnamed_3::human_base_1024 => 32,
            C2RustUnnamed_3::human_space_before_unit => 64,
            C2RustUnnamed_3::human_SI => 128,
            C2RustUnnamed_3::human_B => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            0 => C2RustUnnamed_3::human_ceiling,
            1 => C2RustUnnamed_3::human_round_to_nearest,
            2 => C2RustUnnamed_3::human_floor,
            4 => C2RustUnnamed_3::human_group_digits,
            8 => C2RustUnnamed_3::human_suppress_point_zero,
            16 => C2RustUnnamed_3::human_autoscale,
            32 => C2RustUnnamed_3::human_base_1024,
            64 => C2RustUnnamed_3::human_space_before_unit,
            128 => C2RustUnnamed_3::human_SI,
            256 => C2RustUnnamed_3::human_B,
            _ => panic!("Invalid value for C2RustUnnamed_3: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_3 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_3 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_3 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_3 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_3 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn add(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn sub(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn mul(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn div(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_3 {
    type Output = C2RustUnnamed_3;
    fn rem(self, rhs: u32) -> C2RustUnnamed_3 {
        C2RustUnnamed_3::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub type tarlong = libc::c_double;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufmap {
    pub next: *mut bufmap,
    pub start: size_t,
    pub file_name: *mut i8,
    pub sizetotal: off_t,
    pub sizeleft: off_t,
    pub nblocks: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum compress_type {
    ct_none,
    ct_tar,
    ct_compress,
    ct_gzip,
    ct_bzip2,
    ct_lzip,
    ct_lzma,
    ct_lzop,
    ct_xz,
    ct_zstd,
}
impl compress_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            compress_type::ct_none => 0,
            compress_type::ct_tar => 1,
            compress_type::ct_compress => 2,
            compress_type::ct_gzip => 3,
            compress_type::ct_bzip2 => 4,
            compress_type::ct_lzip => 5,
            compress_type::ct_lzma => 6,
            compress_type::ct_lzop => 7,
            compress_type::ct_xz => 8,
            compress_type::ct_zstd => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> compress_type {
        match value {
            0 => compress_type::ct_none,
            1 => compress_type::ct_tar,
            2 => compress_type::ct_compress,
            3 => compress_type::ct_gzip,
            4 => compress_type::ct_bzip2,
            5 => compress_type::ct_lzip,
            6 => compress_type::ct_lzma,
            7 => compress_type::ct_lzop,
            8 => compress_type::ct_xz,
            9 => compress_type::ct_zstd,
            _ => panic!("Invalid value for compress_type: {}", value),
        }
    }
}
impl AddAssign<u32> for compress_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = compress_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for compress_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = compress_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for compress_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = compress_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for compress_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = compress_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for compress_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = compress_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for compress_type {
    type Output = compress_type;
    fn add(self, rhs: u32) -> compress_type {
        compress_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for compress_type {
    type Output = compress_type;
    fn sub(self, rhs: u32) -> compress_type {
        compress_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for compress_type {
    type Output = compress_type;
    fn mul(self, rhs: u32) -> compress_type {
        compress_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for compress_type {
    type Output = compress_type;
    fn div(self, rhs: u32) -> compress_type {
        compress_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for compress_type {
    type Output = compress_type;
    fn rem(self, rhs: u32) -> compress_type {
        compress_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_magic {
    pub type_0: compress_type,
    pub length: size_t,
    pub magic: *const i8,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zip_program {
    pub type_0: compress_type,
    pub program: *const i8,
    pub option: *const i8,
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
unsafe extern "C" fn fputc_unlocked(mut __c: i32, mut __stream: *mut FILE) -> i32 {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as i32 as i64 != 0
    {
        __overflow(__stream, __c as u8 as i32)
    } else {
        let fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = ((*__stream)._IO_write_ptr).offset(1);
        *fresh0 = __c as i8;
        *fresh0 as u8 as i32
    };
}
#[inline]
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut i8,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as i32, __stream);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x20 as i32 != 0 as i32) as i32;
}
static mut prev_written: tarlong = 0.;
static mut bytes_written: tarlong = 0.;
static mut record_buffer: [*mut libc::c_void; 2] = [0 as *const libc::c_void
    as *mut libc::c_void; 2];
static mut record_buffer_aligned: [*mut block; 2] = [0 as *const block as *mut block; 2];
static mut record_index: i32 = 0;
#[no_mangle]
pub static mut record_start: *mut block = 0 as *const block as *mut block;
#[no_mangle]
pub static mut record_end: *mut block = 0 as *const block as *mut block;
#[no_mangle]
pub static mut current_block: *mut block = 0 as *const block as *mut block;
#[no_mangle]
pub static mut access_mode: access_mode = access_mode::ACCESS_READ;
#[no_mangle]
pub static mut records_read: off_t = 0;
#[no_mangle]
pub static mut records_written: off_t = 0;
static mut record_start_block: off_t = 0;
#[no_mangle]
pub static mut stdlis: *mut FILE = 0 as *const FILE as *mut FILE;
static mut child_pid: pid_t = 0;
static mut read_error_count: i32 = 0;
static mut hit_eof: bool = false;
static mut read_full_records: bool = 0 as i32 != 0;
#[no_mangle]
pub static mut write_archive_to_stdout: bool = false;
static mut flush_write_ptr: Option<unsafe extern "C" fn(size_t) -> ()> = None;
static mut flush_read_ptr: Option<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut volume_label: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut continued_file_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut continued_file_size: uintmax_t = 0;
#[no_mangle]
pub static mut continued_file_offset: uintmax_t = 0;
static mut volno: i32 = 1 as i32;
static mut global_volno: i32 = 1 as i32;
static mut bufmap_head: *mut bufmap = 0 as *const bufmap as *mut bufmap;
static mut bufmap_tail: *mut bufmap = 0 as *const bufmap as *mut bufmap;
static mut inhibit_map: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn mv_begin_write(
    mut file_name: *const i8,
    mut totsize: off_t,
    mut sizeleft: off_t,
) {
    if multi_volume_option {
        let mut bp: *mut bufmap = xmalloc(::core::mem::size_of::<bufmap>() as u64)
            as *mut bufmap;
        if !bufmap_tail.is_null() {
            (*bufmap_tail).next = bp;
        } else {
            bufmap_head = bp;
        }
        bufmap_tail = bp;
        (*bp).next = 0 as *mut bufmap;
        (*bp).start = current_block.offset_from(record_start) as i64 as size_t;
        (*bp).file_name = xstrdup(file_name);
        (*bp).sizetotal = totsize;
        (*bp).sizeleft = sizeleft;
        (*bp).nblocks = 0 as i32 as size_t;
    }
}
unsafe extern "C" fn bufmap_locate(mut off: size_t) -> *mut bufmap {
    let mut map: *mut bufmap = 0 as *mut bufmap;
    map = bufmap_head;
    while !map.is_null() {
        if ((*map).next).is_null()
            || off < ((*(*map).next).start).wrapping_mul(512 as i32 as u64)
        {
            break;
        }
        map = (*map).next;
    }
    return map;
}
unsafe extern "C" fn bufmap_free(mut mark: *mut bufmap) {
    let mut map: *mut bufmap = 0 as *mut bufmap;
    map = bufmap_head;
    while !map.is_null() && map != mark {
        let mut next: *mut bufmap = (*map).next;
        rpl_free((*map).file_name as *mut libc::c_void);
        rpl_free(map as *mut libc::c_void);
        map = next;
    }
    bufmap_head = map;
    if bufmap_head.is_null() {
        bufmap_tail = bufmap_head;
    }
}
unsafe extern "C" fn bufmap_reset(mut map: *mut bufmap, mut fixup: ssize_t) {
    bufmap_free(map);
    if !map.is_null() {
        while !map.is_null() {
            (*map).start = ((*map).start as u64).wrapping_add(fixup as u64) as size_t
                as size_t;
            (*map).nblocks = 0 as i32 as size_t;
            map = (*map).next;
        }
    }
}
static mut dummy: tar_stat_info = tar_stat_info {
    orig_file_name: 0 as *const i8 as *mut i8,
    file_name: 0 as *const i8 as *mut i8,
    had_trailing_slash: false,
    link_name: 0 as *const i8 as *mut i8,
    uname: 0 as *const i8 as *mut i8,
    gname: 0 as *const i8 as *mut i8,
    cntx_name: 0 as *const i8 as *mut i8,
    acls_a_ptr: 0 as *const i8 as *mut i8,
    acls_a_len: 0,
    acls_d_ptr: 0 as *const i8 as *mut i8,
    acls_d_len: 0,
    stat: stat {
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
    },
    atime: timespec { tv_sec: 0, tv_nsec: 0 },
    mtime: timespec { tv_sec: 0, tv_nsec: 0 },
    ctime: timespec { tv_sec: 0, tv_nsec: 0 },
    archive_file_size: 0,
    is_sparse: false,
    sparse_major: 0,
    sparse_minor: 0,
    sparse_map_avail: 0,
    sparse_map_size: 0,
    sparse_map: 0 as *const sp_array as *mut sp_array,
    real_size: 0,
    real_size_set: false,
    sparse_name_done: false,
    xattr_map_size: 0,
    xattr_map: 0 as *const xattr_array as *mut xattr_array,
    xhdr: xheader {
        stk: 0 as *const obstack as *mut obstack,
        size: 0,
        buffer: 0 as *const i8 as *mut i8,
        string_length: 0,
    },
    is_dumpdir: false,
    skipped: false,
    dumpdir: 0 as *const i8 as *mut i8,
    parent: 0 as *const tar_stat_info as *mut tar_stat_info,
    dirstream: 0 as *const DIR as *mut DIR,
    fd: 0,
    exclude_list: 0 as *const exclist as *mut exclist,
};
#[no_mangle]
pub unsafe extern "C" fn buffer_write_global_xheader() {
    xheader_write_global(&mut dummy.xhdr);
}
#[no_mangle]
pub unsafe extern "C" fn mv_begin_read(mut st: *mut tar_stat_info) {
    mv_begin_write((*st).orig_file_name, (*st).stat.st_size, (*st).stat.st_size);
}
#[no_mangle]
pub unsafe extern "C" fn mv_end() {
    if multi_volume_option {
        bufmap_free(0 as *mut bufmap);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mv_size_left(mut size: off_t) {
    if !bufmap_head.is_null() {
        (*bufmap_head).sizeleft = size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clear_read_error_count() {
    read_error_count = 0 as i32;
}
static mut duration: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn set_start_time() {
    gettime(&mut start_time);
    volume_start_time = start_time;
    last_stat_time = start_time;
}
unsafe extern "C" fn set_volume_start_time() {
    gettime(&mut volume_start_time);
    last_stat_time = volume_start_time;
}
#[no_mangle]
pub unsafe extern "C" fn compute_duration() -> libc::c_double {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut now);
    duration
        += (now.tv_sec - last_stat_time.tv_sec) as libc::c_double
            + (now.tv_nsec - last_stat_time.tv_nsec) as libc::c_double / 1e9f64;
    gettime(&mut last_stat_time);
    return duration;
}
static mut archive_compression_type: compress_type = compress_type::ct_none;
static mut magic: [zip_magic; 10] = [
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_none,
            length: 0 as i32 as size_t,
            magic: 0 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_tar,
            length: 0 as i32 as size_t,
            magic: 0 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_compress,
            length: 2 as i32 as size_t,
            magic: b"\x1F\x9D\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_gzip,
            length: 2 as i32 as size_t,
            magic: b"\x1F\x8B\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_bzip2,
            length: 3 as i32 as size_t,
            magic: b"BZh\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_lzip,
            length: 4 as i32 as size_t,
            magic: b"LZIP\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_lzma,
            length: 6 as i32 as size_t,
            magic: b"\xFFLZMA\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_lzop,
            length: 4 as i32 as size_t,
            magic: b"\x89LZO\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_xz,
            length: 6 as i32 as size_t,
            magic: b"\xFD7zXZ\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_magic {
            type_0: compress_type::ct_zstd,
            length: 4 as i32 as size_t,
            magic: b"(\xB5/\xFD\0" as *const u8 as *const i8,
        };
        init
    },
];
static mut zip_program: [zip_program; 12] = [
    {
        let mut init = zip_program {
            type_0: compress_type::ct_compress,
            program: b"compress\0" as *const u8 as *const i8,
            option: b"-Z\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_compress,
            program: b"gzip\0" as *const u8 as *const i8,
            option: b"-z\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_gzip,
            program: b"gzip\0" as *const u8 as *const i8,
            option: b"-z\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_bzip2,
            program: b"bzip2\0" as *const u8 as *const i8,
            option: b"-j\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_bzip2,
            program: b"lbzip2\0" as *const u8 as *const i8,
            option: b"-j\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_lzip,
            program: b"lzip\0" as *const u8 as *const i8,
            option: b"--lzip\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_lzma,
            program: b"lzma\0" as *const u8 as *const i8,
            option: b"--lzma\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_lzma,
            program: b"xz\0" as *const u8 as *const i8,
            option: b"-J\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_lzop,
            program: b"lzop\0" as *const u8 as *const i8,
            option: b"--lzop\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_xz,
            program: b"xz\0" as *const u8 as *const i8,
            option: b"-J\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_zstd,
            program: b"zstd\0" as *const u8 as *const i8,
            option: b"--zstd\0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = zip_program {
            type_0: compress_type::ct_none,
            program: 0 as *const i8,
            option: 0 as *const i8,
        };
        init
    },
];
unsafe extern "C" fn find_zip_program(
    mut type_0: compress_type,
    mut pstate: *mut i32,
) -> *const zip_program {
    let mut i: i32 = 0;
    i = *pstate;
    while zip_program[i as usize].type_0 as u32 != compress_type::ct_none as i32 as u32 {
        if zip_program[i as usize].type_0 as u32 == type_0 as u32 {
            *pstate = i + 1 as i32;
            return zip_program.as_mut_ptr().offset(i as isize);
        }
        i += 1;
        i;
    }
    *pstate = i;
    return 0 as *const zip_program;
}
#[no_mangle]
pub unsafe extern "C" fn first_decompress_program(mut pstate: *mut i32) -> *const i8 {
    let mut zp: *const zip_program = 0 as *const zip_program;
    if !use_compress_program_option.is_null() {
        return use_compress_program_option;
    }
    if archive_compression_type as u32 == compress_type::ct_none as i32 as u32 {
        return 0 as *const i8;
    }
    *pstate = 0 as i32;
    zp = find_zip_program(archive_compression_type, pstate);
    return if !zp.is_null() { (*zp).program } else { 0 as *const i8 };
}
#[no_mangle]
pub unsafe extern "C" fn next_decompress_program(mut pstate: *mut i32) -> *const i8 {
    let mut zp: *const zip_program = 0 as *const zip_program;
    if !use_compress_program_option.is_null() {
        return 0 as *const i8;
    }
    zp = find_zip_program(archive_compression_type, pstate);
    return if !zp.is_null() { (*zp).program } else { 0 as *const i8 };
}
unsafe extern "C" fn compress_option(mut type_0: compress_type) -> *const i8 {
    let mut zp: *const zip_program = 0 as *const zip_program;
    let mut i: i32 = 0 as i32;
    zp = find_zip_program(type_0, &mut i);
    return if !zp.is_null() { (*zp).option } else { 0 as *const i8 };
}
unsafe extern "C" fn check_compressed_archive(mut pshort: *mut bool) -> compress_type {
    let mut p: *const zip_magic = 0 as *const zip_magic;
    let mut sfr: bool = false;
    let mut temp: bool = false;
    if pshort.is_null() {
        pshort = &mut temp;
    }
    record_end = record_start;
    sfr = read_full_records;
    read_full_records = 1 as i32 != 0;
    *pshort = (find_next_block()).is_null();
    read_full_records = sfr;
    if record_start != record_end
        && (strcmp(
            ((*record_start).header.magic).as_mut_ptr(),
            b"ustar\0" as *const u8 as *const i8,
        ) == 0 as i32
            || strcmp(
                ((*record_start).buffer).as_mut_ptr().offset(257 as u64 as isize),
                b"ustar  \0" as *const u8 as *const i8,
            ) == 0 as i32)
        && tar_checksum(record_start, 1 as i32 != 0) as u32
            == read_header::HEADER_SUCCESS as i32 as u32
    {
        return compress_type::ct_tar;
    }
    p = magic.as_ptr().offset(2 as i32 as isize);
    while p
        < magic
            .as_ptr()
            .offset(
                (::core::mem::size_of::<[zip_magic; 10]>() as u64)
                    .wrapping_div(::core::mem::size_of::<zip_magic>() as u64) as isize,
            )
    {
        if memcmp(
            ((*record_start).buffer).as_mut_ptr() as *const libc::c_void,
            (*p).magic as *const libc::c_void,
            (*p).length,
        ) == 0 as i32
        {
            return (*p).type_0;
        }
        p = p.offset(1);
        p;
    }
    return compress_type::ct_none;
}
unsafe extern "C" fn guess_seekable_archive() {
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
    if subcommand_option as u32 == subcommand::DELETE_SUBCOMMAND as i32 as u32 {
        seekable_archive = 0 as i32 != 0;
    }
    if seek_option != -(1 as i32) {
        seekable_archive = seek_option != 0;
        return;
    }
    if !multi_volume_option && use_compress_program_option.is_null()
        && fstat(archive, &mut st) == 0 as i32
    {
        seekable_archive = st.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32;
    } else {
        seekable_archive = 0 as i32 != 0;
    };
}
unsafe extern "C" fn open_compressed_archive() -> i32 {
    archive = if !force_local_option
        && {
            rmt_dev_name__ = strchr(
                *archive_name_array.offset(0 as i32 as isize),
                ':' as i32,
            );
            !rmt_dev_name__.is_null()
        } && rmt_dev_name__ > *archive_name_array.offset(0 as i32 as isize)
        && (memchr(
            *archive_name_array.offset(0 as i32 as isize) as *const libc::c_void,
            '/' as i32,
            rmt_dev_name__.offset_from(*archive_name_array.offset(0 as i32 as isize))
                as i64 as u64,
        ))
            .is_null()
    {
        rmt_open__(
            *archive_name_array.offset(0 as i32 as isize),
            0 as i32 | 0 as i32,
            (1 as i32) << 30 as i32,
            rsh_command_option,
        )
    } else {
        open(
            *archive_name_array.offset(0 as i32 as isize),
            0 as i32 | 0 as i32,
            0o200 as i32 | 0o200 as i32 >> 3 as i32
                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                    | 0o400 as i32 >> 3 as i32 >> 3 as i32),
        )
    };
    if archive == -(1 as i32) {
        return archive;
    }
    if !multi_volume_option {
        if use_compress_program_option.is_null() {
            let mut shortfile: bool = false;
            let mut type_0: compress_type = check_compressed_archive(&mut shortfile);
            match type_0 as u32 {
                1 => {
                    if shortfile {
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
                        exit_status = 2 as i32;
                    }
                    return archive;
                }
                0 => {
                    if shortfile {
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
                        exit_status = 2 as i32;
                    }
                    set_compression_program_by_suffix(
                        *archive_name_array.offset(0 as i32 as isize),
                        0 as *const i8,
                    );
                    if use_compress_program_option.is_null() {
                        return archive;
                    }
                }
                _ => {
                    archive_compression_type = type_0;
                }
            }
        }
        if archive >= (1 as i32) << 30 as i32 {
            rmt_close__(archive - ((1 as i32) << 30 as i32));
        } else {
            close(archive);
        };
        hit_eof = 0 as i32 != 0;
        child_pid = sys_child_open_for_uncompress();
        read_full_records = 1 as i32 != 0;
    }
    records_read = 0 as i32 as off_t;
    record_end = record_start;
    return archive;
}
unsafe extern "C" fn print_stats(
    mut fp: *mut FILE,
    mut text: *const i8,
    mut numbytes: tarlong,
) -> i32 {
    let mut abbr: [i8; 652] = [0; 652];
    let mut rate: [i8; 652] = [0; 652];
    let mut n: i32 = 0 as i32;
    let mut human_opts: i32 = C2RustUnnamed_3::human_autoscale as i32
        | C2RustUnnamed_3::human_base_1024 as i32 | C2RustUnnamed_3::human_SI as i32
        | C2RustUnnamed_3::human_B as i32;
    if !text.is_null() && *text.offset(0 as i32 as isize) as i32 != 0 {
        n
            += fprintf(
                fp,
                b"%s: \0" as *const u8 as *const i8,
                dcgettext(0 as *const i8, text, 5 as i32),
            );
    }
    return n
        + fprintf(
            fp,
            b"%.0f (%s, %s/s)\0" as *const u8 as *const i8,
            numbytes,
            human_readable(
                numbytes as uintmax_t,
                abbr.as_mut_ptr(),
                human_opts,
                1 as i32 as uintmax_t,
                1 as i32 as uintmax_t,
            ),
            (if (0 as i32 as libc::c_double) < duration
                && numbytes / duration < -(1 as i32) as uintmax_t as libc::c_double
            {
                human_readable(
                    (numbytes / duration) as uintmax_t,
                    rate.as_mut_ptr(),
                    human_opts,
                    1 as i32 as uintmax_t,
                    1 as i32 as uintmax_t,
                )
            } else {
                b"?\0" as *const u8 as *const i8
            }),
        );
}
#[no_mangle]
pub unsafe extern "C" fn format_total_stats(
    mut fp: *mut FILE,
    mut formats: *const *const i8,
    mut eor: i32,
    mut eol: i32,
) -> i32 {
    let mut n: i32 = 0;
    match subcommand_option as u32 {
        3 | 2 | 8 | 1 => {
            n = print_stats(
                fp,
                *formats.offset(1 as i32 as isize),
                prev_written + bytes_written,
            );
        }
        4 => {
            let mut buf: [i8; 21] = [0; 21];
            n = print_stats(
                fp,
                *formats.offset(0 as i32 as isize),
                (records_read as u64).wrapping_mul(record_size) as tarlong,
            );
            fputc_unlocked(eor, fp);
            n += 1;
            n;
            n
                += print_stats(
                    fp,
                    *formats.offset(1 as i32 as isize),
                    prev_written + bytes_written,
                );
            fputc_unlocked(eor, fp);
            n += 1;
            n;
            if !(*formats.offset(2 as i32 as isize)).is_null()
                && *(*formats.offset(2 as i32 as isize)).offset(0 as i32 as isize) as i32
                    != 0
            {
                n
                    += fprintf(
                        fp,
                        b"%s: \0" as *const u8 as *const i8,
                        dcgettext(
                            0 as *const i8,
                            *formats.offset(2 as i32 as isize),
                            5 as i32,
                        ),
                    );
            }
            n
                += fprintf(
                    fp,
                    b"%s\0" as *const u8 as *const i8,
                    umaxtostr(
                        (((records_read - records_skipped) as u64)
                            .wrapping_mul(record_size) as libc::c_double
                            - (prev_written + bytes_written)) as uintmax_t,
                        buf.as_mut_ptr(),
                    ),
                );
        }
        6 | 7 | 5 => {
            n = print_stats(
                fp,
                dcgettext(0 as *const i8, *formats.offset(0 as i32 as isize), 5 as i32),
                (records_read as u64).wrapping_mul(record_size) as tarlong,
            );
        }
        _ => {
            abort();
        }
    }
    if eol != 0 {
        fputc_unlocked(eol, fp);
        n += 1;
        n;
    }
    return n;
}
static mut default_total_format: [*const i8; 3] = [
    b"Total bytes read\0" as *const u8 as *const i8,
    b"Total bytes written\0" as *const u8 as *const i8,
    b"Total bytes deleted\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn print_total_stats() {
    format_total_stats(stderr, default_total_format.as_ptr(), '\n' as i32, '\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn current_block_ordinal() -> off_t {
    return record_start_block + current_block.offset_from(record_start) as i64;
}
#[no_mangle]
pub unsafe extern "C" fn reset_eof() {
    if hit_eof {
        hit_eof = 0 as i32 != 0;
        current_block = record_start;
        record_end = record_start.offset(blocking_factor as isize);
        access_mode = access_mode::ACCESS_WRITE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_next_block() -> *mut block {
    if current_block == record_end {
        if hit_eof {
            return 0 as *mut block;
        }
        flush_archive();
        if current_block == record_end {
            hit_eof = 1 as i32 != 0;
            return 0 as *mut block;
        }
    }
    return current_block;
}
#[no_mangle]
pub unsafe extern "C" fn set_next_block_after(mut block: *mut block) {
    while block >= current_block {
        current_block = current_block.offset(1);
        current_block;
    }
    if current_block > record_end {
        abort();
    }
}
#[no_mangle]
pub unsafe extern "C" fn available_space_after(mut pointer: *mut block) -> size_t {
    return ((*record_end).buffer)
        .as_mut_ptr()
        .offset_from(((*pointer).buffer).as_mut_ptr()) as i64 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn xclose(mut fd: i32) {
    if close(fd) != 0 as i32 {
        close_error(
            dcgettext(0 as *const i8, b"(pipe)\0" as *const u8 as *const i8, 5 as i32),
        );
    }
}
unsafe extern "C" fn init_buffer() {
    if (record_buffer_aligned[record_index as usize]).is_null() {
        record_buffer_aligned[record_index as usize] = page_aligned_alloc(
            &mut *record_buffer.as_mut_ptr().offset(record_index as isize),
            record_size,
        ) as *mut block;
    }
    record_start = record_buffer_aligned[record_index as usize];
    current_block = record_start;
    record_end = record_start.offset(blocking_factor as isize);
}
unsafe extern "C" fn check_tty(mut mode: access_mode) {
    if strcmp(
        *archive_name_array.offset(0 as i32 as isize),
        b"-\0" as *const u8 as *const i8,
    ) == 0 as i32
        && isatty(
            (if mode as u32 == access_mode::ACCESS_READ as i32 as u32 {
                0 as i32
            } else {
                1 as i32
            }),
        ) != 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            if mode as u32 == access_mode::ACCESS_READ as i32 as u32 {
                dcgettext(
                    0 as *const i8,
                    b"Refusing to read archive contents from terminal (missing -f option?)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                )
            } else {
                dcgettext(
                    0 as *const i8,
                    b"Refusing to write archive contents to terminal (missing -f option?)\0"
                        as *const u8 as *const i8,
                    5 as i32,
                )
            },
        );
        fatal_exit();
    }
}
unsafe extern "C" fn _open_archive(mut wanted_access: access_mode) {
    let mut backed_up_flag: i32 = 0 as i32;
    if record_size == 0 as i32 as u64 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Invalid value for record_size\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
    if archive_names == 0 as i32 as u64 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"No archive name given\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
    tar_stat_destroy(&mut current_stat_info);
    record_index = 0 as i32;
    init_buffer();
    access_mode = access_mode::from_libc_c_uint(
        (if wanted_access as u32 == access_mode::ACCESS_UPDATE as i32 as u32 {
            access_mode::ACCESS_READ as i32 as u32
        } else {
            wanted_access as u32
        }) as u32,
    );
    check_tty(access_mode);
    read_full_records = read_full_records_option;
    records_read = 0 as i32 as off_t;
    if !use_compress_program_option.is_null() {
        match wanted_access as u32 {
            0 => {
                child_pid = sys_child_open_for_uncompress();
                read_full_records = 1 as i32 != 0;
                record_end = record_start;
            }
            1 => {
                child_pid = sys_child_open_for_compress();
            }
            2 => {
                abort();
            }
            _ => {}
        }
        if index_file_name.is_null()
            && wanted_access as u32 == access_mode::ACCESS_WRITE as i32 as u32
            && strcmp(
                *archive_name_array.offset(0 as i32 as isize),
                b"-\0" as *const u8 as *const i8,
            ) == 0 as i32
        {
            stdlis = stderr;
        }
    } else if strcmp(
        *archive_name_array.offset(0 as i32 as isize),
        b"-\0" as *const u8 as *const i8,
    ) == 0 as i32
    {
        read_full_records = 1 as i32 != 0;
        if verify_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Cannot verify stdin/stdout archive\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            fatal_exit();
        }
        match wanted_access as u32 {
            0 => {
                let mut shortfile: bool = false;
                let mut type_0: compress_type = compress_type::ct_none;
                archive = 0 as i32;
                type_0 = check_compressed_archive(&mut shortfile);
                if type_0 as u32 != compress_type::ct_tar as i32 as u32
                    && type_0 as u32 != compress_type::ct_none as i32 as u32
                {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Archive is compressed. Use %s option\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                        compress_option(type_0),
                    );
                    fatal_exit();
                }
                if shortfile {
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
                    exit_status = 2 as i32;
                }
            }
            1 => {
                archive = 1 as i32;
                if index_file_name.is_null() {
                    stdlis = stderr;
                }
            }
            2 => {
                archive = 0 as i32;
                write_archive_to_stdout = 1 as i32 != 0;
                record_end = record_start;
                if index_file_name.is_null() {
                    stdlis = stderr;
                }
            }
            _ => {}
        }
    } else {
        match wanted_access as u32 {
            0 => {
                archive = open_compressed_archive();
                if archive >= 0 as i32 {
                    guess_seekable_archive();
                }
            }
            1 => {
                if backup_option {
                    maybe_backup_file(
                        *archive_name_array.offset(0 as i32 as isize),
                        1 as i32 != 0,
                    );
                    backed_up_flag = 1 as i32;
                }
                if verify_option {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(
                                *archive_name_array.offset(0 as i32 as isize),
                                ':' as i32,
                            );
                            !rmt_dev_name__.is_null()
                        }
                        && rmt_dev_name__ > *archive_name_array.offset(0 as i32 as isize)
                        && (memchr(
                            *archive_name_array.offset(0 as i32 as isize)
                                as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__
                                .offset_from(*archive_name_array.offset(0 as i32 as isize))
                                as i64 as u64,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_array.offset(0 as i32 as isize),
                            0o2 as i32 | 0o100 as i32 | 0 as i32,
                            (1 as i32) << 30 as i32,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_array.offset(0 as i32 as isize),
                            0o2 as i32 | 0o100 as i32 | 0 as i32,
                            0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                    | 0o400 as i32 >> 3 as i32 >> 3 as i32),
                        )
                    };
                } else {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(
                                *archive_name_array.offset(0 as i32 as isize),
                                ':' as i32,
                            );
                            !rmt_dev_name__.is_null()
                        }
                        && rmt_dev_name__ > *archive_name_array.offset(0 as i32 as isize)
                        && (memchr(
                            *archive_name_array.offset(0 as i32 as isize)
                                as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__
                                .offset_from(*archive_name_array.offset(0 as i32 as isize))
                                as i64 as u64,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_array.offset(0 as i32 as isize),
                            0o100 as i32 | 0o1 as i32,
                            (1 as i32) << 30 as i32,
                            rsh_command_option,
                        )
                    } else {
                        creat(
                            *archive_name_array.offset(0 as i32 as isize),
                            (0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                    | 0o400 as i32 >> 3 as i32 >> 3 as i32)) as mode_t,
                        )
                    };
                }
            }
            2 => {
                archive = if !force_local_option
                    && {
                        rmt_dev_name__ = strchr(
                            *archive_name_array.offset(0 as i32 as isize),
                            ':' as i32,
                        );
                        !rmt_dev_name__.is_null()
                    } && rmt_dev_name__ > *archive_name_array.offset(0 as i32 as isize)
                    && (memchr(
                        *archive_name_array.offset(0 as i32 as isize)
                            as *const libc::c_void,
                        '/' as i32,
                        rmt_dev_name__
                            .offset_from(*archive_name_array.offset(0 as i32 as isize))
                            as i64 as u64,
                    ))
                        .is_null()
                {
                    rmt_open__(
                        *archive_name_array.offset(0 as i32 as isize),
                        0o2 as i32 | 0o100 as i32 | 0 as i32,
                        (1 as i32) << 30 as i32,
                        rsh_command_option,
                    )
                } else {
                    open(
                        *archive_name_array.offset(0 as i32 as isize),
                        0o2 as i32 | 0o100 as i32 | 0 as i32,
                        0o200 as i32 | 0o200 as i32 >> 3 as i32
                            | 0o200 as i32 >> 3 as i32 >> 3 as i32
                            | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                | 0o400 as i32 >> 3 as i32 >> 3 as i32),
                    )
                };
                match check_compressed_archive(0 as *mut bool) as u32 {
                    0 | 1 => {}
                    _ => {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"Cannot update compressed archives\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        fatal_exit();
                    }
                }
            }
            _ => {}
        }
    }
    if archive < 0 as i32
        || !(archive >= (1 as i32) << 30 as i32) && !sys_get_archive_stat()
    {
        let mut saved_errno: i32 = *__errno_location();
        if backed_up_flag != 0 {
            undo_last_backup();
        }
        *__errno_location() = saved_errno;
        open_fatal(*archive_name_array.offset(0 as i32 as isize));
    }
    sys_detect_dev_null_output();
    sys_save_archive_dev_ino();
    match wanted_access as u32 {
        0 => {
            find_next_block();
        }
        2 | 1 => {
            records_written = 0 as i32 as off_t;
        }
        _ => {}
    };
}
unsafe extern "C" fn _flush_write() -> ssize_t {
    let mut status: ssize_t = 0;
    checkpoint_run(1 as i32 != 0);
    if tape_length_option != 0. && tape_length_option <= bytes_written {
        *__errno_location() = 28 as i32;
        status = 0 as i32 as ssize_t;
    } else if dev_null_output {
        status = record_size as ssize_t;
    } else {
        status = sys_write_archive_buffer() as ssize_t;
    }
    if status != 0 && multi_volume_option as i32 != 0 && inhibit_map == 0 {
        let mut map: *mut bufmap = bufmap_locate(status as size_t);
        if !map.is_null() {
            let mut delta: size_t = (status as u64)
                .wrapping_sub(((*map).start).wrapping_mul(512 as i32 as u64));
            let mut diff: ssize_t = 0;
            (*map).nblocks = ((*map).nblocks as u64)
                .wrapping_add(delta.wrapping_div(512 as i32 as u64)) as size_t as size_t;
            if delta > (*map).sizeleft as u64 {
                delta = (*map).sizeleft as size_t;
            }
            (*map).sizeleft = ((*map).sizeleft as u64).wrapping_sub(delta) as off_t
                as off_t;
            if (*map).sizeleft == 0 as i32 as i64 {
                diff = ((*map).start).wrapping_add((*map).nblocks) as ssize_t;
                map = (*map).next;
            } else {
                diff = (*map).start as ssize_t;
            }
            bufmap_reset(map, -diff);
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn archive_write_error(mut status: ssize_t) -> ! {
    if totals_option {
        let mut e: i32 = *__errno_location();
        print_total_stats();
        *__errno_location() = e;
    }
    write_fatal_details(*archive_name_cursor, status, record_size);
}
#[no_mangle]
pub unsafe extern "C" fn archive_read_error() {
    read_error(*archive_name_cursor);
    if record_start_block == 0 as i32 as i64 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"At beginning of tape, quitting now\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
    let fresh1 = read_error_count;
    read_error_count = read_error_count + 1;
    if fresh1 > 10 as i32 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Too many errors, quitting\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
}
unsafe extern "C" fn archive_is_dev() -> bool {
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
    if fstat(archive, &mut st) != 0 {
        stat_diag(*archive_name_cursor);
        return 0 as i32 != 0;
    }
    return st.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        || st.st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32;
}
unsafe extern "C" fn short_read(mut status: size_t) {
    let mut left: size_t = 0;
    let mut more: *mut i8 = 0 as *mut i8;
    more = ((*record_start).buffer).as_mut_ptr().offset(status as isize);
    left = record_size.wrapping_sub(status);
    if left != 0 && left.wrapping_rem(512 as i32 as u64) == 0 as i32 as u64
        && warning_option & 0x400000 as i32 != 0 && record_start_block == 0 as i32 as i64
        && status != 0 as i32 as u64 && archive_is_dev() as i32 != 0
    {
        let mut rsize: u64 = status.wrapping_div(512 as i32 as u64);
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcngettext(
                0 as *const i8,
                b"Record size = %lu block\0" as *const u8 as *const i8,
                b"Record size = %lu blocks\0" as *const u8 as *const i8,
                rsize,
                5 as i32,
            ),
            rsize,
        );
    }
    while left.wrapping_rem(512 as i32 as u64) != 0 as i32 as u64
        || left != 0 && status != 0 && read_full_records as i32 != 0
    {
        if status != 0 {
            loop {
                status = (if archive >= (1 as i32) << 30 as i32 {
                    rmt_read__(archive - ((1 as i32) << 30 as i32), more, left)
                } else {
                    safe_read(archive, more as *mut libc::c_void, left)
                });
                if !(status == -(1 as i32) as size_t) {
                    break;
                }
                archive_read_error();
            }
        }
        if status == 0 as i32 as u64 {
            break;
        }
        if !read_full_records {
            let mut rest: u64 = record_size.wrapping_sub(left);
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcngettext(
                    0 as *const i8,
                    b"Unaligned block (%lu byte) in archive\0" as *const u8 as *const i8,
                    b"Unaligned block (%lu bytes) in archive\0" as *const u8
                        as *const i8,
                    rest,
                    5 as i32,
                ),
                rest,
            );
            fatal_exit();
        }
        left = (left as u64).wrapping_sub(status) as size_t as size_t;
        more = more.offset(status as isize);
    }
    record_end = record_start
        .offset(record_size.wrapping_sub(left).wrapping_div(512 as i32 as u64) as isize);
    records_read += 1;
    records_read;
}
#[no_mangle]
pub unsafe extern "C" fn flush_archive() {
    let mut buffer_level: size_t = 0;
    if access_mode as u32 == access_mode::ACCESS_READ as i32 as u32
        && time_to_start_writing as i32 != 0
    {
        access_mode = access_mode::ACCESS_WRITE;
        time_to_start_writing = 0 as i32 != 0;
        backspace_output();
        if (record_end.offset_from(record_start) as i64) < blocking_factor as i64 {
            memset(
                record_end as *mut libc::c_void,
                0 as i32,
                ((blocking_factor as i64 - record_end.offset_from(record_start) as i64)
                    * 512 as i32 as i64) as u64,
            );
            record_end = record_start.offset(blocking_factor as isize);
            return;
        }
    }
    buffer_level = ((*current_block).buffer)
        .as_mut_ptr()
        .offset_from(((*record_start).buffer).as_mut_ptr()) as i64 as size_t;
    record_start_block += record_end.offset_from(record_start) as i64;
    current_block = record_start;
    record_end = record_start.offset(blocking_factor as isize);
    match access_mode as u32 {
        0 => {
            flush_read();
        }
        1 => {
            flush_write_ptr.expect("non-null function pointer")(buffer_level);
        }
        2 => {
            abort();
        }
        _ => {}
    };
}
unsafe extern "C" fn backspace_output() {
    let mut operation: mtop = mtop { mt_op: 0, mt_count: 0 };
    operation.mt_op = 4 as i32 as libc::c_short;
    operation.mt_count = 1 as i32;
    if (if archive >= (1 as i32) << 30 as i32 {
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
    }) >= 0 as i32
    {
        return;
    }
    if *__errno_location() == 5 as i32
        && (if archive >= (1 as i32) << 30 as i32 {
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
        }) >= 0 as i32
    {
        return;
    }
    let mut position: off_t = if archive >= (1 as i32) << 30 as i32 {
        rmt_lseek__(archive - ((1 as i32) << 30 as i32), 0 as i32 as off_t, 1 as i32)
    } else {
        lseek(archive, 0 as i32 as off_t, 1 as i32)
    };
    position
        -= ((*record_end).buffer)
            .as_mut_ptr()
            .offset_from(((*record_start).buffer).as_mut_ptr()) as i64;
    if position < 0 as i32 as i64 {
        position = 0 as i32 as off_t;
    }
    if (if archive >= (1 as i32) << 30 as i32 {
        rmt_lseek__(archive - ((1 as i32) << 30 as i32), position, 0 as i32)
    } else {
        lseek(archive, position, 0 as i32)
    }) != position
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Cannot backspace archive file; it may be unreadable without -i\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        if ((*record_start).buffer).as_mut_ptr() != output_start {
            memset(
                ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                0 as i32,
                output_start.offset_from(((*record_start).buffer).as_mut_ptr()) as i64
                    as u64,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn seek_archive(mut size: off_t) -> off_t {
    let mut start: off_t = current_block_ordinal();
    let mut offset: off_t = 0;
    let mut nrec: off_t = 0;
    let mut nblk: off_t = 0;
    let mut skipped: off_t = (blocking_factor as i64
        - current_block.offset_from(record_start) as i64) * 512 as i32 as i64;
    if size <= skipped {
        return 0 as i32 as off_t;
    }
    nrec = ((size - skipped) as u64).wrapping_div(record_size) as off_t;
    if nrec == 0 as i32 as i64 {
        return 0 as i32 as off_t;
    }
    offset = if archive >= (1 as i32) << 30 as i32 {
        rmt_lseek__(
            archive - ((1 as i32) << 30 as i32),
            (nrec as u64).wrapping_mul(record_size) as off_t,
            1 as i32,
        )
    } else {
        lseek(archive, (nrec as u64).wrapping_mul(record_size) as __off_t, 1 as i32)
    };
    if offset < 0 as i32 as i64 {
        return offset;
    }
    if (offset as u64).wrapping_rem(record_size) != 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"rmtlseek not stopped at a record boundary\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
    offset /= 512 as i32 as i64;
    nblk = offset - start;
    records_read += nblk / blocking_factor as i64;
    record_start_block = offset - blocking_factor as i64;
    current_block = record_end;
    return nblk;
}
#[no_mangle]
pub unsafe extern "C" fn close_archive() {
    if time_to_start_writing as i32 != 0
        || access_mode as u32 == access_mode::ACCESS_WRITE as i32 as u32
    {
        loop {
            flush_archive();
            if !(current_block > record_start) {
                break;
            }
        }
    }
    compute_duration();
    if verify_option {
        verify_volume();
    }
    if (if archive >= (1 as i32) << 30 as i32 {
        rmt_close__(archive - ((1 as i32) << 30 as i32))
    } else {
        close(archive)
    }) != 0 as i32
    {
        close_error(*archive_name_cursor);
    }
    sys_wait_for_child(child_pid, hit_eof);
    tar_stat_destroy(&mut current_stat_info);
    rpl_free(record_buffer[0 as i32 as usize]);
    rpl_free(record_buffer[1 as i32 as usize]);
    bufmap_free(0 as *mut bufmap);
}
#[no_mangle]
pub unsafe extern "C" fn write_fatal_details(
    mut name: *const i8,
    mut status: ssize_t,
    mut size: size_t,
) -> ! {
    write_error_details(name, status as size_t, size);
    if (if archive >= (1 as i32) << 30 as i32 {
        rmt_close__(archive - ((1 as i32) << 30 as i32))
    } else {
        close(archive)
    }) != 0 as i32
    {
        close_error(*archive_name_cursor);
    }
    sys_wait_for_child(child_pid, 0 as i32 != 0);
    fatal_exit();
}
#[no_mangle]
pub unsafe extern "C" fn init_volume_number() {
    let mut file: *mut FILE = fopen(volno_file_option, b"r\0" as *const u8 as *const i8);
    if !file.is_null() {
        if fscanf(file, b"%d\0" as *const u8 as *const i8, &mut global_volno as *mut i32)
            != 1 as i32 || global_volno < 0 as i32
        {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: contains invalid volume number\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_colon(volno_file_option),
            );
            fatal_exit();
        }
        if ferror_unlocked(file) != 0 {
            read_error(volno_file_option);
        }
        if fclose(file) != 0 as i32 {
            close_error(volno_file_option);
        }
    } else if *__errno_location() != 2 as i32 {
        open_error(volno_file_option);
    }
}
#[no_mangle]
pub unsafe extern "C" fn closeout_volume_number() {
    let mut file: *mut FILE = fopen(volno_file_option, b"w\0" as *const u8 as *const i8);
    if !file.is_null() {
        fprintf(file, b"%d\n\0" as *const u8 as *const i8, global_volno);
        if ferror_unlocked(file) != 0 {
            write_error(volno_file_option);
        }
        if fclose(file) != 0 as i32 {
            close_error(volno_file_option);
        }
    } else {
        open_error(volno_file_option);
    };
}
unsafe extern "C" fn increase_volume_number() {
    global_volno += 1;
    global_volno;
    if global_volno < 0 as i32 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Volume number overflow\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fatal_exit();
    }
    volno += 1;
    volno;
}
unsafe extern "C" fn change_tape_menu(mut read_file: *mut FILE) {
    let mut input_buffer: *mut i8 = 0 as *mut i8;
    let mut size: size_t = 0 as i32 as size_t;
    let mut stop: bool = 0 as i32 != 0;
    while !stop {
        fputc_unlocked('\u{7}' as i32, stderr);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"Prepare volume #%d for %s and hit return: \0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            global_volno + 1 as i32,
            quote(*archive_name_cursor),
        );
        fflush_unlocked(stderr);
        if getline(&mut input_buffer, &mut size, read_file) <= 0 as i32 as i64 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"EOF where user reply was expected\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            if subcommand_option as u32 != subcommand::EXTRACT_SUBCOMMAND as i32 as u32
                && subcommand_option as u32 != subcommand::LIST_SUBCOMMAND as i32 as u32
                && subcommand_option as u32 != subcommand::DIFF_SUBCOMMAND as i32 as u32
            {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"WARNING: Archive is incomplete\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            fatal_exit();
        }
        if *input_buffer.offset(0 as i32 as isize) as i32 == '\n' as i32
            || *input_buffer.offset(0 as i32 as isize) as i32 == 'y' as i32
            || *input_buffer.offset(0 as i32 as isize) as i32 == 'Y' as i32
        {
            break;
        }
        let mut current_block_50: u64;
        match *input_buffer.offset(0 as i32 as isize) as i32 {
            63 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b" n name        Give a new file name for the next (and subsequent) volume(s)\n q             Abort tar\n y or newline  Continue operation\n\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if !restrict_option {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b" !             Spawn a subshell\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b" ?             Print this list\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                current_block_50 = 2989495919056355252;
            }
            113 => {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"No new volume; exiting.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                if subcommand_option as u32
                    != subcommand::EXTRACT_SUBCOMMAND as i32 as u32
                    && subcommand_option as u32
                        != subcommand::LIST_SUBCOMMAND as i32 as u32
                    && subcommand_option as u32
                        != subcommand::DIFF_SUBCOMMAND as i32 as u32
                {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"WARNING: Archive is incomplete\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                fatal_exit();
            }
            110 => {
                let mut name: *mut i8 = 0 as *mut i8;
                let mut cursor: *mut i8 = 0 as *mut i8;
                name = input_buffer.offset(1 as i32 as isize);
                while *name as i32 == ' ' as i32 || *name as i32 == '\t' as i32 {
                    name = name.offset(1);
                    name;
                }
                cursor = name;
                while *cursor as i32 != 0 && *cursor as i32 != '\n' as i32 {
                    cursor = cursor.offset(1);
                    cursor;
                }
                *cursor = '\0' as i32 as i8;
                if *name.offset(0 as i32 as isize) != 0 {
                    *archive_name_cursor = xstrdup(name);
                    stop = 1 as i32 != 0;
                } else {
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const i8,
                        dcgettext(
                            0 as *const i8,
                            b"File name not specified. Try again.\n\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                }
                current_block_50 = 2989495919056355252;
            }
            33 => {
                if !restrict_option {
                    sys_spawn_shell();
                    current_block_50 = 2989495919056355252;
                } else {
                    current_block_50 = 10389260738713416374;
                }
            }
            _ => {
                current_block_50 = 10389260738713416374;
            }
        }
        match current_block_50 {
            10389260738713416374 => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"Invalid input. Type ? for help.\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
            }
            _ => {}
        }
    }
    rpl_free(input_buffer as *mut libc::c_void);
}
unsafe extern "C" fn new_volume(mut mode: access_mode) -> bool {
    static mut read_file: *mut FILE = 0 as *const FILE as *mut FILE;
    static mut looped: i32 = 0;
    let mut prompt: i32 = 0;
    if read_file.is_null() && info_script_option.is_null() {
        read_file = if archive == 0 as i32 {
            fopen(
                b"/dev/tty\0" as *const u8 as *const i8,
                b"r\0" as *const u8 as *const i8,
            )
        } else {
            stdin
        };
    }
    if now_verifying {
        return 0 as i32 != 0;
    }
    if verify_option {
        verify_volume();
    }
    assign_string(&mut volume_label, 0 as *const i8);
    assign_string(&mut continued_file_name, 0 as *const i8);
    continued_file_offset = 0 as i32 as uintmax_t;
    continued_file_size = continued_file_offset;
    current_block = record_start;
    if (if archive >= (1 as i32) << 30 as i32 {
        rmt_close__(archive - ((1 as i32) << 30 as i32))
    } else {
        close(archive)
    }) != 0 as i32
    {
        close_error(*archive_name_cursor);
    }
    archive_name_cursor = archive_name_cursor.offset(1);
    archive_name_cursor;
    if archive_name_cursor == archive_name_array.offset(archive_names as isize) {
        archive_name_cursor = archive_name_array;
        looped = 1 as i32;
    }
    prompt = looped;
    loop {
        if prompt != 0 {
            if !info_script_option.is_null() {
                if !volno_file_option.is_null() {
                    closeout_volume_number();
                }
                if sys_exec_info_script(archive_name_cursor, global_volno + 1 as i32)
                    != 0
                {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"%s command failed\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote(info_script_option),
                    );
                    fatal_exit();
                }
            } else {
                change_tape_menu(read_file);
            }
        }
        if strcmp(
            *archive_name_cursor.offset(0 as i32 as isize),
            b"-\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            read_full_records = 1 as i32 != 0;
            archive = 0 as i32;
        } else if verify_option {
            archive = if !force_local_option
                && {
                    rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                    !rmt_dev_name__.is_null()
                } && rmt_dev_name__ > *archive_name_cursor
                && (memchr(
                    *archive_name_cursor as *const libc::c_void,
                    '/' as i32,
                    rmt_dev_name__.offset_from(*archive_name_cursor) as i64 as u64,
                ))
                    .is_null()
            {
                rmt_open__(
                    *archive_name_cursor,
                    0o2 as i32 | 0o100 as i32,
                    (1 as i32) << 30 as i32,
                    rsh_command_option,
                )
            } else {
                open(
                    *archive_name_cursor,
                    0o2 as i32 | 0o100 as i32,
                    0o200 as i32 | 0o200 as i32 >> 3 as i32
                        | 0o200 as i32 >> 3 as i32 >> 3 as i32
                        | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                            | 0o400 as i32 >> 3 as i32 >> 3 as i32),
                )
            };
        } else {
            match mode as u32 {
                0 => {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor) as i64
                                as u64,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0 as i32,
                            (1 as i32) << 30 as i32,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_cursor,
                            0 as i32,
                            0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                    | 0o400 as i32 >> 3 as i32 >> 3 as i32),
                        )
                    };
                    guess_seekable_archive();
                }
                1 => {
                    if backup_option {
                        maybe_backup_file(*archive_name_cursor, 1 as i32 != 0);
                    }
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor) as i64
                                as u64,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0o100 as i32 | 0o1 as i32,
                            (1 as i32) << 30 as i32,
                            rsh_command_option,
                        )
                    } else {
                        creat(
                            *archive_name_cursor,
                            (0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                    | 0o400 as i32 >> 3 as i32 >> 3 as i32)) as mode_t,
                        )
                    };
                }
                2 => {
                    archive = if !force_local_option
                        && {
                            rmt_dev_name__ = strchr(*archive_name_cursor, ':' as i32);
                            !rmt_dev_name__.is_null()
                        } && rmt_dev_name__ > *archive_name_cursor
                        && (memchr(
                            *archive_name_cursor as *const libc::c_void,
                            '/' as i32,
                            rmt_dev_name__.offset_from(*archive_name_cursor) as i64
                                as u64,
                        ))
                            .is_null()
                    {
                        rmt_open__(
                            *archive_name_cursor,
                            0o2 as i32 | 0o100 as i32,
                            (1 as i32) << 30 as i32,
                            rsh_command_option,
                        )
                    } else {
                        open(
                            *archive_name_cursor,
                            0o2 as i32 | 0o100 as i32,
                            0o200 as i32 | 0o200 as i32 >> 3 as i32
                                | 0o200 as i32 >> 3 as i32 >> 3 as i32
                                | (0o400 as i32 | 0o400 as i32 >> 3 as i32
                                    | 0o400 as i32 >> 3 as i32 >> 3 as i32),
                        )
                    };
                }
                _ => {}
            }
        }
        if !(archive < 0 as i32) {
            break;
        }
        open_warn(*archive_name_cursor);
        if !verify_option && mode as u32 == access_mode::ACCESS_WRITE as i32 as u32
            && backup_option as i32 != 0
        {
            undo_last_backup();
        }
        prompt = 1 as i32;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn read_header0(mut info: *mut tar_stat_info) -> bool {
    let mut rc: read_header = read_header::HEADER_STILL_UNREAD;
    tar_stat_init(info);
    rc = read_header(&mut current_header, info, read_header_mode::read_header_auto);
    if rc as u32 == read_header::HEADER_SUCCESS as i32 as u32 {
        set_next_block_after(current_header);
        return 1 as i32 != 0;
    }
    if error_hook.is_some() {
        error_hook.expect("non-null function pointer")();
    }
    error(
        0 as i32,
        0 as i32,
        dcgettext(
            0 as *const i8,
            b"This does not look like a tar archive\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
    exit_status = 2 as i32;
    return 0 as i32 != 0;
}
unsafe extern "C" fn try_new_volume() -> bool {
    let mut status: size_t = 0;
    let mut header: *mut block = 0 as *mut block;
    let mut acc: access_mode = access_mode::ACCESS_READ;
    match subcommand_option as u32 {
        1 | 2 | 8 => {
            acc = access_mode::ACCESS_UPDATE;
        }
        _ => {
            acc = access_mode::ACCESS_READ;
        }
    }
    if !new_volume(acc) {
        return 1 as i32 != 0;
    }
    loop {
        status = (if archive >= (1 as i32) << 30 as i32 {
            rmt_read__(
                archive - ((1 as i32) << 30 as i32),
                ((*record_start).buffer).as_mut_ptr(),
                record_size,
            )
        } else {
            safe_read(
                archive,
                ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                record_size,
            )
        });
        if !(status == -(1 as i32) as size_t) {
            break;
        }
        archive_read_error();
    }
    if status != record_size {
        short_read(status);
    }
    header = find_next_block();
    if header.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"This does not look like a tar archive\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return 0 as i32 != 0;
    }
    let mut current_block_50: u64;
    match (*header).header.typeflag as i32 {
        103 => {
            tar_stat_init(&mut dummy);
            if read_header(
                &mut header,
                &mut dummy,
                read_header_mode::read_header_x_global,
            ) as u32 != read_header::HEADER_SUCCESS_EXTENDED as i32 as u32
            {
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
                return 0 as i32 != 0;
            }
            xheader_decode(&mut dummy);
            tar_stat_destroy(&mut dummy);
            match read_header(
                &mut header,
                &mut dummy,
                read_header_mode::read_header_auto,
            ) as u32
            {
                1 => {
                    set_next_block_after(header);
                }
                5 => {}
                _ => {
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
                    return 0 as i32 != 0;
                }
            }
            current_block_50 = 1345366029464561491;
        }
        86 => {
            if !read_header0(&mut dummy) {
                return 0 as i32 != 0;
            }
            tar_stat_destroy(&mut dummy);
            assign_string_n(
                &mut volume_label,
                ((*current_header).header.name).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 100]>() as u64,
            );
            set_next_block_after(header);
            header = find_next_block();
            if (*header).header.typeflag as i32 != 'M' as i32 {
                current_block_50 = 1345366029464561491;
            } else {
                current_block_50 = 12700628668013792433;
            }
        }
        77 => {
            current_block_50 = 12700628668013792433;
        }
        _ => {
            current_block_50 = 1345366029464561491;
        }
    }
    match current_block_50 {
        12700628668013792433 => {
            if !read_header0(&mut dummy) {
                return 0 as i32 != 0;
            }
            tar_stat_destroy(&mut dummy);
            assign_string_n(
                &mut continued_file_name,
                ((*current_header).header.name).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 100]>() as u64,
            );
            continued_file_size = uintmax_from_header(
                ((*current_header).header.size).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 12]>() as u64,
            );
            continued_file_offset = uintmax_from_header(
                ((*current_header).oldgnu_header.offset).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 12]>() as u64,
            );
        }
        _ => {}
    }
    if !bufmap_head.is_null() {
        let mut s: uintmax_t = 0;
        if continued_file_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is not continued on this volume\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote((*bufmap_head).file_name),
            );
            return 0 as i32 != 0;
        }
        if strcmp(continued_file_name, (*bufmap_head).file_name) != 0 {
            if (archive_format as u32 == archive_format::GNU_FORMAT as i32 as u32
                || archive_format as u32 == archive_format::OLDGNU_FORMAT as i32 as u32)
                && strlen((*bufmap_head).file_name) >= 100 as i32 as u64
                && strncmp(
                    continued_file_name,
                    (*bufmap_head).file_name,
                    100 as i32 as u64,
                ) == 0 as i32
            {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is possibly continued on this volume: header contains truncated name\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quote((*bufmap_head).file_name),
                );
            } else {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s is not continued on this volume\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    quote((*bufmap_head).file_name),
                );
                return 0 as i32 != 0;
            }
        }
        s = continued_file_size.wrapping_add(continued_file_offset);
        if (*bufmap_head).sizetotal as u64 != s || s < continued_file_offset {
            let mut totsizebuf: [i8; 21] = [0; 21];
            let mut s1buf: [i8; 21] = [0; 21];
            let mut s2buf: [i8; 21] = [0; 21];
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s is the wrong size (%s != %s + %s)\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(continued_file_name),
                umaxtostr(
                    (*bufmap_head).sizetotal as uintmax_t,
                    totsizebuf.as_mut_ptr(),
                ),
                umaxtostr(continued_file_size, s1buf.as_mut_ptr()),
                umaxtostr(continued_file_offset, s2buf.as_mut_ptr()),
            );
            return 0 as i32 != 0;
        }
        if ((*bufmap_head).sizetotal - (*bufmap_head).sizeleft) as u64
            != continued_file_offset
        {
            let mut totsizebuf_0: [i8; 21] = [0; 21];
            let mut s1buf_0: [i8; 21] = [0; 21];
            let mut s2buf_0: [i8; 21] = [0; 21];
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"This volume is out of sequence (%s - %s != %s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                umaxtostr(
                    (*bufmap_head).sizetotal as uintmax_t,
                    totsizebuf_0.as_mut_ptr(),
                ),
                umaxtostr((*bufmap_head).sizeleft as uintmax_t, s1buf_0.as_mut_ptr()),
                umaxtostr(continued_file_offset, s2buf_0.as_mut_ptr()),
            );
            return 0 as i32 != 0;
        }
    }
    increase_volume_number();
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn drop_volume_label_suffix(mut label: *const i8) -> *mut i8 {
    let mut p: *const i8 = 0 as *const i8;
    let mut len: size_t = strlen(label);
    if len < 1 as i32 as u64 {
        return 0 as *mut i8;
    }
    p = label.offset(len as isize).offset(-(1 as i32 as isize));
    while p > label
        && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
            & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        p = p.offset(-1);
        p;
    }
    if p > label
        && p
            .offset(
                -((::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) as isize),
            ) > label
    {
        p = p
            .offset(
                -((::core::mem::size_of::<[i8; 9]>() as u64)
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) as isize),
            );
        if memcmp(
            p as *const libc::c_void,
            b" Volume \0" as *const u8 as *const i8 as *const libc::c_void,
            (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64),
        ) == 0 as i32
        {
            len = p.offset_from(label) as i64 as size_t;
            let mut s: *mut i8 = xmalloc(len.wrapping_add(1 as i32 as u64)) as *mut i8;
            memcpy(s as *mut libc::c_void, label as *const libc::c_void, len);
            *s.offset(len as isize) = 0 as i32 as i8;
            return s;
        }
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn check_label_pattern(mut label: *const i8) -> bool {
    let mut string: *mut i8 = 0 as *mut i8;
    let mut result: bool = 0 as i32 != 0;
    if fnmatch(volume_label_option, label, 0 as i32) == 0 as i32 {
        return 1 as i32 != 0;
    }
    if !multi_volume_option {
        return 0 as i32 != 0;
    }
    string = drop_volume_label_suffix(label);
    if !string.is_null() {
        result = fnmatch(string, volume_label_option, 0 as i32) == 0 as i32;
        rpl_free(string as *mut libc::c_void);
    }
    return result;
}
unsafe extern "C" fn match_volume_label() {
    if volume_label.is_null() {
        let mut label: *mut block = find_next_block();
        if label.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"Archive not labeled to match %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quote(volume_label_option),
            );
            fatal_exit();
        }
        if (*label).header.typeflag as i32 == 'V' as i32 {
            assign_string_n(
                &mut volume_label,
                ((*label).header.name).as_mut_ptr(),
                ::core::mem::size_of::<[i8; 100]>() as u64,
            );
        } else if (*label).header.typeflag as i32 == 'g' as i32 {
            let mut st: tar_stat_info = tar_stat_info {
                orig_file_name: 0 as *const i8 as *mut i8,
                file_name: 0 as *const i8 as *mut i8,
                had_trailing_slash: false,
                link_name: 0 as *const i8 as *mut i8,
                uname: 0 as *const i8 as *mut i8,
                gname: 0 as *const i8 as *mut i8,
                cntx_name: 0 as *const i8 as *mut i8,
                acls_a_ptr: 0 as *const i8 as *mut i8,
                acls_a_len: 0,
                acls_d_ptr: 0 as *const i8 as *mut i8,
                acls_d_len: 0,
                stat: stat {
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
                },
                atime: timespec { tv_sec: 0, tv_nsec: 0 },
                mtime: timespec { tv_sec: 0, tv_nsec: 0 },
                ctime: timespec { tv_sec: 0, tv_nsec: 0 },
                archive_file_size: 0,
                is_sparse: false,
                sparse_major: 0,
                sparse_minor: 0,
                sparse_map_avail: 0,
                sparse_map_size: 0,
                sparse_map: 0 as *const sp_array as *mut sp_array,
                real_size: 0,
                real_size_set: false,
                sparse_name_done: false,
                xattr_map_size: 0,
                xattr_map: 0 as *const xattr_array as *mut xattr_array,
                xhdr: xheader {
                    stk: 0 as *const obstack as *mut obstack,
                    size: 0,
                    buffer: 0 as *const i8 as *mut i8,
                    string_length: 0,
                },
                is_dumpdir: false,
                skipped: false,
                dumpdir: 0 as *const i8 as *mut i8,
                parent: 0 as *const tar_stat_info as *mut tar_stat_info,
                dirstream: 0 as *const DIR as *mut DIR,
                fd: 0,
                exclude_list: 0 as *const exclist as *mut exclist,
            };
            tar_stat_init(&mut st);
            xheader_read(
                &mut st.xhdr,
                label,
                off_from_header(
                    ((*label).header.size).as_mut_ptr(),
                    ::core::mem::size_of::<[i8; 12]>() as u64,
                ),
            );
            xheader_decode(&mut st);
            tar_stat_destroy(&mut st);
        }
    }
    if volume_label.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Archive not labeled to match %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote(volume_label_option),
        );
        fatal_exit();
    }
    if !check_label_pattern(volume_label) {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Volume %s does not match %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            quote_n(0 as i32, volume_label),
            quote_n(1 as i32, volume_label_option),
        );
        fatal_exit();
    }
}
unsafe extern "C" fn _write_volume_label(mut str: *const i8) {
    if archive_format as u32 == archive_format::POSIX_FORMAT as i32 as u32 {
        xheader_store(
            b"GNU.volume.label\0" as *const u8 as *const i8,
            &mut dummy,
            str as *const libc::c_void,
        );
    } else {
        let mut label: *mut block = find_next_block();
        memset(label as *mut libc::c_void, 0 as i32, 512 as i32 as u64);
        strcpy(((*label).header.name).as_mut_ptr(), str);
        assign_string(
            &mut current_stat_info.file_name,
            ((*label).header.name).as_mut_ptr(),
        );
        current_stat_info.had_trailing_slash = strip_trailing_slashes(
            current_stat_info.file_name,
        );
        (*label).header.typeflag = 'V' as i32 as i8;
        time_to_chars(
            start_time.tv_sec,
            ((*label).header.mtime).as_mut_ptr(),
            ::core::mem::size_of::<[i8; 12]>() as u64,
        );
        finish_header(&mut current_stat_info, label, -(1 as i32) as off_t);
        set_next_block_after(label);
    };
}
unsafe extern "C" fn add_volume_label() {
    let mut buf: [i8; 21] = [0; 21];
    let mut p: *mut i8 = umaxtostr(volno as uintmax_t, buf.as_mut_ptr());
    let mut s: *mut i8 = xmalloc(
        (strlen(volume_label_option))
            .wrapping_add(::core::mem::size_of::<[i8; 7]>() as u64)
            .wrapping_add(strlen(p))
            .wrapping_add(2 as i32 as u64),
    ) as *mut i8;
    sprintf(
        s,
        b"%s %s %s\0" as *const u8 as *const i8,
        volume_label_option,
        b"Volume\0" as *const u8 as *const i8,
        p,
    );
    _write_volume_label(s);
    rpl_free(s as *mut libc::c_void);
}
unsafe extern "C" fn add_chunk_header(mut map: *mut bufmap) {
    if archive_format as u32 == archive_format::POSIX_FORMAT as i32 as u32 {
        let mut blk: *mut block = 0 as *mut block;
        let mut st: tar_stat_info = tar_stat_info {
            orig_file_name: 0 as *const i8 as *mut i8,
            file_name: 0 as *const i8 as *mut i8,
            had_trailing_slash: false,
            link_name: 0 as *const i8 as *mut i8,
            uname: 0 as *const i8 as *mut i8,
            gname: 0 as *const i8 as *mut i8,
            cntx_name: 0 as *const i8 as *mut i8,
            acls_a_ptr: 0 as *const i8 as *mut i8,
            acls_a_len: 0,
            acls_d_ptr: 0 as *const i8 as *mut i8,
            acls_d_len: 0,
            stat: stat {
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
            },
            atime: timespec { tv_sec: 0, tv_nsec: 0 },
            mtime: timespec { tv_sec: 0, tv_nsec: 0 },
            ctime: timespec { tv_sec: 0, tv_nsec: 0 },
            archive_file_size: 0,
            is_sparse: false,
            sparse_major: 0,
            sparse_minor: 0,
            sparse_map_avail: 0,
            sparse_map_size: 0,
            sparse_map: 0 as *const sp_array as *mut sp_array,
            real_size: 0,
            real_size_set: false,
            sparse_name_done: false,
            xattr_map_size: 0,
            xattr_map: 0 as *const xattr_array as *mut xattr_array,
            xhdr: xheader {
                stk: 0 as *const obstack as *mut obstack,
                size: 0,
                buffer: 0 as *const i8 as *mut i8,
                string_length: 0,
            },
            is_dumpdir: false,
            skipped: false,
            dumpdir: 0 as *const i8 as *mut i8,
            parent: 0 as *const tar_stat_info as *mut tar_stat_info,
            dirstream: 0 as *const DIR as *mut DIR,
            fd: 0,
            exclude_list: 0 as *const exclist as *mut exclist,
        };
        memset(
            &mut st as *mut tar_stat_info as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<tar_stat_info>() as u64,
        );
        st.file_name = (*map).file_name;
        st.orig_file_name = st.file_name;
        st.stat.st_mode = (0o100000 as i32 | 0o400 as i32 | 0o200 as i32
            | 0o400 as i32 >> 3 as i32 | 0o400 as i32 >> 3 as i32 >> 3 as i32)
            as __mode_t;
        st.stat.st_uid = getuid();
        st.stat.st_gid = getgid();
        st.orig_file_name = xheader_format_name(
            &mut st,
            b"%d/GNUFileParts/%f.%n\0" as *const u8 as *const i8,
            volno as size_t,
        );
        st.file_name = st.orig_file_name;
        st.stat.st_size = (*map).sizeleft;
        st.archive_file_size = st.stat.st_size;
        blk = start_header(&mut st);
        if blk.is_null() {
            abort();
        }
        simple_finish_header(write_extended(0 as i32 != 0, &mut st, blk));
        rpl_free(st.orig_file_name as *mut libc::c_void);
    }
}
unsafe extern "C" fn write_volume_label() {
    if multi_volume_option {
        add_volume_label();
    } else {
        _write_volume_label(volume_label_option);
    };
}
unsafe extern "C" fn gnu_add_multi_volume_header(mut map: *mut bufmap) {
    let mut tmp: i32 = 0;
    let mut block: *mut block = find_next_block();
    let mut len: size_t = strlen((*map).file_name);
    if len > 100 as i32 as u64 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"%s: file name too long to be stored in a GNU multivolume header, truncated\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            quotearg_colon((*map).file_name),
        );
        len = 100 as i32 as size_t;
    }
    memset(block as *mut libc::c_void, 0 as i32, 512 as i32 as u64);
    memcpy(
        ((*block).header.name).as_mut_ptr() as *mut libc::c_void,
        (*map).file_name as *const libc::c_void,
        len,
    );
    (*block).header.typeflag = 'M' as i32 as i8;
    off_to_chars(
        (*map).sizeleft,
        ((*block).header.size).as_mut_ptr(),
        ::core::mem::size_of::<[i8; 12]>() as u64,
    );
    off_to_chars(
        (*map).sizetotal - (*map).sizeleft,
        ((*block).oldgnu_header.offset).as_mut_ptr(),
        ::core::mem::size_of::<[i8; 12]>() as u64,
    );
    tmp = verbose_option;
    verbose_option = 0 as i32;
    finish_header(&mut current_stat_info, block, -(1 as i32) as off_t);
    verbose_option = tmp;
    set_next_block_after(block);
}
unsafe extern "C" fn add_multi_volume_header(mut map: *mut bufmap) {
    if archive_format as u32 == archive_format::POSIX_FORMAT as i32 as u32 {
        let mut d: off_t = (*map).sizetotal - (*map).sizeleft;
        xheader_store(
            b"GNU.volume.filename\0" as *const u8 as *const i8,
            &mut dummy,
            (*map).file_name as *const libc::c_void,
        );
        xheader_store(
            b"GNU.volume.size\0" as *const u8 as *const i8,
            &mut dummy,
            &mut (*map).sizeleft as *mut off_t as *const libc::c_void,
        );
        xheader_store(
            b"GNU.volume.offset\0" as *const u8 as *const i8,
            &mut dummy,
            &mut d as *mut off_t as *const libc::c_void,
        );
    } else {
        gnu_add_multi_volume_header(map);
    };
}
unsafe extern "C" fn simple_flush_read() {
    let mut status: size_t = 0;
    checkpoint_run(0 as i32 != 0);
    read_error_count = 0 as i32;
    if write_archive_to_stdout as i32 != 0 && record_start_block != 0 as i32 as i64 {
        archive = 1 as i32;
        status = sys_write_archive_buffer();
        archive = 0 as i32;
        if status != record_size {
            archive_write_error(status as ssize_t);
        }
    }
    loop {
        status = if archive >= (1 as i32) << 30 as i32 {
            rmt_read__(
                archive - ((1 as i32) << 30 as i32),
                ((*record_start).buffer).as_mut_ptr(),
                record_size,
            )
        } else {
            safe_read(
                archive,
                ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                record_size,
            )
        };
        if status == record_size {
            records_read += 1;
            records_read;
            return;
        }
        if !(status == -(1 as i32) as size_t) {
            break;
        }
        archive_read_error();
    }
    short_read(status);
}
unsafe extern "C" fn simple_flush_write(mut level: size_t) {
    let mut status: ssize_t = 0;
    status = _flush_write();
    if status as u64 != record_size {
        archive_write_error(status);
    } else {
        records_written += 1;
        records_written;
        bytes_written += status as libc::c_double;
    };
}
unsafe extern "C" fn _gnu_flush_read() {
    let mut status: size_t = 0;
    checkpoint_run(0 as i32 != 0);
    read_error_count = 0 as i32;
    if write_archive_to_stdout as i32 != 0 && record_start_block != 0 as i32 as i64 {
        archive = 1 as i32;
        status = sys_write_archive_buffer();
        archive = 0 as i32;
        if status != record_size {
            archive_write_error(status as ssize_t);
        }
    }
    loop {
        status = if archive >= (1 as i32) << 30 as i32 {
            rmt_read__(
                archive - ((1 as i32) << 30 as i32),
                ((*record_start).buffer).as_mut_ptr(),
                record_size,
            )
        } else {
            safe_read(
                archive,
                ((*record_start).buffer).as_mut_ptr() as *mut libc::c_void,
                record_size,
            )
        };
        if status == record_size {
            records_read += 1;
            records_read;
            return;
        }
        if (status == 0 as i32 as u64
            || status == -(1 as i32) as size_t && *__errno_location() == 28 as i32)
            && multi_volume_option as i32 != 0
        {
            while !try_new_volume() {}
            if current_block == record_end {
                flush_archive();
            }
            return;
        } else {
            if !(status == -(1 as i32) as size_t) {
                break;
            }
            archive_read_error();
        }
    }
    short_read(status);
}
unsafe extern "C" fn gnu_flush_read() {
    flush_read_ptr = Some(simple_flush_read as unsafe extern "C" fn() -> ());
    _gnu_flush_read();
    flush_read_ptr = Some(gnu_flush_read as unsafe extern "C" fn() -> ());
}
unsafe extern "C" fn _gnu_flush_write(mut buffer_level: size_t) {
    let mut status: ssize_t = 0;
    let mut header: *mut block = 0 as *mut block;
    let mut copy_ptr: *mut i8 = 0 as *mut i8;
    let mut copy_size: size_t = 0;
    let mut bufsize: size_t = 0;
    let mut map: *mut bufmap = 0 as *mut bufmap;
    status = _flush_write();
    if status as u64 != record_size && !multi_volume_option {
        archive_write_error(status);
    } else {
        if status != 0 {
            records_written += 1;
            records_written;
        }
        bytes_written += status as libc::c_double;
    }
    if status as u64 == record_size {
        return;
    }
    map = bufmap_locate(status as size_t);
    if status % 512 as i32 as i64 != 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"write did not end on a block boundary\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit_status = 2 as i32;
        archive_write_error(status);
    }
    if status < 0 as i32 as i64 && *__errno_location() != 28 as i32
        && *__errno_location() != 5 as i32 && *__errno_location() != 6 as i32
    {
        archive_write_error(status);
    }
    if !new_volume(access_mode::ACCESS_WRITE) {
        return;
    }
    tar_stat_destroy(&mut dummy);
    increase_volume_number();
    prev_written += bytes_written;
    bytes_written = 0 as i32 as tarlong;
    copy_ptr = ((*record_start).buffer).as_mut_ptr().offset(status as isize);
    copy_size = buffer_level.wrapping_sub(status as u64);
    record_index = (record_index == 0) as i32;
    init_buffer();
    inhibit_map = 1 as i32;
    if !volume_label_option.is_null() {
        add_volume_label();
    }
    if !map.is_null() {
        add_multi_volume_header(map);
    }
    write_extended(1 as i32 != 0, &mut dummy, find_next_block());
    tar_stat_destroy(&mut dummy);
    if !map.is_null() {
        add_chunk_header(map);
    }
    header = find_next_block();
    bufmap_reset(map, header.offset_from(record_start) as i64);
    bufsize = available_space_after(header);
    inhibit_map = 0 as i32;
    while bufsize < copy_size {
        memcpy(
            ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
            copy_ptr as *const libc::c_void,
            bufsize,
        );
        copy_ptr = copy_ptr.offset(bufsize as isize);
        copy_size = (copy_size as u64).wrapping_sub(bufsize) as size_t as size_t;
        set_next_block_after(
            header
                .offset(
                    bufsize.wrapping_sub(1 as i32 as u64).wrapping_div(512 as i32 as u64)
                        as isize,
                ),
        );
        header = find_next_block();
        bufsize = available_space_after(header);
    }
    memcpy(
        ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
        copy_ptr as *const libc::c_void,
        copy_size,
    );
    memset(
        ((*header).buffer).as_mut_ptr().offset(copy_size as isize) as *mut libc::c_void,
        0 as i32,
        bufsize.wrapping_sub(copy_size),
    );
    set_next_block_after(
        header
            .offset(
                copy_size.wrapping_sub(1 as i32 as u64).wrapping_div(512 as i32 as u64)
                    as isize,
            ),
    );
    find_next_block();
}
unsafe extern "C" fn gnu_flush_write(mut buffer_level: size_t) {
    flush_write_ptr = Some(simple_flush_write as unsafe extern "C" fn(size_t) -> ());
    _gnu_flush_write(buffer_level);
    flush_write_ptr = Some(gnu_flush_write as unsafe extern "C" fn(size_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn flush_read() {
    flush_read_ptr.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn flush_write() {
    flush_write_ptr.expect("non-null function pointer")(record_size);
}
#[no_mangle]
pub unsafe extern "C" fn open_archive(mut wanted_access: access_mode) {
    flush_read_ptr = Some(gnu_flush_read as unsafe extern "C" fn() -> ());
    flush_write_ptr = Some(gnu_flush_write as unsafe extern "C" fn(size_t) -> ());
    _open_archive(wanted_access);
    match wanted_access as u32 {
        0 | 2 => {
            if !volume_label_option.is_null() {
                match_volume_label();
            }
        }
        1 => {
            records_written = 0 as i32 as off_t;
            if !volume_label_option.is_null() {
                write_volume_label();
            }
        }
        _ => {}
    }
    set_volume_start_time();
}