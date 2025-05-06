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
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    static tiny_access: tinysym;
    static tiny_author: tinysym;
    static tiny_branch: tinysym;
    static tiny_branches: tinysym;
    static tiny_comment: tinysym;
    static tiny_commitid: tinysym;
    static tiny_date: tinysym;
    fn countnumflds(s: *const i8) -> i32;
    fn checkssym(sym: *const i8);
    static ctab: [tokens; 0];
    static tiny_desc: tinysym;
    static tiny_expand: tinysym;
    static tiny_head: tinysym;
    static tiny_integrity: tinysym;
    static tiny_locks: tinysym;
    static tiny_log: tinysym;
    static tiny_next: tinysym;
    static tiny_state: tinysym;
    static tiny_strict: tinysym;
    static tiny_symbols: tinysym;
    static tiny_text: tinysym;
    fn recognize_kwsub(x: *const cbuf) -> i32;
    fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const i8,
        __args: ::core::ffi::VaList,
    ) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn hash_pjw(x: *const libc::c_void, tablesize: size_t) -> size_t;
    fn complain(fmt: *const i8, _: ...);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_syntax(lno: size_t, fmt: *const i8, _: ...);
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    static mut single: *mut divvy;
    fn make_space(name: *const i8) -> *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn accs(divvy: *mut divvy, string: *const i8);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn close_space(divvy: *mut divvy);
    fn extend(tp: *mut link, x: *const libc::c_void, to: *mut divvy) -> *mut link;
    fn wextend(tp: *mut wlink, x: *mut libc::c_void, to: *mut divvy) -> *mut wlink;
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
    fn wprepend(x: *mut libc::c_void, ls: *mut wlink, to: *mut divvy) -> *mut wlink;
    fn fro_tello(f: *mut fro) -> off_t;
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_try_getbyte(c: *mut i32, f: *mut fro) -> bool;
    fn string_from_atat(space: *mut divvy, atat: *const atat) -> cbuf;
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
pub type va_list = __builtin_va_list;
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
pub type ptrdiff_t = i64;
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
pub struct wlink {
    pub entry: *mut libc::c_void,
    pub next: *mut wlink,
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
pub struct rcslock {
    pub login: *const i8,
    pub delta: *mut delta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdef {
    pub meaningful: *const i8,
    pub underlying: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 0],
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
pub struct hash {
    pub sz: size_t,
    pub a: *mut *mut wlink,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lockdef {
    pub login: *const i8,
    pub revno: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link {
    pub entry: *const libc::c_void,
    pub next: *mut link,
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
impl C2RustUnnamed_3 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_3::_ISalnum => 8,
            C2RustUnnamed_3::_ISpunct => 4,
            C2RustUnnamed_3::_IScntrl => 2,
            C2RustUnnamed_3::_ISblank => 1,
            C2RustUnnamed_3::_ISgraph => 32768,
            C2RustUnnamed_3::_ISprint => 16384,
            C2RustUnnamed_3::_ISspace => 8192,
            C2RustUnnamed_3::_ISxdigit => 4096,
            C2RustUnnamed_3::_ISdigit => 2048,
            C2RustUnnamed_3::_ISalpha => 1024,
            C2RustUnnamed_3::_ISlower => 512,
            C2RustUnnamed_3::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_3 {
        match value {
            8 => C2RustUnnamed_3::_ISalnum,
            4 => C2RustUnnamed_3::_ISpunct,
            2 => C2RustUnnamed_3::_IScntrl,
            1 => C2RustUnnamed_3::_ISblank,
            32768 => C2RustUnnamed_3::_ISgraph,
            16384 => C2RustUnnamed_3::_ISprint,
            8192 => C2RustUnnamed_3::_ISspace,
            4096 => C2RustUnnamed_3::_ISxdigit,
            2048 => C2RustUnnamed_3::_ISdigit,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grok {
    pub c: i32,
    pub from: *mut fro,
    pub to: *mut divvy,
    pub systolic: *mut divvy,
    pub tranquil: *mut divvy,
    pub xrep: cbuf,
    pub lno: size_t,
    pub head_lno: size_t,
    pub bor_no: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct notyet {
    pub revno: *const i8,
    pub next: *const i8,
    pub branches: *mut link,
    pub d: *mut delta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fwref {
    pub revno: *const i8,
    pub lno: size_t,
}
unsafe extern "C" fn ignoble(mut g: *mut grok, mut fmt: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut msg: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut scratch: *mut divvy = (*g).systolic;
    let mut o: *mut obstack = &mut (*scratch).space;
    let mut __o: *mut obstack = o;
    let mut __obj: *mut libc::c_void = ({
        let mut __o1: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 as u64)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    });
    if __obj > (*__o).chunk as *mut libc::c_void
        && __obj < (*__o).chunk_limit as *mut libc::c_void
    {
        (*__o).object_base = __obj as *mut i8;
        (*__o).next_free = (*__o).object_base;
    } else {
        _obstack_free(__o, __obj);
    }
    args_0 = args.clone();
    obstack_vprintf(o, fmt, args_0.as_va_list());
    msg.string = finish_string(scratch, &mut msg.size);
    complain(b"\n\0" as *const u8 as *const i8);
    fatal_syntax((*g).lno, b"%s\0" as *const u8 as *const i8, msg.string);
}
unsafe extern "C" fn eof_too_soon(mut g: *mut grok) {
    ignoble(g, b"unexpected end of file\0" as *const u8 as *const i8);
}
unsafe extern "C" fn skip_whitespace(mut g: *mut grok) {
    loop {
        if '\n' as i32 == (*g).c {
            (*g).lno = ((*g).lno).wrapping_add(1);
            (*g).lno;
        }
        if *(*__ctype_b_loc()).offset((*g).c as isize) as i32
            & C2RustUnnamed_3::_ISspace as i32 as libc::c_ushort as i32 == 0
        {
            return;
        }
        if fro_try_getbyte(&mut (*g).c, (*g).from) {
            eof_too_soon(g);
        }
    };
}
unsafe extern "C" fn must_read_keyword(mut g: *mut grok, mut kw: *const tinysym) {
    skip_whitespace(g);
    let mut i: size_t = 0 as i32 as size_t;
    while i < (*kw).len as u64 {
        if *(((*kw).bytes).as_ptr() as *const i8).offset(i as isize) as i32 != (*g).c {
            ignoble(
                g,
                b"missing `%s' keyword\0" as *const u8 as *const i8,
                ((*kw).bytes).as_ptr() as *const i8,
            );
        }
        if fro_try_getbyte(&mut (*g).c, (*g).from) {
            eof_too_soon(g);
        }
        i = i.wrapping_add(1);
        i;
    }
    (*g).xrep.string = ((*kw).bytes).as_ptr() as *const i8;
    (*g).xrep.size = (*kw).len as size_t;
}
unsafe extern "C" fn probe_keyword(mut g: *mut grok, mut kw: *const tinysym) -> bool {
    let mut was: off_t = 0;
    let mut rv: bool = 1 as i32 != 0;
    skip_whitespace(g);
    was = fro_tello((*g).from);
    let mut i: size_t = 0 as i32 as size_t;
    while i < (*kw).len as u64 {
        rv = *(((*kw).bytes).as_ptr() as *const i8).offset(i as isize) as i32 == (*g).c;
        if !rv {
            break;
        }
        if fro_try_getbyte(&mut (*g).c, (*g).from) {
            eof_too_soon(g);
        }
        i = i.wrapping_add(1);
        i;
    }
    if rv {
        (*g).xrep.string = ((*kw).bytes).as_ptr() as *const i8;
        (*g).xrep.size = (*kw).len as size_t;
    } else {
        fro_move((*g).from, was - 1 as i32 as i64);
        if fro_try_getbyte(&mut (*g).c, (*g).from) {
            eof_too_soon(g);
        }
    }
    return rv;
}
unsafe extern "C" fn accb(mut g: *mut grok) {
    accumulate_byte((*g).to, (*g).c);
    if fro_try_getbyte(&mut (*g).c, (*g).from) {
        eof_too_soon(g);
    }
}
unsafe extern "C" fn maybe_read_num(mut g: *mut grok, mut must_be_delta: bool) -> bool {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut dots: size_t = 0 as i32 as size_t;
    skip_whitespace(g);
    while '.' as i32 == (*g).c
        || *(*__ctype_b_loc()).offset((*g).c as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        if must_be_delta {
            dots = (dots as u64).wrapping_add(('.' as i32 == (*g).c) as i32 as u64)
                as size_t as size_t;
        }
        accb(g);
    }
    p = finish_string((*g).to, &mut (*g).xrep.size);
    if (*g).xrep.size != 0 {
        let mut trailing_garbage: bool = ';' as i32 != (*g).c
            && *(*__ctype_b_loc()).offset((*g).c as isize) as i32
                & C2RustUnnamed_3::_ISspace as i32 as libc::c_ushort as i32 == 0;
        if trailing_garbage {
            accs((*g).to, p);
            while ';' as i32 != (*g).c
                && *(*__ctype_b_loc()).offset((*g).c as isize) as i32
                    & C2RustUnnamed_3::_ISspace as i32 as libc::c_ushort as i32 == 0
            {
                accb(g);
            }
            p = finish_string((*g).to, &mut (*g).xrep.size);
        }
        if trailing_garbage as i32 != 0
            || must_be_delta as i32 != 0 && dots & 1 as i32 as u64 == 0
        {
            ignoble(
                g,
                b"invalid %s: %s\0" as *const u8 as *const i8,
                ks_revno.as_ptr(),
                p,
            );
        }
        (*g).xrep.string = p;
        return 1 as i32 != 0;
    }
    brush_off((*g).to, p as *mut libc::c_void);
    (*g).xrep.string = b"\0" as *const u8 as *const i8;
    return 0 as i32 != 0;
}
unsafe extern "C" fn must_read_num(mut g: *mut grok, mut role: *const i8) {
    if !maybe_read_num(g, ks_revno.as_ptr() == role) {
        ignoble(g, b"missing %s\0" as *const u8 as *const i8, role);
    }
}
unsafe extern "C" fn maybe_read_snippet(mut g: *mut grok) -> bool {
    let mut p: *mut i8 = 0 as *mut i8;
    skip_whitespace(g);
    while ';' as i32 != (*g).c && ':' as i32 != (*g).c
        && *(*__ctype_b_loc()).offset((*g).c as isize) as i32
            & C2RustUnnamed_3::_ISspace as i32 as libc::c_ushort as i32 == 0
        && tokens::UNKN as i32 as u32 != *ctab.as_ptr().offset((*g).c as isize) as u32
    {
        accb(g);
    }
    p = finish_string((*g).to, &mut (*g).xrep.size);
    if (*g).xrep.size != 0 {
        (*g).xrep.string = p;
        return 1 as i32 != 0;
    }
    brush_off((*g).to, p as *mut libc::c_void);
    (*g).xrep.string = b"\0" as *const u8 as *const i8;
    return 0 as i32 != 0;
}
unsafe extern "C" fn must_read_snippet(mut g: *mut grok, mut role: *const i8) {
    if !maybe_read_snippet(g) {
        ignoble(g, b"missing %s\0" as *const u8 as *const i8, role);
    }
}
unsafe extern "C" fn start_atat(mut to: *mut divvy, mut blank: bool) {
    let mut o: *mut obstack = &mut (*to).space;
    if blank {
        let mut __o: *mut obstack = o;
        let mut __len: size_t = ::core::mem::size_of::<atat>() as u64;
        if ({
            let mut __o1: *const obstack = __o;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    }
}
unsafe extern "C" fn finish_atat(mut to: *mut divvy) -> *mut atat {
    let mut rv: *mut atat = 0 as *mut atat;
    let mut o: *mut obstack = &mut (*to).space;
    let mut hsize: size_t = ({
        let mut __o: *const obstack = o;
        ((*__o).next_free).offset_from((*__o).object_base) as i64 as size_t
    })
        .wrapping_sub(::core::mem::size_of::<atat>() as u64);
    rv = ({
        let mut __o1: *mut obstack = o;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
        if (*__o1).next_free == __value as *mut i8 {
            (*__o1).set_maybe_empty_object(1 as i32 as u32);
        }
        (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
            < ::core::mem::size_of::<*mut libc::c_void>() as u64
        {
            (*__o1).object_base
        } else {
            0 as *mut i8
        })
            .offset(
                ((((*__o1).next_free)
                    .offset_from(
                        (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                            < ::core::mem::size_of::<*mut libc::c_void>() as u64
                        {
                            (*__o1).object_base
                        } else {
                            0 as *mut i8
                        }),
                    ) as i64 as u64)
                    .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                    as isize,
            );
        if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64 as size_t
            > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8) as i64
                as size_t
        {
            (*__o1).next_free = (*__o1).chunk_limit;
        }
        (*__o1).object_base = (*__o1).next_free;
        __value
    }) as *mut atat;
    (*rv).count = hsize.wrapping_div(::core::mem::size_of::<off_t>() as u64);
    return rv;
}
unsafe extern "C" fn maybe_read_atat(mut g: *mut grok, mut res: *mut *mut atat) -> bool {
    let mut atat: *mut atat = 0 as *mut atat;
    let mut beg: off_t = 0;
    let mut lno_start: size_t = 0;
    let mut newline: bool = 0 as i32 != 0;
    skip_whitespace(g);
    if !('@' as i32 != (*g).c) {
        lno_start = (*g).lno;
        beg = fro_tello((*g).from) + -(1 as i32) as i64;
        start_atat((*g).systolic, 1 as i32 != 0);
        while '@' as i32 == (*g).c {
            let mut hole: off_t = 0;
            let mut needexp: bool = 0 as i32 != 0;
            if fro_try_getbyte(&mut (*g).c, (*g).from) {
                eof_too_soon(g);
            }
            while '@' as i32 != (*g).c {
                if 0 as i32 != 0 && '$' as i32 == (*g).c {
                    needexp = 1 as i32 != 0;
                } else {
                    newline = '\n' as i32 == (*g).c;
                    if newline {
                        (*g).lno = ((*g).lno).wrapping_add(1);
                        (*g).lno;
                    }
                }
                if fro_try_getbyte(&mut (*g).c, (*g).from) {
                    eof_too_soon(g);
                }
            }
            if fro_try_getbyte(&mut (*g).c, (*g).from) {
                eof_too_soon(g);
            }
            hole = ((if needexp as i32 != 0 {
                (1 as libc::c_ulonglong)
                    << (::core::mem::size_of::<off_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(1 as i32 as u64)
            } else {
                0 as i32 as libc::c_ulonglong
            })
                | (fro_tello((*g).from)
                    + (if '@' as i32 == (*g).c { -(1 as i32) } else { -(2 as i32) })
                        as i64) as libc::c_ulonglong) as off_t;
            let mut __o: *mut obstack = &mut (*(*g).systolic).space;
            let mut __len: size_t = ::core::mem::size_of::<off_t>() as u64;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            memcpy(
                (*__o).next_free as *mut libc::c_void,
                &mut hole as *mut off_t as *const libc::c_void,
                __len,
            );
            (*__o).next_free = ((*__o).next_free).offset(__len as isize);
        }
        atat = finish_atat((*g).systolic);
        if !atat.is_null() {
            let mut count: size_t = (*atat).count;
            (*atat).lno = lno_start;
            (*atat).line_count = ((*g).lno)
                .wrapping_sub((*atat).lno)
                .wrapping_add(!newline as i32 as u64);
            (*atat).beg = beg;
            (*atat).from = (*g).from;
            start_atat((*g).to, 0 as i32 != 0);
            *res = ({
                let mut __h: *mut obstack = &mut (*(*g).to).space as *mut obstack;
                let mut __o_0: *mut obstack = __h;
                let mut __len_0: size_t = (::core::mem::size_of::<atat>() as u64)
                    .wrapping_add(
                        count.wrapping_mul(::core::mem::size_of::<off_t>() as u64),
                    );
                if ({
                    let mut __o1: *const obstack = __o_0;
                    ((*__o1).chunk_limit).offset_from((*__o1).next_free) as i64 as size_t
                }) < __len_0
                {
                    _obstack_newchunk(__o_0, __len_0);
                }
                memcpy(
                    (*__o_0).next_free as *mut libc::c_void,
                    atat as *const libc::c_void,
                    __len_0,
                );
                (*__o_0).next_free = ((*__o_0).next_free).offset(__len_0 as isize);
                ({
                    let mut __o1: *mut obstack = __h;
                    let mut __value: *mut libc::c_void = (*__o1).object_base
                        as *mut libc::c_void;
                    if (*__o1).next_free == __value as *mut i8 {
                        (*__o1).set_maybe_empty_object(1 as i32 as u32);
                    }
                    (*__o1).next_free = (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                        < ::core::mem::size_of::<*mut libc::c_void>() as u64
                    {
                        (*__o1).object_base
                    } else {
                        0 as *mut i8
                    })
                        .offset(
                            ((((*__o1).next_free)
                                .offset_from(
                                    (if (::core::mem::size_of::<ptrdiff_t>() as u64)
                                        < ::core::mem::size_of::<*mut libc::c_void>() as u64
                                    {
                                        (*__o1).object_base
                                    } else {
                                        0 as *mut i8
                                    }),
                                ) as i64 as u64)
                                .wrapping_add((*__o1).alignment_mask)
                                & !(*__o1).alignment_mask) as isize,
                        );
                    if ((*__o1).next_free).offset_from((*__o1).chunk as *mut i8) as i64
                        as size_t
                        > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut i8)
                            as i64 as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                })
            }) as *mut atat;
        }
    }
    return if !atat.is_null() { 1 as i32 } else { 0 as i32 } != 0;
}
unsafe extern "C" fn must_read_atat(
    mut g: *mut grok,
    mut res: *mut *mut atat,
    mut role: *const i8,
) {
    if !maybe_read_atat(g, res) {
        ignoble(g, b"missing string after %s\0" as *const u8 as *const i8, role);
    }
}
unsafe extern "C" fn must_colon_revno(mut g: *mut grok, mut role: *const i8) {
    if ':' as i32 != (*g).c {
        ignoble(g, b"missing ':' in %s\0" as *const u8 as *const i8, role);
    }
    if fro_try_getbyte(&mut (*g).c, (*g).from) {
        eof_too_soon(g);
    }
    must_read_num(g, (*g).bor_no.string);
}
unsafe extern "C" fn must_semi(mut g: *mut grok, mut clause: *const i8) {
    skip_whitespace(g);
    if ';' as i32 != (*g).c {
        ignoble(g, b"missing semicolon after `%s'\0" as *const u8 as *const i8, clause);
    }
    if fro_try_getbyte(&mut (*g).c, (*g).from) {
        eof_too_soon(g);
    }
}
unsafe extern "C" fn make_hash_table(mut to: *mut divvy, mut sz: size_t) -> *mut hash {
    let mut ht: *mut hash = alloc(to, ::core::mem::size_of::<hash>() as u64)
        as *mut hash;
    (*ht).sz = sz;
    (*ht).a = zlloc(to, sz.wrapping_mul(::core::mem::size_of::<*mut wlink>() as u64))
        as *mut *mut wlink;
    return ht;
}
unsafe extern "C" fn hash(mut key: *const i8, mut ht: *mut hash) -> size_t {
    return hash_pjw(key as *const libc::c_void, (*ht).sz);
}
unsafe extern "C" fn puthash(
    mut to: *mut divvy,
    mut ny: *mut notyet,
    mut ht: *mut hash,
) {
    let mut slot: size_t = hash((*ny).revno, ht);
    let mut box_0: wlink = wlink {
        entry: 0 as *mut libc::c_void,
        next: 0 as *mut wlink,
    };
    let mut tp: *mut wlink = 0 as *mut wlink;
    let mut cur: *mut wlink = 0 as *mut wlink;
    box_0.next = *((*ht).a).offset(slot as isize);
    tp = &mut box_0;
    loop {
        cur = (*tp).next;
        if cur.is_null() {
            break;
        }
        let mut maybe: *mut notyet = (*cur).entry as *mut notyet;
        if strcmp((*ny).revno, (*maybe).revno) == 0 {
            (*tp).entry = ny as *mut libc::c_void;
            return;
        }
        tp = (*tp).next;
    }
    tp = wextend(tp, ny as *mut libc::c_void, to);
    let ref mut fresh0 = *((*ht).a).offset(slot as isize);
    *fresh0 = box_0.next;
}
unsafe extern "C" fn gethash(
    mut revno: *const i8,
    mut ht: *mut hash,
) -> *mut libc::c_void {
    let mut slot: size_t = hash(revno, ht);
    let mut ls: *mut wlink = *((*ht).a).offset(slot as isize);
    while !ls.is_null() {
        let mut ny: *mut notyet = (*ls).entry as *mut notyet;
        if strcmp(revno, (*ny).revno) == 0 {
            return ny as *mut libc::c_void;
        }
        ls = (*ls).next;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn empty_repo(mut to: *mut divvy) -> *mut repo {
    let mut repo: *mut repo = zlloc(to, ::core::mem::size_of::<repo>() as u64)
        as *mut repo;
    (*repo).strict = 1 as i32 != 0;
    (*repo).expand = -(1 as i32);
    (*repo).neck = -(1 as i32) as off_t;
    return repo;
}
static mut ks_ner: [i8; 22] = unsafe {
    *::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"non-existent revision\0")
};
unsafe extern "C" fn full(mut to: *mut divvy, mut f: *mut fro) -> *mut repo {
    let mut current_block: u64;
    let mut neck: off_t = 0;
    let mut count: size_t = 0;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut follow: *mut wlink = 0 as *mut wlink;
    let mut all_br: *mut wlink = 0 as *mut wlink;
    let mut g: *mut grok = zlloc(single, ::core::mem::size_of::<grok>() as u64)
        as *mut grok;
    let mut repo: *mut repo = empty_repo(to);
    (*repo).ht = make_hash_table(to, 149 as i32 as size_t);
    (*g).from = f;
    (*g).to = to;
    (*g).systolic = make_space(b"systolic\0" as *const u8 as *const i8);
    (*g).tranquil = make_space(b"tranquil\0" as *const u8 as *const i8);
    (*g).lno = 1 as i32 as size_t;
    accf((*g).tranquil, b"branch or %s\0" as *const u8 as *const i8, ks_revno.as_ptr());
    (*g).bor_no.string = finish_string((*g).tranquil, &mut (*g).bor_no.size);
    if fro_try_getbyte(&mut (*g).c, (*g).from) {
        eof_too_soon(g);
    }
    must_read_keyword(g, &tiny_head);
    if maybe_read_num(g, 1 as i32 != 0) {
        (*g).head_lno = (*g).lno;
        (*repo).head = (*g).xrep.string;
    }
    must_semi(g, (tiny_head.bytes).as_ptr() as *const i8);
    if probe_keyword(g, &tiny_branch) {
        if maybe_read_num(g, 0 as i32 != 0) {
            (*repo).branch = (*g).xrep.string;
        }
        must_semi(g, (tiny_branch.bytes).as_ptr() as *const i8);
    }
    must_read_keyword(g, &tiny_access);
    count = 0 as i32 as size_t;
    box_0.next = (*repo).access;
    tp = &mut box_0;
    while maybe_read_snippet(g) {
        tp = extend(tp, (*g).xrep.string as *const libc::c_void, to);
        count = count.wrapping_add(1);
        count;
    }
    (*repo).access = box_0.next;
    (*repo).access_count = count;
    must_semi(g, (tiny_access.bytes).as_ptr() as *const i8);
    must_read_keyword(g, &tiny_symbols);
    count = 0 as i32 as size_t;
    box_0.next = (*repo).symbols;
    tp = &mut box_0;
    while maybe_read_snippet(g) {
        let mut sym: *mut symdef = alloc(to, ::core::mem::size_of::<symdef>() as u64)
            as *mut symdef;
        (*sym).meaningful = (*g).xrep.string;
        must_colon_revno(g, b"symbolic name definition\0" as *const u8 as *const i8);
        (*sym).underlying = (*g).xrep.string;
        tp = extend(tp, sym as *const libc::c_void, to);
        count = count.wrapping_add(1);
        count;
    }
    (*repo).symbols = box_0.next;
    (*repo).symbols_count = count;
    must_semi(g, (tiny_symbols.bytes).as_ptr() as *const i8);
    must_read_keyword(g, &tiny_locks);
    count = 0 as i32 as size_t;
    box_0.next = (*repo).locks;
    tp = &mut box_0;
    while maybe_read_snippet(g) {
        let mut lock: *mut lockdef = alloc(to, ::core::mem::size_of::<lockdef>() as u64)
            as *mut lockdef;
        (*lock).login = (*g).xrep.string;
        must_colon_revno(g, b"locker definition\0" as *const u8 as *const i8);
        (*lock).revno = (*g).xrep.string;
        tp = extend(tp, lock as *const libc::c_void, to);
        count = count.wrapping_add(1);
        count;
    }
    (*repo).locks = box_0.next;
    (*repo).locks_count = count;
    must_semi(g, (tiny_locks.bytes).as_ptr() as *const i8);
    (*repo).lockdefs = alloc(
        to,
        count.wrapping_mul(::core::mem::size_of::<lockdef>() as u64),
    ) as *mut lockdef;
    tp = (*repo).locks;
    loop {
        let fresh1 = count;
        count = count.wrapping_sub(1);
        if !(fresh1 != 0) {
            break;
        }
        let mut orig: *const lockdef = (*tp).entry as *const lockdef;
        *((*repo).lockdefs).offset(count as isize) = *orig;
        tp = (*tp).next;
    }
    (*repo).strict = probe_keyword(g, &tiny_strict);
    if (*repo).strict {
        must_semi(g, (tiny_strict.bytes).as_ptr() as *const i8);
    }
    if probe_keyword(g, &tiny_integrity) {
        if maybe_read_atat(g, &mut (*repo).integrity) as i32 != 0
            && (1 as i32 as u64) < (*(*repo).integrity).count
        {
            ignoble(
                g,
                b"spurious '@' in `%s' value\0" as *const u8 as *const i8,
                (tiny_integrity.bytes).as_ptr() as *const i8,
            );
        }
        must_semi(g, (tiny_integrity.bytes).as_ptr() as *const i8);
    }
    if probe_keyword(g, &tiny_comment) {
        maybe_read_atat(g, &mut (*repo).comment);
        must_semi(g, (tiny_comment.bytes).as_ptr() as *const i8);
    }
    (*repo).expand = -(1 as i32);
    if probe_keyword(g, &tiny_expand) {
        let mut expand: *mut atat = 0 as *mut atat;
        if maybe_read_atat(g, &mut expand) {
            let mut cb: cbuf = string_from_atat((*g).systolic, expand);
            (*repo).expand = recognize_kwsub(&mut cb);
            if 0 as i32 > (*repo).expand {
                ignoble(
                    g,
                    b"invalid expand mode: %s\0" as *const u8 as *const i8,
                    cb.string,
                );
            }
        }
        must_semi(g, (tiny_expand.bytes).as_ptr() as *const i8);
    }
    let mut wbox: wlink = wlink {
        entry: 0 as *mut libc::c_void,
        next: 0 as *mut wlink,
    };
    let mut wtp: *mut wlink = 0 as *mut wlink;
    let mut prev: *mut notyet = 0 as *mut notyet;
    let mut fw: *mut fwref = 0 as *mut fwref;
    count = 0 as i32 as size_t;
    wbox.next = (*repo).deltas;
    wtp = &mut wbox;
    while maybe_read_num(g, 1 as i32 != 0) {
        let mut ny: *mut notyet = alloc(to, ::core::mem::size_of::<notyet>() as u64)
            as *mut notyet;
        (*ny).d = alloc(to, ::core::mem::size_of::<delta>() as u64) as *mut delta;
        let mut d: *mut delta = (*ny).d;
        let mut numlen: size_t = (*g).xrep.size;
        (*d).num = (*g).xrep.string;
        if !prev.is_null() && ((*prev).next).is_null()
            && 2 as i32 <= countnumflds((*d).num)
        {
            let mut ls: *mut wlink = 0 as *mut wlink;
            ls = all_br;
            while !ls.is_null() {
                fw = (*ls).entry as *mut fwref;
                if (*fw).lno != 0 && strcmp((*d).num, (*fw).revno) == 0 {
                    (*fw).lno = 0 as i32 as size_t;
                    break;
                } else {
                    ls = (*ls).next;
                }
            }
            if ls.is_null() {
                ignoble(
                    g,
                    b"unexpected new branch %s: %s\0" as *const u8 as *const i8,
                    ks_revno.as_ptr(),
                    (*d).num,
                );
            }
        }
        (*d).branches = 0 as *mut wlink;
        (*d).ilk = 0 as *mut delta;
        (*d).lockedby = 0 as *const i8;
        (*d).pretty_log.string = 0 as *const i8;
        (*d).pretty_log.size = 0 as i32 as size_t;
        (*d).selector = 1 as i32 != 0;
        (*d).log = 0 as *mut atat;
        (*ny).revno = (*g).xrep.string;
        must_read_keyword(g, &tiny_date);
        must_read_num(g, b"date\0" as *const u8 as *const i8);
        (*d).date = (*g).xrep.string;
        must_semi(g, (tiny_date.bytes).as_ptr() as *const i8);
        must_read_keyword(g, &tiny_author);
        must_read_snippet(g, (tiny_author.bytes).as_ptr() as *const i8);
        (*d).author = (*g).xrep.string;
        must_semi(g, (tiny_author.bytes).as_ptr() as *const i8);
        must_read_keyword(g, &tiny_state);
        must_read_snippet(g, (tiny_state.bytes).as_ptr() as *const i8);
        (*d).state = (*g).xrep.string;
        must_semi(g, (tiny_state.bytes).as_ptr() as *const i8);
        must_read_keyword(g, &tiny_branches);
        box_0.next = 0 as *mut link;
        tp = &mut box_0;
        while maybe_read_num(g, 1 as i32 != 0) {
            let mut gs: *const i8 = (*g).xrep.string;
            if numlen >= (*g).xrep.size || strncmp((*d).num, gs, numlen) != 0
                || '.' as i32 != *gs.offset(numlen as isize) as i32
                || 2 as i32
                    != countnumflds(gs.offset(numlen as isize).offset(1 as i32 as isize))
            {
                ignoble(
                    g,
                    b"invalid branch `%s' at branchpoint `%s'\0" as *const u8
                        as *const i8,
                    gs,
                    (*d).num,
                );
            }
            fw = alloc((*g).tranquil, ::core::mem::size_of::<fwref>() as u64)
                as *mut fwref;
            (*fw).revno = gs;
            (*fw).lno = (*g).lno;
            all_br = wprepend(fw as *mut libc::c_void, all_br, (*g).tranquil);
            tp = extend(tp, gs as *const libc::c_void, to);
        }
        (*ny).branches = box_0.next;
        must_semi(g, (tiny_branches.bytes).as_ptr() as *const i8);
        must_read_keyword(g, &tiny_next);
        if maybe_read_num(g, 1 as i32 != 0) {
            (*ny).next = (*g).xrep.string;
        } else {
            (*ny).next = 0 as *const i8;
        }
        must_semi(g, (tiny_next.bytes).as_ptr() as *const i8);
        if probe_keyword(g, &tiny_commitid) {
            must_read_snippet(g, (tiny_commitid.bytes).as_ptr() as *const i8);
            (*d).commitid = (*g).xrep.string;
            checkssym((*d).commitid);
            must_semi(g, (tiny_commitid.bytes).as_ptr() as *const i8);
        } else {
            (*d).commitid = 0 as *const i8;
        }
        wtp = wextend(wtp, ny as *mut libc::c_void, to);
        puthash(to, ny, (*repo).ht);
        prev = ny;
        count = count.wrapping_add(1);
        count;
    }
    while !all_br.is_null() {
        fw = (*all_br).entry as *mut fwref;
        if (*fw).lno != 0 {
            (*g).lno = (*fw).lno;
            ignoble(
                g,
                b"branch refers to %s `%s'\0" as *const u8 as *const i8,
                ks_ner.as_ptr(),
                (*fw).revno,
            );
        }
        all_br = (*all_br).next;
    }
    (*repo).deltas = wbox.next;
    (*repo).deltas_count = count;
    must_read_keyword(g, &tiny_desc);
    (*repo).neck = fro_tello((*g).from);
    must_read_atat(g, &mut (*repo).desc, (tiny_desc.bytes).as_ptr() as *const i8);
    let mut lock_0: *const lockdef = (*repo).lockdefs;
    while lock_0 < ((*repo).lockdefs).offset((*repo).locks_count as isize) {
        let mut ny_0: *mut notyet = gethash((*lock_0).revno, (*repo).ht) as *mut notyet;
        if ny_0.is_null() {
            generic_warn(
                (*top).repository.filename,
                b"user `%s' holds a lock for %s `%s'\0" as *const u8 as *const i8,
                (*lock_0).login,
                ks_ner.as_ptr(),
                (*lock_0).revno,
            );
            ny_0 = zlloc(to, ::core::mem::size_of::<notyet>() as u64) as *mut notyet;
            (*ny_0).d = zlloc(to, ::core::mem::size_of::<delta>() as u64) as *mut delta;
            (*(*ny_0).d).num = (*lock_0).revno;
            (*ny_0).revno = (*(*ny_0).d).num;
            puthash(to, ny_0, (*repo).ht);
        }
        lock_0 = lock_0.offset(1);
        lock_0;
    }
    count = 0 as i32 as size_t;
    follow = (*repo).deltas;
    loop {
        neck = fro_tello((*g).from);
        if !(neck != 0 && count < (*repo).deltas_count) {
            break;
        }
        let mut revno: *const i8 = 0 as *const i8;
        let mut ny_1: *mut notyet = 0 as *mut notyet;
        let mut d_0: *mut delta = 0 as *mut delta;
        must_read_num(g, ks_revno.as_ptr());
        revno = (*g).xrep.string;
        ny_1 = gethash(revno, (*repo).ht) as *mut notyet;
        if ny_1.is_null() {
            ignoble(
                g,
                b"found edits for %s `%s'\0" as *const u8 as *const i8,
                ks_ner.as_ptr(),
                revno,
            );
        }
        (*follow).entry = ny_1 as *mut libc::c_void;
        follow = (*follow).next;
        d_0 = (*ny_1).d;
        if !((*d_0).log).is_null() {
            ignoble(
                g,
                b"duplicate delta log for %s `%s'\0" as *const u8 as *const i8,
                ks_revno.as_ptr(),
                (*d_0).num,
            );
        }
        (*d_0).neck = neck;
        must_read_keyword(g, &tiny_log);
        must_read_atat(g, &mut (*d_0).log, (tiny_log.bytes).as_ptr() as *const i8);
        must_read_keyword(g, &tiny_text);
        must_read_atat(g, &mut (*d_0).text, (tiny_text.bytes).as_ptr() as *const i8);
        count = count.wrapping_add(1);
        count;
    }
    loop {
        if !(*(*__ctype_b_loc()).offset((*g).c as isize) as i32
            & C2RustUnnamed_3::_ISspace as i32 as libc::c_ushort as i32 != 0)
        {
            current_block = 4338462691184853296;
            break;
        }
        if '\n' as i32 == (*g).c {
            (*g).lno = ((*g).lno).wrapping_add(1);
            (*g).lno;
        }
        if fro_try_getbyte(&mut (*g).c, (*g).from) {
            current_block = 5250576585193495047;
            break;
        }
    }
    match current_block {
        4338462691184853296 => {
            ignoble(g, b"junk at end of file: '%c'\0" as *const u8 as *const i8, (*g).c);
        }
        _ => {}
    }
    if !((*repo).head).is_null() && (gethash((*repo).head, (*repo).ht)).is_null() {
        fatal_syntax(
            (*g).head_lno,
            b"RCS file head names a %s `%s'\0" as *const u8 as *const i8,
            ks_ner.as_ptr(),
            (*repo).head,
        );
    }
    let mut ls_0: *mut wlink = (*repo).deltas;
    while !ls_0.is_null() {
        let mut ny_2: *mut notyet = (*ls_0).entry as *mut notyet;
        let mut deref: *mut notyet = 0 as *mut notyet;
        let mut d_1: *mut delta = (*ny_2).d;
        if !((*ny_2).next).is_null() {
            deref = gethash((*ny_2).next, (*repo).ht) as *mut notyet;
            (*d_1).ilk = (*deref).d;
        }
        if !((*ny_2).branches).is_null() {
            let mut bls: *mut link = 0 as *mut link;
            let mut wbox_0: wlink = wlink {
                entry: 0 as *mut libc::c_void,
                next: 0 as *mut wlink,
            };
            let mut wtp_0: *mut wlink = 0 as *mut wlink;
            bls = (*ny_2).branches;
            wbox_0.next = (*d_1).branches;
            wtp_0 = &mut wbox_0;
            while !bls.is_null() {
                deref = gethash((*bls).entry as *const i8, (*repo).ht) as *mut notyet;
                wtp_0 = wextend(wtp_0, (*deref).d as *mut libc::c_void, to);
                bls = (*bls).next;
            }
            (*d_1).branches = wbox_0.next;
        }
        (*ls_0).entry = d_1 as *mut libc::c_void;
        ls_0 = (*ls_0).next;
    }
    close_space((*g).systolic);
    close_space((*g).tranquil);
    return repo;
}
#[no_mangle]
pub unsafe extern "C" fn grok_all(mut to: *mut divvy, mut f: *mut fro) -> *mut repo {
    let mut repo: *mut repo = full(to, f);
    grok_resynch(repo);
    return repo;
}
#[no_mangle]
pub unsafe extern "C" fn grok_resynch(mut repo: *mut repo) {
    let mut ny: *mut notyet = 0 as *mut notyet;
    (*top).repository.tip = if !((*repo).head).is_null()
        && {
            ny = gethash((*repo).head, (*repo).ht) as *mut notyet;
            !ny.is_null()
        }
    {
        (*ny).d
    } else {
        0 as *mut delta
    };
    (*repo).locks = 0 as *mut link;
    let mut orig: *const lockdef = ((*repo).lockdefs)
        .offset((*repo).locks_count as isize);
    loop {
        let fresh2 = orig;
        orig = orig.offset(-1);
        if !((*repo).lockdefs < fresh2 as *mut lockdef
            && {
                ny = gethash((*orig).revno, (*repo).ht) as *mut notyet;
                !ny.is_null()
            })
        {
            break;
        }
        let mut d: *mut delta = (*ny).d;
        let mut rl: *mut rcslock = alloc(
            single,
            ::core::mem::size_of::<rcslock>() as u64,
        ) as *mut rcslock;
        (*d).lockedby = (*orig).login;
        (*rl).login = (*d).lockedby;
        (*rl).delta = d;
        (*repo).locks = prepend(rl as *const libc::c_void, (*repo).locks, single);
    }
    (*top).behavior.strictly_locking = (*repo).strict;
    if !((*repo).comment).is_null() {
        (*top).repository.log_lead = string_from_atat(single, (*repo).comment);
    } else {
        (*top).repository.log_lead.string = 0 as *const i8;
        (*top).repository.log_lead.size = 0 as i32 as size_t;
    }
    (*top).behavior.kws = if 0 as i32 > (*repo).expand {
        kwsub::kwsub_kv as i32
    } else {
        (*repo).expand
    };
}