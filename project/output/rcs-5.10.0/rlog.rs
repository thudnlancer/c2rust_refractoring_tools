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
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    static equal_line: [i8; 0];
    static mut top: *mut top;
    static mut stdout: *mut _IO_FILE;
    fn kwsub_string(i: kwsub) -> *const i8;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: i32,
        argv: *mut *mut i8,
        rcsopen: Option<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> i32;
    fn format_assocs(out: *mut FILE, fmt: *const i8);
    fn format_locks(out: *mut FILE, fmt: *const i8);
    fn countnumflds(s: *const i8) -> i32;
    fn take(count: size_t, ref_0: *const i8) -> cbuf;
    fn cmpdate(d1: *const i8, d2: *const i8) -> i32;
    fn compartial(num1: *const i8, num2: *const i8, length: i32) -> i32;
    fn fully_numeric(ans: *mut cbuf, source: *const i8, fp: *mut fro) -> bool;
    fn tiprev() -> *const i8;
    fn str2date(source: *const i8, target: *mut i8);
    fn zone_set(s: *const i8);
    fn date2str(date: *const i8, datebuf: *mut i8) -> *const i8;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn parse_revpairs(
        option: i8,
        arg: *mut i8,
        data: *mut libc::c_void,
        put: Option<
            unsafe extern "C" fn(*const i8, *const i8, bool, *mut libc::c_void) -> (),
        >,
    );
    fn ffree();
    fn awrite(buf: *const i8, chars: size_t, f: *mut FILE);
    fn setRCSversion(str: *const i8);
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn __errno_location() -> *mut i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    static mut exit_failure: i32;
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn accumulate_range(divvy: *mut divvy, beg: *const i8, end: *const i8);
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
    fn getusername(suspicious: bool) -> *const i8;
    fn lock_on(delta: *const delta) -> *const rcslock;
    fn Ozclose(p: *mut *mut FILE);
    fn afputc(c: i32, f: *mut FILE);
    fn newline(f: *mut FILE);
    fn aputs(s: *const i8, iop: *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const i8, _: ...);
    fn fro_zclose(p: *mut *mut fro);
    fn string_from_atat(space: *mut divvy, atat: *const atat) -> cbuf;
    fn atat_display(to: *mut FILE, atat: *const atat, ensure_end_nl: bool);
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
    kwsub_kv,
    kwsub_kvl,
    kwsub_k,
    kwsub_v,
    kwsub_o,
    kwsub_b,
}
impl kwsub {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            kwsub::kwsub_kv => 0,
            kwsub::kwsub_kvl => 1,
            kwsub::kwsub_k => 2,
            kwsub::kwsub_v => 3,
            kwsub::kwsub_o => 4,
            kwsub::kwsub_b => 5,
        }
    }
    fn from_libc_c_uint(value: u32) -> kwsub {
        match value {
            0 => kwsub::kwsub_kv,
            1 => kwsub::kwsub_kvl,
            2 => kwsub::kwsub_k,
            3 => kwsub::kwsub_v,
            4 => kwsub::kwsub_o,
            5 => kwsub::kwsub_b,
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
pub struct rcslock {
    pub login: *const i8,
    pub delta: *mut delta,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maybe {
    pub open: Option<open_rcsfile_fn>,
    pub mustread: bool,
    pub tentative: cbuf,
    pub space: *mut divvy,
    pub bestfit: cbuf,
    pub status: *mut stat,
    pub eno: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const i8,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
}
pub type open_rcsfile_fn = unsafe extern "C" fn(*mut maybe) -> *mut fro;
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
pub type submain_t = unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yacmd {
    pub func: Option<submain_t>,
    pub aka: *const uint8_t,
    pub pr: *mut program,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct revrange {
    pub beg: *const i8,
    pub end: *const i8,
    pub nfield: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daterange {
    pub beg: [i8; 22],
    pub end: [i8; 22],
    pub open_end: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct date_selection {
    pub in_0: *mut link,
    pub by: *mut link,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct criteria {
    pub revs: *mut link,
    pub actual: *mut link,
    pub authors: *mut link,
    pub lockers: *mut link,
    pub states: *mut link,
}
static mut rlog_blurb: [i8; 58] = unsafe {
    *::core::mem::transmute::<
        &[u8; 58],
        &[i8; 58],
    >(b"Print log messages and other information about RCS files.\0")
};
static mut rlog_help: [i8; 1601] = unsafe {
    *::core::mem::transmute::<
        &[u8; 1601],
        &[i8; 1601],
    >(
        b"[options] file ...\nOptions:\n  -L            Ignore RCS files with no locks set.\n  -R            Print the RCS file name only.\n  -h            Print only the \"header\" information.\n  -t            Like -h, but also include the description.\n  -N            Omit symbolic names.\n  -b            Select the default branch.\n  -dDATES       Select revisions in the range DATES, with spec:\n                  D      -- single revision D or earlier\n                  D1<D2  -- between D1 and D2, exclusive\n                  D2>D1  -- likewise\n                  <D, D> -- before D\n                  >D, D< -- after D\n                Use <= or >= to make ranges inclusive; DATES\n                may also be a list of semicolon-separated specs.\n  -l[WHO]       Select revisions locked by WHO (comma-separated list)\n                only, or by anyone if WHO is omitted.\n  -r[REVS]      Select revisions in REVS, a comma-separated list of\n                range specs, one of: REV, REV:, :REV, REV1:REV2\n  -sSTATES      Select revisions with state in STATES (comma-separated list).\n  -w[WHO]       Select revisions checked in by WHO (comma-separated list),\n                or by the user if WHO is omitted.\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n  -q            No effect, included for consistency with other commands.\n\0",
    )
};
static mut ks_delims: [i8; 6] = unsafe {
    *::core::mem::transmute::<&[u8; 6], &[i8; 6]>(b", \t\n;\0")
};
unsafe extern "C" fn tokenize(mut argv: *mut i8, mut chain: *mut *mut link) -> bool {
    let mut before: *mut link = *chain;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut save: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    s = argv;
    loop {
        token = strtok_r(s, ks_delims.as_ptr(), &mut save);
        if token.is_null() {
            break;
        }
        *chain = prepend(token as *const libc::c_void, *chain, plexus);
        s = 0 as *mut i8;
    }
    return *chain != before;
}
unsafe extern "C" fn cleanup(mut exitstatus: *mut i32) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
}
unsafe extern "C" fn getlocker(mut argv: *mut i8, mut criteria: *mut criteria) {
    tokenize(argv, &mut (*criteria).lockers);
}
unsafe extern "C" fn read_positive_integer(mut p: *mut *const i8) -> i64 {
    let mut rv: i64 = 0;
    let mut end: *mut i8 = 0 as *mut i8;
    *__errno_location() = 0 as i32;
    rv = strtol(*p, &mut end, 10 as i32);
    if 1 as i32 as i64 > rv {
        generic_fatal(
            (*top).repository.filename,
            b"non-positive integer\0" as *const u8 as *const i8,
        );
    }
    if 34 as i32 == *__errno_location() {
        generic_fatal(
            (*top).repository.filename,
            b"bad integer\0" as *const u8 as *const i8,
        );
    }
    *p = end;
    return rv;
}
unsafe extern "C" fn count_a_d(mut a: *mut i64, mut d: *mut i64, mut edits: *mut atat) {
    let mut s: cbuf = string_from_atat(single, edits);
    let mut end: *const i8 = (s.string).offset(s.size as isize);
    let mut totals: *mut i64 = zlloc(
        single,
        (2 as i32 as u64).wrapping_mul(::core::mem::size_of::<i64>() as u64),
    ) as *mut i64;
    let mut p: *const i8 = s.string;
    's_8: while p < end {
        let fresh1 = p;
        p = p.offset(1);
        let mut add: bool = 'a' as i32 == *fresh1 as i32;
        let mut count: i64 = 0;
        p = strchr(p, ' ' as i32);
        count = read_positive_integer(&mut p);
        *totals.offset(add as isize) += count;
        if add {
            loop {
                let fresh2 = count;
                count = count - 1;
                if !(fresh2 != 0) {
                    break;
                }
                let mut remaining: size_t = end.offset_from(p) as i64 as size_t;
                p = p.offset(1);
                p = memchr(p as *const libc::c_void, '\n' as i32, remaining)
                    as *const i8;
                if p.is_null() {
                    break 's_8;
                }
            }
        }
        p = p.offset(1);
        p;
    }
    *a = *totals.offset(1 as i32 as isize);
    *d = *totals.offset(0 as i32 as isize);
    brush_off(single, totals as *mut libc::c_void);
}
unsafe extern "C" fn putadelta(
    mut node: *const delta,
    mut editscript: *const delta,
    mut insDelFormat: *const i8,
) {
    let mut out: *mut FILE = stdout;
    let mut datebuf: [i8; 31] = [0; 31];
    let mut pre5: bool = (*top).behavior.version < 5 as i32 - 5 as i32;
    let mut log: *mut atat = 0 as *mut atat;
    aprintf(
        out,
        b"----------------------------\nrevision %s%s\0" as *const u8 as *const i8,
        (*node).num,
        if pre5 as i32 != 0 {
            b"        \0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    if !((*node).lockedby).is_null() {
        aprintf(
            out,
            (b"\tlocked by: %s;\0" as *const u8 as *const i8)
                .offset(pre5 as i32 as isize),
            (*node).lockedby,
        );
    }
    aprintf(
        out,
        b"\ndate: %s;  author: %s;  state: %s;\0" as *const u8 as *const i8,
        date2str((*node).date, datebuf.as_mut_ptr()),
        (*node).author,
        (*node).state,
    );
    if !editscript.is_null() && editscript != (*top).repository.tip {
        let mut trunk: bool = node != editscript;
        let mut a: i64 = 0;
        let mut d: i64 = 0;
        count_a_d(
            if trunk as i32 != 0 { &mut d } else { &mut a },
            if trunk as i32 != 0 { &mut a } else { &mut d },
            (*editscript).text,
        );
        aprintf(out, insDelFormat, a, d);
    }
    if !((*node).branches).is_null() {
        aputs(b"\nbranches:\0" as *const u8 as *const i8, out);
        let mut ls: *mut wlink = (*node).branches;
        while !ls.is_null() {
            let mut delta: *mut delta = (*ls).entry as *mut delta;
            aprintf(
                out,
                b"  %s;\0" as *const u8 as *const i8,
                (take(0 as i32 as size_t, (*delta).num)).string,
            );
            ls = (*ls).next;
        }
    }
    if !((*node).commitid).is_null() {
        aprintf(
            out,
            b"%s commitid: %s\0" as *const u8 as *const i8,
            if !editscript.is_null() {
                b";\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            (*node).commitid,
        );
    }
    newline(out);
    log = (*node).log;
    if !log.is_null()
        && ((*log).beg + 1 as i32 as i64)
            < *((*log).holes)
                .as_mut_ptr()
                .offset(((*log).count).wrapping_sub(1 as i32 as u64) as isize)
    {
        atat_display(out, log, 1 as i32 != 0);
    } else {
        awrite(
            b"*** empty log message ***\n\0" as *const u8 as *const i8,
            ::core::mem::size_of::<[i8; 26]>() as u64,
            out,
        );
    };
}
unsafe extern "C" fn putrunk(mut insDelFormat: *const i8) {
    let mut ptr: *const delta = (*top).repository.tip;
    while !ptr.is_null() {
        if (*ptr).selector {
            putadelta(ptr, (*ptr).ilk, insDelFormat);
        }
        ptr = (*ptr).ilk;
    }
}
unsafe extern "C" fn putree(mut root: *const delta, mut insDelFormat: *const i8) {
    while !root.is_null() {
        if ((*root).branches).is_null() {
            root = (*root).ilk;
        } else {
            putree((*root).ilk, insDelFormat);
            root = putforest((*root).branches, insDelFormat);
        }
    }
}
unsafe extern "C" fn putabranch(mut root: *const delta, mut insDelFormat: *const i8) {
    while !(*root).selector {
        root = (*root).ilk;
        if root.is_null() {
            return;
        }
    }
    if !((*root).ilk).is_null() {
        putabranch((*root).ilk, insDelFormat);
    }
    putadelta(root, root, insDelFormat);
}
unsafe extern "C" fn putforest(
    mut branchroot: *const wlink,
    mut insDelFormat: *const i8,
) -> *const delta {
    if !((*branchroot).next).is_null() {
        putforest((*branchroot).next, insDelFormat);
    }
    putabranch((*branchroot).entry as *const delta, insDelFormat);
    return (*branchroot).entry as *const delta;
}
unsafe extern "C" fn extractdelta(
    mut pdelta: *const delta,
    mut lockflag: bool,
    mut criteria: *mut criteria,
) -> bool {
    let mut pstate: *const link = 0 as *const link;
    let mut pauthor: *const link = 0 as *const link;
    let mut length: i32 = 0;
    pauthor = (*criteria).authors;
    if !pauthor.is_null() {
        while strcmp((*pauthor).entry as *const i8, (*pdelta).author) != 0 {
            pauthor = (*pauthor).next;
            if pauthor.is_null() {
                return 0 as i32 != 0;
            }
        }
    }
    pstate = (*criteria).states;
    if !pstate.is_null() {
        while strcmp((*pstate).entry as *const i8, (*pdelta).state) != 0 {
            pstate = (*pstate).next;
            if pstate.is_null() {
                return 0 as i32 != 0;
            }
        }
    }
    if lockflag as i32 != 0 && (lock_on(pdelta)).is_null() {
        return 0 as i32 != 0;
    }
    let mut ls: *mut link = (*criteria).actual;
    while !ls.is_null() {
        let mut rr: *const revrange = (*ls).entry as *const revrange;
        length = (*rr).nfield;
        if countnumflds((*pdelta).num) == length + (length & 1 as i32)
            && 0 as i32 <= compartial((*pdelta).num, (*rr).beg, length)
            && 0 as i32 <= compartial((*rr).end, (*pdelta).num, length)
        {
            break;
        }
        ls = (*ls).next;
        if ls.is_null() {
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn exttree(
    mut root: *mut delta,
    mut lockflag: bool,
    mut criteria: *mut criteria,
) {
    while !root.is_null() {
        (*root).selector = extractdelta(root, lockflag, criteria);
        (*root).pretty_log.string = 0 as *const i8;
        if ((*root).branches).is_null() {
            root = (*root).ilk;
        } else {
            let mut ls: *mut wlink = 0 as *mut wlink;
            exttree((*root).ilk, lockflag, criteria);
            ls = (*root).branches;
            while !((*ls).next).is_null() {
                exttree((*ls).entry as *mut delta, lockflag, criteria);
                ls = (*ls).next;
            }
            root = (*ls).entry as *mut delta;
        }
    }
}
unsafe extern "C" fn getauthor(mut argv: *mut i8, mut criteria: *mut criteria) {
    if !tokenize(argv, &mut (*criteria).authors) {
        (*criteria).authors = prepend(
            getusername(0 as i32 != 0) as *const libc::c_void,
            (*criteria).authors,
            plexus,
        );
    }
}
unsafe extern "C" fn getstate(mut argv: *mut i8, mut criteria: *mut criteria) {
    if !tokenize(argv, &mut (*criteria).states) {
        generic_error(
            0 as *const i8,
            b"missing state attributes after -s option\0" as *const u8 as *const i8,
        );
    }
}
unsafe extern "C" fn trunclocks(mut criteria: *mut criteria) {
    let mut plocker: *const link = 0 as *const link;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    if ((*criteria).lockers).is_null() {
        return;
    }
    box_0.next = (*(*top).repository.r).locks;
    tp = &mut box_0;
    while !((*tp).next).is_null() {
        let mut rl: *const rcslock = (*(*tp).next).entry as *const rcslock;
        plocker = (*criteria).lockers;
        loop {
            if strcmp((*plocker).entry as *const i8, (*rl).login) == 0 {
                tp = (*tp).next;
                break;
            } else {
                plocker = (*plocker).next;
                if !plocker.is_null() {
                    continue;
                }
                (*tp).next = (*(*tp).next).next;
                (*(*top).repository.r).locks = box_0.next;
                break;
            }
        }
    }
}
unsafe extern "C" fn recentdate(mut root: *const delta, mut r: *mut daterange) {
    while !root.is_null() {
        if (*root).selector as i32 != 0
            && !(0 as i32 > cmpdate((*root).date, ((*r).beg).as_mut_ptr()))
            && !((0 as i32) < cmpdate((*root).date, ((*r).end).as_mut_ptr()))
        {
            strncpy(
                ((*r).beg).as_mut_ptr(),
                (*root).date,
                (6 as i32 + 16 as i32) as u64,
            );
            (*r).beg[(6 as i32 + 16 as i32 - 1 as i32) as usize] = '\0' as i32 as i8;
        }
        let mut ls: *mut wlink = (*root).branches;
        if ls.is_null() {
            root = (*root).ilk;
        } else {
            while !((*ls).next).is_null() {
                recentdate((*ls).entry as *const delta, r);
                ls = (*ls).next;
            }
            root = (*ls).entry as *const delta;
        }
    }
}
unsafe extern "C" fn extdate(
    mut root: *mut delta,
    mut datesel: *mut date_selection,
) -> size_t {
    let mut revno: size_t = 0 as i32 as size_t;
    while !root.is_null() {
        if !((*datesel).in_0).is_null() || !((*datesel).by).is_null() {
            let mut r: *const daterange = 0 as *const daterange;
            let mut open_end: bool = false;
            let mut sel: bool = 0 as i32 != 0;
            let mut ls: *mut link = (*datesel).in_0;
            while !ls.is_null() {
                r = (*ls).entry as *const daterange;
                open_end = (*r).open_end;
                sel = ((*r).beg[0 as i32 as usize] == 0
                    || (if open_end as i32 != 0 {
                        (0 as i32 > cmpdate(((*r).beg).as_ptr(), (*root).date)) as i32
                    } else {
                        !((0 as i32) < cmpdate(((*r).beg).as_ptr(), (*root).date)) as i32
                    }) != 0)
                    && ((*r).end[0 as i32 as usize] == 0
                        || (if open_end as i32 != 0 {
                            (0 as i32 > cmpdate((*root).date, ((*r).end).as_ptr()))
                                as i32
                        } else {
                            !((0 as i32) < cmpdate((*root).date, ((*r).end).as_ptr()))
                                as i32
                        }) != 0);
                if sel {
                    break;
                }
                ls = (*ls).next;
            }
            if !sel {
                let mut ls_0: *mut link = (*datesel).by;
                while !ls_0.is_null() {
                    r = (*ls_0).entry as *const daterange;
                    sel = 0 as i32 == cmpdate((*root).date, ((*r).beg).as_ptr());
                    if sel {
                        break;
                    }
                    ls_0 = (*ls_0).next;
                }
                if !sel {
                    (*root).selector = 0 as i32 != 0;
                }
            }
        }
        revno = (revno as u64).wrapping_add((*root).selector as u64) as size_t as size_t;
        let mut ls_1: *mut wlink = (*root).branches;
        while !ls_1.is_null() {
            revno = (revno as u64)
                .wrapping_add(extdate((*ls_1).entry as *mut delta, datesel)) as size_t
                as size_t;
            ls_1 = (*ls_1).next;
        }
        root = (*root).ilk;
    }
    return revno;
}
unsafe extern "C" fn getdatepair(mut argv: *mut i8, mut datesel: *mut date_selection) {
    let mut current_block: u64;
    let mut c: i8 = 0;
    let mut r: *mut daterange = 0 as *mut daterange;
    let mut rawdate: *const i8 = 0 as *const i8;
    let mut switchflag: bool = false;
    argv = argv.offset(-1);
    argv;
    loop {
        argv = argv.offset(1);
        c = *argv;
        if !(c as i32 == ',' as i32 || c as i32 == ' ' as i32 || c as i32 == '\t' as i32
            || c as i32 == '\n' as i32 || c as i32 == ';' as i32)
        {
            break;
        }
    }
    if c as i32 == '\0' as i32 {
        generic_error(
            0 as *const i8,
            b"missing date/time after -d\0" as *const u8 as *const i8,
        );
        return;
    }
    while c as i32 != '\0' as i32 {
        switchflag = 0 as i32 != 0;
        r = zlloc(
            plexus,
            (::core::mem::size_of::<daterange>() as u64).wrapping_mul(1 as i32 as u64),
        ) as *mut daterange;
        if c as i32 == '<' as i32 {
            argv = argv.offset(1);
            c = *argv;
            (*r).open_end = c as i32 != '=' as i32;
            if !(*r).open_end {
                argv = argv.offset(1);
                c = *argv;
            }
            (*r).beg[0 as i32 as usize] = '\0' as i32 as i8;
            current_block = 18377268871191777778;
        } else if c as i32 == '>' as i32 {
            argv = argv.offset(1);
            c = *argv;
            (*r).open_end = c as i32 != '=' as i32;
            if !(*r).open_end {
                argv = argv.offset(1);
                c = *argv;
            }
            (*r).end[0 as i32 as usize] = '\0' as i32 as i8;
            switchflag = 1 as i32 != 0;
            current_block = 18377268871191777778;
        } else {
            rawdate = argv;
            while c as i32 != '<' as i32 && c as i32 != '>' as i32
                && c as i32 != ';' as i32 && c as i32 != '\0' as i32
            {
                argv = argv.offset(1);
                c = *argv;
            }
            *argv = '\0' as i32 as i8;
            if c as i32 == '>' as i32 {
                switchflag = 1 as i32 != 0;
            }
            str2date(
                rawdate,
                if switchflag as i32 != 0 {
                    ((*r).end).as_mut_ptr()
                } else {
                    ((*r).beg).as_mut_ptr()
                },
            );
            if c as i32 == ';' as i32 || c as i32 == '\0' as i32 {
                memcpy(
                    ((*r).end).as_mut_ptr() as *mut libc::c_void,
                    ((*r).beg).as_mut_ptr() as *const libc::c_void,
                    (6 as i32 + 16 as i32) as u64,
                );
                (*datesel).by = prepend(r as *const libc::c_void, (*datesel).by, plexus);
                current_block = 15178606439809983032;
            } else {
                let mut eq: bool = *argv.offset(1 as i32 as isize) as i32 == '=' as i32;
                (*r).open_end = !eq;
                argv = argv.offset(eq as i32 as isize);
                loop {
                    argv = argv.offset(1);
                    c = *argv;
                    if !(c as i32 == ' ' as i32 || c as i32 == '\t' as i32
                        || c as i32 == '\n' as i32)
                    {
                        break;
                    }
                }
                if c as i32 == ';' as i32 || c as i32 == '\0' as i32 {
                    *if switchflag as i32 != 0 {
                        ((*r).beg).as_mut_ptr()
                    } else {
                        ((*r).end).as_mut_ptr()
                    }
                        .offset(0 as i32 as isize) = '\0' as i32 as i8;
                    (*datesel).in_0 = prepend(
                        r as *const libc::c_void,
                        (*datesel).in_0,
                        plexus,
                    );
                    current_block = 15178606439809983032;
                } else {
                    current_block = 18377268871191777778;
                }
            }
        }
        match current_block {
            18377268871191777778 => {
                rawdate = argv;
                while c as i32 != '>' as i32 && c as i32 != '<' as i32
                    && c as i32 != ';' as i32 && c as i32 != '\0' as i32
                {
                    argv = argv.offset(1);
                    c = *argv;
                }
                *argv = '\0' as i32 as i8;
                str2date(
                    rawdate,
                    if switchflag as i32 != 0 {
                        ((*r).beg).as_mut_ptr()
                    } else {
                        ((*r).end).as_mut_ptr()
                    },
                );
                (*datesel).in_0 = prepend(
                    r as *const libc::c_void,
                    (*datesel).in_0,
                    plexus,
                );
            }
            _ => {}
        }
        if (*top).behavior.version < 5 as i32 - 5 as i32 {
            (*r).open_end = 0 as i32 != 0;
        }
        if c as i32 == '\0' as i32 {
            return;
        }
        loop {
            argv = argv.offset(1);
            c = *argv;
            if !(c as i32 == ';' as i32 || c as i32 == ' ' as i32
                || c as i32 == '\t' as i32 || c as i32 == '\n' as i32)
            {
                break;
            }
        }
    }
}
unsafe extern "C" fn checkrevpair(mut num1: *const i8, mut num2: *const i8) -> bool {
    let mut length: i32 = countnumflds(num1);
    if countnumflds(num2) != length
        || (2 as i32) < length && compartial(num1, num2, length - 1 as i32) != 0 as i32
    {
        generic_error(
            (*top).repository.filename,
            b"invalid branch or revision pair %s : %s\0" as *const u8 as *const i8,
            num1,
            num2,
        );
        return 0 as i32 != 0;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn getnumericrev(
    mut branchflag: bool,
    mut criteria: *mut criteria,
) -> bool {
    let mut current_block: u64;
    let mut ls: *mut link = 0 as *mut link;
    let mut rr: *mut revrange = 0 as *mut revrange;
    let mut n: i32 = 0;
    let mut s: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut e: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut lrev: *const i8 = 0 as *const i8;
    let mut rstart: *const cbuf = 0 as *const cbuf;
    let mut rend: *const cbuf = 0 as *const cbuf;
    let mut tip: *mut delta = (*top).repository.tip;
    let mut defbr: *const i8 = (*(*top).repository.r).branch;
    (*criteria).actual = 0 as *mut link;
    ls = (*criteria).revs;
    loop {
        if ls.is_null() {
            current_block = 18153031941552419006;
            break;
        }
        let mut from: *const revrange = (*ls).entry as *const revrange;
        n = 0 as i32;
        rstart = &mut s;
        rend = &mut e;
        match (*from).nfield {
            1 => {
                if !fully_numeric(&mut s, (*from).beg, 0 as *mut fro) {
                    current_block = 454450084440093415;
                    break;
                }
                rend = &mut s;
                n = countnumflds(s.string);
                if n == 0
                    && {
                        lrev = tiprev();
                        !lrev.is_null()
                    }
                {
                    s.string = lrev;
                    n = countnumflds(lrev);
                }
            }
            2 => {
                if !fully_numeric(&mut s, (*from).beg, 0 as *mut fro) {
                    current_block = 454450084440093415;
                    break;
                }
                n = countnumflds(s.string);
                e.string = if 2 as i32 > n {
                    b"\0" as *const u8 as *const i8
                } else {
                    accumulate_range(plexus, s.string, strrchr(s.string, '.' as i32));
                    finish_string(plexus, &mut e.size)
                };
            }
            3 => {
                if !fully_numeric(&mut e, (*from).end, 0 as *mut fro) {
                    current_block = 454450084440093415;
                    break;
                }
                n = countnumflds(e.string);
                if n < 2 as i32 {
                    s.string = b".0\0" as *const u8 as *const i8;
                } else {
                    accumulate_range(plexus, e.string, strrchr(e.string, '.' as i32));
                    accf(plexus, b".0\0" as *const u8 as *const i8);
                    s.string = finish_string(plexus, &mut s.size);
                }
            }
            _ => {
                if !(fully_numeric(&mut s, (*from).beg, 0 as *mut fro) as i32 != 0
                    && fully_numeric(&mut e, (*from).end, 0 as *mut fro) as i32 != 0
                    && checkrevpair(s.string, e.string) as i32 != 0)
                {
                    current_block = 454450084440093415;
                    break;
                }
                n = countnumflds(s.string);
                if compartial(s.string, e.string, n) > 0 as i32 {
                    rstart = &mut e;
                    rend = &mut s;
                }
            }
        }
        if n != 0 {
            rr = alloc(single, ::core::mem::size_of::<revrange>() as u64)
                as *mut revrange;
            (*rr).nfield = n;
            (*rr).beg = (*rstart).string;
            (*rr).end = (*rend).string;
            (*criteria).actual = prepend(
                rr as *const libc::c_void,
                (*criteria).actual,
                plexus,
            );
        }
        ls = (*ls).next;
    }
    match current_block {
        18153031941552419006 => {
            if branchflag as i32 != 0 && (!defbr.is_null() || !tip.is_null()) {
                rr = alloc(single, ::core::mem::size_of::<revrange>() as u64)
                    as *mut revrange;
                (*rr).end = if !defbr.is_null() {
                    defbr
                } else {
                    (take(1 as i32 as size_t, (*tip).num)).string
                };
                (*rr).beg = (*rr).end;
                (*rr).nfield = countnumflds((*rr).beg);
                (*criteria).actual = prepend(
                    rr as *const libc::c_void,
                    (*criteria).actual,
                    plexus,
                );
            }
        }
        _ => {}
    }
    return ls.is_null();
}
unsafe extern "C" fn putrevpairs(
    mut b: *const i8,
    mut e: *const i8,
    mut sawsep: bool,
    mut data: *mut libc::c_void,
) {
    let mut criteria: *mut criteria = data as *mut criteria;
    let mut rr: *mut revrange = zlloc(
        plexus,
        (::core::mem::size_of::<revrange>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut revrange;
    (*rr).beg = b;
    (*rr).end = e;
    (*rr).nfield = if !sawsep {
        1 as i32
    } else if *e.offset(0 as i32 as isize) == 0 {
        2 as i32
    } else if *b.offset(0 as i32 as isize) == 0 {
        3 as i32
    } else {
        4 as i32
    };
    (*criteria).revs = prepend(rr as *const libc::c_void, (*criteria).revs, plexus);
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: rlog_blurb.as_ptr(),
            help: rlog_help.as_ptr(),
            tyag: 0 as i32,
        };
        init
    }
};
unsafe extern "C" fn rlog_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut exitstatus: i32 = 0 as i32;
    let mut branchflag: bool = 0 as i32 != 0;
    let mut lockflag: bool = 0 as i32 != 0;
    let mut datesel: date_selection = {
        let mut init = date_selection {
            in_0: 0 as *mut link,
            by: 0 as *mut link,
        };
        init
    };
    let mut criteria: criteria = {
        let mut init = criteria {
            revs: 0 as *mut link,
            actual: 0 as *mut link,
            authors: 0 as *mut link,
            lockers: 0 as *mut link,
            states: 0 as *mut link,
        };
        init
    };
    let mut insDelFormat: *const i8 = 0 as *const i8;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut a: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut accessListString: *const i8 = 0 as *const i8;
    let mut accessFormat: *const i8 = 0 as *const i8;
    let mut headFormat: *const i8 = 0 as *const i8;
    let mut symbolFormat: *const i8 = 0 as *const i8;
    let mut descflag: bool = false;
    let mut selectflag: bool = false;
    let mut onlylockflag: bool = false;
    let mut onlyRCSflag: bool = false;
    let mut pre5: bool = false;
    let mut shownames: bool = false;
    let mut revno: size_t = 0;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    shownames = 1 as i32 != 0;
    selectflag = shownames;
    descflag = selectflag;
    onlyRCSflag = 0 as i32 != 0;
    onlylockflag = onlyRCSflag;
    out = stdout;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as i32) < argc
            && {
                let fresh3 = a;
                a = a.offset(1);
                *fresh3 as i32 == '-' as i32
            })
        {
            break;
        }
        let mut current_block_27: u64;
        let fresh4 = a;
        a = a.offset(1);
        match *fresh4 as i32 {
            76 => {
                onlylockflag = 1 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            78 => {
                shownames = 0 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            82 => {
                onlyRCSflag = 1 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            108 => {
                lockflag = 1 as i32 != 0;
                getlocker(a, &mut criteria);
                current_block_27 = 7226443171521532240;
            }
            98 => {
                branchflag = 1 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            114 => {
                parse_revpairs(
                    'r' as i32 as i8,
                    a,
                    &mut criteria as *mut criteria as *mut libc::c_void,
                    Some(
                        putrevpairs
                            as unsafe extern "C" fn(
                                *const i8,
                                *const i8,
                                bool,
                                *mut libc::c_void,
                            ) -> (),
                    ),
                );
                current_block_27 = 7226443171521532240;
            }
            100 => {
                getdatepair(a, &mut datesel);
                current_block_27 = 7226443171521532240;
            }
            115 => {
                getstate(a, &mut criteria);
                current_block_27 = 7226443171521532240;
            }
            119 => {
                getauthor(a, &mut criteria);
                current_block_27 = 7226443171521532240;
            }
            104 => {
                descflag = 0 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            116 => {
                selectflag = 0 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            113 => {
                (*top).behavior.quiet = 1 as i32 != 0;
                current_block_27 = 7226443171521532240;
            }
            120 => {
                (*top).behavior.pe = a;
                current_block_27 = 7226443171521532240;
            }
            122 => {
                zone_set(a);
                current_block_27 = 7226443171521532240;
            }
            84 => {
                if *a != 0 {
                    current_block_27 = 14616541514141444294;
                } else {
                    current_block_27 = 7226443171521532240;
                }
            }
            86 => {
                setRCSversion(*argv);
                current_block_27 = 7226443171521532240;
            }
            _ => {
                current_block_27 = 14616541514141444294;
            }
        }
        match current_block_27 {
            14616541514141444294 => {
                bad_option(*argv);
            }
            _ => {}
        }
    }
    if descflag as i32 | selectflag as i32 == 0 {
        generic_warn(0 as *const i8, b"-t overrides -h.\0" as *const u8 as *const i8);
        descflag = 1 as i32 != 0;
    }
    pre5 = (*top).behavior.version < 5 as i32 - 5 as i32;
    if pre5 {
        accessListString = b"\naccess list:   \0" as *const u8 as *const i8;
        accessFormat = b"  %s\0" as *const u8 as *const i8;
        headFormat = b"\nRCS file:        %s;   Working file:    %s\nhead:           %s%s\nbranch:         %s%s\nlocks:         \0"
            as *const u8 as *const i8;
        insDelFormat = b"  lines added/del: %ld/%ld\0" as *const u8 as *const i8;
        symbolFormat = b"  %s: %s;\0" as *const u8 as *const i8;
    } else {
        accessListString = b"\naccess list:\0" as *const u8 as *const i8;
        accessFormat = b"\n\t%s\0" as *const u8 as *const i8;
        headFormat = b"\nRCS file: %s\nWorking file: %s\nhead:%s%s\nbranch:%s%s\nlocks:%s\0"
            as *const u8 as *const i8;
        insDelFormat = b"  lines: +%ld -%ld\0" as *const u8 as *const i8;
        symbolFormat = b"\n\t%s: %s\0" as *const u8 as *const i8;
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus);
    } else if argc < 1 as i32 {
        generic_fatal(0 as *const i8, b"no input file\0" as *const u8 as *const i8);
    } else {
        while (0 as i32) < argc {
            let mut repo_filename: *const i8 = 0 as *const i8;
            let mut tip: *mut delta = 0 as *mut delta;
            let mut defbr: *const i8 = 0 as *const i8;
            let mut strictly_locking: bool = false;
            let mut kws: i32 = 0;
            let mut locks: *mut link = 0 as *mut link;
            ffree();
            if !(pairnames(
                argc,
                argv,
                Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                1 as i32 != 0,
                0 as i32 != 0,
            ) <= 0 as i32)
            {
                repo_filename = (*top).repository.filename;
                tip = (*top).repository.tip;
                defbr = (*(*top).repository.r).branch;
                locks = (*(*top).repository.r).locks;
                strictly_locking = (*top).behavior.strictly_locking;
                kws = (*top).behavior.kws;
                if lockflag {
                    trunclocks(&mut criteria);
                }
                if !(onlylockflag as i32 != 0 && locks.is_null()) {
                    if onlyRCSflag {
                        aprintf(out, b"%s\n\0" as *const u8 as *const i8, repo_filename);
                    } else if getnumericrev(branchflag, &mut criteria) {
                        aprintf(
                            out,
                            headFormat,
                            repo_filename,
                            (*top).manifestation.filename,
                            if !tip.is_null() {
                                b" \0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            if !tip.is_null() {
                                (*tip).num
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            if !defbr.is_null() {
                                b" \0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            if !defbr.is_null() {
                                defbr
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            if strictly_locking as i32 != 0 {
                                b" strict\0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                        );
                        format_locks(out, symbolFormat);
                        if strictly_locking as i32 != 0 && pre5 as i32 != 0 {
                            aputs(
                                (b"  ;  strict\0" as *const u8 as *const i8)
                                    .offset(
                                        (if !locks.is_null() { 3 as i32 } else { 0 as i32 })
                                            as isize,
                                    ),
                                out,
                            );
                        }
                        aputs(accessListString, out);
                        let mut ls: *mut link = (*(*top).repository.r).access;
                        while !ls.is_null() {
                            aprintf(out, accessFormat, (*ls).entry);
                            ls = (*ls).next;
                        }
                        if shownames {
                            aputs(b"\nsymbolic names:\0" as *const u8 as *const i8, out);
                            format_assocs(out, symbolFormat);
                        }
                        if pre5 {
                            aputs(
                                b"\ncomment leader:  \"\0" as *const u8 as *const i8,
                                out,
                            );
                            awrite(
                                (*top).repository.log_lead.string,
                                (*top).repository.log_lead.size,
                                out,
                            );
                            afputc('"' as i32, out);
                        }
                        if !pre5 || kws != kwsub::kwsub_kv as i32 {
                            aprintf(
                                out,
                                b"\nkeyword substitution: %s\0" as *const u8 as *const i8,
                                kwsub_string(kwsub::from_libc_c_uint(kws as u32)),
                            );
                        }
                        aprintf(
                            out,
                            b"\ntotal revisions: %zu\0" as *const u8 as *const i8,
                            (*(*top).repository.r).deltas_count,
                        );
                        revno = 0 as i32 as size_t;
                        if !tip.is_null() && selectflag as i32 & descflag as i32 != 0 {
                            exttree(tip, lockflag, &mut criteria);
                            let mut ls_0: *mut link = datesel.by;
                            while !ls_0.is_null() {
                                let mut incomplete: *const daterange = (*ls_0).entry
                                    as *const daterange;
                                let mut r: *mut daterange = zlloc(
                                    plexus,
                                    (::core::mem::size_of::<daterange>() as u64)
                                        .wrapping_mul(1 as i32 as u64),
                                ) as *mut daterange;
                                *r = *incomplete;
                                memcpy(
                                    ((*r).beg).as_mut_ptr() as *mut libc::c_void,
                                    b"0.0.0.0.0.0\0" as *const u8 as *const i8
                                        as *const libc::c_void,
                                    ::core::mem::size_of::<[i8; 12]>() as u64,
                                );
                                (*ls_0).entry = r as *const libc::c_void;
                                recentdate(tip, r);
                                ls_0 = (*ls_0).next;
                            }
                            revno = extdate(tip, &mut datesel);
                            aprintf(
                                out,
                                b";\tselected revisions: %zu\0" as *const u8 as *const i8,
                                revno,
                            );
                        }
                        newline(out);
                        if descflag {
                            let mut desc: *mut atat = (*(*top).repository.r).desc;
                            aputs(b"description:\n\0" as *const u8 as *const i8, out);
                            atat_display(out, desc, 1 as i32 != 0);
                        }
                        if revno != 0 {
                            putrunk(insDelFormat);
                            putree(tip, insDelFormat);
                        }
                        aputs(equal_line.as_ptr(), out);
                    }
                }
            }
            cleanup(&mut exitstatus);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    Ozclose(&mut out);
    gnurcs_goodbye();
    return exitstatus;
}
static mut rlog_aka: [uint8_t; 10] = [
    2 as i32 as uint8_t,
    3 as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    4 as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'g' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_rlog: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rlog_main as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: rlog_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};