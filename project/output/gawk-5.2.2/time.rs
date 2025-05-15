use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn exit(_: i32) -> !;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn mktime(__tp: *mut tm) -> time_t;
    fn strptime(__s: *const i8, __fmt: *const i8, __tp: *mut tm) -> *mut i8;
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __suseconds_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub enum awk_bool {
    awk_false = 0,
    awk_true,
}
impl awk_bool {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_bool::awk_false => 0,
            awk_bool::awk_true => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_bool {
        match value {
            0 => awk_bool::awk_false,
            1 => awk_bool::awk_true,
            _ => panic!("Invalid value for awk_bool: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_bool {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_bool {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_bool {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_bool {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_bool {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_bool {
    type Output = awk_bool;
    fn add(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_bool {
    type Output = awk_bool;
    fn sub(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_bool {
    type Output = awk_bool;
    fn mul(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_bool {
    type Output = awk_bool;
    fn div(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_bool {
    type Output = awk_bool;
    fn rem(self, rhs: u32) -> awk_bool {
        awk_bool::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type awk_bool_t = awk_bool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: size_t,
    pub fields: [awk_field_info; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_field_info {
    pub skip: size_t,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input {
    pub name: *const i8,
    pub fd: i32,
    pub opaque: *mut libc::c_void,
    pub get_record: Option<
        unsafe extern "C" fn(
            *mut *mut i8,
            *mut awk_input,
            *mut i32,
            *mut *mut i8,
            *mut size_t,
            *mut *const awk_fieldwidth_info_t,
        ) -> i32,
    >,
    pub read_func: Option<
        unsafe extern "C" fn(i32, *mut libc::c_void, size_t) -> ssize_t,
    >,
    pub close_func: Option<unsafe extern "C" fn(*mut awk_input) -> ()>,
    pub sbuf: stat,
}
pub type awk_input_buf_t = awk_input;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_input_parser {
    pub name: *const i8,
    pub can_take_file: Option<
        unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option<
        unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
    >,
    pub next: *const awk_input_parser,
}
pub type awk_input_parser_t = awk_input_parser;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_buf {
    pub name: *const i8,
    pub mode: *const i8,
    pub fp: *mut FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut libc::c_void,
    pub gawk_fwrite: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            size_t,
            size_t,
            *mut FILE,
            *mut libc::c_void,
        ) -> size_t,
    >,
    pub gawk_fflush: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_ferror: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
    pub gawk_fclose: Option<unsafe extern "C" fn(*mut FILE, *mut libc::c_void) -> i32>,
}
pub type awk_output_buf_t = awk_output_buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_output_wrapper {
    pub name: *const i8,
    pub can_take_file: Option<
        unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
    >,
    pub take_control_of: Option<
        unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: *const awk_output_wrapper,
}
pub type awk_output_wrapper_t = awk_output_wrapper;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_two_way_processor {
    pub name: *const i8,
    pub can_take_two_way: Option<unsafe extern "C" fn(*const i8) -> awk_bool_t>,
    pub take_control_of: Option<
        unsafe extern "C" fn(
            *const i8,
            *mut awk_input_buf_t,
            *mut awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub next: *const awk_two_way_processor,
}
pub type awk_two_way_processor_t = awk_two_way_processor;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    GAWK_API_MINOR_VERSION = 2,
    GAWK_API_MAJOR_VERSION = 3,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::GAWK_API_MINOR_VERSION => 2,
            C2RustUnnamed::GAWK_API_MAJOR_VERSION => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            2 => C2RustUnnamed::GAWK_API_MINOR_VERSION,
            3 => C2RustUnnamed::GAWK_API_MAJOR_VERSION,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut i8,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
}
impl AWK_NUMBER_TYPE {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE => 0,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR => 1,
            AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> AWK_NUMBER_TYPE {
        match value {
            0 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE,
            1 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPFR,
            2 => AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_MPZ,
            _ => panic!("Invalid value for AWK_NUMBER_TYPE: {}", value),
        }
    }
}
impl AddAssign<u32> for AWK_NUMBER_TYPE {
    fn add_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for AWK_NUMBER_TYPE {
    fn sub_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for AWK_NUMBER_TYPE {
    fn mul_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for AWK_NUMBER_TYPE {
    fn div_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for AWK_NUMBER_TYPE {
    fn rem_assign(&mut self, rhs: u32) {
        *self = AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn add(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn sub(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn mul(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn div(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for AWK_NUMBER_TYPE {
    type Output = AWK_NUMBER_TYPE;
    fn rem(self, rhs: u32) -> AWK_NUMBER_TYPE {
        AWK_NUMBER_TYPE::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: libc::c_double,
    pub type_0: AWK_NUMBER_TYPE,
    pub ptr: *mut libc::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut libc::c_void;
pub type awk_scalar_t = *mut libc::c_void;
pub type awk_value_cookie_t = *mut libc::c_void;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum awk_valtype_t {
    AWK_UNDEFINED,
    AWK_NUMBER,
    AWK_STRING,
    AWK_REGEX,
    AWK_STRNUM,
    AWK_ARRAY,
    AWK_SCALAR,
    AWK_VALUE_COOKIE,
    AWK_BOOL,
}
impl awk_valtype_t {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            awk_valtype_t::AWK_UNDEFINED => 0,
            awk_valtype_t::AWK_NUMBER => 1,
            awk_valtype_t::AWK_STRING => 2,
            awk_valtype_t::AWK_REGEX => 3,
            awk_valtype_t::AWK_STRNUM => 4,
            awk_valtype_t::AWK_ARRAY => 5,
            awk_valtype_t::AWK_SCALAR => 6,
            awk_valtype_t::AWK_VALUE_COOKIE => 7,
            awk_valtype_t::AWK_BOOL => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> awk_valtype_t {
        match value {
            0 => awk_valtype_t::AWK_UNDEFINED,
            1 => awk_valtype_t::AWK_NUMBER,
            2 => awk_valtype_t::AWK_STRING,
            3 => awk_valtype_t::AWK_REGEX,
            4 => awk_valtype_t::AWK_STRNUM,
            5 => awk_valtype_t::AWK_ARRAY,
            6 => awk_valtype_t::AWK_SCALAR,
            7 => awk_valtype_t::AWK_VALUE_COOKIE,
            8 => awk_valtype_t::AWK_BOOL,
            _ => panic!("Invalid value for awk_valtype_t: {}", value),
        }
    }
}
impl AddAssign<u32> for awk_valtype_t {
    fn add_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for awk_valtype_t {
    fn sub_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for awk_valtype_t {
    fn mul_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for awk_valtype_t {
    fn div_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for awk_valtype_t {
    fn rem_assign(&mut self, rhs: u32) {
        *self = awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn add(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn sub(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn mul(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn div(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for awk_valtype_t {
    type Output = awk_valtype_t;
    fn rem(self, rhs: u32) -> awk_valtype_t {
        awk_valtype_t::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_element {
    pub next: *mut awk_element,
    pub flags: C2RustUnnamed_1,
    pub index: awk_value_t,
    pub value: awk_value_t,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    AWK_ELEMENT_DELETE = 1,
    AWK_ELEMENT_DEFAULT = 0,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::AWK_ELEMENT_DELETE => 1,
            C2RustUnnamed_1::AWK_ELEMENT_DEFAULT => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            1 => C2RustUnnamed_1::AWK_ELEMENT_DELETE,
            0 => C2RustUnnamed_1::AWK_ELEMENT_DEFAULT,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type awk_element_t = awk_element;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_flat_array {
    pub opaque1: *const libc::c_void,
    pub opaque2: *const libc::c_void,
    pub count: size_t,
    pub elements: [awk_element_t; 1],
}
pub type awk_flat_array_t = awk_flat_array;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const i8,
    pub function: Option<
        unsafe extern "C" fn(
            i32,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut libc::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
pub type awk_ext_id_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gawk_api {
    pub major_version: i32,
    pub minor_version: i32,
    pub gmp_major_version: i32,
    pub gmp_minor_version: i32,
    pub mpfr_major_version: i32,
    pub mpfr_minor_version: i32,
    pub do_flags: [i32; 6],
    pub api_add_ext_func: Option<
        unsafe extern "C" fn(awk_ext_id_t, *const i8, *mut awk_ext_func_t) -> awk_bool_t,
    >,
    pub api_register_input_parser: Option<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t) -> (),
    >,
    pub api_register_output_wrapper: Option<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_output_wrapper_t) -> (),
    >,
    pub api_register_two_way_processor: Option<
        unsafe extern "C" fn(awk_ext_id_t, *mut awk_two_way_processor_t) -> (),
    >,
    pub api_awk_atexit: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            Option<unsafe extern "C" fn(*mut libc::c_void, i32) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
    pub api_register_ext_version: Option<
        unsafe extern "C" fn(awk_ext_id_t, *const i8) -> (),
    >,
    pub api_fatal: Option<unsafe extern "C" fn(awk_ext_id_t, *const i8, ...) -> ()>,
    pub api_warning: Option<unsafe extern "C" fn(awk_ext_id_t, *const i8, ...) -> ()>,
    pub api_lintwarn: Option<unsafe extern "C" fn(awk_ext_id_t, *const i8, ...) -> ()>,
    pub api_nonfatal: Option<unsafe extern "C" fn(awk_ext_id_t, *const i8, ...) -> ()>,
    pub api_update_ERRNO_int: Option<unsafe extern "C" fn(awk_ext_id_t, i32) -> ()>,
    pub api_update_ERRNO_string: Option<
        unsafe extern "C" fn(awk_ext_id_t, *const i8) -> (),
    >,
    pub api_unset_ERRNO: Option<unsafe extern "C" fn(awk_ext_id_t) -> ()>,
    pub api_get_argument: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            size_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_argument: Option<
        unsafe extern "C" fn(awk_ext_id_t, size_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_sym_lookup: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const i8,
            *const i8,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const i8,
            *const i8,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_lookup_scalar: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_scalar_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update_scalar: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_scalar_t, *mut awk_value_t) -> awk_bool_t,
    >,
    pub api_create_value: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *mut awk_value_t,
            *mut awk_value_cookie_t,
        ) -> awk_bool_t,
    >,
    pub api_release_value: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_value_cookie_t) -> awk_bool_t,
    >,
    pub api_get_element_count: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *mut size_t) -> awk_bool_t,
    >,
    pub api_get_array_element: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_array_element: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *const awk_value_t,
            *const awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_del_array_element: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t, *const awk_value_t) -> awk_bool_t,
    >,
    pub api_create_array: Option<unsafe extern "C" fn(awk_ext_id_t) -> awk_array_t>,
    pub api_clear_array: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
    pub api_flatten_array_typed: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut *mut awk_flat_array_t,
            awk_valtype_t,
            awk_valtype_t,
        ) -> awk_bool_t,
    >,
    pub api_release_flattened_array: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            awk_array_t,
            *mut awk_flat_array_t,
        ) -> awk_bool_t,
    >,
    pub api_malloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub api_calloc: Option<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
    pub api_realloc: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub api_free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub api_get_mpfr: Option<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_mpz: Option<unsafe extern "C" fn(awk_ext_id_t) -> *mut libc::c_void>,
    pub api_get_file: Option<
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const i8,
            size_t,
            *const i8,
            i32,
            *mut *const awk_input_buf_t,
            *mut *const awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub api_destroy_array: Option<
        unsafe extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t,
    >,
}
pub type gawk_api_t = gawk_api;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[inline]
unsafe extern "C" fn make_number(
    mut num: libc::c_double,
    mut result: *mut awk_value_t,
) -> *mut awk_value_t {
    (*result).val_type = awk_valtype_t::AWK_NUMBER;
    (*result).u.n.d = num;
    (*result).u.n.type_0 = AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE;
    return result;
}
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ext_version: *const i8 = b"time extension: version 1.2\0" as *const u8
    as *const i8;
static mut init_func: Option<unsafe extern "C" fn() -> awk_bool_t> = None;
#[no_mangle]
pub static mut plugin_is_GPL_compatible: i32 = 0;
unsafe extern "C" fn do_gettimeofday(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut curtime: libc::c_double = 0.;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut timezone);
    curtime = tv.tv_sec as libc::c_double + tv.tv_usec as libc::c_double / 1000000.0f64;
    return make_number(curtime, result);
}
unsafe extern "C" fn do_sleep(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut num: awk_value_t = awk_value_t {
        val_type: awk_valtype_t::AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut secs: libc::c_double = 0.;
    let mut rc: i32 = 0;
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, awk_valtype_t::AWK_NUMBER, &mut num) as u64 == 0
    {
        ((*api).api_update_ERRNO_string)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"sleep: missing required numeric argument\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    secs = num.u.n.d;
    if secs < 0 as i32 as libc::c_double {
        ((*api).api_update_ERRNO_string)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"sleep: argument is negative\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    let mut req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    req.tv_sec = secs as __time_t;
    req.tv_nsec = ((secs - req.tv_sec as libc::c_double) * 1000000000.0f64)
        as __syscall_slong_t;
    rc = nanosleep(&mut req, 0 as *mut timespec);
    if rc < 0 as i32 {
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, *__errno_location());
    }
    return make_number(rc as libc::c_double, result);
}
unsafe extern "C" fn do_strptime(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut broken_time: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    let mut epoch_time: time_t = 0;
    let mut current_block: u64;
    let mut string: awk_value_t = awk_value_t {
        val_type: awk_valtype_t::AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut format: awk_value_t = awk_value_t {
        val_type: awk_valtype_t::AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    make_number(0.0f64, result);
    if (*api).do_flags[0 as i32 as usize] != 0 {
        if nargs == 0 as i32 {
            ((*api).api_lintwarn)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                dcgettext(
                    0 as *const i8,
                    b"strptime: called with no arguments\0" as *const u8 as *const i8,
                    5 as i32,
                ),
            );
            make_number(-1.0f64, result);
            current_block = 15863381370523918674;
        } else {
            current_block = 15240798224410183470;
        }
    } else {
        current_block = 15240798224410183470;
    }
    match current_block {
        15240798224410183470 => {
            if ((*api).api_get_argument)
                .expect(
                    "non-null function pointer",
                )(ext_id, 0 as i32 as size_t, awk_valtype_t::AWK_STRING, &mut string)
                as u64 == 0
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"do_strptime: argument 1 is not a string\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                *__errno_location() = 22 as i32;
                current_block = 15989625506452605830;
            } else if ((*api).api_get_argument)
                .expect(
                    "non-null function pointer",
                )(ext_id, 1 as i32 as size_t, awk_valtype_t::AWK_STRING, &mut format)
                as u64 == 0
            {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"do_strptime: argument 2 is not a string\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                );
                *__errno_location() = 22 as i32;
                current_block = 15989625506452605830;
            } else {
                broken_time = tm {
                    tm_sec: 0,
                    tm_min: 0,
                    tm_hour: 0,
                    tm_mday: 0,
                    tm_mon: 0,
                    tm_year: 0,
                    tm_wday: 0,
                    tm_yday: 0,
                    tm_isdst: 0,
                    tm_gmtoff: 0,
                    tm_zone: 0 as *const i8,
                };
                memset(
                    &mut broken_time as *mut tm as *mut libc::c_void,
                    0 as i32,
                    ::core::mem::size_of::<tm>() as u64,
                );
                broken_time.tm_isdst = -(1 as i32);
                if (strptime(string.u.s.str_0, format.u.s.str_0, &mut broken_time))
                    .is_null()
                {
                    make_number(-1.0f64, result);
                } else {
                    epoch_time = mktime(&mut broken_time);
                    make_number(epoch_time as libc::c_double, result);
                }
                current_block = 15863381370523918674;
            }
            match current_block {
                15863381370523918674 => {}
                _ => {
                    ((*api).api_update_ERRNO_int)
                        .expect(
                            "non-null function pointer",
                        )(ext_id, *__errno_location());
                }
            }
        }
        _ => {}
    }
    return result;
}
static mut func_table: [awk_ext_func_t; 3] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"gettimeofday\0" as *const u8 as *const i8,
                function: Some(
                    do_gettimeofday
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 0 as i32 as size_t,
                min_required_args: 0 as i32 as size_t,
                suppress_lint: awk_bool::awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"sleep\0" as *const u8 as *const i8,
                function: Some(
                    do_sleep
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as i32 as size_t,
                min_required_args: 1 as i32 as size_t,
                suppress_lint: awk_bool::awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"strptime\0" as *const u8 as *const i8,
                function: Some(
                    do_strptime
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 2 as i32 as size_t,
                min_required_args: 2 as i32 as size_t,
                suppress_lint: awk_bool::awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, mut id: awk_ext_id_t) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut errors: i32 = 0 as i32;
    api = api_p;
    ext_id = id as *mut *mut libc::c_void as awk_ext_id_t;
    if (*api).major_version != C2RustUnnamed::GAWK_API_MAJOR_VERSION as i32
        || (*api).minor_version < C2RustUnnamed::GAWK_API_MINOR_VERSION as i32
    {
        fprintf(
            stderr,
            b"time: version mismatch with gawk!\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr,
            b"\tmy version (API %d.%d), gawk version (API %d.%d)\n\0" as *const u8
                as *const i8,
            C2RustUnnamed::GAWK_API_MAJOR_VERSION as i32,
            C2RustUnnamed::GAWK_API_MINOR_VERSION as i32,
            (*api).major_version,
            (*api).minor_version,
        );
        exit(1 as i32);
    }
    i = 0 as i32 as size_t;
    j = (::core::mem::size_of::<[awk_ext_func_t; 3]>() as u64)
        .wrapping_div(::core::mem::size_of::<awk_ext_func_t>() as u64);
    while i < j {
        if (func_table[i as usize].name).is_null() {
            break;
        }
        if ((*api).api_add_ext_func)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"\0" as *const u8 as *const i8,
            &mut *func_table.as_mut_ptr().offset(i as isize),
        ) as u64 == 0
        {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                b"time: could not add %s\0" as *const u8 as *const i8,
                func_table[i as usize].name,
            );
            errors += 1;
            errors;
        }
        i = i.wrapping_add(1);
        i;
    }
    if init_func.is_some() {
        if init_func.expect("non-null function pointer")() as u64 == 0 {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                b"time: initialization function failed\0" as *const u8 as *const i8,
            );
            errors += 1;
            errors;
        }
    }
    if !ext_version.is_null() {
        ((*api).api_register_ext_version)
            .expect("non-null function pointer")(ext_id, ext_version);
    }
    return (errors == 0 as i32) as i32;
}