#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    static mut stdout: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fputs_unlocked(__s: *const i8, __stream: *mut FILE) -> i32;
    static version_etc_copyright: [i8; 0];
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub enum C2RustUnnamed {
    COPYRIGHT_YEAR = 2022,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::COPYRIGHT_YEAR => 2022,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            2022 => C2RustUnnamed::COPYRIGHT_YEAR,
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
pub unsafe extern "C" fn version_etc_arn(
    mut stream: *mut FILE,
    mut command_name: *const i8,
    mut package: *const i8,
    mut version: *const i8,
    mut authors: *const *const i8,
    mut n_authors: size_t,
) {
    if !command_name.is_null() {
        fprintf(
            stream,
            b"%s (%s) %s\n\0" as *const u8 as *const i8,
            command_name,
            package,
            version,
        );
    } else {
        fprintf(stream, b"%s %s\n\0" as *const u8 as *const i8, package, version);
    }
    fprintf(
        stream,
        version_etc_copyright.as_ptr(),
        dcgettext(0 as *const i8, b"(C)\0" as *const u8 as *const i8, 5 as i32),
        C2RustUnnamed::COPYRIGHT_YEAR as i32,
    );
    fputs_unlocked(b"\n\0" as *const u8 as *const i8, stream);
    fprintf(
        stream,
        dcgettext(
            0 as *const i8,
            b"License GPLv3+: GNU GPL version 3 or later <%s>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        b"https://gnu.org/licenses/gpl.html\0" as *const u8 as *const i8,
    );
    fputs_unlocked(b"\n\0" as *const u8 as *const i8, stream);
    match n_authors {
        0 => {}
        1 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
            );
        }
        2 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s and %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
            );
        }
        3 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, and %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
            );
        }
        4 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\nand %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
            );
        }
        5 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, and %s.\n\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
            );
        }
        6 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, %s, and %s.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
                *authors.offset(5 as i32 as isize),
            );
        }
        7 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, %s, %s, and %s.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
                *authors.offset(5 as i32 as isize),
                *authors.offset(6 as i32 as isize),
            );
        }
        8 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, %s, %s, %s,\nand %s.\n\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
                *authors.offset(5 as i32 as isize),
                *authors.offset(6 as i32 as isize),
                *authors.offset(7 as i32 as isize),
            );
        }
        9 => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, and %s.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
                *authors.offset(5 as i32 as isize),
                *authors.offset(6 as i32 as isize),
                *authors.offset(7 as i32 as isize),
                *authors.offset(8 as i32 as isize),
            );
        }
        _ => {
            fprintf(
                stream,
                dcgettext(
                    0 as *const i8,
                    b"Written by %s, %s, %s,\n%s, %s, %s, %s,\n%s, %s, and others.\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                *authors.offset(0 as i32 as isize),
                *authors.offset(1 as i32 as isize),
                *authors.offset(2 as i32 as isize),
                *authors.offset(3 as i32 as isize),
                *authors.offset(4 as i32 as isize),
                *authors.offset(5 as i32 as isize),
                *authors.offset(6 as i32 as isize),
                *authors.offset(7 as i32 as isize),
                *authors.offset(8 as i32 as isize),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn version_etc_ar(
    mut stream: *mut FILE,
    mut command_name: *const i8,
    mut package: *const i8,
    mut version: *const i8,
    mut authors: *const *const i8,
) {
    let mut n_authors: size_t = 0;
    n_authors = 0 as i32 as size_t;
    while !(*authors.offset(n_authors as isize)).is_null() {
        n_authors = n_authors.wrapping_add(1);
        n_authors;
    }
    version_etc_arn(stream, command_name, package, version, authors, n_authors);
}
#[no_mangle]
pub unsafe extern "C" fn version_etc_va(
    mut stream: *mut FILE,
    mut command_name: *const i8,
    mut package: *const i8,
    mut version: *const i8,
    mut authors: ::core::ffi::VaList,
) {
    let mut n_authors: size_t = 0;
    let mut authtab: [*const i8; 10] = [0 as *const i8; 10];
    n_authors = 0 as i32 as size_t;
    while n_authors < 10 as i32 as u64
        && {
            authtab[n_authors as usize] = authors.arg::<*const i8>();
            !(authtab[n_authors as usize]).is_null()
        }
    {
        n_authors = n_authors.wrapping_add(1);
        n_authors;
    }
    version_etc_arn(
        stream,
        command_name,
        package,
        version,
        authtab.as_mut_ptr(),
        n_authors,
    );
}
#[no_mangle]
pub unsafe extern "C" fn version_etc(
    mut stream: *mut FILE,
    mut command_name: *const i8,
    mut package: *const i8,
    mut version: *const i8,
    mut args: ...
) {
    let mut authors: ::core::ffi::VaListImpl;
    authors = args.clone();
    version_etc_va(stream, command_name, package, version, authors.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn emit_bug_reporting_address() {
    fputs_unlocked(b"\n\0" as *const u8 as *const i8, stdout);
    printf(
        dcgettext(
            0 as *const i8,
            b"Report bugs to: %s\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        b"bug-sed@gnu.org\0" as *const u8 as *const i8,
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"%s home page: <%s>\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        b"GNU sed\0" as *const u8 as *const i8,
        b"https://www.gnu.org/software/sed/\0" as *const u8 as *const i8,
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"General help using GNU software: <%s>\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
        b"https://www.gnu.org/gethelp/\0" as *const u8 as *const i8,
    );
}