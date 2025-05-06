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
    pub type editstuff;
    static prog_diff: [i8; 0];
    static diff_flags: [i8; 0];
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    fn ORCSclose();
    fn tmpfile() -> *mut FILE;
    fn rewind(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> i32;
    fn donerewrite(changed: i32, mtime: timespec) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn file_mtime(enable: bool, st: *const stat) -> timespec;
    fn str2expmode(s: *const i8) -> i32;
    fn make_editstuff() -> *mut editstuff;
    fn unmake_editstuff(es: *mut editstuff);
    fn finishedit(
        es: *mut editstuff,
        delta: *const delta,
        outfile: *mut FILE,
        done: bool,
    );
    fn snapshotedit(es: *mut editstuff, f: *mut FILE);
    fn enterstring(es: *mut editstuff, atat: *mut atat);
    fn editstring(es: *mut editstuff, script: *const atat, delta: *const delta);
    fn rcswriteopen(m: *mut maybe) -> *mut fro;
    fn findlock(delete: bool, target: *mut *mut delta) -> i32;
    fn addsymbol(num: *const i8, name: *const i8, rebind: bool) -> i32;
    fn checkaccesslist() -> bool;
    fn putdtext(
        delta: *const delta,
        srcname: *const i8,
        fout: *mut FILE,
        diffmt: bool,
    ) -> bool;
    fn rcsreadopen(m: *mut maybe) -> *mut fro;
    fn pairnames(
        argc: i32,
        argv: *mut *mut i8,
        rcsopen: Option<open_rcsfile_fn>,
        mustread: bool,
        quiet: bool,
    ) -> i32;
    fn cleanlogmsg(m: *const i8, s: size_t) -> cbuf;
    fn yesorno(default_answer: bool, question: *const i8, _: ...) -> bool;
    fn putdesc(cb: *mut cbuf, textflag: bool, textfile: *mut i8);
    fn putadmin();
    fn puttree(root: *const delta, fout: *mut FILE);
    fn putstring(out: *mut FILE, s: cbuf, log: bool);
    fn checkid(id: *const i8, delimiter: i32) -> *const i8;
    fn checksym(sym: *const i8, delimiter: i32) -> *const i8;
    fn countnumflds(s: *const i8) -> i32;
    fn take(count: size_t, ref_0: *const i8) -> cbuf;
    fn cmpnum(num1: *const i8, num2: *const i8) -> i32;
    fn cmpnumfld(num1: *const i8, num2: *const i8, fld: i32) -> i32;
    fn compartial(num1: *const i8, num2: *const i8, length: i32) -> i32;
    fn gr_revno(revno: *const i8, store: *mut *mut wlink) -> *mut delta;
    fn fully_numeric(ans: *mut cbuf, source: *const i8, fp: *mut fro) -> bool;
    fn tiprev() -> *const i8;
    fn zone_set(s: *const i8);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn redefined(c: i32);
    fn parse_revpairs(
        option: i8,
        arg: *mut i8,
        data: *mut libc::c_void,
        put: Option<
            unsafe extern "C" fn(*const i8, *const i8, bool, *mut libc::c_void) -> (),
        >,
    );
    fn set_empty_log_message(cb: *mut cbuf);
    fn ffree();
    fn str_save(s: *const i8) -> *mut i8;
    fn runv(infd: i32, outname: *const i8, args: *mut *const i8) -> i32;
    fn setRCSversion(str: *const i8);
    fn getRCSINIT(argc: i32, argv: *mut *mut i8, newargv: *mut *mut *mut i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn complain(fmt: *const i8, _: ...);
    static mut exit_failure: i32;
    fn diagnose(fmt: *const i8, _: ...);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_sys(who: *const i8);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accumulate_range(divvy: *mut divvy, beg: *const i8, end: *const i8);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn extend(tp: *mut link, x: *const libc::c_void, to: *mut divvy) -> *mut link;
    fn nosetid();
    fn caller_login_p(login: *const i8) -> bool;
    fn lock_memq(ls: *mut link, login: bool, x: *const libc::c_void) -> *mut link;
    fn lock_on(delta: *const delta) -> *const rcslock;
    fn lock_drop(box_0: *mut link, tp: *mut link);
    fn addlock_maybe(delta: *mut delta, selfsame: bool, verbose: bool) -> i32;
    fn Ozclose(p: *mut *mut FILE);
    fn aflush(f: *mut FILE);
    fn newline(f: *mut FILE);
    fn maketemp(n: i32) -> *const i8;
    fn tempunlink();
    fn dirtempunlink();
    fn fro_zclose(p: *mut *mut fro);
    fn fro_move(f: *mut fro, change: off_t);
    fn fro_trundling(sequential: bool, f: *mut fro);
    fn fro_spew_partial(to: *mut FILE, f: *mut fro, r: *mut range);
    fn string_from_atat(space: *mut divvy, atat: *const atat) -> cbuf;
    fn atat_put(to: *mut FILE, atat: *const atat);
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
pub struct u_symdef {
    pub u: symdef,
    pub override_0: bool,
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
pub struct range {
    pub beg: off_t,
    pub end: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_log {
    pub revno: *const i8,
    pub message: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct u_state {
    pub revno: *const i8,
    pub status: *const i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum changeaccess {
    append,
    erase,
}
impl changeaccess {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            changeaccess::append => 0,
            changeaccess::erase => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> changeaccess {
        match value {
            0 => changeaccess::append,
            1 => changeaccess::erase,
            _ => panic!("Invalid value for changeaccess: {}", value),
        }
    }
}
impl AddAssign<u32> for changeaccess {
    fn add_assign(&mut self, rhs: u32) {
        *self = changeaccess::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for changeaccess {
    fn sub_assign(&mut self, rhs: u32) {
        *self = changeaccess::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for changeaccess {
    fn mul_assign(&mut self, rhs: u32) {
        *self = changeaccess::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for changeaccess {
    fn div_assign(&mut self, rhs: u32) {
        *self = changeaccess::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for changeaccess {
    fn rem_assign(&mut self, rhs: u32) {
        *self = changeaccess::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for changeaccess {
    type Output = changeaccess;
    fn add(self, rhs: u32) -> changeaccess {
        changeaccess::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for changeaccess {
    type Output = changeaccess;
    fn sub(self, rhs: u32) -> changeaccess {
        changeaccess::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for changeaccess {
    type Output = changeaccess;
    fn mul(self, rhs: u32) -> changeaccess {
        changeaccess::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for changeaccess {
    type Output = changeaccess;
    fn div(self, rhs: u32) -> changeaccess {
        changeaccess::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for changeaccess {
    type Output = changeaccess;
    fn rem(self, rhs: u32) -> changeaccess {
        changeaccess::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chaccess {
    pub login: *const i8,
    pub command: changeaccess,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delrevpair {
    pub strt: *const i8,
    pub end: *const i8,
    pub code: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adminstuff {
    pub rv: i32,
    pub deltas: *mut wlink,
    pub suppress_mail: bool,
    pub lockhead: bool,
    pub unlockcaller: bool,
    pub newlocks: *mut link,
    pub byelocks: *mut link,
    pub headstate: *const i8,
    pub headstate_changed: bool,
    pub states: link,
    pub tp_state: *mut link,
    pub accesses: link,
    pub tp_access: *mut link,
    pub assocs: link,
    pub tp_assoc: *mut link,
    pub logs: link,
    pub tp_log: *mut link,
    pub cuthead: *mut delta,
    pub cuttail: *mut delta,
    pub delstrt: *mut delta,
    pub delrev: delrevpair,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut rcs_blurb: [i8; 28] = unsafe {
    *::core::mem::transmute::<&[u8; 28], &[i8; 28]>(b"Change RCS file attributes.\0")
};
static mut rcs_help: [i8; 2044] = unsafe {
    *::core::mem::transmute::<
        &[u8; 2044],
        &[i8; 2044],
    >(
        b"[options] file ...\nOptions:\n  -i              Create and initialize a new RCS file.\n  -L              Set strict locking.\n  -U              Set non-strict locking.\n  -M              Don't send mail when breaking someone else's lock.\n  -T              Preserve the modification time on the\n                  RCS file unless a revision is removed.\n  -I              Interactive.\n  -q              Quiet mode.\n  -aLOGINS        Append LOGINS (comma-separated) to access-list.\n  -e[LOGINS]      Erase LOGINS (all if unspecified) from access-list.\n  -AFILENAME      Append access-list of FILENAME to current access-list.\n  -b[REV]         Set default branch to that of REV or\n                  highest branch on trunk if REV is omitted.\n  -l[REV]         Lock revision REV.\n  -u[REV]         Unlock revision REV.\n  -cSTRING        Set comment leader to STRING; don't use: obsolete.\n  -kSUBST         Set default keyword substitution to SUBST (see co(1)).\n  -mREV:MSG       Replace REV's log message with MSG.\n  -nNAME[:[REV]]  If :REV is omitted, delete symbolic NAME.\n                  Otherwise, associate NAME with REV; NAME must be new.\n  -NNAME[:[REV]]  Like -n, but overwrite any previous assignment.\n  -oRANGE         Outdate revisions in RANGE:\n                    REV       -- single revision\n                    BR        -- latest revision on branch BR\n                    REV1:REV2 -- REV1 to REV2 on same branch\n                    :REV      -- beginning of branch to REV\n                    REV:      -- REV to end of branch\n  -sSTATE[:REV]   Set state of REV to STATE.\n  -t-TEXT         Set description in RCS file to TEXT.\n  -tFILENAME      Set description in RCS file to contents of FILENAME.\n  -V              Obsolete; do not use.\n  -VN             Emulate RCS version N.\n  -xSUFF          Specify SUFF as a slash-separated list of suffixes\n                  used to identify RCS file names.\n  -zZONE          No effect; included for compatibility with other commands.\n\nREV defaults to the latest revision on the default branch.\n\0",
    )
};
static mut ks_ws_comma: [i8; 5] = unsafe {
    *::core::mem::transmute::<&[u8; 5], &[i8; 5]>(b" \n\t,\0")
};
unsafe extern "C" fn cleanup(mut exitstatus: *mut i32) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
    Ozclose(&mut (*top).flow.res);
    ORCSclose();
    dirtempunlink();
}
unsafe extern "C" fn getassoclst(mut dc: *mut adminstuff, mut sp: *mut i8) {
    let fresh0 = sp;
    sp = sp.offset(1);
    let mut option: i8 = *fresh0;
    let mut ud: *mut u_symdef = 0 as *mut u_symdef;
    let mut name: *const i8 = 0 as *const i8;
    let mut len: size_t = 0;
    let mut c: i32 = *sp as i32;
    let mut tp: *mut *mut link = &mut (*dc).tp_assoc;
    if (*tp).is_null() {
        *tp = &mut (*dc).assocs;
    }
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as i32;
    }
    accumulate_range(plexus, sp, checksym(sp, ':' as i32));
    name = finish_string(plexus, &mut len);
    sp = sp.offset(len as isize);
    c = *sp as i32;
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as i32;
    }
    if c != ':' as i32 && c != '\0' as i32 {
        generic_error(
            0 as *const i8,
            b"invalid string `%s' after option `-%c'\0" as *const u8 as *const i8,
            sp,
            option as i32,
        );
        return;
    }
    ud = zlloc(
        plexus,
        (::core::mem::size_of::<u_symdef>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut u_symdef;
    (*ud).u.meaningful = name;
    (*ud).override_0 = 'N' as i32 == option as i32;
    if c == '\0' as i32 {
        (*ud).u.underlying = 0 as *const i8;
    } else {
        sp = sp.offset(1);
        c = *sp as i32;
        while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
            sp = sp.offset(1);
            c = *sp as i32;
        }
        (*ud).u.underlying = sp;
    }
    *tp = extend(*tp, ud as *const libc::c_void, plexus);
}
unsafe extern "C" fn getchaccess(
    mut dc: *mut adminstuff,
    mut login: *const i8,
    mut command: changeaccess,
) {
    let mut ch: *mut chaccess = 0 as *mut chaccess;
    let mut tp: *mut *mut link = &mut (*dc).tp_access;
    if (*tp).is_null() {
        *tp = &mut (*dc).accesses;
    }
    ch = zlloc(
        plexus,
        (::core::mem::size_of::<chaccess>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut chaccess;
    (*ch).login = login;
    (*ch).command = command;
    *tp = extend(*tp, ch as *const libc::c_void, plexus);
}
unsafe extern "C" fn getaccessor(
    mut dc: *mut adminstuff,
    mut argv: *mut i8,
    mut command: changeaccess,
) {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut save: *mut i8 = 0 as *mut i8;
    let mut who: *mut i8 = 0 as *mut i8;
    let mut any: bool = false;
    any = 0 as i32 != 0;
    s = argv.offset(2 as i32 as isize);
    loop {
        who = strtok_r(s, ks_ws_comma.as_ptr(), &mut save);
        if who.is_null() {
            break;
        }
        checkid(who, 0 as i32);
        getchaccess(dc, who, command);
        any = 1 as i32 != 0;
        s = 0 as *mut i8;
    }
    if !any {
        let mut current_block_6: u64;
        match command as u32 {
            0 => {
                generic_error(
                    0 as *const i8,
                    b"missing login name after option -a\0" as *const u8 as *const i8,
                );
                current_block_6 = 8494331716230207544;
            }
            1 => {
                current_block_6 = 8494331716230207544;
            }
            _ => {
                current_block_6 = 17216689946888361452;
            }
        }
        match current_block_6 {
            8494331716230207544 => {
                getchaccess(dc, 0 as *const i8, command);
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn getmessage(mut dc: *mut adminstuff, mut option: *mut i8) {
    let mut um: *mut u_log = 0 as *mut u_log;
    let mut cb: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut m: *mut i8 = 0 as *mut i8;
    let mut tp: *mut *mut link = &mut (*dc).tp_log;
    if (*tp).is_null() {
        *tp = &mut (*dc).logs;
    }
    m = strchr(option, ':' as i32);
    if m.is_null() {
        generic_error(
            0 as *const i8,
            b"-m option lacks %s\0" as *const u8 as *const i8,
            ks_revno.as_ptr(),
        );
        return;
    }
    let fresh1 = m;
    m = m.offset(1);
    *fresh1 = '\0' as i32 as i8;
    cb = cleanlogmsg(m, strlen(m));
    if cb.size == 0 {
        set_empty_log_message(&mut cb);
    }
    um = zlloc(
        plexus,
        (::core::mem::size_of::<u_log>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut u_log;
    (*um).revno = option;
    (*um).message = cb;
    *tp = extend(*tp, um as *const libc::c_void, plexus);
}
unsafe extern "C" fn getstates(mut dc: *mut adminstuff, mut sp: *mut i8) {
    let mut temp: *const i8 = 0 as *const i8;
    let mut us: *mut u_state = 0 as *mut u_state;
    let mut c: i32 = 0;
    let mut len: size_t = 0;
    let mut tp: *mut *mut link = &mut (*dc).tp_state;
    if (*tp).is_null() {
        *tp = &mut (*dc).states;
    }
    loop {
        sp = sp.offset(1);
        c = *sp as i32;
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32) {
            break;
        }
    }
    temp = checkid(sp, ':' as i32);
    accumulate_range(plexus, sp, temp);
    temp = finish_string(plexus, &mut len);
    sp = sp.offset(len as isize);
    c = *sp as i32;
    while c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32 {
        sp = sp.offset(1);
        c = *sp as i32;
    }
    if c == '\0' as i32 {
        (*dc).headstate_changed = 1 as i32 != 0;
        (*dc).headstate = temp;
        return;
    } else if c != ':' as i32 {
        generic_error(
            0 as *const i8,
            b"missing ':' after state in option -s\0" as *const u8 as *const i8,
        );
        return;
    }
    loop {
        sp = sp.offset(1);
        c = *sp as i32;
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\n' as i32) {
            break;
        }
    }
    us = zlloc(
        plexus,
        (::core::mem::size_of::<u_state>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut u_state;
    (*us).status = temp;
    (*us).revno = sp;
    *tp = extend(*tp, us as *const libc::c_void, plexus);
}
unsafe extern "C" fn putdelrev(
    mut b: *const i8,
    mut e: *const i8,
    mut sawsep: bool,
    mut data: *mut libc::c_void,
) {
    let mut dc: *mut adminstuff = data as *mut adminstuff;
    if !((*dc).delrev.strt).is_null() || !((*dc).delrev.end).is_null() {
        generic_warn(
            0 as *const i8,
            b"ignoring spurious `-o' range `%s:%s'\0" as *const u8 as *const i8,
            if !b.is_null() { b } else { b"(unspecified)\0" as *const u8 as *const i8 },
            if !e.is_null() { e } else { b"(unspecified)\0" as *const u8 as *const i8 },
        );
        return;
    }
    if !sawsep {
        (*dc).delrev.strt = b;
        (*dc).delrev.code = 0 as i32;
    } else if b.is_null() || *b.offset(0 as i32 as isize) == 0 {
        (*dc).delrev.strt = e;
        (*dc).delrev.code = 1 as i32;
    } else if *e.offset(0 as i32 as isize) == 0 {
        (*dc).delrev.strt = b;
        (*dc).delrev.code = 2 as i32;
    } else {
        (*dc).delrev.strt = b;
        (*dc).delrev.end = e;
        (*dc).delrev.code = 3 as i32;
    };
}
unsafe extern "C" fn scanlogtext(
    mut dc: *mut adminstuff,
    mut es: *mut editstuff,
    mut ls: *mut *mut wlink,
    mut delta: *mut delta,
    mut edit: bool,
) {
    let mut nextdelta: *const delta = 0 as *const delta;
    let mut from: *mut fro = (*top).flow.from;
    let mut to: *mut FILE = 0 as *mut FILE;
    let mut log: *mut atat = 0 as *mut atat;
    let mut text: *mut atat = 0 as *mut atat;
    let mut range: range = range { beg: 0, end: 0 };
    loop {
        if (*ls).is_null() {
            return;
        }
        (*top).flow.to = 0 as *mut FILE;
        to = (*top).flow.to;
        nextdelta = (**ls).entry as *const delta;
        log = (*nextdelta).log;
        text = (*nextdelta).text;
        range.beg = (*nextdelta).neck;
        if (*nextdelta).selector {
            (*top).flow.to = (*top).flow.rewr;
            to = (*top).flow.to;
            range.end = (*log).beg;
            fro_spew_partial(to, from, &mut range);
        }
        if nextdelta == (*dc).cuttail {
            if ((*delta).pretty_log.string).is_null() {
                (*delta).pretty_log = string_from_atat(single, log);
                (*delta).pretty_log = cleanlogmsg(
                    (*delta).pretty_log.string,
                    (*delta).pretty_log.size,
                );
            }
        } else if !((*nextdelta).pretty_log.string).is_null()
            && (*nextdelta).selector as i32 != 0
        {
            putstring(to, (*nextdelta).pretty_log, 1 as i32 != 0);
            newline(to);
        } else if !to.is_null() {
            atat_put(to, log);
        }
        range.beg = *((*log).holes)
            .as_mut_ptr()
            .offset(((*log).count).wrapping_sub(1 as i32 as u64) as isize)
            + 2 as i32 as i64;
        range.end = (*text).beg;
        if !to.is_null() {
            fro_spew_partial(to, from, &mut range);
        }
        if delta == nextdelta as *mut delta {
            break;
        }
        if !to.is_null() {
            atat_put(to, text);
        }
        *ls = (**ls).next;
    }
    fro_move(from, range.end);
    if edit {
        editstring(es, text, 0 as *const delta);
    } else {
        enterstring(es, text);
    };
}
unsafe extern "C" fn rmnewlocklst(
    mut dc: *mut adminstuff,
    mut which: *const i8,
) -> *mut link {
    let mut pt: *mut link = 0 as *mut link;
    let mut pre: *mut *mut link = 0 as *mut *mut link;
    pre = &mut (*dc).newlocks;
    loop {
        pt = *pre;
        if pt.is_null() {
            break;
        }
        if strcmp((*pt).entry as *const i8, which) != 0 {
            pre = &mut (*pt).next;
        } else {
            *pre = (*pt).next;
        }
    }
    return *pre;
}
unsafe extern "C" fn doaccess(mut dc: *mut adminstuff) -> bool {
    let mut changed: bool = 0 as i32 != 0;
    let mut ls: *mut link = 0 as *mut link;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    ls = (*dc).accesses.next;
    while !ls.is_null() {
        let mut ch: *const chaccess = (*ls).entry as *const chaccess;
        let mut login: *const i8 = (*ch).login;
        match (*ch).command as u32 {
            1 => {
                if login.is_null() {
                    if !((*(*top).repository.r).access).is_null() {
                        (*(*top).repository.r).access = 0 as *mut link;
                        changed = 1 as i32 != 0;
                    }
                } else {
                    box_0.next = (*(*top).repository.r).access;
                    tp = &mut box_0;
                    while !((*tp).next).is_null() {
                        if strcmp(login, (*(*tp).next).entry as *const i8) == 0 {
                            (*tp).next = (*(*tp).next).next;
                            changed = 1 as i32 != 0;
                            (*(*top).repository.r).access = box_0.next;
                            break;
                        } else {
                            tp = (*tp).next;
                        }
                    }
                }
            }
            0 => {
                box_0.next = (*(*top).repository.r).access;
                tp = &mut box_0;
                while !((*tp).next).is_null() {
                    if strcmp(login, (*(*tp).next).entry as *const i8) == 0 {
                        break;
                    }
                    tp = (*tp).next;
                }
                if ((*tp).next).is_null() {
                    extend(tp, login as *const libc::c_void, single);
                    changed = 1 as i32 != 0;
                    (*(*top).repository.r).access = box_0.next;
                }
            }
            _ => {}
        }
        ls = (*ls).next;
    }
    return changed;
}
unsafe extern "C" fn sendmail(
    mut Delta: *const i8,
    mut who: *const i8,
    mut suppress_mail: bool,
) -> bool {
    complain(
        b"Revision %s is already locked by %s.\n\0" as *const u8 as *const i8,
        Delta,
        who,
    );
    if suppress_mail {
        return 1 as i32 != 0;
    }
    if !yesorno(
        0 as i32 != 0,
        b"Do you want to break the lock\0" as *const u8 as *const i8,
    ) {
        return 0 as i32 != 0;
    }
    generic_warn(
        0 as *const i8,
        b"Mail notification of broken locks is not available.\0" as *const u8
            as *const i8,
    );
    generic_warn(
        0 as *const i8,
        b"Please tell `%s' why you broke the lock.\0" as *const u8 as *const i8,
        who,
    );
    return 1 as i32 != 0;
}
unsafe extern "C" fn breaklock(
    mut delta: *const delta,
    mut suppress_mail: bool,
) -> bool {
    let mut rl: *const rcslock = 0 as *const rcslock;
    let mut box_0: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tp: *mut link = 0 as *mut link;
    let mut num: *const i8 = 0 as *const i8;
    let mut before: *const i8 = 0 as *const i8;
    num = (*delta).num;
    box_0.next = (*(*top).repository.r).locks;
    tp = lock_memq(&mut box_0, 0 as i32 != 0, delta as *const libc::c_void);
    if tp.is_null() {
        generic_error(
            (*top).repository.filename,
            b"no lock set on revision %s\0" as *const u8 as *const i8,
            num,
        );
        return 0 as i32 != 0;
    }
    rl = (*(*tp).next).entry as *const rcslock;
    before = (*rl).login;
    if !caller_login_p(before) && !sendmail(num, before, suppress_mail) {
        generic_error(
            (*top).repository.filename,
            b"revision %s still locked by %s\0" as *const u8 as *const i8,
            num,
            before,
        );
        return 0 as i32 != 0;
    }
    diagnose(b"%s unlocked\0" as *const u8 as *const i8, num);
    lock_drop(&mut box_0, tp);
    return 1 as i32 != 0;
}
unsafe extern "C" fn searchcutpt(
    mut dc: *mut adminstuff,
    mut object: *const i8,
    mut length: i32,
    mut store: *mut wlink,
) -> *mut delta {
    let mut delta: *mut delta = 0 as *mut delta;
    (*dc).cuthead = 0 as *mut delta;
    loop {
        delta = (*store).entry as *mut delta;
        if !(compartial((*delta).num, object, length) != 0) {
            break;
        }
        (*dc).cuthead = delta;
        store = (*store).next;
    }
    return delta;
}
unsafe extern "C" fn branchpoint(mut strt: *mut delta, mut tail: *mut delta) -> bool {
    let mut pt: *mut delta = 0 as *mut delta;
    pt = strt;
    while pt != tail {
        if !((*pt).branches).is_null() {
            generic_error(
                (*top).repository.filename,
                b"can't remove branch point %s\0" as *const u8 as *const i8,
                (*pt).num,
            );
            return 1 as i32 != 0;
        }
        if !(lock_on(pt)).is_null() {
            generic_error(
                (*top).repository.filename,
                b"can't remove locked revision %s\0" as *const u8 as *const i8,
                (*pt).num,
            );
            return 1 as i32 != 0;
        }
        (*pt).selector = 0 as i32 != 0;
        diagnose(b"deleting revision %s\0" as *const u8 as *const i8, (*pt).num);
        pt = (*pt).ilk;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn removerevs(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut target2: *mut delta = 0 as *mut delta;
    let mut temp: *mut delta = 0 as *mut delta;
    let mut ls: *mut wlink = 0 as *mut wlink;
    let mut length: i32 = 0;
    let mut different: bool = false;
    if !fully_numeric(&mut numrev, (*dc).delrev.strt, 0 as *mut fro) {
        return 0 as i32 != 0;
    }
    target = gr_revno(numrev.string, &mut ls);
    if target.is_null() {
        return 0 as i32 != 0;
    }
    different = !(0 as i32 == cmpnum((*target).num, numrev.string));
    length = countnumflds(numrev.string);
    if (*dc).delrev.code == 0 as i32 {
        if length & 1 as i32 != 0 {
            temp = searchcutpt(dc, (*target).num, length + 1 as i32, ls);
        } else if different {
            generic_error(
                (*top).repository.filename,
                b"Revision %s doesn't exist.\0" as *const u8 as *const i8,
                numrev.string,
            );
            return 0 as i32 != 0;
        } else {
            temp = searchcutpt(dc, numrev.string, length, ls);
        }
        (*dc).cuttail = (*target).ilk;
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as i32 != 0;
        }
        (*dc).delstrt = temp;
        return 1 as i32 != 0;
    }
    if length & 1 as i32 != 0 {
        generic_error(
            (*top).repository.filename,
            b"invalid branch range %s after -o\0" as *const u8 as *const i8,
            numrev.string,
        );
        return 0 as i32 != 0;
    }
    if (*dc).delrev.code == 1 as i32 {
        if length > 2 as i32 {
            temp = searchcutpt(dc, (*target).num, length - 1 as i32, ls);
            (*dc).cuttail = (*target).ilk;
        } else {
            temp = searchcutpt(dc, (*target).num, length, ls);
            (*dc).cuttail = target;
            while !((*dc).cuttail).is_null()
                && 0 as i32 == cmpnumfld((*target).num, (*(*dc).cuttail).num, 1 as i32)
            {
                (*dc).cuttail = (*(*dc).cuttail).ilk;
            }
        }
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as i32 != 0;
        }
        (*dc).delstrt = temp;
        return 1 as i32 != 0;
    }
    if (*dc).delrev.code == 2 as i32 {
        if length == 2 as i32 {
            temp = searchcutpt(dc, (*target).num, 1 as i32, ls);
            (*dc).cuttail = if different as i32 != 0 { target } else { (*target).ilk };
        } else {
            if different {
                (*dc).cuthead = target;
                temp = (*target).ilk;
                if temp.is_null() {
                    return 0 as i32 != 0;
                }
            } else {
                temp = searchcutpt(dc, (*target).num, length, ls);
            }
            gr_revno((take(0 as i32 as size_t, (*temp).num)).string, &mut ls);
        }
        if branchpoint(temp, (*dc).cuttail) {
            (*dc).cuttail = 0 as *mut delta;
            return 0 as i32 != 0;
        }
        (*dc).delstrt = temp;
        return 1 as i32 != 0;
    }
    if !fully_numeric(&mut numrev, (*dc).delrev.end, 0 as *mut fro) {
        return 0 as i32 != 0;
    }
    if length != countnumflds(numrev.string)
        || length > 2 as i32
            && compartial(numrev.string, (*target).num, length - 1 as i32) != 0
    {
        generic_error(
            (*top).repository.filename,
            b"invalid revision range %s-%s\0" as *const u8 as *const i8,
            (*target).num,
            numrev.string,
        );
        return 0 as i32 != 0;
    }
    target2 = gr_revno(numrev.string, &mut ls);
    if target2.is_null() {
        return 0 as i32 != 0;
    }
    if length > 2 as i32 {
        if (0 as i32) < cmpnum((*target).num, (*target2).num) {
            different = !(0 as i32 == cmpnum((*target2).num, numrev.string));
            temp = target;
            target = target2;
            target2 = temp;
        }
        if different {
            if 0 as i32 == cmpnum((*target).num, (*target2).num) {
                generic_error(
                    (*top).repository.filename,
                    b"Revisions %s-%s don't exist.\0" as *const u8 as *const i8,
                    (*dc).delrev.strt,
                    (*dc).delrev.end,
                );
                return 0 as i32 != 0;
            }
            (*dc).cuthead = target;
            temp = (*target).ilk;
        } else {
            temp = searchcutpt(dc, (*target).num, length, ls);
        }
        (*dc).cuttail = (*target2).ilk;
    } else {
        if 0 as i32 > cmpnum((*target).num, (*target2).num) {
            temp = target;
            target = target2;
            target2 = temp;
        } else {
            different = !(0 as i32 == cmpnum((*target2).num, numrev.string));
        }
        if different {
            if 0 as i32 == cmpnum((*target).num, (*target2).num) {
                generic_error(
                    (*top).repository.filename,
                    b"Revisions %s-%s don't exist.\0" as *const u8 as *const i8,
                    (*dc).delrev.strt,
                    (*dc).delrev.end,
                );
                return 0 as i32 != 0;
            }
            (*dc).cuttail = target2;
        } else {
            (*dc).cuttail = (*target2).ilk;
        }
        temp = searchcutpt(dc, (*target).num, length, ls);
    }
    if branchpoint(temp, (*dc).cuttail) {
        (*dc).cuttail = 0 as *mut delta;
        return 0 as i32 != 0;
    }
    (*dc).delstrt = temp;
    return 1 as i32 != 0;
}
unsafe extern "C" fn doassoc(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut p: *const i8 = 0 as *const i8;
    let mut changed: bool = 0 as i32 != 0;
    let mut cur: *mut link = (*dc).assocs.next;
    while !cur.is_null() {
        let mut u: *const u_symdef = (*cur).entry as *const u_symdef;
        let mut ssymbol: *const i8 = (*u).u.meaningful;
        let mut under: *const i8 = (*u).u.underlying;
        if under.is_null() {
            let mut box_0: link = link {
                entry: 0 as *const libc::c_void,
                next: 0 as *mut link,
            };
            let mut tp: *mut link = 0 as *mut link;
            let mut d: *const symdef = 0 as *const symdef;
            box_0.next = (*(*top).repository.r).symbols;
            tp = &mut box_0;
            while !((*tp).next).is_null() {
                d = (*(*tp).next).entry as *const symdef;
                if strcmp(ssymbol, (*d).meaningful) == 0 {
                    (*tp).next = (*(*tp).next).next;
                    changed = 1 as i32 != 0;
                    break;
                } else {
                    tp = (*tp).next;
                }
            }
            (*(*top).repository.r).symbols = box_0.next;
            if d.is_null() {
                generic_warn(
                    (*top).repository.filename,
                    b"can't delete nonexisting symbol %s\0" as *const u8 as *const i8,
                    ssymbol,
                );
            }
        } else {
            if *under.offset(0 as i32 as isize) != 0 {
                p = if fully_numeric(&mut numrev, under, 0 as *mut fro) as i32 != 0 {
                    numrev.string
                } else {
                    0 as *const i8
                };
            } else {
                p = tiprev();
                if p.is_null() {
                    generic_error(
                        (*top).repository.filename,
                        b"no latest revision to associate with symbol %s\0" as *const u8
                            as *const i8,
                        ssymbol,
                    );
                }
            }
            if !p.is_null() {
                changed = (changed as i32 | addsymbol(p, ssymbol, (*u).override_0))
                    as bool;
            }
        }
        cur = (*cur).next;
    }
    return changed;
}
unsafe extern "C" fn setlock(mut dc: *mut adminstuff, mut rev: *const i8) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    let mut r: i32 = 0;
    if fully_numeric(&mut numrev, rev, 0 as *mut fro) {
        target = gr_revno(numrev.string, &mut (*dc).deltas);
        if !target.is_null() {
            if countnumflds(numrev.string) & 1 as i32 == 0
                && !(0 as i32 == cmpnum((*target).num, numrev.string))
            {
                generic_error(
                    (*top).repository.filename,
                    b"can't lock nonexisting revision %s\0" as *const u8 as *const i8,
                    numrev.string,
                );
            } else {
                r = addlock_maybe(target, 0 as i32 != 0, 0 as i32 != 0);
                if r < 0 as i32 && breaklock(target, (*dc).suppress_mail) as i32 != 0 {
                    r = addlock_maybe(target, 0 as i32 != 0, 1 as i32 != 0);
                }
                if 0 as i32 <= r {
                    if r != 0 {
                        diagnose(
                            b"%s locked\0" as *const u8 as *const i8,
                            (*target).num,
                        );
                    }
                    return r != 0;
                }
            }
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn dolocks(mut dc: *mut adminstuff) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut lockpt: *const link = 0 as *const link;
    let mut target: *mut delta = 0 as *mut delta;
    let mut tip: *mut delta = (*top).repository.tip;
    let mut changed: bool = 0 as i32 != 0;
    let mut bye: *const i8 = 0 as *const i8;
    if (*dc).unlockcaller {
        if !tip.is_null() {
            let mut locks: *mut link = (*(*top).repository.r).locks;
            if !locks.is_null() {
                match findlock(1 as i32 != 0, &mut target) {
                    0 => {
                        let mut rl: *const rcslock = (*locks).entry as *const rcslock;
                        changed = (changed as i32
                            | breaklock((*rl).delta, (*dc).suppress_mail) as i32)
                            as bool;
                    }
                    1 => {
                        diagnose(
                            b"%s unlocked\0" as *const u8 as *const i8,
                            (*target).num,
                        );
                        changed = 1 as i32 != 0;
                    }
                    _ => {}
                }
            } else {
                generic_warn(
                    (*top).repository.filename,
                    b"No locks are set.\0" as *const u8 as *const i8,
                );
            }
        } else {
            generic_warn(
                (*top).repository.filename,
                b"can't unlock an empty tree\0" as *const u8 as *const i8,
            );
        }
    }
    lockpt = (*dc).byelocks;
    while !lockpt.is_null() {
        bye = (*lockpt).entry as *const i8;
        if fully_numeric(&mut numrev, bye, 0 as *mut fro) {
            target = gr_revno(numrev.string, &mut (*dc).deltas);
            if !target.is_null() {
                if countnumflds(numrev.string) & 1 as i32 == 0
                    && !(0 as i32 == cmpnum((*target).num, numrev.string))
                {
                    generic_error(
                        (*top).repository.filename,
                        b"can't unlock nonexisting revision %s\0" as *const u8
                            as *const i8,
                        bye,
                    );
                } else {
                    changed = (changed as i32
                        | breaklock(target, (*dc).suppress_mail) as i32) as bool;
                }
            }
        }
        lockpt = (*lockpt).next;
    }
    lockpt = (*dc).newlocks;
    while !lockpt.is_null() {
        changed = (changed as i32 | setlock(dc, (*lockpt).entry as *const i8) as i32)
            as bool;
        lockpt = (*lockpt).next;
    }
    if (*dc).lockhead {
        let mut defbr: *const i8 = (*(*top).repository.r).branch;
        if !defbr.is_null() {
            changed = (changed as i32 | setlock(dc, defbr) as i32) as bool;
        } else if !tip.is_null() {
            changed = (changed as i32 | setlock(dc, (*tip).num) as i32) as bool;
        } else {
            generic_warn(
                (*top).repository.filename,
                b"can't lock an empty tree\0" as *const u8 as *const i8,
            );
        }
    }
    return changed;
}
unsafe extern "C" fn domessages(mut dc: *mut adminstuff) -> bool {
    let mut target: *mut delta = 0 as *mut delta;
    let mut changed: bool = 0 as i32 != 0;
    let mut ls: *mut link = (*dc).logs.next;
    while !ls.is_null() {
        let mut um: *const u_log = (*ls).entry as *const u_log;
        let mut numrev: cbuf = cbuf {
            string: 0 as *const i8,
            size: 0,
        };
        if fully_numeric(&mut numrev, (*um).revno, 0 as *mut fro) as i32 != 0
            && {
                target = gr_revno(numrev.string, &mut (*dc).deltas);
                !target.is_null()
            }
        {
            (*target).pretty_log = (*um).message;
            changed = 1 as i32 != 0;
        }
        ls = (*ls).next;
    }
    return changed;
}
unsafe extern "C" fn rcs_setstate(
    mut dc: *mut adminstuff,
    mut rev: *const i8,
    mut status: *const i8,
) -> bool {
    let mut numrev: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut target: *mut delta = 0 as *mut delta;
    if fully_numeric(&mut numrev, rev, 0 as *mut fro) {
        target = gr_revno(numrev.string, &mut (*dc).deltas);
        if !target.is_null() {
            if countnumflds(numrev.string) & 1 as i32 == 0
                && !(0 as i32 == cmpnum((*target).num, numrev.string))
            {
                generic_error(
                    (*top).repository.filename,
                    b"can't set state of nonexisting revision %s\0" as *const u8
                        as *const i8,
                    numrev.string,
                );
            } else if strcmp((*target).state, status) != 0 {
                (*target).state = status;
                return 1 as i32 != 0;
            }
        }
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn buildeltatext(
    mut dc: *mut adminstuff,
    mut es: *mut editstuff,
    mut ls: *mut *mut wlink,
    mut deltas: *const wlink,
) -> bool {
    let mut fcut: *mut FILE = 0 as *mut FILE;
    let mut frew: *mut FILE = (*top).flow.rewr;
    fcut = 0 as *mut FILE;
    (*(*dc).cuttail).selector = 0 as i32 != 0;
    scanlogtext(dc, es, ls, (*deltas).entry as *mut delta, 0 as i32 != 0);
    if !((*dc).cuthead).is_null() {
        fcut = tmpfile();
        if fcut.is_null() {
            fatal_sys(b"tmpfile\0" as *const u8 as *const i8);
        }
        while (*deltas).entry != (*dc).cuthead as *mut libc::c_void {
            *ls = (**ls).next;
            deltas = (*deltas).next;
            scanlogtext(dc, es, ls, (*deltas).entry as *mut delta, 1 as i32 != 0);
        }
        snapshotedit(es, fcut);
        rewind(fcut);
        aflush(fcut);
    }
    while (*deltas).entry != (*dc).cuttail as *mut libc::c_void {
        *ls = (**ls).next;
        deltas = (*deltas).next;
        scanlogtext(dc, es, ls, (*deltas).entry as *mut delta, 1 as i32 != 0);
    }
    finishedit(es, 0 as *const delta, 0 as *mut FILE, 1 as i32 != 0);
    Ozclose(&mut (*top).flow.res);
    if !fcut.is_null() {
        let mut diffname: *const i8 = maketemp(0 as i32);
        let mut diffv: [*const i8; 6] = [0 as *const i8; 6];
        let mut diffp: *mut *const i8 = diffv.as_mut_ptr();
        diffp = diffp.offset(1);
        *diffp = prog_diff.as_ptr();
        diffp = diffp.offset(1);
        *diffp = diff_flags.as_ptr();
        if 0 as i32 != 0 && (*top).behavior.kws == kwsub::kwsub_b as i32 {
            diffp = diffp.offset(1);
            *diffp = b"--binary\0" as *const u8 as *const i8;
        }
        diffp = diffp.offset(1);
        *diffp = b"-\0" as *const u8 as *const i8;
        diffp = diffp.offset(1);
        *diffp = (*top).flow.result;
        diffp = diffp.offset(1);
        *diffp = 0 as *const i8;
        if 2 as i32 == runv(fileno(fcut), diffname, diffv.as_mut_ptr()) {
            generic_fatal(
                (*top).repository.filename,
                b"diff failed\0" as *const u8 as *const i8,
            );
        }
        Ozclose(&mut fcut);
        return putdtext((*dc).cuttail, diffname, frew, 1 as i32 != 0);
    } else {
        return putdtext((*dc).cuttail, (*top).flow.result, frew, 0 as i32 != 0)
    };
}
unsafe extern "C" fn buildtree(mut dc: *mut adminstuff) {
    let mut Delta: *mut delta = 0 as *mut delta;
    if !((*dc).cuthead).is_null() {
        if (*(*dc).cuthead).ilk == (*dc).delstrt {
            (*(*dc).cuthead).ilk = (*dc).cuttail;
        } else {
            let mut pt: *mut wlink = (*(*dc).cuthead).branches;
            let mut pre: *mut wlink = pt;
            while !pt.is_null() && (*pt).entry != (*dc).delstrt as *mut libc::c_void {
                pre = pt;
                pt = (*pt).next;
            }
            if !((*dc).cuttail).is_null() {
                (*pt).entry = (*dc).cuttail as *mut libc::c_void;
            } else if pt == pre {
                (*(*dc).cuthead).branches = (*pt).next;
            } else {
                (*pre).next = (*pt).next;
            }
        }
    } else {
        if ((*dc).cuttail).is_null() && !(*top).behavior.quiet {
            if !yesorno(
                0 as i32 != 0,
                b"Do you really want to delete all revisions\0" as *const u8 as *const i8,
            ) {
                generic_error(
                    (*top).repository.filename,
                    b"No revision deleted\0" as *const u8 as *const i8,
                );
                Delta = (*dc).delstrt;
                while !Delta.is_null() {
                    (*Delta).selector = 1 as i32 != 0;
                    Delta = (*Delta).ilk;
                }
                return;
            }
        }
        (*top).repository.tip = (*dc).cuttail;
    };
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: rcs_blurb.as_ptr(),
            help: rcs_help.as_ptr(),
            tyag: (1 as i32) << 3 as i32
                | ((1 as i32) << 2 as i32 | (1 as i32) << 1 as i32),
        };
        init
    }
};
unsafe extern "C" fn rcs_main(
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) -> i32 {
    let mut dc: adminstuff = adminstuff {
        rv: 0,
        deltas: 0 as *mut wlink,
        suppress_mail: false,
        lockhead: false,
        unlockcaller: false,
        newlocks: 0 as *mut link,
        byelocks: 0 as *mut link,
        headstate: 0 as *const i8,
        headstate_changed: false,
        states: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_state: 0 as *mut link,
        accesses: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_access: 0 as *mut link,
        assocs: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_assoc: 0 as *mut link,
        logs: link {
            entry: 0 as *const libc::c_void,
            next: 0 as *mut link,
        },
        tp_log: 0 as *mut link,
        cuthead: 0 as *mut delta,
        cuttail: 0 as *mut delta,
        delstrt: 0 as *mut delta,
        delrev: delrevpair {
            strt: 0 as *const i8,
            end: 0 as *const i8,
            code: 0,
        },
    };
    let mut a: *mut i8 = 0 as *mut i8;
    let mut newargv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut textfile: *mut i8 = 0 as *mut i8;
    let mut branchsym: *const i8 = 0 as *const i8;
    let mut commsyml: *const i8 = 0 as *const i8;
    let mut branchflag: bool = false;
    let mut initflag: bool = false;
    let mut textflag: bool = false;
    let mut changed: i32 = 0;
    let mut expmode: i32 = 0;
    let mut strictlock: bool = false;
    let mut strict_selected: bool = false;
    let mut Ttimeflag: bool = false;
    let mut keepRCStime: bool = false;
    let mut commsymlen: size_t = 0;
    let mut branchnum: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut boxlock: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tplock: *mut link = 0 as *mut link;
    let mut boxrm: link = link {
        entry: 0 as *const libc::c_void,
        next: 0 as *mut link,
    };
    let mut tprm: *mut link = 0 as *mut link;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = cmd;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    memset(
        &mut dc as *mut adminstuff as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<adminstuff>() as u64,
    );
    dc.rv = 0 as i32;
    nosetid();
    textfile = 0 as *mut i8;
    commsyml = textfile;
    branchsym = commsyml;
    strictlock = 0 as i32 != 0;
    branchflag = strictlock;
    commsymlen = 0 as i32 as size_t;
    boxlock.next = dc.newlocks;
    tplock = &mut boxlock;
    boxrm.next = dc.byelocks;
    tprm = &mut boxrm;
    expmode = -(1 as i32);
    textflag = 0 as i32 != 0;
    initflag = textflag;
    strict_selected = 0 as i32 != 0;
    Ttimeflag = 0 as i32 != 0;
    if (1 as i32) < argc
        && *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize) as i32
            != '-' as i32
    {
        generic_warn(
            0 as *const i8,
            b"No options were given; this usage is obsolescent.\0" as *const u8
                as *const i8,
        );
    }
    argc = getRCSINIT(argc, argv, &mut newargv);
    argv = newargv;
    loop {
        argv = argv.offset(1);
        a = *argv;
        argc -= 1;
        if !((0 as i32) < argc
            && {
                let fresh2 = a;
                a = a.offset(1);
                *fresh2 as i32 == '-' as i32
            })
        {
            break;
        }
        let mut current_block_83: u64;
        let fresh3 = a;
        a = a.offset(1);
        match *fresh3 as i32 {
            105 => {
                initflag = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            98 => {
                if branchflag {
                    redefined('b' as i32);
                }
                branchflag = 1 as i32 != 0;
                branchsym = a;
                current_block_83 = 16313536926714486912;
            }
            99 => {
                if !commsyml.is_null() {
                    redefined('c' as i32);
                }
                commsyml = a;
                commsymlen = strlen(a);
                current_block_83 = 16313536926714486912;
            }
            97 => {
                getaccessor(&mut dc, *argv, changeaccess::append);
                current_block_83 = 16313536926714486912;
            }
            65 => {
                if *a == 0 {
                    generic_error(
                        0 as *const i8,
                        b"missing filename after -A\0" as *const u8 as *const i8,
                    );
                } else {
                    *argv = a;
                    if (0 as i32)
                        < pairnames(
                            1 as i32,
                            argv,
                            Some(
                                rcsreadopen as unsafe extern "C" fn(*mut maybe) -> *mut fro,
                            ),
                            1 as i32 != 0,
                            0 as i32 != 0,
                        )
                    {
                        let mut ls: *mut link = (*(*top).repository.r).access;
                        while !ls.is_null() {
                            getchaccess(
                                &mut dc,
                                str_save((*ls).entry as *const i8),
                                changeaccess::append,
                            );
                            ls = (*ls).next;
                        }
                        fro_zclose(&mut (*top).flow.from);
                    }
                }
                current_block_83 = 16313536926714486912;
            }
            101 => {
                getaccessor(&mut dc, *argv, changeaccess::erase);
                current_block_83 = 16313536926714486912;
            }
            108 => {
                if *a == 0 {
                    dc.lockhead = 1 as i32 != 0;
                } else {
                    tplock = extend(tplock, a as *const libc::c_void, plexus);
                }
                current_block_83 = 16313536926714486912;
            }
            117 => {
                if *a == 0 {
                    dc.unlockcaller = 1 as i32 != 0;
                } else {
                    tprm = extend(tprm, a as *const libc::c_void, plexus);
                    dc.newlocks = boxlock.next;
                    tplock = rmnewlocklst(&mut dc, a);
                }
                current_block_83 = 16313536926714486912;
            }
            76 => {
                if strict_selected {
                    if !strictlock {
                        generic_warn(
                            0 as *const i8,
                            b"-U overridden by -L\0" as *const u8 as *const i8,
                        );
                    }
                }
                strictlock = 1 as i32 != 0;
                strict_selected = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            85 => {
                if strict_selected {
                    if strictlock {
                        generic_warn(
                            0 as *const i8,
                            b"-L overridden by -U\0" as *const u8 as *const i8,
                        );
                    }
                }
                strict_selected = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            110 => {
                current_block_83 = 10304134390680427967;
            }
            78 => {
                current_block_83 = 10304134390680427967;
            }
            109 => {
                getmessage(&mut dc, a);
                current_block_83 = 16313536926714486912;
            }
            77 => {
                dc.suppress_mail = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            111 => {
                if !(dc.delrev.strt).is_null() {
                    redefined('o' as i32);
                }
                if *a == 0 {
                    generic_error(
                        0 as *const i8,
                        b"missing revision range after -o\0" as *const u8 as *const i8,
                    );
                } else {
                    parse_revpairs(
                        'o' as i32 as i8,
                        (*argv).offset(2 as i32 as isize),
                        &mut dc as *mut adminstuff as *mut libc::c_void,
                        Some(
                            putdelrev
                                as unsafe extern "C" fn(
                                    *const i8,
                                    *const i8,
                                    bool,
                                    *mut libc::c_void,
                                ) -> (),
                        ),
                    );
                }
                current_block_83 = 16313536926714486912;
            }
            115 => {
                if *a == 0 {
                    generic_error(
                        0 as *const i8,
                        b"state missing after -s\0" as *const u8 as *const i8,
                    );
                } else {
                    getstates(&mut dc, (*argv).offset(1 as i32 as isize));
                }
                current_block_83 = 16313536926714486912;
            }
            116 => {
                textflag = 1 as i32 != 0;
                if *a != 0 {
                    if !textfile.is_null() {
                        redefined('t' as i32);
                    }
                    textfile = a;
                }
                current_block_83 = 16313536926714486912;
            }
            84 => {
                if *a != 0 {
                    current_block_83 = 8716096135224759145;
                } else {
                    Ttimeflag = 1 as i32 != 0;
                    current_block_83 = 16313536926714486912;
                }
            }
            73 => {
                (*top).behavior.interactive = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            113 => {
                (*top).behavior.quiet = 1 as i32 != 0;
                current_block_83 = 16313536926714486912;
            }
            120 => {
                (*top).behavior.pe = a;
                current_block_83 = 16313536926714486912;
            }
            86 => {
                setRCSversion(*argv);
                current_block_83 = 16313536926714486912;
            }
            122 => {
                zone_set(a);
                current_block_83 = 16313536926714486912;
            }
            107 => {
                if 0 as i32 <= expmode {
                    redefined('k' as i32);
                }
                expmode = str2expmode(a);
                if 0 as i32 <= expmode {
                    current_block_83 = 16313536926714486912;
                } else {
                    current_block_83 = 8716096135224759145;
                }
            }
            _ => {
                current_block_83 = 8716096135224759145;
            }
        }
        match current_block_83 {
            10304134390680427967 => {
                if *a == 0 {
                    generic_error(
                        0 as *const i8,
                        b"missing symbolic name after -%c\0" as *const u8 as *const i8,
                        *(*argv).offset(1 as i32 as isize) as i32,
                    );
                } else {
                    getassoclst(&mut dc, (*argv).offset(1 as i32 as isize));
                }
            }
            8716096135224759145 => {
                bad_option(*argv);
            }
            _ => {}
        }
    }
    dc.newlocks = boxlock.next;
    dc.byelocks = boxrm.next;
    if (*top).flow.erroneous {
        cleanup(&mut dc.rv);
    } else if argc < 1 as i32 {
        generic_fatal(0 as *const i8, b"no input file\0" as *const u8 as *const i8);
    } else {
        let mut current_block_182: u64;
        while (0 as i32) < argc {
            let mut tip: *mut delta = 0 as *mut delta;
            let mut defbr: *const i8 = 0 as *const i8;
            let mut repo_stat: *mut stat = 0 as *mut stat;
            let mut newdesc: cbuf = {
                let mut init = cbuf {
                    string: 0 as *const i8,
                    size: 0 as i32 as size_t,
                };
                init
            };
            ffree();
            if initflag {
                match pairnames(
                    argc,
                    argv,
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                    0 as i32 != 0,
                    0 as i32 != 0,
                ) {
                    0 => {
                        current_block_182 = 796174441944384681;
                    }
                    1 => {
                        generic_error(
                            (*top).repository.filename,
                            b"already exists\0" as *const u8 as *const i8,
                        );
                        current_block_182 = 796174441944384681;
                    }
                    -1 | _ => {
                        current_block_182 = 5404178929002277151;
                    }
                }
            } else {
                match pairnames(
                    argc,
                    argv,
                    Some(rcswriteopen as unsafe extern "C" fn(*mut maybe) -> *mut fro),
                    1 as i32 != 0,
                    0 as i32 != 0,
                ) {
                    -1 | 0 => {
                        current_block_182 = 796174441944384681;
                    }
                    1 | _ => {
                        current_block_182 = 5404178929002277151;
                    }
                }
            }
            match current_block_182 {
                5404178929002277151 => {
                    repo_stat = &mut (*top).repository.stat;
                    tip = (*top).repository.tip;
                    defbr = (*(*top).repository.r).branch;
                    diagnose(
                        b"RCS file: %s\0" as *const u8 as *const i8,
                        (*top).repository.filename,
                    );
                    changed = initflag as i32 | textflag as i32;
                    keepRCStime = Ttimeflag;
                    if !initflag {
                        if !checkaccesslist() {
                            current_block_182 = 796174441944384681;
                        } else {
                            current_block_182 = 8225018548522317130;
                        }
                    } else {
                        current_block_182 = 8225018548522317130;
                    }
                    match current_block_182 {
                        796174441944384681 => {}
                        _ => {
                            if strict_selected {
                                changed
                                    |= (*top).behavior.strictly_locking as i32
                                        ^ strictlock as i32;
                                (*top).behavior.strictly_locking = strictlock;
                            }
                            if !commsyml.is_null()
                                && (commsymlen != (*top).repository.log_lead.size
                                    || 0 as i32
                                        != memcmp(
                                            commsyml as *const libc::c_void,
                                            (*top).repository.log_lead.string as *const libc::c_void,
                                            commsymlen,
                                        ))
                            {
                                (*top).repository.log_lead.string = commsyml;
                                (*top).repository.log_lead.size = commsymlen;
                                changed = 1 as i32;
                            }
                            if 0 as i32 <= expmode && (*top).behavior.kws != expmode {
                                (*top).behavior.kws = expmode;
                                changed = 1 as i32;
                            }
                            if branchflag as i32 != 0
                                && fully_numeric(&mut branchnum, branchsym, 0 as *mut fro)
                                    as i32 != 0
                            {
                                if countnumflds(branchnum.string) != 0 {
                                    if !(0 as i32 == cmpnum(defbr, branchnum.string)) {
                                        (*(*top).repository.r).branch = branchnum.string;
                                        defbr = (*(*top).repository.r).branch;
                                        changed = 1 as i32;
                                    }
                                } else if !defbr.is_null() {
                                    (*(*top).repository.r).branch = 0 as *const i8;
                                    defbr = (*(*top).repository.r).branch;
                                    changed = 1 as i32;
                                }
                            }
                            changed |= doaccess(&mut dc) as i32;
                            changed |= doassoc(&mut dc) as i32;
                            changed |= dolocks(&mut dc) as i32;
                            changed |= domessages(&mut dc) as i32;
                            if dc.headstate_changed {
                                if defbr.is_null() {
                                    if tip.is_null() {
                                        generic_warn(
                                            (*top).repository.filename,
                                            b"can't change states in an empty tree\0" as *const u8
                                                as *const i8,
                                        );
                                    } else if strcmp((*tip).state, dc.headstate) != 0 {
                                        (*tip).state = dc.headstate;
                                        changed = 1 as i32;
                                    }
                                } else {
                                    changed
                                        |= rcs_setstate(&mut dc, defbr, dc.headstate) as i32;
                                }
                            }
                            let mut ls_0: *mut link = dc.states.next;
                            while !ls_0.is_null() {
                                let mut us: *const u_state = (*ls_0).entry
                                    as *const u_state;
                                changed
                                    |= rcs_setstate(&mut dc, (*us).revno, (*us).status) as i32;
                                ls_0 = (*ls_0).next;
                            }
                            dc.cuttail = 0 as *mut delta;
                            if !(dc.delrev.strt).is_null()
                                && removerevs(&mut dc) as i32 != 0
                            {
                                if !(dc.cuttail).is_null() {
                                    gr_revno((*dc.cuttail).num, &mut dc.deltas);
                                }
                                buildtree(&mut dc);
                                tip = (*top).repository.tip;
                                changed = 1 as i32;
                                keepRCStime = 0 as i32 != 0;
                            }
                            if !(*top).flow.erroneous {
                                putadmin();
                                if !tip.is_null() {
                                    puttree(tip, (*top).flow.rewr);
                                }
                                putdesc(&mut newdesc, textflag, textfile);
                                if !(dc.delrev.strt).is_null() || !(dc.logs.next).is_null()
                                {
                                    let mut from: *mut fro = (*top).flow.from;
                                    let mut es: *mut editstuff = make_editstuff();
                                    let mut ls_1: *mut wlink = (*(*top).repository.r).deltas;
                                    if (dc.cuttail).is_null()
                                        || buildeltatext(&mut dc, es, &mut ls_1, dc.deltas) as i32
                                            != 0
                                    {
                                        fro_trundling(1 as i32 != 0, from);
                                        if !(dc.cuttail).is_null() {
                                            ls_1 = (*ls_1).next;
                                        }
                                        scanlogtext(
                                            &mut dc,
                                            es,
                                            &mut ls_1,
                                            0 as *mut delta,
                                            0 as i32 != 0,
                                        );
                                        changed = 1 as i32;
                                    }
                                    unmake_editstuff(es);
                                    (*from).verbatim = (*from).end;
                                } else if !((*(*top).repository.r).desc).is_null() {
                                    (*(*top).flow.from).verbatim = *((*(*(*top).repository.r)
                                        .desc)
                                        .holes)
                                        .as_mut_ptr()
                                        .offset(
                                            ((*(*(*top).repository.r).desc).count)
                                                .wrapping_sub(1 as i32 as u64) as isize,
                                        ) + 2 as i32 as i64;
                                }
                                if initflag {
                                    if 0 as i32 > stat((*top).manifestation.filename, repo_stat)
                                    {
                                        changed = -(1 as i32);
                                    }
                                    (*repo_stat).st_nlink = 0 as i32 as __nlink_t;
                                    keepRCStime = 0 as i32 != 0;
                                }
                                if 0 as i32
                                    > donerewrite(changed, file_mtime(keepRCStime, repo_stat))
                                {
                                    break;
                                }
                                diagnose(b"done\0" as *const u8 as *const i8);
                            }
                        }
                    }
                }
                _ => {}
            }
            cleanup(&mut dc.rv);
            argv = argv.offset(1);
            argv;
            argc -= 1;
            argc;
        }
    }
    tempunlink();
    gnurcs_goodbye();
    return dc.rv;
}
static mut rcs_aka: [uint8_t; 16] = [
    3 as i32 as uint8_t,
    4 as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'b' as i32 as uint8_t,
    3 as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    's' as i32 as uint8_t,
    5 as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'n' as i32 as uint8_t,
];
#[no_mangle]
pub static mut ya_rcs: yacmd = unsafe {
    {
        let mut init = yacmd {
            func: Some(
                rcs_main as unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32,
            ),
            aka: rcs_aka.as_ptr(),
            pr: &program as *const program as *mut program,
        };
        init
    }
};