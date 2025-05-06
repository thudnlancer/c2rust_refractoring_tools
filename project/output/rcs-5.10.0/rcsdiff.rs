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
    static prog_diff: [i8; 0];
    static equal_line: [i8; 0];
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    fn str2expmode(s: *const i8) -> i32;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: i32,
        argv: *mut *mut i8,
        rcsopen: Option<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> i32;
    fn delta_from_ref(ref_0: *const i8) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const i8, fp: *mut fro) -> bool;
    fn time2date(unixtime: time_t, date: *mut i8);
    fn zone_set(s: *const i8);
    fn date2str(date: *const i8, datebuf: *mut i8) -> *const i8;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn minus_p(xrev: *const i8, rev: *const i8) -> cbuf;
    fn ffree();
    fn str_save(s: *const i8) -> *mut i8;
    fn runv(infd: i32, outname: *const i8, args: *mut *const i8) -> i32;
    fn setRCSversion(str: *const i8);
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn diagnose(fmt: *const i8, _: ...);
    fn syserror(e: i32, who: *const i8);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut plexus: *mut divvy;
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn maketemp(n: i32) -> *const i8;
    fn tempunlink();
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    static mut peer_super: symdef;
    fn find_peer_prog(prog: *mut symdef) -> *const i8;
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
pub type mode_t = __mode_t;
pub type time_t = __time_t;
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
pub struct symdef {
    pub meaningful: *const i8,
    pub underlying: *const i8,
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
pub struct unique {
    pub eqval_ok: bool,
    pub minlen: size_t,
    pub full: [i8; 0],
}
pub type s_unique = unique;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work {
    pub st: stat,
    pub fro: *mut fro,
}
static mut rcsdiff_blurb: [i8; 23] = unsafe {
    *::core::mem::transmute::<&[u8; 23], &[i8; 23]>(b"Compare RCS revisions.\0")
};
static mut rcsdiff_help: [i8; 1073] = unsafe {
    *::core::mem::transmute::<
        &[u8; 1073],
        &[i8; 1073],
    >(
        b"[options] file ...\nOptions:\n  -rREV         (zero, one, or two times) Name a revision.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -q            Quiet mode.\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nIf given two revisions (-rREV1 -rREV2), compare those revisions.\nIf given only one revision (-rREV), compare the working file with it.\nIf given no revisions, compare the working file with the latest\nrevision on the default branch.\n\nAdditionally, the following options (and their argument, if any) are\npassed to the underlying diff(1) command:\n  -0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -B, -C, -D, -F, -H, -I,\n  -L, -U, -W, -a, -b, -c, -d, -e, -f, -h, -i, -n, -p, -t, -u, -w, -y,\n  [long options (that start with \"--\")].\n(Not all of these options are meaningful.)\n\0",
    )
};
static mut minus_y: s_unique = unsafe {
    {
        let mut init = unique {
            eqval_ok: 0 as i32 != 0,
            minlen: 4 as i32 as size_t,
            full: *::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"--side-by-side\0"),
        };
        init
    }
};
static mut minus_D: s_unique = unsafe {
    {
        let mut init = unique {
            eqval_ok: 1 as i32 != 0,
            minlen: 4 as i32 as size_t,
            full: *::core::mem::transmute::<&[u8; 8], &[i8; 8]>(b"--ifdef\0"),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn longopt_maybe_p(
    mut arg: *const i8,
    mut u: *const s_unique,
) -> bool {
    let mut equal: *const i8 = if (*u).eqval_ok as i32 != 0 {
        strchr(arg, '=' as i32)
    } else {
        0 as *mut i8
    };
    let mut len: size_t = if !equal.is_null() {
        equal.offset_from(arg) as i64 as size_t
    } else {
        strlen(arg)
    };
    return !((*u).minlen > len) && 0 as i32 == strncmp(arg, ((*u).full).as_ptr(), len);
}
unsafe extern "C" fn cleanup(mut exitstatus: *mut i32, mut work: *mut work) {
    if (*top).flow.erroneous {
        *exitstatus = 2 as i32;
    }
    fro_zclose(&mut (*top).flow.from);
    fro_zclose(&mut (*work).fro);
}
unsafe extern "C" fn setup_label(mut num: *const i8, mut date: *const i8) -> *const i8 {
    let mut len: size_t = 0;
    let mut datestr: [i8; 31] = [0; 31];
    date2str(date, datestr.as_mut_ptr());
    accf(
        plexus,
        b"--label=%s\t%s\0" as *const u8 as *const i8,
        (*top).manifestation.filename,
        datestr.as_mut_ptr(),
    );
    if !num.is_null() {
        accf(plexus, b"\t%s\0" as *const u8 as *const i8, num);
    }
    return finish_string(plexus, &mut len);
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: rcsdiff_blurb.as_ptr(),
            help: rcsdiff_help.as_ptr(),
            tyag: (1 as i32) << 1 as i32 | (1 as i32) << 0 as i32,
        };
        init
    }
};
unsafe extern "C" fn rcsdiff_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut exitstatus: i32 = 0 as i32;
    let mut work: work = work {
        st: stat {
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
        fro: 0 as *mut fro,
    };
    let mut revnums: i32 = 0;
    let mut rev1: *const i8 = 0 as *const i8;
    let mut rev2: *const i8 = 0 as *const i8;
    let mut xrev1: *const i8 = 0 as *const i8;
    let mut xrev2: *const i8 = 0 as *const i8;
    let mut expandarg: *const i8 = 0 as *const i8;
    let mut lexpandarg: *const i8 = 0 as *const i8;
    let mut suffixarg: *const i8 = 0 as *const i8;
    let mut versionarg: *const i8 = 0 as *const i8;
    let mut zonearg: *const i8 = 0 as *const i8;
    let mut file_labels: i32 = 0;
    let mut diff_label1: *mut *const i8 = 0 as *mut *const i8;
    let mut diff_label2: *mut *const i8 = 0 as *mut *const i8;
    let mut date2: [i8; 22] = [0; 22];
    let mut cov: [*const i8; 11] = [0 as *const i8; 11];
    let mut diffv: *mut *const i8 = 0 as *mut *const i8;
    let mut diffp: *mut *const i8 = 0 as *mut *const i8;
    let mut diffpend: *mut *const i8 = 0 as *mut *const i8;
    let mut pp: *mut *const i8 = 0 as *mut *const i8;
    let mut diffvstr: *const i8 = 0 as *const i8;
    let mut commarg: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut a: *mut i8 = 0 as *mut i8;
    let mut dcp: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut no_diff_means_no_output: bool = false;
    let mut c: i32 = 0;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut work as *mut work as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<work>() as u64,
    );
    exitstatus = 0 as i32;
    revnums = 0 as i32;
    xrev2 = 0 as *const i8;
    rev2 = xrev2;
    rev1 = rev2;
    file_labels = 0 as i32;
    zonearg = 0 as *const i8;
    versionarg = zonearg;
    suffixarg = versionarg;
    expandarg = suffixarg;
    no_diff_means_no_output = 1 as i32 != 0;
    diffv = pointer_array(
        plexus,
        (1 as i32 + argc + (0 as i32 != 0) as i32 + 2 as i32 * 1 as i32 + 2 as i32)
            as size_t,
    ) as *mut *const i8;
    diffp = diffv.offset(1 as i32 as isize);
    let fresh0 = diffp;
    diffp = diffp.offset(1);
    *fresh0 = prog_diff.as_ptr();
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as i32) < argc
            && {
                let fresh1 = a;
                a = a.offset(1);
                *fresh1 as i32 == '-' as i32
            })
        {
            break;
        }
        dcp = a;
        loop {
            let fresh2 = a;
            a = a.offset(1);
            c = *fresh2 as i32;
            if !(c != 0) {
                break;
            }
            match c {
                114 => {
                    revnums += 1;
                    match revnums {
                        1 => {
                            rev1 = a;
                        }
                        2 => {
                            rev2 = a;
                        }
                        _ => {
                            generic_error(
                                0 as *const i8,
                                b"too many %ss\0" as *const u8 as *const i8,
                                ks_revno.as_ptr(),
                            );
                        }
                    }
                    break;
                }
                45 | 68 => {
                    if 'D' as i32 == c || longopt_maybe_p(*argv, &minus_D) as i32 != 0
                        || longopt_maybe_p(*argv, &minus_y) as i32 != 0
                    {
                        no_diff_means_no_output = 0 as i32 != 0;
                    }
                    current_block = 109591404199396;
                }
                67 | 70 | 73 | 76 | 85 | 87 => {
                    current_block = 109591404199396;
                }
                121 => {
                    no_diff_means_no_output = 0 as i32 != 0;
                    current_block = 1484704610631084712;
                }
                66 | 72 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99
                | 100 | 101 | 102 | 104 | 105 | 110 | 112 | 116 | 117 | 119 => {
                    current_block = 1484704610631084712;
                }
                113 => {
                    (*top).behavior.quiet = 1 as i32 != 0;
                    continue;
                }
                120 => {
                    suffixarg = *argv;
                    (*top).behavior.pe = (*argv).offset(2 as i32 as isize);
                    break;
                }
                122 => {
                    zonearg = *argv;
                    zone_set((*argv).offset(2 as i32 as isize));
                    break;
                }
                84 => {
                    if !(*a != 0) {
                        continue;
                    }
                    current_block = 12815801928638373013;
                }
                86 => {
                    versionarg = *argv;
                    setRCSversion(versionarg);
                    break;
                }
                107 => {
                    expandarg = *argv;
                    if 0 as i32 <= str2expmode(expandarg.offset(2 as i32 as isize)) {
                        break;
                    }
                    current_block = 12815801928638373013;
                }
                _ => {
                    current_block = 12815801928638373013;
                }
            }
            match current_block {
                12815801928638373013 => {
                    bad_option(*argv);
                }
                1484704610631084712 => {
                    let fresh8 = dcp;
                    dcp = dcp.offset(1);
                    *fresh8 = c as i8;
                }
                _ => {
                    if 1 as i32 != 0 && c == 'L' as i32
                        && {
                            file_labels += 1;
                            file_labels == 2 as i32
                        }
                    {
                        generic_fatal(
                            0 as *const i8,
                            b"too many -L options\0" as *const u8 as *const i8,
                        );
                    }
                    let fresh3 = dcp;
                    dcp = dcp.offset(1);
                    *fresh3 = c as i8;
                    if *a != 0 {
                        loop {
                            let fresh4 = a;
                            a = a.offset(1);
                            let fresh5 = dcp;
                            dcp = dcp.offset(1);
                            *fresh5 = *fresh4;
                            if !(*a != 0) {
                                break;
                            }
                        }
                    } else {
                        argc -= 1;
                        if argc == 0 {
                            generic_fatal(
                                0 as *const i8,
                                b"-%c needs following argument\0" as *const u8 as *const i8,
                                c,
                            );
                        }
                        let fresh6 = argv;
                        argv = argv.offset(1);
                        let fresh7 = diffp;
                        diffp = diffp.offset(1);
                        *fresh7 = *fresh6;
                    }
                }
            }
        }
        if dcp != (*argv).offset(1 as i32 as isize) {
            *dcp = '\0' as i32 as i8;
            let fresh9 = diffp;
            diffp = diffp.offset(1);
            *fresh9 = *argv;
        }
    }
    if !(*top).behavior.quiet {
        let mut len: size_t = 0;
        pp = diffv.offset(2 as i32 as isize);
        while pp < diffp {
            let fresh10 = pp;
            pp = pp.offset(1);
            accf(plexus, b" %s\0" as *const u8 as *const i8, *fresh10);
        }
        diffvstr = finish_string(plexus, &mut len);
    }
    diff_label2 = 0 as *mut *const i8;
    diff_label1 = diff_label2;
    if file_labels < 2 as i32 {
        if file_labels == 0 {
            let fresh11 = diffp;
            diffp = diffp.offset(1);
            diff_label1 = fresh11;
        }
        let fresh12 = diffp;
        diffp = diffp.offset(1);
        diff_label2 = fresh12;
    }
    diffpend = diffp;
    cov[1 as i32 as usize] = find_peer_prog(&mut peer_super);
    cov[2 as i32 as usize] = b"co\0" as *const u8 as *const i8;
    cov[3 as i32 as usize] = b"-q\0" as *const u8 as *const i8;
    if 1 as i32 == 0 {
        cov[(4 as i32 + (1 as i32 == 0) as i32 - 1 as i32) as usize] = b"-M\0"
            as *const u8 as *const i8;
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut work);
    } else if argc < 1 as i32 {
        generic_fatal(0 as *const i8, b"no input file\0" as *const u8 as *const i8);
    } else {
        let mut current_block_134: u64;
        while (0 as i32) < argc {
            let mut numericrev: cbuf = cbuf {
                string: 0 as *const i8,
                size: 0,
            };
            let mut tip: *mut delta = 0 as *mut delta;
            let mut mani_filename: *const i8 = 0 as *const i8;
            let mut defbr: *const i8 = 0 as *const i8;
            let mut kws: i32 = 0;
            ffree();
            if !(pairnames(
                argc,
                argv,
                Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                1 as i32 != 0,
                0 as i32 != 0,
            ) <= 0 as i32)
            {
                tip = (*top).repository.tip;
                mani_filename = (*top).manifestation.filename;
                kws = (*top).behavior.kws;
                defbr = (*(*top).repository.r).branch;
                diagnose(
                    b"%sRCS file: %s\0" as *const u8 as *const i8,
                    equal_line.as_ptr().offset(10 as i32 as isize),
                    (*top).repository.filename,
                );
                if rev2.is_null() {
                    work.fro = fro_open(
                        mani_filename,
                        b"r\0" as *const u8 as *const i8,
                        &mut work.st,
                    );
                    if (work.fro).is_null() {
                        syserror(*__errno_location(), mani_filename);
                        current_block_134 = 9505035279996566320;
                    } else {
                        current_block_134 = 1765866445182206997;
                    }
                } else {
                    current_block_134 = 1765866445182206997;
                }
                match current_block_134 {
                    9505035279996566320 => {}
                    _ => {
                        if tip.is_null() {
                            generic_error(
                                (*top).repository.filename,
                                b"no revisions present\0" as *const u8 as *const i8,
                            );
                        } else {
                            if revnums == 0 as i32 || *rev1 == 0 {
                                rev1 = if !defbr.is_null() { defbr } else { (*tip).num };
                            }
                            if fully_numeric(&mut numericrev, rev1, work.fro) {
                                target = delta_from_ref(numericrev.string);
                                if !target.is_null() {
                                    xrev1 = (*target).num;
                                    if 1 as i32 != 0 && !diff_label1.is_null() {
                                        *diff_label1 = setup_label((*target).num, (*target).date);
                                    }
                                    lexpandarg = expandarg;
                                    if revnums == 2 as i32 {
                                        if !fully_numeric(
                                            &mut numericrev,
                                            if *rev2 as i32 != 0 {
                                                rev2
                                            } else if !defbr.is_null() {
                                                defbr
                                            } else {
                                                (*tip).num
                                            },
                                            work.fro,
                                        ) {
                                            current_block_134 = 9505035279996566320;
                                        } else {
                                            target = delta_from_ref(numericrev.string);
                                            if target.is_null() {
                                                current_block_134 = 9505035279996566320;
                                            } else {
                                                xrev2 = (*target).num;
                                                if no_diff_means_no_output as i32 != 0 && xrev1 == xrev2 {
                                                    current_block_134 = 9505035279996566320;
                                                } else {
                                                    current_block_134 = 8602574157404971894;
                                                }
                                            }
                                        }
                                    } else {
                                        if !((*target).lockedby).is_null() && lexpandarg.is_null()
                                            && kws == kwsub::kwsub_kv as i32
                                            && (*top).repository.stat.st_mode
                                                & !(0o200 as i32 | 0o200 as i32 >> 3 as i32
                                                    | 0o200 as i32 >> 3 as i32 >> 3 as i32) as mode_t
                                                | (if 1 as i32 != 0 { 0o200 as i32 } else { 0 as i32 })
                                                    as u32 == work.st.st_mode
                                        {
                                            lexpandarg = b"-kkvl\0" as *const u8 as *const i8;
                                        }
                                        current_block_134 = 8602574157404971894;
                                    }
                                    match current_block_134 {
                                        9505035279996566320 => {}
                                        _ => {
                                            fro_zclose(&mut work.fro);
                                            if 1 as i32 != 0 && !diff_label2.is_null() {
                                                if revnums == 2 as i32 {
                                                    *diff_label2 = setup_label((*target).num, (*target).date);
                                                } else {
                                                    time2date(work.st.st_mtim.tv_sec, date2.as_mut_ptr());
                                                    *diff_label2 = setup_label(
                                                        0 as *const i8,
                                                        date2.as_mut_ptr() as *const i8,
                                                    );
                                                }
                                            }
                                            commarg = minus_p(xrev1, rev1);
                                            pp = &mut *cov
                                                .as_mut_ptr()
                                                .offset((4 as i32 + (1 as i32 == 0) as i32) as isize)
                                                as *mut *const i8;
                                            let fresh13 = pp;
                                            pp = pp.offset(1);
                                            *fresh13 = commarg.string;
                                            if !lexpandarg.is_null() {
                                                let fresh14 = pp;
                                                pp = pp.offset(1);
                                                *fresh14 = lexpandarg;
                                            }
                                            if !suffixarg.is_null() {
                                                let fresh15 = pp;
                                                pp = pp.offset(1);
                                                *fresh15 = suffixarg;
                                            }
                                            if !versionarg.is_null() {
                                                let fresh16 = pp;
                                                pp = pp.offset(1);
                                                *fresh16 = versionarg;
                                            }
                                            if !zonearg.is_null() {
                                                let fresh17 = pp;
                                                pp = pp.offset(1);
                                                *fresh17 = zonearg;
                                            }
                                            let fresh18 = pp;
                                            pp = pp.offset(1);
                                            *fresh18 = (*top).repository.filename;
                                            *pp = 0 as *const i8;
                                            diffp = diffpend;
                                            if 0 as i32 != 0 && kws == kwsub::kwsub_b as i32 {
                                                let fresh19 = diffp;
                                                diffp = diffp.offset(1);
                                                *fresh19 = b"--binary\0" as *const u8 as *const i8;
                                            }
                                            let ref mut fresh20 = *diffp.offset(0 as i32 as isize);
                                            *fresh20 = maketemp(0 as i32);
                                            if runv(
                                                -(1 as i32),
                                                *diffp.offset(0 as i32 as isize),
                                                cov.as_mut_ptr(),
                                            ) != 0
                                            {
                                                generic_error(
                                                    (*top).repository.filename,
                                                    b"co failed\0" as *const u8 as *const i8,
                                                );
                                            } else {
                                                if rev2.is_null() {
                                                    let ref mut fresh21 = *diffp.offset(1 as i32 as isize);
                                                    *fresh21 = mani_filename;
                                                    if *mani_filename as i32 == '-' as i32 {
                                                        accf(
                                                            plexus,
                                                            b".%c\0" as *const u8 as *const i8,
                                                            '/' as i32,
                                                        );
                                                        let ref mut fresh22 = *diffp.offset(1 as i32 as isize);
                                                        *fresh22 = str_save(mani_filename);
                                                    }
                                                    current_block_134 = 1830138855519935310;
                                                } else {
                                                    commarg = minus_p(xrev2, rev2);
                                                    cov[(4 as i32 + (1 as i32 == 0) as i32) as usize] = commarg
                                                        .string;
                                                    let ref mut fresh23 = *diffp.offset(1 as i32 as isize);
                                                    *fresh23 = maketemp(1 as i32);
                                                    if runv(
                                                        -(1 as i32),
                                                        *diffp.offset(1 as i32 as isize),
                                                        cov.as_mut_ptr(),
                                                    ) != 0
                                                    {
                                                        generic_error(
                                                            (*top).repository.filename,
                                                            b"co failed\0" as *const u8 as *const i8,
                                                        );
                                                        current_block_134 = 9505035279996566320;
                                                    } else {
                                                        current_block_134 = 1830138855519935310;
                                                    }
                                                }
                                                match current_block_134 {
                                                    9505035279996566320 => {}
                                                    _ => {
                                                        if rev2.is_null() {
                                                            diagnose(
                                                                b"diff%s -r%s %s\0" as *const u8 as *const i8,
                                                                diffvstr,
                                                                xrev1,
                                                                mani_filename,
                                                            );
                                                        } else {
                                                            diagnose(
                                                                b"diff%s -r%s -r%s\0" as *const u8 as *const i8,
                                                                diffvstr,
                                                                xrev1,
                                                                xrev2,
                                                            );
                                                        }
                                                        let ref mut fresh24 = *diffp.offset(2 as i32 as isize);
                                                        *fresh24 = 0 as *const i8;
                                                        let mut s: i32 = runv(-(1 as i32), 0 as *const i8, diffv);
                                                        if 2 as i32 == s {
                                                            generic_error(
                                                                (*top).manifestation.filename,
                                                                b"diff failed\0" as *const u8 as *const i8,
                                                            );
                                                        }
                                                        if 1 as i32 == s && 0 as i32 == exitstatus {
                                                            exitstatus = s;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            cleanup(&mut exitstatus, &mut work);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    gnurcs_goodbye();
    return exitstatus;
}
static mut rcsdiff_aka: [uint8_t; 14] = [
    2 as i32 as uint8_t,
    4 as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    7 as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'f' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_rcsdiff: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsdiff_main as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: rcsdiff_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};