use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type hash_table;
    pub type quoting_options;
    pub type wordsplit_node;
    pub type exclist;
    pub type exclude;
    pub type directory;
    pub type textual_date;
    static mut stdin: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn openat(__fd: i32, __file: *const i8, __oflag: i32, _: ...) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn fatal_exit() -> !;
    fn stat_fatal(_: *const i8) -> !;
    fn open_fatal(_: *const i8) -> !;
    static mut exit_status: i32;
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    fn getgrnam(__name: *const i8) -> *mut group;
    fn getgrgid(__gid: __gid_t) -> *mut group;
    fn getpwnam(__name: *const i8) -> *mut passwd;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_string(string: *const i8, n_buckets: size_t) -> size_t;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_free(table: *mut Hash_table);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn set_char_quoting(o: *mut quoting_options, c: i8, i: i32) -> i32;
    fn quotearg_n(n: i32, arg: *const i8) -> *mut i8;
    fn quotearg_colon(arg: *const i8) -> *mut i8;
    fn wordsplit(s: *const i8, ws: *mut wordsplit_t, flags: u32) -> i32;
    fn wordsplit_free(ws: *mut wordsplit_t);
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const i8;
    static mut subcommand_option: subcommand;
    static mut excluded: *mut exclude;
    static mut occurrence_option: uintmax_t;
    static mut listed_incremental_option: *const i8;
    static mut recursion_option: i32;
    static mut same_order_option: bool;
    static mut starting_file_option: bool;
    static mut verbose_option: i32;
    static mut open_read_flags: i32;
    static mut unquote_option: bool;
    fn add_exclusion_tag(
        name: *const i8,
        type_0: exclusion_tag_type,
        predicate: Option<unsafe extern "C" fn(i32) -> bool>,
    );
    fn cachedir_file_p(fd: i32) -> bool;
    fn subfile_open(dir: *const tar_stat_info, file: *const i8, flags: i32) -> i32;
    fn restore_parent_fd(st: *const tar_stat_info);
    fn scan_directory(st: *mut tar_stat_info) -> *mut directory;
    fn directory_contents(dir: *mut directory) -> *const i8;
    fn rebase_directory(
        dir: *mut directory,
        samp: *const i8,
        slen: size_t,
        repl: *const i8,
        rlen: size_t,
    );
    fn append_incremental_renames(dir: *mut directory);
    fn read_directory_file();
    fn assign_string(dest: *mut *mut i8, src: *const i8);
    fn unquote_string(str: *mut i8) -> i32;
    fn normalize_filename(cdidx: i32, name: *const i8) -> *mut i8;
    fn deref_stat(name: *const i8, buf: *mut stat) -> i32;
    static mut chdir_fd: i32;
    fn chdir_arg(dir: *const i8) -> i32;
    fn chdir_do(dir: i32);
    fn chdir_count() -> i32;
    fn open_diag(name: *const i8);
    fn stat_diag(name: *const i8);
    fn usage(_: i32);
    fn tar_stat_init(st: *mut tar_stat_info);
    fn tar_stat_destroy(st: *mut tar_stat_info);
    fn set_exit_status(val: i32);
    fn request_stdin(rpl_option: *const i8);
    fn more_options(argc: i32, argv: *mut *mut i8, loc: *mut option_locus);
    static mut warning_option: i32;
    static mut program_name: *const i8;
    fn excfile_add(name: *const i8, flags: i32);
    fn exclude_vcs_ignores();
    fn quote(arg: *const i8) -> *const i8;
    fn add_exclude_file(
        _: Option<unsafe extern "C" fn(*mut exclude, *const i8, i32) -> ()>,
        _: *mut exclude,
        _: *const i8,
        _: i32,
        _: i8,
    ) -> i32;
    fn add_exclude(_: *mut exclude, _: *const i8, _: i32);
    fn fnmatch_pattern_has_wildcards(_: *const i8, _: i32) -> bool;
    fn exclude_fnmatch(_: *const i8, _: *const i8, _: i32) -> bool;
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
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
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
pub type error_t = i32;
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
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut i8,
    pub gr_passwd: *mut i8,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut i8,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut i8,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: u32,
    pub ws_options: u32,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const i8,
    pub ws_comment: *const i8,
    pub ws_escape: [*const i8; 2],
    pub ws_alloc_die: Option<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_debug: Option<unsafe extern "C" fn(*const i8, ...) -> ()>,
    pub ws_env: *mut *const i8,
    pub ws_envbuf: *mut *mut i8,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option<
        unsafe extern "C" fn(*mut *mut i8, *const i8, size_t, *mut libc::c_void) -> i32,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *const i8,
            size_t,
            *mut *mut i8,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub ws_input: *const i8,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: i32,
    pub ws_usererr: *mut i8,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: i32,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const i8,
    pub key: i32,
    pub arg: *const i8,
    pub flags: i32,
    pub doc: *const i8,
    pub group: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const i8,
    pub doc: *const i8,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(i32, *const i8, *mut libc::c_void) -> *mut i8,
    >,
    pub argp_domain: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: i32,
    pub header: *const i8,
    pub group: i32,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: i32,
    pub argv: *mut *mut i8,
    pub next: i32,
    pub flags: u32,
    pub arg_num: u32,
    pub quoted: i32,
    pub input: *mut libc::c_void,
    pub child_inputs: *mut *mut libc::c_void,
    pub hook: *mut libc::c_void,
    pub name: *mut i8,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut libc::c_void,
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
pub enum exclusion_tag_type {
    exclusion_tag_none,
    exclusion_tag_contents,
    exclusion_tag_under,
    exclusion_tag_all,
}
impl exclusion_tag_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            exclusion_tag_type::exclusion_tag_none => 0,
            exclusion_tag_type::exclusion_tag_contents => 1,
            exclusion_tag_type::exclusion_tag_under => 2,
            exclusion_tag_type::exclusion_tag_all => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> exclusion_tag_type {
        match value {
            0 => exclusion_tag_type::exclusion_tag_none,
            1 => exclusion_tag_type::exclusion_tag_contents,
            2 => exclusion_tag_type::exclusion_tag_under,
            3 => exclusion_tag_type::exclusion_tag_all,
            _ => panic!("Invalid value for exclusion_tag_type: {}", value),
        }
    }
}
impl AddAssign<u32> for exclusion_tag_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for exclusion_tag_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for exclusion_tag_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for exclusion_tag_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for exclusion_tag_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for exclusion_tag_type {
    type Output = exclusion_tag_type;
    fn add(self, rhs: u32) -> exclusion_tag_type {
        exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for exclusion_tag_type {
    type Output = exclusion_tag_type;
    fn sub(self, rhs: u32) -> exclusion_tag_type {
        exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for exclusion_tag_type {
    type Output = exclusion_tag_type;
    fn mul(self, rhs: u32) -> exclusion_tag_type {
        exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for exclusion_tag_type {
    type Output = exclusion_tag_type;
    fn div(self, rhs: u32) -> exclusion_tag_type {
        exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for exclusion_tag_type {
    type Output = exclusion_tag_type;
    fn rem(self, rhs: u32) -> exclusion_tag_type {
        exclusion_tag_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
pub enum files_count {
    FILES_NONE,
    FILES_ONE,
    FILES_MANY,
}
impl files_count {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            files_count::FILES_NONE => 0,
            files_count::FILES_ONE => 1,
            files_count::FILES_MANY => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> files_count {
        match value {
            0 => files_count::FILES_NONE,
            1 => files_count::FILES_ONE,
            2 => files_count::FILES_MANY,
            _ => panic!("Invalid value for files_count: {}", value),
        }
    }
}
impl AddAssign<u32> for files_count {
    fn add_assign(&mut self, rhs: u32) {
        *self = files_count::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for files_count {
    fn sub_assign(&mut self, rhs: u32) {
        *self = files_count::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for files_count {
    fn mul_assign(&mut self, rhs: u32) {
        *self = files_count::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for files_count {
    fn div_assign(&mut self, rhs: u32) {
        *self = files_count::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for files_count {
    fn rem_assign(&mut self, rhs: u32) {
        *self = files_count::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for files_count {
    type Output = files_count;
    fn add(self, rhs: u32) -> files_count {
        files_count::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for files_count {
    type Output = files_count;
    fn sub(self, rhs: u32) -> files_count {
        files_count::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for files_count {
    type Output = files_count;
    fn mul(self, rhs: u32) -> files_count {
        files_count::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for files_count {
    type Output = files_count;
    fn div(self, rhs: u32) -> files_count {
        files_count::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for files_count {
    type Output = files_count;
    fn rem(self, rhs: u32) -> files_count {
        files_count::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_elt {
    pub next: *mut name_elt,
    pub prev: *mut name_elt,
    pub type_0: nelt_type,
    pub v: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub name: *const i8,
    pub file: C2RustUnnamed_5,
    pub opt: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub rpl_option: i32,
    pub arg: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub name: *const i8,
    pub line: size_t,
    pub term: i32,
    pub verbatim: bool,
    pub fp: *mut FILE,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum nelt_type {
    NELT_NAME,
    NELT_CHDIR,
    NELT_FILE,
    NELT_NOOP,
    NELT_OPTION,
}
impl nelt_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            nelt_type::NELT_NAME => 0,
            nelt_type::NELT_CHDIR => 1,
            nelt_type::NELT_FILE => 2,
            nelt_type::NELT_NOOP => 3,
            nelt_type::NELT_OPTION => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> nelt_type {
        match value {
            0 => nelt_type::NELT_NAME,
            1 => nelt_type::NELT_CHDIR,
            2 => nelt_type::NELT_FILE,
            3 => nelt_type::NELT_NOOP,
            4 => nelt_type::NELT_OPTION,
            _ => panic!("Invalid value for nelt_type: {}", value),
        }
    }
}
impl AddAssign<u32> for nelt_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = nelt_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for nelt_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = nelt_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for nelt_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = nelt_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for nelt_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = nelt_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for nelt_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = nelt_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for nelt_type {
    type Output = nelt_type;
    fn add(self, rhs: u32) -> nelt_type {
        nelt_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for nelt_type {
    type Output = nelt_type;
    fn sub(self, rhs: u32) -> nelt_type {
        nelt_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for nelt_type {
    type Output = nelt_type;
    fn mul(self, rhs: u32) -> nelt_type {
        nelt_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for nelt_type {
    type Output = nelt_type;
    fn div(self, rhs: u32) -> nelt_type {
        nelt_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for nelt_type {
    type Output = nelt_type;
    fn rem(self, rhs: u32) -> nelt_type {
        nelt_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_7 {
    GRID_MATCH = 3,
    GRH_MATCH = 2,
    GRID_LOCAL = 1,
    GRH_LOCAL = 0,
}
impl C2RustUnnamed_7 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_7::GRID_MATCH => 3,
            C2RustUnnamed_7::GRH_MATCH => 2,
            C2RustUnnamed_7::GRID_LOCAL => 1,
            C2RustUnnamed_7::GRH_LOCAL => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_7 {
        match value {
            3 => C2RustUnnamed_7::GRID_MATCH,
            2 => C2RustUnnamed_7::GRH_MATCH,
            1 => C2RustUnnamed_7::GRID_LOCAL,
            0 => C2RustUnnamed_7::GRH_LOCAL,
            _ => panic!("Invalid value for C2RustUnnamed_7: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_7 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_7 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_7 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_7 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_7 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn add(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn sub(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn mul(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn div(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_7 {
    type Output = C2RustUnnamed_7;
    fn rem(self, rhs: u32) -> C2RustUnnamed_7 {
        C2RustUnnamed_7::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_6 {
    NO_WILDCARDS_MATCH_SLASH_OPTION = 278,
    WILDCARDS_MATCH_SLASH_OPTION = 283,
    NO_WILDCARDS_OPTION = 279,
    WILDCARDS_OPTION = 284,
    NO_IGNORE_CASE_OPTION = 270,
    IGNORE_CASE_OPTION = 269,
    NO_ANCHORED_OPTION = 272,
    ANCHORED_OPTION = 271,
    NO_RECURSION_OPTION = 274,
    RECURSION_OPTION = 273,
    EXCLUDE_BACKUPS_OPTION = 257,
    EXCLUDE_VCS_IGNORES_OPTION = 268,
    EXCLUDE_VCS_OPTION = 267,
    EXCLUDE_TAG_ALL_OPTION = 266,
    EXCLUDE_TAG_UNDER_OPTION = 265,
    EXCLUDE_IGNORE_RECURSIVE_OPTION = 263,
    EXCLUDE_IGNORE_OPTION = 262,
    EXCLUDE_TAG_OPTION = 264,
    EXCLUDE_CACHES_ALL_OPTION = 260,
    EXCLUDE_CACHES_UNDER_OPTION = 259,
    EXCLUDE_CACHES_OPTION = 258,
    EXCLUDE_OPTION = 261,
    NO_VERBATIM_FILES_FROM_OPTION = 277,
    VERBATIM_FILES_FROM_OPTION = 282,
    NO_UNQUOTE_OPTION = 276,
    UNQUOTE_OPTION = 275,
    NO_NULL_OPTION = 281,
    NULL_OPTION = 280,
    ADD_FILE_OPTION = 256,
}
impl C2RustUnnamed_6 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_6::NO_WILDCARDS_MATCH_SLASH_OPTION => 278,
            C2RustUnnamed_6::WILDCARDS_MATCH_SLASH_OPTION => 283,
            C2RustUnnamed_6::NO_WILDCARDS_OPTION => 279,
            C2RustUnnamed_6::WILDCARDS_OPTION => 284,
            C2RustUnnamed_6::NO_IGNORE_CASE_OPTION => 270,
            C2RustUnnamed_6::IGNORE_CASE_OPTION => 269,
            C2RustUnnamed_6::NO_ANCHORED_OPTION => 272,
            C2RustUnnamed_6::ANCHORED_OPTION => 271,
            C2RustUnnamed_6::NO_RECURSION_OPTION => 274,
            C2RustUnnamed_6::RECURSION_OPTION => 273,
            C2RustUnnamed_6::EXCLUDE_BACKUPS_OPTION => 257,
            C2RustUnnamed_6::EXCLUDE_VCS_IGNORES_OPTION => 268,
            C2RustUnnamed_6::EXCLUDE_VCS_OPTION => 267,
            C2RustUnnamed_6::EXCLUDE_TAG_ALL_OPTION => 266,
            C2RustUnnamed_6::EXCLUDE_TAG_UNDER_OPTION => 265,
            C2RustUnnamed_6::EXCLUDE_IGNORE_RECURSIVE_OPTION => 263,
            C2RustUnnamed_6::EXCLUDE_IGNORE_OPTION => 262,
            C2RustUnnamed_6::EXCLUDE_TAG_OPTION => 264,
            C2RustUnnamed_6::EXCLUDE_CACHES_ALL_OPTION => 260,
            C2RustUnnamed_6::EXCLUDE_CACHES_UNDER_OPTION => 259,
            C2RustUnnamed_6::EXCLUDE_CACHES_OPTION => 258,
            C2RustUnnamed_6::EXCLUDE_OPTION => 261,
            C2RustUnnamed_6::NO_VERBATIM_FILES_FROM_OPTION => 277,
            C2RustUnnamed_6::VERBATIM_FILES_FROM_OPTION => 282,
            C2RustUnnamed_6::NO_UNQUOTE_OPTION => 276,
            C2RustUnnamed_6::UNQUOTE_OPTION => 275,
            C2RustUnnamed_6::NO_NULL_OPTION => 281,
            C2RustUnnamed_6::NULL_OPTION => 280,
            C2RustUnnamed_6::ADD_FILE_OPTION => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_6 {
        match value {
            278 => C2RustUnnamed_6::NO_WILDCARDS_MATCH_SLASH_OPTION,
            283 => C2RustUnnamed_6::WILDCARDS_MATCH_SLASH_OPTION,
            279 => C2RustUnnamed_6::NO_WILDCARDS_OPTION,
            284 => C2RustUnnamed_6::WILDCARDS_OPTION,
            270 => C2RustUnnamed_6::NO_IGNORE_CASE_OPTION,
            269 => C2RustUnnamed_6::IGNORE_CASE_OPTION,
            272 => C2RustUnnamed_6::NO_ANCHORED_OPTION,
            271 => C2RustUnnamed_6::ANCHORED_OPTION,
            274 => C2RustUnnamed_6::NO_RECURSION_OPTION,
            273 => C2RustUnnamed_6::RECURSION_OPTION,
            257 => C2RustUnnamed_6::EXCLUDE_BACKUPS_OPTION,
            268 => C2RustUnnamed_6::EXCLUDE_VCS_IGNORES_OPTION,
            267 => C2RustUnnamed_6::EXCLUDE_VCS_OPTION,
            266 => C2RustUnnamed_6::EXCLUDE_TAG_ALL_OPTION,
            265 => C2RustUnnamed_6::EXCLUDE_TAG_UNDER_OPTION,
            263 => C2RustUnnamed_6::EXCLUDE_IGNORE_RECURSIVE_OPTION,
            262 => C2RustUnnamed_6::EXCLUDE_IGNORE_OPTION,
            264 => C2RustUnnamed_6::EXCLUDE_TAG_OPTION,
            260 => C2RustUnnamed_6::EXCLUDE_CACHES_ALL_OPTION,
            259 => C2RustUnnamed_6::EXCLUDE_CACHES_UNDER_OPTION,
            258 => C2RustUnnamed_6::EXCLUDE_CACHES_OPTION,
            261 => C2RustUnnamed_6::EXCLUDE_OPTION,
            277 => C2RustUnnamed_6::NO_VERBATIM_FILES_FROM_OPTION,
            282 => C2RustUnnamed_6::VERBATIM_FILES_FROM_OPTION,
            276 => C2RustUnnamed_6::NO_UNQUOTE_OPTION,
            275 => C2RustUnnamed_6::UNQUOTE_OPTION,
            281 => C2RustUnnamed_6::NO_NULL_OPTION,
            280 => C2RustUnnamed_6::NULL_OPTION,
            256 => C2RustUnnamed_6::ADD_FILE_OPTION,
            _ => panic!("Invalid value for C2RustUnnamed_6: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_6 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_6 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_6 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_6 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_6 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn add(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn sub(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn mul(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn div(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_6 {
    type Output = C2RustUnnamed_6;
    fn rem(self, rhs: u32) -> C2RustUnnamed_6 {
        C2RustUnnamed_6::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum wildcards {
    default_wildcards,
    disable_wildcards,
    enable_wildcards,
}
impl wildcards {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            wildcards::default_wildcards => 0,
            wildcards::disable_wildcards => 1,
            wildcards::enable_wildcards => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> wildcards {
        match value {
            0 => wildcards::default_wildcards,
            1 => wildcards::disable_wildcards,
            2 => wildcards::enable_wildcards,
            _ => panic!("Invalid value for wildcards: {}", value),
        }
    }
}
impl AddAssign<u32> for wildcards {
    fn add_assign(&mut self, rhs: u32) {
        *self = wildcards::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for wildcards {
    fn sub_assign(&mut self, rhs: u32) {
        *self = wildcards::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for wildcards {
    fn mul_assign(&mut self, rhs: u32) {
        *self = wildcards::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for wildcards {
    fn div_assign(&mut self, rhs: u32) {
        *self = wildcards::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for wildcards {
    fn rem_assign(&mut self, rhs: u32) {
        *self = wildcards::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for wildcards {
    type Output = wildcards;
    fn add(self, rhs: u32) -> wildcards {
        wildcards::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for wildcards {
    type Output = wildcards;
    fn sub(self, rhs: u32) -> wildcards {
        wildcards::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for wildcards {
    type Output = wildcards;
    fn mul(self, rhs: u32) -> wildcards {
        wildcards::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for wildcards {
    type Output = wildcards;
    fn div(self, rhs: u32) -> wildcards {
        wildcards::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for wildcards {
    type Output = wildcards;
    fn rem(self, rhs: u32) -> wildcards {
        wildcards::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum read_file_list_state {
    file_list_success,
    file_list_end,
    file_list_zero,
    file_list_skip,
}
impl read_file_list_state {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            read_file_list_state::file_list_success => 0,
            read_file_list_state::file_list_end => 1,
            read_file_list_state::file_list_zero => 2,
            read_file_list_state::file_list_skip => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> read_file_list_state {
        match value {
            0 => read_file_list_state::file_list_success,
            1 => read_file_list_state::file_list_end,
            2 => read_file_list_state::file_list_zero,
            3 => read_file_list_state::file_list_skip,
            _ => panic!("Invalid value for read_file_list_state: {}", value),
        }
    }
}
impl AddAssign<u32> for read_file_list_state {
    fn add_assign(&mut self, rhs: u32) {
        *self = read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for read_file_list_state {
    fn sub_assign(&mut self, rhs: u32) {
        *self = read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for read_file_list_state {
    fn mul_assign(&mut self, rhs: u32) {
        *self = read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for read_file_list_state {
    fn div_assign(&mut self, rhs: u32) {
        *self = read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for read_file_list_state {
    fn rem_assign(&mut self, rhs: u32) {
        *self = read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for read_file_list_state {
    type Output = read_file_list_state;
    fn add(self, rhs: u32) -> read_file_list_state {
        read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for read_file_list_state {
    type Output = read_file_list_state;
    fn sub(self, rhs: u32) -> read_file_list_state {
        read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for read_file_list_state {
    type Output = read_file_list_state;
    fn mul(self, rhs: u32) -> read_file_list_state {
        read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for read_file_list_state {
    type Output = read_file_list_state;
    fn div(self, rhs: u32) -> read_file_list_state {
        read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for read_file_list_state {
    type Output = read_file_list_state;
    fn rem(self, rhs: u32) -> read_file_list_state {
        read_file_list_state::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_locus {
    pub source: option_source,
    pub name: *const i8,
    pub line: size_t,
    pub prev: *mut option_locus,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum option_source {
    OPTS_ENVIRON,
    OPTS_COMMAND_LINE,
    OPTS_FILE,
}
impl option_source {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            option_source::OPTS_ENVIRON => 0,
            option_source::OPTS_COMMAND_LINE => 1,
            option_source::OPTS_FILE => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> option_source {
        match value {
            0 => option_source::OPTS_ENVIRON,
            1 => option_source::OPTS_COMMAND_LINE,
            2 => option_source::OPTS_FILE,
            _ => panic!("Invalid value for option_source: {}", value),
        }
    }
}
impl AddAssign<u32> for option_source {
    fn add_assign(&mut self, rhs: u32) {
        *self = option_source::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for option_source {
    fn sub_assign(&mut self, rhs: u32) {
        *self = option_source::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for option_source {
    fn mul_assign(&mut self, rhs: u32) {
        *self = option_source::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for option_source {
    fn div_assign(&mut self, rhs: u32) {
        *self = option_source::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for option_source {
    fn rem_assign(&mut self, rhs: u32) {
        *self = option_source::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for option_source {
    type Output = option_source;
    fn add(self, rhs: u32) -> option_source {
        option_source::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for option_source {
    type Output = option_source;
    fn sub(self, rhs: u32) -> option_source {
        option_source::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for option_source {
    type Output = option_source;
    fn mul(self, rhs: u32) -> option_source {
        option_source::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for option_source {
    type Output = option_source;
    fn div(self, rhs: u32) -> option_source {
        option_source::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for option_source {
    type Output = option_source;
    fn rem(self, rhs: u32) -> option_source {
        option_source::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_id_list {
    pub next: *mut file_id_list,
    pub ino: ino_t,
    pub dev: dev_t,
    pub from_file: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_args {
    pub loc: *mut option_locus,
    pub textual_date: *mut textual_date,
    pub o_option: bool,
    pub pax_option: bool,
    pub compress_autodetect: bool,
    pub backup_suffix_string: *const i8,
    pub version_control_string: *const i8,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
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
static mut names_options: [argp_option; 35] = [
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"Local file name selection:\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRH_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"add-file\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::ADD_FILE_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"add given FILE to the archive (useful if its name starts with a dash)\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"directory\0" as *const u8 as *const i8,
            key: 'C' as i32,
            arg: b"DIR\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"change to directory DIR\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"files-from\0" as *const u8 as *const i8,
            key: 'T' as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"get names to extract or create from FILE\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"null\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NULL_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"-T reads null-terminated names; implies --verbatim-files-from\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-null\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_NULL_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"disable the effect of the previous --null option\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"unquote\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::UNQUOTE_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"unquote input file or member names (default)\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-unquote\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_UNQUOTE_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"do not unquote input file or member names\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"verbatim-files-from\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::VERBATIM_FILES_FROM_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"-T reads file names verbatim (no escape or option handling)\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-verbatim-files-from\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_VERBATIM_FILES_FROM_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"-T treats file names starting with dash as options (default)\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_OPTION as i32,
            arg: b"PATTERN\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"exclude files, given as a PATTERN\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-from\0" as *const u8 as *const i8,
            key: 'X' as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"exclude patterns listed in FILE\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_CACHES_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"exclude contents of directories containing CACHEDIR.TAG, except for the tag file itself\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches-under\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_CACHES_UNDER_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"exclude everything under directories containing CACHEDIR.TAG\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-caches-all\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_CACHES_ALL_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"exclude directories containing CACHEDIR.TAG\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_TAG_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"exclude contents of directories containing FILE, except for FILE itself\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-ignore\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_IGNORE_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"read exclude patterns for each directory from FILE, if it exists\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-ignore-recursive\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_IGNORE_RECURSIVE_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"read exclude patterns for each directory and its subdirectories from FILE, if it exists\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag-under\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_TAG_UNDER_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"exclude everything under directories containing FILE\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-tag-all\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_TAG_ALL_OPTION as i32,
            arg: b"FILE\0" as *const u8 as *const i8,
            flags: 0 as i32,
            doc: b"exclude directories containing FILE\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-vcs\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_VCS_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"exclude version control system directories\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-vcs-ignores\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_VCS_IGNORES_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"read exclude patterns from the VCS ignore files\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"exclude-backups\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::EXCLUDE_BACKUPS_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"exclude backup and lock files\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"recursion\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::RECURSION_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"recurse into directories (default)\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-recursion\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_RECURSION_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"avoid descending automatically in directories\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_LOCAL as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0 as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"File name matching options (affect both exclude and include patterns):\0"
                as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRH_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"anchored\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::ANCHORED_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"patterns match file name start\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-anchored\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_ANCHORED_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"patterns match after any '/' (default for exclusion)\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"ignore-case\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::IGNORE_CASE_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"ignore case\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-ignore-case\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_IGNORE_CASE_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"case sensitive matching (default)\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"wildcards\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::WILDCARDS_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"use wildcards (default for exclusion)\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-wildcards\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_WILDCARDS_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"verbatim string matching\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"wildcards-match-slash\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::WILDCARDS_MATCH_SLASH_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"wildcards match '/' (default for exclusion)\0" as *const u8
                as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: b"no-wildcards-match-slash\0" as *const u8 as *const i8,
            key: C2RustUnnamed_6::NO_WILDCARDS_MATCH_SLASH_OPTION as i32,
            arg: 0 as *const i8,
            flags: 0 as i32,
            doc: b"wildcards do not match '/'\0" as *const u8 as *const i8,
            group: C2RustUnnamed_7::GRID_MATCH as i32,
        };
        init
    },
    {
        let mut init = argp_option {
            name: 0 as *const i8,
            key: 0,
            arg: 0 as *const i8,
            flags: 0,
            doc: 0 as *const i8,
            group: 0,
        };
        init
    },
];
unsafe extern "C" fn file_selection_option(mut key: i32) -> *const argp_option {
    let mut p: *mut argp_option = 0 as *mut argp_option;
    p = names_options.as_mut_ptr();
    while !(((*p).name).is_null() && (*p).key == 0 as i32 && ((*p).doc).is_null()) {
        if (*p).key == key {
            return p;
        }
        p = p.offset(1);
        p;
    }
    return 0 as *const argp_option;
}
unsafe extern "C" fn file_selection_option_name(mut key: i32) -> *const i8 {
    let mut opt: *const argp_option = file_selection_option(key);
    return if !opt.is_null() { (*opt).name } else { 0 as *const i8 };
}
unsafe extern "C" fn is_file_selection_option(mut key: i32) -> bool {
    return !(file_selection_option(key)).is_null();
}
static mut filename_terminator: i8 = '\n' as i32 as i8;
static mut verbatim_files_from_option: bool = false;
unsafe extern "C" fn names_parse_opt(
    mut key: i32,
    mut arg: *mut i8,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        67 => {
            name_add_dir(arg);
        }
        84 => {
            name_add_file(arg);
        }
        256 => {
            name_add_name(arg);
        }
        16777221 => {
            let mut args: *mut tar_args = (*state).input as *mut tar_args;
            if (*(*args).loc).source as u32 == option_source::OPTS_FILE as i32 as u32 {
                error(
                    0 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"%s:%lu: unrecognized option\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    (*(*args).loc).name,
                    (*(*args).loc).line,
                );
                set_exit_status(2 as i32);
            }
            return 7 as i32;
        }
        _ => {
            if is_file_selection_option(key) {
                name_add_option(key, arg);
            } else {
                return 7 as i32
            }
        }
    }
    return 0 as i32;
}
static mut wildcards: wildcards = wildcards::default_wildcards;
static mut matching_flags: i32 = 0 as i32;
static mut include_anchored: i32 = (1 as i32) << 30 as i32;
static mut vcs_file_table: [*const i8; 22] = [
    b"CVS\0" as *const u8 as *const i8,
    b".cvsignore\0" as *const u8 as *const i8,
    b"RCS\0" as *const u8 as *const i8,
    b"SCCS\0" as *const u8 as *const i8,
    b".svn\0" as *const u8 as *const i8,
    b".git\0" as *const u8 as *const i8,
    b".gitignore\0" as *const u8 as *const i8,
    b".gitattributes\0" as *const u8 as *const i8,
    b".gitmodules\0" as *const u8 as *const i8,
    b".arch-ids\0" as *const u8 as *const i8,
    b"{arch}\0" as *const u8 as *const i8,
    b"=RELEASE-ID\0" as *const u8 as *const i8,
    b"=meta-update\0" as *const u8 as *const i8,
    b"=update\0" as *const u8 as *const i8,
    b".bzr\0" as *const u8 as *const i8,
    b".bzrignore\0" as *const u8 as *const i8,
    b".bzrtags\0" as *const u8 as *const i8,
    b".hg\0" as *const u8 as *const i8,
    b".hgignore\0" as *const u8 as *const i8,
    b".hgtags\0" as *const u8 as *const i8,
    b"_darcs\0" as *const u8 as *const i8,
    0 as *const i8,
];
static mut backup_file_table: [*const i8; 4] = [
    b".#*\0" as *const u8 as *const i8,
    b"*~\0" as *const u8 as *const i8,
    b"#*#\0" as *const u8 as *const i8,
    0 as *const i8,
];
unsafe extern "C" fn add_exclude_array(mut fv: *const *const i8, mut opts: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while !(*fv.offset(i as isize)).is_null() {
        add_exclude(excluded, *fv.offset(i as isize), opts);
        i += 1;
        i;
    }
}
unsafe extern "C" fn handle_file_selection_option(mut key: i32, mut arg: *const i8) {
    match key {
        257 => {
            add_exclude_array(backup_file_table.as_ptr(), (1 as i32) << 28 as i32);
        }
        261 => {
            add_exclude(
                excluded,
                arg,
                (if wildcards as u32 != wildcards::disable_wildcards as i32 as u32 {
                    (1 as i32) << 28 as i32
                } else {
                    0 as i32
                }) | matching_flags | recursion_option,
            );
        }
        258 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const i8,
                exclusion_tag_type::exclusion_tag_contents,
                Some(cachedir_file_p as unsafe extern "C" fn(i32) -> bool),
            );
        }
        259 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const i8,
                exclusion_tag_type::exclusion_tag_under,
                Some(cachedir_file_p as unsafe extern "C" fn(i32) -> bool),
            );
        }
        260 => {
            add_exclusion_tag(
                b"CACHEDIR.TAG\0" as *const u8 as *const i8,
                exclusion_tag_type::exclusion_tag_all,
                Some(cachedir_file_p as unsafe extern "C" fn(i32) -> bool),
            );
        }
        262 => {
            excfile_add(arg, 0x2 as i32);
        }
        263 => {
            excfile_add(arg, 0x1 as i32);
        }
        264 => {
            add_exclusion_tag(arg, exclusion_tag_type::exclusion_tag_contents, None);
        }
        265 => {
            add_exclusion_tag(arg, exclusion_tag_type::exclusion_tag_under, None);
        }
        266 => {
            add_exclusion_tag(arg, exclusion_tag_type::exclusion_tag_all, None);
        }
        267 => {
            add_exclude_array(vcs_file_table.as_ptr(), 0 as i32);
        }
        268 => {
            exclude_vcs_ignores();
        }
        273 => {
            recursion_option = (1 as i32) << 3 as i32;
        }
        274 => {
            recursion_option = 0 as i32;
        }
        275 => {
            unquote_option = 1 as i32 != 0;
        }
        276 => {
            unquote_option = 0 as i32 != 0;
        }
        280 => {
            filename_terminator = '\0' as i32 as i8;
            verbatim_files_from_option = 1 as i32 != 0;
        }
        281 => {
            filename_terminator = '\n' as i32 as i8;
            verbatim_files_from_option = 0 as i32 != 0;
        }
        88 => {
            if add_exclude_file(
                Some(
                    add_exclude
                        as unsafe extern "C" fn(*mut exclude, *const i8, i32) -> (),
                ),
                excluded,
                arg,
                (if wildcards as u32 != wildcards::disable_wildcards as i32 as u32 {
                    (1 as i32) << 28 as i32
                } else {
                    0 as i32
                }) | matching_flags | recursion_option,
                '\n' as i32 as i8,
            ) != 0 as i32
            {
                let mut e: i32 = *__errno_location();
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                error(
                    0 as i32,
                    e,
                    b"%s\0" as *const u8 as *const i8,
                    quotearg_colon(arg),
                );
                fatal_exit();
            }
        }
        271 => {
            matching_flags |= (1 as i32) << 30 as i32;
        }
        272 => {
            include_anchored = 0 as i32;
            matching_flags &= !((1 as i32) << 30 as i32);
        }
        269 => {
            matching_flags |= (1 as i32) << 4 as i32;
        }
        270 => {
            matching_flags &= !((1 as i32) << 4 as i32);
        }
        284 => {
            wildcards = wildcards::enable_wildcards;
        }
        279 => {
            wildcards = wildcards::disable_wildcards;
        }
        283 => {
            matching_flags &= !((1 as i32) << 0 as i32);
        }
        278 => {
            matching_flags |= (1 as i32) << 0 as i32;
        }
        282 => {
            verbatim_files_from_option = 1 as i32 != 0;
        }
        277 => {
            verbatim_files_from_option = 0 as i32 != 0;
        }
        _ => {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                b"unhandled positional option %d\0" as *const u8 as *const i8,
                key,
            );
            fatal_exit();
        }
    };
}
#[no_mangle]
pub static mut names_argp: argp = unsafe {
    {
        let mut init = argp {
            options: names_options.as_ptr() as *mut _,
            parser: Some(
                names_parse_opt
                    as unsafe extern "C" fn(i32, *mut i8, *mut argp_state) -> error_t,
            ),
            args_doc: 0 as *const i8,
            doc: 0 as *const i8,
            children: 0 as *const argp_child,
            help_filter: None,
            argp_domain: 0 as *const i8,
        };
        init
    }
};
static mut cached_uname: *mut i8 = 0 as *const i8 as *mut i8;
static mut cached_gname: *mut i8 = 0 as *const i8 as *mut i8;
static mut cached_uid: uid_t = 0;
static mut cached_gid: gid_t = 0;
static mut cached_no_such_uname: *mut i8 = 0 as *const i8 as *mut i8;
static mut cached_no_such_gname: *mut i8 = 0 as *const i8 as *mut i8;
static mut cached_no_such_uid: uid_t = 0;
static mut cached_no_such_gid: gid_t = 0;
#[no_mangle]
pub unsafe extern "C" fn uid_to_uname(mut uid: uid_t, mut uname: *mut *mut i8) {
    let mut passwd: *mut passwd = 0 as *mut passwd;
    if uid != 0 as i32 as u32 && uid == cached_no_such_uid {
        *uname = xstrdup(b"\0" as *const u8 as *const i8);
        return;
    }
    if cached_uname.is_null() || uid != cached_uid {
        passwd = getpwuid(uid);
        if !passwd.is_null() {
            cached_uid = uid;
            assign_string(&mut cached_uname, (*passwd).pw_name);
        } else {
            cached_no_such_uid = uid;
            *uname = xstrdup(b"\0" as *const u8 as *const i8);
            return;
        }
    }
    *uname = xstrdup(cached_uname);
}
#[no_mangle]
pub unsafe extern "C" fn gid_to_gname(mut gid: gid_t, mut gname: *mut *mut i8) {
    let mut group: *mut group = 0 as *mut group;
    if gid != 0 as i32 as u32 && gid == cached_no_such_gid {
        *gname = xstrdup(b"\0" as *const u8 as *const i8);
        return;
    }
    if cached_gname.is_null() || gid != cached_gid {
        group = getgrgid(gid);
        if !group.is_null() {
            cached_gid = gid;
            assign_string(&mut cached_gname, (*group).gr_name);
        } else {
            cached_no_such_gid = gid;
            *gname = xstrdup(b"\0" as *const u8 as *const i8);
            return;
        }
    }
    *gname = xstrdup(cached_gname);
}
#[no_mangle]
pub unsafe extern "C" fn uname_to_uid(
    mut uname: *const i8,
    mut uidp: *mut uid_t,
) -> i32 {
    let mut passwd: *mut passwd = 0 as *mut passwd;
    if !cached_no_such_uname.is_null() && strcmp(uname, cached_no_such_uname) == 0 as i32
    {
        return 0 as i32;
    }
    if cached_uname.is_null()
        || *uname.offset(0 as i32 as isize) as i32
            != *cached_uname.offset(0 as i32 as isize) as i32
        || strcmp(uname, cached_uname) != 0 as i32
    {
        passwd = getpwnam(uname);
        if !passwd.is_null() {
            cached_uid = (*passwd).pw_uid;
            assign_string(&mut cached_uname, (*passwd).pw_name);
        } else {
            assign_string(&mut cached_no_such_uname, uname);
            return 0 as i32;
        }
    }
    *uidp = cached_uid;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gname_to_gid(
    mut gname: *const i8,
    mut gidp: *mut gid_t,
) -> i32 {
    let mut group: *mut group = 0 as *mut group;
    if !cached_no_such_gname.is_null() && strcmp(gname, cached_no_such_gname) == 0 as i32
    {
        return 0 as i32;
    }
    if cached_gname.is_null()
        || *gname.offset(0 as i32 as isize) as i32
            != *cached_gname.offset(0 as i32 as isize) as i32
        || strcmp(gname, cached_gname) != 0 as i32
    {
        group = getgrnam(gname);
        if !group.is_null() {
            cached_gid = (*group).gr_gid;
            assign_string(&mut cached_gname, gname);
        } else {
            assign_string(&mut cached_no_such_gname, gname);
            return 0 as i32;
        }
    }
    *gidp = cached_gid;
    return 1 as i32;
}
unsafe extern "C" fn make_name(mut file_name: *const i8) -> *mut name {
    let mut p: *mut name = xzalloc(::core::mem::size_of::<name>() as u64) as *mut name;
    if file_name.is_null() {
        file_name = b"\0" as *const u8 as *const i8;
    }
    (*p).name = xstrdup(file_name);
    (*p).length = strlen((*p).name);
    return p;
}
unsafe extern "C" fn free_name(mut p: *mut name) {
    if !p.is_null() {
        rpl_free((*p).name as *mut libc::c_void);
        rpl_free((*p).caname as *mut libc::c_void);
        rpl_free(p as *mut libc::c_void);
    }
}
static mut namelist: *mut name = 0 as *const name as *mut name;
static mut nametail: *mut name = 0 as *const name as *mut name;
static mut name_head: *mut name_elt = 0 as *const name_elt as *mut name_elt;
#[no_mangle]
pub static mut filename_args: files_count = files_count::FILES_NONE;
unsafe extern "C" fn name_elt_alloc() -> *mut name_elt {
    let mut elt: *mut name_elt = 0 as *mut name_elt;
    elt = xmalloc(::core::mem::size_of::<name_elt>() as u64) as *mut name_elt;
    if name_head.is_null() {
        name_head = elt;
        (*name_head).next = 0 as *mut name_elt;
        (*name_head).prev = (*name_head).next;
        (*name_head).type_0 = nelt_type::NELT_NOOP;
        elt = xmalloc(::core::mem::size_of::<name_elt>() as u64) as *mut name_elt;
    }
    (*elt).prev = (*name_head).prev;
    if !((*name_head).prev).is_null() {
        (*(*name_head).prev).next = elt;
    }
    (*elt).next = name_head;
    (*name_head).prev = elt;
    return elt;
}
unsafe extern "C" fn name_list_adjust() {
    if !name_head.is_null() {
        while !((*name_head).prev).is_null() {
            name_head = (*name_head).prev;
        }
    }
}
#[no_mangle]
pub static mut unconsumed_option_tail: *mut name_elt = 0 as *const name_elt
    as *mut name_elt;
unsafe extern "C" fn unconsumed_option_push(mut elt: *mut name_elt) {
    (*elt).next = 0 as *mut name_elt;
    (*elt).prev = unconsumed_option_tail;
    if !unconsumed_option_tail.is_null() {
        (*unconsumed_option_tail).next = elt;
    }
    unconsumed_option_tail = elt;
}
unsafe extern "C" fn unconsumed_option_free() {
    while !unconsumed_option_tail.is_null() {
        let mut elt: *mut name_elt = unconsumed_option_tail;
        unconsumed_option_tail = (*unconsumed_option_tail).prev;
        rpl_free(elt as *mut libc::c_void);
    }
}
unsafe extern "C" fn unconsumed_option_report() {
    if !unconsumed_option_tail.is_null() {
        let mut elt: *mut name_elt = 0 as *mut name_elt;
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"The following options were used after any non-optional arguments in archive create or update mode.  These options are positional and affect only arguments that follow them.  Please, rearrange them properly.\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        exit_status = 2 as i32;
        elt = unconsumed_option_tail;
        while !((*elt).prev).is_null() {
            elt = (*elt).prev;
        }
        while !elt.is_null() {
            match (*elt).type_0 as u32 {
                1 => {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"-C %s has no effect\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        quote((*elt).v.name),
                    );
                    exit_status = 2 as i32;
                }
                4 => {
                    if !((*elt).v.opt.arg).is_null() {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"--%s %s has no effect\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            file_selection_option_name((*elt).v.opt.rpl_option),
                            quote((*elt).v.opt.arg),
                        );
                        exit_status = 2 as i32;
                    } else {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        error(
                            0 as i32,
                            0 as i32,
                            dcgettext(
                                0 as *const i8,
                                b"--%s has no effect\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            file_selection_option_name((*elt).v.opt.rpl_option),
                        );
                        exit_status = 2 as i32;
                    }
                }
                _ => {}
            }
            elt = (*elt).next;
        }
        unconsumed_option_free();
    }
}
unsafe extern "C" fn name_list_advance() {
    let mut elt: *mut name_elt = name_head;
    name_head = (*elt).next;
    if !name_head.is_null() {
        (*name_head).prev = 0 as *mut name_elt;
    }
    if (*elt).type_0 as u32 == nelt_type::NELT_OPTION as i32 as u32
        || (*elt).type_0 as u32 == nelt_type::NELT_CHDIR as i32 as u32
    {
        if subcommand_option as u32 == subcommand::CREATE_SUBCOMMAND as i32 as u32
            || subcommand_option as u32 == subcommand::UPDATE_SUBCOMMAND as i32 as u32
        {
            unconsumed_option_push(elt);
        }
    } else {
        if (*elt).type_0 as u32 != nelt_type::NELT_NOOP as i32 as u32 {
            unconsumed_option_free();
        }
        rpl_free(elt as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn name_add_name(mut name: *const i8) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = nelt_type::NELT_NAME;
    (*ep).v.name = name;
    match filename_args as u32 {
        0 => {
            filename_args = files_count::FILES_ONE;
        }
        1 => {
            filename_args = files_count::FILES_MANY;
        }
        _ => {}
    };
}
unsafe extern "C" fn name_add_option(mut rpl_option: i32, mut arg: *const i8) {
    let mut elt: *mut name_elt = name_elt_alloc();
    (*elt).type_0 = nelt_type::NELT_OPTION;
    (*elt).v.opt.rpl_option = rpl_option;
    (*elt).v.opt.arg = arg;
}
unsafe extern "C" fn name_add_dir(mut name: *const i8) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = nelt_type::NELT_CHDIR;
    (*ep).v.name = name;
}
unsafe extern "C" fn name_add_file(mut name: *const i8) {
    let mut ep: *mut name_elt = name_elt_alloc();
    (*ep).type_0 = nelt_type::NELT_FILE;
    (*ep).v.file.name = name;
    (*ep).v.file.line = 0 as i32 as size_t;
    (*ep).v.file.fp = 0 as *mut FILE;
    filename_args = files_count::FILES_MANY;
}
static mut name_buffer: *mut i8 = 0 as *const i8 as *mut i8;
static mut name_buffer_length: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn name_init() {
    name_buffer = xmalloc((100 as i32 + 2 as i32) as size_t) as *mut i8;
    name_buffer_length = 100 as i32 as size_t;
    name_list_adjust();
}
#[no_mangle]
pub unsafe extern "C" fn name_term() {
    rpl_free(name_buffer as *mut libc::c_void);
}
static mut file_id_list: *mut file_id_list = 0 as *const file_id_list
    as *mut file_id_list;
unsafe extern "C" fn file_list_name() -> *const i8 {
    let mut elt: *mut name_elt = 0 as *mut name_elt;
    elt = name_head;
    while !elt.is_null() {
        if (*elt).type_0 as u32 == nelt_type::NELT_FILE as i32 as u32
            && !((*elt).v.file.fp).is_null()
        {
            return (*elt).v.file.name;
        }
        elt = (*elt).next;
    }
    return dcgettext(
        0 as *const i8,
        b"command line\0" as *const u8 as *const i8,
        5 as i32,
    );
}
unsafe extern "C" fn add_file_id(mut filename: *const i8) -> i32 {
    let mut p: *mut file_id_list = 0 as *mut file_id_list;
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
    let mut reading_from: *const i8 = 0 as *const i8;
    if stat(filename, &mut st) != 0 {
        stat_fatal(filename);
    }
    reading_from = file_list_name();
    p = file_id_list;
    while !p.is_null() {
        if (*p).ino == st.st_ino && (*p).dev == st.st_dev {
            let mut oldc: i32 = set_char_quoting(
                0 as *mut quoting_options,
                ':' as i32 as i8,
                1 as i32,
            );
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: file list requested from %s already read from %s\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n(0 as i32, filename),
                reading_from,
                (*p).from_file,
            );
            exit_status = 2 as i32;
            set_char_quoting(0 as *mut quoting_options, ':' as i32 as i8, oldc);
            return 1 as i32;
        }
        p = (*p).next;
    }
    p = xmalloc(::core::mem::size_of::<file_id_list>() as u64) as *mut file_id_list;
    (*p).next = file_id_list;
    (*p).ino = st.st_ino;
    (*p).dev = st.st_dev;
    (*p).from_file = reading_from;
    file_id_list = p;
    return 0 as i32;
}
unsafe extern "C" fn chopslash(mut str: *mut i8) {
    let mut p: *mut i8 = str.offset(strlen(str) as isize).offset(-(1 as i32 as isize));
    while p > str && *p as i32 == '/' as i32 {
        let fresh1 = p;
        p = p.offset(-1);
        *fresh1 = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn read_name_from_file(
    mut ent: *mut name_elt,
) -> read_file_list_state {
    let mut c: i32 = 0;
    let mut counter: size_t = 0 as i32 as size_t;
    let mut fp: *mut FILE = (*ent).v.file.fp;
    let mut term: i32 = (*ent).v.file.term;
    (*ent).v.file.line = ((*ent).v.file.line).wrapping_add(1);
    (*ent).v.file.line;
    c = getc_unlocked(fp);
    while c != -(1 as i32) && c != term {
        if counter == name_buffer_length {
            name_buffer = x2realloc(
                name_buffer as *mut libc::c_void,
                &mut name_buffer_length,
            ) as *mut i8;
        }
        let fresh2 = counter;
        counter = counter.wrapping_add(1);
        *name_buffer.offset(fresh2 as isize) = c as i8;
        if c == 0 as i32 {
            return read_file_list_state::file_list_zero;
        }
        c = getc_unlocked(fp);
    }
    if counter == 0 as i32 as u64 && c != -(1 as i32) {
        return read_file_list_state::file_list_skip;
    }
    if counter == name_buffer_length {
        name_buffer = x2realloc(
            name_buffer as *mut libc::c_void,
            &mut name_buffer_length,
        ) as *mut i8;
    }
    *name_buffer.offset(counter as isize) = 0 as i32 as i8;
    chopslash(name_buffer);
    return read_file_list_state::from_libc_c_uint(
        (if counter == 0 as i32 as u64 && c == -(1 as i32) {
            read_file_list_state::file_list_end as i32
        } else {
            read_file_list_state::file_list_success as i32
        }) as u32,
    );
}
unsafe extern "C" fn handle_option(mut str: *const i8, mut ent: *const name_elt) -> i32 {
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut i8,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const i8,
        ws_comment: 0 as *const i8,
        ws_escape: [0 as *const i8; 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const i8,
        ws_envbuf: 0 as *mut *mut i8,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const i8,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut i8,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_lvl: 0,
    };
    let mut i: i32 = 0;
    let mut loc: option_locus = option_locus {
        source: option_source::OPTS_ENVIRON,
        name: 0 as *const i8,
        line: 0,
        prev: 0 as *mut option_locus,
    };
    while *str as i32 != 0
        && *(*__ctype_b_loc()).offset(*str as i32 as isize) as i32
            & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
    {
        str = str.offset(1);
        str;
    }
    if *str as i32 != '-' as i32 {
        return 1 as i32;
    }
    ws.ws_offs = 1 as i32 as size_t;
    if wordsplit(
        str,
        &mut ws,
        (0x40 as i32 | 0x4 as i32 | (0x200 as i32 | 0x400 as i32) | 0x800 as i32
            | 0x2000000 as i32 | 0x2 as i32) as u32,
    ) != 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"cannot split string '%s': %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            str,
            wordsplit_strerror(&mut ws),
        );
        fatal_exit();
    }
    let ref mut fresh3 = *(ws.ws_wordv).offset(0 as i32 as isize);
    *fresh3 = program_name as *mut i8;
    loc.source = option_source::OPTS_FILE;
    loc.name = (*ent).v.file.name;
    loc.line = (*ent).v.file.line;
    more_options((ws.ws_wordc).wrapping_add(ws.ws_offs) as i32, ws.ws_wordv, &mut loc);
    i = 0 as i32;
    while (i as u64) < (ws.ws_wordc).wrapping_add(ws.ws_offs) {
        let ref mut fresh4 = *(ws.ws_wordv).offset(i as isize);
        *fresh4 = 0 as *mut i8;
        i += 1;
        i;
    }
    wordsplit_free(&mut ws);
    return 0 as i32;
}
unsafe extern "C" fn read_next_name(
    mut ent: *mut name_elt,
    mut ret: *mut name_elt,
) -> i32 {
    if ((*ent).v.file.fp).is_null() {
        if strcmp((*ent).v.file.name, b"-\0" as *const u8 as *const i8) == 0 {
            request_stdin(b"-T\0" as *const u8 as *const i8);
            (*ent).v.file.fp = stdin;
        } else {
            if add_file_id((*ent).v.file.name) != 0 {
                name_list_advance();
                return 1 as i32;
            }
            (*ent).v.file.fp = fopen(
                (*ent).v.file.name,
                b"r\0" as *const u8 as *const i8,
            );
            if ((*ent).v.file.fp).is_null() {
                open_fatal((*ent).v.file.name);
            }
        }
        (*ent).v.file.term = filename_terminator as i32;
        (*ent).v.file.verbatim = verbatim_files_from_option;
    }
    loop {
        match read_name_from_file(ent) as u32 {
            2 => {
                if warning_option & 0x200 as i32 != 0 {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        b"%s: file name read contains nul character\0" as *const u8
                            as *const i8,
                        quotearg_colon((*ent).v.file.name),
                    );
                }
                (*ent).v.file.term = 0 as i32;
            }
            0 => {}
            1 => {
                if strcmp((*ent).v.file.name, b"-\0" as *const u8 as *const i8) != 0 {
                    fclose((*ent).v.file.fp);
                }
                (*ent).v.file.fp = 0 as *mut FILE;
                name_list_advance();
                return 1 as i32;
            }
            3 | _ => {
                continue;
            }
        }
        if !(*ent).v.file.verbatim {
            if unquote_option {
                unquote_string(name_buffer);
            }
            if handle_option(name_buffer, ent) == 0 as i32 {
                name_list_adjust();
                return 1 as i32;
            }
        }
        (*ret).type_0 = nelt_type::NELT_NAME;
        (*ret).v.name = name_buffer;
        return 0 as i32;
    };
}
unsafe extern "C" fn copy_name(mut ep: *mut name_elt) {
    let mut source: *const i8 = 0 as *const i8;
    let mut source_len: size_t = 0;
    source = (*ep).v.name;
    source_len = strlen(source);
    while name_buffer_length <= source_len {
        name_buffer = x2realloc(
            name_buffer as *mut libc::c_void,
            &mut name_buffer_length,
        ) as *mut i8;
    }
    strcpy(name_buffer, source);
    chopslash(name_buffer);
}
unsafe extern "C" fn name_next_elt(mut change_dirs: i32) -> *mut name_elt {
    static mut entry: name_elt = name_elt {
        next: 0 as *const name_elt as *mut name_elt,
        prev: 0 as *const name_elt as *mut name_elt,
        type_0: nelt_type::NELT_NAME,
        v: C2RustUnnamed_3 {
            name: 0 as *const i8,
        },
    };
    let mut ep: *mut name_elt = 0 as *mut name_elt;
    loop {
        ep = name_head;
        if ep.is_null() {
            break;
        }
        match (*ep).type_0 as u32 {
            3 => {
                name_list_advance();
                continue;
            }
            2 => {
                if read_next_name(ep, &mut entry) == 0 as i32 {
                    return &mut entry;
                }
                continue;
            }
            1 => {
                if change_dirs != 0 {
                    chdir_do(chdir_arg(xstrdup((*ep).v.name)));
                    name_list_advance();
                    continue;
                }
            }
            0 => {}
            4 => {
                handle_file_selection_option((*ep).v.opt.rpl_option, (*ep).v.opt.arg);
                name_list_advance();
                continue;
            }
            _ => {
                continue;
            }
        }
        copy_name(ep);
        if unquote_option {
            unquote_string(name_buffer);
        }
        entry.type_0 = (*ep).type_0;
        entry.v.name = name_buffer;
        name_list_advance();
        return &mut entry;
    }
    unconsumed_option_report();
    return 0 as *mut name_elt;
}
#[no_mangle]
pub unsafe extern "C" fn name_next(mut change_dirs: i32) -> *const i8 {
    let mut nelt: *mut name_elt = name_next_elt(change_dirs);
    return if !nelt.is_null() { (*nelt).v.name } else { 0 as *const i8 };
}
#[no_mangle]
pub unsafe extern "C" fn name_gather() {
    static mut buffer: *mut name = 0 as *const name as *mut name;
    let mut ep: *mut name_elt = 0 as *mut name_elt;
    if same_order_option {
        static mut change_dir: i32 = 0;
        loop {
            ep = name_next_elt(0 as i32);
            if !(!ep.is_null()
                && (*ep).type_0 as u32 == nelt_type::NELT_CHDIR as i32 as u32)
            {
                break;
            }
            change_dir = chdir_arg(xstrdup((*ep).v.name));
        }
        if !ep.is_null() {
            free_name(buffer);
            buffer = make_name((*ep).v.name);
            (*buffer).change_dir = change_dir;
            (*buffer).next = 0 as *mut name;
            (*buffer).found_count = 0 as i32 as uintmax_t;
            (*buffer).matching_flags = (if wildcards as u32
                == wildcards::enable_wildcards as i32 as u32
            {
                (1 as i32) << 28 as i32
            } else {
                0 as i32
            }) | include_anchored | matching_flags | recursion_option;
            (*buffer).directory = 0 as *mut directory;
            (*buffer).parent = 0 as *mut name;
            (*buffer).cmdline = 1 as i32 != 0;
            nametail = buffer;
            namelist = nametail;
        } else if change_dir != 0 {
            addname(0 as *const i8, change_dir, 0 as i32 != 0, 0 as *mut name);
        }
    } else {
        let mut change_dir_0: i32 = 0 as i32;
        loop {
            let mut change_dir0: i32 = change_dir_0;
            loop {
                ep = name_next_elt(0 as i32);
                if !(!ep.is_null()
                    && (*ep).type_0 as u32 == nelt_type::NELT_CHDIR as i32 as u32)
                {
                    break;
                }
                change_dir_0 = chdir_arg(xstrdup((*ep).v.name));
            }
            if !ep.is_null() {
                addname((*ep).v.name, change_dir_0, 1 as i32 != 0, 0 as *mut name);
            } else {
                if change_dir_0 != change_dir0 {
                    addname(0 as *const i8, change_dir_0, 0 as i32 != 0, 0 as *mut name);
                }
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn addname(
    mut string: *const i8,
    mut change_dir: i32,
    mut cmdline: bool,
    mut parent: *mut name,
) -> *mut name {
    let mut name: *mut name = make_name(string);
    (*name).prev = nametail;
    (*name).next = 0 as *mut name;
    (*name).found_count = 0 as i32 as uintmax_t;
    (*name).matching_flags = (if wildcards as u32
        == wildcards::enable_wildcards as i32 as u32
    {
        (1 as i32) << 28 as i32
    } else {
        0 as i32
    }) | include_anchored | matching_flags | recursion_option;
    (*name).change_dir = change_dir;
    (*name).directory = 0 as *mut directory;
    (*name).parent = parent;
    (*name).cmdline = cmdline;
    if !nametail.is_null() {
        (*nametail).next = name;
    } else {
        namelist = name;
    }
    nametail = name;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn add_starting_file(mut file_name: *const i8) {
    let mut name: *mut name = make_name(file_name);
    if starting_file_option {
        let mut head: *mut name = namelist;
        remname(head);
        free_name(head);
    }
    (*name).prev = 0 as *mut name;
    (*name).next = namelist;
    namelist = name;
    if nametail.is_null() {
        nametail = namelist;
    }
    (*name).found_count = 0 as i32 as uintmax_t;
    (*name).matching_flags = (if wildcards as u32
        == wildcards::enable_wildcards as i32 as u32
    {
        (1 as i32) << 28 as i32
    } else {
        0 as i32
    }) | include_anchored | matching_flags | recursion_option;
    (*name).change_dir = 0 as i32;
    (*name).directory = 0 as *mut directory;
    (*name).parent = 0 as *mut name;
    (*name).cmdline = 1 as i32 != 0;
    starting_file_option = 1 as i32 != 0;
}
unsafe extern "C" fn namelist_match(
    mut file_name: *const i8,
    mut length: size_t,
) -> *mut name {
    let mut p: *mut name = 0 as *mut name;
    p = namelist;
    while !p.is_null() {
        if *((*p).name).offset(0 as i32 as isize) as i32 != 0
            && exclude_fnmatch((*p).name, file_name, (*p).matching_flags) as i32 != 0
        {
            return p;
        }
        p = (*p).next;
    }
    return 0 as *mut name;
}
#[no_mangle]
pub unsafe extern "C" fn remname(mut name: *mut name) {
    let mut p: *mut name = 0 as *mut name;
    p = (*name).prev;
    if !p.is_null() {
        (*p).next = (*name).next;
    } else {
        namelist = (*name).next;
    }
    p = (*name).next;
    if !p.is_null() {
        (*p).prev = (*name).prev;
    } else {
        nametail = (*name).prev;
    };
}
#[no_mangle]
pub unsafe extern "C" fn name_match(mut file_name: *const i8) -> bool {
    let mut length: size_t = strlen(file_name);
    loop {
        let mut cursor: *mut name = namelist;
        if cursor.is_null() {
            return 1 as i32 != 0;
        }
        if *((*cursor).name).offset(0 as i32 as isize) as i32 == 0 as i32 {
            chdir_do((*cursor).change_dir);
            namelist = 0 as *mut name;
            nametail = 0 as *mut name;
            return 1 as i32 != 0;
        }
        cursor = namelist_match(file_name, length);
        if starting_file_option {
            if cursor == namelist {
                starting_file_option = 0 as i32 != 0;
            } else {
                cursor = 0 as *mut name;
            }
        }
        if !cursor.is_null() {
            if !(*file_name.offset((*cursor).length as isize) as i32 == '/' as i32
                && recursion_option != 0) || (*cursor).found_count == 0 as i32 as u64
            {
                (*cursor).found_count = ((*cursor).found_count).wrapping_add(1);
                (*cursor).found_count;
            }
            chdir_do((*cursor).change_dir);
            return if occurrence_option == 0 as i32 as u64 {
                ((*cursor).found_count != 0 as i32 as u64) as i32
            } else {
                ((*cursor).found_count == occurrence_option) as i32
            } != 0;
        }
        if same_order_option as i32 != 0 && (*namelist).found_count != 0 {
            name_gather();
            if (*namelist).found_count != 0 {
                return 0 as i32 != 0;
            }
        } else {
            return 0 as i32 != 0
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn all_names_found(mut p: *mut tar_stat_info) -> bool {
    let mut cursor: *const name = 0 as *const name;
    let mut len: size_t = 0;
    if ((*p).file_name).is_null() || occurrence_option == 0 as i32 as u64
        || (*p).had_trailing_slash as i32 != 0
    {
        return 0 as i32 != 0;
    }
    len = strlen((*p).file_name);
    cursor = namelist;
    while !cursor.is_null() {
        if *((*cursor).name).offset(0 as i32 as isize) as i32 != 0
            && (if occurrence_option == 0 as i32 as u64 {
                ((*cursor).found_count != 0 as i32 as u64) as i32
            } else {
                ((*cursor).found_count >= occurrence_option) as i32
            }) == 0
            || len >= (*cursor).length
                && *((*p).file_name).offset((*cursor).length as isize) as i32
                    == '/' as i32
        {
            return 0 as i32 != 0;
        }
        cursor = (*cursor).next;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn regex_usage_warning(mut name: *const i8) -> i32 {
    static mut warned_once: i32 = 0 as i32;
    if wildcards as u32 == wildcards::default_wildcards as i32 as u32
        && fnmatch_pattern_has_wildcards(name, 0 as i32) as i32 != 0
    {
        warned_once = 1 as i32;
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Pattern matching characters used in file names\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Use --wildcards to enable pattern matching, or --no-wildcards to suppress this warning\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    return warned_once;
}
#[no_mangle]
pub unsafe extern "C" fn names_notfound() {
    let mut cursor: *const name = 0 as *const name;
    cursor = namelist;
    while !cursor.is_null() {
        if (if occurrence_option == 0 as i32 as u64 {
            ((*cursor).found_count != 0 as i32 as u64) as i32
        } else {
            ((*cursor).found_count >= occurrence_option) as i32
        }) == 0 && *((*cursor).name).offset(0 as i32 as isize) as i32 != 0
        {
            regex_usage_warning((*cursor).name);
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                if (*cursor).found_count == 0 as i32 as u64 {
                    dcgettext(
                        0 as *const i8,
                        b"%s: Not found in archive\0" as *const u8 as *const i8,
                        5 as i32,
                    )
                } else {
                    dcgettext(
                        0 as *const i8,
                        b"%s: Required occurrence not found in archive\0" as *const u8
                            as *const i8,
                        5 as i32,
                    )
                },
                quotearg_colon((*cursor).name),
            );
            exit_status = 2 as i32;
        }
        cursor = (*cursor).next;
    }
    namelist = 0 as *mut name;
    nametail = 0 as *mut name;
    if same_order_option {
        let mut name: *const i8 = 0 as *const i8;
        loop {
            name = name_next(1 as i32);
            if name.is_null() {
                break;
            }
            regex_usage_warning(name);
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"%s: Not found in archive\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_colon(name),
            );
            exit_status = 2 as i32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn label_notfound() {
    let mut cursor: *const name = 0 as *const name;
    if namelist.is_null() {
        return;
    }
    cursor = namelist;
    while !cursor.is_null() {
        if if occurrence_option == 0 as i32 as u64 {
            ((*cursor).found_count != 0 as i32 as u64) as i32
        } else {
            ((*cursor).found_count >= occurrence_option) as i32
        } != 0
        {
            return;
        }
        cursor = (*cursor).next;
    }
    if verbose_option != 0 {
        error(
            0 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Archive label mismatch\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    set_exit_status(1 as i32);
    cursor = namelist;
    while !cursor.is_null() {
        if regex_usage_warning((*cursor).name) != 0 {
            break;
        }
        cursor = (*cursor).next;
    }
    namelist = 0 as *mut name;
    nametail = 0 as *mut name;
    if same_order_option {
        let mut name: *const i8 = 0 as *const i8;
        loop {
            name = name_next(1 as i32);
            if !(!name.is_null() && regex_usage_warning(name) == 0 as i32) {
                break;
            }
        }
    }
}
unsafe extern "C" fn merge_sort_sll(
    mut list: *mut name,
    mut length: i32,
    mut compare: Option<unsafe extern "C" fn(*const name, *const name) -> i32>,
) -> *mut name {
    let mut first_list: *mut name = 0 as *mut name;
    let mut second_list: *mut name = 0 as *mut name;
    let mut first_length: i32 = 0;
    let mut second_length: i32 = 0;
    let mut result: *mut name = 0 as *mut name;
    let mut merge_point: *mut *mut name = 0 as *mut *mut name;
    let mut cursor: *mut name = 0 as *mut name;
    let mut counter: i32 = 0;
    if length == 1 as i32 {
        return list;
    }
    if length == 2 as i32 {
        if (Some(compare.expect("non-null function pointer")))
            .expect("non-null function pointer")(list, (*list).next) > 0 as i32
        {
            result = (*list).next;
            (*result).next = list;
            (*list).next = 0 as *mut name;
            return result;
        }
        return list;
    }
    first_list = list;
    first_length = (length + 1 as i32) / 2 as i32;
    second_length = length / 2 as i32;
    cursor = list;
    counter = first_length - 1 as i32;
    while counter != 0 {
        cursor = (*cursor).next;
        counter -= 1;
        counter;
    }
    second_list = (*cursor).next;
    (*cursor).next = 0 as *mut name;
    first_list = merge_sort_sll(first_list, first_length, compare);
    second_list = merge_sort_sll(second_list, second_length, compare);
    merge_point = &mut result;
    while !first_list.is_null() && !second_list.is_null() {
        if (Some(compare.expect("non-null function pointer")))
            .expect("non-null function pointer")(first_list, second_list) < 0 as i32
        {
            cursor = (*first_list).next;
            *merge_point = first_list;
            merge_point = &mut (*first_list).next;
            first_list = cursor;
        } else {
            cursor = (*second_list).next;
            *merge_point = second_list;
            merge_point = &mut (*second_list).next;
            second_list = cursor;
        }
    }
    if !first_list.is_null() {
        *merge_point = first_list;
    } else {
        *merge_point = second_list;
    }
    return result;
}
unsafe extern "C" fn merge_sort(
    mut list: *mut name,
    mut length: i32,
    mut compare: Option<unsafe extern "C" fn(*const name, *const name) -> i32>,
) -> *mut name {
    let mut head: *mut name = 0 as *mut name;
    let mut p: *mut name = 0 as *mut name;
    let mut prev: *mut name = 0 as *mut name;
    head = merge_sort_sll(list, length, compare);
    prev = 0 as *mut name;
    p = head;
    while !p.is_null() {
        (*p).prev = prev;
        prev = p;
        p = (*p).next;
    }
    return head;
}
unsafe extern "C" fn compare_names_found(
    mut n1: *const name,
    mut n2: *const name,
) -> i32 {
    let mut found_diff: i32 = (if occurrence_option == 0 as i32 as u64 {
        ((*n2).found_count != 0 as i32 as u64) as i32
    } else {
        ((*n2).found_count >= occurrence_option) as i32
    })
        - (if occurrence_option == 0 as i32 as u64 {
            ((*n1).found_count != 0 as i32 as u64) as i32
        } else {
            ((*n1).found_count >= occurrence_option) as i32
        });
    return if found_diff != 0 { found_diff } else { strcmp((*n1).name, (*n2).name) };
}
unsafe extern "C" fn compare_names(mut n1: *const name, mut n2: *const name) -> i32 {
    return strcmp((*n1).name, (*n2).name);
}
unsafe extern "C" fn add_hierarchy_to_namelist(
    mut st: *mut tar_stat_info,
    mut name: *mut name,
) {
    let mut buffer: *const i8 = 0 as *const i8;
    (*name).directory = scan_directory(st);
    buffer = directory_contents((*name).directory);
    if !buffer.is_null() {
        let mut child_head: *mut name = 0 as *mut name;
        let mut child_tail: *mut name = 0 as *mut name;
        let mut name_length: size_t = (*name).length;
        let mut allocated_length: size_t = (if name_length >= 100 as i32 as u64 {
            name_length.wrapping_add(100 as i32 as u64)
        } else {
            100 as i32 as u64
        })
            .wrapping_add(2 as i32 as u64);
        let mut namebuf: *mut i8 = xmalloc(allocated_length) as *mut i8;
        let mut string: *const i8 = 0 as *const i8;
        let mut string_length: size_t = 0;
        let mut change_dir: i32 = (*name).change_dir;
        strcpy(namebuf, (*name).name);
        if !(*namebuf.offset(name_length.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32)
        {
            let fresh5 = name_length;
            name_length = name_length.wrapping_add(1);
            *namebuf.offset(fresh5 as isize) = '/' as i32 as i8;
            *namebuf.offset(name_length as isize) = '\0' as i32 as i8;
        }
        string = buffer;
        while *string != 0 {
            string_length = strlen(string);
            if *string as i32 == 'D' as i32 {
                let mut np: *mut name = 0 as *mut name;
                let mut subdir: tar_stat_info = tar_stat_info {
                    orig_file_name: 0 as *mut i8,
                    file_name: 0 as *mut i8,
                    had_trailing_slash: false,
                    link_name: 0 as *mut i8,
                    uname: 0 as *mut i8,
                    gname: 0 as *mut i8,
                    cntx_name: 0 as *mut i8,
                    acls_a_ptr: 0 as *mut i8,
                    acls_a_len: 0,
                    acls_d_ptr: 0 as *mut i8,
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
                    sparse_map: 0 as *mut sp_array,
                    real_size: 0,
                    real_size_set: false,
                    sparse_name_done: false,
                    xattr_map_size: 0,
                    xattr_map: 0 as *mut xattr_array,
                    xhdr: xheader {
                        stk: 0 as *mut obstack,
                        size: 0,
                        buffer: 0 as *mut i8,
                        string_length: 0,
                    },
                    is_dumpdir: false,
                    skipped: false,
                    dumpdir: 0 as *mut i8,
                    parent: 0 as *mut tar_stat_info,
                    dirstream: 0 as *mut DIR,
                    fd: 0,
                    exclude_list: 0 as *mut exclist,
                };
                let mut subfd: i32 = 0;
                while allocated_length < name_length.wrapping_add(string_length) {
                    namebuf = x2realloc(
                        namebuf as *mut libc::c_void,
                        &mut allocated_length,
                    ) as *mut i8;
                }
                strcpy(
                    namebuf.offset(name_length as isize),
                    string.offset(1 as i32 as isize),
                );
                np = addname(namebuf, change_dir, 0 as i32 != 0, name);
                if child_head.is_null() {
                    child_head = np;
                } else {
                    (*child_tail).sibling = np;
                }
                child_tail = np;
                tar_stat_init(&mut subdir);
                subdir.parent = st;
                if (*st).fd < 0 as i32 {
                    subfd = -(1 as i32);
                    *__errno_location() = -(*st).fd;
                } else {
                    subfd = subfile_open(
                        st,
                        string.offset(1 as i32 as isize),
                        open_read_flags | 0o200000 as i32,
                    );
                }
                if subfd < 0 as i32 {
                    open_diag(namebuf);
                } else {
                    subdir.fd = subfd;
                    if fstat(subfd, &mut subdir.stat) != 0 as i32 {
                        stat_diag(namebuf);
                    } else if !(0o200000 as i32 != 0
                        || subdir.stat.st_mode & 0o170000 as i32 as u32
                            == 0o40000 as i32 as u32)
                    {
                        *__errno_location() = 20 as i32;
                        open_diag(namebuf);
                    } else {
                        subdir.orig_file_name = xstrdup(namebuf);
                        add_hierarchy_to_namelist(&mut subdir, np);
                        restore_parent_fd(&mut subdir);
                    }
                }
                tar_stat_destroy(&mut subdir);
            }
            string = string.offset(string_length.wrapping_add(1 as i32 as u64) as isize);
        }
        rpl_free(namebuf as *mut libc::c_void);
        (*name).child = child_head;
    }
}
unsafe extern "C" fn name_hash(
    mut entry: *const libc::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut name: *const name = entry as *const name;
    return hash_string((*name).caname, n_buckets);
}
unsafe extern "C" fn name_compare(
    mut entry1: *const libc::c_void,
    mut entry2: *const libc::c_void,
) -> bool {
    let mut name1: *const name = entry1 as *const name;
    let mut name2: *const name = entry2 as *const name;
    return strcmp((*name1).caname, (*name2).caname) == 0 as i32;
}
unsafe extern "C" fn rebase_child_list(mut child: *mut name, mut parent: *mut name) {
    let mut old_prefix_len: size_t = (*(*child).parent).length;
    let mut new_prefix_len: size_t = (*parent).length;
    let mut new_prefix: *mut i8 = (*parent).name;
    while !child.is_null() {
        let mut size: size_t = ((*child).length)
            .wrapping_sub(old_prefix_len)
            .wrapping_add(new_prefix_len);
        let mut newp: *mut i8 = xmalloc(size.wrapping_add(1 as i32 as u64)) as *mut i8;
        strcpy(newp, new_prefix);
        strcat(newp, ((*child).name).offset(old_prefix_len as isize));
        rpl_free((*child).name as *mut libc::c_void);
        (*child).name = newp;
        (*child).length = size;
        rebase_directory(
            (*child).directory,
            (*(*child).parent).name,
            old_prefix_len,
            new_prefix,
            new_prefix_len,
        );
        child = (*child).sibling;
    }
}
#[no_mangle]
pub unsafe extern "C" fn collect_and_sort_names() {
    let mut name: *mut name = 0 as *mut name;
    let mut next_name: *mut name = 0 as *mut name;
    let mut prev_name: *mut name = 0 as *mut name;
    let mut num_names: i32 = 0;
    let mut nametab: *mut Hash_table = 0 as *mut Hash_table;
    name_gather();
    if namelist.is_null() {
        addname(
            b".\0" as *const u8 as *const i8,
            0 as i32,
            0 as i32 != 0,
            0 as *mut name,
        );
    }
    if !listed_incremental_option.is_null() {
        match chdir_count() {
            0 => {}
            1 => {
                if (*namelist).change_dir == 0 as i32 {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    error(
                        0 as i32,
                        0 as i32,
                        dcgettext(
                            0 as *const i8,
                            b"Using -C option inside file list is not allowed with --listed-incremental\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                    usage(2 as i32);
                }
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
                        b"Only one -C option is allowed with --listed-incremental\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                );
                usage(2 as i32);
            }
        }
        read_directory_file();
    }
    num_names = 0 as i32;
    name = namelist;
    while !name.is_null() {
        let mut st: tar_stat_info = tar_stat_info {
            orig_file_name: 0 as *mut i8,
            file_name: 0 as *mut i8,
            had_trailing_slash: false,
            link_name: 0 as *mut i8,
            uname: 0 as *mut i8,
            gname: 0 as *mut i8,
            cntx_name: 0 as *mut i8,
            acls_a_ptr: 0 as *mut i8,
            acls_a_len: 0,
            acls_d_ptr: 0 as *mut i8,
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
            sparse_map: 0 as *mut sp_array,
            real_size: 0,
            real_size_set: false,
            sparse_name_done: false,
            xattr_map_size: 0,
            xattr_map: 0 as *mut xattr_array,
            xhdr: xheader {
                stk: 0 as *mut obstack,
                size: 0,
                buffer: 0 as *mut i8,
                string_length: 0,
            },
            is_dumpdir: false,
            skipped: false,
            dumpdir: 0 as *mut i8,
            parent: 0 as *mut tar_stat_info,
            dirstream: 0 as *mut DIR,
            fd: 0,
            exclude_list: 0 as *mut exclist,
        };
        if !((*name).found_count != 0 || !((*name).directory).is_null()) {
            if !((*name).matching_flags & (1 as i32) << 28 as i32 != 0) {
                chdir_do((*name).change_dir);
                if !(*((*name).name).offset(0 as i32 as isize) as i32 == 0 as i32) {
                    tar_stat_init(&mut st);
                    if deref_stat((*name).name, &mut st.stat) != 0 as i32 {
                        stat_diag((*name).name);
                    } else {
                        if st.stat.st_mode & 0o170000 as i32 as u32
                            == 0o40000 as i32 as u32
                        {
                            let mut dir_fd: i32 = openat(
                                chdir_fd,
                                (*name).name,
                                open_read_flags | 0o200000 as i32,
                            );
                            if dir_fd < 0 as i32 {
                                open_diag((*name).name);
                            } else {
                                st.fd = dir_fd;
                                if fstat(dir_fd, &mut st.stat) != 0 as i32 {
                                    stat_diag((*name).name);
                                } else if 0o200000 as i32 != 0
                                    || st.stat.st_mode & 0o170000 as i32 as u32
                                        == 0o40000 as i32 as u32
                                {
                                    st.orig_file_name = xstrdup((*name).name);
                                    (*name).found_count = ((*name).found_count).wrapping_add(1);
                                    (*name).found_count;
                                    add_hierarchy_to_namelist(&mut st, name);
                                } else {
                                    *__errno_location() = 20 as i32;
                                    open_diag((*name).name);
                                }
                            }
                        }
                        tar_stat_destroy(&mut st);
                    }
                }
            }
        }
        name = (*name).next;
        num_names += 1;
        num_names;
    }
    namelist = merge_sort(
        namelist,
        num_names,
        Some(compare_names as unsafe extern "C" fn(*const name, *const name) -> i32),
    );
    num_names = 0 as i32;
    nametab = hash_initialize(
        0 as i32 as size_t,
        0 as *const Hash_tuning,
        Some(name_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
        Some(
            name_compare
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    let mut current_block_64: u64;
    name = namelist;
    while !name.is_null() {
        next_name = (*name).next;
        (*name).caname = normalize_filename((*name).change_dir, (*name).name);
        if !prev_name.is_null() {
            let mut p: *mut name = hash_lookup(nametab, name as *const libc::c_void)
                as *mut name;
            if !p.is_null() {
                if ((*name).parent).is_null() {
                    if !((*p).child).is_null() {
                        rebase_child_list((*p).child, name);
                    }
                    hash_remove(nametab, name as *const libc::c_void);
                    remname(p);
                    free_name(p);
                    num_names -= 1;
                    num_names;
                    current_block_64 = 2116367355679836638;
                } else {
                    if !((*name).child).is_null() {
                        rebase_child_list((*name).child, p);
                    }
                    remname(name);
                    free_name(name);
                    current_block_64 = 1622411330066726685;
                }
            } else {
                current_block_64 = 2116367355679836638;
            }
        } else {
            current_block_64 = 2116367355679836638;
        }
        match current_block_64 {
            2116367355679836638 => {
                (*name).found_count = 0 as i32 as uintmax_t;
                if (hash_insert(nametab, name as *const libc::c_void)).is_null() {
                    xalloc_die();
                }
                prev_name = name;
                num_names += 1;
                num_names;
            }
            _ => {}
        }
        name = next_name;
    }
    nametail = prev_name;
    hash_free(nametab);
    namelist = merge_sort(
        namelist,
        num_names,
        Some(
            compare_names_found as unsafe extern "C" fn(*const name, *const name) -> i32,
        ),
    );
    if !listed_incremental_option.is_null() {
        name = namelist;
        while !name.is_null()
            && *((*name).name).offset(0 as i32 as isize) as i32 == 0 as i32
        {
            name = (*name).next;
        }
        if !name.is_null() {
            append_incremental_renames((*name).directory);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn name_scan(mut file_name: *const i8) -> *mut name {
    let mut length: size_t = strlen(file_name);
    loop {
        let mut cursor: *mut name = namelist_match(file_name, length);
        if !cursor.is_null() {
            return cursor;
        }
        if same_order_option as i32 != 0 && !namelist.is_null()
            && (*namelist).found_count != 0
        {
            name_gather();
            if (*namelist).found_count != 0 {
                return 0 as *mut name;
            }
        } else {
            return 0 as *mut name
        }
    };
}
#[no_mangle]
pub static mut gnu_list_name: *mut name = 0 as *const name as *mut name;
#[no_mangle]
pub unsafe extern "C" fn name_from_list() -> *const name {
    if gnu_list_name.is_null() {
        gnu_list_name = namelist;
    }
    while !gnu_list_name.is_null()
        && ((*gnu_list_name).found_count != 0
            || *((*gnu_list_name).name).offset(0 as i32 as isize) as i32 == 0 as i32)
    {
        gnu_list_name = (*gnu_list_name).next;
    }
    if !gnu_list_name.is_null() {
        (*gnu_list_name).found_count = ((*gnu_list_name).found_count).wrapping_add(1);
        (*gnu_list_name).found_count;
        chdir_do((*gnu_list_name).change_dir);
        return gnu_list_name;
    }
    return 0 as *const name;
}
#[no_mangle]
pub unsafe extern "C" fn blank_name_list() {
    let mut name: *mut name = 0 as *mut name;
    gnu_list_name = 0 as *mut name;
    name = namelist;
    while !name.is_null() {
        (*name).found_count = 0 as i32 as uintmax_t;
        name = (*name).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn make_file_name(
    mut directory_name: *const i8,
    mut name: *const i8,
) -> *mut i8 {
    let mut dirlen: size_t = strlen(directory_name);
    let mut namelen: size_t = (strlen(name)).wrapping_add(1 as i32 as u64);
    let mut slash: i32 = (dirlen != 0
        && !(*directory_name.offset(dirlen.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32)) as i32;
    let mut buffer: *mut i8 = xmalloc(
        dirlen.wrapping_add(slash as u64).wrapping_add(namelen),
    ) as *mut i8;
    memcpy(buffer as *mut libc::c_void, directory_name as *const libc::c_void, dirlen);
    *buffer.offset(dirlen as isize) = '/' as i32 as i8;
    memcpy(
        buffer.offset(dirlen as isize).offset(slash as isize) as *mut libc::c_void,
        name as *const libc::c_void,
        namelen,
    );
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn stripped_prefix_len(
    mut file_name: *const i8,
    mut num: size_t,
) -> size_t {
    let mut p: *const i8 = file_name.offset(0 as i32 as isize);
    while *p as i32 == '/' as i32 {
        p = p.offset(1);
        p;
    }
    while *p != 0 {
        let mut slash: bool = *p as i32 == '/' as i32;
        p = p.offset(1);
        p;
        if slash {
            num = num.wrapping_sub(1);
            if num == 0 as i32 as u64 {
                return p.offset_from(file_name) as i64 as size_t;
            }
            while *p as i32 == '/' as i32 {
                p = p.offset(1);
                p;
            }
        }
    }
    return -(1 as i32) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn contains_dot_dot(mut name: *const i8) -> bool {
    let mut p: *const i8 = name.offset(0 as i32 as isize);
    loop {
        if *p.offset(0 as i32 as isize) as i32 == '.' as i32
            && *p.offset(1 as i32 as isize) as i32 == '.' as i32
            && (*p.offset(2 as i32 as isize) as i32 == '/' as i32
                || *p.offset(2 as i32 as isize) == 0)
        {
            return 1 as i32 != 0;
        }
        while !(*p as i32 == '/' as i32) {
            let fresh6 = p;
            p = p.offset(1);
            if *fresh6 == 0 {
                return 0 as i32 != 0;
            }
        }
        p = p.offset(1);
        p;
    };
}