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
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    static mut top: *mut top;
    fn checksid(id: *const i8);
    fn str_save(s: *const i8) -> *mut i8;
    fn cgetenv(name: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn seteuid(__uid: __uid_t) -> i32;
    fn getlogin_r(__name: *mut i8, __name_len: size_t) -> i32;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut i8,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> i32;
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_sys(who: *const i8);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn prepend(x: *const libc::c_void, ls: *mut link, to: *mut divvy) -> *mut link;
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
pub type uid_t = __uid_t;
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
pub struct rcslock {
    pub login: *const i8,
    pub delta: *mut delta,
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
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
unsafe extern "C" fn ruid() -> uid_t {
    if !(*top).behavior.ruid_cached {
        (*top).behavior.ruid = getuid() as i32;
        (*top).behavior.ruid_cached = 1 as i32 != 0;
    }
    return (*top).behavior.ruid as uid_t;
}
#[no_mangle]
pub unsafe extern "C" fn stat_mine_p(mut st: *mut stat) -> bool {
    return ruid() == (*st).st_uid;
}
unsafe extern "C" fn euid() -> uid_t {
    if !(*top).behavior.euid_cached {
        (*top).behavior.euid = geteuid() as i32;
        (*top).behavior.euid_cached = 1 as i32 != 0;
    }
    return (*top).behavior.euid as uid_t;
}
#[no_mangle]
pub unsafe extern "C" fn currently_setuid_p() -> bool {
    return euid() != ruid();
}
unsafe extern "C" fn set_uid_to(mut u: uid_t) {
    if !currently_setuid_p() {
        return;
    }
    if 0 as i32 > seteuid(u) {
        fatal_sys(b"setuid\0" as *const u8 as *const i8);
    }
    if geteuid() != u {
        if (*top).behavior.already_setuid {
            return;
        }
        (*top).behavior.already_setuid = 1 as i32 != 0;
        generic_fatal(
            0 as *const i8,
            (b"root setuid not supported\0" as *const u8 as *const i8)
                .offset((if u != 0 { 5 as i32 } else { 0 as i32 }) as isize),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn nosetid() {
    (*top).behavior.stick_with_euid = 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn seteid() {
    if !(*top).behavior.stick_with_euid {
        set_uid_to(euid());
    }
}
#[no_mangle]
pub unsafe extern "C" fn setrid() {
    if !(*top).behavior.stick_with_euid {
        set_uid_to(ruid());
    }
}
#[no_mangle]
pub unsafe extern "C" fn getusername(mut suspicious: bool) -> *const i8 {
    if ((*top).behavior.username).is_null() {
        let mut buf: [i8; 8192] = [0; 8192];
        if suspicious as i32 != 0
            || {
                (*top).behavior.username = cgetenv(
                    b"LOGNAME\0" as *const u8 as *const i8,
                );
                ((*top).behavior.username).is_null()
                    && {
                        (*top).behavior.username = cgetenv(
                            b"USER\0" as *const u8 as *const i8,
                        );
                        ((*top).behavior.username).is_null()
                    }
                    && !(0 as i32 == getlogin_r(buf.as_mut_ptr(), 8192 as i32 as size_t)
                        && {
                            (*top).behavior.username = str_save(buf.as_mut_ptr());
                            !((*top).behavior.username).is_null()
                        })
            }
        {
            let mut pwbuf: passwd = passwd {
                pw_name: 0 as *mut i8,
                pw_passwd: 0 as *mut i8,
                pw_uid: 0,
                pw_gid: 0,
                pw_gecos: 0 as *mut i8,
                pw_dir: 0 as *mut i8,
                pw_shell: 0 as *mut i8,
            };
            let mut pw: *mut passwd = 0 as *mut passwd;
            if getpwuid_r(
                ruid(),
                &mut pwbuf,
                buf.as_mut_ptr(),
                8192 as i32 as size_t,
                &mut pw,
            ) != 0 || &mut pwbuf as *mut passwd != pw || ((*pw).pw_name).is_null()
            {
                generic_fatal(
                    0 as *const i8,
                    b"no password entry for userid %d\0" as *const u8 as *const i8,
                    ruid(),
                );
            }
            (*top).behavior.username = str_save((*pw).pw_name);
        }
        checksid((*top).behavior.username);
    }
    return (*top).behavior.username;
}
#[no_mangle]
pub unsafe extern "C" fn getcaller() -> *const i8 {
    return getusername(currently_setuid_p());
}
#[no_mangle]
pub unsafe extern "C" fn caller_login_p(mut login: *const i8) -> bool {
    return strcmp(getcaller(), login) == 0;
}
#[no_mangle]
pub unsafe extern "C" fn lock_memq(
    mut ls: *mut link,
    mut login: bool,
    mut x: *const libc::c_void,
) -> *mut link {
    let mut rl: *const rcslock = 0 as *const rcslock;
    while !((*ls).next).is_null() {
        rl = (*(*ls).next).entry as *const rcslock;
        if if login as i32 != 0 {
            (strcmp(x as *const i8, (*rl).login) == 0) as i32
        } else {
            (x == (*rl).delta as *const libc::c_void) as i32
        } != 0
        {
            return ls;
        }
        ls = (*ls).next;
    }
    return 0 as *mut link;
}
#[no_mangle]
pub unsafe extern "C" fn lock_on(mut delta: *const delta) -> *const rcslock {
    let mut ls: *mut link = (*(*top).repository.r).locks;
    while !ls.is_null() {
        let mut rl: *const rcslock = (*ls).entry as *const rcslock;
        if delta == (*rl).delta {
            return rl;
        }
        ls = (*ls).next;
    }
    return 0 as *const rcslock;
}
#[no_mangle]
pub unsafe extern "C" fn lock_drop(mut box_0: *mut link, mut tp: *mut link) {
    let mut rl: *const rcslock = (*(*tp).next).entry as *const rcslock;
    (*(*rl).delta).lockedby = 0 as *const i8;
    (*tp).next = (*(*tp).next).next;
    (*(*top).repository.r).locks = (*box_0).next;
}
#[no_mangle]
pub unsafe extern "C" fn addlock_maybe(
    mut delta: *mut delta,
    mut selfsame: bool,
    mut verbose: bool,
) -> i32 {
    let mut rl: *mut rcslock = 0 as *mut rcslock;
    let mut was: *const rcslock = lock_on(delta);
    if !was.is_null() {
        if !selfsame && caller_login_p((*was).login) as i32 != 0 {
            return 0 as i32;
        }
        if verbose {
            generic_error(
                (*top).repository.filename,
                b"Revision %s is already locked by %s.\0" as *const u8 as *const i8,
                (*delta).num,
                (*was).login,
            );
        }
        return -(1 as i32);
    }
    rl = alloc(single, ::core::mem::size_of::<rcslock>() as u64) as *mut rcslock;
    (*delta).lockedby = getcaller();
    (*rl).login = (*delta).lockedby;
    (*rl).delta = delta;
    (*(*top).repository.r).locks = prepend(
        rl as *const libc::c_void,
        (*(*top).repository.r).locks,
        single,
    );
    return 1 as i32;
}