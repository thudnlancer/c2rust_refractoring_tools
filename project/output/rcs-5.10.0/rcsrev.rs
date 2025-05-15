use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    static mut top: *mut top;
    static ks_revno: [i8; 0];
    static ctab: [tokens; 0];
    fn date2str(date: *const i8, datebuf: *mut i8) -> *const i8;
    fn getoldkeys(fp: *mut fro) -> bool;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    static mut single: *mut divvy;
    fn alloc(divvy: *mut divvy, len: size_t) -> *mut libc::c_void;
    fn accumulate_byte(divvy: *mut divvy, c: i32);
    fn finish_string(divvy: *mut divvy, result_len: *mut size_t) -> *mut i8;
    fn accumulate_range(divvy: *mut divvy, beg: *const i8, end: *const i8);
    fn accf(divvy: *mut divvy, fmt: *const i8, _: ...);
    fn brush_off(divvy: *mut divvy, ptr: *mut libc::c_void);
    fn accs(divvy: *mut divvy, string: *const i8);
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
pub enum tokens {
    DELIM,
    DIGIT,
    IDCHAR,
    NEWLN,
    LETTER,
    Letter,
    PERIOD,
    SBEGIN,
    SPACE,
    UNKN,
    COLON,
    ID,
    NUM,
    SEMI,
    STRING,
}
impl tokens {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            tokens::DELIM => 0,
            tokens::DIGIT => 1,
            tokens::IDCHAR => 2,
            tokens::NEWLN => 3,
            tokens::LETTER => 4,
            tokens::Letter => 5,
            tokens::PERIOD => 6,
            tokens::SBEGIN => 7,
            tokens::SPACE => 8,
            tokens::UNKN => 9,
            tokens::COLON => 10,
            tokens::ID => 11,
            tokens::NUM => 12,
            tokens::SEMI => 13,
            tokens::STRING => 14,
        }
    }
    fn from_libc_c_uint(value: u32) -> tokens {
        match value {
            0 => tokens::DELIM,
            1 => tokens::DIGIT,
            2 => tokens::IDCHAR,
            3 => tokens::NEWLN,
            4 => tokens::LETTER,
            5 => tokens::Letter,
            6 => tokens::PERIOD,
            7 => tokens::SBEGIN,
            8 => tokens::SPACE,
            9 => tokens::UNKN,
            10 => tokens::COLON,
            11 => tokens::ID,
            12 => tokens::NUM,
            13 => tokens::SEMI,
            14 => tokens::STRING,
            _ => panic!("Invalid value for tokens: {}", value),
        }
    }
}
impl AddAssign<u32> for tokens {
    fn add_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for tokens {
    fn sub_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for tokens {
    fn mul_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for tokens {
    fn div_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for tokens {
    fn rem_assign(&mut self, rhs: u32) {
        *self = tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for tokens {
    type Output = tokens;
    fn add(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for tokens {
    type Output = tokens;
    fn sub(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for tokens {
    type Output = tokens;
    fn mul(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for tokens {
    type Output = tokens;
    fn div(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for tokens {
    type Output = tokens;
    fn rem(self, rhs: u32) -> tokens {
        tokens::from_libc_c_uint(self.to_libc_c_uint() % rhs)
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
pub struct symdef {
    pub meaningful: *const i8,
    pub underlying: *const i8,
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
unsafe extern "C" fn split(mut s: *const i8, mut lastdot: *mut *const i8) -> i32 {
    let mut count: size_t = 0;
    *lastdot = 0 as *const i8;
    if s.is_null() || *s == 0 {
        return 0 as i32;
    }
    count = 1 as i32 as size_t;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        if *fresh0 as i32 == '.' as i32 {
            *lastdot = s.offset(-(1 as i32 as isize));
            count = count.wrapping_add(1);
            count;
        }
        if !(*s != 0) {
            break;
        }
    }
    return count as i32;
}
#[no_mangle]
pub unsafe extern "C" fn countnumflds(mut s: *const i8) -> i32 {
    let mut sp: *const i8 = 0 as *const i8;
    let mut count: i32 = 0;
    sp = s;
    if sp.is_null() || *sp == 0 {
        return 0 as i32;
    }
    count = 1 as i32;
    loop {
        let fresh1 = sp;
        sp = sp.offset(1);
        if *fresh1 as i32 == '.' as i32 {
            count += 1;
            count;
        }
        if !(*sp != 0) {
            break;
        }
    }
    return count;
}
unsafe extern "C" fn accumulate_branchno(mut space: *mut divvy, mut revno: *const i8) {
    let mut end: *const i8 = 0 as *const i8;
    let mut nfields: i32 = split(revno, &mut end);
    if nfields & 1 as i32 != 0 {
        accs(space, revno);
    } else {
        accumulate_range(space, revno, end);
    };
}
#[no_mangle]
pub unsafe extern "C" fn take(mut count: size_t, mut ref_0: *const i8) -> cbuf {
    let mut rv: cbuf = cbuf {
        string: 0 as *const i8,
        size: 0,
    };
    let mut end: *const i8 = ref_0;
    if count == 0 {
        count = (-(2 as i32) as u32)
            .wrapping_add(1 as u32 | (1 as i32 + countnumflds(ref_0)) as u32) as size_t;
    }
    loop {
        let fresh2 = count;
        count = count.wrapping_sub(1);
        if !(fresh2 != 0) {
            break;
        }
        while *end as i32 != 0
            && {
                let fresh3 = end;
                end = end.offset(1);
                '.' as i32 != *fresh3 as i32
            }
        {}
    }
    accumulate_range(
        single,
        ref_0,
        if *end as i32 != 0 { end.offset(-(1 as i32 as isize)) } else { end },
    );
    rv.string = finish_string(single, &mut rv.size);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn cmpnum(mut num1: *const i8, mut num2: *const i8) -> i32 {
    let mut s1: *const i8 = 0 as *const i8;
    let mut s2: *const i8 = 0 as *const i8;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    let mut r: i32 = 0;
    s1 = if !num1.is_null() { num1 } else { b"\0" as *const u8 as *const i8 };
    s2 = if !num2.is_null() { num2 } else { b"\0" as *const u8 as *const i8 };
    loop {
        if *s1 == 0 {
            return *s2 as u8 as i32;
        }
        if *s2 == 0 {
            return -(1 as i32);
        }
        while *s1 as i32 == '0' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        while *s2 as i32 == '0' as i32 {
            s2 = s2.offset(1);
            s2;
        }
        d1 = 0 as i32 as size_t;
        while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d1 = d1.wrapping_add(1);
            d1;
        }
        d2 = 0 as i32 as size_t;
        while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d2 = d2.wrapping_add(1);
            d2;
        }
        if d1 != d2 {
            return if d1 < d2 { -(1 as i32) } else { 1 as i32 };
        }
        r = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1);
        if r != 0 {
            return r;
        }
        s1 = s1.offset(d1 as isize);
        s2 = s2.offset(d1 as isize);
        if *s1 != 0 {
            s1 = s1.offset(1);
            s1;
        }
        if *s2 != 0 {
            s2 = s2.offset(1);
            s2;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmpnumfld(
    mut num1: *const i8,
    mut num2: *const i8,
    mut fld: i32,
) -> i32 {
    let mut s1: *const i8 = 0 as *const i8;
    let mut s2: *const i8 = 0 as *const i8;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    s1 = num1;
    s2 = num2;
    loop {
        fld -= 1;
        if !(fld != 0) {
            break;
        }
        loop {
            let fresh4 = s1;
            s1 = s1.offset(1);
            if !(*fresh4 as i32 != '.' as i32) {
                break;
            }
        }
        loop {
            let fresh5 = s2;
            s2 = s2.offset(1);
            if !(*fresh5 as i32 != '.' as i32) {
                break;
            }
        }
    }
    while *s1 as i32 == '0' as i32 {
        s1 = s1.offset(1);
        s1;
    }
    d1 = 0 as i32 as size_t;
    while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as i32 as isize) as i32
        & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        d1 = d1.wrapping_add(1);
        d1;
    }
    while *s2 as i32 == '0' as i32 {
        s2 = s2.offset(1);
        s2;
    }
    d2 = 0 as i32 as size_t;
    while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as i32 as isize) as i32
        & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
    {
        d2 = d2.wrapping_add(1);
        d2;
    }
    return if d1 < d2 {
        -(1 as i32)
    } else if d1 == d2 {
        memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1)
    } else {
        1 as i32
    };
}
unsafe extern "C" fn normalizeyear(mut date: *const i8, mut year: *mut i8) -> *const i8 {
    if *(*__ctype_b_loc()).offset(*date.offset(0 as i32 as isize) as i32 as isize) as i32
        & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        && *(*__ctype_b_loc()).offset(*date.offset(1 as i32 as isize) as i32 as isize)
            as i32 & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        && *(*__ctype_b_loc()).offset(*date.offset(2 as i32 as isize) as i32 as isize)
            as i32 & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 == 0
    {
        *year.offset(0 as i32 as isize) = '1' as i32 as i8;
        *year.offset(1 as i32 as isize) = '9' as i32 as i8;
        *year.offset(2 as i32 as isize) = *date.offset(0 as i32 as isize);
        *year.offset(3 as i32 as isize) = *date.offset(1 as i32 as isize);
        *year.offset(4 as i32 as isize) = 0 as i32 as i8;
        return year as *const i8;
    } else {
        return date
    };
}
#[no_mangle]
pub unsafe extern "C" fn cmpdate(mut d1: *const i8, mut d2: *const i8) -> i32 {
    let mut year1: [i8; 5] = [0; 5];
    let mut year2: [i8; 5] = [0; 5];
    let mut r: i32 = cmpnumfld(
        normalizeyear(d1, year1.as_mut_ptr()),
        normalizeyear(d2, year2.as_mut_ptr()),
        1 as i32,
    );
    if r != 0 {
        return r
    } else {
        while *(*__ctype_b_loc()).offset(*d1 as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d1 = d1.offset(1);
            d1;
        }
        d1 = d1.offset((*d1 as i32 == '.' as i32) as i32 as isize);
        while *(*__ctype_b_loc()).offset(*d2 as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d2 = d2.offset(1);
            d2;
        }
        d2 = d2.offset((*d2 as i32 == '.' as i32) as i32 as isize);
        return cmpnum(d1, d2);
    };
}
unsafe extern "C" fn cantfindbranch(
    mut revno: *const i8,
    mut date: *const i8,
    mut author: *const i8,
    mut state: *const i8,
) {
    let mut datebuf: [i8; 31] = [0; 31];
    generic_error(
        (*top).repository.filename,
        b"No revision on branch %s has%s%s%s%s%s%s.\0" as *const u8 as *const i8,
        revno,
        if !date.is_null() {
            b" a date before \0" as *const u8 as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !date.is_null() {
            date2str(date, datebuf.as_mut_ptr())
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !author.is_null() {
            (b" and author \0" as *const u8 as *const i8)
                .offset((if !date.is_null() { 0 as i32 } else { 4 as i32 }) as isize)
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !author.is_null() { author } else { b"\0" as *const u8 as *const i8 },
        if !state.is_null() {
            (b" and state \0" as *const u8 as *const i8)
                .offset(
                    (if !date.is_null() || !author.is_null() {
                        0 as i32
                    } else {
                        4 as i32
                    }) as isize,
                )
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !state.is_null() { state } else { b"\0" as *const u8 as *const i8 },
    );
}
unsafe extern "C" fn absent(mut revno: *const i8, mut field: i32) {
    generic_error(
        (*top).repository.filename,
        b"%s %s absent\0" as *const u8 as *const i8,
        if field & 1 as i32 != 0 {
            b"revision\0" as *const u8 as *const i8
        } else {
            b"branch\0" as *const u8 as *const i8
        },
        (take(field as size_t, revno)).string,
    );
}
#[no_mangle]
pub unsafe extern "C" fn compartial(
    mut num1: *const i8,
    mut num2: *const i8,
    mut length: i32,
) -> i32 {
    let mut s1: *const i8 = 0 as *const i8;
    let mut s2: *const i8 = 0 as *const i8;
    let mut d1: size_t = 0;
    let mut d2: size_t = 0;
    let mut r: i32 = 0;
    s1 = num1;
    s2 = num2;
    if s1.is_null() {
        return 1 as i32;
    }
    if s2.is_null() {
        return -(1 as i32);
    }
    loop {
        if *s1 == 0 {
            return 1 as i32;
        }
        if *s2 == 0 {
            return -(1 as i32);
        }
        while *s1 as i32 == '0' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        d1 = 0 as i32 as size_t;
        while *(*__ctype_b_loc()).offset(*s1.offset(d1 as isize) as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d1 = d1.wrapping_add(1);
            d1;
        }
        while *s2 as i32 == '0' as i32 {
            s2 = s2.offset(1);
            s2;
        }
        d2 = 0 as i32 as size_t;
        while *(*__ctype_b_loc()).offset(*s2.offset(d2 as isize) as i32 as isize) as i32
            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32 != 0
        {
            d2 = d2.wrapping_add(1);
            d2;
        }
        if d1 != d2 {
            return if d1 < d2 { -(1 as i32) } else { 1 as i32 };
        }
        r = memcmp(s1 as *const libc::c_void, s2 as *const libc::c_void, d1);
        if r != 0 {
            return r;
        }
        length -= 1;
        if length == 0 {
            return 0 as i32;
        }
        s1 = s1.offset(d1 as isize);
        s2 = s2.offset(d1 as isize);
        if *s1 as i32 == '.' as i32 {
            s1 = s1.offset(1);
            s1;
        }
        if *s2 as i32 == '.' as i32 {
            s2 = s2.offset(1);
            s2;
        }
    };
}
unsafe extern "C" fn store1(mut store: *mut *mut *mut wlink, mut next: *mut delta) {
    let mut p: *mut wlink = 0 as *mut wlink;
    p = alloc(single, ::core::mem::size_of::<wlink>() as u64) as *mut wlink;
    (*p).entry = next as *mut libc::c_void;
    **store = p;
    *store = &mut (*p).next;
}
unsafe extern "C" fn genbranch(
    mut bpoint: *const delta,
    mut revno: *const i8,
    mut length: i32,
    mut date: *const i8,
    mut author: *const i8,
    mut state: *const i8,
    mut store: *mut *mut wlink,
) -> *mut delta {
    let mut field: i32 = 0;
    let mut d: *mut delta = 0 as *mut delta;
    let mut trail: *mut delta = 0 as *mut delta;
    let mut bhead: *const wlink = 0 as *const wlink;
    let mut result: i32 = 0;
    let mut datebuf: [i8; 31] = [0; 31];
    field = 3 as i32;
    bhead = (*bpoint).branches;
    loop {
        if bhead.is_null() {
            generic_error(
                (*top).repository.filename,
                b"no side branches present for %s\0" as *const u8 as *const i8,
                (take((field - 1 as i32) as size_t, revno)).string,
            );
            return 0 as *mut delta;
        }
        loop {
            d = (*bhead).entry as *mut delta;
            result = cmpnumfld(revno, (*d).num, field);
            if !((0 as i32) < result) {
                break;
            }
            bhead = (*bhead).next;
            if bhead.is_null() {
                generic_error(
                    (*top).repository.filename,
                    b"branch number %s too high\0" as *const u8 as *const i8,
                    (take(field as size_t, revno)).string,
                );
                return 0 as *mut delta;
            }
        }
        if result < 0 as i32 {
            absent(revno, field);
            return 0 as *mut delta;
        }
        d = (*bhead).entry as *mut delta;
        if length == field {
            trail = 0 as *mut delta;
            loop {
                if (date.is_null() || !(0 as i32 > cmpdate(date, (*d).date)))
                    && (author.is_null() || strcmp(author, (*d).author) == 0)
                    && (state.is_null() || strcmp(state, (*d).state) == 0)
                {
                    trail = d;
                }
                d = (*d).ilk;
                if d.is_null() {
                    break;
                }
            }
            if trail.is_null() {
                cantfindbranch(revno, date, author, state);
                return 0 as *mut delta;
            } else {
                d = (*bhead).entry as *mut delta;
                while d != trail {
                    if !store.is_null() {
                        store1(&mut store, d);
                    }
                    d = (*d).ilk;
                }
                if !store.is_null() {
                    store1(&mut store, d);
                }
            }
            if !store.is_null() {
                *store = 0 as *mut wlink;
            }
            return d;
        }
        if 0 as i32 > cmpnumfld(revno, (*d).num, 1 as i32 + field) {
            generic_error(
                (*top).repository.filename,
                b"%s %s too low\0" as *const u8 as *const i8,
                ks_revno.as_ptr(),
                (take((field + 1 as i32) as size_t, revno)).string,
            );
            return 0 as *mut delta;
        }
        loop {
            if !store.is_null() {
                store1(&mut store, d);
            }
            trail = d;
            d = (*d).ilk;
            if !(!d.is_null()
                && !(0 as i32 > cmpnumfld(revno, (*d).num, 1 as i32 + field)))
            {
                break;
            }
        }
        if length > field + 1 as i32
            && !(0 as i32 == cmpnumfld(revno, (*trail).num, 1 as i32 + field))
        {
            absent(revno, field + 1 as i32);
            return 0 as *mut delta;
        }
        if length == field + 1 as i32 {
            if !date.is_null() && 0 as i32 > cmpdate(date, (*trail).date) {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has date %s.\0" as *const u8 as *const i8,
                    (*trail).num,
                    date2str((*trail).date, datebuf.as_mut_ptr()),
                );
                return 0 as *mut delta;
            }
            if !author.is_null() && strcmp(author, (*trail).author) != 0 {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has author %s.\0" as *const u8 as *const i8,
                    (*trail).num,
                    (*trail).author,
                );
                return 0 as *mut delta;
            }
            if !state.is_null() && strcmp(state, (*trail).state) != 0 {
                generic_error(
                    (*top).repository.filename,
                    b"Revision %s has state %s.\0" as *const u8 as *const i8,
                    (*trail).num,
                    if !((*trail).state).is_null() {
                        (*trail).state
                    } else {
                        b"<empty>\0" as *const u8 as *const i8
                    },
                );
                return 0 as *mut delta;
            }
        }
        bhead = (*trail).branches;
        field += 2 as i32;
        if !(field <= length) {
            break;
        }
    }
    if !store.is_null() {
        *store = 0 as *mut wlink;
    }
    return trail;
}
#[no_mangle]
pub unsafe extern "C" fn genrevs(
    mut revno: *const i8,
    mut date: *const i8,
    mut author: *const i8,
    mut state: *const i8,
    mut store: *mut *mut wlink,
) -> *mut delta {
    let mut current_block: u64;
    let mut length: i32 = 0;
    let mut d: *mut delta = 0 as *mut delta;
    let mut result: i32 = 0;
    let mut branchnum: *const i8 = 0 as *const i8;
    let mut datebuf: [i8; 31] = [0; 31];
    d = (*top).repository.tip;
    if d.is_null() {
        generic_error(
            (*top).repository.filename,
            b"RCS file empty\0" as *const u8 as *const i8,
        );
    } else {
        length = countnumflds(revno);
        if length >= 1 as i32 {
            loop {
                result = cmpnumfld(revno, (*d).num, 1 as i32);
                if !(result < 0 as i32) {
                    current_block = 11650488183268122163;
                    break;
                }
                if !store.is_null() {
                    store1(&mut store, d);
                }
                d = (*d).ilk;
                if !d.is_null() {
                    continue;
                }
                generic_error(
                    (*top).repository.filename,
                    b"branch number %s too low\0" as *const u8 as *const i8,
                    (take(1 as i32 as size_t, revno)).string,
                );
                current_block = 11978812997839462505;
                break;
            }
            match current_block {
                11978812997839462505 => {}
                _ => {
                    if result > 0 as i32 {
                        absent(revno, 1 as i32);
                        current_block = 11978812997839462505;
                    } else {
                        current_block = 10599921512955367680;
                    }
                }
            }
        } else {
            current_block = 10599921512955367680;
        }
        match current_block {
            11978812997839462505 => {}
            _ => {
                if length <= 1 as i32 {
                    branchnum = (*d).num;
                    while !d.is_null()
                        && 0 as i32 == cmpnumfld(branchnum, (*d).num, 1 as i32)
                        && (!date.is_null() && 0 as i32 > cmpdate(date, (*d).date)
                            || !author.is_null() && strcmp(author, (*d).author) != 0
                            || !state.is_null() && strcmp(state, (*d).state) != 0)
                    {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        d = (*d).ilk;
                    }
                    if d.is_null()
                        || !(0 as i32 == cmpnumfld(branchnum, (*d).num, 1 as i32))
                    {
                        cantfindbranch(
                            if length != 0 {
                                revno
                            } else {
                                (take(1 as i32 as size_t, branchnum)).string
                            },
                            date,
                            author,
                            state,
                        );
                    } else {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        if !store.is_null() {
                            *store = 0 as *mut wlink;
                        }
                        return d;
                    }
                } else {
                    loop {
                        result = cmpnumfld(revno, (*d).num, 2 as i32);
                        if !(result < 0 as i32
                            && 0 as i32 == cmpnumfld(revno, (*d).num, 1 as i32))
                        {
                            break;
                        }
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        d = (*d).ilk;
                        if d.is_null() {
                            break;
                        }
                    }
                    if d.is_null() || !(0 as i32 == cmpnumfld(revno, (*d).num, 1 as i32))
                    {
                        generic_error(
                            (*top).repository.filename,
                            b"%s %s too low\0" as *const u8 as *const i8,
                            ks_revno.as_ptr(),
                            (take(2 as i32 as size_t, revno)).string,
                        );
                    } else if length > 2 as i32 && result != 0 as i32 {
                        absent(revno, 2 as i32);
                    } else {
                        if !store.is_null() {
                            store1(&mut store, d);
                        }
                        if length > 2 as i32 {
                            return genbranch(
                                d,
                                revno,
                                length,
                                date,
                                author,
                                state,
                                store,
                            )
                        } else {
                            if !date.is_null() && 0 as i32 > cmpdate(date, (*d).date) {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has date %s.\0" as *const u8 as *const i8,
                                    (*d).num,
                                    date2str((*d).date, datebuf.as_mut_ptr()),
                                );
                                return 0 as *mut delta;
                            }
                            if !author.is_null() && strcmp(author, (*d).author) != 0 {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has author %s.\0" as *const u8 as *const i8,
                                    (*d).num,
                                    (*d).author,
                                );
                                return 0 as *mut delta;
                            }
                            if !state.is_null() && strcmp(state, (*d).state) != 0 {
                                generic_error(
                                    (*top).repository.filename,
                                    b"Revision %s has state %s.\0" as *const u8 as *const i8,
                                    (*d).num,
                                    if !((*d).state).is_null() {
                                        (*d).state
                                    } else {
                                        b"<empty>\0" as *const u8 as *const i8
                                    },
                                );
                                return 0 as *mut delta;
                            }
                            if !store.is_null() {
                                *store = 0 as *mut wlink;
                            }
                            return d;
                        }
                    }
                }
            }
        }
    }
    return 0 as *mut delta;
}
#[no_mangle]
pub unsafe extern "C" fn gr_revno(
    mut revno: *const i8,
    mut store: *mut *mut wlink,
) -> *mut delta {
    return genrevs(revno, 0 as *const i8, 0 as *const i8, 0 as *const i8, store);
}
#[no_mangle]
pub unsafe extern "C" fn delta_from_ref(mut ref_0: *const i8) -> *mut delta {
    return genrevs(
        ref_0,
        0 as *const i8,
        0 as *const i8,
        0 as *const i8,
        0 as *mut *mut wlink,
    );
}
unsafe extern "C" fn rev_from_symbol(mut id: *const cbuf) -> *const i8 {
    let mut ls: *mut link = (*(*top).repository.r).symbols;
    while !ls.is_null() {
        let mut d: *const symdef = (*ls).entry as *const symdef;
        if '\0' as i32 == *((*d).meaningful).offset((*id).size as isize) as i32
            && strncmp((*d).meaningful, (*id).string, (*id).size) == 0
        {
            return (*d).underlying;
        }
        ls = (*ls).next;
    }
    return 0 as *const i8;
}
unsafe extern "C" fn lookupsym(mut id: *const i8) -> *const i8 {
    let mut identifier: cbuf = {
        let mut init = cbuf {
            string: id,
            size: strlen(id),
        };
        init
    };
    return rev_from_symbol(&mut identifier);
}
unsafe extern "C" fn branchtip(mut branch: *const i8) -> *const i8 {
    let mut h: *mut delta = 0 as *mut delta;
    h = delta_from_ref(branch);
    return if !h.is_null() { (*h).num } else { 0 as *const i8 };
}
#[no_mangle]
pub unsafe extern "C" fn fully_numeric(
    mut ans: *mut cbuf,
    mut source: *const i8,
    mut fp: *mut fro,
) -> bool {
    let mut current_block: u64;
    let mut sp: *const i8 = 0 as *const i8;
    let mut bp: *const i8 = 0 as *const i8;
    let mut dots: i32 = 0;
    let mut ugh: *mut i8 = 0 as *mut i8;
    sp = source;
    if !(sp.is_null() || *sp == 0) {
        if *sp.offset(0 as i32 as isize) as i32 == '$' as i32
            && *sp.offset(1 as i32 as isize) == 0
        {
            if !getoldkeys(fp) {
                current_block = 14450225814887664406;
            } else if ((*top).manifestation.prev.rev).is_null() {
                generic_error(
                    (*top).manifestation.filename,
                    b"working file lacks %s\0" as *const u8 as *const i8,
                    ks_revno.as_ptr(),
                );
                current_block = 14450225814887664406;
            } else {
                accs(single, (*top).manifestation.prev.rev);
                current_block = 9689500680931214244;
            }
        } else {
            dots = 0 as i32;
            loop {
                let mut was: *const i8 = sp;
                let mut id: bool = 0 as i32 != 0;
                loop {
                    match *ctab.as_ptr().offset(*sp as u8 as isize) as u32 {
                        2 | 4 | 5 => {
                            id = 1 as i32 != 0;
                        }
                        1 => {}
                        _ => {
                            break;
                        }
                    }
                    sp = sp.offset(1);
                    sp;
                }
                if id {
                    let mut orig: cbuf = {
                        let mut init = cbuf {
                            string: was,
                            size: sp.offset_from(was) as i64 as size_t,
                        };
                        init
                    };
                    let mut expanded: *const i8 = rev_from_symbol(&mut orig);
                    if expanded.is_null() {
                        generic_error(
                            (*top).repository.filename,
                            b"Symbolic name `%s' is undefined.\0" as *const u8
                                as *const i8,
                            was,
                        );
                        current_block = 14450225814887664406;
                        break;
                    } else {
                        accs(single, expanded);
                    }
                } else {
                    if was != sp {
                        accumulate_range(single, was, sp);
                        bp = was;
                    }
                    while '0' as i32 == *sp.offset(0 as i32 as isize) as i32
                        && *(*__ctype_b_loc())
                            .offset(*sp.offset(1 as i32 as isize) as i32 as isize) as i32
                            & C2RustUnnamed_3::_ISdigit as i32 as libc::c_ushort as i32
                            != 0
                    {
                        sp = sp.offset(1);
                        sp;
                    }
                    if bp.is_null() {
                        let mut s: i32 = 0 as i32;
                        if s != 0 || *sp as i32 != '.' as i32 {
                            current_block = 10399321362245223758;
                            break;
                        }
                        let mut b: *const i8 = 0 as *const i8;
                        let mut tip: *mut delta = 0 as *mut delta;
                        if !((*(*top).repository.r).branch).is_null() {
                            b = (*(*top).repository.r).branch;
                        } else {
                            tip = (*top).repository.tip;
                            if tip.is_null() {
                                current_block = 10399321362245223758;
                                break;
                            }
                            b = (*tip).num;
                        }
                        ugh = finish_string(single, &mut (*ans).size);
                        (*ans).string = ugh;
                        if !ugh.is_null() {
                            brush_off(single, ugh as *mut libc::c_void);
                        }
                        accumulate_branchno(single, b);
                    }
                }
                let fresh6 = sp;
                sp = sp.offset(1);
                match *fresh6 as i32 {
                    0 => {
                        current_block = 9689500680931214244;
                        break;
                    }
                    46 => {}
                    _ => {
                        current_block = 10399321362245223758;
                        break;
                    }
                }
                if *sp == 0 {
                    if dots & 1 as i32 != 0 {
                        current_block = 10399321362245223758;
                        break;
                    }
                    ugh = finish_string(single, &mut (*ans).size);
                    (*ans).string = ugh;
                    bp = branchtip((*ans).string);
                    if bp.is_null() {
                        current_block = 14450225814887664406;
                        break;
                    }
                    accf(
                        single,
                        b"%s%s\0" as *const u8 as *const i8,
                        (*ans).string,
                        bp.offset((*ans).size as isize),
                    );
                    current_block = 9689500680931214244;
                    break;
                } else {
                    dots += 1;
                    dots;
                    accumulate_byte(single, '.' as i32);
                }
            }
            match current_block {
                14450225814887664406 => {}
                9689500680931214244 => {}
                _ => {
                    generic_error(
                        (*top).repository.filename,
                        b"improper %s: %s\0" as *const u8 as *const i8,
                        ks_revno.as_ptr(),
                        source,
                    );
                    current_block = 14450225814887664406;
                }
            }
        }
        match current_block {
            9689500680931214244 => {}
            _ => {
                ugh = finish_string(single, &mut (*ans).size);
                (*ans).string = ugh;
                if !ugh.is_null() {
                    brush_off(single, ugh as *mut libc::c_void);
                }
                return 0 as i32 != 0;
            }
        }
    }
    ugh = finish_string(single, &mut (*ans).size);
    (*ans).string = ugh;
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn namedrev(
    mut name: *const i8,
    mut delta: *mut delta,
) -> *const i8 {
    if !name.is_null() {
        let mut id: *const i8 = 0 as *const i8;
        let mut p: *const i8 = 0 as *const i8;
        let mut val: *const i8 = 0 as *const i8;
        p = name;
        loop {
            let mut current_block_4: u64;
            match *ctab.as_ptr().offset(*p as u8 as isize) as u32 {
                2 | 4 | 5 => {
                    id = name;
                    current_block_4 = 8515828400728868193;
                }
                1 => {
                    current_block_4 = 8515828400728868193;
                }
                9 => {
                    if *p == 0 && !id.is_null()
                        && {
                            val = lookupsym(id);
                            !val.is_null()
                        } && strcmp(val, (*delta).num) == 0
                    {
                        return id;
                    }
                    current_block_4 = 9110877840600203652;
                }
                _ => {
                    current_block_4 = 9110877840600203652;
                }
            }
            match current_block_4 {
                9110877840600203652 => return 0 as *const i8,
                _ => {}
            }
            p = p.offset(1);
            p;
        }
    }
    return 0 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn tiprev() -> *const i8 {
    let mut tip: *mut delta = 0 as *mut delta;
    let mut defbr: *const i8 = (*(*top).repository.r).branch;
    return if !defbr.is_null() {
        branchtip(defbr)
    } else {
        tip = (*top).repository.tip;
        if !tip.is_null() { (*tip).num } else { 0 as *const i8 }
    };
}