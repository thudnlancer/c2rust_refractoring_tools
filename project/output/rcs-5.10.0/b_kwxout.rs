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
    pub type wlink;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn awrite(buf: *const i8, chars: size_t, f: *mut FILE);
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut top: *mut top;
    static tiny_ciklog: tinysym;
    fn looking_at(sym: *const tinysym, start: *const i8) -> bool;
    fn recognize_keyword(string: *const i8, found: *mut pool_found) -> bool;
    fn basefilename(p: *const i8) -> *const i8;
    fn getfullRCSname() -> *const i8;
    static ctab: [tokens; 0];
    fn date2str(date: *const i8, datebuf: *mut i8) -> *const i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    static mut single: *mut divvy;
    fn make_space(name: *const i8) -> *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn forget(divvy: *mut divvy);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn testOerror(o: *mut FILE);
    fn afputc(c: i32, f: *mut FILE);
    fn newline(f: *mut FILE);
    fn aputs(s: *const i8, iop: *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const i8, _: ...);
    fn fro_tello(f: *mut fro) -> off_t;
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut i32, f: *mut fro) -> bool;
    fn fro_must_getbyte(c: *mut i32, f: *mut fro);
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum kwsub {
    kwsub_b = 5,
    kwsub_o = 4,
    kwsub_v = 3,
    kwsub_k = 2,
    kwsub_kvl = 1,
    kwsub_kv = 0,
}
impl kwsub {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            kwsub::kwsub_b => 5,
            kwsub::kwsub_o => 4,
            kwsub::kwsub_v => 3,
            kwsub::kwsub_k => 2,
            kwsub::kwsub_kvl => 1,
            kwsub::kwsub_kv => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> kwsub {
        match value {
            5 => kwsub::kwsub_b,
            4 => kwsub::kwsub_o,
            3 => kwsub::kwsub_v,
            2 => kwsub::kwsub_k,
            1 => kwsub::kwsub_kvl,
            0 => kwsub::kwsub_kv,
            _ => panic!("Invalid value for kwsub: {}", value),
        }
    }
}
impl AddAssign<u32> for kwsub {
    fn add_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for kwsub {
    fn sub_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for kwsub {
    fn mul_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for kwsub {
    fn div_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for kwsub {
    fn rem_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for kwsub {
    type Output = kwsub;
    fn add(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for kwsub {
    type Output = kwsub;
    fn sub(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for kwsub {
    type Output = kwsub;
    fn mul(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for kwsub {
    type Output = kwsub;
    fn div(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for kwsub {
    type Output = kwsub;
    fn rem(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum tokens {
    STRING = 14,
    SEMI = 13,
    NUM = 12,
    ID = 11,
    COLON = 10,
    UNKN = 9,
    SPACE = 8,
    SBEGIN = 7,
    PERIOD = 6,
    Letter = 5,
    LETTER = 4,
    NEWLN = 3,
    IDCHAR = 2,
    DIGIT = 1,
    DELIM = 0,
}
impl tokens {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            tokens::STRING => 14,
            tokens::SEMI => 13,
            tokens::NUM => 12,
            tokens::ID => 11,
            tokens::COLON => 10,
            tokens::UNKN => 9,
            tokens::SPACE => 8,
            tokens::SBEGIN => 7,
            tokens::PERIOD => 6,
            tokens::Letter => 5,
            tokens::LETTER => 4,
            tokens::NEWLN => 3,
            tokens::IDCHAR => 2,
            tokens::DIGIT => 1,
            tokens::DELIM => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> tokens {
        match value {
            14 => tokens::STRING,
            13 => tokens::SEMI,
            12 => tokens::NUM,
            11 => tokens::ID,
            10 => tokens::COLON,
            9 => tokens::UNKN,
            8 => tokens::SPACE,
            7 => tokens::SBEGIN,
            6 => tokens::PERIOD,
            5 => tokens::Letter,
            4 => tokens::LETTER,
            3 => tokens::NEWLN,
            2 => tokens::IDCHAR,
            1 => tokens::DIGIT,
            0 => tokens::DELIM,
            _ => panic!("Invalid value for tokens: {}", value),
        }
    }
}
impl AddAssign<u32> for tokens {
    fn add_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for tokens {
    fn sub_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for tokens {
    fn mul_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for tokens {
    fn div_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for tokens {
    fn rem_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for tokens {
    type Output = tokens;
    fn add(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for tokens {
    type Output = tokens;
    fn sub(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for tokens {
    type Output = tokens;
    fn mul(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for tokens {
    type Output = tokens;
    fn div(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for tokens {
    type Output = tokens;
    fn rem(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
    RM_STDIO = 2,
    RM_MEM = 1,
    RM_MMAP = 0,
}
impl readmethod {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            readmethod::RM_STDIO => 2,
            readmethod::RM_MEM => 1,
            readmethod::RM_MMAP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> readmethod {
        match value {
            2 => readmethod::RM_STDIO,
            1 => readmethod::RM_MEM,
            0 => readmethod::RM_MMAP,
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
    State = 10,
    Source = 9,
    Revision = 8,
    RCSfile = 7,
    Name = 6,
    Log = 5,
    Locker = 4,
    Id = 3,
    Header = 2,
    Date = 1,
    Author = 0,
}
impl markers {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            markers::State => 10,
            markers::Source => 9,
            markers::Revision => 8,
            markers::RCSfile => 7,
            markers::Name => 6,
            markers::Log => 5,
            markers::Locker => 4,
            markers::Id => 3,
            markers::Header => 2,
            markers::Date => 1,
            markers::Author => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> markers {
        match value {
            10 => markers::State,
            9 => markers::Source,
            8 => markers::Revision,
            7 => markers::RCSfile,
            6 => markers::Name,
            5 => markers::Log,
            4 => markers::Locker,
            3 => markers::Id,
            2 => markers::Header,
            1 => markers::Date,
            0 => markers::Author,
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
    effective = 2,
    real = 1,
    notmade = 0,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::effective => 2,
            maker::real => 1,
            maker::notmade => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            2 => maker::effective,
            1 => maker::real,
            0 => maker::notmade,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expctx {
    pub to: *mut FILE,
    pub rewr: *mut FILE,
    pub from: *mut fro,
    pub delta: *const delta,
    pub delimstuffed: bool,
    pub dolog: bool,
    pub lparts: *mut divvy,
}
unsafe extern "C" fn afilename(mut base: bool, mut out: *mut FILE) {
    let mut filename: *const i8 = if base as i32 != 0 {
        basefilename((*top).repository.filename)
    } else {
        getfullRCSname()
    };
    let mut c: i8 = 0;
    loop {
        let fresh0 = filename;
        filename = filename.offset(1);
        c = *fresh0;
        if !(c != 0) {
            break;
        }
        let mut current_block_7: u64;
        match c as i32 {
            9 => {
                aputs(b"\\t\0" as *const u8 as *const i8, out);
                current_block_7 = 10599921512955367680;
            }
            10 => {
                aputs(b"\\n\0" as *const u8 as *const i8, out);
                current_block_7 = 10599921512955367680;
            }
            32 => {
                aputs(b"\\040\0" as *const u8 as *const i8, out);
                current_block_7 = 10599921512955367680;
            }
            36 => {
                aputs(b"\\044\0" as *const u8 as *const i8, out);
                current_block_7 = 10599921512955367680;
            }
            92 => {
                if 5 as i32 - 5 as i32 <= (*top).behavior.version {
                    aputs(b"\\\\\0" as *const u8 as *const i8, out);
                    current_block_7 = 10599921512955367680;
                } else {
                    current_block_7 = 9528149638007858428;
                }
            }
            _ => {
                current_block_7 = 9528149638007858428;
            }
        }
        match current_block_7 {
            9528149638007858428 => {
                if _IO_putc(c as i32, out) == -(1 as i32) {
                    testOerror(out);
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn keyreplace(mut marker: *mut pool_found, mut ctx: *mut expctx) {
    let mut infile: *mut fro = (*ctx).from;
    let mut out: *mut FILE = (*ctx).to;
    let mut delta: *const delta = (*ctx).delta;
    let mut dolog: bool = (*ctx).dolog;
    let mut delimstuffed: bool = (*ctx).delimstuffed;
    let mut sp: *const i8 = 0 as *const i8;
    let mut cp: *const i8 = 0 as *const i8;
    let mut date: *const i8 = 0 as *const i8;
    let mut c: i32 = 0;
    let mut cs: size_t = 0;
    let mut cw: size_t = 0;
    let mut ls: size_t = 0;
    let mut sp1: *const i8 = 0 as *const i8;
    let mut datebuf: [i8; 31] = [0; 31];
    let mut RCSv: i32 = 0;
    let mut exp: i32 = 0;
    let mut include_locker: bool = (*top).behavior.inclusive_of_Locker_in_Id_val;
    exp = (*top).behavior.kws;
    date = (*delta).date;
    RCSv = (*top).behavior.version;
    if exp != kwsub::kwsub_v as i32 {
        aprintf(
            out,
            b"%c%s\0" as *const u8 as *const i8,
            '$' as i32,
            ((*(*marker).sym).bytes).as_ptr(),
        );
    }
    if exp != kwsub::kwsub_k as i32 {
        if exp != kwsub::kwsub_v as i32 {
            aprintf(
                out,
                b"%c%c\0" as *const u8 as *const i8,
                ':' as i32,
                if (*marker).i == markers::Log as i32 && RCSv < 5 as i32 - 5 as i32 {
                    '\t' as i32
                } else {
                    ' ' as i32
                },
            );
        }
        match (*marker).i {
            0 => {
                aputs((*delta).author, out);
            }
            1 => {
                aputs(date2str(date, datebuf.as_mut_ptr()), out);
            }
            3 | 2 => {
                afilename(
                    (*marker).i == markers::Id as i32 || RCSv < 4 as i32 - 5 as i32,
                    out,
                );
                aprintf(
                    out,
                    b" %s %s %s %s\0" as *const u8 as *const i8,
                    (*delta).num,
                    date2str(date, datebuf.as_mut_ptr()),
                    (*delta).author,
                    if RCSv == 3 as i32 - 5 as i32 && !((*delta).lockedby).is_null() {
                        b"Locked\0" as *const u8 as *const i8
                    } else {
                        (*delta).state
                    },
                );
                if !((*delta).lockedby).is_null() {
                    if 5 as i32 - 5 as i32 <= RCSv {
                        if include_locker as i32 != 0 || exp == kwsub::kwsub_kvl as i32 {
                            aprintf(
                                out,
                                b" %s\0" as *const u8 as *const i8,
                                (*delta).lockedby,
                            );
                        }
                    } else if RCSv == 4 as i32 - 5 as i32 {
                        aprintf(
                            out,
                            b" markers::Locker: %s\0" as *const u8 as *const i8,
                            (*delta).lockedby,
                        );
                    }
                }
            }
            4 => {
                if !((*delta).lockedby).is_null() {
                    if include_locker as i32 != 0 || exp == kwsub::kwsub_kvl as i32
                        || RCSv <= 4 as i32 - 5 as i32
                    {
                        aputs((*delta).lockedby, out);
                    }
                }
            }
            5 | 7 => {
                afilename(1 as i32 != 0, out);
            }
            6 => {
                if !((*delta).name).is_null() {
                    aputs((*delta).name, out);
                }
            }
            8 => {
                aputs((*delta).num, out);
            }
            9 => {
                afilename(0 as i32 != 0, out);
            }
            10 => {
                aputs((*delta).state, out);
            }
            _ => {}
        }
        if exp != kwsub::kwsub_v as i32 {
            afputc(' ' as i32, out);
        }
    }
    if exp != kwsub::kwsub_v as i32 {
        afputc('$' as i32, out);
    }
    if (*marker).i == markers::Log as i32 && dolog as i32 != 0 {
        let mut leader: *mut i8 = 0 as *mut i8;
        sp = (*delta).pretty_log.string;
        ls = (*delta).pretty_log.size;
        if looking_at(&tiny_ciklog, (*delta).pretty_log.string) {
            return;
        }
        if RCSv < 5 as i32 - 5 as i32 {
            cp = (*top).repository.log_lead.string;
            cs = (*top).repository.log_lead.size;
        } else {
            let mut current_block_74: u64;
            let mut kdelim_found: bool = 0 as i32 != 0;
            let mut chars_read: off_t = fro_tello(infile);
            c = 0 as i32;
            cs = 0 as i32 as size_t;
            loop {
                chars_read -= 1;
                if chars_read == 0 {
                    current_block_74 = 8869332144787829186;
                    break;
                }
                fro_move(infile, -(2 as i32) as off_t);
                fro_must_getbyte(&mut c, infile);
                if c == '\n' as i32 {
                    current_block_74 = 1134115459065347084;
                    break;
                }
                if c == '@' as i32 && delimstuffed as i32 != 0 {
                    chars_read -= 1;
                    if chars_read == 0 {
                        current_block_74 = 1134115459065347084;
                        break;
                    }
                    fro_move(infile, -(2 as i32) as off_t);
                    fro_must_getbyte(&mut c, infile);
                    if c != '@' as i32 {
                        fro_must_getbyte(&mut c, infile);
                        current_block_74 = 1134115459065347084;
                        break;
                    }
                }
                cs = (cs as u64).wrapping_add(kdelim_found as u64) as size_t as size_t;
                kdelim_found = (kdelim_found as i32 | (c == '$' as i32) as i32) as bool;
            }
            match current_block_74 {
                1134115459065347084 => {
                    fro_must_getbyte(&mut c, infile);
                }
                _ => {}
            }
            leader = alloc(single, (1 as i32 as u64).wrapping_add(cs)) as *mut i8;
            cp = leader;
            cw = 0 as i32 as size_t;
            while cw < cs {
                *leader.offset(cw as isize) = c as i8;
                if c == '@' as i32 && delimstuffed as i32 != 0 {
                    fro_must_getbyte(&mut c, infile);
                }
                fro_must_getbyte(&mut c, infile);
                cw = cw.wrapping_add(1);
                cw;
            }
            cw = 0 as i32 as size_t;
            while cw < cs {
                if *ctab.as_ptr().offset(*cp.offset(cw as isize) as u8 as isize) as u32
                    != tokens::SPACE as i32 as u32
                {
                    break;
                }
                cw = cw.wrapping_add(1);
                cw;
            }
            if cw.wrapping_add(1 as i32 as u64) < cs
                && *cp.offset(cw.wrapping_add(1 as i32 as u64) as isize) as i32
                    == '*' as i32
                && (*cp.offset(cw as isize) as i32 == '/' as i32
                    || *cp.offset(cw as isize) as i32 == '(' as i32)
            {
                let mut i: size_t = cw.wrapping_add(1 as i32 as u64);
                loop {
                    i = i.wrapping_add(1);
                    if i == cs {
                        generic_warn(
                            0 as *const i8,
                            b"`%c* $markers::Log' is obsolescent; use ` * $markers::Log'.\0"
                                as *const u8 as *const i8,
                            *cp.offset(cw as isize) as i32,
                        );
                        *leader.offset(cw as isize) = ' ' as i32 as i8;
                        break;
                    } else if *ctab
                        .as_ptr()
                        .offset(*cp.offset(i as isize) as u8 as isize) as u32
                        != tokens::SPACE as i32 as u32
                    {
                        break;
                    }
                }
            }
            loop {
                fro_must_getbyte(&mut c, infile);
                if !(c != '$' as i32) {
                    break;
                }
            }
        }
        newline(out);
        awrite(cp, cs, out);
        sp1 = date2str(date, datebuf.as_mut_ptr());
        if 5 as i32 - 5 as i32 <= RCSv {
            aprintf(
                out,
                b"markers::Revision %s  %s  %s\0" as *const u8 as *const i8,
                (*delta).num,
                sp1,
                (*delta).author,
            );
        } else {
            sp1 = strchr(sp1, ' ' as i32);
            aprintf(
                out,
                b"markers::Revision %s  %.*s %s  %s\0" as *const u8 as *const i8,
                (*delta).num,
                sp1.offset_from(datebuf.as_mut_ptr()) as i64 as i32,
                datebuf.as_mut_ptr(),
                sp1,
                (*delta).author,
            );
        }
        cw = cs;
        if 5 as i32 - 5 as i32 <= RCSv {
            while cw != 0
                && (*cp.offset(cw.wrapping_sub(1 as i32 as u64) as isize) as i32
                    == ' ' as i32
                    || *cp.offset(cw.wrapping_sub(1 as i32 as u64) as isize) as i32
                        == '\t' as i32)
            {
                cw = cw.wrapping_sub(1);
                cw;
            }
        }
        loop {
            newline(out);
            awrite(cp, cw, out);
            if ls == 0 {
                break;
            }
            ls = ls.wrapping_sub(1);
            ls;
            let fresh1 = sp;
            sp = sp.offset(1);
            c = *fresh1 as i32;
            if c != '\n' as i32 {
                awrite(cp.offset(cw as isize), cs.wrapping_sub(cw), out);
                loop {
                    afputc(c, out);
                    if ls == 0 {
                        break;
                    }
                    ls = ls.wrapping_sub(1);
                    ls;
                    let fresh2 = sp;
                    sp = sp.offset(1);
                    c = *fresh2 as i32;
                    if !(c != '\n' as i32) {
                        break;
                    }
                }
            }
        }
        if !leader.is_null() {
            brush_off(single, leader as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn expandline(mut ctx: *mut expctx) -> i32 {
    let mut current_block: u64;
    let mut lparts: *mut divvy = (*ctx).lparts;
    let mut fin: *mut fro = (*ctx).from;
    let mut delimstuffed: bool = (*ctx).delimstuffed;
    let mut c: i32 = 0;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut frew: *mut FILE = 0 as *mut FILE;
    let mut r: i32 = 0;
    let mut e: bool = false;
    let mut matchresult: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut cooked: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    if lparts.is_null() {
        (*ctx).lparts = make_space(b"lparts\0" as *const u8 as *const i8);
        lparts = (*ctx).lparts;
    }
    out = (*ctx).to;
    frew = (*ctx).rewr;
    forget(lparts);
    e = 0 as i32 != 0;
    r = -(1 as i32);
    's_49: loop {
        if delimstuffed {
            fro_must_getbyte(&mut c, fin);
            if !frew.is_null() {
                afputc(c, frew);
            }
        } else if fro_try_getbyte(&mut c, fin) {
            current_block = 7100068138679294007;
            break;
        }
        loop {
            match c {
                64 => {
                    if delimstuffed {
                        fro_must_getbyte(&mut c, fin);
                        if !frew.is_null() {
                            afputc(c, frew);
                        }
                        if c != '@' as i32 {
                            current_block = 7100068138679294007;
                            break 's_49;
                        }
                    }
                }
                10 => {
                    if _IO_putc(c, out) == -(1 as i32) {
                        testOerror(out);
                    }
                    r = 2 as i32;
                    current_block = 7100068138679294007;
                    break 's_49;
                }
                36 => {
                    r = 0 as i32;
                    accumulate_byte(lparts, '$' as i32);
                    len = 0 as i32 as size_t;
                    loop {
                        if delimstuffed {
                            fro_must_getbyte(&mut c, fin);
                            if !frew.is_null() {
                                afputc(c, frew);
                            }
                        } else if fro_try_getbyte(&mut c, fin) {
                            current_block = 5449909609936519693;
                            break 's_49;
                        }
                        if !(len <= (8 as i32 + 3 as i32) as u64) {
                            break;
                        }
                        match *ctab.as_ptr().offset(c as isize) as u32 {
                            4 | 5 => {}
                            _ => {
                                break;
                            }
                        }
                        accumulate_byte(lparts, c);
                        len = len.wrapping_add(1);
                        len;
                    }
                    accumulate_byte(lparts, c);
                    cooked = finish_string(lparts, &mut len);
                    if !recognize_keyword(
                        cooked.offset(1 as i32 as isize),
                        &mut matchresult,
                    ) {
                        *cooked.offset(len.wrapping_sub(1 as i32 as u64) as isize) = '\0'
                            as i32 as i8;
                        aputs(cooked, out);
                        continue;
                    } else {
                        if c == ':' as i32 {
                            loop {
                                if delimstuffed {
                                    fro_must_getbyte(&mut c, fin);
                                    if !frew.is_null() {
                                        afputc(c, frew);
                                    }
                                } else if fro_try_getbyte(&mut c, fin) {
                                    current_block = 5449909609936519693;
                                    break 's_49;
                                }
                                if c == '\n' as i32 || c == '$' as i32 {
                                    break;
                                }
                                accumulate_byte(lparts, c);
                                if !(c == '@' as i32 && delimstuffed as i32 != 0) {
                                    continue;
                                }
                                fro_must_getbyte(&mut c, fin);
                                if !frew.is_null() {
                                    afputc(c, frew);
                                }
                                if c != '@' as i32 {
                                    current_block = 5449909609936519693;
                                    break 's_49;
                                }
                            }
                            if c != '$' as i32 {
                                cooked = finish_string(lparts, &mut len);
                                aputs(cooked, out);
                                continue;
                            } else {
                                cooked = finish_string(lparts, &mut len);
                            }
                        }
                        keyreplace(&mut matchresult, ctx);
                        e = 1 as i32 != 0;
                        break;
                    }
                }
                _ => {}
            }
            if _IO_putc(c, out) == -(1 as i32) {
                testOerror(out);
            }
            r = 0 as i32;
            break;
        }
    }
    match current_block {
        5449909609936519693 => {
            cooked = finish_string(lparts, &mut len);
            aputs(cooked, out);
        }
        _ => {}
    }
    return r + e as i32;
}