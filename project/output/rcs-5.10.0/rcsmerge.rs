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
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn setRCSversion(str: *const i8);
    fn run(infd: i32, outname: *const i8, _: ...) -> i32;
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    fn str2expmode(s: *const i8) -> i32;
    fn merge(tostdout: bool, edarg: *const i8, three_manifestations: *mut symdef) -> i32;
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
    fn zone_set(s: *const i8);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn minus_p(xrev: *const i8, rev: *const i8) -> cbuf;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn diagnose(fmt: *const i8, _: ...);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_sys(who: *const i8);
    fn maketemp(n: i32) -> *const i8;
    fn tempunlink();
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_spew(f: *mut fro, to: *mut FILE);
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
static mut rcsmerge_blurb: [i8; 21] = unsafe {
    *::core::mem::transmute::<&[u8; 21], &[i8; 21]>(b"Merge RCS revisions.\0")
};
static mut rcsmerge_help: [i8; 894] = unsafe {
    *::core::mem::transmute::<
        &[u8; 894],
        &[i8; 894],
    >(
        b"[options] file\nOptions:\n  -A            Passed to diff3(1).\n  -E            Passed to diff3(1); default if unspecified.\n  -e            Passed to diff3(1); do not warn on conflicts.\n  -p[REV]       Write to stdout instead of overwriting the working file.\n  -q[REV]       Quiet mode.\n  -rREV         (one or two times) specify a revision.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nOne or two revisions must be specified (using -p, -q, or -r).\nIf only one is specified, use the latest revision on the default\nbranch to be the second revision.\n\0",
    )
};
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: rcsmerge_blurb.as_ptr(),
            help: rcsmerge_help.as_ptr(),
            tyag: (1 as i32) << 1 as i32 | (1 as i32) << 0 as i32,
        };
        init
    }
};
unsafe extern "C" fn rcsmerge_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut a: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut three_manifestations: [symdef; 3] = [symdef {
        meaningful: 0 as *const i8,
        underlying: 0 as *const i8,
    }; 3];
    let mut rev: [*const i8; 3] = [0 as *const i8; 3];
    let mut edarg: *const i8 = 0 as *const i8;
    let mut expandarg: *const i8 = 0 as *const i8;
    let mut suffixarg: *const i8 = 0 as *const i8;
    let mut versionarg: *const i8 = 0 as *const i8;
    let mut zonearg: *const i8 = 0 as *const i8;
    let mut tostdout: bool = false;
    let mut status: i32 = 0;
    let mut exitstatus: i32 = 0;
    let mut workptr: *mut fro = 0 as *mut fro;
    let mut target: *mut delta = 0 as *mut delta;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    rev[2 as i32 as usize] = 0 as *const i8;
    rev[1 as i32 as usize] = rev[2 as i32 as usize];
    edarg = rev[1 as i32 as usize];
    status = 0 as i32;
    tostdout = 0 as i32 != 0;
    zonearg = b"-q\0" as *const u8 as *const i8;
    versionarg = zonearg;
    suffixarg = versionarg;
    expandarg = suffixarg;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as i32) < argc
            && {
                let fresh0 = a;
                a = a.offset(1);
                *fresh0 as i32 == '-' as i32
            })
        {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        match *fresh1 as i32 {
            112 => {
                tostdout = 1 as i32 != 0;
                current_block = 16653359322154160579;
            }
            113 => {
                (*top).behavior.quiet = 1 as i32 != 0;
                current_block = 16653359322154160579;
            }
            114 => {
                current_block = 1451809985877802007;
            }
            65 | 69 | 101 => {
                if *a != 0 {
                    current_block = 7669208936594607488;
                } else {
                    edarg = *argv;
                    continue;
                }
            }
            120 => {
                suffixarg = *argv;
                (*top).behavior.pe = a;
                continue;
            }
            122 => {
                zonearg = *argv;
                zone_set(a);
                continue;
            }
            84 => {
                if !(*a != 0) {
                    continue;
                }
                current_block = 7669208936594607488;
            }
            86 => {
                versionarg = *argv;
                setRCSversion(versionarg);
                continue;
            }
            107 => {
                expandarg = *argv;
                if 0 as i32 <= str2expmode(expandarg.offset(2 as i32 as isize)) {
                    continue;
                }
                current_block = 7669208936594607488;
            }
            _ => {
                current_block = 7669208936594607488;
            }
        }
        match current_block {
            7669208936594607488 => {
                bad_option(*argv);
                continue;
            }
            16653359322154160579 => {
                if *a == 0 {
                    continue;
                }
            }
            _ => {}
        }
        if (rev[1 as i32 as usize]).is_null() {
            rev[1 as i32 as usize] = a;
        } else if (rev[2 as i32 as usize]).is_null() {
            rev[2 as i32 as usize] = a;
        } else {
            generic_error(
                0 as *const i8,
                b"too many %ss\0" as *const u8 as *const i8,
                ks_revno.as_ptr(),
            );
        }
    }
    if (rev[1 as i32 as usize]).is_null() {
        generic_fatal(
            0 as *const i8,
            b"no base %s given\0" as *const u8 as *const i8,
            ks_revno.as_ptr(),
        );
    }
    if !(*top).flow.erroneous {
        if argc < 1 as i32 {
            generic_fatal(0 as *const i8, b"no input file\0" as *const u8 as *const i8);
        }
        if (0 as i32)
            < pairnames(
                argc,
                argv,
                Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                1 as i32 != 0,
                0 as i32 != 0,
            )
        {
            let mut numericrev: cbuf = cbuf {
                string: 0 as *const i8,
                size: 0,
            };
            let mut repo_filename: *const i8 = (*top).repository.filename;
            let mut mani_filename: *const i8 = (*top).manifestation.filename;
            let mut defbr: *const i8 = (*(*top).repository.r).branch;
            let mut tip: *mut delta = (*top).repository.tip;
            if argc > 2 as i32
                || argc == 2 as i32 && !(*argv.offset(1 as i32 as isize)).is_null()
            {
                generic_warn(
                    0 as *const i8,
                    b"excess arguments ignored\0" as *const u8 as *const i8,
                );
            }
            if (*top).behavior.kws == kwsub::kwsub_b as i32 {
                generic_error(
                    (*top).manifestation.filename,
                    b"merging binary files\0" as *const u8 as *const i8,
                );
            }
            diagnose(b"RCS file: %s\0" as *const u8 as *const i8, repo_filename);
            workptr = fro_open(
                mani_filename,
                b"r\0" as *const u8 as *const i8,
                0 as *mut stat,
            );
            if workptr.is_null() {
                fatal_sys(mani_filename);
            }
            if tip.is_null() {
                generic_fatal(
                    (*top).repository.filename,
                    b"no revisions present\0" as *const u8 as *const i8,
                );
            }
            if *rev[1 as i32 as usize] == 0 {
                rev[1 as i32 as usize] = if !defbr.is_null() {
                    defbr
                } else {
                    (*tip).num
                };
            }
            if fully_numeric(&mut numericrev, rev[1 as i32 as usize], workptr) as i32
                != 0
                && {
                    target = delta_from_ref(numericrev.string);
                    !target.is_null()
                }
            {
                three_manifestations[1 as i32 as usize].meaningful = (*target).num;
                if (rev[2 as i32 as usize]).is_null() || *rev[2 as i32 as usize] == 0 {
                    rev[2 as i32 as usize] = if !defbr.is_null() {
                        defbr
                    } else {
                        (*tip).num
                    };
                }
                if fully_numeric(&mut numericrev, rev[2 as i32 as usize], workptr) as i32
                    != 0
                    && {
                        target = delta_from_ref(numericrev.string);
                        !target.is_null()
                    }
                {
                    three_manifestations[2 as i32 as usize].meaningful = (*target).num;
                    if strcmp(
                        three_manifestations[1 as i32 as usize].meaningful,
                        three_manifestations[2 as i32 as usize].meaningful,
                    ) == 0
                    {
                        if tostdout {
                            fro_spew(workptr, stdout);
                            fclose(stdout);
                        }
                    } else {
                        fro_zclose(&mut workptr);
                        i = 1 as i32;
                        while i <= 2 as i32 {
                            let mut commarg: cbuf = minus_p(
                                three_manifestations[i as usize].meaningful,
                                rev[i as usize],
                            );
                            three_manifestations[i as usize].underlying = maketemp(
                                i + 2 as i32,
                            );
                            if run(
                                -(1 as i32),
                                three_manifestations[i as usize].underlying,
                                find_peer_prog(&mut peer_super),
                                b"co\0" as *const u8 as *const i8,
                                b"-q\0" as *const u8 as *const i8,
                                commarg.string,
                                expandarg,
                                suffixarg,
                                versionarg,
                                zonearg,
                                repo_filename,
                                0 as *mut libc::c_void,
                            ) != 0
                            {
                                generic_fatal(
                                    (*top).repository.filename,
                                    b"co failed\0" as *const u8 as *const i8,
                                );
                            }
                            i += 1;
                            i;
                        }
                        diagnose(
                            b"Merging differences between %s and %s into %s%s\0"
                                as *const u8 as *const i8,
                            three_manifestations[1 as i32 as usize].meaningful,
                            three_manifestations[2 as i32 as usize].meaningful,
                            mani_filename,
                            if tostdout as i32 != 0 {
                                b"; result to stdout\0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                        );
                        three_manifestations[0 as i32 as usize].meaningful = mani_filename;
                        three_manifestations[0 as i32 as usize].underlying = three_manifestations[0
                                as i32 as usize]
                            .meaningful;
                        status = merge(
                            tostdout,
                            edarg,
                            three_manifestations.as_mut_ptr(),
                        );
                    }
                }
            }
            fro_zclose(&mut workptr);
        }
    }
    tempunlink();
    exitstatus = if (*top).flow.erroneous as i32 != 0 { 2 as i32 } else { status };
    gnurcs_goodbye();
    return exitstatus;
}
static mut rcsmerge_aka: [uint8_t; 16] = [
    2 as i32 as uint8_t,
    5 as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    8 as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    'e' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_rcsmerge: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsmerge_main
                    as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: rcsmerge_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};