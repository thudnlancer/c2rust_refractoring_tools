use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    pub type afm_handle_st;
    pub type stringhash_st;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn getuid() -> __uid_t;
    static mut optarg: *mut i8;
    static mut optind: i32;
    fn strerror(_: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn system(__command: *const i8) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn exit(_: i32) -> !;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn strtod(__nptr: *const i8, __endptr: *mut *mut i8) -> libc::c_double;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn printer_close(context: *mut libc::c_void);
    fn printer_open(
        cmd: *mut i8,
        options: *mut i8,
        queue_param_0: *mut i8,
        printer_name: *mut i8,
        context_return: *mut *mut libc::c_void,
    ) -> *mut FILE;
    fn parse_key_value_pair(set: StringHashPtr, kv: *mut i8);
    fn escape_string(string: *mut i8) -> *mut i8;
    fn read_font_info();
    fn parse_font_spec(
        spec: *mut i8,
        name_return: *mut *mut i8,
        size_return: *mut FontPoint,
    ) -> i32;
    fn parse_float(string: *mut i8, units: i32, horizontal: i32) -> libc::c_double;
    fn file_existsp(name: *mut i8, suffix: *mut i8) -> i32;
    fn do_list_missing_characters(array: *mut i32);
    fn process_file(fname: *mut i8, fp: *mut InputStream);
    fn is_close(is: *mut InputStream);
    fn is_open(
        is: *mut InputStream,
        fp: *mut FILE,
        fname: *mut i8,
        input_filter_0: *mut i8,
    ) -> i32;
    fn dump_ps_trailer();
    fn read_config(path: *mut i8, name: *mut i8) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fclose(__stream: *mut FILE) -> i32;
    fn tmpnam(__s: *mut i8) -> *mut i8;
    fn remove(__filename: *const i8) -> i32;
    static mut stderr: *mut _IO_FILE;
    static mut stdout: *mut _IO_FILE;
    static mut stdin: *mut _IO_FILE;
    fn __errno_location() -> *mut i32;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn asctime(__tp: *const tm) -> *mut i8;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn textdomain(__domainname: *const i8) -> *mut i8;
    fn bindtextdomain(__domainname: *const i8, __dirname: *const i8) -> *mut i8;
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn afm_error_to_string(error: AFMError, buf: *mut i8);
    fn afm_create(
        path: *const i8,
        verbose_level: u32,
        handle_return: *mut AFMHandle,
    ) -> AFMError;
    fn strhash_init() -> StringHashPtr;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut i8,
        keylen: i32,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut i8,
        keylen_return: *mut i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn strhash_get_next(
        hash: StringHashPtr,
        key_return: *mut *mut i8,
        keylen_return: *mut i32,
        data_return: *mut *mut libc::c_void,
    ) -> i32;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut i8) -> *mut i8;
    fn fseek(__stream: *mut FILE, __off: i64, __whence: i32) -> i32;
    fn getopt_long(
        argc: i32,
        argv: *const *mut i8,
        shortopts: *const i8,
        longopts: *const option,
        longind: *mut i32,
    ) -> i32;
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
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
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
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut i8,
    pub pw_passwd: *mut i8,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut i8,
    pub pw_dir: *mut i8,
    pub pw_shell: *mut i8,
}
pub type AFMError = u32;
pub type AFMHandle = *mut afm_handle_st;
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct media_entry_st {
    pub next: *mut media_entry_st,
    pub name: *mut i8,
    pub w: i32,
    pub h: i32,
    pub llx: i32,
    pub lly: i32,
    pub urx: i32,
    pub ury: i32,
}
pub type MediaEntry = media_entry_st;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum HeaderType {
    HDR_NONE,
    HDR_SIMPLE,
    HDR_FANCY,
}
impl HeaderType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            HeaderType::HDR_NONE => 0,
            HeaderType::HDR_SIMPLE => 1,
            HeaderType::HDR_FANCY => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> HeaderType {
        match value {
            0 => HeaderType::HDR_NONE,
            1 => HeaderType::HDR_SIMPLE,
            2 => HeaderType::HDR_FANCY,
            _ => panic!("Invalid value for HeaderType: {}", value),
        }
    }
}
impl AddAssign<u32> for HeaderType {
    fn add_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for HeaderType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for HeaderType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for HeaderType {
    fn div_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for HeaderType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = HeaderType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for HeaderType {
    type Output = HeaderType;
    fn add(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for HeaderType {
    type Output = HeaderType;
    fn sub(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for HeaderType {
    type Output = HeaderType;
    fn mul(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for HeaderType {
    type Output = HeaderType;
    fn div(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for HeaderType {
    type Output = HeaderType;
    fn rem(self, rhs: u32) -> HeaderType {
        HeaderType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum InputEncoding {
    ENC_ISO_8859_1,
    ENC_ISO_8859_2,
    ENC_ISO_8859_3,
    ENC_ISO_8859_4,
    ENC_ISO_8859_5,
    ENC_ISO_8859_7,
    ENC_ASCII,
    ENC_ASCII_FISE,
    ENC_ASCII_DKNO,
    ENC_IBMPC,
    ENC_MAC,
    ENC_VMS,
    ENC_HP8,
    ENC_KOI8,
    ENC_PS,
}
impl InputEncoding {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            InputEncoding::ENC_ISO_8859_1 => 0,
            InputEncoding::ENC_ISO_8859_2 => 1,
            InputEncoding::ENC_ISO_8859_3 => 2,
            InputEncoding::ENC_ISO_8859_4 => 3,
            InputEncoding::ENC_ISO_8859_5 => 4,
            InputEncoding::ENC_ISO_8859_7 => 5,
            InputEncoding::ENC_ASCII => 6,
            InputEncoding::ENC_ASCII_FISE => 7,
            InputEncoding::ENC_ASCII_DKNO => 8,
            InputEncoding::ENC_IBMPC => 9,
            InputEncoding::ENC_MAC => 10,
            InputEncoding::ENC_VMS => 11,
            InputEncoding::ENC_HP8 => 12,
            InputEncoding::ENC_KOI8 => 13,
            InputEncoding::ENC_PS => 14,
        }
    }
    fn from_libc_c_uint(value: u32) -> InputEncoding {
        match value {
            0 => InputEncoding::ENC_ISO_8859_1,
            1 => InputEncoding::ENC_ISO_8859_2,
            2 => InputEncoding::ENC_ISO_8859_3,
            3 => InputEncoding::ENC_ISO_8859_4,
            4 => InputEncoding::ENC_ISO_8859_5,
            5 => InputEncoding::ENC_ISO_8859_7,
            6 => InputEncoding::ENC_ASCII,
            7 => InputEncoding::ENC_ASCII_FISE,
            8 => InputEncoding::ENC_ASCII_DKNO,
            9 => InputEncoding::ENC_IBMPC,
            10 => InputEncoding::ENC_MAC,
            11 => InputEncoding::ENC_VMS,
            12 => InputEncoding::ENC_HP8,
            13 => InputEncoding::ENC_KOI8,
            14 => InputEncoding::ENC_PS,
            _ => panic!("Invalid value for InputEncoding: {}", value),
        }
    }
}
impl AddAssign<u32> for InputEncoding {
    fn add_assign(&mut self, rhs: u32) {
        *self = InputEncoding::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for InputEncoding {
    fn sub_assign(&mut self, rhs: u32) {
        *self = InputEncoding::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for InputEncoding {
    fn mul_assign(&mut self, rhs: u32) {
        *self = InputEncoding::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for InputEncoding {
    fn div_assign(&mut self, rhs: u32) {
        *self = InputEncoding::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for InputEncoding {
    fn rem_assign(&mut self, rhs: u32) {
        *self = InputEncoding::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for InputEncoding {
    type Output = InputEncoding;
    fn add(self, rhs: u32) -> InputEncoding {
        InputEncoding::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for InputEncoding {
    type Output = InputEncoding;
    fn sub(self, rhs: u32) -> InputEncoding {
        InputEncoding::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for InputEncoding {
    type Output = InputEncoding;
    fn mul(self, rhs: u32) -> InputEncoding {
        InputEncoding::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for InputEncoding {
    type Output = InputEncoding;
    fn div(self, rhs: u32) -> InputEncoding {
        InputEncoding::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for InputEncoding {
    type Output = InputEncoding;
    fn rem(self, rhs: u32) -> InputEncoding {
        InputEncoding::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum PageLabelFormat {
    LABEL_SHORT,
    LABEL_LONG,
}
impl PageLabelFormat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            PageLabelFormat::LABEL_SHORT => 0,
            PageLabelFormat::LABEL_LONG => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> PageLabelFormat {
        match value {
            0 => PageLabelFormat::LABEL_SHORT,
            1 => PageLabelFormat::LABEL_LONG,
            _ => panic!("Invalid value for PageLabelFormat: {}", value),
        }
    }
}
impl AddAssign<u32> for PageLabelFormat {
    fn add_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for PageLabelFormat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for PageLabelFormat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for PageLabelFormat {
    fn div_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for PageLabelFormat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn add(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn sub(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn mul(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn div(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for PageLabelFormat {
    type Output = PageLabelFormat;
    fn rem(self, rhs: u32) -> PageLabelFormat {
        PageLabelFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum MarkWrappedLinesStyle {
    MWLS_NONE = 0,
    MWLS_PLUS = 1,
    MWLS_BOX = 2,
    MWLS_ARROW = 3,
}
impl MarkWrappedLinesStyle {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            MarkWrappedLinesStyle::MWLS_NONE => 0,
            MarkWrappedLinesStyle::MWLS_PLUS => 1,
            MarkWrappedLinesStyle::MWLS_BOX => 2,
            MarkWrappedLinesStyle::MWLS_ARROW => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> MarkWrappedLinesStyle {
        match value {
            0 => MarkWrappedLinesStyle::MWLS_NONE,
            1 => MarkWrappedLinesStyle::MWLS_PLUS,
            2 => MarkWrappedLinesStyle::MWLS_BOX,
            3 => MarkWrappedLinesStyle::MWLS_ARROW,
            _ => panic!("Invalid value for MarkWrappedLinesStyle: {}", value),
        }
    }
}
impl AddAssign<u32> for MarkWrappedLinesStyle {
    fn add_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for MarkWrappedLinesStyle {
    fn sub_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for MarkWrappedLinesStyle {
    fn mul_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for MarkWrappedLinesStyle {
    fn div_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for MarkWrappedLinesStyle {
    fn rem_assign(&mut self, rhs: u32) {
        *self = MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn add(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn sub(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn mul(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn div(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for MarkWrappedLinesStyle {
    type Output = MarkWrappedLinesStyle;
    fn rem(self, rhs: u32) -> MarkWrappedLinesStyle {
        MarkWrappedLinesStyle::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum NonPrintableFormat {
    NPF_SPACE,
    NPF_QUESTIONMARK,
    NPF_CARET,
    NPF_OCTAL,
}
impl NonPrintableFormat {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            NonPrintableFormat::NPF_SPACE => 0,
            NonPrintableFormat::NPF_QUESTIONMARK => 1,
            NonPrintableFormat::NPF_CARET => 2,
            NonPrintableFormat::NPF_OCTAL => 3,
        }
    }
    fn from_libc_c_uint(value: u32) -> NonPrintableFormat {
        match value {
            0 => NonPrintableFormat::NPF_SPACE,
            1 => NonPrintableFormat::NPF_QUESTIONMARK,
            2 => NonPrintableFormat::NPF_CARET,
            3 => NonPrintableFormat::NPF_OCTAL,
            _ => panic!("Invalid value for NonPrintableFormat: {}", value),
        }
    }
}
impl AddAssign<u32> for NonPrintableFormat {
    fn add_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for NonPrintableFormat {
    fn sub_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for NonPrintableFormat {
    fn mul_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for NonPrintableFormat {
    fn div_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for NonPrintableFormat {
    fn rem_assign(&mut self, rhs: u32) {
        *self = NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn add(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn sub(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn mul(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn div(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for NonPrintableFormat {
    type Output = NonPrintableFormat;
    fn rem(self, rhs: u32) -> NonPrintableFormat {
        NonPrintableFormat::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum FormFeedType {
    FORMFEED_COLUMN,
    FORMFEED_PAGE,
    FORMFEED_HCOLUMN,
}
impl FormFeedType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            FormFeedType::FORMFEED_COLUMN => 0,
            FormFeedType::FORMFEED_PAGE => 1,
            FormFeedType::FORMFEED_HCOLUMN => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> FormFeedType {
        match value {
            0 => FormFeedType::FORMFEED_COLUMN,
            1 => FormFeedType::FORMFEED_PAGE,
            2 => FormFeedType::FORMFEED_HCOLUMN,
            _ => panic!("Invalid value for FormFeedType: {}", value),
        }
    }
}
impl AddAssign<u32> for FormFeedType {
    fn add_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for FormFeedType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for FormFeedType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for FormFeedType {
    fn div_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for FormFeedType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = FormFeedType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for FormFeedType {
    type Output = FormFeedType;
    fn add(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for FormFeedType {
    type Output = FormFeedType;
    fn sub(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for FormFeedType {
    type Output = FormFeedType;
    fn mul(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for FormFeedType {
    type Output = FormFeedType;
    fn div(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for FormFeedType {
    type Output = FormFeedType;
    fn rem(self, rhs: u32) -> FormFeedType {
        FormFeedType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum LineEndType {
    LE_TRUNCATE,
    LE_CHAR_WRAP,
    LE_WORD_WRAP,
}
impl LineEndType {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            LineEndType::LE_TRUNCATE => 0,
            LineEndType::LE_CHAR_WRAP => 1,
            LineEndType::LE_WORD_WRAP => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> LineEndType {
        match value {
            0 => LineEndType::LE_TRUNCATE,
            1 => LineEndType::LE_CHAR_WRAP,
            2 => LineEndType::LE_WORD_WRAP,
            _ => panic!("Invalid value for LineEndType: {}", value),
        }
    }
}
impl AddAssign<u32> for LineEndType {
    fn add_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for LineEndType {
    fn sub_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for LineEndType {
    fn mul_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for LineEndType {
    fn div_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for LineEndType {
    fn rem_assign(&mut self, rhs: u32) {
        *self = LineEndType::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for LineEndType {
    type Output = LineEndType;
    fn add(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for LineEndType {
    type Output = LineEndType;
    fn sub(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for LineEndType {
    type Output = LineEndType;
    fn mul(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for LineEndType {
    type Output = LineEndType;
    fn div(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for LineEndType {
    type Output = LineEndType;
    fn rem(self, rhs: u32) -> LineEndType {
        LineEndType::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_stream_st {
    pub is_pipe: i32,
    pub fp: *mut FILE,
    pub buf: [u8; 4096],
    pub data_in_buf: u32,
    pub bufpos: u32,
    pub nreads: u32,
    pub unget_ch: *mut u8,
    pub unget_pos: u32,
    pub unget_alloc: u32,
}
pub type InputStream = input_stream_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_range_st {
    pub next: *mut page_range_st,
    pub odd: i32,
    pub even: i32,
    pub start: u32,
    pub end: u32,
}
pub type PageRange = page_range_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct font_point_st {
    pub w: libc::c_double,
    pub h: libc::c_double,
}
pub type FontPoint = font_point_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub names: [*mut i8; 3],
    pub encoding: InputEncoding,
    pub nl: i32,
    pub bs: i32,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
    return strtol(__nptr, 0 as *mut libc::c_void as *mut *mut i8, 10 as i32) as i32;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const i8) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut i8);
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[no_mangle]
pub static mut program: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut ofp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut printer_context: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut version_string: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut ps_version_string: [i8; 20] = [0; 20];
#[no_mangle]
pub static mut date_string: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut run_tm: tm = tm {
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
#[no_mangle]
pub static mut mod_tm: tm = tm {
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
#[no_mangle]
pub static mut passwd: *mut passwd = 0 as *const passwd as *mut passwd;
#[no_mangle]
pub static mut enscript_library: *mut i8 = b"/usr/local/share/enscript\0" as *const u8
    as *const i8 as *mut i8;
#[no_mangle]
pub static mut libpath: [i8; 1024] = [0; 1024];
#[no_mangle]
pub static mut afm_path: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut afm_path_buffer: [i8; 1024] = [0; 1024];
#[no_mangle]
pub static mut media_names: *mut MediaEntry = 0 as *const MediaEntry as *mut MediaEntry;
#[no_mangle]
pub static mut media: *mut MediaEntry = 0 as *const MediaEntry as *mut MediaEntry;
#[no_mangle]
pub static mut bs: i32 = 8 as i32;
#[no_mangle]
pub static mut total_pages: i32 = 0 as i32;
#[no_mangle]
pub static mut num_truncated_lines: i32 = 0 as i32;
#[no_mangle]
pub static mut num_missing_chars: i32 = 0 as i32;
#[no_mangle]
pub static mut missing_chars: [i32; 256] = [
    0 as i32,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut num_non_printable_chars: i32 = 0 as i32;
#[no_mangle]
pub static mut non_printable_chars: [i32; 256] = [
    0 as i32,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut d_page_w: i32 = 0 as i32;
#[no_mangle]
pub static mut d_page_h: i32 = 0 as i32;
#[no_mangle]
pub static mut d_header_w: i32 = 0 as i32;
#[no_mangle]
pub static mut d_header_h: i32 = 0 as i32;
#[no_mangle]
pub static mut d_footer_h: i32 = 0 as i32;
#[no_mangle]
pub static mut d_output_w: i32 = 0 as i32;
#[no_mangle]
pub static mut d_output_h: i32 = 0 as i32;
#[no_mangle]
pub static mut d_output_x_margin: i32 = 5 as i32;
#[no_mangle]
pub static mut d_output_y_margin: i32 = 2 as i32;
#[no_mangle]
pub static mut res_fonts: StringHashPtr = 0 as *const stringhash_st
    as *mut stringhash_st;
#[no_mangle]
pub static mut download_fonts: StringHashPtr = 0 as *const stringhash_st
    as *mut stringhash_st;
#[no_mangle]
pub static mut pagedevice: StringHashPtr = 0 as *const stringhash_st
    as *mut stringhash_st;
#[no_mangle]
pub static mut statusdict: StringHashPtr = 0 as *const stringhash_st
    as *mut stringhash_st;
#[no_mangle]
pub static mut user_strings: StringHashPtr = 0 as *const stringhash_st
    as *mut stringhash_st;
#[no_mangle]
pub static mut afm_cache: StringHashPtr = 0 as *const stringhash_st as StringHashPtr;
#[no_mangle]
pub static mut afm_info_cache: StringHashPtr = 0 as *const stringhash_st
    as StringHashPtr;
#[no_mangle]
pub static mut afm: AFMHandle = 0 as *const afm_handle_st as AFMHandle;
#[no_mangle]
pub static mut num_columns: i32 = 1 as i32;
#[no_mangle]
pub static mut page_ranges: *mut PageRange = 0 as *const PageRange as *mut PageRange;
#[no_mangle]
pub static mut file_align: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut page_header: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut line_end: LineEndType = LineEndType::LE_CHAR_WRAP;
#[no_mangle]
pub static mut line_numbers: i32 = 0 as i32;
#[no_mangle]
pub static mut start_line_number: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut printer: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut printer_buf: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut special_escapes: i32 = 0 as i32;
#[no_mangle]
pub static mut escape_char: i32 = '\0' as i32;
#[no_mangle]
pub static mut default_escape_char: i32 = 0;
#[no_mangle]
pub static mut pretty_print: i32 = 0 as i32;
#[no_mangle]
pub static mut pp_start_state: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut pp_filter: [i8; 4096] = [0; 4096];
#[no_mangle]
pub static mut Fname: *mut i8 = b"Courier\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut Fpt: FontPoint = {
    let mut init = font_point_st {
        w: 10.0f64,
        h: 10.0f64,
    };
    init
};
#[no_mangle]
pub static mut default_Fpt: FontPoint = FontPoint { w: 0., h: 0. };
#[no_mangle]
pub static mut default_Fname: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut user_body_font_defined: i32 = 0 as i32;
#[no_mangle]
pub static mut font_widths: [libc::c_double; 256] = [0.; 256];
#[no_mangle]
pub static mut font_ctype: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut font_is_fixed: i32 = 0;
#[no_mangle]
pub static mut font_bbox_lly: libc::c_double = 0.;
#[no_mangle]
pub static mut HFname: *mut i8 = b"Courier-Bold\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut HFpt: FontPoint = {
    let mut init = font_point_st {
        w: 10.0f64,
        h: 10.0f64,
    };
    init
};
#[no_mangle]
pub static mut header: HeaderType = HeaderType::HDR_SIMPLE;
#[no_mangle]
pub static mut fancy_header_name: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut fancy_header_default: [i8; 256] = [0; 256];
static mut no_job_header: i32 = 0 as i32;
#[no_mangle]
pub static mut highlight_bars: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut line_indent: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut line_indent_spec: *mut i8 = b"0\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut input_filter: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut borders: i32 = 0 as i32;
#[no_mangle]
pub static mut page_prefeed: i32 = 0 as i32;
#[no_mangle]
pub static mut lines_per_page: u32 = -(1 as i32) as u32;
#[no_mangle]
pub static mut mail: i32 = 0 as i32;
#[no_mangle]
pub static mut media_name: *mut i8 = b"A4\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut media_name_buffer: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut num_copies: i32 = 1 as i32;
#[no_mangle]
pub static mut nl: i32 = -(1 as i32);
#[no_mangle]
pub static mut output_file: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut list_missing_characters: i32 = 0 as i32;
#[no_mangle]
pub static mut quiet: i32 = 0 as i32;
#[no_mangle]
pub static mut landscape: i32 = 0 as i32;
#[no_mangle]
pub static mut baselineskip: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut title: *mut i8 = b"Enscript Output\0" as *const u8 as *const i8
    as *mut i8;
#[no_mangle]
pub static mut title_given: i32 = 0 as i32;
#[no_mangle]
pub static mut tabsize: i32 = 8 as i32;
#[no_mangle]
pub static mut ul_gray: libc::c_double = 0.8f64;
#[no_mangle]
pub static mut ul_ptsize: FontPoint = {
    let mut init = font_point_st {
        w: 200.0f64,
        h: 200.0f64,
    };
    init
};
#[no_mangle]
pub static mut ul_font: *mut i8 = b"Times-Roman\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut underlay: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut ul_position_buf: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut ul_position: *mut i8 = b"+0-0\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut ul_x: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_y: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_angle: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_style: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut ul_style_str: *mut i8 = b"outline\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut ul_style_str_buf: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut ul_position_p: i32 = 0 as i32;
#[no_mangle]
pub static mut ul_angle_p: i32 = 0 as i32;
#[no_mangle]
pub static mut nup: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut nup_exp: u32 = 0 as i32 as u32;
#[no_mangle]
pub static mut nup_rows: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut nup_columns: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut nup_landscape: i32 = 0 as i32;
#[no_mangle]
pub static mut nup_width: u32 = 0;
#[no_mangle]
pub static mut nup_height: u32 = 0;
#[no_mangle]
pub static mut nup_scale: libc::c_double = 0.;
#[no_mangle]
pub static mut verbose: i32 = 0 as i32;
#[no_mangle]
pub static mut output_language: *mut i8 = b"PostScript\0" as *const u8 as *const i8
    as *mut i8;
#[no_mangle]
pub static mut output_language_pass_through: i32 = 0 as i32;
#[no_mangle]
pub static mut encoding: InputEncoding = InputEncoding::ENC_ISO_8859_1;
#[no_mangle]
pub static mut encoding_name: *mut i8 = b"88591\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut encoding_name_buffer: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut interpret_formfeed: i32 = 1 as i32;
#[no_mangle]
pub static mut pass_through: i32 = 0 as i32;
#[no_mangle]
pub static mut input_filter_stdin: *mut i8 = b"\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut horizontal_column_height: libc::c_double = 283465.0f64;
#[no_mangle]
pub static mut help_pretty_print: i32 = 0 as i32;
#[no_mangle]
pub static mut highlight_bar_gray: libc::c_double = 0.97f64;
#[no_mangle]
pub static mut list_media: i32 = 0 as i32;
#[no_mangle]
pub static mut list_options: i32 = 0 as i32;
#[no_mangle]
pub static mut margins_spec: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut mark_wrapped_lines_style_name: [i8; 256] = [
    0 as i32 as i8,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut mark_wrapped_lines_style: MarkWrappedLinesStyle = MarkWrappedLinesStyle::MWLS_NONE;
#[no_mangle]
pub static mut npf_name: *mut i8 = b"octal\0" as *const u8 as *const i8 as *mut i8;
#[no_mangle]
pub static mut npf_name_buf: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut non_printable_format: NonPrintableFormat = NonPrintableFormat::NPF_OCTAL;
#[no_mangle]
pub static mut nup_xpad: u32 = 10 as i32 as u32;
#[no_mangle]
pub static mut nup_ypad: u32 = 10 as i32 as u32;
#[no_mangle]
pub static mut page_label_format: *mut i8 = b"short\0" as *const u8 as *const i8
    as *mut i8;
#[no_mangle]
pub static mut page_label_format_buf: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut page_label: PageLabelFormat = PageLabelFormat::LABEL_SHORT;
#[no_mangle]
pub static mut pslevel: u32 = 2 as i32 as u32;
#[no_mangle]
pub static mut printer_options: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut rotate_even_pages: i32 = 0 as i32;
#[no_mangle]
pub static mut slicing: i32 = 0 as i32;
#[no_mangle]
pub static mut slice: u32 = 1 as i32 as u32;
#[no_mangle]
pub static mut toc: i32 = 0 as i32;
#[no_mangle]
pub static mut toc_fname: [i8; 512] = [0; 512];
#[no_mangle]
pub static mut toc_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut toc_fmt_string: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub static mut accept_composites: i32 = 0 as i32;
#[no_mangle]
pub static mut append_ctrl_D: i32 = 0 as i32;
#[no_mangle]
pub static mut clean_7bit: i32 = 1 as i32;
#[no_mangle]
pub static mut formfeed_type: FormFeedType = FormFeedType::FORMFEED_COLUMN;
#[no_mangle]
pub static mut generate_PageSize: i32 = 1 as i32;
#[no_mangle]
pub static mut no_job_header_switch: [i8; 16] = [0; 16];
#[no_mangle]
pub static mut output_first_line: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut queue_param: [i8; 16] = [0; 16];
#[no_mangle]
pub static mut spooler_command: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut states_color_model: [i8; 50] = [0; 50];
#[no_mangle]
pub static mut states_config_file: [i8; 256] = [0; 256];
#[no_mangle]
pub static mut states_highlight_level: [i8; 50] = [0; 50];
#[no_mangle]
pub static mut states_path: [i8; 1024] = [0; 1024];
#[no_mangle]
pub static mut line_highlight_gray: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut bggray: libc::c_double = 1.0f64;
static mut long_options: [option; 73] = unsafe {
    [
        {
            let mut init = option {
                name: b"columns\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pages\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-align\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"header\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-header\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"truncate-lines\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-numbers\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"printer\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"setpagedevice\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"escapes\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pretty-print\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"font\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"header-font\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"print-anyway\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fancy-header\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-job-header\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"highlight-bars\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"indent\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"filter\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"borders\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'j' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"page-prefeed\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-page-prefeed\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'K' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lineprinter\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lines-per-page\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mail\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"media\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"copies\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"newline\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'N' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"missing-characters\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"landscape\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"portrait\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"baselineskip\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"statusdict\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"title\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"tabsize\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"underlay\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"language\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'W' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"encoding\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'X' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-formfeed\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pass-through\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 'Z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"color\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 142 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"download-font\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 131 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"filter-stdin\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 138 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"h-column-height\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 148 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 135 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"help-pretty-print\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 141 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"highlight-bar-gray\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 136 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"list-media\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &list_media as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"list-options\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &list_options as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"margins\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 144 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-wrapped-lines\0" as *const u8 as *const i8,
                has_arg: 2 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 143 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"non-printable-format\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 134 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup-xpad\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 145 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup-ypad\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 146 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"page-label-format\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 130 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ps-level\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 149 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"printer-options\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 139 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"rotate-even-pages\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 150 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"slice\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 140 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"toc\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: &toc as *const i32 as *mut i32,
                val: 1 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"word-wrap\0" as *const u8 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 147 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-angle\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 132 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-font\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 128 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-gray\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 129 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-position\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 133 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-style\0" as *const u8 as *const i8,
                has_arg: 1 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 137 as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const i8,
                has_arg: 0 as i32,
                flag: 0 as *const i32 as *mut i32,
                val: 0 as i32,
            };
            init
        },
    ]
};
static mut encodings: [C2RustUnnamed_0; 17] = [
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88591\0" as *const u8 as *const i8 as *mut i8,
                b"latin1\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_1,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88592\0" as *const u8 as *const i8 as *mut i8,
                b"latin2\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_2,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88593\0" as *const u8 as *const i8 as *mut i8,
                b"latin3\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_3,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88594\0" as *const u8 as *const i8 as *mut i8,
                b"latin4\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_4,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88595\0" as *const u8 as *const i8 as *mut i8,
                b"cyrillic\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_5,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88597\0" as *const u8 as *const i8 as *mut i8,
                b"greek\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_7,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ascii\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ASCII,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"asciifise\0" as *const u8 as *const i8 as *mut i8,
                b"asciifi\0" as *const u8 as *const i8 as *mut i8,
                b"asciise\0" as *const u8 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ASCII_FISE,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"asciidkno\0" as *const u8 as *const i8 as *mut i8,
                b"asciidk\0" as *const u8 as *const i8 as *mut i8,
                b"asciino\0" as *const u8 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ASCII_DKNO,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ibmpc\0" as *const u8 as *const i8 as *mut i8,
                b"pc\0" as *const u8 as *const i8 as *mut i8,
                b"dos\0" as *const u8 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_IBMPC,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"mac\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_MAC,
            nl: '\r' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"vms\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_VMS,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"hp8\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_HP8,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"koi8\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_KOI8,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ps\0" as *const u8 as *const i8 as *mut i8,
                b"PS\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_PS,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"pslatin1\0" as *const u8 as *const i8 as *mut i8,
                b"ISOLatin1Encoding\0" as *const u8 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_1,
            nl: '\n' as i32,
            bs: 8 as i32,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
                0 as *const i8 as *mut i8,
            ],
            encoding: InputEncoding::ENC_ISO_8859_1,
            nl: 0 as i32,
            bs: 0 as i32,
        };
        init
    },
];
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut is: InputStream = InputStream {
        is_pipe: 0,
        fp: 0 as *mut FILE,
        buf: [0; 4096],
        data_in_buf: 0,
        bufpos: 0,
        nreads: 0,
        unget_ch: 0 as *mut u8,
        unget_pos: 0,
        unget_alloc: 0,
    };
    let mut tim: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut found: i32 = 0;
    let mut ui: u32 = 0;
    let mut mentry: *mut MediaEntry = 0 as *mut MediaEntry;
    let mut afm_error: AFMError = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    let mut retval: i32 = 0 as i32;
    program = strrchr(*argv.offset(0 as i32 as isize), '/' as i32);
    if program.is_null() {
        program = *argv.offset(0 as i32 as isize);
    } else {
        program = program.offset(1);
        program;
    }
    let ref mut fresh0 = *argv.offset(0 as i32 as isize);
    *fresh0 = program;
    sprintf(
        version_string.as_mut_ptr(),
        b"GNU %s %s\0" as *const u8 as *const i8,
        b"enscript\0" as *const u8 as *const i8,
        b"1.6.1\0" as *const u8 as *const i8,
    );
    strcpy(ps_version_string.as_mut_ptr(), b"1.6.1\0" as *const u8 as *const i8);
    cp = strrchr(ps_version_string.as_mut_ptr(), '.' as i32);
    *cp = ' ' as i32 as i8;
    toc_fmt_string = dcgettext(
        0 as *const i8,
        b"$3v $-40N $3% pages $4L lines  $E $C\0" as *const u8 as *const i8,
        5 as i32,
    );
    setlocale(5 as i32, b"\0" as *const u8 as *const i8);
    bindtextdomain(
        b"enscript\0" as *const u8 as *const i8,
        b"/usr/local/share/locale\0" as *const u8 as *const i8,
    );
    textdomain(b"enscript\0" as *const u8 as *const i8);
    tim = time(0 as *mut time_t);
    tm = localtime(&mut tim);
    memcpy(
        &mut run_tm as *mut tm as *mut libc::c_void,
        tm as *const libc::c_void,
        ::core::mem::size_of::<tm>() as u64,
    );
    sprintf(
        date_string.as_mut_ptr(),
        b"%s\0" as *const u8 as *const i8,
        asctime(&mut run_tm),
    );
    i = strlen(date_string.as_mut_ptr()) as i32;
    date_string[(i - 1 as i32) as usize] = '\0' as i32 as i8;
    passwd = getpwuid(getuid());
    if passwd.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't get passwd entry for uid=%d: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            getuid(),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    strcpy(spooler_command.as_mut_ptr(), b"lpr\0" as *const u8 as *const i8);
    strcpy(queue_param.as_mut_ptr(), b"-P\0" as *const u8 as *const i8);
    strcpy(no_job_header_switch.as_mut_ptr(), b"-h\0" as *const u8 as *const i8);
    strcpy(fancy_header_default.as_mut_ptr(), b"enscript\0" as *const u8 as *const i8);
    strcpy(
        output_first_line.as_mut_ptr(),
        b"%!PS-Adobe-3.0\0" as *const u8 as *const i8,
    );
    cp = getenv(b"ENSCRIPT_LIBRARY\0" as *const u8 as *const i8);
    if !cp.is_null() {
        enscript_library = cp;
    }
    strcpy(states_color_model.as_mut_ptr(), b"blackwhite\0" as *const u8 as *const i8);
    sprintf(
        states_config_file.as_mut_ptr(),
        b"%s/enscript.st\0" as *const u8 as *const i8,
        enscript_library,
    );
    strcpy(states_highlight_level.as_mut_ptr(), b"heavy\0" as *const u8 as *const i8);
    strcpy(states_path.as_mut_ptr(), b"states\0" as *const u8 as *const i8);
    sprintf(
        libpath.as_mut_ptr(),
        b"%s%c%s/.enscript\0" as *const u8 as *const i8,
        enscript_library,
        ':' as i32,
        (*passwd).pw_dir,
    );
    res_fonts = strhash_init();
    download_fonts = strhash_init();
    pagedevice = strhash_init();
    statusdict = strhash_init();
    user_strings = strhash_init();
    if read_config(
        b"/usr/local/etc\0" as *const u8 as *const i8 as *mut i8,
        b"enscript.cfg\0" as *const u8 as *const i8 as *mut i8,
    ) == 0
    {
        let mut saved_errno: i32 = *__errno_location();
        if read_config(
            enscript_library,
            b"enscript.cfg\0" as *const u8 as *const i8 as *mut i8,
        ) == 0
        {
            if read_config(
                b"../lib\0" as *const u8 as *const i8 as *mut i8,
                b"enscript.cfg\0" as *const u8 as *const i8 as *mut i8,
            ) == 0
            {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"couldn't open config file \"%s/%s\": %s\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    enscript_library,
                    b"enscript.cfg\0" as *const u8 as *const i8,
                    strerror(saved_errno),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            strcat(libpath.as_mut_ptr(), b":../lib\0" as *const u8 as *const i8);
        }
    }
    read_config(
        b"/usr/local/etc\0" as *const u8 as *const i8 as *mut i8,
        b"enscriptsite.cfg\0" as *const u8 as *const i8 as *mut i8,
    );
    read_config((*passwd).pw_dir, b".enscriptrc\0" as *const u8 as *const i8 as *mut i8);
    handle_env_options(b"ENSCRIPT\0" as *const u8 as *const i8 as *mut i8);
    handle_env_options(b"GENSCRIPT\0" as *const u8 as *const i8 as *mut i8);
    handle_options(argc, argv);
    default_escape_char = escape_char;
    found = 0 as i32;
    i = 0 as i32;
    while found == 0 && !(encodings[i as usize].names[0 as i32 as usize]).is_null() {
        j = 0 as i32;
        while j < 3 as i32 {
            if !(encodings[i as usize].names[j as usize]).is_null()
                && strcmp(encodings[i as usize].names[j as usize], encoding_name)
                    == 0 as i32
            {
                encoding = encodings[i as usize].encoding;
                encoding_name = encodings[i as usize].names[0 as i32 as usize];
                if nl < 0 as i32 {
                    nl = encodings[i as usize].nl;
                }
                bs = encodings[i as usize].bs;
                found = 1 as i32;
                break;
            } else {
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
    if found == 0 {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"unknown encoding: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            encoding_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if user_body_font_defined == 0 && landscape != 0 && num_columns > 1 as i32 {
        Fpt.h = 7.0f64;
        Fpt.w = Fpt.h;
    }
    afm_cache = strhash_init();
    afm_info_cache = strhash_init();
    afm_error = afm_create(afm_path, verbose as u32, &mut afm);
    if afm_error != 0 as i32 as u32 {
        let mut buf: [i8; 256] = [0; 256];
        afm_error_to_string(afm_error, buf.as_mut_ptr());
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"couldn't open AFM library: %s\0" as *const u8 as *const i8,
                5 as i32,
            ),
            buf.as_mut_ptr(),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    default_Fpt.w = Fpt.w;
    default_Fpt.h = Fpt.h;
    default_Fname = Fname;
    strhash_put(
        res_fonts,
        Fname,
        (strlen(Fname)).wrapping_add(1 as i32 as u64) as i32,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        res_fonts,
        HFname,
        (strlen(HFname)).wrapping_add(1 as i32 as u64) as i32,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        download_fonts,
        Fname,
        (strlen(Fname)).wrapping_add(1 as i32 as u64) as i32,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        download_fonts,
        HFname,
        (strlen(HFname)).wrapping_add(1 as i32 as u64) as i32,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    read_font_info();
    line_indent = parse_float(line_indent_spec, 1 as i32, 1 as i32);
    if list_media != 0 {
        printf(
            dcgettext(
                0 as *const i8,
                b"known media:\nname             width\theight\tllx\tlly\turx\tury\n------------------------------------------------------------\n\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        mentry = media_names;
        while !mentry.is_null() {
            printf(
                b"%-16s %d\t%d\t%d\t%d\t%d\t%d\n\0" as *const u8 as *const i8,
                (*mentry).name,
                (*mentry).w,
                (*mentry).h,
                (*mentry).llx,
                (*mentry).lly,
                (*mentry).urx,
                (*mentry).ury,
            );
            mentry = (*mentry).next;
        }
        exit(0 as i32);
    }
    mentry = media_names;
    while !mentry.is_null() {
        if strcmp(media_name, (*mentry).name) == 0 as i32 {
            media = mentry;
            break;
        } else {
            mentry = (*mentry).next;
        }
    }
    if media.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"do not know anything about media \"%s\"\0" as *const u8 as *const i8,
                5 as i32,
            ),
            media_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if !margins_spec.is_null() {
        i = 0 as i32;
        while i < 4 as i32 {
            if *margins_spec as i32 == '\0' as i32 {
                break;
            }
            if *margins_spec as i32 == ':' as i32 {
                margins_spec = margins_spec.offset(1);
                margins_spec;
            } else {
                j = atoi(margins_spec);
                while *margins_spec as i32 != ':' as i32
                    && *margins_spec as i32 != '\0' as i32
                {
                    margins_spec = margins_spec.offset(1);
                    margins_spec;
                }
                if *margins_spec as i32 == ':' as i32 {
                    margins_spec = margins_spec.offset(1);
                    margins_spec;
                }
                match i {
                    0 => {
                        (*media).llx = j;
                    }
                    1 => {
                        (*media).urx = (*media).w - j;
                    }
                    2 => {
                        (*media).ury = (*media).h - j;
                    }
                    3 => {
                        (*media).lly = j;
                    }
                    _ => {}
                }
            }
            i += 1;
            i;
        }
        if quiet == 0 && verbose >= 1 as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"set new marginals for media `%s' (%dx%d): llx=%d, lly=%d, urx=%d, ury=%d\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                (*media).name,
                (*media).w,
                (*media).h,
                (*media).llx,
                (*media).lly,
                (*media).urx,
                (*media).ury,
            );
        }
    }
    if strcmp(page_label_format, b"short\0" as *const u8 as *const i8) == 0 as i32 {
        page_label = PageLabelFormat::LABEL_SHORT;
    } else if strcmp(page_label_format, b"long\0" as *const u8 as *const i8) == 0 as i32
    {
        page_label = PageLabelFormat::LABEL_LONG;
    } else {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"illegal page label format \"%s\"\0" as *const u8 as *const i8,
                5 as i32,
            ),
            page_label_format,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if strcmp(npf_name, b"space\0" as *const u8 as *const i8) == 0 as i32 {
        non_printable_format = NonPrintableFormat::NPF_SPACE;
    } else if strcmp(npf_name, b"questionmark\0" as *const u8 as *const i8) == 0 as i32 {
        non_printable_format = NonPrintableFormat::NPF_QUESTIONMARK;
    } else if strcmp(npf_name, b"caret\0" as *const u8 as *const i8) == 0 as i32 {
        non_printable_format = NonPrintableFormat::NPF_CARET;
    } else if strcmp(npf_name, b"octal\0" as *const u8 as *const i8) == 0 as i32 {
        non_printable_format = NonPrintableFormat::NPF_OCTAL;
    } else {
        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const i8,
                b"illegal non-printable format \"%s\"\0" as *const u8 as *const i8,
                5 as i32,
            ),
            npf_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        fflush(stderr);
        exit(1 as i32);
    }
    if mark_wrapped_lines_style_name[0 as i32 as usize] != 0 {
        if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"none\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mark_wrapped_lines_style = MarkWrappedLinesStyle::MWLS_NONE;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"plus\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mark_wrapped_lines_style = MarkWrappedLinesStyle::MWLS_PLUS;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"box\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mark_wrapped_lines_style = MarkWrappedLinesStyle::MWLS_BOX;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"arrow\0" as *const u8 as *const i8,
        ) == 0 as i32
        {
            mark_wrapped_lines_style = MarkWrappedLinesStyle::MWLS_ARROW;
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"illegal style for wrapped line marker: \"%s\"\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                mark_wrapped_lines_style_name.as_mut_ptr(),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
    }
    i = 0 as i32;
    loop {
        ui = nup >> i;
        if ui == 0 as i32 as u32 {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"illegal N-up argument: %d\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                nup,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
        if ui & 0x1 as i32 as u32 != 0 {
            if ui != 1 as i32 as u32 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"N-up argument must be power of 2: %d\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    nup,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            nup_exp = i as u32;
            break;
        } else {
            i += 1;
            i;
        }
    }
    nup_rows = nup_exp.wrapping_div(2 as i32 as u32).wrapping_mul(2 as i32 as u32);
    if nup_rows == 0 as i32 as u32 {
        nup_rows = 1 as i32 as u32;
    }
    nup_columns = nup_exp
        .wrapping_add(1 as i32 as u32)
        .wrapping_div(2 as i32 as u32)
        .wrapping_mul(2 as i32 as u32);
    if nup_columns == 0 as i32 as u32 {
        nup_columns = 1 as i32 as u32;
    }
    nup_landscape = (nup_exp & 0x1 as i32 as u32) as i32;
    if landscape != 0 {
        d_page_w = (*media).ury - (*media).lly;
        d_page_h = (*media).urx - (*media).llx;
    } else {
        d_page_w = (*media).urx - (*media).llx;
        d_page_h = (*media).ury - (*media).lly;
    }
    if nup_landscape != 0 {
        nup_width = ((*media).ury - (*media).lly) as u32;
        nup_height = ((*media).urx - (*media).llx) as u32;
    } else {
        nup_width = ((*media).urx - (*media).llx) as u32;
        nup_height = ((*media).ury - (*media).lly) as u32;
    }
    let mut w: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    w = (nup_width as libc::c_double
        - nup_columns.wrapping_sub(1 as i32 as u32).wrapping_mul(nup_xpad)
            as libc::c_double) / nup_columns as libc::c_double;
    h = (nup_height as libc::c_double
        - nup_rows.wrapping_sub(1 as i32 as u32).wrapping_mul(nup_ypad)
            as libc::c_double) / nup_rows as libc::c_double;
    nup_width = w as u32;
    nup_height = h as u32;
    w = w / ((*media).urx - (*media).llx) as libc::c_double;
    h = h / ((*media).ury - (*media).lly) as libc::c_double;
    nup_scale = if w < h { w } else { h };
    if !underlay.is_null() {
        strhash_put(
            res_fonts,
            ul_font,
            (strlen(ul_font)).wrapping_add(1 as i32 as u64) as i32,
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        underlay = escape_string(underlay);
    }
    ul_x = strtod(ul_position, &mut cp);
    if !(cp == ul_position) {
        if *ul_position.offset(0 as i32 as isize) as i32 == '-' as i32 {
            ul_x += d_page_w as libc::c_double;
        }
        ul_y = strtod(cp, &mut cp2);
        if !(cp2 == cp) {
            if *cp.offset(0 as i32 as isize) as i32 == '-' as i32 {
                ul_y += d_page_h as libc::c_double;
            }
            if ul_angle_p == 0 {
                ul_angle = atan2(-d_page_h as libc::c_double, d_page_w as libc::c_double)
                    / 3.14159265f64 * 180 as i32 as libc::c_double;
            }
            if strcmp(ul_style_str, b"outline\0" as *const u8 as *const i8) == 0 as i32 {
                ul_style = 0 as i32 as u32;
            } else if strcmp(ul_style_str, b"filled\0" as *const u8 as *const i8)
                == 0 as i32
            {
                ul_style = 1 as i32 as u32;
            } else {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"illegal underlay style: %s\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    ul_style_str,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            d_header_w = d_page_w;
            match header as u32 {
                0 => {
                    d_header_h = 0 as i32;
                }
                1 => {
                    d_header_h = (HFpt.h * 1.5f64) as i32;
                }
                2 => {
                    d_header_h = 36 as i32;
                }
                _ => {}
            }
            if help_pretty_print != 0 {
                sprintf(
                    pp_filter.as_mut_ptr(),
                    b"%s -f \"%s\" -s describe_languages \"%s\"\0" as *const u8
                        as *const i8,
                    states_path.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                );
                system(pp_filter.as_mut_ptr());
                exit(0 as i32);
            }
            if list_options != 0 {
                do_list_options();
                exit(1 as i32);
            }
            if output_language_pass_through != 0 {
                let mut start_state: *mut i8 = 0 as *mut i8;
                if !pp_start_state.is_null() {
                    start_state = pp_start_state;
                } else if pretty_print != 0 {
                    start_state = 0 as *mut i8;
                } else {
                    start_state = b"passthrough\0" as *const u8 as *const i8 as *mut i8;
                }
                j = 0 as i32;
                i = optind;
                while i < argc {
                    j = (j as u64)
                        .wrapping_add(
                            (strlen(*argv.offset(i as isize)))
                                .wrapping_add(1 as i32 as u64),
                        ) as i32 as i32;
                    i += 1;
                    i;
                }
                j += 256 as i32;
                cp = xmalloc(j as size_t) as *mut i8;
                sprintf(
                    cp,
                    b"%s -f \"%s\" %s%s -Dcolormodel=%s -Dhl_level=%s -Dlanguage=%s -Dnum_input_files=%d -Ddocument_title=\"%s\" -Dtoc=%d\0"
                        as *const u8 as *const i8,
                    states_path.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                    if !start_state.is_null() {
                        b"-s\0" as *const u8 as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    if !start_state.is_null() {
                        start_state
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                    states_color_model.as_mut_ptr(),
                    states_highlight_level.as_mut_ptr(),
                    output_language,
                    if optind == argc { 1 as i32 } else { argc - optind },
                    title,
                    toc,
                );
                i = optind;
                while i < argc {
                    strcat(cp, b" \0" as *const u8 as *const i8);
                    strcat(cp, *argv.offset(i as isize));
                    i += 1;
                    i;
                }
                if is_open(&mut is, stdin, 0 as *mut i8, cp) != 0 {
                    open_output_file();
                    process_file(
                        b"unused\0" as *const u8 as *const i8 as *mut i8,
                        &mut is,
                    );
                    is_close(&mut is);
                }
            } else {
                if pretty_print != 0 {
                    sprintf(
                        pp_filter.as_mut_ptr(),
                        b"%s -f \"%s\" %s%s -Dcolormodel=%s -Dhl_level=%s -Dfont_spec=%s@%g/%g \"%%s\"\0"
                            as *const u8 as *const i8,
                        states_path.as_mut_ptr(),
                        states_config_file.as_mut_ptr(),
                        if !pp_start_state.is_null() {
                            b"-s \0" as *const u8 as *const i8
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                        if !pp_start_state.is_null() {
                            pp_start_state
                        } else {
                            b"\0" as *const u8 as *const i8
                        },
                        states_color_model.as_mut_ptr(),
                        states_highlight_level.as_mut_ptr(),
                        Fname,
                        Fpt.w,
                        Fpt.h,
                    );
                    input_filter = pp_filter.as_mut_ptr();
                    input_filter_stdin = b"-\0" as *const u8 as *const i8 as *mut i8;
                }
                if toc != 0 {
                    cp = tmpnam(toc_fname.as_mut_ptr());
                    if cp.is_null() {
                        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"couldn't create toc file name: %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                        fflush(stderr);
                        exit(1 as i32);
                    }
                    toc_fp = fopen(
                        toc_fname.as_mut_ptr(),
                        b"w+b\0" as *const u8 as *const i8,
                    );
                    if toc_fp.is_null() {
                        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"couldn't create toc file \"%s\": %s\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            toc_fname.as_mut_ptr(),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                        fflush(stderr);
                        exit(1 as i32);
                    }
                    if remove(toc_fname.as_mut_ptr()) == 0 as i32 {
                        toc_fname[0 as i32 as usize] = '\0' as i32 as i8;
                    }
                }
                if optind == argc {
                    memcpy(
                        &mut mod_tm as *mut tm as *mut libc::c_void,
                        &mut run_tm as *mut tm as *const libc::c_void,
                        ::core::mem::size_of::<tm>() as u64,
                    );
                    if is_open(&mut is, stdin, 0 as *mut i8, input_filter) != 0 {
                        open_output_file();
                        process_file(
                            (if title_given != 0 {
                                title
                            } else {
                                b"\0" as *const u8 as *const i8
                            }) as *mut i8,
                            &mut is,
                        );
                        is_close(&mut is);
                    }
                } else {
                    while optind < argc {
                        if is_open(
                            &mut is,
                            0 as *mut FILE,
                            *argv.offset(optind as isize),
                            input_filter,
                        ) != 0
                        {
                            let mut stat_st: stat = stat {
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
                            if stat(*argv.offset(optind as isize), &mut stat_st)
                                == 0 as i32
                            {
                                tim = stat_st.st_mtim.tv_sec;
                                tm = localtime(&mut tim);
                                memcpy(
                                    &mut mod_tm as *mut tm as *mut libc::c_void,
                                    tm as *const libc::c_void,
                                    ::core::mem::size_of::<tm>() as u64,
                                );
                                open_output_file();
                                process_file(*argv.offset(optind as isize), &mut is);
                            } else {
                                fprintf(
                                    stderr,
                                    b"%s: \0" as *const u8 as *const i8,
                                    program,
                                );
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const i8,
                                        b"couldn't stat input file \"%s\": %s\0" as *const u8
                                            as *const i8,
                                        5 as i32,
                                    ),
                                    *argv.offset(optind as isize),
                                    strerror(*__errno_location()),
                                );
                                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                                fflush(stderr);
                            }
                            is_close(&mut is);
                        }
                        optind += 1;
                        optind;
                    }
                }
                if toc != 0 {
                    toc = 0 as i32;
                    special_escapes = 1 as i32;
                    line_numbers = 0 as i32;
                    if fseek(toc_fp, 0 as i32 as i64, 0 as i32) != 0 as i32 {
                        fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"couldn't rewind toc file: %s\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                        fflush(stderr);
                        exit(1 as i32);
                    }
                    memcpy(
                        &mut mod_tm as *mut tm as *mut libc::c_void,
                        &mut run_tm as *mut tm as *const libc::c_void,
                        ::core::mem::size_of::<tm>() as u64,
                    );
                    if is_open(&mut is, toc_fp, 0 as *mut i8, 0 as *mut i8) != 0 {
                        process_file(
                            dcgettext(
                                0 as *const i8,
                                b"Table of Contents\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            &mut is,
                        );
                        is_close(&mut is);
                    }
                    fclose(toc_fp);
                    if toc_fname[0 as i32 as usize] != 0 {
                        remove(toc_fname.as_mut_ptr());
                    }
                }
                dump_ps_trailer();
                if !ofp.is_null() && append_ctrl_D != 0 {
                    fprintf(ofp, b"\x04\n\0" as *const u8 as *const i8);
                }
            }
            close_output_file();
            if ofp.is_null() {
                if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"no output generated\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            } else if output_language_pass_through != 0 {
                if output_file.is_null() {
                    if quiet == 0 && verbose >= 0 as i32 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"output sent to %s\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            if !printer.is_null() {
                                printer
                            } else {
                                dcgettext(
                                    0 as *const i8,
                                    b"printer\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            },
                        );
                    }
                } else if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"output left in %s\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        if output_file == 1 as i32 as *mut i8 {
                            b"-\0" as *const u8 as *const i8
                        } else {
                            output_file
                        },
                    );
                }
            } else {
                let mut real_total_pages: u32 = 0;
                if nup > 1 as i32 as u32 {
                    if total_pages > 0 as i32 {
                        real_total_pages = ((total_pages - 1 as i32) as u32)
                            .wrapping_div(nup)
                            .wrapping_add(1 as i32 as u32);
                    } else {
                        real_total_pages = 0 as i32 as u32;
                    }
                } else {
                    real_total_pages = total_pages as u32;
                }
                if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"[ %d pages * %d copy ]\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        real_total_pages,
                        num_copies,
                    );
                }
                if output_file.is_null() {
                    if quiet == 0 && verbose >= 0 as i32 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b" sent to %s\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            if !printer.is_null() {
                                printer
                            } else {
                                dcgettext(
                                    0 as *const i8,
                                    b"printer\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            },
                        );
                    }
                } else if quiet == 0 && verbose >= 0 as i32 {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b" left in %s\n\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        if output_file == 1 as i32 as *mut i8 {
                            b"-\0" as *const u8 as *const i8
                        } else {
                            output_file
                        },
                    );
                }
                if num_truncated_lines != 0 {
                    retval |= 2 as i32;
                    if quiet == 0 && verbose >= 0 as i32 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%d lines were %s\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            num_truncated_lines,
                            if line_end as u32 == LineEndType::LE_TRUNCATE as i32 as u32
                            {
                                dcgettext(
                                    0 as *const i8,
                                    b"truncated\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            } else {
                                dcgettext(
                                    0 as *const i8,
                                    b"wrapped\0" as *const u8 as *const i8,
                                    5 as i32,
                                )
                            },
                        );
                    }
                }
                if num_missing_chars != 0 {
                    retval |= 4 as i32;
                    if quiet == 0 && verbose >= 0 as i32 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%d characters were missing\n\0" as *const u8 as *const i8,
                                5 as i32,
                            ),
                            num_missing_chars,
                        );
                    }
                    if list_missing_characters != 0 {
                        if quiet == 0 && verbose >= 0 as i32 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"missing character codes (decimal):\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        do_list_missing_characters(missing_chars.as_mut_ptr());
                    }
                }
                if num_non_printable_chars != 0 {
                    retval |= 8 as i32;
                    if quiet == 0 && verbose >= 0 as i32 {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const i8,
                                b"%d non-printable characters\n\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                            num_non_printable_chars,
                        );
                    }
                    if list_missing_characters != 0 {
                        if quiet == 0 && verbose >= 0 as i32 {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const i8,
                                    b"non-printable character codes (decimal):\n\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                        }
                        do_list_missing_characters(non_printable_chars.as_mut_ptr());
                    }
                }
            }
            return retval;
        }
    }
    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
    fprintf(
        stderr,
        dcgettext(
            0 as *const i8,
            b"malformed underlay position: %s\0" as *const u8 as *const i8,
            5 as i32,
        ),
        ul_position,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    fflush(stderr);
    exit(1 as i32);
}
unsafe extern "C" fn open_output_file() {
    if !ofp.is_null() {
        return;
    }
    if output_file.is_null() {
        let mut spooler_options: [i8; 512] = [0; 512];
        spooler_options[0 as i32 as usize] = '\0' as i32 as i8;
        if mail != 0 {
            strcat(spooler_options.as_mut_ptr(), b"-m \0" as *const u8 as *const i8);
        }
        if no_job_header != 0 {
            strcat(spooler_options.as_mut_ptr(), no_job_header_switch.as_mut_ptr());
            strcat(spooler_options.as_mut_ptr(), b" \0" as *const u8 as *const i8);
        }
        if !printer_options.is_null() {
            strcat(spooler_options.as_mut_ptr(), printer_options);
        }
        ofp = printer_open(
            spooler_command.as_mut_ptr(),
            spooler_options.as_mut_ptr(),
            queue_param.as_mut_ptr(),
            printer,
            &mut printer_context,
        );
        if ofp.is_null() {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"couldn't open printer `%s': %s\0" as *const u8 as *const i8,
                    5 as i32,
                ),
                printer,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
    } else if output_file == 1 as i32 as *mut i8 {
        ofp = stdout;
    } else {
        ofp = fopen(output_file, b"w\0" as *const u8 as *const i8);
        if ofp.is_null() {
            fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"couldn't create output file \"%s\": %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                output_file,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            fflush(stderr);
            exit(1 as i32);
        }
    };
}
unsafe extern "C" fn close_output_file() {
    if ofp.is_null() {
        return;
    }
    if output_file.is_null() {
        printer_close(printer_context);
    } else if output_file != 1 as i32 as *mut i8 {
        fclose(ofp);
    }
}
unsafe extern "C" fn handle_env_options(mut var: *mut i8) {
    let mut argc: i32 = 0;
    let mut argv: *mut *mut i8 = 0 as *mut *mut i8;
    let mut string: *mut i8 = 0 as *mut i8;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    string = getenv(var);
    if string.is_null() {
        return;
    }
    if quiet == 0 && verbose >= 2 as i32 {
        fprintf(
            stderr,
            b"handle_env_options(): %s=\"%s\"\n\0" as *const u8 as *const i8,
            var,
            string,
        );
    }
    str = xstrdup(string);
    argc = (strlen(str))
        .wrapping_add(1 as i32 as u64)
        .wrapping_div(2 as i32 as u64)
        .wrapping_add(2 as i32 as u64) as i32;
    argv = xcalloc(argc as size_t, ::core::mem::size_of::<*mut i8>() as u64)
        as *mut *mut i8;
    argc = 0 as i32;
    let fresh1 = argc;
    argc = argc + 1;
    let ref mut fresh2 = *argv.offset(fresh1 as isize);
    *fresh2 = program;
    i = 0 as i32;
    while *str.offset(i as isize) != 0 {
        while *str.offset(i as isize) as i32 != 0
            && *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize) as i32
                & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 != 0
        {
            i += 1;
            i;
        }
        if *str.offset(i as isize) == 0 {
            break;
        }
        if *str.offset(i as isize) as i32 == '"' as i32
            || *str.offset(i as isize) as i32 == '\'' as i32
        {
            let fresh3 = i;
            i = i + 1;
            let mut endch: i32 = *str.offset(fresh3 as isize) as i32;
            let fresh4 = argc;
            argc = argc + 1;
            let ref mut fresh5 = *argv.offset(fresh4 as isize);
            *fresh5 = str.offset(i as isize);
            while *str.offset(i as isize) as i32 != 0
                && *str.offset(i as isize) as i32 != endch
            {
                i += 1;
                i;
            }
            if *str.offset(i as isize) == 0 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"syntax error in option string %s=\"%s\":\nmissing end of quotation: %c\0"
                            as *const u8 as *const i8,
                        5 as i32,
                    ),
                    var,
                    string,
                    endch,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
            let fresh6 = i;
            i = i + 1;
            *str.offset(fresh6 as isize) = '\0' as i32 as i8;
        } else {
            let fresh7 = argc;
            argc = argc + 1;
            let ref mut fresh8 = *argv.offset(fresh7 as isize);
            *fresh8 = str.offset(i as isize);
            while *str.offset(i as isize) as i32 != 0
                && *(*__ctype_b_loc()).offset(*str.offset(i as isize) as i32 as isize)
                    as i32 & C2RustUnnamed::_ISspace as i32 as libc::c_ushort as i32 == 0
            {
                i += 1;
                i;
            }
            if *str.offset(i as isize) != 0 {
                let fresh9 = i;
                i = i + 1;
                *str.offset(fresh9 as isize) = '\0' as i32 as i8;
            }
        }
    }
    let ref mut fresh10 = *argv.offset(argc as isize);
    *fresh10 = 0 as *mut i8;
    if quiet == 0 && verbose >= 2 as i32 {
        fprintf(
            stderr,
            b"found following options (argc=%d):\n\0" as *const u8 as *const i8,
            argc,
        );
    }
    i = 0 as i32;
    while i < argc {
        if quiet == 0 && verbose >= 2 as i32 {
            fprintf(
                stderr,
                b"%3d = \"%s\"\n\0" as *const u8 as *const i8,
                i,
                *argv.offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    handle_options(argc, argv);
    if optind != argc {
        if quiet == 0 && verbose >= 0 as i32 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const i8,
                    b"warning: didn't process following options from environment variable %s:\n\0"
                        as *const u8 as *const i8,
                    5 as i32,
                ),
                var,
            );
        }
        while optind < argc {
            if quiet == 0 && verbose >= 0 as i32 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"  option %d = \"%s\"\n\0" as *const u8 as *const i8,
                        5 as i32,
                    ),
                    optind,
                    *argv.offset(optind as isize),
                );
            }
            optind += 1;
            optind;
        }
    }
    xfree(argv as *mut libc::c_void);
}
unsafe extern "C" fn handle_options(mut argc: i32, mut argv: *mut *mut i8) {
    let mut c: i32 = 0;
    let mut prange: *mut PageRange = 0 as *mut PageRange;
    optind = 0 as i32;
    loop {
        let mut option_index: i32 = 0 as i32;
        let mut cp: *const i8 = 0 as *const i8;
        c = getopt_long(
            argc,
            argv as *const *mut i8,
            b"#:12a:A:b:BcC::d:D:e::E::f:F:gGhH::i:I:jJ:kKlL:mM:n:N:o:Op:P:qrRs:S:t:T:u::U:vVW:X:zZ\0"
                as *const u8 as *const i8,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as i32) {
            break;
        }
        let mut current_block_199: u64;
        match c {
            0 => {
                cp = long_options[option_index as usize].name;
                if strcmp(cp, b"columns\0" as *const u8 as *const i8) == 0 as i32 {
                    num_columns = atoi(optarg);
                }
                current_block_199 = 17170253997621722914;
            }
            49 | 50 => {
                num_columns = c - '0' as i32;
                current_block_199 = 17170253997621722914;
            }
            97 => {
                prange = xcalloc(
                    1 as i32 as size_t,
                    ::core::mem::size_of::<PageRange>() as u64,
                ) as *mut PageRange;
                if strcmp(optarg, b"odd\0" as *const u8 as *const i8) == 0 as i32 {
                    (*prange).odd = 1 as i32;
                } else if strcmp(optarg, b"even\0" as *const u8 as *const i8) == 0 as i32
                {
                    (*prange).even = 1 as i32;
                } else {
                    cp = strchr(optarg, '-' as i32);
                    if !cp.is_null() {
                        if *optarg.offset(0 as i32 as isize) as i32 == '-' as i32 {
                            (*prange).end = atoi(optarg.offset(1 as i32 as isize))
                                as u32;
                        } else if *cp.offset(1 as i32 as isize) as i32 == '\0' as i32 {
                            (*prange).start = atoi(optarg) as u32;
                            (*prange).end = -(1 as i32) as u32;
                        } else {
                            (*prange).start = atoi(optarg) as u32;
                            (*prange).end = atoi(cp.offset(1 as i32 as isize)) as u32;
                        }
                    } else {
                        (*prange).end = atoi(optarg) as u32;
                        (*prange).start = (*prange).end;
                    }
                }
                (*prange).next = page_ranges;
                page_ranges = prange;
                current_block_199 = 17170253997621722914;
            }
            65 => {
                file_align = atoi(optarg) as u32;
                if file_align == 0 as i32 as u32 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"file alignment must be larger than zero\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            98 => {
                page_header = optarg;
                current_block_199 = 17170253997621722914;
            }
            66 => {
                header = HeaderType::HDR_NONE;
                current_block_199 = 17170253997621722914;
            }
            99 => {
                line_end = LineEndType::LE_TRUNCATE;
                current_block_199 = 17170253997621722914;
            }
            67 => {
                line_numbers = 1 as i32;
                if !optarg.is_null() {
                    start_line_number = atoi(optarg) as u32;
                }
                current_block_199 = 17170253997621722914;
            }
            100 | 80 => {
                printer = optarg;
                output_file = 0 as *mut i8;
                current_block_199 = 17170253997621722914;
            }
            68 => {
                parse_key_value_pair(pagedevice, optarg);
                current_block_199 = 17170253997621722914;
            }
            101 => {
                special_escapes = 1 as i32;
                if !optarg.is_null() {
                    if *(*__ctype_b_loc())
                        .offset(*optarg.offset(0 as i32 as isize) as i32 as isize) as i32
                        & C2RustUnnamed::_ISdigit as i32 as libc::c_ushort as i32 != 0
                    {
                        escape_char = strtoul(optarg, 0 as *mut *mut i8, 0 as i32)
                            as i32;
                    } else {
                        escape_char = *(optarg as *mut u8).offset(0 as i32 as isize)
                            as i32;
                    }
                }
                current_block_199 = 17170253997621722914;
            }
            69 => {
                pretty_print = 1 as i32;
                special_escapes = 1 as i32;
                escape_char = '\0' as i32;
                pp_start_state = optarg;
                current_block_199 = 17170253997621722914;
            }
            102 => {
                if parse_font_spec(optarg, &mut Fname, &mut Fpt) == 0 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"malformed font spec: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                user_body_font_defined = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            70 => {
                if parse_font_spec(optarg, &mut HFname, &mut HFpt) == 0 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"malformed font spec: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            103 => {
                current_block_199 = 17170253997621722914;
            }
            71 => {
                header = HeaderType::HDR_FANCY;
                if !optarg.is_null() {
                    fancy_header_name = optarg;
                } else {
                    fancy_header_name = fancy_header_default.as_mut_ptr();
                }
                if file_existsp(
                    fancy_header_name,
                    b".hdr\0" as *const u8 as *const i8 as *mut i8,
                ) == 0
                {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"couldn't find header definition file \"%s.hdr\"\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        fancy_header_name,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            104 => {
                no_job_header = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            72 => {
                if !optarg.is_null() {
                    highlight_bars = atoi(optarg) as u32;
                } else {
                    highlight_bars = 2 as i32 as u32;
                }
                current_block_199 = 17170253997621722914;
            }
            105 => {
                line_indent_spec = optarg;
                current_block_199 = 17170253997621722914;
            }
            73 => {
                input_filter = optarg;
                current_block_199 = 17170253997621722914;
            }
            106 => {
                borders = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            107 => {
                page_prefeed = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            75 => {
                page_prefeed = 0 as i32;
                current_block_199 = 17170253997621722914;
            }
            108 => {
                lines_per_page = 66 as i32 as u32;
                header = HeaderType::HDR_NONE;
                current_block_199 = 17170253997621722914;
            }
            76 => {
                lines_per_page = atoi(optarg) as u32;
                if lines_per_page <= 0 as i32 as u32 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"must print at least one line per each page: %s\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        *argv.offset(optind as isize),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            109 => {
                mail = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            77 => {
                media_name = optarg;
                current_block_199 = 17170253997621722914;
            }
            110 | 35 => {
                num_copies = atoi(optarg);
                current_block_199 = 17170253997621722914;
            }
            78 => {
                if !(*optarg.offset(0 as i32 as isize) as i32 == 'n' as i32
                    || *optarg.offset(0 as i32 as isize) as i32 == 'r' as i32)
                    || *optarg.offset(1 as i32 as isize) as i32 != '\0' as i32
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"%s: illegal newline character specifier: '%s': expected 'n' or 'r'\n\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                        program,
                        optarg,
                    );
                    current_block_199 = 18234923695733410170;
                } else {
                    if *optarg.offset(0 as i32 as isize) as i32 == 'n' as i32 {
                        nl = '\n' as i32;
                    } else {
                        nl = '\r' as i32;
                    }
                    current_block_199 = 17170253997621722914;
                }
            }
            111 | 112 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const i8) == 0 as i32 {
                    output_file = 1 as i32 as *mut i8;
                } else {
                    output_file = optarg;
                }
                current_block_199 = 17170253997621722914;
            }
            79 => {
                list_missing_characters = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            113 => {
                quiet = 1 as i32;
                verbose = 0 as i32;
                current_block_199 = 17170253997621722914;
            }
            114 => {
                landscape = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            82 => {
                landscape = 0 as i32;
                current_block_199 = 17170253997621722914;
            }
            115 => {
                baselineskip = atof(optarg);
                current_block_199 = 17170253997621722914;
            }
            83 => {
                parse_key_value_pair(statusdict, optarg);
                current_block_199 = 17170253997621722914;
            }
            116 | 74 => {
                title = optarg;
                title_given = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            84 => {
                tabsize = atoi(optarg);
                if tabsize <= 0 as i32 {
                    tabsize = 1 as i32;
                }
                current_block_199 = 17170253997621722914;
            }
            117 => {
                underlay = optarg;
                current_block_199 = 17170253997621722914;
            }
            85 => {
                nup = atoi(optarg) as u32;
                current_block_199 = 17170253997621722914;
            }
            118 => {
                if !optarg.is_null() {
                    verbose = atoi(optarg);
                } else {
                    verbose += 1;
                    verbose;
                }
                quiet = 0 as i32;
                current_block_199 = 17170253997621722914;
            }
            86 => {
                version();
                exit(0 as i32);
            }
            87 => {
                output_language = optarg;
                if strcmp(output_language, b"PostScript\0" as *const u8 as *const i8)
                    != 0 as i32
                {
                    output_language_pass_through = 1 as i32;
                }
                current_block_199 = 17170253997621722914;
            }
            88 => {
                encoding_name = optarg;
                current_block_199 = 17170253997621722914;
            }
            122 => {
                interpret_formfeed = 0 as i32;
                current_block_199 = 17170253997621722914;
            }
            90 => {
                pass_through = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            128 => {
                if parse_font_spec(optarg, &mut ul_font, &mut ul_ptsize) == 0 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"malformed font spec: %s\0" as *const u8 as *const i8,
                            5 as i32,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            129 => {
                ul_gray = atof(optarg);
                current_block_199 = 17170253997621722914;
            }
            130 => {
                page_label_format = optarg;
                current_block_199 = 17170253997621722914;
            }
            131 => {
                strhash_put(
                    download_fonts,
                    optarg,
                    (strlen(optarg)).wrapping_add(1 as i32 as u64) as i32,
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                current_block_199 = 17170253997621722914;
            }
            132 => {
                ul_angle = atof(optarg);
                ul_angle_p = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            133 => {
                ul_position = optarg;
                ul_position_p = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            134 => {
                npf_name = optarg;
                current_block_199 = 17170253997621722914;
            }
            135 => {
                usage();
                exit(0 as i32);
            }
            136 => {
                highlight_bar_gray = atof(optarg);
                current_block_199 = 17170253997621722914;
            }
            137 => {
                ul_style_str = optarg;
                current_block_199 = 17170253997621722914;
            }
            138 => {
                input_filter_stdin = optarg;
                current_block_199 = 17170253997621722914;
            }
            139 => {
                printer_options = optarg;
                current_block_199 = 17170253997621722914;
            }
            140 => {
                slicing = 1 as i32;
                slice = atoi(optarg) as u32;
                if slice <= 0 as i32 as u32 {
                    fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const i8,
                            b"slice must be greater than zero\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                    fflush(stderr);
                    exit(1 as i32);
                }
                current_block_199 = 17170253997621722914;
            }
            141 => {
                help_pretty_print = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            142 => {
                if optarg.is_null() {
                    strcpy(
                        states_color_model.as_mut_ptr(),
                        b"emacs\0" as *const u8 as *const i8,
                    );
                } else {
                    strcpy(states_color_model.as_mut_ptr(), optarg);
                }
                current_block_199 = 17170253997621722914;
            }
            143 => {
                if !optarg.is_null() {
                    strcpy(mark_wrapped_lines_style_name.as_mut_ptr(), optarg);
                } else {
                    mark_wrapped_lines_style = MarkWrappedLinesStyle::MWLS_BOX;
                }
                current_block_199 = 17170253997621722914;
            }
            144 => {
                margins_spec = optarg;
                current_block_199 = 17170253997621722914;
            }
            145 => {
                nup_xpad = atoi(optarg) as u32;
                current_block_199 = 17170253997621722914;
            }
            146 => {
                nup_ypad = atoi(optarg) as u32;
                current_block_199 = 17170253997621722914;
            }
            147 => {
                line_end = LineEndType::LE_WORD_WRAP;
                current_block_199 = 17170253997621722914;
            }
            148 => {
                horizontal_column_height = atof(optarg);
                formfeed_type = FormFeedType::FORMFEED_HCOLUMN;
                current_block_199 = 17170253997621722914;
            }
            149 => {
                pslevel = atoi(optarg) as u32;
                current_block_199 = 17170253997621722914;
            }
            150 => {
                rotate_even_pages = 1 as i32;
                current_block_199 = 17170253997621722914;
            }
            63 => {
                current_block_199 = 18234923695733410170;
            }
            _ => {
                printf(
                    b"Hey!  main() didn't handle option \"%c\" (%d)\0" as *const u8
                        as *const i8,
                    c,
                    c,
                );
                if !optarg.is_null() {
                    printf(b" with arg %s\0" as *const u8 as *const i8, optarg);
                }
                printf(b"\n\0" as *const u8 as *const i8);
                fprintf(stderr, b"%s: \0" as *const u8 as *const i8, program);
                fprintf(stderr, b"This is a bug!\0" as *const u8 as *const i8);
                fprintf(stderr, b"\n\0" as *const u8 as *const i8);
                fflush(stderr);
                exit(1 as i32);
            }
        }
        match current_block_199 {
            17170253997621722914 => {}
            _ => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const i8,
                        b"Try `%s --help' for more information.\n\0" as *const u8
                            as *const i8,
                        5 as i32,
                    ),
                    program,
                );
                exit(1 as i32);
            }
        }
    };
}
unsafe extern "C" fn do_list_options() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cp: *mut i8 = 0 as *mut i8;
    let mut cp2: *mut i8 = 0 as *mut i8;
    printf(b"libpath=\"%s\"\n\0" as *const u8 as *const i8, libpath.as_mut_ptr());
    printf(
        b"printer=\"%s\"\n\0" as *const u8 as *const i8,
        if !printer.is_null() { printer } else { b"\0" as *const u8 as *const i8 },
    );
    printf(
        b"queue_param=\"%s\"\n\0" as *const u8 as *const i8,
        queue_param.as_mut_ptr(),
    );
    printf(b"verbose=%d\n\0" as *const u8 as *const i8, verbose);
    printf(b"num_copies=%d\n\0" as *const u8 as *const i8, num_copies);
    printf(
        b"title=\"%s\"\n\0" as *const u8 as *const i8,
        if !title.is_null() { title } else { b"\0" as *const u8 as *const i8 },
    );
    printf(b"columns=%d\n\0" as *const u8 as *const i8, num_columns);
    printf(
        b"line_numbers=#%c\n\0" as *const u8 as *const i8,
        if line_numbers != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"mail=#%c\n\0" as *const u8 as *const i8,
        if mail != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"quiet=#%c\n\0" as *const u8 as *const i8,
        if quiet != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"landscape=#%c\n\0" as *const u8 as *const i8,
        if landscape != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(b"header=\0" as *const u8 as *const i8);
    match header as u32 {
        0 => {
            printf(b"none\0" as *const u8 as *const i8);
        }
        1 => {
            printf(b"simple\0" as *const u8 as *const i8);
        }
        2 => {
            printf(b"fancy (%s)\0" as *const u8 as *const i8, fancy_header_name);
        }
        _ => {}
    }
    printf(b"\n\0" as *const u8 as *const i8);
    printf(
        b"page_header=\"%s\"\n\0" as *const u8 as *const i8,
        if !page_header.is_null() {
            page_header
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    printf(
        b"font: name=%s size=%g/%gpt\n\0" as *const u8 as *const i8,
        Fname,
        Fpt.w,
        Fpt.h,
    );
    printf(
        b"header font: name=%s size=%g/%gpt\n\0" as *const u8 as *const i8,
        HFname,
        HFpt.w,
        HFpt.h,
    );
    printf(
        b"output_file=%s\n\0" as *const u8 as *const i8,
        if output_file.is_null() {
            b"none\0" as *const u8 as *const i8
        } else if output_file == 1 as i32 as *mut i8 {
            b"stdout\0" as *const u8 as *const i8
        } else {
            output_file
        },
    );
    printf(
        b"media=%s (w=%d, h=%d, llx=%d, lly=%d, urx=%d, ury=%d)\n\0" as *const u8
            as *const i8,
        (*media).name,
        (*media).w,
        (*media).h,
        (*media).llx,
        (*media).lly,
        (*media).urx,
        (*media).ury,
    );
    printf(b"encoding=%s\n\0" as *const u8 as *const i8, encoding_name);
    printf(
        b"interpret_formfeed=#%c\n\0" as *const u8 as *const i8,
        if interpret_formfeed != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"pass_through=#%c\n\0" as *const u8 as *const i8,
        if pass_through != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"spooler_command=\"%s\"\n\0" as *const u8 as *const i8,
        spooler_command.as_mut_ptr(),
    );
    printf(
        b"special_escapes=#%c\n\0" as *const u8 as *const i8,
        if special_escapes != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(b"tabsize=%d\n\0" as *const u8 as *const i8, tabsize);
    printf(b"baselineskip=%g\n\0" as *const u8 as *const i8, baselineskip);
    printf(b"statusdict: \0" as *const u8 as *const i8);
    i = strhash_get_first(
        statusdict,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b"%s %s \0" as *const u8 as *const i8, cp2, cp);
        i = strhash_get_next(
            statusdict,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
    }
    printf(b"\n\0" as *const u8 as *const i8);
    printf(b"setpagedevice: << \0" as *const u8 as *const i8);
    i = strhash_get_first(
        pagedevice,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b"/%s %s \0" as *const u8 as *const i8, cp, cp2);
        i = strhash_get_next(
            pagedevice,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
    }
    printf(b">>\n\0" as *const u8 as *const i8);
    printf(
        b"nl=%c\n\0" as *const u8 as *const i8,
        if nl == '\n' as i32 { 'n' as i32 } else { 'r' as i32 },
    );
    printf(
        b"AFM path=%s\n\0" as *const u8 as *const i8,
        if !afm_path.is_null() {
            afm_path
        } else {
            b"(default)\0" as *const u8 as *const i8
        },
    );
    printf(
        b"underlay=(%s)\n\0" as *const u8 as *const i8,
        if !underlay.is_null() { underlay } else { b"\0" as *const u8 as *const i8 },
    );
    printf(b"ul_gray=%g\n\0" as *const u8 as *const i8, ul_gray);
    printf(
        b"ul_font=%s %g/%gpt\n\0" as *const u8 as *const i8,
        ul_font,
        ul_ptsize.w,
        ul_ptsize.h,
    );
    printf(
        b"ul_position=%s (%g, %g)\n\0" as *const u8 as *const i8,
        ul_position,
        ul_x,
        ul_y,
    );
    printf(b"ul_angle=%g\n\0" as *const u8 as *const i8, ul_angle);
    printf(b"download-fonts:\0" as *const u8 as *const i8);
    i = strhash_get_first(
        download_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b" %s\0" as *const u8 as *const i8, cp);
        i = strhash_get_next(
            download_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut i8 as *mut *mut libc::c_void,
        );
    }
    printf(b"\n\0" as *const u8 as *const i8);
}
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const i8,
            b"Usage: %s [OPTION]... [FILE]...\nMandatory arguments to long options are mandatory for short options too.\n  -#                         an alias for option -n, --copies\n  -1                         same as --columns=1\n  -2                         same as --columns=2\n      --columns=NUM          specify the number of columns per page\n  -a, --pages=PAGES          specify which pages are printed\n  -A, --file-align=ALIGN     align separate input files to ALIGN\n  -b, --header=HEADER        set page header\n  -B, --no-header            no page headers\n  -c, --truncate-lines       cut long lines (default is to wrap)\n  -C, --line-numbers[=START]\n                             precede each line with its line number\n  -d                         an alias for option --printer\n  -D, --setpagedevice=KEY[:VALUE]\n                             pass a page device definition to output\n  -e, --escapes[=CHAR]       enable special escape interpretation\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
        program,
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  -E, --pretty-print[=LANG]  pretty-print source code\n\0" as *const u8
                as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  -f, --font=NAME            use font NAME for body text\n  -F, --header-font=NAME     use font NAME for header texts\n  -g, --print-anyway         nothing (compatibility option)\n  -G                         same as --fancy-header\n      --fancy-header[=NAME]  select fancy page header\n  -h, --no-job-header        suppress the job header page\n  -H, --highlight-bars=NUM   specify how high highlight bars are\n  -i, --indent=NUM           set line indent to NUM characters\n  -I, --filter=CMD           read input files through input filter CMD\n  -j, --borders              print borders around columns\n  -J,                        an alias for option --title\n  -k, --page-prefeed         enable page prefeed\n  -K, --no-page-prefeed      disable page prefeed\n  -l, --lineprinter          simulate lineprinter, this is an alias for:\n                               --lines-per-page=66, --no-header, --portrait,\n                               --columns=1\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  -L, --lines-per-page=NUM   specify how many lines are printed on each page\n  -m, --mail                 send mail upon completion\n  -M, --media=NAME           use output media NAME\n  -n, --copies=NUM           print NUM copies of each page\n  -N, --newline=NL           select the newline character.  Possible\n                             values for NL are: n (`\\n') and r (`\\r').\n  -o                         an alias for option --output\n  -O, --missing-characters   list missing characters\n  -p, --output=FILE          leave output to file FILE.  If FILE is `-',\n                             leave output to stdout.\n  -P, --printer=NAME         print output to printer NAME\n  -q, --quiet, --silent      be really quiet\n  -r, --landscape            print in landscape mode\n  -R, --portrait             print in portrait mode\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  -s, --baselineskip=NUM     set baselineskip to NUM\n  -S, --statusdict=KEY[:VALUE]\n                             pass a statusdict definition to the output\n  -t, --title=TITLE          set banner page's job title to TITLE.  Option\n                             sets also the name of the input file stdin.\n  -T, --tabsize=NUM          set tabulator size to NUM\n  -u, --underlay[=TEXT]      print TEXT under every page\n  -U, --nup=NUM              print NUM logical pages on each output page\n  -v, --verbose              tell what we are doing\n  -V, --version              print version number\n  -W, --language=LANG        set output language to LANG\n  -X, --encoding=NAME        use input encoding NAME\n  -z, --no-formfeed          do not interpret form feed characters\n  -Z, --pass-through         pass through PostScript and PCL files\n                             without any modifications\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"Long-only options:\n  --color[=COLOR]            set output color model to COLOR\n  --download-font=NAME       download font NAME\n  --filter-stdin=NAME        specify how stdin is shown to the input filter\n  --h-column-height=HEIGHT   set the horizontal column height to HEIGHT\n  --help                     print this help and exit\n  --help-pretty-print        describe all supported --pretty-print languages\n                             and file formats\n  --highlight-bar-gray=NUM   print highlight bars with gray NUM (0 - 1)\n  --list-media               list names of all known media\n  --list-options             list all options and their values\n  --margins=LEFT:RIGHT:TOP:BOTTOM\n                             adjust page marginals\n  --mark-wrapped-lines[STYLE]\n                             mark wrapped lines in the output with STYLE\n  --non-printable-format=FMT specify how non-printable chars are printed\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  --nup-xpad=NUM             set the page x-padding of N-up printing to NUM\n  --nup-ypad=NUM             set the page y-padding of N-up printing to NUM\n  --page-label-format=FMT    set page label format to FMT\n  --ps-level=LEVEL           set the PostScript language level that enscript\n                             should use\n  --printer-options=OPTIONS  pass extra options to the printer command\n  --rotate-even-pages        rotate even-numbered pages 180 degrees\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"  --slice=NUM                print vertical slice NUM\n  --toc                      print table of contents\n  --ul-angle=ANGLE           set underlay text's angle to ANGLE\n  --ul-font=NAME             print underlays with font NAME\n  --ul-gray=NUM              print underlays with gray value NUM\n  --ul-position=POS          set underlay's starting position to POS\n  --ul-style=STYLE           print underlays with style STYLE\n  --word-wrap                wrap long lines from word boundaries\n\0"
                as *const u8 as *const i8,
            5 as i32,
        ),
    );
    printf(
        dcgettext(
            0 as *const i8,
            b"\nReport bugs to mtr@iki.fi.\n\0" as *const u8 as *const i8,
            5 as i32,
        ),
    );
}
unsafe extern "C" fn version() {
    printf(
        b"%s\nCopyright (C) 1998 Markku Rossi.\nGNU enscript comes with NO WARRANTY, to the extent permitted by law.\nYou may redistribute copies of GNU enscript under the terms of the GNU\nGeneral Public License.  For more information about these matters, see\nthe files named COPYING.\n\0"
            as *const u8 as *const i8,
        version_string.as_mut_ptr(),
    );
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}