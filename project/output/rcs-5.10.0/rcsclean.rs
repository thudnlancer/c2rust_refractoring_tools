use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    static mut top: *mut top;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn setRCSversion(str: *const i8);
    fn str2expmode(s: *const i8) -> i32;
    fn un_link(s: *const i8) -> i32;
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn findlock(delete: bool, target: *mut *mut delta) -> i32;
    fn checkaccesslist() -> bool;
    fn dorewrite(lockflag: bool, changed: i32) -> i32;
    fn donerewrite(changed: i32, mtime: timespec) -> i32;
    fn ORCSclose();
    fn rcsfcmp(
        xfp: *mut fro,
        xstatp: *const stat,
        uname: *const i8,
        delta: *const delta,
    ) -> i32;
    fn rcssuffix(name: *const i8) -> *const i8;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: i32,
        argv: *mut *mut i8,
        rcsopen: Option<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> i32;
    fn buildrevision(
        deltas: *const wlink,
        target: *mut delta,
        outfile: *mut FILE,
        expandflag: bool,
    ) -> *const i8;
    fn write_desc_maybe(to: *mut FILE);
    fn gr_revno(revno: *const i8, store: *mut *mut wlink) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const i8, fp: *mut fro) -> bool;
    fn zone_set(s: *const i8);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn redefined(c: i32);
    fn chk_set_rev(rev: *mut *const i8, arg: *mut i8);
    fn ffree();
    fn str_save(s: *const i8) -> *mut i8;
    fn __errno_location() -> *mut i32;
    fn free(__ptr: *mut libc::c_void);
    fn scandir(
        __dir: *const i8,
        __namelist: *mut *mut *mut dirent,
        __selector: Option<unsafe extern "C" fn(*const dirent) -> i32>,
        __cmp: Option<
            unsafe extern "C" fn(*mut *const dirent, *mut *const dirent) -> i32,
        >,
    ) -> i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn fatal_sys(who: *const i8);
    static mut exit_failure: i32;
    fn syserror(e: i32, who: *const i8);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    static mut plexus: *mut divvy;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn setrid();
    fn caller_login_p(login: *const i8) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn Ozclose(p: *mut *mut FILE);
    fn aprintf(iop: *mut FILE, fmt: *const i8, _: ...);
    fn tempunlink();
    fn dirtempunlink();
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn fro_zclose(p: *mut *mut fro);
    fn fro_trundling(sequential: bool, f: *mut fro);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
