#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm)]
use c2rust_asm_casts::AsmCastTrait;
use core::arch::asm;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr(__stream: *mut FILE);
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum GetwordEndianState {
    GetwordEndianStateInitial = 0,
    GetwordEndianStateNative = 1,
    GetwordEndianStateSwab = 2,
}
impl GetwordEndianState {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            GetwordEndianState::GetwordEndianStateInitial => 0,
            GetwordEndianState::GetwordEndianStateNative => 1,
            GetwordEndianState::GetwordEndianStateSwab => 2,
        }
    }
}

pub const GetwordEndianStateSwab: GetwordEndianState = 2;
pub const GetwordEndianStateNative: GetwordEndianState = 1;
pub const GetwordEndianStateInitial: GetwordEndianState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ival: libc::c_int,
    pub data: [libc::c_uchar; 4],
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_2 {
    WORDBYTES = 4,
}
impl C2RustUnnamed_2 {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed_2::WORDBYTES => 4,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
unsafe extern "C" fn decode_value(
    mut data: *const libc::c_uchar,
    mut limit: libc::c_int,
    mut endian_state_flag: *mut GetwordEndianState,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut swapped: libc::c_int = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed { ival: 0 };
    u.ival = 0 as libc::c_int;
    memcpy(
        &mut u.data as *mut [libc::c_uchar; 4] as *mut libc::c_void,
        data as *const libc::c_void,
        WORDBYTES as libc::c_int as libc::c_ulong,
    );
    swapped = ({
        let mut __v: libc::c_uint = 0;
        let mut __x: libc::c_uint = u.ival as libc::c_uint;
        if 0 != 0 {
            __v = (__x & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
                | (__x & 0xff0000 as libc::c_int as libc::c_uint) >> 8 as libc::c_int
                | (__x & 0xff00 as libc::c_int as libc::c_uint) << 8 as libc::c_int
                | (__x & 0xff as libc::c_int as libc::c_uint) << 24 as libc::c_int;
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
    }) as libc::c_int;
    if *endian_state_flag as libc::c_uint
        == GetwordEndianStateInitial as libc::c_int as libc::c_uint
    {
        if u.ival <= limit {
            if swapped > limit {
                *endian_state_flag = GetwordEndianStateNative;
            }
            return u.ival;
        } else if swapped <= limit {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: locate database %s was built with a different byte order\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, filename),
            );
            *endian_state_flag = GetwordEndianStateSwab;
            return swapped;
        } else {
            return u.ival
        }
    } else if *endian_state_flag as libc::c_uint
        == GetwordEndianStateSwab as libc::c_int as libc::c_uint
    {
        return swapped
    } else {
        return u.ival
    };
}
#[no_mangle]
pub unsafe extern "C" fn getword(
    mut fp: *mut FILE,
    mut filename: *const libc::c_char,
    mut maxvalue: size_t,
    mut endian_state_flag: *mut GetwordEndianState,
) -> libc::c_int {
    let mut data: [libc::c_uchar; 4] = [0; 4];
    let mut bytes_read: size_t = 0;
    clearerr(fp);
    bytes_read = fread(
        data.as_mut_ptr() as *mut libc::c_void,
        WORDBYTES as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    if bytes_read != 1 as libc::c_int as libc::c_ulong {
        let mut quoted_name: *const libc::c_char = quotearg_n_style(
            0 as libc::c_int,
            locale_quoting_style,
            filename,
        );
        if feof(fp) != 0 {
            if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF in %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"unexpected EOF in %s\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        } else {
            if ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error reading a word from %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            } else {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"error reading a word from %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    quoted_name,
                );
                if 0 as libc::c_int != 0 {} else {
                    unreachable!();
                };
            };
        }
        abort();
    } else {
        return decode_value(
            data.as_mut_ptr() as *const libc::c_uchar,
            maxvalue as libc::c_int,
            endian_state_flag,
            filename,
        )
    };
}
