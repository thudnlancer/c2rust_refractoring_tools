use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn __errno_location() -> *mut i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> i32;
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
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
pub enum quoting_style {
    literal_quoting_style,
    shell_quoting_style,
    shell_always_quoting_style,
    shell_escape_quoting_style,
    shell_escape_always_quoting_style,
    c_quoting_style,
    c_maybe_quoting_style,
    escape_quoting_style,
    locale_quoting_style,
    clocale_quoting_style,
    custom_quoting_style,
}
impl quoting_style {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            quoting_style::literal_quoting_style => 0,
            quoting_style::shell_quoting_style => 1,
            quoting_style::shell_always_quoting_style => 2,
            quoting_style::shell_escape_quoting_style => 3,
            quoting_style::shell_escape_always_quoting_style => 4,
            quoting_style::c_quoting_style => 5,
            quoting_style::c_maybe_quoting_style => 6,
            quoting_style::escape_quoting_style => 7,
            quoting_style::locale_quoting_style => 8,
            quoting_style::clocale_quoting_style => 9,
            quoting_style::custom_quoting_style => 10,
        }
    }
    fn from_libc_c_uint(value: u32) -> quoting_style {
        match value {
            0 => quoting_style::literal_quoting_style,
            1 => quoting_style::shell_quoting_style,
            2 => quoting_style::shell_always_quoting_style,
            3 => quoting_style::shell_escape_quoting_style,
            4 => quoting_style::shell_escape_always_quoting_style,
            5 => quoting_style::c_quoting_style,
            6 => quoting_style::c_maybe_quoting_style,
            7 => quoting_style::escape_quoting_style,
            8 => quoting_style::locale_quoting_style,
            9 => quoting_style::clocale_quoting_style,
            10 => quoting_style::custom_quoting_style,
            _ => panic!("Invalid value for quoting_style: {}", value),
        }
    }
}
impl AddAssign<u32> for quoting_style {
    fn add_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for quoting_style {
    fn sub_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for quoting_style {
    fn mul_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for quoting_style {
    fn div_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for quoting_style {
    fn rem_assign(&mut self, rhs: u32) {
        *self = quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for quoting_style {
    type Output = quoting_style;
    fn add(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for quoting_style {
    type Output = quoting_style;
    fn sub(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for quoting_style {
    type Output = quoting_style;
    fn mul(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for quoting_style {
    type Output = quoting_style;
    fn div(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for quoting_style {
    type Output = quoting_style;
    fn rem(self, rhs: u32) -> quoting_style {
        quoting_style::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum GetwordEndianState {
    GetwordEndianStateInitial = 0,
    GetwordEndianStateNative = 1,
    GetwordEndianStateSwab = 2,
}
impl GetwordEndianState {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            GetwordEndianState::GetwordEndianStateInitial => 0,
            GetwordEndianState::GetwordEndianStateNative => 1,
            GetwordEndianState::GetwordEndianStateSwab => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> GetwordEndianState {
        match value {
            0 => GetwordEndianState::GetwordEndianStateInitial,
            1 => GetwordEndianState::GetwordEndianStateNative,
            2 => GetwordEndianState::GetwordEndianStateSwab,
            _ => panic!("Invalid value for GetwordEndianState: {}", value),
        }
    }
}
impl AddAssign<u32> for GetwordEndianState {
    fn add_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for GetwordEndianState {
    fn sub_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for GetwordEndianState {
    fn mul_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for GetwordEndianState {
    fn div_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for GetwordEndianState {
    fn rem_assign(&mut self, rhs: u32) {
        *self = GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn add(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn sub(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn mul(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn div(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for GetwordEndianState {
    type Output = GetwordEndianState;
    fn rem(self, rhs: u32) -> GetwordEndianState {
        GetwordEndianState::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ival: i32,
    pub data: [u8; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    WORDBYTES = 4,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_2::WORDBYTES => 4,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_2 {
        match value {
            4 => C2RustUnnamed_2::WORDBYTES,
            _ => panic!("Invalid value for C2RustUnnamed_2: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_2 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_2 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_2 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_2 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_2 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn add(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn sub(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn mul(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn div(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_2 {
    type Output = C2RustUnnamed_2;
    fn rem(self, rhs: u32) -> C2RustUnnamed_2 {
        C2RustUnnamed_2::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: i32,
}
unsafe extern "C" fn decode_value(
    mut data: *const u8,
    mut limit: i32,
    mut endian_state_flag: *mut GetwordEndianState,
    mut filename: *const i8,
) -> i32 {
    let mut swapped: i32 = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed { ival: 0 };
    u.ival = 0 as i32;
    memcpy(
        &mut u.data as *mut [u8; 4] as *mut libc::c_void,
        data as *const libc::c_void,
        C2RustUnnamed_2::WORDBYTES as i32 as u64,
    );
    swapped = ({
        let mut __v: u32 = 0;
        let mut __x: u32 = u.ival as u32;
        if 0 != 0 {
            __v = (__x & 0xff000000 as u32) >> 24 as i32
                | (__x & 0xff0000 as i32 as u32) >> 8 as i32
                | (__x & 0xff00 as i32 as u32) << 8 as i32
                | (__x & 0xff as i32 as u32) << 24 as i32;
        } else {
            let fresh0 = &mut __v;
            let fresh1;
            let fresh2 = __x;
            asm!(
                "bswap {0}", inlateout(reg) c2rust_asm_casts::AsmCast::cast_in(fresh0,
                fresh2) => fresh1, options(preserves_flags, pure, readonly, att_syntax)
            );
            c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
        }
        __v
    }) as i32;
    if *endian_state_flag as u32
        == GetwordEndianState::GetwordEndianStateInitial as i32 as u32
    {
        if u.ival <= limit {
            if swapped > limit {
                *endian_state_flag = GetwordEndianState::GetwordEndianStateNative;
            }
            return u.ival;
        } else if swapped <= limit {
            error(
                0 as i32,
                0 as i32,
                dcgettext(
                    0 as *const i8,
                    b"WARNING: locate database %s was built with a different byte order\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, filename),
            );
            *endian_state_flag = GetwordEndianState::GetwordEndianStateSwab;
            return swapped;
        } else {
            return u.ival
        }
    } else if *endian_state_flag as u32
        == GetwordEndianState::GetwordEndianStateSwab as i32 as u32
    {
        return swapped
    } else {
        return u.ival
    };
}
#[no_mangle]
pub unsafe extern "C" fn getword(
    mut fp: *mut FILE,
    mut filename: *const i8,
    mut maxvalue: size_t,
    mut endian_state_flag: *mut GetwordEndianState,
) -> i32 {
    let mut data: [u8; 4] = [0; 4];
    let mut bytes_read: size_t = 0;
    clearerr(fp);
    bytes_read = fread(
        data.as_mut_ptr() as *mut libc::c_void,
        C2RustUnnamed_2::WORDBYTES as i32 as size_t,
        1 as i32 as size_t,
        fp,
    );
    if bytes_read != 1 as i32 as u64 {
        let mut quoted_name: *const i8 = quotearg_n_style(
            0 as i32,
            quoting_style::locale_quoting_style,
            filename,
        );
        if feof(fp) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as u64 != 0 {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unexpected EOF in %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quoted_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    0 as i32,
                    dcgettext(
                        0 as *const i8,
                        b"unexpected EOF in %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quoted_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as u64 != 0 {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"error reading a word from %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quoted_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as i32,
                    *__errno_location(),
                    dcgettext(
                        0 as *const i8,
                        b"error reading a word from %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    quoted_name,
                );
                if 0 as i32 != 0 {} else {
                    unreachable!();
                };
            };
        }
        abort();
    } else {
        return decode_value(
            data.as_mut_ptr() as *const u8,
            maxvalue as i32,
            endian_state_flag,
            filename,
        )
    };
}