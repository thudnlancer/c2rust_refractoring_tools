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
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    fn un_link(s: *const i8) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn mkstemp(__template: *mut i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn close(__fd: i32) -> i32;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const i8) -> i32;
    fn fd_safer(_: i32) -> i32;
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut plexus: *mut divvy;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn seteid();
    fn setrid();
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
pub struct ephemstuff {
    pub standard: *const i8,
    pub tpnames: *mut sff,
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
#[no_mangle]
pub unsafe extern "C" fn init_ephemstuff() {
    (*top).behavior.sff = zlloc(
        plexus,
        (::core::mem::size_of::<sff>() as u64).wrapping_mul((0 as i32 + 2 as i32) as u64),
    ) as *mut sff;
    (*top).behavior.ephemstuff = zlloc(
        plexus,
        (::core::mem::size_of::<ephemstuff>() as u64).wrapping_mul(1 as i32 as u64),
    ) as *mut ephemstuff;
    (*(*top).behavior.ephemstuff).tpnames = zlloc(
        plexus,
        (::core::mem::size_of::<sff>() as u64).wrapping_mul(5 as i32 as u64),
    ) as *mut sff;
}
unsafe extern "C" fn jam_sff(mut sff: *mut sff, mut prefix: *const i8) {
    let mut fn_0: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut fd: i32 = 0;
    if prefix.is_null() {
        if ((*(*top).behavior.ephemstuff).standard).is_null() {
            let mut dir: *const i8 = 0 as *const i8;
            let mut slash: [i8; 2] = ['/' as i32 as i8, '\0' as i32 as i8];
            if dir.is_null() {
                dir = getenv(b"TMPDIR\0" as *const u8 as *const i8);
            }
            if dir.is_null() {
                dir = getenv(b"TMP\0" as *const u8 as *const i8);
            }
            if dir.is_null() {
                dir = getenv(b"TEMP\0" as *const u8 as *const i8);
            }
            if dir.is_null() {
                dir = b"/tmp\0" as *const u8 as *const i8;
            }
            accf(
                plexus,
                b"%s%s%s\0" as *const u8 as *const i8,
                dir,
                if '/' as i32
                    != *dir.offset((strlen(dir)).wrapping_sub(1 as i32 as u64) as isize)
                        as i32
                {
                    slash.as_mut_ptr()
                } else {
                    b"\0" as *const u8 as *const i8
                },
                (*(*top).program).name,
            );
            (*(*top).behavior.ephemstuff).standard = finish_string(plexus, &mut len);
        }
        prefix = (*(*top).behavior.ephemstuff).standard;
    }
    accf(plexus, b"%sXXXXXX\0" as *const u8 as *const i8, prefix);
    fn_0 = finish_string(plexus, &mut len);
    if '/' as i32 != '/' as i32 {
        let mut end: *mut i8 = fn_0.offset(len as isize).offset(-(6 as i32 as isize));
        let mut lastsep: *mut i8 = strrchr(fn_0, '/' as i32);
        let mut ndfc: *mut i8 = if !lastsep.is_null() {
            lastsep.offset(1 as i32 as isize)
        } else {
            fn_0
        };
        let mut dot: *mut i8 = 0 as *mut i8;
        if ndfc.offset(2 as i32 as isize) < end {
            memset(
                ndfc.offset(2 as i32 as isize) as *mut libc::c_void,
                'X' as i32,
                6 as i32 as u64,
            );
            *dot = '\0' as i32 as i8;
        }
        dot = strchr(ndfc, '.' as i32);
        if !dot.is_null() {
            *dot = ('0' as i32 + getpid() % 10 as i32) as i8;
        }
    }
    fd = fd_safer(mkstemp(fn_0));
    if 0 as i32 > fd {
        generic_fatal(
            0 as *const i8,
            b"could not make temporary file name (template \"%s\")\0" as *const u8
                as *const i8,
            fn_0,
        );
    }
    close(fd);
    (*sff).filename = fn_0;
    (*sff).disposition = maker::real;
}
#[no_mangle]
pub unsafe extern "C" fn maketemp(mut n: i32) -> *const i8 {
    if ((*((*(*top).behavior.ephemstuff).tpnames).offset(n as isize)).filename).is_null()
    {
        jam_sff(
            &mut *((*(*top).behavior.ephemstuff).tpnames).offset(n as isize),
            0 as *const i8,
        );
    }
    return (*((*(*top).behavior.ephemstuff).tpnames).offset(n as isize)).filename;
}
#[no_mangle]
pub unsafe extern "C" fn makedirtemp(mut isworkfile: bool) -> *const i8 {
    let mut sff: *mut sff = (*top).behavior.sff;
    let mut slot: i32 = 0 as i32 + isworkfile as i32;
    jam_sff(
        &mut *sff.offset(slot as isize),
        if isworkfile as i32 != 0 {
            (*top).manifestation.filename
        } else {
            (*top).repository.filename
        },
    );
    return (*sff.offset(slot as isize)).filename;
}
#[no_mangle]
pub unsafe extern "C" fn keepdirtemp(mut name: *const i8) {
    let mut sff: *mut sff = (*top).behavior.sff;
    let mut i: i32 = 0 as i32;
    while i < 0 as i32 + 2 as i32 {
        if name == (*sff.offset(i as isize)).filename {
            (*sff.offset(i as isize)).disposition = maker::notmade;
            return;
        }
        i += 1;
        i;
    }
    generic_fatal(0 as *const i8, b"keepdirtemp\0" as *const u8 as *const i8);
}
unsafe extern "C" fn reap(
    mut count: size_t,
    mut all: *mut sff,
    mut cut: Option<unsafe extern "C" fn(*const i8) -> i32>,
) {
    let mut m: maker = maker::notmade;
    let mut i: size_t = 0 as i32 as size_t;
    while i < count {
        m = (*all.offset(i as isize)).disposition;
        if maker::notmade as i32 as u32 != m as u32 {
            if maker::effective as i32 as u32 == m as u32 {
                seteid();
            }
            cut.expect("non-null function pointer")((*all.offset(i as isize)).filename);
            let ref mut fresh0 = (*all.offset(i as isize)).filename;
            *fresh0 = 0 as *const i8;
            if maker::effective as i32 as u32 == m as u32 {
                setrid();
            }
            (*all.offset(i as isize)).disposition = maker::notmade;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn tempunlink() {
    reap(
        5 as i32 as size_t,
        (*(*top).behavior.ephemstuff).tpnames,
        Some(unlink as unsafe extern "C" fn(*const i8) -> i32),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dirtempunlink() {
    reap(
        (0 as i32 + 2 as i32) as size_t,
        (*top).behavior.sff,
        Some(un_link as unsafe extern "C" fn(*const i8) -> i32),
    );
}