pub type submain_t = unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yacmd {
    pub func: Option<submain_t>,
    pub aka: *const uint8_t,
    pub pr: *mut program,
}
static mut rcsclean_blurb: [i8; 24] = unsafe {
    *::core::mem::transmute::<&[u8; 24], &[i8; 24]>(b"Clean up working files.\0")
};
static mut rcsclean_help: [i8; 705] = unsafe {
    *::core::mem::transmute::<
        &[u8; 705],
        &[i8; 705],
    >(
        b"[options] file ...\nOptions:\n  -r[REV]       Specify revision.\n  -u[REV]       Unlock if is locked and no differences found.\n  -n[REV]       Dry run (no act, don't operate).\n  -q[REV]       Quiet mode.\n  -kSUBST       Substitute using mode SUBST (see co(1)).\n  -T            Preserve the modification time on the RCS file\n                even if it changes because a lock is removed.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n\nREV defaults to the latest revision on the default branch.\n\0",
    )
};
unsafe extern "C" fn cleanup(mut exitstatus: *mut i32, mut workptr: *mut *mut fro) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    fro_zclose(workptr);
    Ozclose(&mut (*top).flow.res);
    ORCSclose();
    dirtempunlink();
}
unsafe extern "C" fn unlock(mut delta: *mut delta) -> bool {
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    if !delta.is_null() && !((*delta).lockedby).is_null()
        && caller_login_p((*delta).lockedby) as i32 != 0
        && {
            box_0.next = (*(*top).repository.r).locks;
            !(box_0.next).is_null()
        }
        && {
            tp = lock_memq(&mut box_0, 0 as i32 != 0, delta as *const libc::c_void);
            !tp.is_null()
        }
    {
        lock_drop(&mut box_0, tp);
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn valid_filename_p(mut ent: *const dirent) -> i32 {
    let mut en: *const i8 = ((*ent).d_name).as_ptr();
    if *en.offset(0 as i32 as isize) as i32 == '.' as i32
        && (*en.offset(1 as i32 as isize) == 0
            || *en.offset(1 as i32 as isize) as i32 == '.' as i32
                && *en.offset(2 as i32 as isize) == 0)
    {
        return 0 as i32;
    }
    if !(rcssuffix(en)).is_null() {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn get_cwd_filenames(mut aargv: *mut *mut *mut i8) -> i32 {
    let mut dot: [i8; 2] = *::core::mem::transmute::<&[u8; 2], &mut [i8; 2]>(b".\0");
    let mut names: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut count: i32 = 0;
    let mut i: i32 = 0;
    count = scandir(
        dot.as_mut_ptr(),
        &mut names,
        Some(valid_filename_p as unsafe extern "C" fn(*const dirent) -> i32),
        None,
    );
    if 0 as i32 > count {
        fatal_sys(dot.as_mut_ptr());
    }
    *aargv = pointer_array(plexus, count as size_t) as *mut *mut i8;
    i = count;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        let ref mut fresh1 = *(*aargv).offset(i as isize);
        *fresh1 = str_save(((**names.offset(i as isize)).d_name).as_mut_ptr());
        free(*names.offset(i as isize) as *mut libc::c_void);
    }
    free(names as *mut libc::c_void);
    return count;
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: rcsclean_blurb.as_ptr(),
            help: rcsclean_help.as_ptr(),
            tyag: (1 as i32) << 3 as i32
                | ((1 as i32) << 2 as i32 | (1 as i32) << 1 as i32),
        };
        init
    }
};
unsafe extern "C" fn rcsclean_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut current_block: u64;
    let mut exitstatus: i32 = 0 as i32;
    let mut workptr: *mut fro = 0 as *mut fro;
    let mut a: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut rev: *const i8 = 0 as *const i8;
    let mut p: *const i8 = 0 as *const i8;
    let mut dounlock: bool = false;
    let mut perform: bool = false;
    let mut unlocked: bool = false;
    let mut unlockflag: bool = false;
    let mut waslocked: bool = false;
    let mut Ttimeflag: bool = false;
    let mut expmode: i32 = 0;
    let mut deltas: *mut wlink = 0 as *mut wlink;
    let mut delta: *mut delta = 0 as *mut delta;
    let mut workstat: stat = stat {
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
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    setrid();
    expmode = -(1 as i32);
    rev = 0 as *const i8;
    perform = 1 as i32 != 0;
    unlockflag = 0 as i32 != 0;
    Ttimeflag = 0 as i32 != 0;
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argc -= 1;
        if argc < 1 as i32 {
            argc = get_cwd_filenames(&mut newargv);
            argv = newargv;
            break;
        } else {
            argv = argv.offset(1);
            a = *argv;
            if *a == 0
                || {
                    let fresh2 = a;
                    a = a.offset(1);
                    *fresh2 as i32 != '-' as i32
                }
            {
                break;
            }
            let fresh3 = a;
            a = a.offset(1);
            match *fresh3 as i32 {
                107 => {
                    if 0 as i32 <= expmode {
                        redefined('k' as i32);
                    }
                    expmode = str2expmode(a);
                    if !(0 as i32 > expmode) {
                        continue;
                    }
                    current_block = 7420818398119998636;
                }
                110 => {
                    perform = 0 as i32 != 0;
                    current_block = 10377355788769887259;
                }
                113 => {
                    (*top).behavior.quiet = 1 as i32 != 0;
                    current_block = 10377355788769887259;
                }
                114 => {
                    current_block = 10377355788769887259;
                }
                84 => {
                    if *a != 0 {
                        current_block = 7420818398119998636;
                    } else {
                        Ttimeflag = 1 as i32 != 0;
                        continue;
                    }
                }
                117 => {
                    unlockflag = 1 as i32 != 0;
                    current_block = 10377355788769887259;
                }
                86 => {
                    setRCSversion(*argv);
                    continue;
                }
                120 => {
                    (*top).behavior.pe = a;
                    continue;
                }
                122 => {
                    zone_set(a);
                    continue;
                }
                _ => {
                    current_block = 7420818398119998636;
                }
            }
            match current_block {
                7420818398119998636 => {
                    bad_option(*argv);
                }
                _ => {
                    chk_set_rev(&mut rev, a);
                }
            }
        }
    }
    dounlock = perform as i32 & unlockflag as i32 != 0;
    if (*top).flow.erroneous {
        cleanup(&mut exitstatus, &mut workptr);
    } else {
        let mut current_block_56: u64;
        while (0 as i32) < argc {
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut mani_filename: *const i8 = 0 as *const i8;
            ffree();
            if (0 as i32)
                < pairnames(
                    argc,
                    argv,
                    (if dounlock as i32 != 0 {
                        Some(
                            rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro,
                        )
                    } else {
                        Some(rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro)
                    }),
                    1 as i32 != 0,
                    1 as i32 != 0,
                )
                && {
                    mani_filename = (*top).manifestation.filename;
                    !mani_filename.is_null()
                }
                && {
                    workptr = fro_open(
                        mani_filename,
                        b"r\0" as *const u8 as *const i8,
                        &mut workstat,
                    );
                    !workptr.is_null()
                }
            {
                repo_stat = &mut (*top).repository.stat;
                if (*top).repository.stat.st_ino == workstat.st_ino
                    && (*top).repository.stat.st_dev == workstat.st_dev
                {
                    generic_error(
                        (*top).repository.filename,
                        b"RCS file is the same as working file %s.\0" as *const u8
                            as *const i8,
                        mani_filename,
                    );
                } else {
                    p = 0 as *const i8;
                    if !rev.is_null() {
                        let mut numeric: cbuf = cbuf {
                            string: 0 as *const i8,
                            size: 0,
                        };
                        if !fully_numeric(&mut numeric, rev, workptr) {
                            current_block_56 = 11743904203796629665;
                        } else {
                            p = numeric.string;
                            current_block_56 = 1854459640724737493;
                        }
                    } else if !((*top).repository.tip).is_null() {
                        match if unlockflag as i32 != 0 {
                            findlock(0 as i32 != 0, &mut delta)
                        } else {
                            0 as i32
                        } {
                            0 => {
                                current_block_56 = 16801834866831430556;
                                match current_block_56 {
                                    12984810581895796367 => {
                                        p = (*delta).num;
                                    }
                                    _ => {
                                        p = if !((*(*top).repository.r).branch).is_null() {
                                            (*(*top).repository.r).branch
                                        } else {
                                            b"\0" as *const u8 as *const i8
                                        };
                                    }
                                }
                                current_block_56 = 1854459640724737493;
                            }
                            1 => {
                                current_block_56 = 12984810581895796367;
                                match current_block_56 {
                                    12984810581895796367 => {
                                        p = (*delta).num;
                                    }
                                    _ => {
                                        p = if !((*(*top).repository.r).branch).is_null() {
                                            (*(*top).repository.r).branch
                                        } else {
                                            b"\0" as *const u8 as *const i8
                                        };
                                    }
                                }
                                current_block_56 = 1854459640724737493;
                            }
                            _ => {
                                current_block_56 = 11743904203796629665;
                            }
                        }
                    } else {
                        current_block_56 = 1854459640724737493;
                    }
                    match current_block_56 {
                        11743904203796629665 => {}
                        _ => {
                            delta = 0 as *mut delta;
                            deltas = 0 as *mut wlink;
                            if !(!p.is_null()
                                && {
                                    delta = gr_revno(p, &mut deltas);
                                    delta.is_null()
                                })
                            {
                                waslocked = !delta.is_null()
                                    && !((*delta).lockedby).is_null();
                                (*top).behavior.inclusive_of_Locker_in_Id_val = unlock(
                                    delta,
                                );
                                unlocked = (*top).behavior.inclusive_of_Locker_in_Id_val
                                    as i32 & unlockflag as i32 != 0;
                                if !((unlocked as i32) < waslocked as i32
                                    && workstat.st_mode
                                        & (0o200 as i32 | 0o200 as i32 >> 3 as i32
                                            | 0o200 as i32 >> 3 as i32 >> 3 as i32) as u32 != 0)
                                {
                                    if !(unlocked as i32 != 0 && !checkaccesslist()) {
                                        if !(0 as i32 > dorewrite(dounlock, unlocked as i32)) {
                                            if 0 as i32 <= expmode {
                                                (*top).behavior.kws = expmode;
                                            } else if waslocked as i32 != 0
                                                && (*top).behavior.kws == kwsub::kwsub_kv as i32
                                                && (*repo_stat).st_mode
                                                    & !(0o200 as i32 | 0o200 as i32 >> 3 as i32
                                                        | 0o200 as i32 >> 3 as i32 >> 3 as i32) as mode_t
                                                    | (if 1 as i32 != 0 { 0o200 as i32 } else { 0 as i32 })
                                                        as u32 == workstat.st_mode
                                            {
                                                (*top).behavior.kws = kwsub::kwsub_kvl as i32;
                                            }
                                            write_desc_maybe((*top).flow.to);
                                            if !(if delta.is_null() {
                                                (workstat.st_size != 0 as i32 as i64) as i32
                                            } else {
                                                ((0 as i32)
                                                    < rcsfcmp(
                                                        workptr,
                                                        &mut workstat,
                                                        buildrevision(deltas, delta, 0 as *mut FILE, 0 as i32 != 0),
                                                        delta,
                                                    )) as i32
                                            } != 0)
                                            {
                                                if ((*top).behavior.quiet as i32) < unlocked as i32 {
                                                    aprintf(
                                                        stdout,
                                                        b"rcs -u%s %s\n\0" as *const u8 as *const i8,
                                                        (*delta).num,
                                                        (*top).repository.filename,
                                                    );
                                                }
                                                if perform as i32 & unlocked as i32 != 0 {
                                                    let mut from: *mut fro = (*top).flow.from;
                                                    (*from).verbatim = *((*(*delta).text).holes)
                                                        .as_mut_ptr()
                                                        .offset(
                                                            ((*(*delta).text).count).wrapping_sub(1 as i32 as u64)
                                                                as isize,
                                                        ) + 2 as i32 as i64;
                                                    if (*deltas).entry != delta as *mut libc::c_void {
                                                        fro_trundling(1 as i32 != 0, from);
                                                    }
                                                    if 0 as i32
                                                        > donerewrite(1 as i32, file_mtime(Ttimeflag, repo_stat))
                                                    {
                                                        current_block_56 = 11743904203796629665;
                                                    } else {
                                                        current_block_56 = 6367734732029634840;
                                                    }
                                                } else {
                                                    current_block_56 = 6367734732029634840;
                                                }
                                                match current_block_56 {
                                                    11743904203796629665 => {}
                                                    _ => {
                                                        if !(*top).behavior.quiet {
                                                            aprintf(
                                                                stdout,
                                                                b"rm -f %s\n\0" as *const u8 as *const i8,
                                                                mani_filename,
                                                            );
                                                        }
                                                        fro_zclose(&mut workptr);
                                                        if perform as i32 != 0 && 0 as i32 > un_link(mani_filename)
                                                        {
                                                            syserror(*__errno_location(), mani_filename);
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
            cleanup(&mut exitstatus, &mut workptr);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    if !(*top).behavior.quiet {
        fclose(stdout);
    }
    gnurcs_goodbye();
    return exitstatus;
}
static mut rcsclean_aka: [uint8_t; 16] = [
    2 as i32 as uint8_t,
    5 as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    8 as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'n' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_rcsclean: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcsclean_main
                    as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: rcsclean_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};