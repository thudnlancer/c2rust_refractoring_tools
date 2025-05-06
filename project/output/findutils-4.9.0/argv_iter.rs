#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    fn getdelim(
        __lineptr: *mut *mut i8,
        __n: *mut size_t,
        __delimiter: i32,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argv_iterator {
    pub fp: *mut FILE,
    pub item_idx: size_t,
    pub tok: *mut i8,
    pub buf_len: size_t,
    pub arg_list: *mut *mut i8,
    pub p: *mut *mut i8,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum argv_iter_err {
    AI_ERR_READ = 4,
    AI_ERR_MEM = 3,
    AI_ERR_EOF = 2,
    AI_ERR_OK = 1,
}
impl argv_iter_err {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            argv_iter_err::AI_ERR_READ => 4,
            argv_iter_err::AI_ERR_MEM => 3,
            argv_iter_err::AI_ERR_EOF => 2,
            argv_iter_err::AI_ERR_OK => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> argv_iter_err {
        match value {
            4 => argv_iter_err::AI_ERR_READ,
            3 => argv_iter_err::AI_ERR_MEM,
            2 => argv_iter_err::AI_ERR_EOF,
            1 => argv_iter_err::AI_ERR_OK,
            _ => panic!("Invalid value for argv_iter_err: {}", value),
        }
    }
}
impl AddAssign<u32> for argv_iter_err {
    fn add_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for argv_iter_err {
    fn sub_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for argv_iter_err {
    fn mul_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for argv_iter_err {
    fn div_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for argv_iter_err {
    fn rem_assign(&mut self, rhs: u32) {
        *self = argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn add(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn sub(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn mul(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn div(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for argv_iter_err {
    type Output = argv_iter_err;
    fn rem(self, rhs: u32) -> argv_iter_err {
        argv_iter_err::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[no_mangle]
pub unsafe extern "C" fn argv_iter_init_argv(
    mut argv: *mut *mut i8,
) -> *mut argv_iterator {
    let mut ai: *mut argv_iterator = malloc(
        ::core::mem::size_of::<argv_iterator>() as u64,
    ) as *mut argv_iterator;
    if ai.is_null() {
        return 0 as *mut argv_iterator;
    }
    (*ai).fp = 0 as *mut FILE;
    (*ai).arg_list = argv;
    (*ai).p = argv;
    return ai;
}
#[no_mangle]
pub unsafe extern "C" fn argv_iter_init_stream(mut fp: *mut FILE) -> *mut argv_iterator {
    let mut ai: *mut argv_iterator = malloc(
        ::core::mem::size_of::<argv_iterator>() as u64,
    ) as *mut argv_iterator;
    if ai.is_null() {
        return 0 as *mut argv_iterator;
    }
    (*ai).fp = fp;
    (*ai).tok = 0 as *mut i8;
    (*ai).buf_len = 0 as i32 as size_t;
    (*ai).item_idx = 0 as i32 as size_t;
    (*ai).arg_list = 0 as *mut *mut i8;
    return ai;
}
#[no_mangle]
pub unsafe extern "C" fn argv_iter(
    mut ai: *mut argv_iterator,
    mut err: *mut argv_iter_err,
) -> *mut i8 {
    if !((*ai).fp).is_null() {
        let mut len: ssize_t = getdelim(
            &mut (*ai).tok,
            &mut (*ai).buf_len,
            '\0' as i32,
            (*ai).fp,
        );
        if len < 0 as i32 as i64 {
            *err = argv_iter_err::from_libc_c_uint(
                (if feof((*ai).fp) != 0 {
                    argv_iter_err::AI_ERR_EOF as i32
                } else {
                    argv_iter_err::AI_ERR_READ as i32
                }) as u32,
            );
            return 0 as *mut i8;
        }
        *err = argv_iter_err::AI_ERR_OK;
        (*ai).item_idx = ((*ai).item_idx).wrapping_add(1);
        (*ai).item_idx;
        return (*ai).tok;
    } else if (*(*ai).p).is_null() {
        *err = argv_iter_err::AI_ERR_EOF;
        return 0 as *mut i8;
    } else {
        *err = argv_iter_err::AI_ERR_OK;
        let fresh0 = (*ai).p;
        (*ai).p = ((*ai).p).offset(1);
        return *fresh0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn argv_iter_n_args(mut ai: *const argv_iterator) -> size_t {
    return if !((*ai).fp).is_null() {
        (*ai).item_idx
    } else {
        ((*ai).p).offset_from((*ai).arg_list) as i64 as u64
    };
}
#[no_mangle]
pub unsafe extern "C" fn argv_iter_free(mut ai: *mut argv_iterator) {
    if !((*ai).fp).is_null() {
        rpl_free((*ai).tok as *mut libc::c_void);
    }
    rpl_free(ai as *mut libc::c_void);
}