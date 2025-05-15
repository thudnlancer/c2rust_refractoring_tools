use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type wlink;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn snprintf(_: *mut i8, _: u64, _: *const i8, _: ...) -> i32;
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    fn recognize_keyword(string: *const i8, found: *mut pool_found) -> bool;
    fn checkssym(sym: *const i8);
    fn checksid(id: *const i8);
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn syserror(e: i32, who: *const i8);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn intern(divvy: *mut divvy, s: *const i8, len: size_t) -> *mut i8;
    static mut single: *mut divvy;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn fro_close(f: *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut i32, f: *mut fro) -> bool;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __uint8_t = u8;
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
pub type uint8_t = __uint8_t;
pub type size_t = u64;
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
pub type off_t = __off_t;
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
pub struct cbuf {
    pub string: *const i8,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const i8,
    pub date: *const i8,
    pub author: *const i8,
    pub lockedby: *const i8,
    pub state: *const i8,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const i8,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const i8,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atat {
    pub count: size_t,
    pub lno: size_t,
    pub line_count: size_t,
    pub from: *mut fro,
    pub beg: off_t,
    pub holes: [off_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fro {
    pub fd: i32,
    pub end: off_t,
    pub rm: readmethod,
    pub ptr: *mut i8,
    pub lim: *mut i8,
    pub base: *mut i8,
    pub deallocate: Option<unsafe extern "C" fn(*mut fro) -> ()>,
    pub stream: *mut FILE,
    pub verbatim: off_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum readmethod {
    RM_MMAP,
    RM_MEM,
    RM_STDIO,
}
impl readmethod {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            readmethod::RM_MMAP => 0,
            readmethod::RM_MEM => 1,
            readmethod::RM_STDIO => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> readmethod {
        match value {
            0 => readmethod::RM_MMAP,
            1 => readmethod::RM_MEM,
            2 => readmethod::RM_STDIO,
            _ => panic!("Invalid value for readmethod: {}", value),
        }
    }
}
impl AddAssign<u32> for readmethod {
    fn add_assign(&mut self, rhs: u32) {
        *self = readmethod::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for readmethod {
    fn sub_assign(&mut self, rhs: u32) {
        *self = readmethod::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for readmethod {
    fn mul_assign(&mut self, rhs: u32) {
        *self = readmethod::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for readmethod {
    fn div_assign(&mut self, rhs: u32) {
        *self = readmethod::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for readmethod {
    fn rem_assign(&mut self, rhs: u32) {
        *self = readmethod::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for readmethod {
    type Output = readmethod;
    fn add(self, rhs: u32) -> readmethod {
        readmethod::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for readmethod {
    type Output = readmethod;
    fn sub(self, rhs: u32) -> readmethod {
        readmethod::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for readmethod {
    type Output = readmethod;
    fn mul(self, rhs: u32) -> readmethod {
        readmethod::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for readmethod {
    type Output = readmethod;
    fn div(self, rhs: u32) -> readmethod {
        readmethod::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for readmethod {
    type Output = readmethod;
    fn rem(self, rhs: u32) -> readmethod {
        readmethod::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_found {
    pub i: i32,
    pub sym: *const tinysym,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum markers {
    Author,
    Date,
    Header,
    Id,
    Locker,
    Log,
    Name,
    RCSfile,
    Revision,
    Source,
    State,
}
impl markers {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            markers::Author => 0,
            markers::Date => 1,
            markers::Header => 2,
            markers::Id => 3,
            markers::Locker => 4,
            markers::Log => 5,
            markers::Name => 6,
            markers::RCSfile => 7,
            markers::Revision => 8,
            markers::Source => 9,
            markers::State => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> markers {
        match value {
            0 => markers::Author,
            1 => markers::Date,
            2 => markers::Header,
            3 => markers::Id,
            4 => markers::Locker,
            5 => markers::Log,
            6 => markers::Name,
            7 => markers::RCSfile,
            8 => markers::Revision,
            9 => markers::Source,
            10 => markers::State,
            _ => panic!("Invalid value for markers: {}", value),
        }
    }
}
impl AddAssign<u32> for markers {
    fn add_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for markers {
    fn sub_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for markers {
    fn mul_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for markers {
    fn div_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for markers {
    fn rem_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for markers {
    type Output = markers;
    fn add(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for markers {
    type Output = markers;
    fn sub(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for markers {
    type Output = markers;
    fn mul(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for markers {
    type Output = markers;
    fn div(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for markers {
    type Output = markers;
    fn rem(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const i8,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const i8,
    pub name: *const i8,
    pub desc: *const i8,
    pub help: *const i8,
    pub tyag: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum maker {
    notmade,
    real,
    effective,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::notmade => 0,
            maker::real => 1,
            maker::effective => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            0 => maker::notmade,
            1 => maker::real,
            2 => maker::effective,
            _ => panic!("Invalid value for maker: {}", value),
        }
    }
}
impl AddAssign<u32> for maker {
    fn add_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for maker {
    fn sub_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for maker {
    fn mul_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for maker {
    fn div_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for maker {
    fn rem_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for maker {
    type Output = maker;
    fn add(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for maker {
    type Output = maker;
    fn sub(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for maker {
    type Output = maker;
    fn mul(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for maker {
    type Output = maker;
    fn div(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for maker {
    type Output = maker;
    fn rem(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const i8,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const i8,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: i32,
    pub stick_with_euid: bool,
    pub ruid: i32,
    pub euid: i32,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: i32,
    pub pe: *const i8,
    pub zone_offset: zone_offset,
    pub username: *mut i8,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut i8,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut i8,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub valid: bool,
    pub author: *mut i8,
    pub date: *mut i8,
    pub name: *mut i8,
    pub rev: *mut i8,
    pub state: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const i8,
    pub branch: *const i8,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: i32,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const i8,
    pub fd_lock: i32,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const i8,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_3 {
    _ISdigit = 2048,
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::_ISdigit => 2048,
            C2RustUnnamed_3::_ISalnum => 8,
            C2RustUnnamed_3::_ISpunct => 4,
            C2RustUnnamed_3::_IScntrl => 2,
            C2RustUnnamed_3::_ISblank => 1,
            C2RustUnnamed_3::_ISgraph => 32768,
            C2RustUnnamed_3::_ISprint => 16384,
            C2RustUnnamed_3::_ISspace => 8192,
            C2RustUnnamed_3::_ISxdigit => 4096,
            C2RustUnnamed_3::_ISalpha => 1024,
            C2RustUnnamed_3::_ISlower => 512,
            C2RustUnnamed_3::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            2048 => C2RustUnnamed_3::_ISdigit,
            8 => C2RustUnnamed_3::_ISalnum,
            4 => C2RustUnnamed_3::_ISpunct,
            2 => C2RustUnnamed_3::_IScntrl,
            1 => C2RustUnnamed_3::_ISblank,
            32768 => C2RustUnnamed_3::_ISgraph,
            16384 => C2RustUnnamed_3::_ISprint,
            8192 => C2RustUnnamed_3::_ISspace,
            4096 => C2RustUnnamed_3::_ISxdigit,
            1024 => C2RustUnnamed_3::_ISalpha,
            512 => C2RustUnnamed_3::_ISlower,
            256 => C2RustUnnamed_3::_ISupper,
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
unsafe extern "C" fn sorry(mut save: bool, mut msg: *const i8) -> *mut i8 {
    if save {
        let mut partial: *mut i8 = 0 as *mut i8;
        let mut len: size_t = 0;
        partial = finish_string(single, &mut len);
        brush_off(single, partial as *mut libc::c_void);
    }
    if !msg.is_null() {
        generic_error(
            (*top).manifestation.filename,
            b"%s\0" as *const u8 as *const i8,
            msg,
        );
    }
    return 0 as *mut i8;
}
unsafe extern "C" fn badly_terminated(mut save: bool) -> *mut i8 {
    return sorry(save, b"badly terminated keyword value\0" as *const u8 as *const i8);
}
unsafe extern "C" fn get0val(
    mut c: i32,
    mut fp: *mut fro,
    mut save: bool,
    mut optional: bool,
) -> *mut i8 {
    let mut val: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut got1: bool = false;
    got1 = 0 as i32 != 0;
    loop {
        's_102: {
            match c {
                32 | 9 => {
                    if save {
                        val = finish_string(single, &mut len);
                        if !got1 {
                            brush_off(single, val as *mut libc::c_void);
                            val = 0 as *mut i8;
                        }
                    }
                    if got1 as i32 != 0 && val.is_null() {
                        val = b"non-NULL\0" as *const u8 as *const i8 as *mut i8;
                    }
                    return val;
                }
                36 => {
                    if !got1 && optional as i32 != 0 {
                        if !val.is_null() {
                            brush_off(single, val as *mut libc::c_void);
                        }
                        return 0 as *mut i8;
                    }
                }
                10 | 0 => {}
                _ => {
                    got1 = 1 as i32 != 0;
                    if save {
                        accumulate_byte(single, c);
                    }
                    break 's_102;
                }
            }
            return badly_terminated(save);
        }
        if fro_try_getbyte(&mut c, fp) {
            return badly_terminated(save);
        }
    };
}
unsafe extern "C" fn keepid(mut c: i32, mut fp: *mut fro) -> *mut i8 {
    let mut maybe: *mut i8 = 0 as *mut i8;
    if c == 0 {
        if fro_try_getbyte(&mut c, fp) {
            return sorry(1 as i32 != 0, 0 as *const i8);
        }
    }
    maybe = get0val(c, fp, 1 as i32 != 0, 0 as i32 != 0);
    if maybe.is_null() {
        return 0 as *mut i8;
    }
    checksid(maybe);
    if (*top).flow.erroneous {
        brush_off(single, maybe as *mut libc::c_void);
        maybe = 0 as *mut i8;
    }
    return maybe;
}
unsafe extern "C" fn getval(
    mut fp: *mut fro,
    mut save: bool,
    mut optional: bool,
) -> *mut i8 {
    let mut c: i32 = 0;
    if fro_try_getbyte(&mut c, fp) {
        return badly_terminated(save);
    }
    return get0val(c, fp, save, optional);
}
unsafe extern "C" fn keepdate(mut fp: *mut fro) -> i32 {
    let mut d: *mut i8 = 0 as *mut i8;
    let mut t: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    c = 0 as i32;
    d = getval(fp, 1 as i32 != 0, 0 as i32 != 0);
    if !d.is_null() {
        t = getval(fp, 1 as i32 != 0, 0 as i32 != 0);
        if t.is_null() {
            brush_off(single, d as *mut libc::c_void);
        } else {
            if fro_try_getbyte(&mut c, fp) {
                c = 0 as i32;
            }
            if c == 0 {
                brush_off(single, t as *mut libc::c_void);
            } else {
                let mut buf: [i8; 64] = [0; 64];
                let mut len: size_t = 0;
                len = snprintf(
                    buf.as_mut_ptr(),
                    64 as i32 as u64,
                    b"%s%s %s%s\0" as *const u8 as *const i8,
                    if *(*__ctype_b_loc())
                        .offset(*d.offset(0 as i32 as isize) as i32 as isize) as i32
                        & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
                        && *(*__ctype_b_loc())
                            .offset(*d.offset(1 as i32 as isize) as i32 as isize) as i32
                            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32
                            != 0
                        && *(*__ctype_b_loc())
                            .offset(*d.offset(2 as i32 as isize) as i32 as isize) as i32
                            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32
                            == 0
                    {
                        b"19\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    d,
                    t,
                    if (strchr(t, '-' as i32)).is_null()
                        && (strchr(t, '+' as i32)).is_null()
                    {
                        b"+0000\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                ) as size_t;
                brush_off(single, t as *mut libc::c_void);
                brush_off(single, d as *mut libc::c_void);
                (*top).manifestation.prev.date = intern(single, buf.as_mut_ptr(), len);
            }
        }
    }
    return c;
}
unsafe extern "C" fn keeprev(mut fp: *mut fro) -> *const i8 {
    let mut current_block: u64;
    let mut s: *mut i8 = getval(fp, 1 as i32 != 0, 0 as i32 != 0);
    if !s.is_null() {
        let mut sp: *const i8 = 0 as *const i8;
        let mut dotcount: i32 = 0 as i32;
        sp = s;
        loop {
            match *sp as i32 {
                0 => {
                    if dotcount & 1 as i32 != 0 {
                        current_block = 1935794082185786268;
                        break;
                    } else {
                        current_block = 1917311967535052937;
                        break;
                    }
                }
                46 => {
                    dotcount += 1;
                    dotcount;
                }
                _ => {
                    if !(*(*__ctype_b_loc()).offset(*sp as i32 as isize) as i32
                        & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0)
                    {
                        current_block = 1917311967535052937;
                        break;
                    }
                }
            }
            sp = sp.offset(1);
            sp;
        }
        match current_block {
            1935794082185786268 => {}
            _ => {
                generic_error(
                    (*top).manifestation.filename,
                    b"%s is not a %s\0" as *const u8 as *const i8,
                    s,
                    ks_revno.as_ptr(),
                );
                brush_off(single, s as *mut libc::c_void);
                s = 0 as *mut i8;
            }
        }
    }
    (*top).manifestation.prev.rev = s;
    return (*top).manifestation.prev.rev;
}
#[no_mangle]
pub unsafe extern "C" fn getoldkeys(mut fp: *mut fro) -> bool {
    let mut current_block: u64;
    let mut c: i32 = 0;
    let mut keyword: [i8; 9] = [0; 9];
    let mut tp: *mut i8 = 0 as *mut i8;
    let mut needs_closing: bool = false;
    let mut match_0: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut mani_filename: *const i8 = (*top).manifestation.filename;
    if (*top).manifestation.prev.valid {
        return 1 as i32 != 0;
    }
    needs_closing = 0 as i32 != 0;
    if fp.is_null() {
        fp = fro_open(mani_filename, b"r\0" as *const u8 as *const i8, 0 as *mut stat);
        if fp.is_null() {
            syserror(*__errno_location(), mani_filename);
            return 0 as i32 != 0;
        }
        needs_closing = 1 as i32 != 0;
    }
    c = '\0' as i32;
    's_56: loop {
        if c == '$' as i32 {
            loop {
                tp = keyword.as_mut_ptr();
                loop {
                    if fro_try_getbyte(&mut c, fp) {
                        current_block = 6626219656103296975;
                        break 's_56;
                    }
                    match c {
                        10 | 36 | 58 => {
                            break;
                        }
                        _ => {}
                    }
                    if keyword.as_mut_ptr().offset(8 as i32 as isize) <= tp {
                        break;
                    }
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = c as i8;
                }
                if !(c == '$' as i32) {
                    break;
                }
            }
            if c != ':' as i32 {
                continue;
            }
            *tp = c as i8;
            if fro_try_getbyte(&mut c, fp) {
                current_block = 6626219656103296975;
                break;
            }
            match c {
                32 | 9 => {}
                _ => {
                    continue;
                }
            }
            recognize_keyword(keyword.as_mut_ptr(), &mut match_0);
            match match_0.i {
                0 => {
                    (*top).manifestation.prev.author = keepid('\0' as i32, fp);
                    if ((*top).manifestation.prev.author).is_null() {
                        current_block = 16377669147236323531;
                        break;
                    }
                    c = 0 as i32;
                }
                1 => {
                    c = keepdate(fp);
                    if c == 0 {
                        current_block = 16377669147236323531;
                        break;
                    }
                }
                2 | 3 => {
                    if !(!(getval(fp, 0 as i32 != 0, 0 as i32 != 0)).is_null()
                        && !(keeprev(fp)).is_null()
                        && {
                            c = keepdate(fp);
                            c != 0
                        }
                        && {
                            (*top).manifestation.prev.author = keepid(c, fp);
                            !((*top).manifestation.prev.author).is_null()
                        }
                        && {
                            (*top).manifestation.prev.state = keepid('\0' as i32, fp);
                            !((*top).manifestation.prev.state).is_null()
                        })
                    {
                        current_block = 16377669147236323531;
                        break;
                    }
                    if !(getval(fp, 0 as i32 != 0, 1 as i32 != 0)).is_null()
                        && !(getval(fp, 0 as i32 != 0, 1 as i32 != 0)).is_null()
                    {
                        c = 0 as i32;
                    } else {
                        if (*top).flow.erroneous {
                            current_block = 16377669147236323531;
                            break;
                        }
                        c = '$' as i32;
                    }
                }
                4 => {
                    getval(fp, 0 as i32 != 0, 0 as i32 != 0);
                    c = 0 as i32;
                }
                5 | 7 | 9 => {
                    if (getval(fp, 0 as i32 != 0, 0 as i32 != 0)).is_null() {
                        current_block = 16377669147236323531;
                        break;
                    }
                    c = 0 as i32;
                }
                6 => {
                    (*top).manifestation.prev.name = getval(
                        fp,
                        1 as i32 != 0,
                        0 as i32 != 0,
                    );
                    if !((*top).manifestation.prev.name).is_null()
                        && *(*top).manifestation.prev.name as i32 != 0
                    {
                        checkssym((*top).manifestation.prev.name);
                    }
                    c = 0 as i32;
                }
                8 => {
                    if (keeprev(fp)).is_null() {
                        current_block = 16377669147236323531;
                        break;
                    }
                    c = 0 as i32;
                }
                10 => {
                    (*top).manifestation.prev.state = keepid('\0' as i32, fp);
                    if ((*top).manifestation.prev.state).is_null() {
                        current_block = 16377669147236323531;
                        break;
                    }
                    c = 0 as i32;
                }
                _ => {
                    continue;
                }
            }
            if c == 0 {
                if fro_try_getbyte(&mut c, fp) {
                    c = 0 as i32;
                }
            }
            if c != '$' as i32 {
                generic_error(
                    (*top).manifestation.filename,
                    b"closing %c missing on keyword\0" as *const u8 as *const i8,
                    '$' as i32,
                );
                current_block = 16377669147236323531;
                break;
            } else if !((*top).manifestation.prev.name).is_null()
                && !((*top).manifestation.prev.author).is_null()
                && !((*top).manifestation.prev.date).is_null()
                && !((*top).manifestation.prev.rev).is_null()
                && !((*top).manifestation.prev.state).is_null()
            {
                current_block = 6626219656103296975;
                break;
            }
        }
        if fro_try_getbyte(&mut c, fp) {
            current_block = 6626219656103296975;
            break;
        }
    }
    match current_block {
        16377669147236323531 => return 0 as i32 != 0,
        _ => {
            if needs_closing {
                fro_close(fp);
            } else {
                fro_move(fp, 0 as i32 as off_t);
            }
            if !((*top).manifestation.prev.name).is_null()
                && *(*top).manifestation.prev.name == 0
            {
                (*top).manifestation.prev.name = 0 as *mut i8;
            }
            if !((*top).manifestation.prev.author).is_null()
                && *(*top).manifestation.prev.author == 0
            {
                (*top).manifestation.prev.author = 0 as *mut i8;
            }
            if !((*top).manifestation.prev.date).is_null()
                && *(*top).manifestation.prev.date == 0
            {
                (*top).manifestation.prev.date = 0 as *mut i8;
            }
            if !((*top).manifestation.prev.rev).is_null()
                && *(*top).manifestation.prev.rev == 0
            {
                (*top).manifestation.prev.rev = 0 as *mut i8;
            }
            if !((*top).manifestation.prev.state).is_null()
                && *(*top).manifestation.prev.state == 0
            {
                (*top).manifestation.prev.state = 0 as *mut i8;
            }
            (*top).manifestation.prev.valid = 1 as i32 != 0;
            return 1 as i32 != 0;
        }
    };
}