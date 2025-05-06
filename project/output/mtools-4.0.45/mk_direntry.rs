#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, label_break_value)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type doscp_t;
    fn free(__ptr: *mut libc::c_void);
    static mut stderr: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fgetc(__stream: *mut FILE) -> i32;
    fn fgets(__s: *mut i8, __n: i32, __stream: *mut FILE) -> *mut i8;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(_: *mut i8, _: *const i8, _: u64) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut short_illegals: *const i8;
    static mut long_illegals: *const i8;
    fn unix_normalize(
        cp: *mut doscp_t,
        ans: *mut i8,
        dn: *mut dos_name_t,
        ans_size: size_t,
    ) -> *mut i8;
    fn dos_name(
        cp: *mut doscp_t,
        filename: *const i8,
        verbose: i32,
        mangled: *mut i32,
        _: *mut dos_name_t,
    );
    fn opentty(mode: i32) -> *mut FILE;
    fn dir_grow(Dir: *mut Stream_t, size: u32) -> i32;
    static mut got_signal: i32;
    static mut mtools_raw_tty: i32;
    static mut progname: *const i8;
    fn ask_confirmation(_: *const i8, _: ...) -> i32;
    fn lookupForInsert(
        Dir: *mut Stream_t,
        direntry: *mut direntry_t,
        dosname: *mut dos_name_t,
        longname: *mut i8,
        ssp: *mut scan_state,
        ignore_entry: i32,
        source_entry: i32,
        pessimisticShortRename: i32,
        use_longname: i32,
    ) -> i32;
    fn dir_read(entry: *mut direntry_t, error: *mut i32) -> *mut directory;
    fn setEntryToPos(entry: *mut direntry_t, pos: u32);
    fn fatFreeWithDirentry(entry: *mut direntry_t) -> i32;
    fn write_vfat(
        _: *mut Stream_t,
        _: *mut dos_name_t,
        _: *mut i8,
        _: u32,
        _: *mut direntry_t,
    ) -> u32;
    fn wipeEntry(entry: *mut direntry_t);
    fn autorename_short(_: *mut dos_name_t, _: i32);
    fn autorename_long(_: *mut i8, _: i32);
    fn fat_error(Dir: *mut Stream_t) -> i32;
    fn native_to_wchar(
        native: *const i8,
        wchar: *mut wchar_t,
        len: size_t,
        end: *const i8,
        mangled: *mut i32,
    ) -> size_t;
    fn isSpecial(name: *const i8) -> i32;
}
pub type __uint8_t = u8;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type wchar_t = i32;
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    _ISalnum = 8,
    _ISpunct = 4,
    _IScntrl = 2,
    _ISblank = 1,
    _ISgraph = 32768,
    _ISprint = 16384,
    _ISspace = 8192,
    _ISxdigit = 4096,
    _ISdigit = 2048,
    _ISalpha = 1024,
    _ISlower = 512,
    _ISupper = 256,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::_ISalnum => 8,
            C2RustUnnamed::_ISpunct => 4,
            C2RustUnnamed::_IScntrl => 2,
            C2RustUnnamed::_ISblank => 1,
            C2RustUnnamed::_ISgraph => 32768,
            C2RustUnnamed::_ISprint => 16384,
            C2RustUnnamed::_ISspace => 8192,
            C2RustUnnamed::_ISxdigit => 4096,
            C2RustUnnamed::_ISdigit => 2048,
            C2RustUnnamed::_ISalpha => 1024,
            C2RustUnnamed::_ISlower => 512,
            C2RustUnnamed::_ISupper => 256,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            8 => C2RustUnnamed::_ISalnum,
            4 => C2RustUnnamed::_ISpunct,
            2 => C2RustUnnamed::_IScntrl,
            1 => C2RustUnnamed::_ISblank,
            32768 => C2RustUnnamed::_ISgraph,
            16384 => C2RustUnnamed::_ISprint,
            8192 => C2RustUnnamed::_ISspace,
            4096 => C2RustUnnamed::_ISxdigit,
            2048 => C2RustUnnamed::_ISdigit,
            1024 => C2RustUnnamed::_ISalpha,
            512 => C2RustUnnamed::_ISlower,
            256 => C2RustUnnamed::_ISupper,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type mt_off_t = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dos_name_t {
    pub base: [i8; 8],
    pub ext: [i8; 3],
    pub sentinel: i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream_t {
    pub Class: *mut Class_t,
    pub refs: i32,
    pub Next: *mut Stream_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Class_t {
    pub read: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut Stream_t, *mut i8, size_t) -> ssize_t>,
    pub pread: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub pwrite: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut i8, mt_off_t, size_t) -> ssize_t,
    >,
    pub flush: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub freeFunc: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
    pub set_geom: Option<
        unsafe extern "C" fn(*mut Stream_t, *mut device_t, *mut device_t) -> i32,
    >,
    pub get_data: Option<
        unsafe extern "C" fn(
            *mut Stream_t,
            *mut time_t,
            *mut mt_off_t,
            *mut i32,
            *mut uint32_t,
        ) -> i32,
    >,
    pub pre_allocate: Option<unsafe extern "C" fn(*mut Stream_t, mt_off_t) -> i32>,
    pub get_dosConvert: Option<unsafe extern "C" fn(*mut Stream_t) -> *mut doscp_t>,
    pub discard: Option<unsafe extern "C" fn(*mut Stream_t) -> i32>,
}
pub type device_t = device;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct device {
    pub name: *const i8,
    pub drive: i8,
    pub fat_bits: i32,
    pub mode: i32,
    pub tracks: u32,
    pub heads: uint16_t,
    pub sectors: uint16_t,
    pub hidden: u32,
    pub offset: mt_off_t,
    pub partition: u32,
    pub misc_flags: u32,
    pub ssize: uint8_t,
    pub use_2m: u32,
    pub precmd: *mut i8,
    pub file_nr: i32,
    pub blocksize: u32,
    pub codepage: u32,
    pub data_map: *const i8,
    pub tot_sectors: uint32_t,
    pub sector_size: uint16_t,
    pub postcmd: *mut i8,
    pub cfg_filename: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct directory {
    pub name: [i8; 8],
    pub ext: [i8; 3],
    pub attr: u8,
    pub Case: u8,
    pub ctime_ms: u8,
    pub ctime: [u8; 2],
    pub cdate: [u8; 2],
    pub adate: [u8; 2],
    pub startHi: [u8; 2],
    pub time: [u8; 2],
    pub date: [u8; 2],
    pub start: [u8; 2],
    pub size: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scan_state {
    pub match_free: i32,
    pub shortmatch: i32,
    pub longmatch: i32,
    pub free_start: u32,
    pub free_end: u32,
    pub slot: u32,
    pub got_slots: i32,
    pub size_needed: u32,
    pub max_entry: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub Dir: *mut Stream_t,
    pub entry: i32,
    pub dir: directory,
    pub name: [wchar_t; 256],
    pub beginSlot: u32,
    pub endSlot: u32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum clash_action {
    NAMEMATCH_GREW = 9,
    NAMEMATCH_SUCCESS = 8,
    NAMEMATCH_ERROR = 7,
    NAMEMATCH_OVERWRITE = 6,
    NAMEMATCH_PRENAME = 5,
    NAMEMATCH_RENAME = 4,
    NAMEMATCH_SKIP = 3,
    NAMEMATCH_QUIT = 2,
    NAMEMATCH_AUTORENAME = 1,
    NAMEMATCH_NONE = 0,
}
impl clash_action {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            clash_action::NAMEMATCH_GREW => 9,
            clash_action::NAMEMATCH_SUCCESS => 8,
            clash_action::NAMEMATCH_ERROR => 7,
            clash_action::NAMEMATCH_OVERWRITE => 6,
            clash_action::NAMEMATCH_PRENAME => 5,
            clash_action::NAMEMATCH_RENAME => 4,
            clash_action::NAMEMATCH_SKIP => 3,
            clash_action::NAMEMATCH_QUIT => 2,
            clash_action::NAMEMATCH_AUTORENAME => 1,
            clash_action::NAMEMATCH_NONE => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> clash_action {
        match value {
            9 => clash_action::NAMEMATCH_GREW,
            8 => clash_action::NAMEMATCH_SUCCESS,
            7 => clash_action::NAMEMATCH_ERROR,
            6 => clash_action::NAMEMATCH_OVERWRITE,
            5 => clash_action::NAMEMATCH_PRENAME,
            4 => clash_action::NAMEMATCH_RENAME,
            3 => clash_action::NAMEMATCH_SKIP,
            2 => clash_action::NAMEMATCH_QUIT,
            1 => clash_action::NAMEMATCH_AUTORENAME,
            0 => clash_action::NAMEMATCH_NONE,
            _ => panic!("Invalid value for clash_action: {}", value),
        }
    }
}
impl AddAssign<u32> for clash_action {
    fn add_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for clash_action {
    fn sub_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for clash_action {
    fn mul_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for clash_action {
    fn div_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for clash_action {
    fn rem_assign(&mut self, rhs: u32) {
        *self = clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for clash_action {
    type Output = clash_action;
    fn add(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for clash_action {
    type Output = clash_action;
    fn sub(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for clash_action {
    type Output = clash_action;
    fn mul(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for clash_action {
    type Output = clash_action;
    fn div(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for clash_action {
    type Output = clash_action;
    fn rem(self, rhs: u32) -> clash_action {
        clash_action::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClashHandling_t {
    pub action: [clash_action; 2],
    pub namematch_default: [clash_action; 2],
    pub nowarn: i32,
    pub got_slots: i32,
    pub mod_time: i32,
    pub myname: *mut i8,
    pub dosname: *mut u8,
    pub single: i32,
    pub use_longname: i32,
    pub ignore_entry: i32,
    pub source: i32,
    pub source_entry: i32,
    pub name_converter: Option<
        unsafe extern "C" fn(
            *mut doscp_t,
            *const i8,
            i32,
            *mut i32,
            *mut dos_name_t,
        ) -> (),
    >,
    pub is_label: i32,
}
pub type write_data_callback = unsafe extern "C" fn(
    *mut dos_name_t,
    *mut i8,
    *mut libc::c_void,
    *mut direntry_t,
) -> i32;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn convert_to_shortname(
    mut cp: *mut doscp_t,
    mut ch: *mut ClashHandling_t,
    mut un: *const i8,
    mut dn: *mut dos_name_t,
) -> i32 {
    let mut mangled: i32 = 0;
    ((*ch).name_converter)
        .expect("non-null function pointer")(cp, un, 0 as i32, &mut mangled, dn);
    (*dn).sentinel = '\0' as i32 as i8;
    if (*dn).base[0 as i32 as usize] as i32 == -27i32 {
        (*dn).base[0 as i32 as usize] = '\u{5}' as i32 as i8;
    }
    return mangled;
}
#[inline]
unsafe extern "C" fn chomp(mut line: *mut i8) {
    let mut l: size_t = strlen(line);
    while l > 0 as i32 as u64
        && (*line.offset(l.wrapping_sub(1 as i32 as u64) as isize) as i32 == '\n' as i32
            || *line.offset(l.wrapping_sub(1 as i32 as u64) as isize) as i32
                == '\r' as i32)
    {
        l = l.wrapping_sub(1);
        *line.offset(l as isize) = '\0' as i32 as i8;
    }
}
#[inline]
unsafe extern "C" fn ask_rename(
    mut cp: *mut doscp_t,
    mut ch: *mut ClashHandling_t,
    mut shortname: *mut dos_name_t,
    mut longname: *mut i8,
    mut isprimary: i32,
) -> i32 {
    let mut mangled: i32 = 0;
    if (opentty(0 as i32)).is_null() {
        return 0 as i32;
    }
    mangled = 0 as i32;
    loop {
        let mut tname: [i8; 1021] = [0; 1021];
        fprintf(
            stderr,
            b"New %s name for \"%s\": \0" as *const u8 as *const i8,
            if isprimary != 0 {
                b"primary\0" as *const u8 as *const i8
            } else {
                b"secondary\0" as *const u8 as *const i8
            },
            longname,
        );
        fflush(stderr);
        if (fgets(
            tname.as_mut_ptr(),
            4 as i32 * 255 as i32 + 1 as i32,
            opentty(0 as i32),
        ))
            .is_null()
        {
            return 0 as i32;
        }
        chomp(tname.as_mut_ptr());
        if isprimary != 0 {
            strcpy(longname, tname.as_mut_ptr());
        } else {
            mangled = convert_to_shortname(cp, ch, tname.as_mut_ptr(), shortname);
        }
        if !(mangled & 1 as i32 != 0) {
            break;
        }
    }
    return 1 as i32;
}
#[inline]
unsafe extern "C" fn ask_namematch(
    mut cp: *mut doscp_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut isprimary: i32,
    mut ch: *mut ClashHandling_t,
    mut no_overwrite: i32,
    mut reason: i32,
) -> clash_action {
    let mut ans: [i8; 10] = [0; 10];
    let mut a: clash_action = clash_action::NAMEMATCH_NONE;
    let mut perm: i32 = 0;
    let mut name_buffer: [i8; 52] = [0; 52];
    let mut name: *mut i8 = 0 as *mut i8;
    static mut reasons: [*const i8; 3] = [
        b"already exists\0" as *const u8 as *const i8,
        b"is reserved\0" as *const u8 as *const i8,
        b"contains illegal character(s)\0" as *const u8 as *const i8,
    ];
    a = (*ch).action[isprimary as usize];
    if a as u32 == clash_action::NAMEMATCH_NONE as i32 as u32
        && (opentty(1 as i32)).is_null()
    {
        return clash_action::NAMEMATCH_SKIP;
    }
    if isprimary == 0 {
        name = unix_normalize(
            cp,
            name_buffer.as_mut_ptr(),
            dosname,
            ::core::mem::size_of::<dos_name_t>() as u64,
        );
    } else {
        name = longname;
    }
    perm = 0 as i32;
    while a as u32 == clash_action::NAMEMATCH_NONE as i32 as u32 {
        fprintf(
            stderr,
            b"%s file name \"%s\" %s.\n\0" as *const u8 as *const i8,
            if isprimary != 0 {
                b"Long\0" as *const u8 as *const i8
            } else {
                b"Short\0" as *const u8 as *const i8
            },
            name,
            reasons[reason as usize],
        );
        fprintf(
            stderr,
            b"a)utorename A)utorename-all r)ename R)ename-all \0" as *const u8
                as *const i8,
        );
        if no_overwrite == 0 {
            fprintf(stderr, b"o)verwrite O)verwrite-all\0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"\ns)kip S)kip-all q)uit (aArR\0" as *const u8 as *const i8);
        if no_overwrite == 0 {
            fprintf(stderr, b"oO\0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"sSq): \0" as *const u8 as *const i8);
        fflush(stderr);
        fflush(opentty(1 as i32));
        if mtools_raw_tty != 0 {
            let mut rep: i32 = 0;
            rep = fgetc(opentty(1 as i32));
            fputs(b"\n\0" as *const u8 as *const i8, stderr);
            if rep == -(1 as i32) {
                ans[0 as i32 as usize] = 'q' as i32 as i8;
            } else {
                ans[0 as i32 as usize] = rep as i8;
            }
        } else if (fgets(ans.as_mut_ptr(), 9 as i32, opentty(0 as i32))).is_null() {
            ans[0 as i32 as usize] = 'q' as i32 as i8;
        }
        perm = *(*__ctype_b_loc()).offset(ans[0 as i32 as usize] as u8 as i32 as isize)
            as i32 & C2RustUnnamed::_ISupper as i32 as libc::c_ushort as i32;
        match ({
            let mut __res: i32 = 0;
            if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                if 0 != 0 {
                    let mut __c: i32 = ans[0 as i32 as usize] as u8 as i32;
                    __res = if __c < -(128 as i32) || __c > 255 as i32 {
                        __c
                    } else {
                        *(*__ctype_tolower_loc()).offset(__c as isize)
                    };
                } else {
                    __res = tolower(ans[0 as i32 as usize] as u8 as i32);
                }
            } else {
                __res = *(*__ctype_tolower_loc())
                    .offset(ans[0 as i32 as usize] as u8 as i32 as isize);
            }
            __res
        }) {
            97 => {
                a = clash_action::NAMEMATCH_AUTORENAME;
            }
            114 => {
                if isprimary != 0 {
                    a = clash_action::NAMEMATCH_PRENAME;
                } else {
                    a = clash_action::NAMEMATCH_RENAME;
                }
            }
            111 => {
                if no_overwrite != 0 {
                    continue;
                }
                a = clash_action::NAMEMATCH_OVERWRITE;
            }
            115 => {
                a = clash_action::NAMEMATCH_SKIP;
            }
            113 => {
                perm = 0 as i32;
                a = clash_action::NAMEMATCH_QUIT;
            }
            _ => {
                perm = 0 as i32;
            }
        }
    }
    (*ch).action[isprimary as usize] = a;
    if perm != 0 {
        (*ch).namematch_default[isprimary as usize] = a;
    }
    if a as u32 == clash_action::NAMEMATCH_OVERWRITE as i32 as u32 {
        (*ch).action[isprimary as usize] = clash_action::NAMEMATCH_NONE;
    }
    return a;
}
#[inline]
unsafe extern "C" fn process_namematch(
    mut cp: *mut doscp_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut isprimary: i32,
    mut ch: *mut ClashHandling_t,
    mut no_overwrite: i32,
    mut reason: i32,
) -> clash_action {
    let mut action: clash_action = clash_action::NAMEMATCH_NONE;
    action = ask_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    match action as u32 {
        2 => {
            got_signal = 1 as i32;
            return clash_action::NAMEMATCH_SKIP;
        }
        3 => return clash_action::NAMEMATCH_SKIP,
        4 | 5 => {
            ask_rename(cp, ch, dosname, longname, isprimary);
            return action;
        }
        1 => {
            if isprimary != 0 {
                autorename_long(longname, 1 as i32);
                return clash_action::NAMEMATCH_PRENAME;
            } else {
                autorename_short(dosname, 1 as i32);
                return clash_action::NAMEMATCH_RENAME;
            }
        }
        6 => {
            if no_overwrite != 0 {
                return clash_action::NAMEMATCH_SKIP
            } else {
                return clash_action::NAMEMATCH_OVERWRITE
            }
        }
        0 | 7 | 8 | 9 => return clash_action::NAMEMATCH_NONE,
        _ => {}
    }
    return action;
}
unsafe extern "C" fn contains_illegals(
    mut string: *const i8,
    mut illegals: *const i8,
    mut len: i32,
) -> i32 {
    while *string as i32 != 0
        && {
            let fresh0 = len;
            len = len - 1;
            fresh0 != 0
        }
    {
        if (*string as i32) < ' ' as i32 && *string as i32 != '\u{5}' as i32
            && *string as i32 & 0x80 as i32 == 0
            || !(strchr(illegals, *string as i32)).is_null()
        {
            return 1 as i32;
        }
        string = string.offset(1);
        string;
    }
    return 0 as i32;
}
unsafe extern "C" fn is_reserved(mut ans: *mut i8, mut islong: i32) -> i32 {
    let mut i: u32 = 0;
    static mut dev3: [*const i8; 5] = [
        b"CON\0" as *const u8 as *const i8,
        b"AUX\0" as *const u8 as *const i8,
        b"PRN\0" as *const u8 as *const i8,
        b"NUL\0" as *const u8 as *const i8,
        b"   \0" as *const u8 as *const i8,
    ];
    static mut dev4: [*const i8; 2] = [
        b"COM\0" as *const u8 as *const i8,
        b"LPT\0" as *const u8 as *const i8,
    ];
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[*const i8; 5]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        if strncasecmp(ans, dev3[i as usize], 3 as i32 as u64) == 0
            && (islong != 0 && *ans.offset(3 as i32 as isize) == 0
                || islong == 0
                    && strncmp(
                        ans.offset(3 as i32 as isize),
                        b"     \0" as *const u8 as *const i8,
                        5 as i32 as u64,
                    ) == 0)
        {
            return 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as i32 as u32;
    while (i as u64)
        < (::core::mem::size_of::<[*const i8; 2]>() as u64)
            .wrapping_div(::core::mem::size_of::<*const i8>() as u64)
    {
        if strncasecmp(ans, dev4[i as usize], 3 as i32 as u64) == 0
            && (*ans.offset(3 as i32 as isize) as i32 >= '1' as i32
                && *ans.offset(3 as i32 as isize) as i32 <= '4' as i32)
            && (islong != 0 && *ans.offset(4 as i32 as isize) == 0
                || islong == 0
                    && strncmp(
                        ans.offset(4 as i32 as isize),
                        b"    \0" as *const u8 as *const i8,
                        4 as i32 as u64,
                    ) == 0)
        {
            return 1 as i32;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn get_slots(
    mut Dir: *mut Stream_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut ssp: *mut scan_state,
    mut ch: *mut ClashHandling_t,
) -> clash_action {
    let mut error: i32 = 0;
    let mut ret: clash_action = clash_action::NAMEMATCH_NONE;
    let mut match_pos: i32 = 0 as i32;
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    let mut isprimary: i32 = 0;
    let mut no_overwrite: i32 = 0;
    let mut reason: i32 = 0;
    let mut pessimisticShortRename: i32 = 0;
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert)
        .expect("non-null function pointer")(Dir);
    pessimisticShortRename = ((*ch).action[0 as i32 as usize] as u32
        == clash_action::NAMEMATCH_AUTORENAME as i32 as u32) as i32;
    entry.Dir = Dir;
    no_overwrite = 1 as i32;
    if is_reserved(longname, 1 as i32) != 0
        || *longname.offset(strspn(longname, b". \0" as *const u8 as *const i8) as isize)
            as i32 == '\0' as i32
    {
        reason = 1 as i32;
        isprimary = 1 as i32;
    } else if contains_illegals(longname, long_illegals, 1024 as i32) != 0 {
        reason = 2 as i32;
        isprimary = 1 as i32;
    } else if is_reserved(((*dosname).base).as_mut_ptr(), 0 as i32) != 0 {
        reason = 1 as i32;
        (*ch).use_longname = 1 as i32;
        isprimary = 0 as i32;
    } else if (*ch).is_label == 0
        && contains_illegals(((*dosname).base).as_mut_ptr(), short_illegals, 11 as i32)
            != 0
    {
        reason = 2 as i32;
        (*ch).use_longname = 1 as i32;
        isprimary = 0 as i32;
    } else {
        reason = 0 as i32;
        match lookupForInsert(
            Dir,
            &mut entry,
            dosname,
            longname,
            ssp,
            (*ch).ignore_entry,
            (*ch).source_entry,
            (pessimisticShortRename != 0 && (*ch).use_longname != 0) as i32,
            (*ch).use_longname,
        ) {
            -1 => return clash_action::NAMEMATCH_ERROR,
            0 => return clash_action::NAMEMATCH_SKIP,
            5 => return clash_action::NAMEMATCH_GREW,
            6 => return clash_action::NAMEMATCH_SUCCESS,
            _ => {}
        }
        if (*ssp).longmatch >= 0 as i32 {
            match_pos = (*ssp).longmatch;
            isprimary = 1 as i32;
        } else if (*ch).use_longname & 1 as i32 != 0 && (*ssp).shortmatch != -(1 as i32)
        {
            match_pos = (*ssp).shortmatch;
            isprimary = 0 as i32;
        } else if (*ssp).shortmatch >= 0 as i32 {
            match_pos = (*ssp).shortmatch;
            isprimary = 1 as i32;
        } else {
            return clash_action::NAMEMATCH_RENAME
        }
        if match_pos > -(1 as i32) {
            entry.entry = match_pos;
            dir_read(&mut entry, &mut error);
            if error != 0 {
                return clash_action::NAMEMATCH_ERROR;
            }
            no_overwrite = (match_pos == (*ch).source
                || entry.dir.attr as i32 & 0x10 as i32 != 0) as i32;
        }
    }
    ret = process_namematch(cp, dosname, longname, isprimary, ch, no_overwrite, reason);
    if ret as u32 == clash_action::NAMEMATCH_OVERWRITE as i32 as u32
        && match_pos > -(1 as i32)
    {
        if entry.dir.attr as i32 & 0x5 as i32 != 0
            && ask_confirmation(
                b"file is read only, overwrite anyway (y/n) ? \0" as *const u8
                    as *const i8,
            ) != 0
        {
            return clash_action::NAMEMATCH_RENAME;
        }
        if fatFreeWithDirentry(&mut entry) != 0 {
            return clash_action::NAMEMATCH_ERROR;
        }
        wipeEntry(&mut entry);
        return clash_action::NAMEMATCH_RENAME;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn write_slots(
    mut Dir: *mut Stream_t,
    mut dosname: *mut dos_name_t,
    mut longname: *mut i8,
    mut ssp: *mut scan_state,
    mut cb: Option<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut Case: i32,
) -> i32 {
    let mut entry: direntry_t = direntry_t {
        Dir: 0 as *mut Stream_t,
        entry: 0,
        dir: directory {
            name: [0; 8],
            ext: [0; 3],
            attr: 0,
            Case: 0,
            ctime_ms: 0,
            ctime: [0; 2],
            cdate: [0; 2],
            adate: [0; 2],
            startHi: [0; 2],
            time: [0; 2],
            date: [0; 2],
            start: [0; 2],
            size: [0; 4],
        },
        name: [0; 256],
        beginSlot: 0,
        endSlot: 0,
    };
    if fat_error(Dir) != 0 {
        return 0 as i32;
    }
    entry.Dir = Dir;
    if (*ssp).got_slots != 0 {} else {
        __assert_fail(
            b"ssp->got_slots\0" as *const u8 as *const i8,
            b"mk_direntry.c\0" as *const u8 as *const i8,
            499 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 107],
                &[i8; 107],
            >(
                b"int write_slots(Stream_t *, dos_name_t *, char *, struct scan_state *, write_data_callback *, void *, int)\0",
            ))
                .as_ptr(),
        );
    }
    'c_7749: {
        if (*ssp).got_slots != 0 {} else {
            __assert_fail(
                b"ssp->got_slots\0" as *const u8 as *const i8,
                b"mk_direntry.c\0" as *const u8 as *const i8,
                499 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 107],
                    &[i8; 107],
                >(
                    b"int write_slots(Stream_t *, dos_name_t *, char *, struct scan_state *, write_data_callback *, void *, int)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    setEntryToPos(&mut entry, (*ssp).slot);
    native_to_wchar(
        longname,
        (entry.name).as_mut_ptr(),
        255 as i32 as size_t,
        0 as *const i8,
        0 as *mut i32,
    );
    entry.name[255 as i32 as usize] = '\0' as i32;
    entry.dir.Case = (Case & (0x10 as i32 | 0x8 as i32)) as u8;
    if cb.expect("non-null function pointer")(dosname, longname, arg, &mut entry)
        >= 0 as i32
    {
        if (*ssp).size_needed > 1 as i32 as u32
            && ((*ssp).free_end).wrapping_sub((*ssp).free_start) >= (*ssp).size_needed
        {
            (*ssp).slot = write_vfat(
                Dir,
                dosname,
                longname,
                (*ssp).free_start,
                &mut entry,
            );
        } else {
            (*ssp).size_needed = 1 as i32 as u32;
            write_vfat(Dir, dosname, 0 as *mut i8, (*ssp).free_start, &mut entry);
        }
    } else {
        return 0 as i32
    }
    return 1 as i32;
}
unsafe extern "C" fn stripspaces(mut name: *mut i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut non_space: *mut i8 = 0 as *mut i8;
    non_space = name;
    p = name;
    while *p != 0 {
        if *p as i32 != ' ' as i32 {
            non_space = p;
        }
        p = p.offset(1);
        p;
    }
    if *name.offset(0 as i32 as isize) != 0 {
        *non_space.offset(1 as i32 as isize) = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn mt_mwrite_one(
    mut Dir: *mut Stream_t,
    mut argname: *mut i8,
    mut shortname: *mut i8,
    mut cb: Option<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut ch: *mut ClashHandling_t,
) -> i32 {
    let mut longname: [i8; 261] = [0; 261];
    let mut dstname: *const i8 = 0 as *const i8;
    let mut dosname: dos_name_t = dos_name_t {
        base: [0; 8],
        ext: [0; 3],
        sentinel: 0,
    };
    let mut expanded: i32 = 0;
    let mut scan: scan_state = scan_state {
        match_free: 0,
        shortmatch: 0,
        longmatch: 0,
        free_start: 0,
        free_end: 0,
        slot: 0,
        got_slots: 0,
        size_needed: 0,
        max_entry: 0,
    };
    let mut ret: clash_action = clash_action::NAMEMATCH_NONE;
    let mut cp: *mut doscp_t = ((*(*Dir).Class).get_dosConvert)
        .expect("non-null function pointer")(Dir);
    expanded = 0 as i32;
    if isSpecial(argname) != 0 {
        fprintf(
            stderr,
            b"Cannot create entry named . or ..\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    if (*ch).name_converter
        == Some(
            dos_name
                as unsafe extern "C" fn(
                    *mut doscp_t,
                    *const i8,
                    i32,
                    *mut i32,
                    *mut dos_name_t,
                ) -> (),
        )
    {
        if !shortname.is_null() {
            stripspaces(shortname);
        }
        if !argname.is_null() {
            stripspaces(argname);
        }
    }
    if !shortname.is_null() {
        convert_to_shortname(cp, ch, shortname, &mut dosname);
        if (*ch).use_longname & 1 as i32 != 0 {
            argname = shortname;
            shortname = 0 as *mut i8;
        }
    }
    if *argname.offset(0 as i32 as isize) as i32 != 0
        && *argname.offset(1 as i32 as isize) as i32 == ':' as i32
    {
        dstname = argname.offset(2 as i32 as isize);
    } else {
        dstname = argname;
    }
    strncpy(
        longname.as_mut_ptr(),
        dstname,
        (20 as i32 * 13 as i32 + 1 as i32 - 1 as i32) as u64,
    );
    if !shortname.is_null() {
        (*ch).use_longname = convert_to_shortname(cp, ch, shortname, &mut dosname);
        if strcmp(shortname, longname.as_mut_ptr()) != 0 {
            (*ch).use_longname |= 1 as i32;
        }
    } else {
        (*ch).use_longname = convert_to_shortname(
            cp,
            ch,
            longname.as_mut_ptr(),
            &mut dosname,
        );
    }
    (*ch).action[0 as i32 as usize] = (*ch).namematch_default[0 as i32 as usize];
    (*ch).action[1 as i32 as usize] = (*ch).namematch_default[1 as i32 as usize];
    loop {
        ret = get_slots(Dir, &mut dosname, longname.as_mut_ptr(), &mut scan, ch);
        match ret as u32 {
            7 => return -(1 as i32),
            3 => return -(1 as i32),
            5 => {
                (*ch).use_longname = convert_to_shortname(
                    cp,
                    ch,
                    longname.as_mut_ptr(),
                    &mut dosname,
                );
            }
            9 => {
                if expanded != 0 {
                    fprintf(
                        stderr,
                        b"%s: No directory slots\n\0" as *const u8 as *const i8,
                        progname,
                    );
                    return -(1 as i32);
                }
                expanded = 1 as i32;
                if dir_grow(Dir, scan.max_entry) != 0 {
                    return -(1 as i32);
                }
            }
            6 | 8 => {
                return write_slots(
                    Dir,
                    &mut dosname,
                    longname.as_mut_ptr(),
                    &mut scan,
                    cb,
                    arg,
                    (*ch).use_longname,
                );
            }
            0 | 1 | 2 => {
                fprintf(
                    stderr,
                    b"Internal error: clash_action=%d\n\0" as *const u8 as *const i8,
                    ret as u32,
                );
                return -(1 as i32);
            }
            4 | _ => {}
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mwrite_one(
    mut Dir: *mut Stream_t,
    mut _argname: *const i8,
    mut _shortname: *const i8,
    mut cb: Option<write_data_callback>,
    mut arg: *mut libc::c_void,
    mut ch: *mut ClashHandling_t,
) -> i32 {
    let mut argname: *mut i8 = 0 as *mut i8;
    let mut shortname: *mut i8 = 0 as *mut i8;
    let mut ret: i32 = 0;
    if !_argname.is_null() {
        argname = strdup(_argname);
    } else {
        argname = 0 as *mut i8;
    }
    if !_shortname.is_null() {
        shortname = strdup(_shortname);
    } else {
        shortname = 0 as *mut i8;
    }
    ret = mt_mwrite_one(Dir, argname, shortname, cb, arg, ch);
    if !argname.is_null() {
        free(argname as *mut libc::c_void);
    }
    if !shortname.is_null() {
        free(shortname as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn init_clash_handling(mut ch: *mut ClashHandling_t) {
    (*ch).ignore_entry = -(1 as i32);
    (*ch).source_entry = -(2 as i32);
    (*ch).nowarn = 0 as i32;
    (*ch).namematch_default[0 as i32 as usize] = clash_action::NAMEMATCH_AUTORENAME;
    (*ch).namematch_default[1 as i32 as usize] = clash_action::NAMEMATCH_NONE;
    (*ch).name_converter = Some(
        dos_name
            as unsafe extern "C" fn(
                *mut doscp_t,
                *const i8,
                i32,
                *mut i32,
                *mut dos_name_t,
            ) -> (),
    );
    (*ch).source = -(2 as i32);
    (*ch).is_label = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn handle_clash_options(
    mut ch: *mut ClashHandling_t,
    mut c: i32,
) -> i32 {
    let mut isprimary: i32 = 0;
    if *(*__ctype_b_loc()).offset(c as isize) as i32
        & C2RustUnnamed::_ISupper as i32 as libc::c_ushort as i32 != 0
    {
        isprimary = 0 as i32;
    } else {
        isprimary = 1 as i32;
    }
    c = ({
        let mut __res: i32 = 0;
        if ::core::mem::size_of::<i32>() as u64 > 1 as i32 as u64 {
            if 0 != 0 {
                let mut __c: i32 = c;
                __res = if __c < -(128 as i32) || __c > 255 as i32 {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(c);
            }
        } else {
            __res = *(*__ctype_tolower_loc()).offset(c as isize);
        }
        __res
    });
    match c {
        111 => {
            (*ch).namematch_default[isprimary as usize] = clash_action::NAMEMATCH_OVERWRITE;
            return 0 as i32;
        }
        114 => {
            (*ch).namematch_default[isprimary as usize] = clash_action::NAMEMATCH_RENAME;
            return 0 as i32;
        }
        115 => {
            (*ch).namematch_default[isprimary as usize] = clash_action::NAMEMATCH_SKIP;
            return 0 as i32;
        }
        109 => {
            (*ch).namematch_default[isprimary as usize] = clash_action::NAMEMATCH_NONE;
            return 0 as i32;
        }
        97 => {
            (*ch).namematch_default[isprimary as usize] = clash_action::NAMEMATCH_AUTORENAME;
            return 0 as i32;
        }
        _ => return -(1 as i32),
    };
}
#[no_mangle]
pub unsafe extern "C" fn dosnameToDirentry(
    mut dn: *const dos_name_t,
    mut dir: *mut directory,
) {
    strncpy(((*dir).name).as_mut_ptr(), ((*dn).base).as_ptr(), 8 as i32 as u64);
    strncpy(((*dir).ext).as_mut_ptr(), ((*dn).ext).as_ptr(), 3 as i32 as u64);
}