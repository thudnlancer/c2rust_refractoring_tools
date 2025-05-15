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
    fn _IO_getc(__fp: *mut _IO_FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fdopen(__fd: i32, __modes: *const i8) -> *mut FILE;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fseeko(__stream: *mut FILE, __off: __off_t, __whence: i32) -> i32;
    fn ftello(__stream: *mut FILE) -> __off_t;
    fn __fxstat(__ver: i32, __fildes: i32, __stat_buf: *mut stat) -> i32;
    fn awrite(buf: *const i8, chars: size_t, f: *mut FILE);
    fn __errno_location() -> *mut i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
    fn madvise(__addr: *mut libc::c_void, __len: size_t, __advice: i32) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn fd_safer(_: i32) -> i32;
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    fn fatal_syntax(lno: size_t, fmt: *const i8, _: ...);
    fn fatal_sys(who: *const i8);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn zlloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accumulate_nbytes(divvy: *mut divvy, start: *const i8, count: size_t);
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn Ierror();
    fn testIerror(f: *mut FILE);
    fn newline(f: *mut FILE);
    fn access_page(scratch: *mut isr_scratch, filename: *const i8, p: *const i8) -> i8;
    fn isr_do(scratch: *mut isr_scratch, action: isr_actions);
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
pub type __ssize_t = i64;
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
    RM_STDIO = 2,
    RM_MEM = 1,
    RM_MMAP = 0,
}
impl readmethod {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            readmethod::RM_STDIO => 2,
            readmethod::RM_MEM => 1,
            readmethod::RM_MMAP => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> readmethod {
        match value {
            2 => readmethod::RM_STDIO,
            1 => readmethod::RM_MEM,
            0 => readmethod::RM_MMAP,
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
pub struct range {
    pub beg: off_t,
    pub end: off_t,
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
#[inline]
unsafe extern "C" fn fstat(mut __fd: i32, mut __statbuf: *mut stat) -> i32 {
    return __fxstat(1 as i32, __fd, __statbuf);
}
unsafe extern "C" fn mmap_deallocate(mut f: *mut fro) {
    if 0 as i32
        > munmap(
            (*f).base as *mut libc::c_void,
            ((*f).lim).offset_from((*f).base) as i64 as size_t,
        )
    {
        fatal_sys(b"munmap\0" as *const u8 as *const i8);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fro_open(
    mut name: *const i8,
    mut type_0: *const i8,
    mut status: *mut stat,
) -> *mut fro {
    let mut current_block: u64;
    let mut f: *mut fro = 0 as *mut fro;
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut st: stat = stat {
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
    let mut s: off_t = 0;
    let mut fd: i32 = fd_safer(
        open(
            name,
            0 as i32
                | (if 0 as i32 != 0 && !(strchr(type_0, 'b' as i32)).is_null() {
                    0 as i32
                } else {
                    0 as i32
                }),
        ),
    );
    let mut unlimited: bool = -(1 as i32) as i64 == (*top).behavior.mem_limit;
    if 0 as i32 > fd {
        return 0 as *mut fro;
    }
    if status.is_null() {
        status = &mut st;
    }
    if 0 as i32 > fstat(fd, status) {
        fatal_sys(name);
    }
    if !((*status).st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
        generic_error(
            0 as *const i8,
            b"`%s' is not a regular file\0" as *const u8 as *const i8,
            name,
        );
        close(fd);
        *__errno_location() = 22 as i32;
        return 0 as *mut fro;
    }
    s = (*status).st_size;
    f = zlloc(single, ::core::mem::size_of::<fro>() as u64) as *mut fro;
    (*f).end = s;
    (*f).rm = readmethod::from_libc_c_uint(
        (if unlimited as i32 != 0
            || ((*status).st_size >> 10 as i32) < (*top).behavior.mem_limit
        {
            if 7 as i32 != 0 && (*status).st_size != 0 {
                readmethod::RM_MMAP as i32
            } else {
                readmethod::RM_MEM as i32
            }
        } else {
            readmethod::RM_STDIO as i32
        }) as u32,
    );
    '_retry: loop {
        match (*f).rm as u32 {
            0 => {
                if s != (*status).st_size {
                    generic_fatal(
                        0 as *const i8,
                        b"%s: too large\0" as *const u8 as *const i8,
                        name,
                    );
                }
                (*f).stream = 0 as *mut FILE;
                (*f).deallocate = None;
                isr_do((*top).behavior.isr, isr_actions::ISR_CATCHMMAPINTS);
                (*f).base = mmap(
                    0 as *mut libc::c_void,
                    s as size_t,
                    0x1 as i32,
                    0x1 as i32,
                    fd,
                    0 as i32 as __off_t,
                ) as *mut i8;
                if !((*f).base == -(1 as i32) as *mut libc::c_void as *mut i8) {
                    current_block = 11307063007268554308;
                    break;
                }
                if unlimited {
                    (*f).rm = readmethod::RM_MEM;
                } else {
                    fatal_sys(name);
                    current_block = 11307063007268554308;
                    break;
                }
            }
            1 => {
                if s == 0 {
                    (*f).base = 0 as *mut i8;
                    current_block = 11777552016271000781;
                    break;
                } else {
                    let mut r: ssize_t = 0;
                    let mut bufptr: *mut i8 = 0 as *mut i8;
                    let mut bufsiz: size_t = s as size_t;
                    (*f).base = alloc(single, s as size_t) as *mut i8;
                    bufptr = (*f).base;
                    loop {
                        r = read(fd, bufptr as *mut libc::c_void, bufsiz);
                        if 0 as i32 as i64 > r {
                            if unlimited {
                                (*f).rm = readmethod::RM_STDIO;
                                continue '_retry;
                            } else {
                                fatal_sys(name);
                            }
                        }
                        if r == 0 {
                            s = (s as u64).wrapping_sub(bufsiz) as off_t as off_t;
                            (*status).st_size = s;
                            bufsiz = 0 as i32 as size_t;
                        } else {
                            bufptr = bufptr.offset(r as isize);
                            bufsiz = (bufsiz as u64).wrapping_sub(r as u64) as size_t
                                as size_t;
                        }
                        if !(bufsiz != 0) {
                            break;
                        }
                    }
                    if !(0 as i32 as i64 > lseek(fd, 0 as i32 as __off_t, 0 as i32)) {
                        current_block = 11777552016271000781;
                        break;
                    }
                    if unlimited {
                        (*f).rm = readmethod::RM_STDIO;
                    } else {
                        fatal_sys(name);
                        current_block = 11777552016271000781;
                        break;
                    }
                }
            }
            2 => {
                stream = fdopen(fd, type_0);
                if stream.is_null() {
                    fatal_sys(name);
                }
                (*f).stream = stream;
                current_block = 6072622540298447352;
                break;
            }
            _ => {
                current_block = 6072622540298447352;
                break;
            }
        }
    }
    match current_block {
        11307063007268554308 => {
            access_page((*top).behavior.isr, name, (*f).base);
            (*f).deallocate = Some(
                mmap_deallocate as unsafe extern "C" fn(*mut fro) -> (),
            );
            (*f).ptr = (*f).base;
            (*f).lim = ((*f).base).offset(s as isize);
            fro_trundling(1 as i32 != 0, f);
        }
        11777552016271000781 => {
            (*f).ptr = (*f).base;
            (*f).lim = ((*f).base).offset(s as isize);
        }
        _ => {}
    }
    (*f).fd = fd;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn fro_close(mut f: *mut fro) {
    let mut res: i32 = -(1 as i32);
    if f.is_null() {
        return;
    }
    match (*f).rm as u32 {
        0 | 1 => {
            if ((*f).deallocate).is_some() {
                (Some(((*f).deallocate).expect("non-null function pointer")))
                    .expect("non-null function pointer")(f);
            }
            (*f).base = 0 as *mut i8;
            res = close((*f).fd);
        }
        2 => {
            res = fclose((*f).stream);
        }
        _ => {}
    }
    if res != 0 {
        Ierror();
    }
    (*f).fd = -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn fro_zclose(mut p: *mut *mut fro) {
    fro_close(*p);
    *p = 0 as *mut fro;
}
#[no_mangle]
pub unsafe extern "C" fn fro_tello(mut f: *mut fro) -> off_t {
    let mut rv: off_t = 0 as i32 as off_t;
    match (*f).rm as u32 {
        0 | 1 => {
            rv = ((*f).ptr).offset_from((*f).base) as i64;
        }
        2 => {
            rv = ftello((*f).stream);
        }
        _ => {}
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn fro_move(mut f: *mut fro, mut change: off_t) {
    match (*f).rm as u32 {
        0 | 1 => {
            (*f).ptr = (if 0 as i32 as i64 > change { (*f).ptr } else { (*f).base })
                .offset(change as isize);
        }
        2 => {
            if 0 as i32
                > fseeko(
                    (*f).stream,
                    change,
                    (if 0 as i32 as i64 > change { 1 as i32 } else { 0 as i32 }),
                )
            {
                Ierror();
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn fro_try_getbyte(mut c: *mut i32, mut f: *mut fro) -> bool {
    match (*f).rm as u32 {
        0 | 1 => {
            if (*f).ptr == (*f).lim {
                return 1 as i32 != 0;
            }
            let fresh0 = (*f).ptr;
            (*f).ptr = ((*f).ptr).offset(1);
            *c = *fresh0 as i32;
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut maybe: i32 = _IO_getc(stream);
            if -(1 as i32) == maybe {
                testIerror(stream);
                return 1 as i32 != 0;
            }
            *c = maybe;
        }
        _ => {}
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fro_must_getbyte(mut c: *mut i32, mut f: *mut fro) {
    match (*f).rm as u32 {
        0 | 1 => {
            if (*f).ptr == (*f).lim {
                fatal_syntax(
                    0 as i32 as size_t,
                    b"unexpected end of file\0" as *const u8 as *const i8,
                );
            }
            let fresh1 = (*f).ptr;
            (*f).ptr = ((*f).ptr).offset(1);
            *c = *fresh1 as i32;
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut maybe: i32 = _IO_getc(stream);
            if -(1 as i32) == maybe {
                testIerror(stream);
                fatal_syntax(
                    0 as i32 as size_t,
                    b"unexpected end of file\0" as *const u8 as *const i8,
                );
            }
            *c = maybe;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn fro_trundling(mut sequential: bool, mut f: *mut fro) {
    match (*f).rm as u32 {
        0 => {
            madvise(
                (*f).base as *mut libc::c_void,
                ((*f).lim).offset_from((*f).base) as i64 as size_t,
                if sequential as i32 != 0 { 2 as i32 } else { 0 as i32 },
            );
        }
        1 | 2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn fro_spew_partial(
    mut to: *mut FILE,
    mut f: *mut fro,
    mut r: *mut range,
) {
    match (*f).rm as u32 {
        0 | 1 => {
            awrite(
                ((*f).base).offset((*r).beg as isize),
                ((*r).end - (*r).beg) as size_t,
                to,
            );
            if (*f).end == (*r).end {
                (*f).ptr = (*f).lim;
            }
        }
        2 => {
            let mut buf: [i8; 65536] = [0; 65536];
            let mut count: size_t = 0;
            let mut pos: off_t = (*r).beg;
            fseeko((*f).stream, pos, 0 as i32);
            while pos < (*r).end {
                count = fread(
                    buf.as_mut_ptr() as *mut libc::c_void,
                    ::core::mem::size_of::<i8>() as u64,
                    (if pos < (*r).end - (8 as i32 * 8192 as i32) as i64 {
                        (8 as i32 * 8192 as i32) as i64
                    } else {
                        (*r).end - pos
                    }) as size_t,
                    (*f).stream,
                );
                if count == 0 {
                    testIerror((*f).stream);
                    return;
                }
                awrite(buf.as_mut_ptr(), count, to);
                pos = (pos as u64).wrapping_add(count) as off_t as off_t;
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn fro_spew(mut f: *mut fro, mut to: *mut FILE) {
    let mut finish: range = {
        let mut init = range {
            beg: (*f).verbatim,
            end: (*f).end,
        };
        init
    };
    fro_spew_partial(to, f, &mut finish);
    (*f).verbatim = (*f).end;
}
#[no_mangle]
pub unsafe extern "C" fn string_from_atat(
    mut space: *mut divvy,
    mut atat: *const atat,
) -> cbuf {
    let mut f: *mut fro = (*atat).from;
    let mut count: size_t = (*atat).count;
    let mut cb: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut i: size_t = 0;
    match (*f).rm as u32 {
        0 | 1 => {
            i = 0 as i32 as size_t;
            while i < count {
                let mut rbeg: off_t = 1 as i32 as i64
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(i.wrapping_sub(1 as i32 as u64) as isize)
                    } else {
                        (*atat).beg
                    });
                let mut beg: *const i8 = ((*f).base).offset(rbeg as isize);
                let mut len: off_t = *((*atat).holes).as_ptr().offset(i as isize) - rbeg;
                while (9223372036854775807 as i64) < len {
                    accumulate_nbytes(space, beg, 9223372036854775807 as i64 as size_t);
                    len -= 9223372036854775807 as i64;
                    beg = beg.offset(9223372036854775807 as i64 as isize);
                }
                accumulate_nbytes(space, beg, len as size_t);
                i = i.wrapping_add(1);
                i;
            }
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut was: off_t = ftello(stream);
            i = 0 as i32 as size_t;
            while i < count {
                let mut pos: off_t = 1 as i32 as i64
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(i.wrapping_sub(1 as i32 as u64) as isize)
                    } else {
                        (*atat).beg
                    });
                fseeko(stream, pos, 0 as i32);
                loop {
                    let fresh2 = pos;
                    pos = pos + 1;
                    if !(fresh2 < *((*atat).holes).as_ptr().offset(i as isize)) {
                        break;
                    }
                    accumulate_byte(space, _IO_getc((*f).stream));
                }
                i = i.wrapping_add(1);
                i;
            }
            fseeko(stream, was, 0 as i32);
        }
        _ => {}
    }
    cb.string = finish_string(space, &mut cb.size);
    return cb;
}
#[no_mangle]
pub unsafe extern "C" fn atat_put(mut to: *mut FILE, mut atat: *const atat) {
    let mut range: range = {
        let mut init = range {
            beg: (*atat).beg,
            end: *((*atat).holes)
                .as_ptr()
                .offset(((*atat).count).wrapping_sub(1 as i32 as u64) as isize)
                + 2 as i32 as i64,
        };
        init
    };
    fro_spew_partial(to, (*atat).from, &mut range);
}
#[no_mangle]
pub unsafe extern "C" fn atat_display(
    mut to: *mut FILE,
    mut atat: *const atat,
    mut ensure_end_nl: bool,
) {
    let mut i: size_t = 0 as i32 as size_t;
    while i < (*atat).count {
        let mut range: range = {
            let mut init = range {
                beg: 1 as i32 as i64
                    + (if i != 0 {
                        *((*atat).holes)
                            .as_ptr()
                            .offset(i.wrapping_sub(1 as i32 as u64) as isize)
                    } else {
                        (*atat).beg
                    }),
                end: *((*atat).holes).as_ptr().offset(i as isize),
            };
            init
        };
        fro_spew_partial(to, (*atat).from, &mut range);
        i = i.wrapping_add(1);
        i;
    }
    if !ensure_end_nl
        || 1 as i32 as u64 == (*atat).count
            && (*atat).beg + 1 as i32 as i64
                == *((*atat).holes).as_ptr().offset(0 as i32 as isize)
    {
        return;
    }
    let mut f: *mut fro = (*atat).from;
    let mut pos: off_t = *((*atat).holes)
        .as_ptr()
        .offset(((*atat).count).wrapping_sub(1 as i32 as u64) as isize)
        - 1 as i32 as i64;
    let mut lc: i8 = '\0' as i32 as i8;
    match (*f).rm as u32 {
        0 | 1 => {
            lc = *((*f).base).offset(pos as isize);
        }
        2 => {
            let mut stream: *mut FILE = (*f).stream;
            let mut was: off_t = ftello(stream);
            fseeko(stream, pos, 0 as i32);
            lc = fgetc(stream) as i8;
            fseeko(stream, was, 0 as i32);
        }
        _ => {}
    }
    if '\n' as i32 != lc as i32 {
        newline(to);
    }
}