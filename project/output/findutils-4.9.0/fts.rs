use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type __dirstream;
    pub type hash_table;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dirfd(__dirp: *mut DIR) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn __fxstatat(
        __ver: i32,
        __fildes: i32,
        __filename: *const i8,
        __stat_buf: *mut stat,
        __flag: i32,
    ) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn i_ring_init(ir: *mut I_ring, ir_default_val: i32);
    fn i_ring_push(ir: *mut I_ring, val: i32) -> i32;
    fn i_ring_pop(ir: *mut I_ring) -> i32;
    fn i_ring_empty(ir: *const I_ring) -> bool;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn calloc(_: u64, _: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn close(__fd: i32) -> i32;
    fn fchdir(__fd: i32) -> i32;
    fn open_safer(_: *const i8, _: i32, _: ...) -> i32;
    fn openat_safer(_: i32, _: *const i8, _: i32, _: ...) -> i32;
    fn opendirat(_: i32, _: *const i8, _: i32, _: *mut i32) -> *mut DIR;
    fn hash_free(table: *mut Hash_table);
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
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
    fn cycle_check_init(state: *mut cycle_check_state);
    fn cycle_check(state: *mut cycle_check_state, sb: *const stat) -> bool;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn fstatfs(__fildes: i32, __buf: *mut statfs) -> i32;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type __uintmax_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __fsblkcnt_t = u64;
pub type __fsfilcnt_t = u64;
pub type __fsword_t = i64;
pub type __syscall_slong_t = i64;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    DT_WHT = 14,
    DT_SOCK = 12,
    DT_LNK = 10,
    DT_REG = 8,
    DT_BLK = 6,
    DT_DIR = 4,
    DT_CHR = 2,
    DT_FIFO = 1,
    DT_UNKNOWN = 0,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::DT_WHT => 14,
            C2RustUnnamed::DT_SOCK => 12,
            C2RustUnnamed::DT_LNK => 10,
            C2RustUnnamed::DT_REG => 8,
            C2RustUnnamed::DT_BLK => 6,
            C2RustUnnamed::DT_DIR => 4,
            C2RustUnnamed::DT_CHR => 2,
            C2RustUnnamed::DT_FIFO => 1,
            C2RustUnnamed::DT_UNKNOWN => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            14 => C2RustUnnamed::DT_WHT,
            12 => C2RustUnnamed::DT_SOCK,
            10 => C2RustUnnamed::DT_LNK,
            8 => C2RustUnnamed::DT_REG,
            6 => C2RustUnnamed::DT_BLK,
            4 => C2RustUnnamed::DT_DIR,
            2 => C2RustUnnamed::DT_CHR,
            1 => C2RustUnnamed::DT_FIFO,
            0 => C2RustUnnamed::DT_UNKNOWN,
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
pub type DIR = __dirstream;
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
pub struct I_ring {
    pub ir_data: [i32; 4],
    pub ir_default_val: i32,
    pub ir_front: u32,
    pub ir_back: u32,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut i8,
    pub fts_rfd: i32,
    pub fts_cwd_fd: i32,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> i32,
    >,
    pub fts_options: i32,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2RustUnnamed_0,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cycle_check_state {
    pub dev_ino: dev_ino,
    pub chdir_counter: uintmax_t,
    pub magic: i32,
}
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_ino {
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: i64,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut i8,
    pub fts_path: *mut i8,
    pub fts_errno: i32,
    pub fts_symfd: i32,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [i8; 0],
}
pub type FTSENT = _ftsent;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type fsword = __fsword_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dev_type {
    pub st_dev: dev_t,
    pub f_type: fsword,
}
pub type Hash_table = hash_table;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_4 {
    DEV_TYPE_HT_INITIAL_SIZE = 13,
}
impl C2RustUnnamed_4 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_4::DEV_TYPE_HT_INITIAL_SIZE => 13,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_4 {
        match value {
            13 => C2RustUnnamed_4::DEV_TYPE_HT_INITIAL_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed_4: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_4 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_4 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_4 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_4 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_4 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn add(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn sub(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn mul(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn div(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_4 {
    type Output = C2RustUnnamed_4;
    fn rem(self, rhs: u32) -> C2RustUnnamed_4 {
        C2RustUnnamed_4::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    _FTS_INODE_SORT_DIR_ENTRIES_THRESHOLD = 10000,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::_FTS_INODE_SORT_DIR_ENTRIES_THRESHOLD => 10000,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            10000 => C2RustUnnamed_2::_FTS_INODE_SORT_DIR_ENTRIES_THRESHOLD,
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
pub enum Fts_stat {
    FTS_NO_STAT_REQUIRED = 1,
    FTS_STAT_REQUIRED = 2,
}
impl Fts_stat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            Fts_stat::FTS_NO_STAT_REQUIRED => 1,
            Fts_stat::FTS_STAT_REQUIRED => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> Fts_stat {
        match value {
            1 => Fts_stat::FTS_NO_STAT_REQUIRED,
            2 => Fts_stat::FTS_STAT_REQUIRED,
            _ => panic!("Invalid value for Fts_stat: {}", value),
        }
    }
}
impl AddAssign<u32> for Fts_stat {
    fn add_assign(&mut self, rhs: u32) {
        *self = Fts_stat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for Fts_stat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = Fts_stat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for Fts_stat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = Fts_stat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for Fts_stat {
    fn div_assign(&mut self, rhs: u32) {
        *self = Fts_stat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for Fts_stat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = Fts_stat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for Fts_stat {
    type Output = Fts_stat;
    fn add(self, rhs: u32) -> Fts_stat {
        Fts_stat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for Fts_stat {
    type Output = Fts_stat;
    fn sub(self, rhs: u32) -> Fts_stat {
        Fts_stat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for Fts_stat {
    type Output = Fts_stat;
    fn mul(self, rhs: u32) -> Fts_stat {
        Fts_stat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for Fts_stat {
    type Output = Fts_stat;
    fn div(self, rhs: u32) -> Fts_stat {
        Fts_stat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for Fts_stat {
    type Output = Fts_stat;
    fn rem(self, rhs: u32) -> Fts_stat {
        Fts_stat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum leaf_optimization {
    NO_LEAF_OPTIMIZATION,
    OK_LEAF_OPTIMIZATION,
}
impl leaf_optimization {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            leaf_optimization::NO_LEAF_OPTIMIZATION => 0,
            leaf_optimization::OK_LEAF_OPTIMIZATION => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> leaf_optimization {
        match value {
            0 => leaf_optimization::NO_LEAF_OPTIMIZATION,
            1 => leaf_optimization::OK_LEAF_OPTIMIZATION,
            _ => panic!("Invalid value for leaf_optimization: {}", value),
        }
    }
}
impl AddAssign<u32> for leaf_optimization {
    fn add_assign(&mut self, rhs: u32) {
        *self = leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for leaf_optimization {
    fn sub_assign(&mut self, rhs: u32) {
        *self = leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for leaf_optimization {
    fn mul_assign(&mut self, rhs: u32) {
        *self = leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for leaf_optimization {
    fn div_assign(&mut self, rhs: u32) {
        *self = leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for leaf_optimization {
    fn rem_assign(&mut self, rhs: u32) {
        *self = leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for leaf_optimization {
    type Output = leaf_optimization;
    fn add(self, rhs: u32) -> leaf_optimization {
        leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for leaf_optimization {
    type Output = leaf_optimization;
    fn sub(self, rhs: u32) -> leaf_optimization {
        leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for leaf_optimization {
    type Output = leaf_optimization;
    fn mul(self, rhs: u32) -> leaf_optimization {
        leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for leaf_optimization {
    type Output = leaf_optimization;
    fn div(self, rhs: u32) -> leaf_optimization {
        leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for leaf_optimization {
    type Output = leaf_optimization;
    fn rem(self, rhs: u32) -> leaf_optimization {
        leaf_optimization::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    MIN_DIR_NLINK = 2,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::MIN_DIR_NLINK => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            2 => C2RustUnnamed_3::MIN_DIR_NLINK,
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
pub struct Active_dir {
    pub dev: dev_t,
    pub ino: ino_t,
    pub fts_ent: *mut FTSENT,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    HT_INITIAL_SIZE = 31,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::HT_INITIAL_SIZE => 31,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            31 => C2RustUnnamed_1::HT_INITIAL_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
unsafe extern "C" fn fstatat(
    mut __fd: i32,
    mut __filename: *const i8,
    mut __statbuf: *mut stat,
    mut __flag: i32,
) -> i32 {
    return __fxstatat(1 as i32, __fd, __filename, __statbuf, __flag);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn AD_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut ax: *const Active_dir = x as *const Active_dir;
    let mut ay: *const Active_dir = y as *const Active_dir;
    return (*ax).ino == (*ay).ino && (*ax).dev == (*ay).dev;
}
unsafe extern "C" fn AD_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut ax: *const Active_dir = x as *const Active_dir;
    return ((*ax).ino).wrapping_rem(table_size);
}
unsafe extern "C" fn setup_dir(mut fts: *mut FTS) -> bool {
    if (*fts).fts_options & (0x100 as i32 | 0x2 as i32) != 0 {
        (*fts).fts_cycle.ht = hash_initialize(
            C2RustUnnamed_1::HT_INITIAL_SIZE as i32 as size_t,
            0 as *const Hash_tuning,
            Some(AD_hash as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
            Some(
                AD_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if ((*fts).fts_cycle.ht).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        (*fts).fts_cycle.state = malloc(
            ::core::mem::size_of::<cycle_check_state>() as u64,
        ) as *mut cycle_check_state;
        if ((*fts).fts_cycle.state).is_null() {
            return 0 as i32 != 0;
        }
        cycle_check_init((*fts).fts_cycle.state);
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn free_dir(mut sp: *mut FTS) {
    if (*sp).fts_options & (0x100 as i32 | 0x2 as i32) != 0 {
        if !((*sp).fts_cycle.ht).is_null() {
            hash_free((*sp).fts_cycle.ht);
        }
    } else {
        rpl_free((*sp).fts_cycle.state as *mut libc::c_void);
    };
}
unsafe extern "C" fn enter_dir(mut fts: *mut FTS, mut ent: *mut FTSENT) -> bool {
    if (*fts).fts_options & (0x100 as i32 | 0x2 as i32) != 0 {
        let mut st: *const stat = ((*ent).fts_statp).as_mut_ptr();
        let mut ad: *mut Active_dir = malloc(::core::mem::size_of::<Active_dir>() as u64)
            as *mut Active_dir;
        let mut ad_from_table: *mut Active_dir = 0 as *mut Active_dir;
        if ad.is_null() {
            return 0 as i32 != 0;
        }
        (*ad).dev = (*st).st_dev;
        (*ad).ino = (*st).st_ino;
        (*ad).fts_ent = ent;
        ad_from_table = hash_insert((*fts).fts_cycle.ht, ad as *const libc::c_void)
            as *mut Active_dir;
        if ad_from_table != ad {
            rpl_free(ad as *mut libc::c_void);
            if ad_from_table.is_null() {
                return 0 as i32 != 0;
            }
            (*ent).fts_cycle = (*ad_from_table).fts_ent;
            (*ent).fts_info = 2 as i32 as libc::c_ushort;
        }
    } else if cycle_check((*fts).fts_cycle.state, ((*ent).fts_statp).as_mut_ptr()) {
        (*ent).fts_cycle = ent;
        (*ent).fts_info = 2 as i32 as libc::c_ushort;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn leave_dir(mut fts: *mut FTS, mut ent: *mut FTSENT) {
    let mut st: *const stat = ((*ent).fts_statp).as_mut_ptr();
    if (*fts).fts_options & (0x100 as i32 | 0x2 as i32) != 0 {
        let mut obj: Active_dir = Active_dir {
            dev: 0,
            ino: 0,
            fts_ent: 0 as *mut FTSENT,
        };
        let mut found: *mut libc::c_void = 0 as *mut libc::c_void;
        obj.dev = (*st).st_dev;
        obj.ino = (*st).st_ino;
        found = hash_remove(
            (*fts).fts_cycle.ht,
            &mut obj as *mut Active_dir as *const libc::c_void,
        );
        if found.is_null() {
            abort();
        }
        rpl_free(found);
    } else {
        let mut parent: *mut FTSENT = (*ent).fts_parent;
        if !parent.is_null() && 0 as i32 as i64 <= (*parent).fts_level {
            if (*(*fts).fts_cycle.state).chdir_counter == 0 as i32 as u64 {
                abort();
            }
            if (*(*fts).fts_cycle.state).dev_ino.st_ino == (*st).st_ino
                && (*(*fts).fts_cycle.state).dev_ino.st_dev == (*st).st_dev
            {
                (*(*fts).fts_cycle.state).dev_ino.st_dev = (*((*parent).fts_statp)
                    .as_mut_ptr())
                    .st_dev;
                (*(*fts).fts_cycle.state).dev_ino.st_ino = (*((*parent).fts_statp)
                    .as_mut_ptr())
                    .st_ino;
            }
        }
    };
}
unsafe extern "C" fn fd_ring_clear(mut fd_ring: *mut I_ring) {
    while !i_ring_empty(fd_ring) {
        let mut fd: i32 = i_ring_pop(fd_ring);
        if 0 as i32 <= fd {
            close(fd);
        }
    }
}
unsafe extern "C" fn fts_set_stat_required(mut p: *mut FTSENT, mut required: bool) {
    if !((*p).fts_info as i32 == 11 as i32) {
        abort();
    }
    (*((*p).fts_statp).as_mut_ptr()).st_size = (if required as i32 != 0 {
        Fts_stat::FTS_STAT_REQUIRED as i32
    } else {
        Fts_stat::FTS_NO_STAT_REQUIRED as i32
    }) as __off_t;
}
unsafe extern "C" fn cwd_advance_fd(
    mut sp: *mut FTS,
    mut fd: i32,
    mut chdir_down_one: bool,
) {
    let mut old: i32 = (*sp).fts_cwd_fd;
    if !(old != fd || old == -(100 as i32)) {
        abort();
    }
    if chdir_down_one {
        let mut prev_fd_in_slot: i32 = i_ring_push(&mut (*sp).fts_fd_ring, old);
        if 0 as i32 <= prev_fd_in_slot {
            close(prev_fd_in_slot);
        }
    } else if (*sp).fts_options & 0x4 as i32 == 0 {
        if 0 as i32 <= old {
            close(old);
        }
    }
    (*sp).fts_cwd_fd = fd;
}
unsafe extern "C" fn restore_initial_cwd(mut sp: *mut FTS) -> i32 {
    let mut fail: i32 = ((*sp).fts_options & 0x4 as i32 == 0
        && (if (*sp).fts_options & 0x200 as i32 != 0 {
            cwd_advance_fd(
                sp,
                (if (*sp).fts_options & 0x200 as i32 != 0 {
                    -(100 as i32)
                } else {
                    (*sp).fts_rfd
                }),
                1 as i32 != 0,
            );
            0 as i32
        } else {
            fchdir(
                (if (*sp).fts_options & 0x200 as i32 != 0 {
                    -(100 as i32)
                } else {
                    (*sp).fts_rfd
                }),
            )
        }) != 0) as i32;
    fd_ring_clear(&mut (*sp).fts_fd_ring);
    return fail;
}
unsafe extern "C" fn diropen(mut sp: *const FTS, mut dir: *const i8) -> i32 {
    let mut open_flags: i32 = 0 as i32 | 0o2000000 as i32 | 0o200000 as i32
        | 0o400 as i32 | 0o4000 as i32
        | (if (*sp).fts_options & 0x10 as i32 != 0 {
            0o400000 as i32
        } else {
            0 as i32
        });
    let mut fd: i32 = if (*sp).fts_options & 0x200 as i32 != 0 {
        openat_safer((*sp).fts_cwd_fd, dir, open_flags)
    } else {
        open_safer(dir, open_flags)
    };
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fts_open(
    mut argv: *const *mut i8,
    mut options: i32,
    mut compar: Option<
        unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> i32,
    >,
) -> *mut FTS {
    let mut current_block: u64;
    let mut sp: *mut FTS = 0 as *mut FTS;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut root: *mut FTSENT = 0 as *mut FTSENT;
    let mut nitems: size_t = 0;
    let mut parent: *mut FTSENT = 0 as *mut FTSENT;
    let mut tmp: *mut FTSENT = 0 as *mut FTSENT;
    let mut defer_stat: bool = false;
    if options & !(0xfff as i32) != 0 {
        *__errno_location() = 22 as i32;
        return 0 as *mut FTS;
    }
    if options & 0x4 as i32 != 0 && options & 0x200 as i32 != 0 {
        *__errno_location() = 22 as i32;
        return 0 as *mut FTS;
    }
    if options & (0x2 as i32 | 0x10 as i32) == 0 {
        *__errno_location() = 22 as i32;
        return 0 as *mut FTS;
    }
    sp = calloc(1 as i32 as u64, ::core::mem::size_of::<FTS>() as u64) as *mut FTS;
    if sp.is_null() {
        return 0 as *mut FTS;
    }
    (*sp).fts_compar = compar;
    (*sp).fts_options = options;
    if (*sp).fts_options & 0x2 as i32 != 0 {
        (*sp).fts_options |= 0x4 as i32;
        (*sp).fts_options &= !(0x200 as i32);
    }
    (*sp).fts_cwd_fd = -(100 as i32);
    if (*sp).fts_options & 0x200 as i32 != 0 && 1 as i32 == 0 {
        let mut fd: i32 = open_safer(
            b".\0" as *const u8 as *const i8,
            0 as i32 | 0o2000000 as i32,
        );
        if !(fd < 0 as i32) {
            close(fd);
        }
    }
    let mut maxarglen: size_t = fts_maxarglen(argv);
    if fts_palloc(
        sp,
        if maxarglen > 4096 as i32 as u64 { maxarglen } else { 4096 as i32 as u64 },
    ) {
        if !(*argv).is_null() {
            parent = fts_alloc(sp, b"\0" as *const u8 as *const i8, 0 as i32 as size_t);
            if parent.is_null() {
                current_block = 2954222851719664042;
            } else {
                (*parent).fts_level = -(1 as i32) as ptrdiff_t;
                current_block = 17788412896529399552;
            }
        } else {
            current_block = 17788412896529399552;
        }
        match current_block {
            17788412896529399552 => {
                defer_stat = compar.is_none() || (*sp).fts_options & 0x400 as i32 != 0;
                root = 0 as *mut FTSENT;
                nitems = 0 as i32 as size_t;
                loop {
                    if (*argv).is_null() {
                        current_block = 10930818133215224067;
                        break;
                    }
                    let mut len: size_t = strlen(*argv);
                    if options & 0x800 as i32 == 0 {
                        let mut v: *const i8 = *argv;
                        if (2 as i32 as u64) < len
                            && *v.offset(len.wrapping_sub(1 as i32 as u64) as isize)
                                as i32 == '/' as i32
                        {
                            while (1 as i32 as u64) < len
                                && *v.offset(len.wrapping_sub(2 as i32 as u64) as isize)
                                    as i32 == '/' as i32
                            {
                                len = len.wrapping_sub(1);
                                len;
                            }
                        }
                    }
                    p = fts_alloc(sp, *argv, len);
                    if p.is_null() {
                        current_block = 16629745464101581754;
                        break;
                    }
                    (*p).fts_level = 0 as i32 as ptrdiff_t;
                    (*p).fts_parent = parent;
                    (*p).fts_accpath = ((*p).fts_name).as_mut_ptr();
                    if defer_stat as i32 != 0 && !root.is_null() {
                        (*p).fts_info = 11 as i32 as libc::c_ushort;
                        fts_set_stat_required(p, 1 as i32 != 0);
                    } else {
                        (*p).fts_info = fts_stat(sp, p, 0 as i32 != 0);
                    }
                    if compar.is_some() {
                        (*p).fts_link = root;
                        root = p;
                    } else {
                        (*p).fts_link = 0 as *mut _ftsent;
                        if root.is_null() {
                            root = p;
                            tmp = root;
                        } else {
                            (*tmp).fts_link = p;
                            tmp = p;
                        }
                    }
                    argv = argv.offset(1);
                    argv;
                    nitems = nitems.wrapping_add(1);
                    nitems;
                }
                match current_block {
                    10930818133215224067 => {
                        if compar.is_some() && nitems > 1 as i32 as u64 {
                            root = fts_sort(sp, root, nitems);
                        }
                        (*sp).fts_cur = fts_alloc(
                            sp,
                            b"\0" as *const u8 as *const i8,
                            0 as i32 as size_t,
                        );
                        if !((*sp).fts_cur).is_null() {
                            (*(*sp).fts_cur).fts_link = root;
                            (*(*sp).fts_cur).fts_info = 9 as i32 as libc::c_ushort;
                            (*(*sp).fts_cur).fts_level = 1 as i32 as ptrdiff_t;
                            if setup_dir(sp) {
                                if (*sp).fts_options & 0x4 as i32 == 0
                                    && (*sp).fts_options & 0x200 as i32 == 0
                                    && {
                                        (*sp).fts_rfd = diropen(
                                            sp,
                                            b".\0" as *const u8 as *const i8,
                                        );
                                        (*sp).fts_rfd < 0 as i32
                                    }
                                {
                                    (*sp).fts_options |= 0x4 as i32;
                                }
                                i_ring_init(&mut (*sp).fts_fd_ring, -(1 as i32));
                                return sp;
                            }
                        }
                    }
                    _ => {}
                }
                fts_lfree(root);
                rpl_free(parent as *mut libc::c_void);
            }
            _ => {}
        }
        rpl_free((*sp).fts_path as *mut libc::c_void);
    }
    rpl_free(sp as *mut libc::c_void);
    return 0 as *mut FTS;
}
unsafe extern "C" fn fts_load(mut sp: *mut FTS, mut p: *mut FTSENT) {
    let mut len: size_t = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    (*p).fts_pathlen = (*p).fts_namelen;
    len = (*p).fts_pathlen;
    memmove(
        (*sp).fts_path as *mut libc::c_void,
        ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
        len.wrapping_add(1 as i32 as u64),
    );
    cp = strrchr(((*p).fts_name).as_mut_ptr(), '/' as i32);
    if !cp.is_null()
        && (cp != ((*p).fts_name).as_mut_ptr()
            || *cp.offset(1 as i32 as isize) as i32 != 0)
    {
        cp = cp.offset(1);
        len = strlen(cp);
        memmove(
            ((*p).fts_name).as_mut_ptr() as *mut libc::c_void,
            cp as *const libc::c_void,
            len.wrapping_add(1 as i32 as u64),
        );
        (*p).fts_namelen = len;
    }
    (*p).fts_path = (*sp).fts_path;
    (*p).fts_accpath = (*p).fts_path;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fts_close(mut sp: *mut FTS) -> i32 {
    let mut freep: *mut FTSENT = 0 as *mut FTSENT;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut saved_errno: i32 = 0 as i32;
    if !((*sp).fts_cur).is_null() {
        p = (*sp).fts_cur;
        while (*p).fts_level >= 0 as i32 as i64 {
            freep = p;
            p = if !((*p).fts_link).is_null() { (*p).fts_link } else { (*p).fts_parent };
            rpl_free(freep as *mut libc::c_void);
        }
        rpl_free(p as *mut libc::c_void);
    }
    if !((*sp).fts_child).is_null() {
        fts_lfree((*sp).fts_child);
    }
    rpl_free((*sp).fts_array as *mut libc::c_void);
    rpl_free((*sp).fts_path as *mut libc::c_void);
    if (*sp).fts_options & 0x200 as i32 != 0 {
        if 0 as i32 <= (*sp).fts_cwd_fd {
            if close((*sp).fts_cwd_fd) != 0 {
                saved_errno = *__errno_location();
            }
        }
    } else if (*sp).fts_options & 0x4 as i32 == 0 {
        if fchdir((*sp).fts_rfd) != 0 {
            saved_errno = *__errno_location();
        }
        if close((*sp).fts_rfd) != 0 {
            if saved_errno == 0 as i32 {
                saved_errno = *__errno_location();
            }
        }
    }
    fd_ring_clear(&mut (*sp).fts_fd_ring);
    if !((*sp).fts_leaf_optimization_works_ht).is_null() {
        hash_free((*sp).fts_leaf_optimization_works_ht);
    }
    free_dir(sp);
    rpl_free(sp as *mut libc::c_void);
    if saved_errno != 0 {
        *__errno_location() = saved_errno;
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn dev_type_hash(
    mut x: *const libc::c_void,
    mut table_size: size_t,
) -> size_t {
    let mut ax: *const dev_type = x as *const dev_type;
    let mut dev: uintmax_t = (*ax).st_dev;
    return dev.wrapping_rem(table_size);
}
unsafe extern "C" fn dev_type_compare(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> bool {
    let mut ax: *const dev_type = x as *const dev_type;
    let mut ay: *const dev_type = y as *const dev_type;
    return (*ax).st_dev == (*ay).st_dev;
}
unsafe extern "C" fn filesystem_type(mut p: *const FTSENT, mut fd: i32) -> fsword {
    let mut sp: *mut FTS = (*p).fts_fts;
    let mut h: *mut Hash_table = (*sp).fts_leaf_optimization_works_ht;
    let mut ent: *mut dev_type = 0 as *mut dev_type;
    let mut fs_buf: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    if (*sp).fts_options & 0x200 as i32 == 0 {
        return 0 as i32 as fsword;
    }
    if h.is_null() {
        (*sp).fts_leaf_optimization_works_ht = hash_initialize(
            C2RustUnnamed_4::DEV_TYPE_HT_INITIAL_SIZE as i32 as size_t,
            0 as *const Hash_tuning,
            Some(
                dev_type_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                dev_type_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        h = (*sp).fts_leaf_optimization_works_ht;
    }
    if !h.is_null() {
        let mut tmp: dev_type = dev_type { st_dev: 0, f_type: 0 };
        tmp.st_dev = (*((*p).fts_statp).as_ptr()).st_dev;
        ent = hash_lookup(h, &mut tmp as *mut dev_type as *const libc::c_void)
            as *mut dev_type;
        if !ent.is_null() {
            return (*ent).f_type;
        }
    }
    if fd < 0 as i32 || fstatfs(fd, &mut fs_buf) != 0 as i32 {
        return 0 as i32 as fsword;
    }
    if !h.is_null() {
        let mut t2: *mut dev_type = malloc(::core::mem::size_of::<dev_type>() as u64)
            as *mut dev_type;
        if !t2.is_null() {
            (*t2).st_dev = (*((*p).fts_statp).as_ptr()).st_dev;
            (*t2).f_type = fs_buf.f_type;
            ent = hash_insert(h, t2 as *const libc::c_void) as *mut dev_type;
            if !ent.is_null() {
                if !(ent == t2) {
                    abort();
                }
            } else {
                rpl_free(t2 as *mut libc::c_void);
            }
        }
    }
    return fs_buf.f_type;
}
unsafe extern "C" fn dirent_inode_sort_may_be_useful(
    mut p: *const FTSENT,
    mut dir_fd: i32,
) -> bool {
    match filesystem_type(p, dir_fd) {
        4283649346 | 26985 | 16914836 => return 0 as i32 != 0,
        _ => return 1 as i32 != 0,
    };
}
unsafe extern "C" fn leaf_optimization(
    mut p: *const FTSENT,
    mut dir_fd: i32,
) -> leaf_optimization {
    match filesystem_type(p, dir_fd) {
        1397113167 => {}
        4283649346 => {}
        26985 => {}
        0 | 40864 => {}
        _ => return leaf_optimization::OK_LEAF_OPTIMIZATION,
    }
    return leaf_optimization::NO_LEAF_OPTIMIZATION;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fts_read(mut sp: *mut FTS) -> *mut FTSENT {
    let mut current_block: u64;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut tmp: *mut FTSENT = 0 as *mut FTSENT;
    let mut instr: libc::c_ushort = 0;
    let mut t: *mut i8 = 0 as *mut i8;
    if ((*sp).fts_cur).is_null() || (*sp).fts_options & 0x2000 as i32 != 0 {
        return 0 as *mut FTSENT;
    }
    p = (*sp).fts_cur;
    instr = (*p).fts_instr;
    (*p).fts_instr = 3 as i32 as libc::c_ushort;
    if instr as i32 == 1 as i32 {
        (*p).fts_info = fts_stat(sp, p, 0 as i32 != 0);
        return p;
    }
    if instr as i32 == 2 as i32
        && ((*p).fts_info as i32 == 12 as i32 || (*p).fts_info as i32 == 13 as i32)
    {
        (*p).fts_info = fts_stat(sp, p, 1 as i32 != 0);
        if (*p).fts_info as i32 == 1 as i32 && (*sp).fts_options & 0x4 as i32 == 0 {
            (*p).fts_symfd = diropen(sp, b".\0" as *const u8 as *const i8);
            if (*p).fts_symfd < 0 as i32 {
                (*p).fts_errno = *__errno_location();
                (*p).fts_info = 7 as i32 as libc::c_ushort;
            } else {
                (*p).fts_flags = ((*p).fts_flags as i32 | 0x2 as i32) as libc::c_ushort;
            }
        }
    } else {
        if (*p).fts_info as i32 == 1 as i32 {
            if instr as i32 == 4 as i32
                || (*sp).fts_options & 0x40 as i32 != 0
                    && (*((*p).fts_statp).as_mut_ptr()).st_dev != (*sp).fts_dev
            {
                if (*p).fts_flags as i32 & 0x2 as i32 != 0 {
                    close((*p).fts_symfd);
                }
                if !((*sp).fts_child).is_null() {
                    fts_lfree((*sp).fts_child);
                    (*sp).fts_child = 0 as *mut _ftsent;
                }
                (*p).fts_info = 6 as i32 as libc::c_ushort;
                leave_dir(sp, p);
                return p;
            }
            if !((*sp).fts_child).is_null() && (*sp).fts_options & 0x1000 as i32 != 0 {
                (*sp).fts_options &= !(0x1000 as i32);
                fts_lfree((*sp).fts_child);
                (*sp).fts_child = 0 as *mut _ftsent;
            }
            if !((*sp).fts_child).is_null() {
                if fts_safe_changedir(sp, p, -(1 as i32), (*p).fts_accpath) != 0 {
                    (*p).fts_errno = *__errno_location();
                    (*p).fts_flags = ((*p).fts_flags as i32 | 0x1 as i32)
                        as libc::c_ushort;
                    p = (*sp).fts_child;
                    while !p.is_null() {
                        (*p).fts_accpath = (*(*p).fts_parent).fts_accpath;
                        p = (*p).fts_link;
                    }
                }
            } else {
                (*sp).fts_child = fts_build(sp, 3 as i32);
                if ((*sp).fts_child).is_null() {
                    if (*sp).fts_options & 0x2000 as i32 != 0 {
                        return 0 as *mut FTSENT;
                    }
                    if (*p).fts_errno != 0 && (*p).fts_info as i32 != 4 as i32 {
                        (*p).fts_info = 7 as i32 as libc::c_ushort;
                    }
                    leave_dir(sp, p);
                    return p;
                }
            }
            p = (*sp).fts_child;
            (*sp).fts_child = 0 as *mut _ftsent;
            current_block = 8586622911422059577;
        } else {
            loop {
                tmp = p;
                if ((*p).fts_link).is_null() && !((*(*p).fts_parent).fts_dirp).is_null()
                {
                    p = (*tmp).fts_parent;
                    (*sp).fts_cur = p;
                    *((*sp).fts_path).offset((*p).fts_pathlen as isize) = '\0' as i32
                        as i8;
                    p = fts_build(sp, 3 as i32);
                    if p.is_null() {
                        current_block = 13321564401369230990;
                        break;
                    } else {
                        current_block = 9353995356876505083;
                        break;
                    }
                } else {
                    p = (*p).fts_link;
                    if p.is_null() {
                        current_block = 12225841302944392257;
                        break;
                    }
                    (*sp).fts_cur = p;
                    rpl_free(tmp as *mut libc::c_void);
                    if (*p).fts_level == 0 as i32 as i64 {
                        if restore_initial_cwd(sp) != 0 {
                            (*sp).fts_options |= 0x2000 as i32;
                            return 0 as *mut FTSENT;
                        }
                        free_dir(sp);
                        fts_load(sp, p);
                        setup_dir(sp);
                        current_block = 9854792751334759659;
                        break;
                    } else {
                        if (*p).fts_instr as i32 == 4 as i32 {
                            continue;
                        }
                        if (*p).fts_instr as i32 == 2 as i32 {
                            (*p).fts_info = fts_stat(sp, p, 1 as i32 != 0);
                            if (*p).fts_info as i32 == 1 as i32
                                && (*sp).fts_options & 0x4 as i32 == 0
                            {
                                (*p).fts_symfd = diropen(
                                    sp,
                                    b".\0" as *const u8 as *const i8,
                                );
                                if (*p).fts_symfd < 0 as i32 {
                                    (*p).fts_errno = *__errno_location();
                                    (*p).fts_info = 7 as i32 as libc::c_ushort;
                                } else {
                                    (*p).fts_flags = ((*p).fts_flags as i32 | 0x2 as i32)
                                        as libc::c_ushort;
                                }
                            }
                            (*p).fts_instr = 3 as i32 as libc::c_ushort;
                        }
                        current_block = 8586622911422059577;
                        break;
                    }
                }
            }
            match current_block {
                9854792751334759659 => {}
                8586622911422059577 => {}
                _ => {
                    match current_block {
                        9353995356876505083 => {
                            rpl_free(tmp as *mut libc::c_void);
                            current_block = 8586622911422059577;
                        }
                        13321564401369230990 => {
                            if (*sp).fts_options & 0x2000 as i32 != 0 {
                                return 0 as *mut FTSENT;
                            }
                            current_block = 12225841302944392257;
                        }
                        _ => {}
                    }
                    match current_block {
                        8586622911422059577 => {}
                        _ => {
                            p = (*tmp).fts_parent;
                            (*sp).fts_cur = p;
                            rpl_free(tmp as *mut libc::c_void);
                            if (*p).fts_level == -(1 as i32) as i64 {
                                rpl_free(p as *mut libc::c_void);
                                *__errno_location() = 0 as i32;
                                (*sp).fts_cur = 0 as *mut _ftsent;
                                return (*sp).fts_cur;
                            }
                            if !((*p).fts_info as i32 != 11 as i32) {
                                abort();
                            }
                            *((*sp).fts_path).offset((*p).fts_pathlen as isize) = '\0'
                                as i32 as i8;
                            if (*p).fts_level == 0 as i32 as i64 {
                                if restore_initial_cwd(sp) != 0 {
                                    (*p).fts_errno = *__errno_location();
                                    (*sp).fts_options |= 0x2000 as i32;
                                }
                            } else if (*p).fts_flags as i32 & 0x2 as i32 != 0 {
                                if (*sp).fts_options & 0x4 as i32 == 0
                                    && (if (*sp).fts_options & 0x200 as i32 != 0 {
                                        cwd_advance_fd(sp, (*p).fts_symfd, 1 as i32 != 0);
                                        0 as i32
                                    } else {
                                        fchdir((*p).fts_symfd)
                                    }) != 0
                                {
                                    (*p).fts_errno = *__errno_location();
                                    (*sp).fts_options |= 0x2000 as i32;
                                }
                                close((*p).fts_symfd);
                            } else if (*p).fts_flags as i32 & 0x1 as i32 == 0
                                && fts_safe_changedir(
                                    sp,
                                    (*p).fts_parent,
                                    -(1 as i32),
                                    b"..\0" as *const u8 as *const i8,
                                ) != 0
                            {
                                (*p).fts_errno = *__errno_location();
                                (*sp).fts_options |= 0x2000 as i32;
                            }
                            if (*p).fts_info as i32 != 2 as i32 {
                                (*p).fts_info = (if (*p).fts_errno != 0 {
                                    7 as i32
                                } else {
                                    6 as i32
                                }) as libc::c_ushort;
                                if (*p).fts_errno == 0 as i32 {
                                    leave_dir(sp, p);
                                }
                            }
                            return if (*sp).fts_options & 0x2000 as i32 != 0 {
                                0 as *mut FTSENT
                            } else {
                                p
                            };
                        }
                    }
                }
            }
        }
        match current_block {
            9854792751334759659 => {}
            _ => {
                t = ((*sp).fts_path)
                    .offset(
                        (if *((*(*p).fts_parent).fts_path)
                            .offset(
                                ((*(*p).fts_parent).fts_pathlen)
                                    .wrapping_sub(1 as i32 as u64) as isize,
                            ) as i32 == '/' as i32
                        {
                            ((*(*p).fts_parent).fts_pathlen)
                                .wrapping_sub(1 as i32 as u64)
                        } else {
                            (*(*p).fts_parent).fts_pathlen
                        }) as isize,
                    );
                let fresh0 = t;
                t = t.offset(1);
                *fresh0 = '/' as i32 as i8;
                memmove(
                    t as *mut libc::c_void,
                    ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
                    ((*p).fts_namelen).wrapping_add(1 as i32 as u64),
                );
            }
        }
    }
    (*sp).fts_cur = p;
    if (*p).fts_info as i32 == 11 as i32 {
        if (*((*p).fts_statp).as_mut_ptr()).st_size
            == Fts_stat::FTS_STAT_REQUIRED as i32 as i64
        {
            (*p).fts_info = fts_stat(sp, p, 0 as i32 != 0);
        } else if !((*((*p).fts_statp).as_mut_ptr()).st_size
            == Fts_stat::FTS_NO_STAT_REQUIRED as i32 as i64)
        {
            abort();
        }
    }
    if (*p).fts_info as i32 == 1 as i32 {
        if (*p).fts_level == 0 as i32 as i64 {
            (*sp).fts_dev = (*((*p).fts_statp).as_mut_ptr()).st_dev;
        }
        if !enter_dir(sp, p) {
            *__errno_location() = 12 as i32;
            return 0 as *mut FTSENT;
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fts_set(
    mut sp: *mut FTS,
    mut p: *mut FTSENT,
    mut instr: i32,
) -> i32 {
    if instr != 0 as i32 && instr != 1 as i32 && instr != 2 as i32 && instr != 3 as i32
        && instr != 4 as i32
    {
        *__errno_location() = 22 as i32;
        return 1 as i32;
    }
    (*p).fts_instr = instr as libc::c_ushort;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_fts_children(
    mut sp: *mut FTS,
    mut instr: i32,
) -> *mut FTSENT {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut fd: i32 = 0;
    if instr != 0 as i32 && instr != 0x1000 as i32 {
        *__errno_location() = 22 as i32;
        return 0 as *mut FTSENT;
    }
    p = (*sp).fts_cur;
    *__errno_location() = 0 as i32;
    if (*sp).fts_options & 0x2000 as i32 != 0 {
        return 0 as *mut FTSENT;
    }
    if (*p).fts_info as i32 == 9 as i32 {
        return (*p).fts_link;
    }
    if (*p).fts_info as i32 != 1 as i32 {
        return 0 as *mut FTSENT;
    }
    if !((*sp).fts_child).is_null() {
        fts_lfree((*sp).fts_child);
    }
    if instr == 0x1000 as i32 {
        (*sp).fts_options |= 0x1000 as i32;
        instr = 2 as i32;
    } else {
        instr = 1 as i32;
    }
    if (*p).fts_level != 0 as i32 as i64
        || *((*p).fts_accpath).offset(0 as i32 as isize) as i32 == '/' as i32
        || (*sp).fts_options & 0x4 as i32 != 0
    {
        (*sp).fts_child = fts_build(sp, instr);
        return (*sp).fts_child;
    }
    fd = diropen(sp, b".\0" as *const u8 as *const i8);
    if fd < 0 as i32 {
        (*sp).fts_child = 0 as *mut _ftsent;
        return (*sp).fts_child;
    }
    (*sp).fts_child = fts_build(sp, instr);
    if (*sp).fts_options & 0x200 as i32 != 0 {
        cwd_advance_fd(sp, fd, 1 as i32 != 0);
    } else {
        if fchdir(fd) != 0 {
            let mut saved_errno: i32 = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
            return 0 as *mut FTSENT;
        }
        close(fd);
    }
    return (*sp).fts_child;
}
unsafe extern "C" fn fts_compare_ino(
    mut a: *mut *const _ftsent,
    mut b: *mut *const _ftsent,
) -> i32 {
    return ((*((**a.offset(0 as i32 as isize)).fts_statp).as_ptr()).st_ino
        > (*((**b.offset(0 as i32 as isize)).fts_statp).as_ptr()).st_ino) as i32
        - ((*((**a.offset(0 as i32 as isize)).fts_statp).as_ptr()).st_ino
            < (*((**b.offset(0 as i32 as isize)).fts_statp).as_ptr()).st_ino) as i32;
}
unsafe extern "C" fn set_stat_type(mut st: *mut stat, mut dtype: u32) {
    let mut type_0: mode_t = 0;
    match dtype {
        6 => {
            type_0 = 0o60000 as i32 as mode_t;
        }
        2 => {
            type_0 = 0o20000 as i32 as mode_t;
        }
        4 => {
            type_0 = 0o40000 as i32 as mode_t;
        }
        1 => {
            type_0 = 0o10000 as i32 as mode_t;
        }
        10 => {
            type_0 = 0o120000 as i32 as mode_t;
        }
        8 => {
            type_0 = 0o100000 as i32 as mode_t;
        }
        12 => {
            type_0 = 0o140000 as i32 as mode_t;
        }
        _ => {
            type_0 = 0 as i32 as mode_t;
        }
    }
    (*st).st_mode = type_0;
}
unsafe extern "C" fn fts_build(mut sp: *mut FTS, mut type_0: i32) -> *mut FTSENT {
    let mut current_block: u64;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut head: *mut FTSENT = 0 as *mut FTSENT;
    let mut nitems: size_t = 0;
    let mut tail: *mut FTSENT = 0 as *mut FTSENT;
    let mut oldaddr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut saved_errno: i32 = 0;
    let mut descend: bool = false;
    let mut doadjust: bool = false;
    let mut level: ptrdiff_t = 0;
    let mut len: size_t = 0;
    let mut maxlen: size_t = 0;
    let mut new_len: size_t = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut dir_fd: i32 = 0;
    let mut cur: *mut FTSENT = (*sp).fts_cur;
    let mut continue_readdir: bool = !((*cur).fts_dirp).is_null();
    let mut sort_by_inode: bool = 0 as i32 != 0;
    let mut max_entries: size_t = 0;
    if continue_readdir {
        let mut dp: *mut DIR = (*cur).fts_dirp;
        dir_fd = dirfd(dp);
        if dir_fd < 0 as i32 {
            closedir((*cur).fts_dirp);
            (*cur).fts_dirp = 0 as *mut DIR;
            if type_0 == 3 as i32 {
                (*cur).fts_info = 4 as i32 as libc::c_ushort;
                (*cur).fts_errno = *__errno_location();
            }
            return 0 as *mut FTSENT;
        }
    } else {
        (*cur).fts_dirp = opendirat(
            if (*sp).fts_options & 0x4 as i32 == 0
                && (*sp).fts_options & 0x200 as i32 != 0
            {
                (*sp).fts_cwd_fd
            } else {
                -(100 as i32)
            },
            (*cur).fts_accpath,
            if (*sp).fts_options & 0x10 as i32 != 0
                && !((*sp).fts_options & 0x1 as i32 != 0
                    && (*cur).fts_level == 0 as i32 as i64)
            {
                0o400000 as i32
            } else {
                0 as i32
            },
            &mut dir_fd,
        );
        if ((*cur).fts_dirp).is_null() {
            if type_0 == 3 as i32 {
                (*cur).fts_info = 4 as i32 as libc::c_ushort;
                (*cur).fts_errno = *__errno_location();
            }
            return 0 as *mut FTSENT;
        }
        if (*cur).fts_info as i32 == 11 as i32 {
            (*cur).fts_info = fts_stat(sp, cur, 0 as i32 != 0);
        } else if (*sp).fts_options & 0x100 as i32 != 0 {
            leave_dir(sp, cur);
            fts_stat(sp, cur, 0 as i32 != 0);
            if !enter_dir(sp, cur) {
                *__errno_location() = 12 as i32;
                return 0 as *mut FTSENT;
            }
        }
    }
    max_entries = if ((*sp).fts_compar).is_some() {
        18446744073709551615 as u64
    } else {
        100000 as i32 as u64
    };
    if continue_readdir {
        descend = 1 as i32 != 0;
    } else {
        descend = type_0 != 2 as i32
            && !((*sp).fts_options & 0x8 as i32 != 0
                && (*sp).fts_options & 0x10 as i32 != 0
                && (*sp).fts_options & 0x20 as i32 == 0
                && (*((*cur).fts_statp).as_mut_ptr()).st_nlink
                    == C2RustUnnamed_3::MIN_DIR_NLINK as i32 as u64
                && leaf_optimization(cur, dir_fd) as u32
                    != leaf_optimization::NO_LEAF_OPTIMIZATION as i32 as u32);
        if descend as i32 != 0 || type_0 == 3 as i32 {
            if (*sp).fts_options & 0x200 as i32 != 0 {
                dir_fd = rpl_fcntl(dir_fd, 1030 as i32, 2 as i32 + 1 as i32);
            }
            if dir_fd < 0 as i32
                || fts_safe_changedir(sp, cur, dir_fd, 0 as *const i8) != 0
            {
                if descend as i32 != 0 && type_0 == 3 as i32 {
                    (*cur).fts_errno = *__errno_location();
                }
                (*cur).fts_flags = ((*cur).fts_flags as i32 | 0x1 as i32)
                    as libc::c_ushort;
                descend = 0 as i32 != 0;
                closedir((*cur).fts_dirp);
                (*cur).fts_dirp = 0 as *mut DIR;
                if (*sp).fts_options & 0x200 as i32 != 0 && 0 as i32 <= dir_fd {
                    close(dir_fd);
                }
                (*cur).fts_dirp = 0 as *mut DIR;
            } else {
                descend = 1 as i32 != 0;
            }
        }
    }
    len = if *((*cur).fts_path)
        .offset(((*cur).fts_pathlen).wrapping_sub(1 as i32 as u64) as isize) as i32
        == '/' as i32
    {
        ((*cur).fts_pathlen).wrapping_sub(1 as i32 as u64)
    } else {
        (*cur).fts_pathlen
    };
    if (*sp).fts_options & 0x4 as i32 != 0 {
        cp = ((*sp).fts_path).offset(len as isize);
        let fresh1 = cp;
        cp = cp.offset(1);
        *fresh1 = '/' as i32 as i8;
    } else {
        cp = 0 as *mut i8;
    }
    len = len.wrapping_add(1);
    len;
    maxlen = ((*sp).fts_pathlen).wrapping_sub(len);
    level = (*cur).fts_level + 1 as i32 as i64;
    doadjust = 0 as i32 != 0;
    head = 0 as *mut FTSENT;
    tail = 0 as *mut FTSENT;
    nitems = 0 as i32 as size_t;
    loop {
        if ((*cur).fts_dirp).is_null() {
            current_block = 17623951255125923504;
            break;
        }
        let mut d_namelen: size_t = 0;
        *__errno_location() = 0 as i32;
        let mut dp_0: *mut dirent = readdir((*cur).fts_dirp);
        if dp_0.is_null() {
            if *__errno_location() != 0 {
                (*cur).fts_errno = *__errno_location();
                (*cur).fts_info = (if continue_readdir as i32 != 0 || nitems != 0 {
                    7 as i32
                } else {
                    4 as i32
                }) as libc::c_ushort;
            }
            current_block = 17623951255125923504;
            break;
        } else {
            if (*sp).fts_options & 0x20 as i32 == 0
                && ((*dp_0).d_name[0 as i32 as usize] as i32 == '.' as i32
                    && ((*dp_0).d_name[1 as i32 as usize] == 0
                        || (*dp_0).d_name[1 as i32 as usize] as i32 == '.' as i32
                            && (*dp_0).d_name[2 as i32 as usize] == 0))
            {
                continue;
            }
            d_namelen = strlen(((*dp_0).d_name).as_mut_ptr());
            p = fts_alloc(sp, ((*dp_0).d_name).as_mut_ptr(), d_namelen);
            if !p.is_null() {
                if d_namelen >= maxlen {
                    oldaddr = (*sp).fts_path as *mut libc::c_void;
                    if !fts_palloc(
                        sp,
                        d_namelen.wrapping_add(len).wrapping_add(1 as i32 as u64),
                    ) {
                        current_block = 14268776075521355435;
                    } else {
                        if oldaddr != (*sp).fts_path as *mut libc::c_void {
                            doadjust = 1 as i32 != 0;
                            if (*sp).fts_options & 0x4 as i32 != 0 {
                                cp = ((*sp).fts_path).offset(len as isize);
                            }
                        }
                        maxlen = ((*sp).fts_pathlen).wrapping_sub(len);
                        current_block = 17688141731389699982;
                    }
                } else {
                    current_block = 17688141731389699982;
                }
                match current_block {
                    14268776075521355435 => {}
                    _ => {
                        new_len = len.wrapping_add(d_namelen);
                        if new_len < len {
                            rpl_free(p as *mut libc::c_void);
                            fts_lfree(head);
                            closedir((*cur).fts_dirp);
                            (*cur).fts_dirp = 0 as *mut DIR;
                            (*cur).fts_info = 7 as i32 as libc::c_ushort;
                            (*sp).fts_options |= 0x2000 as i32;
                            *__errno_location() = 36 as i32;
                            return 0 as *mut FTSENT;
                        }
                        (*p).fts_level = level;
                        (*p).fts_parent = (*sp).fts_cur;
                        (*p).fts_pathlen = new_len;
                        (*((*p).fts_statp).as_mut_ptr()).st_ino = (*dp_0).d_ino;
                        if (*sp).fts_options & 0x4 as i32 != 0 {
                            (*p).fts_accpath = (*p).fts_path;
                            memmove(
                                cp as *mut libc::c_void,
                                ((*p).fts_name).as_mut_ptr() as *const libc::c_void,
                                ((*p).fts_namelen).wrapping_add(1 as i32 as u64),
                            );
                        } else {
                            (*p).fts_accpath = ((*p).fts_name).as_mut_ptr();
                        }
                        if ((*sp).fts_compar).is_none()
                            || (*sp).fts_options & 0x400 as i32 != 0
                        {
                            let mut skip_stat: bool = (*sp).fts_options & 0x8 as i32 != 0
                                && (*dp_0).d_type as i32 != C2RustUnnamed::DT_UNKNOWN as i32
                                && !((*dp_0).d_type as i32 == C2RustUnnamed::DT_DIR as i32)
                                && ((*sp).fts_options & 0x10 as i32 != 0
                                    || !((*dp_0).d_type as i32
                                        == C2RustUnnamed::DT_LNK as i32));
                            (*p).fts_info = 11 as i32 as libc::c_ushort;
                            set_stat_type(
                                ((*p).fts_statp).as_mut_ptr(),
                                (*dp_0).d_type as u32,
                            );
                            fts_set_stat_required(p, !skip_stat);
                        } else {
                            (*p).fts_info = fts_stat(sp, p, 0 as i32 != 0);
                        }
                        (*p).fts_link = 0 as *mut _ftsent;
                        if head.is_null() {
                            tail = p;
                            head = tail;
                        } else {
                            (*tail).fts_link = p;
                            tail = p;
                        }
                        if nitems
                            == C2RustUnnamed_2::_FTS_INODE_SORT_DIR_ENTRIES_THRESHOLD
                                as i32 as u64 && ((*sp).fts_compar).is_none()
                        {
                            sort_by_inode = dirent_inode_sort_may_be_useful(cur, dir_fd);
                        }
                        nitems = nitems.wrapping_add(1);
                        nitems;
                        if max_entries <= nitems {
                            current_block = 15966040735278438222;
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            }
            saved_errno = *__errno_location();
            rpl_free(p as *mut libc::c_void);
            fts_lfree(head);
            closedir((*cur).fts_dirp);
            (*cur).fts_dirp = 0 as *mut DIR;
            (*cur).fts_info = 7 as i32 as libc::c_ushort;
            (*sp).fts_options |= 0x2000 as i32;
            *__errno_location() = saved_errno;
            return 0 as *mut FTSENT;
        }
    }
    match current_block {
        17623951255125923504 => {
            if !((*cur).fts_dirp).is_null() {
                closedir((*cur).fts_dirp);
                (*cur).fts_dirp = 0 as *mut DIR;
            }
        }
        _ => {}
    }
    if doadjust {
        fts_padjust(sp, head);
    }
    if (*sp).fts_options & 0x4 as i32 != 0 {
        if len == (*sp).fts_pathlen || nitems == 0 as i32 as u64 {
            cp = cp.offset(-1);
            cp;
        }
        *cp = '\0' as i32 as i8;
    }
    if !continue_readdir && descend as i32 != 0 && (type_0 == 1 as i32 || nitems == 0)
        && (if (*cur).fts_level == 0 as i32 as i64 {
            restore_initial_cwd(sp)
        } else {
            fts_safe_changedir(
                sp,
                (*cur).fts_parent,
                -(1 as i32),
                b"..\0" as *const u8 as *const i8,
            )
        }) != 0
    {
        (*cur).fts_info = 7 as i32 as libc::c_ushort;
        (*sp).fts_options |= 0x2000 as i32;
        fts_lfree(head);
        return 0 as *mut FTSENT;
    }
    if nitems == 0 {
        if type_0 == 3 as i32 && (*cur).fts_info as i32 != 4 as i32
            && (*cur).fts_info as i32 != 7 as i32
        {
            (*cur).fts_info = 6 as i32 as libc::c_ushort;
        }
        fts_lfree(head);
        return 0 as *mut FTSENT;
    }
    if sort_by_inode {
        (*sp).fts_compar = Some(
            fts_compare_ino
                as unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> i32,
        );
        head = fts_sort(sp, head, nitems);
        (*sp).fts_compar = None;
    }
    if ((*sp).fts_compar).is_some() && nitems > 1 as i32 as u64 {
        head = fts_sort(sp, head, nitems);
    }
    return head;
}
unsafe extern "C" fn fts_stat(
    mut sp: *mut FTS,
    mut p: *mut FTSENT,
    mut follow: bool,
) -> libc::c_ushort {
    let mut sbp: *mut stat = ((*p).fts_statp).as_mut_ptr();
    if (*p).fts_level == 0 as i32 as i64 && (*sp).fts_options & 0x1 as i32 != 0 {
        follow = 1 as i32 != 0;
    }
    let mut current_block_9: u64;
    if (*sp).fts_options & 0x2 as i32 != 0 || follow as i32 != 0 {
        if stat((*p).fts_accpath, sbp) != 0 {
            if *__errno_location() == 2 as i32
                && lstat((*p).fts_accpath, sbp) == 0 as i32
            {
                *__errno_location() = 0 as i32;
                return 13 as i32 as libc::c_ushort;
            }
            (*p).fts_errno = *__errno_location();
            current_block_9 = 15814601397147744907;
        } else {
            current_block_9 = 13536709405535804910;
        }
    } else if fstatat((*sp).fts_cwd_fd, (*p).fts_accpath, sbp, 0x100 as i32) != 0 {
        (*p).fts_errno = *__errno_location();
        current_block_9 = 15814601397147744907;
    } else {
        current_block_9 = 13536709405535804910;
    }
    match current_block_9 {
        13536709405535804910 => {}
        _ => {
            memset(
                sbp as *mut libc::c_void,
                0 as i32,
                ::core::mem::size_of::<stat>() as u64,
            );
            return 10 as i32 as libc::c_ushort;
        }
    }
    if (*sbp).st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
        if *((*p).fts_name).as_mut_ptr().offset(0 as i32 as isize) as i32 == '.' as i32
            && (*((*p).fts_name).as_mut_ptr().offset(1 as i32 as isize) == 0
                || *((*p).fts_name).as_mut_ptr().offset(1 as i32 as isize) as i32
                    == '.' as i32
                    && *((*p).fts_name).as_mut_ptr().offset(2 as i32 as isize) == 0)
        {
            return (if (*p).fts_level == 0 as i32 as i64 { 1 as i32 } else { 5 as i32 })
                as libc::c_ushort;
        }
        return 1 as i32 as libc::c_ushort;
    }
    if (*sbp).st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
        return 12 as i32 as libc::c_ushort;
    }
    if (*sbp).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
        return 8 as i32 as libc::c_ushort;
    }
    return 3 as i32 as libc::c_ushort;
}
unsafe extern "C" fn fts_compar(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> i32 {
    let mut pa: *mut *const FTSENT = a as *mut *const FTSENT;
    let mut pb: *mut *const FTSENT = b as *mut *const FTSENT;
    return ((*(**pa.offset(0 as i32 as isize)).fts_fts).fts_compar)
        .expect("non-null function pointer")(pa, pb);
}
unsafe extern "C" fn fts_sort(
    mut sp: *mut FTS,
    mut head: *mut FTSENT,
    mut nitems: size_t,
) -> *mut FTSENT {
    let mut ap: *mut *mut FTSENT = 0 as *mut *mut FTSENT;
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut dummy: *mut FTSENT = 0 as *mut FTSENT;
    let mut compare: Option<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
    > = if ::core::mem::size_of::<*mut *mut FTSENT>() as u64
        == ::core::mem::size_of::<*mut libc::c_void>() as u64
        && &mut dummy as *mut *mut FTSENT as i64
            == &mut dummy as *mut *mut FTSENT as *mut libc::c_void as i64
    {
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> i32,
            >,
            Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>,
        >((*sp).fts_compar)
    } else {
        Some(
            fts_compar
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        )
    };
    if nitems > (*sp).fts_nitems {
        let mut a: *mut *mut FTSENT = 0 as *mut *mut FTSENT;
        (*sp).fts_nitems = nitems.wrapping_add(40 as i32 as u64);
        if (18446744073709551615 as u64)
            .wrapping_div(::core::mem::size_of::<*mut FTSENT>() as u64)
            < (*sp).fts_nitems
            || {
                a = realloc(
                    (*sp).fts_array as *mut libc::c_void,
                    ((*sp).fts_nitems)
                        .wrapping_mul(::core::mem::size_of::<*mut FTSENT>() as u64),
                ) as *mut *mut FTSENT;
                a.is_null()
            }
        {
            rpl_free((*sp).fts_array as *mut libc::c_void);
            (*sp).fts_array = 0 as *mut *mut _ftsent;
            (*sp).fts_nitems = 0 as i32 as size_t;
            return head;
        }
        (*sp).fts_array = a;
    }
    ap = (*sp).fts_array;
    p = head;
    while !p.is_null() {
        let fresh2 = ap;
        ap = ap.offset(1);
        *fresh2 = p;
        p = (*p).fts_link;
    }
    qsort(
        (*sp).fts_array as *mut libc::c_void,
        nitems,
        ::core::mem::size_of::<*mut FTSENT>() as u64,
        compare,
    );
    ap = (*sp).fts_array;
    head = *ap;
    loop {
        nitems = nitems.wrapping_sub(1);
        if !(nitems != 0) {
            break;
        }
        let ref mut fresh3 = (**ap.offset(0 as i32 as isize)).fts_link;
        *fresh3 = *ap.offset(1 as i32 as isize);
        ap = ap.offset(1);
        ap;
    }
    let ref mut fresh4 = (**ap.offset(0 as i32 as isize)).fts_link;
    *fresh4 = 0 as *mut _ftsent;
    return head;
}
unsafe extern "C" fn fts_alloc(
    mut sp: *mut FTS,
    mut name: *const i8,
    mut namelen: size_t,
) -> *mut FTSENT {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut len: size_t = 0;
    len = (256 as u64)
        .wrapping_add(::core::mem::align_of::<FTSENT>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_add(namelen.wrapping_add(1 as i32 as u64))
        & !(::core::mem::align_of::<FTSENT>() as u64).wrapping_sub(1 as i32 as u64);
    p = malloc(len) as *mut FTSENT;
    if p.is_null() {
        return 0 as *mut FTSENT;
    }
    memcpy(
        ((*p).fts_name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        namelen,
    );
    *((*p).fts_name).as_mut_ptr().offset(namelen as isize) = '\0' as i32 as i8;
    (*p).fts_namelen = namelen;
    (*p).fts_fts = sp;
    (*p).fts_path = (*sp).fts_path;
    (*p).fts_errno = 0 as i32;
    (*p).fts_dirp = 0 as *mut DIR;
    (*p).fts_flags = 0 as i32 as libc::c_ushort;
    (*p).fts_instr = 3 as i32 as libc::c_ushort;
    (*p).fts_number = 0 as i32 as i64;
    (*p).fts_pointer = 0 as *mut libc::c_void;
    return p;
}
unsafe extern "C" fn fts_lfree(mut head: *mut FTSENT) {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    loop {
        p = head;
        if p.is_null() {
            break;
        }
        head = (*head).fts_link;
        if !((*p).fts_dirp).is_null() {
            closedir((*p).fts_dirp);
        }
        rpl_free(p as *mut libc::c_void);
    };
}
unsafe extern "C" fn fts_palloc(mut sp: *mut FTS, mut more: size_t) -> bool {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut new_len: size_t = ((*sp).fts_pathlen)
        .wrapping_add(more)
        .wrapping_add(256 as i32 as u64);
    if new_len < (*sp).fts_pathlen {
        rpl_free((*sp).fts_path as *mut libc::c_void);
        (*sp).fts_path = 0 as *mut i8;
        *__errno_location() = 36 as i32;
        return 0 as i32 != 0;
    }
    (*sp).fts_pathlen = new_len;
    p = realloc((*sp).fts_path as *mut libc::c_void, (*sp).fts_pathlen) as *mut i8;
    if p.is_null() {
        rpl_free((*sp).fts_path as *mut libc::c_void);
        (*sp).fts_path = 0 as *mut i8;
        return 0 as i32 != 0;
    }
    (*sp).fts_path = p;
    return 1 as i32 != 0;
}
unsafe extern "C" fn fts_padjust(mut sp: *mut FTS, mut head: *mut FTSENT) {
    let mut p: *mut FTSENT = 0 as *mut FTSENT;
    let mut addr: *mut i8 = (*sp).fts_path;
    p = (*sp).fts_child;
    while !p.is_null() {
        if (*p).fts_accpath != ((*p).fts_name).as_mut_ptr() {
            (*p).fts_accpath = addr
                .offset(((*p).fts_accpath).offset_from((*p).fts_path) as i64 as isize);
        }
        (*p).fts_path = addr;
        p = (*p).fts_link;
    }
    p = head;
    while (*p).fts_level >= 0 as i32 as i64 {
        if (*p).fts_accpath != ((*p).fts_name).as_mut_ptr() {
            (*p).fts_accpath = addr
                .offset(((*p).fts_accpath).offset_from((*p).fts_path) as i64 as isize);
        }
        (*p).fts_path = addr;
        p = if !((*p).fts_link).is_null() { (*p).fts_link } else { (*p).fts_parent };
    }
}
unsafe extern "C" fn fts_maxarglen(mut argv: *const *mut i8) -> size_t {
    let mut len: size_t = 0;
    let mut max: size_t = 0;
    max = 0 as i32 as size_t;
    while !(*argv).is_null() {
        len = strlen(*argv);
        if len > max {
            max = len;
        }
        argv = argv.offset(1);
        argv;
    }
    return max.wrapping_add(1 as i32 as u64);
}
unsafe extern "C" fn fts_safe_changedir(
    mut sp: *mut FTS,
    mut p: *mut FTSENT,
    mut fd: i32,
    mut dir: *const i8,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0;
    let mut is_dotdot: bool = !dir.is_null()
        && strcmp(dir, b"..\0" as *const u8 as *const i8) == 0 as i32;
    let mut newfd: i32 = 0;
    if (*sp).fts_options & 0x4 as i32 != 0 {
        if (*sp).fts_options & 0x200 as i32 != 0 && 0 as i32 <= fd {
            close(fd);
        }
        return 0 as i32;
    }
    if fd < 0 as i32 && is_dotdot as i32 != 0 && (*sp).fts_options & 0x200 as i32 != 0 {
        if !i_ring_empty(&mut (*sp).fts_fd_ring) {
            let mut parent_fd: i32 = 0;
            parent_fd = i_ring_pop(&mut (*sp).fts_fd_ring);
            if 0 as i32 <= parent_fd {
                fd = parent_fd;
                dir = 0 as *const i8;
            }
        }
    }
    newfd = fd;
    if fd < 0 as i32
        && {
            newfd = diropen(sp, dir);
            newfd < 0 as i32
        }
    {
        return -(1 as i32);
    }
    if (*sp).fts_options & 0x2 as i32 != 0 || 1 as i32 == 0
        || !dir.is_null() && strcmp(dir, b"..\0" as *const u8 as *const i8) == 0 as i32
    {
        let mut sb: stat = stat {
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
        if fstat(newfd, &mut sb) != 0 {
            ret = -(1 as i32);
            current_block = 9428649222103807762;
        } else if (*((*p).fts_statp).as_mut_ptr()).st_dev != sb.st_dev
            || (*((*p).fts_statp).as_mut_ptr()).st_ino != sb.st_ino
        {
            *__errno_location() = 2 as i32;
            ret = -(1 as i32);
            current_block = 9428649222103807762;
        } else {
            current_block = 5601891728916014340;
        }
    } else {
        current_block = 5601891728916014340;
    }
    match current_block {
        5601891728916014340 => {
            if (*sp).fts_options & 0x200 as i32 != 0 {
                cwd_advance_fd(sp, newfd, !is_dotdot);
                return 0 as i32;
            }
            ret = fchdir(newfd);
        }
        _ => {}
    }
    if fd < 0 as i32 {
        let mut oerrno: i32 = *__errno_location();
        close(newfd);
        *__errno_location() = oerrno;
    }
    return ret;
}