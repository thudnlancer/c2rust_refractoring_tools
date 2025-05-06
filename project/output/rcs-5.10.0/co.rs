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
    static mut top: *mut top;
    static mut stdout: *mut _IO_FILE;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: i32,
        argv: *mut *mut i8,
        rcsopen: Option<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn str2expmode(s: *const i8) -> i32;
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn chnamemod(
        fromp: *mut *mut FILE,
        from: *const i8,
        to: *const i8,
        set_mode: i32,
        mode: mode_t,
        mtime: timespec,
    ) -> i32;
    fn findlock(delete: bool, target: *mut *mut delta) -> i32;
    fn checkaccesslist() -> bool;
    fn dorewrite(lockflag: bool, changed: i32) -> i32;
    fn donerewrite(changed: i32, mtime: timespec) -> i32;
    fn ORCSclose();
    fn buildrevision(
        deltas: *const wlink,
        target: *mut delta,
        outfile: *mut FILE,
        expandflag: bool,
    ) -> *const i8;
    fn ttystdin() -> bool;
    fn yesorno(default_answer: bool, question: *const i8, _: ...) -> bool;
    fn write_desc_maybe(to: *mut FILE);
    fn countnumflds(s: *const i8) -> i32;
    fn take(count: size_t, ref_0: *const i8) -> cbuf;
    fn cmpnum(num1: *const i8, num2: *const i8) -> i32;
    fn cmpnumfld(num1: *const i8, num2: *const i8, fld: i32) -> i32;
    fn genrevs(
        revno: *const i8,
        date: *const i8,
        author: *const i8,
        state: *const i8,
        store: *mut *mut wlink,
    ) -> *mut delta;
    fn delta_from_ref(ref_0: *const i8) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const i8, fp: *mut fro) -> bool;
    fn namedrev(name: *const i8, delta: *mut delta) -> *const i8;
    fn str2date(source: *const i8, target: *mut i8);
    fn date2time(source: *const i8) -> time_t;
    fn zone_set(s: *const i8);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn redefined(c: i32);
    fn chk_set_rev(rev: *mut *const i8, arg: *mut i8);
    fn ffree();
    fn str_save(s: *const i8) -> *mut i8;
    fn runv(infd: i32, outname: *const i8, args: *mut *const i8) -> i32;
    fn setRCSversion(str: *const i8);
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn diagnose(fmt: *const i8, _: ...);
    fn syserror(e: i32, who: *const i8);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut exit_failure: i32;
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn make_space(name: *const i8) -> *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn close_space(divvy: *mut divvy);
    fn extend(tp: *mut link, x: *const libc::c_void, to: *mut divvy) -> *mut link;
    fn stat_mine_p(st: *mut stat) -> bool;
    fn setrid();
    fn getcaller() -> *const i8;
    fn caller_login_p(login: *const i8) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn addlock_maybe(delta: *mut delta, selfsame: bool, verbose: bool) -> i32;
    fn fopen_safer(filename: *const i8, type_0: *const i8) -> *mut FILE;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn maketemp(n: i32) -> *const i8;
    fn makedirtemp(isworkfile: bool) -> *const i8;
    fn keepdirtemp(name: *const i8);
    fn tempunlink();
    fn dirtempunlink();
    fn fro_zclose(p: *mut *mut fro);
    fn fro_trundling(sequential: bool, f: *mut fro);
    fn isr_do(scratch: *mut isr_scratch, action: isr_actions);
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum isr_actions {
    ISR_CATCHINTS,
    ISR_IGNOREINTS,
    ISR_RESTOREINTS,
    ISR_CATCHMMAPINTS,
}
impl isr_actions {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            isr_actions::ISR_CATCHINTS => 0,
            isr_actions::ISR_IGNOREINTS => 1,
            isr_actions::ISR_RESTOREINTS => 2,
            isr_actions::ISR_CATCHMMAPINTS => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> isr_actions {
        match value {
            0 => isr_actions::ISR_CATCHINTS,
            1 => isr_actions::ISR_IGNOREINTS,
            2 => isr_actions::ISR_RESTOREINTS,
            3 => isr_actions::ISR_CATCHMMAPINTS,
            _ => panic!("Invalid value for isr_actions: {}", value),
        }
    }
}
impl AddAssign<u32> for isr_actions {
    fn add_assign(&mut self, rhs: u32) {
        *self = isr_actions::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for isr_actions {
    fn sub_assign(&mut self, rhs: u32) {
        *self = isr_actions::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for isr_actions {
    fn mul_assign(&mut self, rhs: u32) {
        *self = isr_actions::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for isr_actions {
    fn div_assign(&mut self, rhs: u32) {
        *self = isr_actions::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for isr_actions {
    fn rem_assign(&mut self, rhs: u32) {
        *self = isr_actions::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for isr_actions {
    type Output = isr_actions;
    fn add(self, rhs: u32) -> isr_actions {
        isr_actions::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for isr_actions {
    type Output = isr_actions;
    fn sub(self, rhs: u32) -> isr_actions {
        isr_actions::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for isr_actions {
    type Output = isr_actions;
    fn mul(self, rhs: u32) -> isr_actions {
        isr_actions::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for isr_actions {
    type Output = isr_actions;
    fn div(self, rhs: u32) -> isr_actions {
        isr_actions::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for isr_actions {
    type Output = isr_actions;
    fn rem(self, rhs: u32) -> isr_actions {
        isr_actions::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct work {
    pub st: stat,
    pub force: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jstuff {
    pub jstuff: *mut divvy,
    pub head: link,
    pub tp: *mut link,
    pub merge: *mut symdef,
    pub expand: *const i8,
    pub suffix: *const i8,
    pub version: *const i8,
    pub zone: *const i8,
    pub d: *mut delta,
    pub ls: *mut *const i8,
    pub lastidx: i32,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: i64) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
static mut co_blurb: [i8; 53] = unsafe {
    *::core::mem::transmute::<
        &[u8; 53],
        &[i8; 53],
    >(b"Check out working files from revisions of RCS files.\0")
};
static mut co_help: [i8; 1679] = unsafe {
    *::core::mem::transmute::<
        &[u8; 1679],
        &[i8; 1679],
    >(
        b"[options] file ...\nOptions:\n  -f[REV]       Force overwrite of working file.\n  -I[REV]       Interactive.\n  -p[REV]       Write to stdout instead of the working file.\n  -q[REV]       Quiet mode.\n  -r[REV]       Normal checkout.\n  -l[REV]       Like -r, but also lock.\n  -u[REV]       Like -l, but unlock.\n  -M[REV]       Reset working file mtime (relevant for -l, -u).\n  -kSUBST       Use SUBST substitution, one of: kv, kvl, k, o, b, v.\n  -dDATE        Select latest before or on DATE.\n  -jJOINS       Merge using JOINS, a list of REV:REV pairs;\n                this option is obsolete -- see rcsmerge(1).\n  -sSTATE       Select matching state STATE.\n  -S            Enable \"self-same\" mode.\n  -T            Preserve the modification time on the RCS file\n                even if it changes because a lock is added or removed.\n  -wWHO         Select matching login WHO.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution\n                and also the default timezone for -dDATE.\n\nMultiple flags in {fIlMpqru} may be used, except for -r, -l, -u, which are\nmutually exclusive.  If specified, REV can be symbolic, numeric, or mixed:\n  symbolic -- must have been defined previously (see ci(1))\n  $        -- determine the revision number from keyword values\n              in the working file\n  .N       -- prepend default branch => DEFBR.N\n  BR.N     -- use this\n  BR       -- latest revision on branch BR\nIf REV is omitted, take it to be the latest on the default branch.\n\0",
    )
};
static mut ks_hws: [i8; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[i8; 3]>(b" \t\0")
};
static mut quietarg: [i8; 3] = unsafe {
    *::core::mem::transmute::<&[u8; 3], &[i8; 3]>(b"-q\0")
};
unsafe extern "C" fn cleanup(mut exitstatus: *mut i32, mut neworkptr: *mut *mut FILE) {
    let mut mstdout: *mut FILE = (*top).manifestation.standard_output;
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    ORCSclose();
    if !((*top).flow.from).is_null()
        && readmethod::RM_STDIO as i32 as u32 == (*(*top).flow.from).rm as u32
        && !((*top).flow.res).is_null() && (*top).flow.res != mstdout
    {
        Ozclose(&mut (*top).flow.res);
    }
    if *neworkptr != mstdout {
        Ozclose(neworkptr);
    }
    dirtempunlink();
}
unsafe extern "C" fn rmworkfile(mut work: *mut work) -> bool {
    if (*work).st.st_mode
        & (0o200 as i32 | 0o200 as i32 >> 3 as i32
            | 0o200 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0 && !(*work).force
    {
        let mut mani_filename: *const i8 = (*top).manifestation.filename;
        if !yesorno(
            0 as i32 != 0,
            b"writable %s exists%s; remove it\0" as *const u8 as *const i8,
            mani_filename,
            if stat_mine_p(&mut (*work).st) as i32 != 0 {
                b"\0" as *const u8 as *const i8
            } else {
                b", and you do not own it\0" as *const u8 as *const i8
            },
        ) {
            if !(*top).behavior.quiet && ttystdin() as i32 != 0 {
                generic_error(
                    0 as *const i8,
                    b"checkout aborted\0" as *const u8 as *const i8,
                );
            } else {
                generic_error(
                    0 as *const i8,
                    b"writable %s exists; checkout aborted\0" as *const u8 as *const i8,
                    mani_filename,
                );
            }
            return 0 as i32 != 0;
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn rmlock(mut delta: *const delta) -> i32 {
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut rl: *const rcslock = 0 as *const rcslock;
    box_0.next = (*(*top).repository.r).locks;
    tp = lock_memq(&mut box_0, 0 as i32 != 0, delta as *const libc::c_void);
    if tp.is_null() {
        return 0 as i32;
    }
    rl = (*(*tp).next).entry as *const rcslock;
    if !caller_login_p((*rl).login) {
        generic_error(
            (*top).repository.filename,
            b"revision %s locked by %s; use co -r or rcs -u\0" as *const u8 as *const i8,
            (*delta).num,
            (*rl).login,
        );
        return -(1 as i32);
    }
    lock_drop(&mut box_0, tp);
    return 1 as i32;
}
unsafe extern "C" fn jpush(mut rev: *const i8, mut js: *mut jstuff) {
    (*js).tp = extend((*js).tp, rev as *const libc::c_void, (*js).jstuff);
    (*js).lastidx += 1;
    (*js).lastidx;
}
unsafe extern "C" fn addjoin(mut spec: *mut i8, mut js: *mut jstuff) -> *mut i8 {
    let delims: [i8; 7] = *::core::mem::transmute::<&[u8; 7], &[i8; 7]>(b" \t\n:,;\0");
    let mut eot: *mut i8 = 0 as *mut i8;
    let mut save: i8 = 0;
    let mut cool: *mut delta = 0 as *mut delta;
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    spec = spec.offset(strspn(spec, ks_hws.as_ptr()) as isize);
    eot = spec.offset(strcspn(spec, delims.as_ptr()) as isize);
    save = *eot;
    *eot = '\0' as i32 as i8;
    cool = if fully_numeric(&mut numrev, spec, 0 as *mut fro) as i32 != 0 {
        delta_from_ref(numrev.string)
    } else {
        0 as *mut delta
    };
    *eot = save;
    if cool.is_null() {
        return 0 as *mut i8;
    }
    jpush((*cool).num, js);
    eot = eot.offset(strspn(eot, ks_hws.as_ptr()) as isize);
    return eot;
}
unsafe extern "C" fn getancestor(mut r1: *const i8, mut r2: *const i8) -> *const i8 {
    let mut t1: *const i8 = 0 as *const i8;
    let mut t2: *const i8 = 0 as *const i8;
    let mut l1: i32 = 0;
    let mut l2: i32 = 0;
    let mut l3: i32 = 0;
    let mut r: *const i8 = 0 as *const i8;
    l1 = countnumflds(r1);
    l2 = countnumflds(r2);
    if ((2 as i32) < l1 || (2 as i32) < l2) && !(0 as i32 == cmpnum(r1, r2)) {
        l3 = 0 as i32;
        while 0 as i32 == cmpnumfld(r1, r2, 1 as i32 + l3)
            && 0 as i32 == cmpnumfld(r1, r2, 2 as i32 + l3)
        {
            l3 += 2 as i32;
        }
        if l3 == 0 as i32 {
            t1 = (take((if l1 > 2 as i32 { 2 as i32 } else { l1 }) as size_t, r1))
                .string;
            t2 = (take((if l2 > 2 as i32 { 2 as i32 } else { l2 }) as size_t, r2))
                .string;
            r = if 0 as i32 > cmpnum(t1, t2) { t1 } else { t2 };
            if !(0 as i32 == cmpnum(r, r1)) && !(0 as i32 == cmpnum(r, r2)) {
                return str_save(r);
            }
        } else if !(0 as i32 == cmpnumfld(r1, r2, 1 as i32 + l3)) {
            return str_save((take(l3 as size_t, r1)).string)
        }
    }
    generic_error(
        (*top).repository.filename,
        b"common ancestor of %s and %s undefined\0" as *const u8 as *const i8,
        r1,
        r2,
    );
    return 0 as *const i8;
}
unsafe extern "C" fn preparejoin(mut argv: *mut i8, mut js: *mut jstuff) -> bool {
    let ks_comma: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &[i8; 2]>(b",\0");
    let mut s: *mut i8 = 0 as *mut i8;
    let mut save: *mut i8 = 0 as *mut i8;
    let mut j: *mut i8 = 0 as *mut i8;
    let mut rv: bool = 1 as i32 != 0;
    (*js).jstuff = make_space(b"jstuff\0" as *const u8 as *const i8);
    (*js).head.next = 0 as *mut link;
    (*js).tp = &mut (*js).head;
    if ((*js).merge).is_null() {
        (*js).merge = zlloc(
            plexus,
            (::core::mem::size_of::<symdef>() as u64).wrapping_mul(1 as i32 as u64),
        ) as *mut symdef;
        (*(*js).merge).meaningful = b"merge\0" as *const u8 as *const i8;
    }
    (*js).lastidx = -(1 as i32);
    s = argv;
    loop {
        j = strtok_r(s, ks_comma.as_ptr(), &mut save);
        if j.is_null() {
            break;
        }
        j = addjoin(j, js);
        if j.is_null() {
            return 0 as i32 != 0;
        }
        let mut current_block_18: u64;
        let fresh0 = j;
        j = j.offset(1);
        if *fresh0 as i32 == ':' as i32 {
            j = j.offset(strspn(j, ks_hws.as_ptr()) as isize);
            if *j as i32 == '\0' as i32 {
                current_block_18 = 18056988002315465010;
            } else {
                j = addjoin(j, js);
                if j.is_null() {
                    return 0 as i32 != 0;
                }
                current_block_18 = 11584701595673473500;
            }
        } else if (*js).lastidx == 0 as i32 {
            let mut two: *const i8 = (*(*js).tp).entry as *const i8;
            jpush(two, js);
            (*(*js).tp).entry = getancestor((*(*js).d).num, two) as *const libc::c_void;
            if ((*(*js).tp).entry).is_null() {
                rv = 0 as i32 != 0;
            }
            current_block_18 = 11584701595673473500;
        } else {
            current_block_18 = 18056988002315465010;
        }
        match current_block_18 {
            18056988002315465010 => {
                generic_fatal(
                    (*top).repository.filename,
                    b"join pair incomplete\0" as *const u8 as *const i8,
                );
            }
            _ => {}
        }
        s = 0 as *mut i8;
    }
    if (*js).lastidx < 1 as i32 {
        generic_fatal(
            (*top).repository.filename,
            b"empty join\0" as *const u8 as *const i8,
        );
    }
    (*js).ls = pointer_array(plexus, (1 as i32 + (*js).lastidx) as size_t)
        as *mut *const i8;
    (*js).tp = (*js).head.next;
    let mut i: i32 = 0 as i32;
    while i <= (*js).lastidx {
        let ref mut fresh1 = *((*js).ls).offset(i as isize);
        *fresh1 = (*(*js).tp).entry as *const i8;
        i += 1;
        i;
        (*js).tp = (*(*js).tp).next;
    }
    close_space((*js).jstuff);
    (*js).jstuff = 0 as *mut divvy;
    return rv;
}
unsafe extern "C" fn buildjoin(mut initialfile: *const i8, mut js: *mut jstuff) -> bool {
    let mut current_block: u64;
    let mut rev2: *const i8 = 0 as *const i8;
    let mut rev3: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut cov: [*const i8; 11] = [0 as *const i8; 11];
    let mut mergev: [*const i8; 11] = [0 as *const i8; 11];
    let mut p: *mut *const i8 = 0 as *mut *const i8;
    let mut len: size_t = 0;
    let mut subs: *const i8 = 0 as *const i8;
    rev2 = maketemp(0 as i32);
    rev3 = maketemp(3 as i32);
    cov[1 as i32 as usize] = find_peer_prog(&mut peer_super);
    cov[2 as i32 as usize] = b"co\0" as *const u8 as *const i8;
    p = &mut *cov.as_mut_ptr().offset((1 as i32 + 3 as i32) as isize) as *mut *const i8;
    if !((*js).expand).is_null() {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = (*js).expand;
    }
    if !((*js).suffix).is_null() {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = (*js).suffix;
    }
    if !((*js).version).is_null() {
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (*js).version;
    }
    if !((*js).zone).is_null() {
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = (*js).zone;
    }
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = quietarg.as_ptr();
    let fresh7 = p;
    p = p.offset(1);
    *fresh7 = (*top).repository.filename;
    *p = 0 as *const i8;
    mergev[1 as i32 as usize] = find_peer_prog((*js).merge);
    mergev[4 as i32 as usize] = b"-L\0" as *const u8 as *const i8;
    mergev[2 as i32 as usize] = mergev[4 as i32 as usize];
    i = 0 as i32;
    loop {
        if !(i < (*js).lastidx) {
            current_block = 17184638872671510253;
            break;
        }
        if i == 0 as i32 {
            subs = (*(*js).d).num;
        } else {
            accf(
                single,
                b"%s,%s:%s\0" as *const u8 as *const i8,
                subs,
                *((*js).ls).offset((i - 2 as i32) as isize),
                *((*js).ls).offset((i - 1 as i32) as isize),
            );
            subs = finish_string(single, &mut len);
        }
        diagnose(
            b"revision %s\0" as *const u8 as *const i8,
            *((*js).ls).offset(i as isize),
        );
        accf(
            single,
            b"-p%s\0" as *const u8 as *const i8,
            *((*js).ls).offset(i as isize),
        );
        cov[3 as i32 as usize] = finish_string(single, &mut len);
        if runv(-(1 as i32), rev2, cov.as_mut_ptr()) != 0 {
            current_block = 2538888681427581301;
            break;
        }
        diagnose(
            b"revision %s\0" as *const u8 as *const i8,
            *((*js).ls).offset((i + 1 as i32) as isize),
        );
        accf(
            single,
            b"-p%s\0" as *const u8 as *const i8,
            *((*js).ls).offset((i + 1 as i32) as isize),
        );
        cov[3 as i32 as usize] = finish_string(single, &mut len);
        if runv(-(1 as i32), rev3, cov.as_mut_ptr()) != 0 {
            current_block = 2538888681427581301;
            break;
        }
        diagnose(b"merging...\0" as *const u8 as *const i8);
        mergev[3 as i32 as usize] = subs;
        mergev[5 as i32 as usize] = *((*js).ls).offset((i + 1 as i32) as isize);
        p = &mut *mergev.as_mut_ptr().offset(6 as i32 as isize) as *mut *const i8;
        if (*top).behavior.quiet {
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = quietarg.as_ptr();
        }
        if (*js).lastidx <= i + 2 as i32
            && !((*top).manifestation.standard_output).is_null()
        {
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = b"-p\0" as *const u8 as *const i8;
        }
        let fresh10 = p;
        p = p.offset(1);
        *fresh10 = initialfile;
        let fresh11 = p;
        p = p.offset(1);
        *fresh11 = rev2;
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = rev3;
        *p = 0 as *const i8;
        if 2 as i32 == runv(-(1 as i32), 0 as *const i8, mergev.as_mut_ptr()) {
            current_block = 2538888681427581301;
            break;
        }
        i = i + 2 as i32;
    }
    match current_block {
        17184638872671510253 => return 1 as i32 != 0,
        _ => {
            (*top).flow.erroneous = 1 as i32 != 0;
            return 0 as i32 != 0;
        }
    };
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: co_blurb.as_ptr(),
            help: co_help.as_ptr(),
            tyag: (1 as i32) << 3 as i32
                | ((1 as i32) << 2 as i32 | (1 as i32) << 1 as i32),
        };
        init
    }
};
unsafe extern "C" fn co_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut exitstatus: i32 = 0 as i32;
    let mut work: work = {
        let mut init = work {
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
            force: 0 as i32 != 0,
        };
        init
    };
    let mut jstuff: jstuff = jstuff {
        jstuff: 0 as *mut divvy,
        head: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp: 0 as *mut link,
        merge: 0 as *mut symdef,
        expand: 0 as *const i8,
        suffix: 0 as *const i8,
        version: 0 as *const i8,
        zone: 0 as *const i8,
        d: 0 as *mut delta,
        ls: 0 as *mut *const i8,
        lastidx: 0,
    };
    let mut neworkptr: *mut FILE = 0 as *mut FILE;
    let mut lockflag: i32 = 0 as i32;
    let mut mtimeflag: bool = 0 as i32 != 0;
    let mut a: *mut i8 = 0 as *mut i8;
    let mut joinflag: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut author: *const i8 = 0 as *const i8;
    let mut date: *const i8 = 0 as *const i8;
    let mut rev: *const i8 = 0 as *const i8;
    let mut state: *const i8 = 0 as *const i8;
    let mut joinname: *const i8 = 0 as *const i8;
    let mut newdate: *const i8 = 0 as *const i8;
    let mut neworkname: *const i8 = 0 as *const i8;
    let mut changelock: i32 = 0;
    let mut expmode: i32 = 0;
    let mut r: i32 = 0;
    let mut workstatstat: i32 = 0;
    let mut tostdout: bool = false;
    let mut Ttimeflag: bool = false;
    let mut selfsame: bool = 0 as i32 != 0;
    let mut finaldate: [i8; 22] = [0; 22];
    let mut deltas: *mut wlink = 0 as *mut wlink;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut jstuff as *mut jstuff as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<jstuff>() as u64,
    );
    setrid();
    state = 0 as *const i8;
    rev = state;
    date = rev;
    author = date;
    joinflag = 0 as *mut i8;
    expmode = -(1 as i32);
    tostdout = 0 as i32 != 0;
    Ttimeflag = 0 as i32 != 0;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as i32) < argc
            && {
                let fresh13 = a;
                a = a.offset(1);
                *fresh13 as i32 == '-' as i32
            })
        {
            break;
        }
        let fresh14 = a;
        a = a.offset(1);
        match *fresh14 as i32 {
            114 => {
                current_block = 11259977801582577642;
            }
            102 => {
                work.force = 1 as i32 != 0;
                current_block = 11259977801582577642;
            }
            108 => {
                if lockflag < 0 as i32 {
                    generic_warn(
                        0 as *const i8,
                        b"-u overridden by -l.\0" as *const u8 as *const i8,
                    );
                }
                lockflag = 1 as i32;
                current_block = 11259977801582577642;
            }
            117 => {
                if (0 as i32) < lockflag {
                    generic_warn(
                        0 as *const i8,
                        b"-l overridden by -u.\0" as *const u8 as *const i8,
                    );
                }
                lockflag = -(1 as i32);
                current_block = 11259977801582577642;
            }
            112 => {
                tostdout = 1 as i32 != 0;
                current_block = 11259977801582577642;
            }
            73 => {
                (*top).behavior.interactive = 1 as i32 != 0;
                current_block = 11259977801582577642;
            }
            113 => {
                (*top).behavior.quiet = 1 as i32 != 0;
                current_block = 11259977801582577642;
            }
            100 => {
                if !date.is_null() {
                    redefined('d' as i32);
                }
                str2date(a, finaldate.as_mut_ptr());
                date = finaldate.as_mut_ptr();
                continue;
            }
            106 => {
                if *a != 0 {
                    if !joinflag.is_null() {
                        redefined('j' as i32);
                    }
                    joinflag = a;
                }
                continue;
            }
            77 => {
                mtimeflag = 1 as i32 != 0;
                current_block = 11259977801582577642;
            }
            115 => {
                if *a != 0 {
                    if !state.is_null() {
                        redefined('s' as i32);
                    }
                    state = a;
                }
                continue;
            }
            83 => {
                selfsame = 1 as i32 != 0;
                continue;
            }
            84 => {
                if *a != 0 {
                    current_block = 15717506275965100823;
                } else {
                    Ttimeflag = 1 as i32 != 0;
                    continue;
                }
            }
            119 => {
                if !author.is_null() {
                    redefined('w' as i32);
                }
                author = if *a as i32 != 0 { a } else { getcaller() };
                continue;
            }
            120 => {
                jstuff.suffix = *argv;
                (*top).behavior.pe = a;
                continue;
            }
            86 => {
                jstuff.version = *argv;
                setRCSversion(jstuff.version);
                continue;
            }
            122 => {
                jstuff.zone = *argv;
                zone_set(a);
                continue;
            }
            107 => {
                jstuff.expand = *argv;
                if 0 as i32 <= expmode {
                    redefined('k' as i32);
                }
                expmode = str2expmode(a);
                if 0 as i32 <= expmode {
                    continue;
                }
                current_block = 15717506275965100823;
            }
            _ => {
                current_block = 15717506275965100823;
            }
        }
        match current_block {
            15717506275965100823 => {
                bad_option(*argv);
            }
            _ => {
                chk_set_rev(&mut rev, a);
            }
        }
    }
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut neworkptr);
    } else if argc < 1 as i32 {
        generic_fatal(0 as *const i8, b"no input file\0" as *const u8 as *const i8);
    } else {
        let mut current_block_118: u64;
        while (0 as i32) < argc {
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut mani_filename: *const i8 = 0 as *const i8;
            let mut kws: i32 = 0;
            ffree();
            if !(pairnames(
                argc,
                argv,
                (if lockflag != 0 {
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                } else {
                    Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                }),
                1 as i32 != 0,
                0 as i32 != 0,
            ) <= 0 as i32)
            {
                repo_stat = &mut (*top).repository.stat;
                mani_filename = (*top).manifestation.filename;
                kws = (*top).behavior.kws;
                diagnose(
                    b"%s  -->  %s\0" as *const u8 as *const i8,
                    (*top).repository.filename,
                    if tostdout as i32 != 0 {
                        b"standard output\0" as *const u8 as *const i8
                    } else {
                        mani_filename
                    },
                );
                workstatstat = -(1 as i32);
                if tostdout {
                    neworkname = 0 as *const i8;
                    (*top).manifestation.standard_output = stdout;
                    neworkptr = (*top).manifestation.standard_output;
                    current_block_118 = 9199578309995299736;
                } else {
                    workstatstat = stat(mani_filename, &mut work.st);
                    if !(0 as i32 > workstatstat)
                        && ((*top).repository.stat.st_ino == work.st.st_ino
                            && (*top).repository.stat.st_dev == work.st.st_dev)
                    {
                        generic_error(
                            (*top).repository.filename,
                            b"RCS file is the same as working file %s.\0" as *const u8
                                as *const i8,
                            mani_filename,
                        );
                        current_block_118 = 18038362259723567392;
                    } else {
                        neworkname = makedirtemp(1 as i32 != 0);
                        neworkptr = fopen_safer(
                            neworkname,
                            b"w\0" as *const u8 as *const i8,
                        );
                        if neworkptr.is_null() {
                            if *__errno_location() == 13 as i32 {
                                generic_error(
                                    (*top).manifestation.filename,
                                    b"permission denied on parent directory\0" as *const u8
                                        as *const i8,
                                );
                            } else {
                                syserror(*__errno_location(), neworkname);
                            }
                            current_block_118 = 18038362259723567392;
                        } else {
                            current_block_118 = 9199578309995299736;
                        }
                    }
                }
                match current_block_118 {
                    18038362259723567392 => {}
                    _ => {
                        if ((*top).repository.tip).is_null() {
                            diagnose(
                                b"no revisions present; generating empty revision 0.0\0"
                                    as *const u8 as *const i8,
                            );
                            if lockflag != 0 {
                                generic_warn(
                                    0 as *const i8,
                                    b"no revisions, so nothing can be %slocked\0" as *const u8
                                        as *const i8,
                                    if lockflag < 0 as i32 {
                                        b"un\0" as *const u8 as *const i8
                                    } else {
                                        b"\0" as *const u8 as *const i8
                                    },
                                );
                            }
                            Ozclose(&mut (*top).flow.res);
                            if !(0 as i32 > workstatstat) {
                                if !rmworkfile(&mut work) {
                                    current_block_118 = 18038362259723567392;
                                } else {
                                    current_block_118 = 10369920510435091891;
                                }
                            } else {
                                current_block_118 = 10369920510435091891;
                            }
                            match current_block_118 {
                                18038362259723567392 => {}
                                _ => {
                                    changelock = 0 as i32;
                                    newdate = 0 as *const i8;
                                    current_block_118 = 10945915984064580713;
                                }
                            }
                        } else {
                            let mut numericrev: cbuf = cbuf {
                                string: 0 as *const i8,
                                size: 0,
                            };
                            let mut locks: i32 = if lockflag != 0 {
                                findlock(0 as i32 != 0, &mut jstuff.d)
                            } else {
                                0 as i32
                            };
                            let mut from: *mut fro = (*top).flow.from;
                            if !rev.is_null() {
                                if !fully_numeric(&mut numericrev, rev, 0 as *mut fro) {
                                    current_block_118 = 18038362259723567392;
                                } else {
                                    current_block_118 = 2220405792722996547;
                                }
                            } else {
                                match locks {
                                    0 => {
                                        current_block_118 = 16756902662845971877;
                                        match current_block_118 {
                                            409366286401371922 => {
                                                numericrev.string = str_save((*jstuff.d).num);
                                            }
                                            _ => {
                                                numericrev.string = if !((*(*top).repository.r).branch)
                                                    .is_null()
                                                {
                                                    (*(*top).repository.r).branch
                                                } else {
                                                    b"\0" as *const u8 as *const i8
                                                };
                                            }
                                        }
                                        current_block_118 = 2220405792722996547;
                                    }
                                    1 => {
                                        current_block_118 = 409366286401371922;
                                        match current_block_118 {
                                            409366286401371922 => {
                                                numericrev.string = str_save((*jstuff.d).num);
                                            }
                                            _ => {
                                                numericrev.string = if !((*(*top).repository.r).branch)
                                                    .is_null()
                                                {
                                                    (*(*top).repository.r).branch
                                                } else {
                                                    b"\0" as *const u8 as *const i8
                                                };
                                            }
                                        }
                                        current_block_118 = 2220405792722996547;
                                    }
                                    _ => {
                                        current_block_118 = 18038362259723567392;
                                    }
                                }
                            }
                            match current_block_118 {
                                18038362259723567392 => {}
                                _ => {
                                    jstuff.d = genrevs(
                                        numericrev.string,
                                        date,
                                        author,
                                        state,
                                        &mut deltas,
                                    );
                                    if (jstuff.d).is_null() {
                                        current_block_118 = 18038362259723567392;
                                    } else {
                                        changelock = if lockflag < 0 as i32 {
                                            rmlock(jstuff.d)
                                        } else if lockflag == 0 as i32 {
                                            0 as i32
                                        } else {
                                            addlock_maybe(jstuff.d, selfsame, 1 as i32 != 0)
                                        };
                                        if changelock < 0 as i32
                                            || changelock != 0 && !checkaccesslist()
                                            || 0 as i32 > dorewrite(lockflag != 0, changelock)
                                        {
                                            current_block_118 = 18038362259723567392;
                                        } else {
                                            if 0 as i32 <= expmode {
                                                (*top).behavior.kws = expmode;
                                                kws = (*top).behavior.kws;
                                            }
                                            if (0 as i32) < lockflag && kws == kwsub::kwsub_v as i32 {
                                                generic_error(
                                                    (*top).repository.filename,
                                                    b"cannot combine -kv and -l\0" as *const u8 as *const i8,
                                                );
                                                current_block_118 = 18038362259723567392;
                                            } else if !joinflag.is_null()
                                                && !preparejoin(joinflag, &mut jstuff)
                                            {
                                                current_block_118 = 18038362259723567392;
                                            } else {
                                                diagnose(
                                                    b"revision %s%s\0" as *const u8 as *const i8,
                                                    (*jstuff.d).num,
                                                    if (0 as i32) < lockflag {
                                                        b" (locked)\0" as *const u8 as *const i8
                                                    } else if lockflag < 0 as i32 {
                                                        b" (unlocked)\0" as *const u8 as *const i8
                                                    } else {
                                                        b"\0" as *const u8 as *const i8
                                                    },
                                                );
                                                (*from).verbatim = *((*(*jstuff.d).text).holes)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        ((*(*jstuff.d).text).count).wrapping_sub(1 as i32 as u64)
                                                            as isize,
                                                    ) + 2 as i32 as i64;
                                                if !(0 as i32 > workstatstat) {
                                                    if !rmworkfile(&mut work) {
                                                        current_block_118 = 18038362259723567392;
                                                    } else {
                                                        current_block_118 = 1677945370889843322;
                                                    }
                                                } else {
                                                    current_block_118 = 1677945370889843322;
                                                }
                                                match current_block_118 {
                                                    18038362259723567392 => {}
                                                    _ => {
                                                        write_desc_maybe((*top).flow.to);
                                                        (*top).behavior.inclusive_of_Locker_in_Id_val = (0 as i32)
                                                            < lockflag;
                                                        (*jstuff.d).name = namedrev(rev, jstuff.d);
                                                        joinname = buildrevision(
                                                            deltas,
                                                            jstuff.d,
                                                            if !joinflag.is_null() && tostdout as i32 != 0 {
                                                                0 as *mut FILE
                                                            } else {
                                                                neworkptr
                                                            },
                                                            kws < kwsub::kwsub_o as i32,
                                                        );
                                                        if (*top).flow.res == neworkptr {
                                                            (*top).flow.res = 0 as *mut FILE;
                                                        }
                                                        if changelock != 0
                                                            && (*deltas).entry != jstuff.d as *mut libc::c_void
                                                        {
                                                            fro_trundling(1 as i32 != 0, from);
                                                        }
                                                        if 0 as i32
                                                            > donerewrite(changelock, file_mtime(Ttimeflag, repo_stat))
                                                        {
                                                            current_block_118 = 18038362259723567392;
                                                        } else {
                                                            if changelock != 0 {
                                                                locks += lockflag;
                                                                if (1 as i32) < locks {
                                                                    generic_warn(
                                                                        (*top).repository.filename,
                                                                        b"You now have %d locks.\0" as *const u8 as *const i8,
                                                                        locks,
                                                                    );
                                                                }
                                                            }
                                                            newdate = (*jstuff.d).date;
                                                            if !joinflag.is_null() {
                                                                newdate = 0 as *const i8;
                                                                if joinname.is_null() {
                                                                    aflush(neworkptr);
                                                                    joinname = neworkname;
                                                                }
                                                                if kws == kwsub::kwsub_b as i32 {
                                                                    generic_error(
                                                                        (*top).manifestation.filename,
                                                                        b"merging binary files\0" as *const u8 as *const i8,
                                                                    );
                                                                }
                                                                if !buildjoin(joinname, &mut jstuff) {
                                                                    current_block_118 = 18038362259723567392;
                                                                } else {
                                                                    current_block_118 = 10945915984064580713;
                                                                }
                                                            } else {
                                                                current_block_118 = 10945915984064580713;
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
                        match current_block_118 {
                            18038362259723567392 => {}
                            _ => {
                                if !tostdout {
                                    let mut m: mode_t = (*repo_stat).st_mode
                                        & !(0o200 as i32 | 0o200 as i32 >> 3 as i32
                                            | 0o200 as i32 >> 3 as i32 >> 3 as i32) as mode_t
                                        | (if !(kws == kwsub::kwsub_v as i32
                                            || lockflag <= 0 as i32
                                                && (*top).behavior.strictly_locking as i32 != 0)
                                        {
                                            0o200 as i32
                                        } else {
                                            0 as i32
                                        }) as u32;
                                    let mut t: time_t = if mtimeflag as i32 != 0
                                        && !newdate.is_null()
                                    {
                                        date2time(newdate)
                                    } else {
                                        -(1 as i32) as time_t
                                    };
                                    aflush(neworkptr);
                                    isr_do((*top).behavior.isr, isr_actions::ISR_IGNOREINTS);
                                    r = chnamemod(
                                        &mut neworkptr,
                                        neworkname,
                                        mani_filename,
                                        1 as i32,
                                        m,
                                        make_timespec(t, 0 as i32 as i64),
                                    );
                                    keepdirtemp(neworkname);
                                    isr_do((*top).behavior.isr, isr_actions::ISR_RESTOREINTS);
                                    if 0 as i32 > r {
                                        syserror(*__errno_location(), mani_filename);
                                        generic_error(
                                            0 as *const i8,
                                            b"see %s\0" as *const u8 as *const i8,
                                            neworkname,
                                        );
                                    } else {
                                        diagnose(b"done\0" as *const u8 as *const i8);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            cleanup(&mut exitstatus, &mut neworkptr);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    Ozclose(&mut (*top).manifestation.standard_output);
    gnurcs_goodbye();
    return exitstatus;
}
static mut co_aka: [uint8_t; 13] = [
    2 as i32 as uint8_t,
    2 as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    8 as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'h' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    't' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_co: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                co_main as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: co_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};