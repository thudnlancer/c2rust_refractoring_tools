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
    pub type __dirstream;
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn pathconf(__path: *const i8, __name: i32) -> i64;
    fn fpathconf(__fd: i32, __name: i32) -> i64;
    fn last_component(filename: *const i8) -> *mut i8;
    fn base_len(filename: *const i8) -> size_t;
    fn opendirat(_: i32, _: *const i8, _: i32, _: *mut i32) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn renameatu(_: i32, _: *const i8, _: i32, _: *const i8, _: u32) -> i32;
}
pub type __ino_t = u64;
pub type __off_t = i64;
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum backup_type {
    no_backups,
    simple_backups,
    numbered_existing_backups,
    numbered_backups,
}
impl backup_type {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            backup_type::no_backups => 0,
            backup_type::simple_backups => 1,
            backup_type::numbered_existing_backups => 2,
            backup_type::numbered_backups => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> backup_type {
        match value {
            0 => backup_type::no_backups,
            1 => backup_type::simple_backups,
            2 => backup_type::numbered_existing_backups,
            3 => backup_type::numbered_backups,
            _ => panic!("Invalid value for backup_type: {}", value),
        }
    }
}
impl AddAssign<u32> for backup_type {
    fn add_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for backup_type {
    fn sub_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for backup_type {
    fn mul_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for backup_type {
    fn div_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for backup_type {
    fn rem_assign(&mut self, rhs: u32) {
        *self = backup_type::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for backup_type {
    type Output = backup_type;
    fn add(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for backup_type {
    type Output = backup_type;
    fn sub(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for backup_type {
    type Output = backup_type;
    fn mul(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for backup_type {
    type Output = backup_type;
    fn div(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for backup_type {
    type Output = backup_type;
    fn rem(self, rhs: u32) -> backup_type {
        backup_type::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type idx_t = ptrdiff_t;
pub type ptrdiff_t = i64;
pub type DIR = __dirstream;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum numbered_backup_result {
    BACKUP_IS_SAME_LENGTH,
    BACKUP_IS_LONGER,
    BACKUP_IS_NEW,
    BACKUP_NOMEM,
}
impl numbered_backup_result {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            numbered_backup_result::BACKUP_IS_SAME_LENGTH => 0,
            numbered_backup_result::BACKUP_IS_LONGER => 1,
            numbered_backup_result::BACKUP_IS_NEW => 2,
            numbered_backup_result::BACKUP_NOMEM => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> numbered_backup_result {
        match value {
            0 => numbered_backup_result::BACKUP_IS_SAME_LENGTH,
            1 => numbered_backup_result::BACKUP_IS_LONGER,
            2 => numbered_backup_result::BACKUP_IS_NEW,
            3 => numbered_backup_result::BACKUP_NOMEM,
            _ => panic!("Invalid value for numbered_backup_result: {}", value),
        }
    }
}
impl AddAssign<u32> for numbered_backup_result {
    fn add_assign(&mut self, rhs: u32) {
        *self = numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for numbered_backup_result {
    fn sub_assign(&mut self, rhs: u32) {
        *self = numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for numbered_backup_result {
    fn mul_assign(&mut self, rhs: u32) {
        *self = numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for numbered_backup_result {
    fn div_assign(&mut self, rhs: u32) {
        *self = numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for numbered_backup_result {
    fn rem_assign(&mut self, rhs: u32) {
        *self = numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for numbered_backup_result {
    type Output = numbered_backup_result;
    fn add(self, rhs: u32) -> numbered_backup_result {
        numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for numbered_backup_result {
    type Output = numbered_backup_result;
    fn sub(self, rhs: u32) -> numbered_backup_result {
        numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for numbered_backup_result {
    type Output = numbered_backup_result;
    fn mul(self, rhs: u32) -> numbered_backup_result {
        numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for numbered_backup_result {
    type Output = numbered_backup_result;
    fn div(self, rhs: u32) -> numbered_backup_result {
        numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for numbered_backup_result {
    type Output = numbered_backup_result;
    fn rem(self, rhs: u32) -> numbered_backup_result {
        numbered_backup_result::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    _PC_NAME_MAX = 3,
    _PC_2_SYMLINKS = 20,
    _PC_SYMLINK_MAX = 19,
    _PC_ALLOC_SIZE_MIN = 18,
    _PC_REC_XFER_ALIGN = 17,
    _PC_REC_MIN_XFER_SIZE = 16,
    _PC_REC_MAX_XFER_SIZE = 15,
    _PC_REC_INCR_XFER_SIZE = 14,
    _PC_FILESIZEBITS = 13,
    _PC_SOCK_MAXBUF = 12,
    _PC_PRIO_IO = 11,
    _PC_ASYNC_IO = 10,
    _PC_SYNC_IO = 9,
    _PC_VDISABLE = 8,
    _PC_NO_TRUNC = 7,
    _PC_CHOWN_RESTRICTED = 6,
    _PC_PIPE_BUF = 5,
    _PC_PATH_MAX = 4,
    _PC_MAX_INPUT = 2,
    _PC_MAX_CANON = 1,
    _PC_LINK_MAX = 0,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::_PC_NAME_MAX => 3,
            C2RustUnnamed_0::_PC_2_SYMLINKS => 20,
            C2RustUnnamed_0::_PC_SYMLINK_MAX => 19,
            C2RustUnnamed_0::_PC_ALLOC_SIZE_MIN => 18,
            C2RustUnnamed_0::_PC_REC_XFER_ALIGN => 17,
            C2RustUnnamed_0::_PC_REC_MIN_XFER_SIZE => 16,
            C2RustUnnamed_0::_PC_REC_MAX_XFER_SIZE => 15,
            C2RustUnnamed_0::_PC_REC_INCR_XFER_SIZE => 14,
            C2RustUnnamed_0::_PC_FILESIZEBITS => 13,
            C2RustUnnamed_0::_PC_SOCK_MAXBUF => 12,
            C2RustUnnamed_0::_PC_PRIO_IO => 11,
            C2RustUnnamed_0::_PC_ASYNC_IO => 10,
            C2RustUnnamed_0::_PC_SYNC_IO => 9,
            C2RustUnnamed_0::_PC_VDISABLE => 8,
            C2RustUnnamed_0::_PC_NO_TRUNC => 7,
            C2RustUnnamed_0::_PC_CHOWN_RESTRICTED => 6,
            C2RustUnnamed_0::_PC_PIPE_BUF => 5,
            C2RustUnnamed_0::_PC_PATH_MAX => 4,
            C2RustUnnamed_0::_PC_MAX_INPUT => 2,
            C2RustUnnamed_0::_PC_MAX_CANON => 1,
            C2RustUnnamed_0::_PC_LINK_MAX => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            3 => C2RustUnnamed_0::_PC_NAME_MAX,
            20 => C2RustUnnamed_0::_PC_2_SYMLINKS,
            19 => C2RustUnnamed_0::_PC_SYMLINK_MAX,
            18 => C2RustUnnamed_0::_PC_ALLOC_SIZE_MIN,
            17 => C2RustUnnamed_0::_PC_REC_XFER_ALIGN,
            16 => C2RustUnnamed_0::_PC_REC_MIN_XFER_SIZE,
            15 => C2RustUnnamed_0::_PC_REC_MAX_XFER_SIZE,
            14 => C2RustUnnamed_0::_PC_REC_INCR_XFER_SIZE,
            13 => C2RustUnnamed_0::_PC_FILESIZEBITS,
            12 => C2RustUnnamed_0::_PC_SOCK_MAXBUF,
            11 => C2RustUnnamed_0::_PC_PRIO_IO,
            10 => C2RustUnnamed_0::_PC_ASYNC_IO,
            9 => C2RustUnnamed_0::_PC_SYNC_IO,
            8 => C2RustUnnamed_0::_PC_VDISABLE,
            7 => C2RustUnnamed_0::_PC_NO_TRUNC,
            6 => C2RustUnnamed_0::_PC_CHOWN_RESTRICTED,
            5 => C2RustUnnamed_0::_PC_PIPE_BUF,
            4 => C2RustUnnamed_0::_PC_PATH_MAX,
            2 => C2RustUnnamed_0::_PC_MAX_INPUT,
            1 => C2RustUnnamed_0::_PC_MAX_CANON,
            0 => C2RustUnnamed_0::_PC_LINK_MAX,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    GUESS = 9,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::GUESS => 9,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            9 => C2RustUnnamed::GUESS,
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
#[no_mangle]
pub static mut simple_backup_suffix: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn set_simple_backup_suffix(mut s: *const i8) {
    if s.is_null() {
        s = getenv(b"SIMPLE_BACKUP_SUFFIX\0" as *const u8 as *const i8);
    }
    simple_backup_suffix = if !s.is_null() && *s as i32 != 0 && s == last_component(s) {
        s
    } else {
        b"~\0" as *const u8 as *const i8
    };
}
unsafe extern "C" fn check_extension(
    mut file: *mut i8,
    mut filelen: size_t,
    mut e: i8,
    mut dir_fd: i32,
    mut base_max: *mut size_t,
) {
    let mut base: *mut i8 = last_component(file);
    let mut baselen: size_t = base_len(base);
    let mut baselen_max: size_t = (if 1 as i32 != 0 { 255 as i32 } else { 14 as i32 })
        as size_t;
    if 0 as i32 != 0 || (14 as i32 as u64) < baselen {
        if *base_max == 0 as i32 as u64 {
            let mut name_max: i64 = 0;
            if dir_fd < 0 as i32 {
                let mut tmp: [i8; 2] = [0; 2];
                memcpy(
                    tmp.as_mut_ptr() as *mut libc::c_void,
                    base as *const libc::c_void,
                    ::core::mem::size_of::<[i8; 2]>() as u64,
                );
                strcpy(base, b".\0" as *const u8 as *const i8);
                *__errno_location() = 0 as i32;
                name_max = pathconf(file, C2RustUnnamed_0::_PC_NAME_MAX as i32);
                name_max -= (*__errno_location() == 0) as i32 as i64;
                memcpy(
                    base as *mut libc::c_void,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[i8; 2]>() as u64,
                );
            } else {
                *__errno_location() = 0 as i32;
                name_max = fpathconf(dir_fd, C2RustUnnamed_0::_PC_NAME_MAX as i32);
                name_max -= (*__errno_location() == 0) as i32 as i64;
            }
            *base_max = if 0 as i32 as i64 <= name_max
                && name_max as u64 <= 18446744073709551615 as u64
            {
                name_max as u64
            } else if name_max < -(1 as i32) as i64 {
                14 as i32 as u64
            } else {
                18446744073709551615 as u64
            };
        }
        baselen_max = *base_max;
    }
    if 0 as i32 != 0 && baselen_max <= 12 as i32 as u64 {
        let mut dot: *mut i8 = strchr(base, '.' as i32);
        if dot.is_null() {
            baselen_max = 8 as i32 as size_t;
        } else {
            let mut second_dot: *const i8 = strchr(
                dot.offset(1 as i32 as isize),
                '.' as i32,
            );
            baselen_max = (if !second_dot.is_null() {
                second_dot.offset_from(base) as i64
            } else {
                dot.offset(1 as i32 as isize).offset_from(base) as i64 + 3 as i32 as i64
            }) as size_t;
        }
    }
    if baselen_max < baselen {
        baselen = file.offset(filelen as isize).offset_from(base) as i64 as size_t;
        if baselen_max <= baselen {
            baselen = baselen_max.wrapping_sub(1 as i32 as u64);
        }
        *base.offset(baselen as isize) = e;
        *base.offset(baselen.wrapping_add(1 as i32 as u64) as isize) = '\0' as i32 as i8;
    }
}
unsafe extern "C" fn numbered_backup(
    mut dir_fd: i32,
    mut buffer: *mut *mut i8,
    mut buffer_size: size_t,
    mut filelen: size_t,
    mut base_offset: idx_t,
    mut dirpp: *mut *mut DIR,
    mut pnew_fd: *mut i32,
) -> numbered_backup_result {
    let mut result: numbered_backup_result = numbered_backup_result::BACKUP_IS_NEW;
    let mut dirp: *mut DIR = *dirpp;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut buf: *mut i8 = *buffer;
    let mut versionlenmax: size_t = 1 as i32 as size_t;
    let mut base: *mut i8 = buf.offset(base_offset as isize);
    let mut baselen: size_t = base_len(base);
    if !dirp.is_null() {
        rewinddir(dirp);
    } else {
        let mut tmp: [i8; 2] = [0; 2];
        memcpy(
            tmp.as_mut_ptr() as *mut libc::c_void,
            base as *const libc::c_void,
            ::core::mem::size_of::<[i8; 2]>() as u64,
        );
        strcpy(base, b".\0" as *const u8 as *const i8);
        dirp = opendirat(dir_fd, buf, 0 as i32, pnew_fd);
        if dirp.is_null() && *__errno_location() == 12 as i32 {
            result = numbered_backup_result::BACKUP_NOMEM;
        }
        memcpy(
            base as *mut libc::c_void,
            tmp.as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[i8; 2]>() as u64,
        );
        strcpy(base.offset(baselen as isize), b".~1~\0" as *const u8 as *const i8);
        if dirp.is_null() {
            return result;
        }
        *dirpp = dirp;
    }
    loop {
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        let mut p: *const i8 = 0 as *const i8;
        let mut q: *mut i8 = 0 as *mut i8;
        let mut all_9s: bool = false;
        let mut versionlen: size_t = 0;
        if strlen(((*dp).d_name).as_mut_ptr()) < baselen.wrapping_add(4 as i32 as u64) {
            continue;
        }
        if memcmp(
            buf.offset(base_offset as isize) as *const libc::c_void,
            ((*dp).d_name).as_mut_ptr() as *const libc::c_void,
            baselen.wrapping_add(2 as i32 as u64),
        ) != 0 as i32
        {
            continue;
        }
        p = ((*dp).d_name)
            .as_mut_ptr()
            .offset(baselen as isize)
            .offset(2 as i32 as isize);
        if !('1' as i32 <= *p as i32 && *p as i32 <= '9' as i32) {
            continue;
        }
        all_9s = *p as i32 == '9' as i32;
        versionlen = 1 as i32 as size_t;
        while (*p.offset(versionlen as isize) as u32).wrapping_sub('0' as i32 as u32)
            <= 9 as i32 as u32
        {
            all_9s = (all_9s as i32
                & (*p.offset(versionlen as isize) as i32 == '9' as i32) as i32) as bool;
            versionlen = versionlen.wrapping_add(1);
            versionlen;
        }
        if !(*p.offset(versionlen as isize) as i32 == '~' as i32
            && *p.offset(versionlen.wrapping_add(1 as i32 as u64) as isize) == 0
            && (versionlenmax < versionlen
                || versionlenmax == versionlen
                    && memcmp(
                        buf.offset(filelen as isize).offset(2 as i32 as isize)
                            as *const libc::c_void,
                        p as *const libc::c_void,
                        versionlen,
                    ) <= 0 as i32))
        {
            continue;
        }
        versionlenmax = (all_9s as u64).wrapping_add(versionlen);
        result = numbered_backup_result::from_libc_c_uint(
            (if all_9s as i32 != 0 {
                numbered_backup_result::BACKUP_IS_LONGER as i32
            } else {
                numbered_backup_result::BACKUP_IS_SAME_LENGTH as i32
            }) as u32,
        );
        let mut new_buffer_size: size_t = filelen
            .wrapping_add(2 as i32 as u64)
            .wrapping_add(versionlenmax)
            .wrapping_add(2 as i32 as u64);
        if buffer_size < new_buffer_size {
            if !((if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
                9223372036854775807 as i64 as u64
            } else {
                (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
            })
                .wrapping_div(2 as i32 as u64) < new_buffer_size)
            {
                new_buffer_size = (new_buffer_size as u64).wrapping_mul(2 as i32 as u64)
                    as size_t as size_t;
            }
            let mut new_buf: *mut i8 = realloc(buf as *mut libc::c_void, new_buffer_size)
                as *mut i8;
            if new_buf.is_null() {
                *buffer = buf;
                return numbered_backup_result::BACKUP_NOMEM;
            }
            buf = new_buf;
            buffer_size = new_buffer_size;
        }
        q = buf.offset(filelen as isize);
        let fresh0 = q;
        q = q.offset(1);
        *fresh0 = '.' as i32 as i8;
        let fresh1 = q;
        q = q.offset(1);
        *fresh1 = '~' as i32 as i8;
        *q = '0' as i32 as i8;
        q = q.offset(all_9s as i32 as isize);
        memcpy(
            q as *mut libc::c_void,
            p as *const libc::c_void,
            versionlen.wrapping_add(2 as i32 as u64),
        );
        q = q.offset(versionlen as isize);
        loop {
            q = q.offset(-1);
            if !(*q as i32 == '9' as i32) {
                break;
            }
            *q = '0' as i32 as i8;
        }
        *q += 1;
        *q;
    }
    *buffer = buf;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn backupfile_internal(
    mut dir_fd: i32,
    mut file: *const i8,
    mut backup_type: backup_type,
    mut rename: bool,
) -> *mut i8 {
    let mut base_offset: idx_t = (last_component(file)).offset_from(file) as i64;
    let mut filelen: size_t = (base_offset as u64)
        .wrapping_add(strlen(file.offset(base_offset as isize)));
    if simple_backup_suffix.is_null() {
        set_simple_backup_suffix(0 as *const i8);
    }
    let mut simple_backup_suffix_size: size_t = (strlen(simple_backup_suffix))
        .wrapping_add(1 as i32 as u64);
    let mut backup_suffix_size_guess: size_t = simple_backup_suffix_size;
    if backup_suffix_size_guess < C2RustUnnamed::GUESS as i32 as u64 {
        backup_suffix_size_guess = C2RustUnnamed::GUESS as i32 as size_t;
    }
    let mut ssize: ssize_t = filelen
        .wrapping_add(backup_suffix_size_guess)
        .wrapping_add(1 as i32 as u64) as ssize_t;
    let mut s: *mut i8 = malloc(ssize as u64) as *mut i8;
    if s.is_null() {
        return s;
    }
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut sdir: i32 = -(1 as i32);
    let mut base_max: size_t = 0 as i32 as size_t;
    loop {
        memcpy(
            s as *mut libc::c_void,
            file as *const libc::c_void,
            filelen.wrapping_add(1 as i32 as u64),
        );
        if backup_type as u32 == backup_type::simple_backups as i32 as u32 {
            memcpy(
                s.offset(filelen as isize) as *mut libc::c_void,
                simple_backup_suffix as *const libc::c_void,
                simple_backup_suffix_size,
            );
        } else {
            let mut current_block_18: u64;
            match numbered_backup(
                dir_fd,
                &mut s,
                ssize as size_t,
                filelen,
                base_offset,
                &mut dirp,
                &mut sdir,
            ) as u32
            {
                2 => {
                    if backup_type as u32
                        == backup_type::numbered_existing_backups as i32 as u32
                    {
                        backup_type = backup_type::simple_backups;
                        memcpy(
                            s.offset(filelen as isize) as *mut libc::c_void,
                            simple_backup_suffix as *const libc::c_void,
                            simple_backup_suffix_size,
                        );
                    }
                    current_block_18 = 8585585342168113498;
                }
                1 => {
                    current_block_18 = 8585585342168113498;
                }
                3 => {
                    if !dirp.is_null() {
                        closedir(dirp);
                    }
                    rpl_free(s as *mut libc::c_void);
                    *__errno_location() = 12 as i32;
                    return 0 as *mut i8;
                }
                0 | _ => {
                    current_block_18 = 15089075282327824602;
                }
            }
            match current_block_18 {
                8585585342168113498 => {
                    check_extension(s, filelen, '~' as i32 as i8, sdir, &mut base_max);
                }
                _ => {}
            }
        }
        if !rename {
            break;
        }
        if sdir < 0 as i32 {
            sdir = -(100 as i32);
            base_offset = 0 as i32 as idx_t;
        }
        let mut flags: u32 = (if backup_type as u32
            == backup_type::simple_backups as i32 as u32
        {
            0 as i32
        } else {
            (1 as i32) << 0 as i32
        }) as u32;
        if renameatu(-(100 as i32), file, sdir, s.offset(base_offset as isize), flags)
            == 0 as i32
        {
            break;
        }
        let mut e: i32 = *__errno_location();
        if e != 17 as i32 {
            if !dirp.is_null() {
                closedir(dirp);
            }
            rpl_free(s as *mut libc::c_void);
            *__errno_location() = e;
            return 0 as *mut i8;
        }
    }
    if !dirp.is_null() {
        closedir(dirp);
    }
    return s;
}