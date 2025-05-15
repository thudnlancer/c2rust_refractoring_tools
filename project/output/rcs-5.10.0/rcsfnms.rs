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
    static mut top: *mut top;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn cgetenv(name: *const i8) -> *mut i8;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strcasecmp(_: *const i8, _: *const i8) -> i32;
    fn __errno_location() -> *mut i32;
    fn getcwd(__buf: *mut i8, __size: size_t) -> *mut i8;
    fn syserror(e: i32, who: *const i8);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_sys(who: *const i8);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn make_space(name: *const i8) -> *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn close_space(divvy: *mut divvy);
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn intern(divvy: *mut divvy, s: *const i8, len: size_t) -> *mut i8;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn accumulate_nbytes(divvy: *mut divvy, start: *const i8, count: size_t);
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn empty_repo(to: *mut divvy) -> *mut repo;
    fn grok_all(to: *mut divvy, f: *mut fro) -> *mut repo;
}
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
pub struct compair {
    pub suffix: *const i8,
    pub comlead: *const i8,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut comtable: [compair; 39] = [
    {
        let mut init = compair {
            suffix: b"a\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ada\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"adb\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ads\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"asm\0" as *const u8 as *const i8,
            comlead: b";; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"bat\0" as *const u8 as *const i8,
            comlead: b":: \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"body\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"c\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"c++\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cc\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cpp\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cxx\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cl\0" as *const u8 as *const i8,
            comlead: b";;; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cmd\0" as *const u8 as *const i8,
            comlead: b":: \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cmf\0" as *const u8 as *const i8,
            comlead: b"c \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"cs\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"el\0" as *const u8 as *const i8,
            comlead: b"; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"f\0" as *const u8 as *const i8,
            comlead: b"c \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"for\0" as *const u8 as *const i8,
            comlead: b"c \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"h\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"hpp\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"hxx\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"l\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"lisp\0" as *const u8 as *const i8,
            comlead: b";;; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"lsp\0" as *const u8 as *const i8,
            comlead: b";; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"m\0" as *const u8 as *const i8,
            comlead: b"// \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"mac\0" as *const u8 as *const i8,
            comlead: b";; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"me\0" as *const u8 as *const i8,
            comlead: b".\\\" \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ml\0" as *const u8 as *const i8,
            comlead: b"; \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"mm\0" as *const u8 as *const i8,
            comlead: b".\\\" \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ms\0" as *const u8 as *const i8,
            comlead: b".\\\" \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"p\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"pas\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"ps\0" as *const u8 as *const i8,
            comlead: b"% \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"spec\0" as *const u8 as *const i8,
            comlead: b"-- \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"sty\0" as *const u8 as *const i8,
            comlead: b"% \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"tex\0" as *const u8 as *const i8,
            comlead: b"% \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: b"y\0" as *const u8 as *const i8,
            comlead: b" * \0" as *const u8 as *const i8,
        };
        init
    },
    {
        let mut init = compair {
            suffix: 0 as *const i8,
            comlead: b"# \0" as *const u8 as *const i8,
        };
        init
    },
];
unsafe extern "C" fn InitAdmin() {
    let mut ext: *const i8 = 0 as *const i8;
    (*top).repository.tip = 0 as *mut delta;
    (*top).behavior.strictly_locking = 1 as i32 != 0;
    (*top).repository.r = empty_repo(single);
    ext = strrchr((*top).manifestation.filename, '.' as i32);
    ext = if !ext.is_null() {
        ext.offset(1 as i32 as isize)
    } else {
        b"\0" as *const u8 as *const i8
    };
    let mut ent: *const compair = comtable.as_ptr();
    loop {
        if ((*ent).suffix).is_null() || strcasecmp((*ent).suffix, ext) == 0 {
            (*top).repository.log_lead.string = (*ent).comlead;
            (*top).repository.log_lead.size = strlen((*ent).comlead);
            break;
        } else {
            ent = ent.offset(1);
            ent;
        }
    }
    (*top).behavior.kws = kwsub::kwsub_kv as i32;
}
#[no_mangle]
pub unsafe extern "C" fn basefilename(mut p: *const i8) -> *const i8 {
    let mut b: *const i8 = p;
    let mut q: *const i8 = p;
    loop {
        let fresh0 = q;
        q = q.offset(1);
        match *fresh0 as i32 {
            47 => {
                b = q;
            }
            0 => return b,
            _ => {}
        }
    };
}
unsafe extern "C" fn suffixlen(mut x: *const i8) -> size_t {
    let mut p: *const i8 = 0 as *const i8;
    p = x;
    loop {
        match *p as i32 {
            0 | 47 => return p.offset_from(x) as i64 as size_t,
            _ => {
                p = p.offset(1);
                p;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rcssuffix(mut name: *const i8) -> *const i8 {
    let mut x: *const i8 = 0 as *const i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut nz: *const i8 = 0 as *const i8;
    let mut nl: size_t = 0;
    let mut xl: size_t = 0;
    nl = strlen(name);
    nz = name.offset(nl as isize);
    x = (*top).behavior.pe;
    loop {
        xl = suffixlen(x);
        if xl != 0 {
            if xl <= nl
                && {
                    p = nz.offset(-(xl as isize));
                    0 as i32
                        == memcmp(p as *const libc::c_void, x as *const libc::c_void, xl)
                }
            {
                return p;
            }
        } else {
            p = name;
            while p
                < nz
                    .offset(
                        -((::core::mem::size_of::<[i8; 4]>() as u64)
                            .wrapping_sub(1 as i32 as u64) as isize),
                    )
            {
                if isSLASH(
                    *p
                        .offset(
                            (::core::mem::size_of::<[i8; 4]>() as u64)
                                .wrapping_sub(1 as i32 as u64) as isize,
                        ) as i32,
                ) as i32 != 0
                    && (p == name
                        || isSLASH(*p.offset(-(1 as i32) as isize) as i32) as i32 != 0)
                    && 0 as i32
                        == memcmp(
                            p as *const libc::c_void,
                            b"RCS\0" as *const u8 as *const i8 as *const libc::c_void,
                            (::core::mem::size_of::<[i8; 4]>() as u64)
                                .wrapping_sub(1 as i32 as u64),
                        )
                {
                    return nz;
                }
                p = p.offset(1);
                p;
            }
        }
        x = x.offset(xl as isize);
        let fresh1 = x;
        x = x.offset(1);
        if !(*fresh1 != 0) {
            break;
        }
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn rcsreadopen(mut m: *mut maybe) -> *mut fro {
    return fro_open(
        (*m).tentative.string,
        b"r\0" as *const u8 as *const i8,
        (*m).status,
    );
}
unsafe extern "C" fn finopen(mut m: *mut maybe) -> bool {
    let mut interesting: bool = false;
    let mut preferold: bool = false;
    preferold = *((*m).bestfit.string).offset(0 as i32 as isize) as i32 != 0
        && ((*m).mustread as i32 != 0 || 0 as i32 <= (*top).repository.fd_lock);
    (*top).flow.from = ((*m).open).expect("non-null function pointer")(m);
    interesting = !((*top).flow.from).is_null() || *__errno_location() != 2 as i32;
    if interesting as i32 != 0 || !preferold {
        (*m).eno = *__errno_location();
        (*m).bestfit = (*m).tentative;
    }
    return interesting;
}
unsafe extern "C" fn fin2open(
    mut d: *const i8,
    mut dlen: size_t,
    mut base: *const i8,
    mut baselen: size_t,
    mut x: *const i8,
    mut xlen: size_t,
    mut m: *mut maybe,
) -> bool {
    accumulate_nbytes((*m).space, d, dlen);
    accumulate_nbytes(
        (*m).space,
        b"RCS\0" as *const u8 as *const i8,
        (::core::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1 as i32 as u64),
    );
    accumulate_byte((*m).space, '/' as i32);
    accumulate_nbytes((*m).space, base, baselen);
    accumulate_nbytes((*m).space, x, xlen);
    (*m).tentative.string = finish_string((*m).space, &mut (*m).tentative.size);
    if xlen != 0 {
        if finopen(m) {
            return 1 as i32 != 0;
        }
        accumulate_nbytes((*m).space, d, dlen);
        accumulate_nbytes((*m).space, base, baselen);
        accumulate_nbytes((*m).space, x, xlen);
        (*m).tentative.string = finish_string((*m).space, &mut (*m).tentative.size);
    }
    return finopen(m);
}
#[no_mangle]
pub unsafe extern "C" fn pairnames(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut rcsopen: Option<open_rcsfile_fn>,
    mut mustread: bool,
    mut quiet: bool,
) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut arg: *mut i8 = 0 as *mut i8;
    let mut RCS1: *mut i8 = 0 as *mut i8;
    let mut base: *const i8 = 0 as *const i8;
    let mut RCSbase: *const i8 = 0 as *const i8;
    let mut x: *const i8 = 0 as *const i8;
    let mut mani_filename: *mut i8 = 0 as *mut i8;
    let mut paired: bool = false;
    let mut arglen: size_t = 0;
    let mut dlen: size_t = 0;
    let mut baselen: size_t = 0;
    let mut xlen: size_t = 0;
    let mut from: *mut fro = 0 as *mut fro;
    let mut maybe: maybe = {
        let mut init = maybe {
            open: rcsopen,
            mustread: mustread,
            tentative: cbuf {
                string: 0 as *const i8,
                size: 0,
            },
            space: 0 as *mut divvy,
            bestfit: cbuf {
                string: 0 as *const i8,
                size: 0,
            },
            status: &mut (*top).repository.stat,
            eno: 0,
        };
        init
    };
    (*top).repository.fd_lock = -(1 as i32);
    arg = *argv;
    if arg.is_null() {
        return 0 as i32;
    }
    if *arg as i32 == '-' as i32 {
        generic_error(
            0 as *const i8,
            b"%s option is ignored after filenames\0" as *const u8 as *const i8,
            arg,
        );
        return 0 as i32;
    }
    base = basefilename(arg);
    paired = 0 as i32 != 0;
    x = rcssuffix(arg);
    if !x.is_null() {
        RCS1 = arg;
        RCSbase = base;
        baselen = x.offset_from(base) as i64 as size_t;
        if (1 as i32) < argc
            && {
                p = *argv.offset(1 as i32 as isize);
                mani_filename = p;
                (rcssuffix(mani_filename)).is_null()
            }
            && {
                arglen = strlen(p);
                baselen <= arglen
            }
            && {
                p = p.offset(arglen.wrapping_sub(baselen) as isize);
                p == mani_filename
                    || isSLASH(*p.offset(-(1 as i32) as isize) as i32) as i32 != 0
            }
            && 0 as i32
                == memcmp(base as *const libc::c_void, p as *const libc::c_void, baselen)
        {
            let ref mut fresh2 = *argv.offset(1 as i32 as isize);
            *fresh2 = 0 as *mut i8;
            paired = 1 as i32 != 0;
        } else {
            mani_filename = intern(single, base, baselen.wrapping_add(1 as i32 as u64));
            *mani_filename.offset(baselen as isize) = '\0' as i32 as i8;
        }
    } else {
        mani_filename = arg;
        baselen = strlen(base);
        if (1 as i32) < argc
            && {
                RCS1 = *argv.offset(1 as i32 as isize);
                x = rcssuffix(RCS1);
                !x.is_null()
            } && RCS1.offset(baselen as isize) <= x as *mut i8
            && {
                RCSbase = x.offset(-(baselen as isize));
                RCSbase == RCS1
                    || isSLASH(*RCSbase.offset(-(1 as i32) as isize) as i32) as i32 != 0
            }
            && 0 as i32
                == memcmp(
                    base as *const libc::c_void,
                    RCSbase as *const libc::c_void,
                    baselen,
                )
        {
            let ref mut fresh3 = *argv.offset(1 as i32 as isize);
            *fresh3 = 0 as *mut i8;
            paired = 1 as i32 != 0;
        } else {
            RCS1 = 0 as *mut i8;
            RCSbase = RCS1;
        }
    }
    (*top).manifestation.filename = mani_filename;
    maybe.space = make_space(
        (*::core::mem::transmute::<&[u8; 10], &[i8; 10]>(b"pairnames\0")).as_ptr(),
    );
    if RCSbase != RCS1 {
        maybe.bestfit.string = RCS1;
        maybe.bestfit.size = strlen(RCS1);
        maybe.tentative = maybe.bestfit;
        (*top).flow.from = (Some(rcsopen.expect("non-null function pointer")))
            .expect("non-null function pointer")(&mut maybe);
        maybe.eno = *__errno_location();
    } else {
        maybe.bestfit.string = b"\0" as *const u8 as *const i8;
        maybe.bestfit.size = 0 as i32 as size_t;
        if !RCS1.is_null() {
            fin2open(
                arg,
                0 as i32 as size_t,
                RCSbase,
                baselen,
                x,
                strlen(x),
                &mut maybe,
            );
        } else {
            dlen = base.offset_from(arg) as i64 as size_t;
            x = (*top).behavior.pe;
            loop {
                xlen = suffixlen(x);
                if fin2open(arg, dlen, base, baselen, x, xlen, &mut maybe) {
                    break;
                }
                x = x.offset(xlen as isize);
                let fresh4 = x;
                x = x.offset(1);
                if *fresh4 == 0 {
                    break;
                }
            }
        }
    }
    p = intern(single, maybe.bestfit.string, maybe.bestfit.size);
    (*top).repository.filename = p;
    (*top).flow.erroneous = 0 as i32 != 0;
    (*top).behavior.Oerrloop = 0 as i32 != 0;
    from = (*top).flow.from;
    if !from.is_null() {
        if !((*maybe.status).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32)
        {
            generic_error(
                0 as *const i8,
                b"%s isn't a regular file -- ignored\0" as *const u8 as *const i8,
                p,
            );
            return 0 as i32;
        }
        (*top).repository.r = grok_all(single, from);
        (*top).flow.to = 0 as *mut FILE;
    } else {
        if maybe.eno != 2 as i32 || mustread as i32 != 0
            || 0 as i32 > (*top).repository.fd_lock
        {
            if maybe.eno == 17 as i32 {
                generic_error(
                    0 as *const i8,
                    b"RCS file %s is in use\0" as *const u8 as *const i8,
                    p,
                );
            } else if !quiet || maybe.eno != 2 as i32 {
                syserror(maybe.eno, p);
            }
            return 0 as i32;
        }
        InitAdmin();
    }
    if paired as i32 != 0 && !((*top).manifestation.standard_output).is_null() {
        generic_warn(
            (*top).manifestation.filename,
            b"Working file ignored due to -p option\0" as *const u8 as *const i8,
        );
    }
    (*top).manifestation.prev.valid = 0 as i32 != 0;
    close_space(maybe.space);
    return if !from.is_null() { 1 as i32 } else { -(1 as i32) };
}
unsafe extern "C" fn dir_useful_len(mut d: *const i8) -> size_t {
    let mut dlen: size_t = strlen(d);
    if 0 as i32 != 0 && dlen == 2 as i32 as u64
        && isSLASH(*d.offset(0 as i32 as isize) as i32) as i32 != 0
        && isSLASH(*d.offset(1 as i32 as isize) as i32) as i32 != 0
    {
        dlen = dlen.wrapping_sub(1);
        dlen;
    } else {
        while dlen != 0
            && isSLASH(*d.offset(dlen.wrapping_sub(1 as i32 as u64) as isize) as i32)
                as i32 != 0
        {
            dlen = dlen.wrapping_sub(1);
            dlen;
        }
    }
    return dlen;
}
#[no_mangle]
pub unsafe extern "C" fn getfullRCSname() -> *const i8 {
    let mut r: *const i8 = (*top).repository.filename;
    if isSLASH(*r.offset(0 as i32 as isize) as i32) {
        return r
    } else {
        let mut cwd: *mut i8 = 0 as *mut i8;
        let mut rv: *mut i8 = 0 as *mut i8;
        let mut len: size_t = 0;
        cwd = (*top).behavior.cwd;
        if cwd.is_null() {
            let mut PWD: *mut i8 = cgetenv(b"PWD\0" as *const u8 as *const i8);
            let mut PWDstat: stat = stat {
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
            let mut dotstat: stat = stat {
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
            cwd = PWD;
            if !(!cwd.is_null()
                && isSLASH(*PWD.offset(0 as i32 as isize) as i32) as i32 != 0
                && !(0 as i32 > stat(PWD, &mut PWDstat))
                && !(0 as i32 > stat(b".\0" as *const u8 as *const i8, &mut dotstat))
                && (PWDstat.st_ino == dotstat.st_ino
                    && PWDstat.st_dev == dotstat.st_dev))
            {
                let mut sz: size_t = 64 as i32 as size_t;
                loop {
                    cwd = alloc(plexus, sz) as *mut i8;
                    if !(getcwd(cwd, sz)).is_null() {
                        break;
                    }
                    brush_off(plexus, cwd as *mut libc::c_void);
                    if *__errno_location() == 34 as i32 {
                        sz <<= 1 as i32;
                    } else {
                        cwd = PWD;
                        if !cwd.is_null() {
                            break;
                        }
                        fatal_sys(b"getcwd\0" as *const u8 as *const i8);
                    }
                }
            }
            *cwd.offset(dir_useful_len(cwd) as isize) = '\0' as i32 as i8;
            (*top).behavior.cwd = cwd;
        }
        while *r.offset(0 as i32 as isize) as i32 == '.' as i32
            && isSLASH(*r.offset(1 as i32 as isize) as i32) as i32 != 0
        {
            while isSLASH(*r.offset(2 as i32 as isize) as i32) {
                r = r.offset(1);
                r;
            }
            r = r.offset(2 as i32 as isize);
        }
        accf(single, b"%s%c%s\0" as *const u8 as *const i8, cwd, '/' as i32, r);
        rv = finish_string(single, &mut len);
        return rv;
    };
}
#[no_mangle]
pub unsafe extern "C" fn isSLASH(mut c: i32) -> bool {
    if 0 as i32 == 0 {
        return '/' as i32 == c;
    }
    match c {
        47 => return 1 as i32 != 0,
        _ => return 0 as i32 != 0,
    };
}