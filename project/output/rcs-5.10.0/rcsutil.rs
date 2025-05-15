use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    fn ORCSerror();
    fn gettime(_: *mut timespec);
    fn __errno_location() -> *mut i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn close(__fd: i32) -> i32;
    fn execv(__path: *const i8, __argv: *const *mut i8) -> i32;
    fn vfork() -> i32;
    fn waitpid(__pid: __pid_t, __stat_loc: *mut i32, __options: i32) -> __pid_t;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    fn unbuffer_standard_error();
    fn vcomplain(fmt: *const i8, args: ::core::ffi::VaList);
    fn complain(fmt: *const i8, _: ...);
    fn diagnose(fmt: *const i8, _: ...);
    fn generic_warn(who: *const i8, fmt: *const i8, _: ...);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut exit_failure: i32;
    fn fatal_sys(who: *const i8);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn make_space(name: *const i8) -> *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn intern(divvy: *mut divvy, s: *const i8, len: size_t) -> *mut i8;
    fn forget(divvy: *mut divvy);
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn pointer_array(divvy: *mut divvy, count: size_t) -> *mut libc::c_void;
    fn close_space(divvy: *mut divvy);
    fn Oerror();
    fn oflush();
    fn init_ephemstuff();
    fn tempunlink();
    fn dirtempunlink();
    fn maybe_reset_sigchld();
    fn complain_signal(msg: *const i8, signo: i32);
    fn isr_init(be_quiet: *mut bool) -> *mut isr_scratch;
    fn display_version(prog: *const program, flags: i32);
    fn set_program_name(argv0: *const i8);
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
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
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
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
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
pub struct maketimestuff {
    pub tzset_already_called: bool,
    pub TZ: *const i8,
    pub time2tm_stash: tm,
    pub t_cache: [time_t; 2],
    pub tm_cache: [tm; 2],
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
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: i64) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
unsafe extern "C" fn exit_diff_trouble(mut fmt: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vcomplain(fmt, args_0.as_va_list());
    complain(b"\n\0" as *const u8 as *const i8);
    exit(2 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn thank_you_and_goodnight(how: i32) {
    if how & (1 as i32) << 3 as i32 != 0 {
        ORCSerror();
    }
    if how & (1 as i32) << 2 as i32 != 0 {
        dirtempunlink();
    }
    if how & (1 as i32) << 1 as i32 != 0 {
        tempunlink();
    }
    exit(if how & (1 as i32) << 0 as i32 != 0 { 1 as i32 } else { exit_failure });
}
#[no_mangle]
pub unsafe extern "C" fn gnurcs_init(mut program: *const program) {
    set_program_name((*program).invoke);
    plexus = make_space(b"plexus\0" as *const u8 as *const i8);
    single = make_space(b"single\0" as *const u8 as *const i8);
    top = zlloc(
        plexus,
        (::core::mem::size_of::<top>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut top;
    unbuffer_standard_error();
    (*top).program = program;
    (*top).behavior.pe = if 0 as i32 != 0 {
        b"\\,v\0" as *const u8 as *const i8
    } else {
        b",v/\0" as *const u8 as *const i8
    };
    (*top).behavior.isr = isr_init(&mut (*top).behavior.quiet);
    init_ephemstuff();
    (*top).behavior.maketimestuff = zlloc(
        plexus,
        (::core::mem::size_of::<maketimestuff>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut maketimestuff;
    gettime(&mut (*top).behavior.now);
    let mut v: *mut i8 = 0 as *mut i8;
    let mut lim: i64 = 0;
    v = getenv(b"RCS_MEM_LIMIT\0" as *const u8 as *const i8);
    (*top).behavior.mem_limit = if !if !v.is_null()
        && *v.offset(0 as i32 as isize) as i32 != 0
    {
        v
    } else {
        0 as *mut i8
    }
        .is_null()
    {
        lim = strtol(v, 0 as *mut *mut i8, 10 as i32);
        if 0 as i32 as i64 > lim { 0 as i32 as i64 } else { lim }
    } else {
        -(1 as i32) as i64
    };
}
#[no_mangle]
pub unsafe extern "C" fn gnurcs_goodbye() {
    top = 0 as *mut top;
    close_space(single);
    single = 0 as *mut divvy;
    close_space(plexus);
    plexus = 0 as *mut divvy;
}
#[no_mangle]
pub unsafe extern "C" fn bad_option(mut option: *const i8) {
    generic_error(
        0 as *const i8,
        b"unknown option: %s\0" as *const u8 as *const i8,
        option,
    );
}
#[no_mangle]
pub unsafe extern "C" fn redefined(mut c: i32) {
    generic_warn(
        0 as *const i8,
        b"redefinition of -%c option\0" as *const u8 as *const i8,
        c,
    );
}
#[no_mangle]
pub unsafe extern "C" fn chk_set_rev(mut rev: *mut *const i8, mut arg: *mut i8) {
    if *arg == 0 {
        return;
    }
    if !(*rev).is_null() {
        generic_warn(
            0 as *const i8,
            b"redefinition of %s\0" as *const u8 as *const i8,
            ks_revno.as_ptr(),
        );
    }
    *rev = arg;
}
#[no_mangle]
pub unsafe extern "C" fn minus_p(mut xrev: *const i8, mut rev: *const i8) -> cbuf {
    let mut rv: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    diagnose(b"retrieving revision %s\0" as *const u8 as *const i8, xrev);
    accf(single, b"-p%s\0" as *const u8 as *const i8, rev);
    rv.string = finish_string(single, &mut rv.size);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn parse_revpairs(
    mut option: i8,
    mut arg: *mut i8,
    mut data: *mut libc::c_void,
    mut put: Option<
        unsafe extern "C" fn(*const i8, *const i8, bool, *mut libc::c_void) -> (),
    >,
) {
    let mut c: i8 = 0;
    let mut separator: i32 = if !(strchr(arg, ':' as i32)).is_null() {
        ':' as i32
    } else {
        '-' as i32
    };
    let mut b: *const i8 = 0 as *const i8;
    let mut e: *const i8 = 0 as *const i8;
    c = *arg;
    if '-' as i32 == separator && !(strchr(arg, '-' as i32)).is_null()
        && 5 as i32 - 5 as i32 <= (*top).behavior.version
    {
        generic_warn(
            0 as *const i8,
            b"`-' is obsolete in `-%c%s'; use `:' instead\0" as *const u8 as *const i8,
            option as i32,
            arg,
        );
    }
    loop {
        while c as i32 == ' ' as i32 || c as i32 == '\t' as i32
            || c as i32 == '\n' as i32
        {
            arg = arg.offset(1);
            c = *arg;
        }
        b = arg;
        loop {
            match c as i32 {
                0 | 32 | 9 | 10 | 44 | 59 => {
                    break;
                }
                58 | 45 => {
                    if c as i32 == separator {
                        break;
                    }
                }
                _ => {}
            }
            arg = arg.offset(1);
            c = *arg;
        }
        *arg = '\0' as i32 as i8;
        while c as i32 == ' ' as i32 || c as i32 == '\t' as i32
            || c as i32 == '\n' as i32
        {
            arg = arg.offset(1);
            c = *arg;
        }
        if c as i32 == separator {
            loop {
                arg = arg.offset(1);
                c = *arg;
                if !(c as i32 == ' ' as i32 || c as i32 == '\t' as i32
                    || c as i32 == '\n' as i32)
                {
                    break;
                }
            }
            e = arg;
            loop {
                match c as i32 {
                    0 | 32 | 9 | 10 | 44 | 59 => {
                        break;
                    }
                    58 | 45 => {
                        if c as i32 == separator {
                            break;
                        }
                    }
                    _ => {}
                }
                arg = arg.offset(1);
                c = *arg;
            }
            *arg = '\0' as i32 as i8;
            put.expect("non-null function pointer")(b, e, 1 as i32 != 0, data);
            while c as i32 == ' ' as i32 || c as i32 == '\t' as i32
                || c as i32 == '\n' as i32
            {
                arg = arg.offset(1);
                c = *arg;
            }
        } else {
            put.expect("non-null function pointer")(b, e, 0 as i32 != 0, data);
        }
        if c == 0 {
            break;
        }
        if c as i32 == ',' as i32 || c as i32 == ';' as i32 {
            arg = arg.offset(1);
            c = *arg;
        } else {
            generic_error(
                0 as *const i8,
                b"missing `,' near `%c%s'\0" as *const u8 as *const i8,
                c as i32,
                arg.offset(1 as i32 as isize),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_empty_log_message(mut cb: *mut cbuf) {
    (*cb).string = b"*** empty log message ***\0" as *const u8 as *const i8;
    (*cb).size = (::core::mem::size_of::<[i8; 26]>() as u64)
        .wrapping_sub(1 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn ffree() {
    forget(single);
}
#[no_mangle]
pub unsafe extern "C" fn str_save(mut s: *const i8) -> *mut i8 {
    return intern(plexus, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn cgetenv(mut name: *const i8) -> *mut i8 {
    let mut p: *mut i8 = 0 as *mut i8;
    p = getenv(name);
    return if !p.is_null() { str_save(p) } else { p };
}
#[no_mangle]
pub unsafe extern "C" fn awrite(
    mut buf: *const i8,
    mut chars: size_t,
    mut f: *mut FILE,
) {
    while (9223372036854775807 as i64 as u64) < chars {
        if fwrite(
            buf as *const libc::c_void,
            ::core::mem::size_of::<i8>() as u64,
            9223372036854775807 as i64 as size_t,
            f,
        ) != 9223372036854775807 as i64 as u64
        {
            Oerror();
        }
        buf = buf.offset(9223372036854775807 as i64 as isize);
        chars = (chars as u64).wrapping_sub(9223372036854775807 as i64 as u64) as size_t
            as size_t;
    }
    if fwrite(buf as *const libc::c_void, ::core::mem::size_of::<i8>() as u64, chars, f)
        != chars
    {
        Oerror();
    }
}
unsafe extern "C" fn movefd(mut old: i32, mut new: i32) -> i32 {
    if 0 as i32 > old || old == new {
        return old;
    }
    new = rpl_fcntl(old, 0 as i32, new);
    return if !(0 as i32 > close(old)) { new } else { -(1 as i32) };
}
unsafe extern "C" fn fdreopen(mut fd: i32, mut file: *const i8, mut flags: i32) -> i32 {
    let mut newfd: i32 = 0;
    close(fd);
    newfd = open(file, flags, 0o400 as i32 | 0o200 as i32);
    return movefd(newfd, fd);
}
#[no_mangle]
pub unsafe extern "C" fn runv(
    mut infd: i32,
    mut outname: *const i8,
    mut args: *mut *const i8,
) -> i32 {
    let mut wstatus: i32 = 0;
    if !(*top).behavior.fixed_SIGCHLD {
        (*top).behavior.fixed_SIGCHLD = 1 as i32 != 0;
        maybe_reset_sigchld();
    }
    oflush();
    let mut pid: pid_t = 0;
    pid = vfork();
    if pid == 0 {
        let mut notfound: *const i8 = 0 as *const i8;
        if infd != -(1 as i32) && 0 as i32 != infd
            && {
                close(0 as i32);
                0 as i32 != rpl_fcntl(infd, 0 as i32, 0 as i32)
            }
        {
            exit_diff_trouble(
                b"%s: I/O redirection failed\0" as *const u8 as *const i8,
                *args.offset(1 as i32 as isize),
            );
        }
        if !outname.is_null() {
            if 0 as i32
                > fdreopen(1 as i32, outname, 0o100 as i32 | 0o1000 as i32 | 0o1 as i32)
            {
                exit_diff_trouble(
                    b"%s: %s: cannot create\0" as *const u8 as *const i8,
                    *args.offset(1 as i32 as isize),
                    outname,
                );
            }
        }
        execv(
            *args.offset(1 as i32 as isize),
            args.offset(1 as i32 as isize) as *mut *mut i8 as *const *mut i8,
        );
        notfound = *args.offset(1 as i32 as isize);
        if *__errno_location() == 8 as i32 {
            notfound = b"/bin/sh\0" as *const u8 as *const i8;
            let ref mut fresh0 = *args.offset(0 as i32 as isize);
            *fresh0 = notfound;
            execv(
                *args.offset(0 as i32 as isize),
                args as *mut *mut i8 as *const *mut i8,
            );
        }
        exit_diff_trouble(b"%s: not found\0" as *const u8 as *const i8, notfound);
    }
    if 0 as i32 > pid {
        fatal_sys(b"fork\0" as *const u8 as *const i8);
    }
    if 0 as i32 > waitpid(pid, &mut wstatus, 0 as i32) {
        fatal_sys(b"waitpid\0" as *const u8 as *const i8);
    }
    if !(wstatus & 0x7f as i32 == 0 as i32) {
        if ((wstatus & 0x7f as i32) + 1 as i32) as libc::c_schar as i32 >> 1 as i32
            > 0 as i32
        {
            complain_signal(*args.offset(1 as i32 as isize), wstatus & 0x7f as i32);
            generic_fatal(
                0 as *const i8,
                b"%s got a fatal signal\0" as *const u8 as *const i8,
                *args.offset(1 as i32 as isize),
            );
        }
        generic_fatal(
            0 as *const i8,
            b"%s failed for unknown reason\0" as *const u8 as *const i8,
            *args.offset(1 as i32 as isize),
        );
    }
    return (wstatus & 0xff00 as i32) >> 8 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn run(
    mut infd: i32,
    mut outname: *const i8,
    mut args: ...
) -> i32 {
    let mut ap: ::core::ffi::VaListImpl;
    let mut rgargs: [*const i8; 20] = [0 as *const i8; 20];
    let mut i: i32 = 0;
    ap = args.clone();
    i = 1 as i32;
    loop {
        let fresh1 = i;
        i = i + 1;
        rgargs[fresh1 as usize] = ap.arg::<*const i8>();
        if (rgargs[fresh1 as usize]).is_null() {
            break;
        }
        if 20 as i32 <= i {
            generic_fatal(
                0 as *const i8,
                b"too many command arguments\0" as *const u8 as *const i8,
            );
        }
    }
    return runv(infd, outname, rgargs.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn setRCSversion(mut str: *const i8) {
    let mut s: *const i8 = str.offset(2 as i32 as isize);
    if *s != 0 {
        let mut v: i32 = 5 as i32;
        if (*top).behavior.version_set {
            redefined('V' as i32);
        }
        (*top).behavior.version_set = 1 as i32 != 0;
        v = 0 as i32;
        while *(*__ctype_b_loc()).offset(*s as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            let fresh2 = s;
            s = s.offset(1);
            v = 10 as i32 * v + *fresh2 as i32 - '0' as i32;
        }
        if *s != 0 {
            generic_error(
                0 as *const i8,
                b"%s isn't a number\0" as *const u8 as *const i8,
                str,
            );
        } else if v < 3 as i32 || (5 as i32) < v {
            generic_error(
                0 as *const i8,
                b"%s out of range %d..%d\0" as *const u8 as *const i8,
                str,
                3 as i32,
                5 as i32,
            );
        }
        (*top).behavior.version = v - 5 as i32;
    } else {
        display_version((*top).program, 1 as i32 | 2 as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn getRCSINIT(
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut newargv: *mut *mut *mut i8,
) -> i32 {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut q: *mut i8 = 0 as *mut i8;
    let mut pp: *mut *mut i8 = 0 as *mut *mut i8;
    let mut n: size_t = 0;
    q = cgetenv(b"RCSINIT\0" as *const u8 as *const i8);
    if q.is_null() {
        *newargv = argv;
    } else {
        n = (argc + 2 as i32) as size_t;
        p = q;
        loop {
            let fresh3 = p;
            p = p.offset(1);
            match *fresh3 as i32 {
                32 | 8 | 12 | 10 | 13 | 9 | 11 => {}
                0 => {
                    break;
                }
                _ => {
                    continue;
                }
            }
            n = n.wrapping_add(1);
            n;
        }
        pp = pointer_array(plexus, n) as *mut *mut i8;
        *newargv = pp;
        let fresh4 = argv;
        argv = argv.offset(1);
        let fresh5 = pp;
        pp = pp.offset(1);
        *fresh5 = *fresh4;
        p = q;
        's_79: loop {
            match *q as i32 {
                0 => {
                    break;
                }
                32 | 8 | 12 | 10 | 13 | 9 | 11 => {
                    q = q.offset(1);
                    q;
                }
                _ => {
                    let fresh6 = pp;
                    pp = pp.offset(1);
                    *fresh6 = p;
                    argc += 1;
                    argc;
                    loop {
                        let fresh7 = q;
                        q = q.offset(1);
                        let fresh8 = p;
                        p = p.offset(1);
                        *fresh8 = *fresh7;
                        match *fresh8 as i32 {
                            0 => {
                                break 's_79;
                            }
                            92 => {}
                            32 | 8 | 12 | 10 | 13 | 9 | 11 => {
                                break;
                            }
                            _ => {
                                continue;
                            }
                        }
                        if *q == 0 {
                            break 's_79;
                        }
                        let fresh9 = q;
                        q = q.offset(1);
                        *p.offset(-(1 as i32) as isize) = *fresh9;
                    }
                    *p.offset(-(1 as i32) as isize) = '\0' as i32 as i8;
                }
            }
        }
        loop {
            let fresh10 = argv;
            argv = argv.offset(1);
            let fresh11 = pp;
            pp = pp.offset(1);
            *fresh11 = *fresh10;
            if (*fresh11).is_null() {
                break;
            }
        }
    }
    return argc;
}
#[no_mangle]
pub unsafe extern "C" fn unspecified_timespec() -> timespec {
    return make_timespec(-(1 as i32) as time_t, 0 as i32 as i64);
}
#[no_mangle]
pub unsafe extern "C" fn file_mtime(mut enable: bool, mut st: *const stat) -> timespec {
    return if enable as i32 != 0 { get_stat_mtime(st) } else { unspecified_timespec() };
}