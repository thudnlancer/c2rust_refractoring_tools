use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type wlink;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    static mut top: *mut top;
    static tiny_ciklog: tinysym;
    fn looking_at(sym: *const tinysym, start: *const i8) -> bool;
    fn recognize_keyword(string: *const i8, found: *mut pool_found) -> bool;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn fatal_sys(who: *const i8);
    fn fro_open(filename: *const i8, type_0: *const i8, status: *mut stat) -> *mut fro;
    fn fro_close(f: *mut fro);
    fn fro_try_getbyte(c: *mut i32, f: *mut fro) -> bool;
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
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_found {
    pub i: i32,
    pub sym: *const tinysym,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum markers {
    Author,
    Date,
    Header,
    Id,
    Locker,
    Log,
    Name,
    RCSfile,
    Revision,
    Source,
    State,
}
impl markers {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            markers::Author => 0,
            markers::Date => 1,
            markers::Header => 2,
            markers::Id => 3,
            markers::Locker => 4,
            markers::Log => 5,
            markers::Name => 6,
            markers::RCSfile => 7,
            markers::Revision => 8,
            markers::Source => 9,
            markers::State => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> markers {
        match value {
            0 => markers::Author,
            1 => markers::Date,
            2 => markers::Header,
            3 => markers::Id,
            4 => markers::Locker,
            5 => markers::Log,
            6 => markers::Name,
            7 => markers::RCSfile,
            8 => markers::Revision,
            9 => markers::Source,
            10 => markers::State,
            _ => panic!("Invalid value for markers: {}", value),
        }
    }
}
impl AddAssign<u32> for markers {
    fn add_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for markers {
    fn sub_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for markers {
    fn mul_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for markers {
    fn div_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for markers {
    fn rem_assign(&mut self, rhs: u32) {
        *self = markers::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for markers {
    type Output = markers;
    fn add(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for markers {
    type Output = markers;
    fn sub(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for markers {
    type Output = markers;
    fn mul(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for markers {
    type Output = markers;
    fn div(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for markers {
    type Output = markers;
    fn rem(self, rhs: u32) -> markers {
        markers::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
pub struct zone_offset {
    pub valid: bool,
    pub seconds: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut i8,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
unsafe extern "C" fn discardkeyval(mut c: i32, mut f: *mut fro) -> i32 {
    loop {
        match c {
            36 | 10 => return c,
            _ => {
                if fro_try_getbyte(&mut c, f) {
                    return -(1 as i32);
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rcsfcmp(
    mut xfp: *mut fro,
    mut xstatp: *const stat,
    mut uname: *const i8,
    mut delta: *const delta,
) -> i32 {
    let mut current_block: u64;
    let mut xc: i32 = 0;
    let mut uc: i32 = 0;
    let mut xkeyword: [i8; 10] = [0; 10];
    let mut eqkeyvals: bool = false;
    let mut ufp: *mut fro = 0 as *mut fro;
    let mut xeof: bool = false;
    let mut ueof: bool = false;
    let mut tp: *mut i8 = 0 as *mut i8;
    let mut sp: *const i8 = 0 as *const i8;
    let mut leaderlen: size_t = 0;
    let mut result: i32 = 0;
    let mut match1: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    let mut ustat: stat = stat {
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
    ufp = fro_open(uname, b"r\0" as *const u8 as *const i8, &mut ustat);
    if ufp.is_null() {
        fatal_sys(uname);
    }
    ueof = 0 as i32 != 0;
    xeof = ueof;
    if kwsub::kwsub_o as i32 <= (*top).behavior.kws {
        result = ((*xstatp).st_size != ustat.st_size) as i32;
        if result == 0 {
            if readmethod::RM_STDIO as i32 as u32 != (*xfp).rm as u32
                && readmethod::RM_STDIO as i32 as u32 != (*ufp).rm as u32
            {
                result = (0 as i32
                    != memcmp(
                        (*xfp).base as *const libc::c_void,
                        (*ufp).base as *const libc::c_void,
                        (*xstatp).st_size as u64,
                    )) as i32;
                current_block = 11348956670850924295;
            } else {
                loop {
                    if fro_try_getbyte(&mut xc, xfp) {
                        xeof = 1 as i32 != 0;
                    }
                    if fro_try_getbyte(&mut uc, ufp) {
                        ueof = 1 as i32 != 0;
                    }
                    if xeof as i32 | ueof as i32 != 0 {
                        current_block = 11348956670850924295;
                        break;
                    }
                    if xc != uc {
                        current_block = 1757502968488058812;
                        break;
                    }
                }
            }
        } else {
            current_block = 11348956670850924295;
        }
    } else {
        xc = 0 as i32;
        uc = 0 as i32;
        leaderlen = 0 as i32 as size_t;
        result = 0 as i32;
        's_133: loop {
            if xc != '$' as i32 {
                if fro_try_getbyte(&mut xc, xfp) {
                    xeof = 1 as i32 != 0;
                }
                if fro_try_getbyte(&mut uc, ufp) {
                    ueof = 1 as i32 != 0;
                }
                if xeof as i32 | ueof as i32 != 0 {
                    current_block = 11348956670850924295;
                    break;
                }
            } else {
                tp = xkeyword.as_mut_ptr();
                loop {
                    if fro_try_getbyte(&mut xc, xfp) {
                        xeof = 1 as i32 != 0;
                    }
                    if fro_try_getbyte(&mut uc, ufp) {
                        ueof = 1 as i32 != 0;
                    }
                    if xeof as i32 | ueof as i32 != 0 {
                        current_block = 11348956670850924295;
                        break 's_133;
                    }
                    if xc != uc {
                        break;
                    }
                    match xc {
                        10 | 36 | 58 => {
                            break;
                        }
                        _ => {}
                    }
                    if xkeyword.as_mut_ptr().offset(8 as i32 as isize) <= tp {
                        break;
                    }
                    let fresh0 = tp;
                    tp = tp.offset(1);
                    *fresh0 = xc as i8;
                }
                if (xc == '$' as i32 || xc == ':' as i32)
                    && (uc == '$' as i32 || uc == ':' as i32)
                    && {
                        *tp = xc as i8;
                        recognize_keyword(xkeyword.as_mut_ptr(), &mut match1) as i32 != 0
                    }
                {
                    result = -(1 as i32);
                    loop {
                        if xc != uc {
                            xc = discardkeyval(xc, xfp);
                            uc = discardkeyval(uc, ufp);
                            xeof = xc == -(1 as i32);
                            ueof = uc == -(1 as i32);
                            if xeof as i32 | ueof as i32 != 0 {
                                current_block = 11348956670850924295;
                                break 's_133;
                            }
                            eqkeyvals = 0 as i32 != 0;
                            break;
                        } else {
                            match xc {
                                10 | 36 => {
                                    eqkeyvals = 1 as i32 != 0;
                                    break;
                                }
                                _ => {
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        xeof = 1 as i32 != 0;
                                    }
                                    if fro_try_getbyte(&mut uc, ufp) {
                                        ueof = 1 as i32 != 0;
                                    }
                                    if xeof as i32 | ueof as i32 != 0 {
                                        current_block = 11348956670850924295;
                                        break 's_133;
                                    }
                                }
                            }
                        }
                    }
                    if xc != uc {
                        current_block = 1757502968488058812;
                        break;
                    }
                    if xc == '$' as i32 {
                        if fro_try_getbyte(&mut xc, xfp) {
                            xeof = 1 as i32 != 0;
                        }
                        if fro_try_getbyte(&mut uc, ufp) {
                            ueof = 1 as i32 != 0;
                        }
                        if xeof as i32 | ueof as i32 != 0 {
                            current_block = 11348956670850924295;
                            break;
                        }
                        if match1.i == markers::Log as i32 {
                            let mut lncnt: i32 = 0;
                            let mut ls: size_t = 0;
                            let mut ccnt: size_t = 0;
                            sp = (*delta).pretty_log.string;
                            ls = (*delta).pretty_log.size;
                            if !looking_at(&tiny_ciklog, (*delta).pretty_log.string) {
                                let mut c1: i32 = 1 as i32;
                                ccnt = (*top).repository.log_lead.size;
                                loop {
                                    let fresh1 = ccnt;
                                    ccnt = ccnt.wrapping_sub(1);
                                    if !(fresh1 != 0) {
                                        break;
                                    }
                                    c1
                                        += (*((*top).repository.log_lead.string)
                                            .offset(ccnt as isize) as i32 == '\n' as i32) as i32;
                                }
                                lncnt = 2 as i32 * c1 + 1 as i32;
                                loop {
                                    let fresh2 = ls;
                                    ls = ls.wrapping_sub(1);
                                    if !(fresh2 != 0) {
                                        break;
                                    }
                                    let fresh3 = sp;
                                    sp = sp.offset(1);
                                    if *fresh3 as i32 == '\n' as i32 {
                                        lncnt += c1;
                                    }
                                }
                                loop {
                                    if xc == '\n' as i32 {
                                        lncnt -= 1;
                                        if lncnt == 0 as i32 {
                                            break;
                                        }
                                    }
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        current_block = 17454088596801686518;
                                        break 's_133;
                                    }
                                }
                                ccnt = if (*top).behavior.version < 5 as i32 - 5 as i32 {
                                    (*top).repository.log_lead.size
                                } else {
                                    leaderlen
                                };
                                loop {
                                    if fro_try_getbyte(&mut xc, xfp) {
                                        current_block = 17454088596801686518;
                                        break 's_133;
                                    }
                                    let fresh4 = ccnt;
                                    ccnt = ccnt.wrapping_sub(1);
                                    if !(fresh4 != 0
                                        && (xc != '\n' as i32
                                            || {
                                                c1 -= 1;
                                                c1 != 0
                                            }))
                                    {
                                        break;
                                    }
                                }
                            }
                        }
                    } else if !eqkeyvals {
                        current_block = 1757502968488058812;
                        break;
                    }
                }
            }
            if xc != uc {
                current_block = 1757502968488058812;
                break;
            }
            if xc == '\n' as i32 {
                leaderlen = 0 as i32 as size_t;
            } else {
                leaderlen = leaderlen.wrapping_add(1);
                leaderlen;
            }
        }
    }
    match current_block {
        11348956670850924295 => {
            if xeof as i32 == ueof as i32 {
                current_block = 17454088596801686518;
            } else {
                current_block = 1757502968488058812;
            }
        }
        _ => {}
    }
    match current_block {
        1757502968488058812 => {
            result = 1 as i32;
        }
        _ => {}
    }
    fro_close(ufp);
    return result;
}