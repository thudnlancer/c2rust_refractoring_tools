use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn quote_n(n: i32, arg: *const i8) -> *const i8;
    fn quote(arg: *const i8) -> *const i8;
    fn exit(_: i32) -> !;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
    fn quotearg_n_style(n: i32, s: quoting_style, arg: *const i8) -> *mut i8;
    static mut exit_failure: i32;
}
pub type ptrdiff_t = i64;
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
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
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
unsafe extern "C" fn __argmatch_die() {
    exit(exit_failure);
}
#[no_mangle]
pub static mut argmatch_die: argmatch_exit_fn = unsafe {
    Some(__argmatch_die as unsafe extern "C" fn() -> ())
};
#[no_mangle]
pub unsafe extern "C" fn argmatch(
    mut arg: *const i8,
    mut arglist: *const *const i8,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> ptrdiff_t {
    let mut i: size_t = 0;
    let mut arglen: size_t = 0;
    let mut matchind: ptrdiff_t = -(1 as i32) as ptrdiff_t;
    let mut ambiguous: bool = 0 as i32 != 0;
    arglen = strlen(arg);
    i = 0 as i32 as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if strncmp(*arglist.offset(i as isize), arg, arglen) == 0 {
            if strlen(*arglist.offset(i as isize)) == arglen {
                return i as ptrdiff_t
            } else if matchind == -(1 as i32) as i64 {
                matchind = i as ptrdiff_t;
            } else if vallist == 0 as *mut libc::c_void
                || memcmp(
                    (vallist as *const i8)
                        .offset(valsize.wrapping_mul(matchind as u64) as isize)
                        as *const libc::c_void,
                    (vallist as *const i8).offset(valsize.wrapping_mul(i) as isize)
                        as *const libc::c_void,
                    valsize,
                ) != 0
            {
                ambiguous = 1 as i32 != 0;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if ambiguous { return -(2 as i32) as ptrdiff_t } else { return matchind };
}
#[no_mangle]
pub unsafe extern "C" fn argmatch_exact(
    mut arg: *const i8,
    mut arglist: *const *const i8,
) -> ptrdiff_t {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if strcmp(*arglist.offset(i as isize), arg) == 0 {
            return i as ptrdiff_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as i32) as ptrdiff_t;
}
#[no_mangle]
pub unsafe extern "C" fn argmatch_invalid(
    mut context: *const i8,
    mut value: *const i8,
    mut problem: ptrdiff_t,
) {
    let mut format: *const i8 = if problem == -(1 as i32) as i64 {
        dcgettext(
            0 as *const i8,
            b"invalid argument %s for %s\0" as *const u8 as *const i8,
            5 as i32,
        )
    } else {
        dcgettext(
            0 as *const i8,
            b"ambiguous argument %s for %s\0" as *const u8 as *const i8,
            5 as i32,
        )
    };
    error(
        0 as i32,
        0 as i32,
        format,
        quotearg_n_style(0 as i32, quoting_style::locale_quoting_style, value),
        quote_n(1 as i32, context),
    );
}
#[no_mangle]
pub unsafe extern "C" fn argmatch_valid(
    mut arglist: *const *const i8,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) {
    let mut i: size_t = 0;
    let mut last_val: *const i8 = 0 as *const i8;
    fputs(
        dcgettext(
            0 as *const i8,
            b"Valid arguments are:\0" as *const u8 as *const i8,
            5 as i32,
        ),
        stderr,
    );
    i = 0 as i32 as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if i == 0 as i32 as u64
            || memcmp(
                last_val as *const libc::c_void,
                (vallist as *const i8).offset(valsize.wrapping_mul(i) as isize)
                    as *const libc::c_void,
                valsize,
            ) != 0
        {
            fprintf(
                stderr,
                b"\n  - %s\0" as *const u8 as *const i8,
                quote(*arglist.offset(i as isize)),
            );
            last_val = (vallist as *const i8).offset(valsize.wrapping_mul(i) as isize);
        } else {
            fprintf(
                stderr,
                b", %s\0" as *const u8 as *const i8,
                quote(*arglist.offset(i as isize)),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    _IO_putc('\n' as i32, stderr);
}
#[no_mangle]
pub unsafe extern "C" fn __xargmatch_internal(
    mut context: *const i8,
    mut arg: *const i8,
    mut arglist: *const *const i8,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
    mut exit_fn: argmatch_exit_fn,
    mut allow_abbreviation: bool,
) -> ptrdiff_t {
    let mut res: ptrdiff_t = 0;
    if allow_abbreviation {
        res = argmatch(arg, arglist, vallist, valsize);
    } else {
        res = argmatch_exact(arg, arglist);
    }
    if res >= 0 as i32 as i64 {
        return res;
    }
    argmatch_invalid(context, arg, res);
    argmatch_valid(arglist, vallist, valsize);
    (Some(exit_fn.expect("non-null function pointer")))
        .expect("non-null function pointer")();
    return -(1 as i32) as ptrdiff_t;
}
#[no_mangle]
pub unsafe extern "C" fn argmatch_to_argument(
    mut value: *const libc::c_void,
    mut arglist: *const *const i8,
    mut vallist: *const libc::c_void,
    mut valsize: size_t,
) -> *const i8 {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while !(*arglist.offset(i as isize)).is_null() {
        if memcmp(
            value,
            (vallist as *const i8).offset(valsize.wrapping_mul(i) as isize)
                as *const libc::c_void,
            valsize,
        ) == 0
        {
            return *arglist.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const i8;
}