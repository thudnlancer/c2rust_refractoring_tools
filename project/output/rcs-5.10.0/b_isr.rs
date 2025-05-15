use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    fn perror(__s: *const i8);
    fn thank_you_and_goodnight(how: i32);
    fn __errno_location() -> *mut i32;
    fn signal(__sig: i32, __handler: __sighandler_t) -> __sighandler_t;
    fn psiginfo(__pinfo: *const siginfo_t, __s: *const i8);
    fn sigemptyset(__set: *mut sigset_t) -> i32;
    fn sigaddset(__set: *mut sigset_t, __signo: i32) -> i32;
    fn sigaction(__sig: i32, __act: *const sigaction, __oact: *mut sigaction) -> i32;
    fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> i32;
    fn write(__fd: i32, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strlen(_: *const i8) -> u64;
    fn strsignal(__sig: i32) -> *mut i8;
    fn fatal_sys(who: *const i8);
    static mut plexus: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn setrid();
}
pub type __uint32_t = u32;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __clock_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type __sig_atomic_t = i32;
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
pub type ssize_t = __ssize_t;
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
pub struct __sigset_t {
    pub __val: [u64; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: i32,
    pub sival_ptr: *mut libc::c_void,
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
pub struct isr_scratch {
    pub held: sig_atomic_t,
    pub level: sig_atomic_t,
    pub bufinfo: siginfo_t,
    pub held_info: *mut siginfo_t,
    pub access_name: *const i8,
    pub catching: C2RustUnnamed_2,
    pub be_quiet: *mut bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub regular: bool,
    pub memory_map: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: i32,
    pub si_errno: i32,
    pub si_code: i32,
    pub __pad0: i32,
    pub _sifields: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _pad: [i32; 28],
    pub _kill: C2RustUnnamed_12,
    pub _timer: C2RustUnnamed_11,
    pub _rt: C2RustUnnamed_10,
    pub _sigchld: C2RustUnnamed_9,
    pub _sigfault: C2RustUnnamed_6,
    pub _sigpoll: C2RustUnnamed_5,
    pub _sigsys: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: i32,
    pub _arch: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_band: i64,
    pub si_fd: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub _addr_bnd: C2RustUnnamed_8,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: i32,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_tid: i32,
    pub si_overrun: i32,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type sig_atomic_t = __sig_atomic_t;
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
    pub prev: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
pub type __sighandler_t = Option<unsafe extern "C" fn(i32) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: i32,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(i32, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: i32,
    pub ss_size: size_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum isr_actions {
    ISR_CATCHMMAPINTS = 3,
    ISR_RESTOREINTS = 2,
    ISR_IGNOREINTS = 1,
    ISR_CATCHINTS = 0,
}
impl isr_actions {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            isr_actions::ISR_CATCHMMAPINTS => 3,
            isr_actions::ISR_RESTOREINTS => 2,
            isr_actions::ISR_IGNOREINTS => 1,
            isr_actions::ISR_CATCHINTS => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> isr_actions {
        match value {
            3 => isr_actions::ISR_CATCHMMAPINTS,
            2 => isr_actions::ISR_RESTOREINTS,
            1 => isr_actions::ISR_IGNOREINTS,
            0 => isr_actions::ISR_CATCHINTS,
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
#[no_mangle]
pub unsafe extern "C" fn maybe_reset_sigchld() {
    signal(17 as i32, None);
}
unsafe extern "C" fn werr(mut s: *const i8) {
    let mut len: ssize_t = 0;
    len = strlen(s) as ssize_t;
    if len == 0 {
        return;
    }
    if len != write(2 as i32, s as *const libc::c_void, len as size_t) {
        thank_you_and_goodnight((*(*top).program).tyag);
    }
}
#[no_mangle]
pub unsafe extern "C" fn complain_signal(mut msg: *const i8, mut signo: i32) {
    werr(msg);
    werr(b": \0" as *const u8 as *const i8);
    werr(strsignal(signo));
    werr(b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn access_page(
    mut scratch: *mut isr_scratch,
    mut filename: *const i8,
    mut p: *const i8,
) -> i8 {
    let mut t: i8 = 0;
    (*scratch).access_name = filename;
    ::core::ptr::write_volatile(&mut t as *mut i8, *p);
    (*scratch).access_name = 0 as *const i8;
    return t;
}
unsafe extern "C" fn ignore(mut scratch: *mut isr_scratch) {
    ::core::ptr::write_volatile(
        &mut (*scratch).level as *mut sig_atomic_t,
        ::core::ptr::read_volatile::<
            sig_atomic_t,
        >(&(*scratch).level as *const sig_atomic_t) + 1,
    );
    ::core::ptr::read_volatile::<sig_atomic_t>(&(*scratch).level as *const sig_atomic_t);
}
unsafe extern "C" fn catchsigaction(
    mut signo: i32,
    mut info: *mut siginfo_t,
    mut uc: *mut libc::c_void,
) {
    let mut scratch: *mut isr_scratch = (*top).behavior.isr;
    let mut from_mmap: bool = 7 as i32 != 0 && 7 as i32 == signo;
    if (*scratch).level != 0 {
        ::core::ptr::write_volatile(&mut (*scratch).held as *mut sig_atomic_t, signo);
        if !info.is_null() {
            (*scratch).bufinfo = *info;
            ::core::ptr::write_volatile(
                &mut (*scratch).held_info as *mut *mut siginfo_t,
                &mut (*scratch).bufinfo,
            );
        }
        return;
    }
    ignore(scratch);
    setrid();
    if !*(*scratch).be_quiet {
        if !(from_mmap as i32 != 0 && !((*scratch).access_name).is_null()) {
            let mut nRCS: *mut i8 = b"\nRCS\0" as *const u8 as *const i8 as *mut i8;
            if from_mmap as i32 != 0 && !info.is_null() && (*info).si_errno != 0 {
                *__errno_location() = (*info).si_errno;
                let fresh0 = nRCS;
                nRCS = nRCS.offset(1);
                perror(fresh0);
            }
            if !info.is_null() {
                psiginfo(info, nRCS);
            } else {
                complain_signal(nRCS, signo);
            }
        }
        werr(b"RCS: \0" as *const u8 as *const i8);
        if from_mmap {
            if !((*scratch).access_name).is_null() {
                werr((*scratch).access_name);
                werr(b": Permission denied.  \0" as *const u8 as *const i8);
            } else {
                werr(
                    b"Was a file changed by some other process?  \0" as *const u8
                        as *const i8,
                );
            }
        }
        werr(b"Cleaning up.\n\0" as *const u8 as *const i8);
    }
    thank_you_and_goodnight((*(*top).program).tyag);
}
unsafe extern "C" fn setup_catchsig(mut count: size_t, mut set: *const i32) {
    let mut i_0: size_t = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_14 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut sig: i32 = 0;
    let mut current_block: u64;
    let mut blocked: sigset_t = sigset_t { __val: [0; 16] };
    sigemptyset(&mut blocked);
    let mut i: size_t = 0 as i32 as size_t;
    loop {
        if !(i < count) {
            current_block = 4644295000439058019;
            break;
        }
        if 0 as i32 > sigaddset(&mut blocked, *set.offset(i as isize)) {
            current_block = 11219620342505264191;
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    match current_block {
        4644295000439058019 => {
            i_0 = 0 as i32 as size_t;
            current_block = 17778012151635330486;
        }
        _ => {}
    }
    loop {
        match current_block {
            11219620342505264191 => {
                fatal_sys(b"signal handling\0" as *const u8 as *const i8);
            }
            _ => {
                if !(i_0 < count) {
                    break;
                }
                act = sigaction {
                    __sigaction_handler: C2RustUnnamed_14 {
                        sa_handler: None,
                    },
                    sa_mask: sigset_t { __val: [0; 16] },
                    sa_flags: 0,
                    sa_restorer: None,
                };
                sig = *set.offset(i_0 as isize);
                if 0 as i32 > sigaction(sig, 0 as *const sigaction, &mut act) {
                    current_block = 11219620342505264191;
                    continue;
                }
                if ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as i32 as libc::intptr_t) != act.__sigaction_handler.sa_handler
                {
                    act.__sigaction_handler.sa_sigaction = Some(
                        catchsigaction
                            as unsafe extern "C" fn(
                                i32,
                                *mut siginfo_t,
                                *mut libc::c_void,
                            ) -> (),
                    );
                    act.sa_flags |= 4 as i32 | 0x8000000 as i32;
                    act.sa_mask = blocked;
                    if 0 as i32 > sigaction(sig, &mut act, 0 as *mut sigaction) {
                        current_block = 11219620342505264191;
                        continue;
                    }
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
        current_block = 17778012151635330486;
    };
}
#[no_mangle]
pub unsafe extern "C" fn isr_init(mut be_quiet: *mut bool) -> *mut isr_scratch {
    let mut scratch: *mut isr_scratch = zlloc(
        plexus,
        (::core::mem::size_of::<isr_scratch>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut isr_scratch;
    let mut ss: stack_t = {
        let mut init = stack_t {
            ss_sp: alloc(plexus, (10 as i32 * 8192 as i32) as size_t),
            ss_flags: 0 as i32,
            ss_size: (10 as i32 * 8192 as i32) as size_t,
        };
        init
    };
    if 0 as i32 > sigaltstack(&mut ss, 0 as *mut stack_t) {
        fatal_sys(b"sigaltstack\0" as *const u8 as *const i8);
    }
    (*scratch).be_quiet = be_quiet;
    return scratch;
}
#[no_mangle]
pub unsafe extern "C" fn isr_do(mut scratch: *mut isr_scratch, mut action: isr_actions) {
    match action as u32 {
        0 => {
            let regular: [i32; 7] = [
                1 as i32,
                2 as i32,
                3 as i32,
                13 as i32,
                15 as i32,
                24 as i32,
                25 as i32,
            ];
            if !(*scratch).catching.regular {
                (*scratch).catching.regular = 1 as i32 != 0;
                setup_catchsig(
                    (::core::mem::size_of::<[i32; 7]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32
                        as size_t,
                    regular.as_ptr(),
                );
            }
        }
        1 => {
            ignore(scratch);
        }
        2 => {
            ::core::ptr::write_volatile(
                &mut (*scratch).level as *mut sig_atomic_t,
                ::core::ptr::read_volatile::<
                    sig_atomic_t,
                >(&(*scratch).level as *const sig_atomic_t) - 1,
            );
            if ::core::ptr::read_volatile::<
                sig_atomic_t,
            >(&(*scratch).level as *const sig_atomic_t) == 0 && (*scratch).held != 0
            {
                catchsigaction(
                    (*scratch).held,
                    (*scratch).held_info,
                    0 as *mut libc::c_void,
                );
            }
        }
        3 => {
            let mmapsigs: [i32; 1] = [7 as i32];
            if 7 as i32 != 0 && !(*scratch).catching.memory_map {
                (*scratch).catching.memory_map = 1 as i32 != 0;
                setup_catchsig(
                    (::core::mem::size_of::<[i32; 1]>() as u64)
                        .wrapping_div(::core::mem::size_of::<i32>() as u64) as i32
                        as size_t,
                    mmapsigs.as_ptr(),
                );
            }
        }
        _ => {}
    };
}