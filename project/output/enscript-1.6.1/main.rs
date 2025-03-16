#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type afm_handle_st;
    pub type stringhash_st;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn getuid() -> __uid_t;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    fn strtod(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn printer_close(context: *mut libc::c_void);
    fn printer_open(
        cmd: *mut libc::c_char,
        options: *mut libc::c_char,
        queue_param_0: *mut libc::c_char,
        printer_name: *mut libc::c_char,
        context_return: *mut *mut libc::c_void,
    ) -> *mut FILE;
    fn parse_key_value_pair(set: StringHashPtr, kv: *mut libc::c_char);
    fn escape_string(string: *mut libc::c_char) -> *mut libc::c_char;
    fn read_font_info();
    fn parse_font_spec(
        spec: *mut libc::c_char,
        name_return: *mut *mut libc::c_char,
        size_return: *mut FontPoint,
    ) -> libc::c_int;
    fn parse_float(
        string: *mut libc::c_char,
        units: libc::c_int,
        horizontal: libc::c_int,
    ) -> libc::c_double;
    fn file_existsp(name: *mut libc::c_char, suffix: *mut libc::c_char) -> libc::c_int;
    fn do_list_missing_characters(array: *mut libc::c_int);
    fn process_file(fname: *mut libc::c_char, fp: *mut InputStream);
    fn is_close(is: *mut InputStream);
    fn is_open(
        is: *mut InputStream,
        fp: *mut FILE,
        fname: *mut libc::c_char,
        input_filter_0: *mut libc::c_char,
    ) -> libc::c_int;
    fn dump_ps_trailer();
    fn read_config(path: *mut libc::c_char, name: *mut libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn tmpnam(__s: *mut libc::c_char) -> *mut libc::c_char;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn asctime(__tp: *const tm) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn afm_error_to_string(error: AFMError, buf: *mut libc::c_char);
    fn afm_create(
        path: *const libc::c_char,
        verbose_level: libc::c_uint,
        handle_return: *mut AFMHandle,
    ) -> AFMError;
    fn strhash_init() -> StringHashPtr;
    fn strhash_put(
        hash: StringHashPtr,
        key: *mut libc::c_char,
        keylen: libc::c_int,
        data: *mut libc::c_void,
        old_data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get_first(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn strhash_get_next(
        hash: StringHashPtr,
        key_return: *mut *mut libc::c_char,
        keylen_return: *mut libc::c_int,
        data_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    fn xcalloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn xstrdup(_: *mut libc::c_char) -> *mut libc::c_char;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn getopt_long(
        argc: libc::c_int,
        argv: *const *mut libc::c_char,
        shortopts: *const libc::c_char,
        longopts: *const option,
        longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
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
    pub __pad0: libc::c_int,
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
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut libc::c_char,
    pub pw_passwd: *mut libc::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut libc::c_char,
    pub pw_dir: *mut libc::c_char,
    pub pw_shell: *mut libc::c_char,
}
pub type AFMError = libc::c_uint;
pub type AFMHandle = *mut afm_handle_st;
pub type StringHashPtr = *mut stringhash_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct media_entry_st {
    pub next: *mut media_entry_st,
    pub name: *mut libc::c_char,
    pub w: libc::c_int,
    pub h: libc::c_int,
    pub llx: libc::c_int,
    pub lly: libc::c_int,
    pub urx: libc::c_int,
    pub ury: libc::c_int,
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
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            HeaderType::HDR_NONE => 0,
            HeaderType::HDR_SIMPLE => 1,
            HeaderType::HDR_FANCY => 2,
        }
    }
}

pub const HDR_FANCY: HeaderType = 2;
pub const HDR_SIMPLE: HeaderType = 1;
pub const HDR_NONE: HeaderType = 0;
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
    fn to_libc_c_uint(self) -> libc::c_uint {
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
}

pub const ENC_PS: InputEncoding = 14;
pub const ENC_KOI8: InputEncoding = 13;
pub const ENC_HP8: InputEncoding = 12;
pub const ENC_VMS: InputEncoding = 11;
pub const ENC_MAC: InputEncoding = 10;
pub const ENC_IBMPC: InputEncoding = 9;
pub const ENC_ASCII_DKNO: InputEncoding = 8;
pub const ENC_ASCII_FISE: InputEncoding = 7;
pub const ENC_ASCII: InputEncoding = 6;
pub const ENC_ISO_8859_7: InputEncoding = 5;
pub const ENC_ISO_8859_5: InputEncoding = 4;
pub const ENC_ISO_8859_4: InputEncoding = 3;
pub const ENC_ISO_8859_3: InputEncoding = 2;
pub const ENC_ISO_8859_2: InputEncoding = 1;
pub const ENC_ISO_8859_1: InputEncoding = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum PageLabelFormat {
    LABEL_SHORT,
    LABEL_LONG,
}
impl PageLabelFormat {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            PageLabelFormat::LABEL_SHORT => 0,
            PageLabelFormat::LABEL_LONG => 1,
        }
    }
}

pub const LABEL_LONG: PageLabelFormat = 1;
pub const LABEL_SHORT: PageLabelFormat = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum MarkWrappedLinesStyle {
    MWLS_NONE = 0,
    MWLS_PLUS = 1,
    MWLS_BOX = 2,
    MWLS_ARROW = 3,
}
impl MarkWrappedLinesStyle {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            MarkWrappedLinesStyle::MWLS_NONE => 0,
            MarkWrappedLinesStyle::MWLS_PLUS => 1,
            MarkWrappedLinesStyle::MWLS_BOX => 2,
            MarkWrappedLinesStyle::MWLS_ARROW => 3,
        }
    }
}

pub const MWLS_ARROW: MarkWrappedLinesStyle = 3;
pub const MWLS_BOX: MarkWrappedLinesStyle = 2;
pub const MWLS_PLUS: MarkWrappedLinesStyle = 1;
pub const MWLS_NONE: MarkWrappedLinesStyle = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum NonPrintableFormat {
    NPF_SPACE,
    NPF_QUESTIONMARK,
    NPF_CARET,
    NPF_OCTAL,
}
impl NonPrintableFormat {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            NonPrintableFormat::NPF_SPACE => 0,
            NonPrintableFormat::NPF_QUESTIONMARK => 1,
            NonPrintableFormat::NPF_CARET => 2,
            NonPrintableFormat::NPF_OCTAL => 3,
        }
    }
}

pub const NPF_OCTAL: NonPrintableFormat = 3;
pub const NPF_CARET: NonPrintableFormat = 2;
pub const NPF_QUESTIONMARK: NonPrintableFormat = 1;
pub const NPF_SPACE: NonPrintableFormat = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum FormFeedType {
    FORMFEED_COLUMN,
    FORMFEED_PAGE,
    FORMFEED_HCOLUMN,
}
impl FormFeedType {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            FormFeedType::FORMFEED_COLUMN => 0,
            FormFeedType::FORMFEED_PAGE => 1,
            FormFeedType::FORMFEED_HCOLUMN => 2,
        }
    }
}

pub const FORMFEED_HCOLUMN: FormFeedType = 2;
pub const FORMFEED_PAGE: FormFeedType = 1;
pub const FORMFEED_COLUMN: FormFeedType = 0;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum LineEndType {
    LE_TRUNCATE,
    LE_CHAR_WRAP,
    LE_WORD_WRAP,
}
impl LineEndType {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            LineEndType::LE_TRUNCATE => 0,
            LineEndType::LE_CHAR_WRAP => 1,
            LineEndType::LE_WORD_WRAP => 2,
        }
    }
}

pub const LE_WORD_WRAP: LineEndType = 2;
pub const LE_CHAR_WRAP: LineEndType = 1;
pub const LE_TRUNCATE: LineEndType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct input_stream_st {
    pub is_pipe: libc::c_int,
    pub fp: *mut FILE,
    pub buf: [libc::c_uchar; 4096],
    pub data_in_buf: libc::c_uint,
    pub bufpos: libc::c_uint,
    pub nreads: libc::c_uint,
    pub unget_ch: *mut libc::c_uchar,
    pub unget_pos: libc::c_uint,
    pub unget_alloc: libc::c_uint,
}
pub type InputStream = input_stream_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct page_range_st {
    pub next: *mut page_range_st,
    pub odd: libc::c_int,
    pub even: libc::c_int,
    pub start: libc::c_uint,
    pub end: libc::c_uint,
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
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub names: [*mut libc::c_char; 3],
    pub encoding: InputEncoding,
    pub nl: libc::c_int,
    pub bs: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[no_mangle]
pub static mut program: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut ofp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut printer_context: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut version_string: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ps_version_string: [libc::c_char; 20] = [0; 20];
#[no_mangle]
pub static mut date_string: [libc::c_char; 256] = [0; 256];
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
    tm_zone: 0 as *const libc::c_char,
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
    tm_zone: 0 as *const libc::c_char,
};
#[no_mangle]
pub static mut passwd: *mut passwd = 0 as *const passwd as *mut passwd;
#[no_mangle]
pub static mut enscript_library: *mut libc::c_char = b"/usr/local/share/enscript\0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut libpath: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut afm_path: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut afm_path_buffer: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut media_names: *mut MediaEntry = 0 as *const MediaEntry as *mut MediaEntry;
#[no_mangle]
pub static mut media: *mut MediaEntry = 0 as *const MediaEntry as *mut MediaEntry;
#[no_mangle]
pub static mut bs: libc::c_int = 8 as libc::c_int;
#[no_mangle]
pub static mut total_pages: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut num_truncated_lines: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut num_missing_chars: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut missing_chars: [libc::c_int; 256] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
pub static mut num_non_printable_chars: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut non_printable_chars: [libc::c_int; 256] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
pub static mut d_page_w: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_page_h: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_header_w: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_header_h: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_footer_h: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_output_w: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_output_h: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut d_output_x_margin: libc::c_int = 5 as libc::c_int;
#[no_mangle]
pub static mut d_output_y_margin: libc::c_int = 2 as libc::c_int;
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
pub static mut num_columns: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut page_ranges: *mut PageRange = 0 as *const PageRange as *mut PageRange;
#[no_mangle]
pub static mut file_align: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut page_header: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut line_end: LineEndType = LE_CHAR_WRAP;
#[no_mangle]
pub static mut line_numbers: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut start_line_number: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut printer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut printer_buf: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut special_escapes: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut escape_char: libc::c_int = '\0' as i32;
#[no_mangle]
pub static mut default_escape_char: libc::c_int = 0;
#[no_mangle]
pub static mut pretty_print: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pp_start_state: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut pp_filter: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub static mut Fname: *mut libc::c_char = b"Courier\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
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
pub static mut default_Fname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut user_body_font_defined: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut font_widths: [libc::c_double; 256] = [0.; 256];
#[no_mangle]
pub static mut font_ctype: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut font_is_fixed: libc::c_int = 0;
#[no_mangle]
pub static mut font_bbox_lly: libc::c_double = 0.;
#[no_mangle]
pub static mut HFname: *mut libc::c_char = b"Courier-Bold\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut HFpt: FontPoint = {
    let mut init = font_point_st {
        w: 10.0f64,
        h: 10.0f64,
    };
    init
};
#[no_mangle]
pub static mut header: HeaderType = HDR_SIMPLE;
#[no_mangle]
pub static mut fancy_header_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut fancy_header_default: [libc::c_char; 256] = [0; 256];
static mut no_job_header: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut highlight_bars: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut line_indent: libc::c_double = 0.0f64;
#[no_mangle]
pub static mut line_indent_spec: *mut libc::c_char = b"0\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut input_filter: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut borders: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut page_prefeed: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut lines_per_page: libc::c_uint = -(1 as libc::c_int) as libc::c_uint;
#[no_mangle]
pub static mut mail: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut media_name: *mut libc::c_char = b"A4\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut media_name_buffer: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut num_copies: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut nl: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut output_file: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut list_missing_characters: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut quiet: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut landscape: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut baselineskip: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut title: *mut libc::c_char = b"Enscript Output\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut title_given: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut tabsize: libc::c_int = 8 as libc::c_int;
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
pub static mut ul_font: *mut libc::c_char = b"Times-Roman\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut underlay: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut ul_position_buf: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ul_position: *mut libc::c_char = b"+0-0\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ul_x: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_y: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_angle: libc::c_double = 0.;
#[no_mangle]
pub static mut ul_style: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut ul_style_str: *mut libc::c_char = b"outline\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut ul_style_str_buf: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut ul_position_p: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut ul_angle_p: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut nup: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut nup_exp: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut nup_rows: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut nup_columns: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut nup_landscape: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut nup_width: libc::c_uint = 0;
#[no_mangle]
pub static mut nup_height: libc::c_uint = 0;
#[no_mangle]
pub static mut nup_scale: libc::c_double = 0.;
#[no_mangle]
pub static mut verbose: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut output_language: *mut libc::c_char = b"PostScript\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut output_language_pass_through: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut encoding: InputEncoding = ENC_ISO_8859_1;
#[no_mangle]
pub static mut encoding_name: *mut libc::c_char = b"88591\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut encoding_name_buffer: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut interpret_formfeed: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut pass_through: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut input_filter_stdin: *mut libc::c_char = b"\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut horizontal_column_height: libc::c_double = 283465.0f64;
#[no_mangle]
pub static mut help_pretty_print: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut highlight_bar_gray: libc::c_double = 0.97f64;
#[no_mangle]
pub static mut list_media: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut list_options: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut margins_spec: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut mark_wrapped_lines_style_name: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
pub static mut mark_wrapped_lines_style: MarkWrappedLinesStyle = MWLS_NONE;
#[no_mangle]
pub static mut npf_name: *mut libc::c_char = b"octal\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut npf_name_buf: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut non_printable_format: NonPrintableFormat = NPF_OCTAL;
#[no_mangle]
pub static mut nup_xpad: libc::c_uint = 10 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut nup_ypad: libc::c_uint = 10 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut page_label_format: *mut libc::c_char = b"short\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut page_label_format_buf: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut page_label: PageLabelFormat = LABEL_SHORT;
#[no_mangle]
pub static mut pslevel: libc::c_uint = 2 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut printer_options: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut rotate_even_pages: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut slicing: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut slice: libc::c_uint = 1 as libc::c_int as libc::c_uint;
#[no_mangle]
pub static mut toc: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut toc_fname: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub static mut toc_fp: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut toc_fmt_string: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub static mut accept_composites: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut append_ctrl_D: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut clean_7bit: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut formfeed_type: FormFeedType = FORMFEED_COLUMN;
#[no_mangle]
pub static mut generate_PageSize: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut no_job_header_switch: [libc::c_char; 16] = [0; 16];
#[no_mangle]
pub static mut output_first_line: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut queue_param: [libc::c_char; 16] = [0; 16];
#[no_mangle]
pub static mut spooler_command: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut states_color_model: [libc::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut states_config_file: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut states_highlight_level: [libc::c_char; 50] = [0; 50];
#[no_mangle]
pub static mut states_path: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut line_highlight_gray: libc::c_double = 1.0f64;
#[no_mangle]
pub static mut bggray: libc::c_double = 1.0f64;
static mut long_options: [option; 73] = unsafe {
    [
        {
            let mut init = option {
                name: b"columns\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"pages\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"file-align\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'A' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"header\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-header\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'B' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"truncate-lines\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"line-numbers\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'C' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"printer\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'd' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"setpagedevice\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'D' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"escapes\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'e' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pretty-print\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'E' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"font\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"header-font\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'F' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"print-anyway\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'g' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"fancy-header\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'G' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-job-header\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"highlight-bars\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'H' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"indent\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'i' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"filter\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'I' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"borders\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'j' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"page-prefeed\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'k' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-page-prefeed\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'K' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lineprinter\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'l' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"lines-per-page\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'L' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"mail\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'm' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"media\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"copies\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'n' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"newline\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'N' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"output\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"missing-characters\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'O' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"quiet\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"silent\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'q' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"landscape\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'r' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"portrait\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'R' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"baselineskip\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"statusdict\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"title\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"tabsize\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'T' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"underlay\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'u' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'U' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"version\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'V' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"language\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'W' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"encoding\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'X' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"no-formfeed\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"pass-through\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 'Z' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"color\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 142 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"download-font\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 131 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"filter-stdin\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 138 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"h-column-height\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 148 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"help\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 135 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"help-pretty-print\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 141 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"highlight-bar-gray\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 136 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"list-media\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &list_media as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"list-options\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &list_options as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"margins\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 144 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"mark-wrapped-lines\0" as *const u8 as *const libc::c_char,
                has_arg: 2 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 143 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"non-printable-format\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 134 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup-xpad\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 145 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"nup-ypad\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 146 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"page-label-format\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 130 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ps-level\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 149 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"printer-options\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 139 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"rotate-even-pages\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 150 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"slice\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 140 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"toc\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: &toc as *const libc::c_int as *mut libc::c_int,
                val: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"word-wrap\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 147 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-angle\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 132 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-font\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 128 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-gray\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 129 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-position\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 133 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: b"ul-style\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 137 as libc::c_int,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *const libc::c_int as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ]
};
static mut encodings: [C2RustUnnamed_0; 17] = [
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88591\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"latin1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_1,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88592\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"latin2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_2,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88593\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"latin3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_3,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88594\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"latin4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_4,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88595\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"cyrillic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_5,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"88597\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"greek\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_7,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ascii\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ASCII,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"asciifise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"asciifi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"asciise\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ASCII_FISE,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"asciidkno\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"asciidk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"asciino\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ASCII_DKNO,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ibmpc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"pc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"dos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_IBMPC,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"mac\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_MAC,
            nl: '\r' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"vms\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_VMS,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"hp8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_HP8,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"koi8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_KOI8,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"ps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"PS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_PS,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                b"pslatin1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"ISOLatin1Encoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_1,
            nl: '\n' as i32,
            bs: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_0 {
            names: [
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
                0 as *const libc::c_char as *mut libc::c_char,
            ],
            encoding: ENC_ISO_8859_1,
            nl: 0 as libc::c_int,
            bs: 0 as libc::c_int,
        };
        init
    },
];
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut is: InputStream = InputStream {
        is_pipe: 0,
        fp: 0 as *mut FILE,
        buf: [0; 4096],
        data_in_buf: 0,
        bufpos: 0,
        nreads: 0,
        unget_ch: 0 as *mut libc::c_uchar,
        unget_pos: 0,
        unget_alloc: 0,
    };
    let mut tim: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut ui: libc::c_uint = 0;
    let mut mentry: *mut MediaEntry = 0 as *mut MediaEntry;
    let mut afm_error: AFMError = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0 as libc::c_int;
    program = strrchr(*argv.offset(0 as libc::c_int as isize), '/' as i32);
    if program.is_null() {
        program = *argv.offset(0 as libc::c_int as isize);
    } else {
        program = program.offset(1);
        program;
    }
    let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
    *fresh0 = program;
    sprintf(
        version_string.as_mut_ptr(),
        b"GNU %s %s\0" as *const u8 as *const libc::c_char,
        b"enscript\0" as *const u8 as *const libc::c_char,
        b"1.6.1\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        ps_version_string.as_mut_ptr(),
        b"1.6.1\0" as *const u8 as *const libc::c_char,
    );
    cp = strrchr(ps_version_string.as_mut_ptr(), '.' as i32);
    *cp = ' ' as i32 as libc::c_char;
    toc_fmt_string = dcgettext(
        0 as *const libc::c_char,
        b"$3v $-40N $3% pages $4L lines  $E $C\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int,
    );
    setlocale(5 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"enscript\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"enscript\0" as *const u8 as *const libc::c_char);
    tim = time(0 as *mut time_t);
    tm = localtime(&mut tim);
    memcpy(
        &mut run_tm as *mut tm as *mut libc::c_void,
        tm as *const libc::c_void,
        ::core::mem::size_of::<tm>() as libc::c_ulong,
    );
    sprintf(
        date_string.as_mut_ptr(),
        b"%s\0" as *const u8 as *const libc::c_char,
        asctime(&mut run_tm),
    );
    i = strlen(date_string.as_mut_ptr()) as libc::c_int;
    date_string[(i - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    passwd = getpwuid(getuid());
    if passwd.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't get passwd entry for uid=%d: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            getuid(),
            strerror(*__errno_location()),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    strcpy(spooler_command.as_mut_ptr(), b"lpr\0" as *const u8 as *const libc::c_char);
    strcpy(queue_param.as_mut_ptr(), b"-P\0" as *const u8 as *const libc::c_char);
    strcpy(
        no_job_header_switch.as_mut_ptr(),
        b"-h\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        fancy_header_default.as_mut_ptr(),
        b"enscript\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        output_first_line.as_mut_ptr(),
        b"%!PS-Adobe-3.0\0" as *const u8 as *const libc::c_char,
    );
    cp = getenv(b"ENSCRIPT_LIBRARY\0" as *const u8 as *const libc::c_char);
    if !cp.is_null() {
        enscript_library = cp;
    }
    strcpy(
        states_color_model.as_mut_ptr(),
        b"blackwhite\0" as *const u8 as *const libc::c_char,
    );
    sprintf(
        states_config_file.as_mut_ptr(),
        b"%s/enscript.st\0" as *const u8 as *const libc::c_char,
        enscript_library,
    );
    strcpy(
        states_highlight_level.as_mut_ptr(),
        b"heavy\0" as *const u8 as *const libc::c_char,
    );
    strcpy(states_path.as_mut_ptr(), b"states\0" as *const u8 as *const libc::c_char);
    sprintf(
        libpath.as_mut_ptr(),
        b"%s%c%s/.enscript\0" as *const u8 as *const libc::c_char,
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
        b"/usr/local/etc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"enscript.cfg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0
    {
        let mut saved_errno: libc::c_int = *__errno_location();
        if read_config(
            enscript_library,
            b"enscript.cfg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
        {
            if read_config(
                b"../lib\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"enscript.cfg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) == 0
            {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"couldn't open config file \"%s/%s\": %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    enscript_library,
                    b"enscript.cfg\0" as *const u8 as *const libc::c_char,
                    strerror(saved_errno),
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            strcat(
                libpath.as_mut_ptr(),
                b":../lib\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    read_config(
        b"/usr/local/etc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"enscriptsite.cfg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    read_config(
        (*passwd).pw_dir,
        b".enscriptrc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    handle_env_options(
        b"ENSCRIPT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    handle_env_options(
        b"GENSCRIPT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    handle_options(argc, argv);
    default_escape_char = escape_char;
    found = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while found == 0
        && !(encodings[i as usize].names[0 as libc::c_int as usize]).is_null()
    {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if !(encodings[i as usize].names[j as usize]).is_null()
                && strcmp(encodings[i as usize].names[j as usize], encoding_name)
                    == 0 as libc::c_int
            {
                encoding = encodings[i as usize].encoding;
                encoding_name = encodings[i as usize].names[0 as libc::c_int as usize];
                if nl < 0 as libc::c_int {
                    nl = encodings[i as usize].nl;
                }
                bs = encodings[i as usize].bs;
                found = 1 as libc::c_int;
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
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"unknown encoding: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            encoding_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if user_body_font_defined == 0 && landscape != 0 && num_columns > 1 as libc::c_int {
        Fpt.h = 7.0f64;
        Fpt.w = Fpt.h;
    }
    afm_cache = strhash_init();
    afm_info_cache = strhash_init();
    afm_error = afm_create(afm_path, verbose as libc::c_uint, &mut afm);
    if afm_error != 0 as libc::c_int as libc::c_uint {
        let mut buf: [libc::c_char; 256] = [0; 256];
        afm_error_to_string(afm_error, buf.as_mut_ptr());
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"couldn't open AFM library: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            buf.as_mut_ptr(),
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    default_Fpt.w = Fpt.w;
    default_Fpt.h = Fpt.h;
    default_Fname = Fname;
    strhash_put(
        res_fonts,
        Fname,
        (strlen(Fname)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        res_fonts,
        HFname,
        (strlen(HFname)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        download_fonts,
        Fname,
        (strlen(Fname)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    strhash_put(
        download_fonts,
        HFname,
        (strlen(HFname)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        0 as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
    read_font_info();
    line_indent = parse_float(line_indent_spec, 1 as libc::c_int, 1 as libc::c_int);
    if list_media != 0 {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"known media:\nname             width\theight\tllx\tlly\turx\tury\n------------------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        mentry = media_names;
        while !mentry.is_null() {
            printf(
                b"%-16s %d\t%d\t%d\t%d\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
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
        exit(0 as libc::c_int);
    }
    mentry = media_names;
    while !mentry.is_null() {
        if strcmp(media_name, (*mentry).name) == 0 as libc::c_int {
            media = mentry;
            break;
        } else {
            mentry = (*mentry).next;
        }
    }
    if media.is_null() {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"do not know anything about media \"%s\"\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            media_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if !margins_spec.is_null() {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if *margins_spec as libc::c_int == '\0' as i32 {
                break;
            }
            if *margins_spec as libc::c_int == ':' as i32 {
                margins_spec = margins_spec.offset(1);
                margins_spec;
            } else {
                j = atoi(margins_spec);
                while *margins_spec as libc::c_int != ':' as i32
                    && *margins_spec as libc::c_int != '\0' as i32
                {
                    margins_spec = margins_spec.offset(1);
                    margins_spec;
                }
                if *margins_spec as libc::c_int == ':' as i32 {
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
        if quiet == 0 && verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"set new marginals for media `%s' (%dx%d): llx=%d, lly=%d, urx=%d, ury=%d\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
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
    if strcmp(page_label_format, b"short\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        page_label = LABEL_SHORT;
    } else if strcmp(page_label_format, b"long\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        page_label = LABEL_LONG;
    } else {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"illegal page label format \"%s\"\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            page_label_format,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if strcmp(npf_name, b"space\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        non_printable_format = NPF_SPACE;
    } else if strcmp(npf_name, b"questionmark\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        non_printable_format = NPF_QUESTIONMARK;
    } else if strcmp(npf_name, b"caret\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        non_printable_format = NPF_CARET;
    } else if strcmp(npf_name, b"octal\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        non_printable_format = NPF_OCTAL;
    } else {
        fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
        fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"illegal non-printable format \"%s\"\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            npf_name,
        );
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stderr);
        exit(1 as libc::c_int);
    }
    if mark_wrapped_lines_style_name[0 as libc::c_int as usize] != 0 {
        if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"none\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mark_wrapped_lines_style = MWLS_NONE;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"plus\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mark_wrapped_lines_style = MWLS_PLUS;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"box\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mark_wrapped_lines_style = MWLS_BOX;
        } else if strcmp(
            mark_wrapped_lines_style_name.as_mut_ptr(),
            b"arrow\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            mark_wrapped_lines_style = MWLS_ARROW;
        } else {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"illegal style for wrapped line marker: \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                mark_wrapped_lines_style_name.as_mut_ptr(),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
    }
    i = 0 as libc::c_int;
    loop {
        ui = nup >> i;
        if ui == 0 as libc::c_int as libc::c_uint {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"illegal N-up argument: %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                nup,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
        if ui & 0x1 as libc::c_int as libc::c_uint != 0 {
            if ui != 1 as libc::c_int as libc::c_uint {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"N-up argument must be power of 2: %d\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    nup,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            nup_exp = i as libc::c_uint;
            break;
        } else {
            i += 1;
            i;
        }
    }
    nup_rows = nup_exp
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_int as libc::c_uint);
    if nup_rows == 0 as libc::c_int as libc::c_uint {
        nup_rows = 1 as libc::c_int as libc::c_uint;
    }
    nup_columns = nup_exp
        .wrapping_add(1 as libc::c_int as libc::c_uint)
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_int as libc::c_uint);
    if nup_columns == 0 as libc::c_int as libc::c_uint {
        nup_columns = 1 as libc::c_int as libc::c_uint;
    }
    nup_landscape = (nup_exp & 0x1 as libc::c_int as libc::c_uint) as libc::c_int;
    if landscape != 0 {
        d_page_w = (*media).ury - (*media).lly;
        d_page_h = (*media).urx - (*media).llx;
    } else {
        d_page_w = (*media).urx - (*media).llx;
        d_page_h = (*media).ury - (*media).lly;
    }
    if nup_landscape != 0 {
        nup_width = ((*media).ury - (*media).lly) as libc::c_uint;
        nup_height = ((*media).urx - (*media).llx) as libc::c_uint;
    } else {
        nup_width = ((*media).urx - (*media).llx) as libc::c_uint;
        nup_height = ((*media).ury - (*media).lly) as libc::c_uint;
    }
    let mut w: libc::c_double = 0.;
    let mut h: libc::c_double = 0.;
    w = (nup_width as libc::c_double
        - nup_columns
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(nup_xpad) as libc::c_double) / nup_columns as libc::c_double;
    h = (nup_height as libc::c_double
        - nup_rows.wrapping_sub(1 as libc::c_int as libc::c_uint).wrapping_mul(nup_ypad)
            as libc::c_double) / nup_rows as libc::c_double;
    nup_width = w as libc::c_uint;
    nup_height = h as libc::c_uint;
    w = w / ((*media).urx - (*media).llx) as libc::c_double;
    h = h / ((*media).ury - (*media).lly) as libc::c_double;
    nup_scale = if w < h { w } else { h };
    if !underlay.is_null() {
        strhash_put(
            res_fonts,
            ul_font,
            (strlen(ul_font)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        underlay = escape_string(underlay);
    }
    ul_x = strtod(ul_position, &mut cp);
    if !(cp == ul_position) {
        if *ul_position.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            ul_x += d_page_w as libc::c_double;
        }
        ul_y = strtod(cp, &mut cp2);
        if !(cp2 == cp) {
            if *cp.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
                ul_y += d_page_h as libc::c_double;
            }
            if ul_angle_p == 0 {
                ul_angle = atan2(-d_page_h as libc::c_double, d_page_w as libc::c_double)
                    / 3.14159265f64 * 180 as libc::c_int as libc::c_double;
            }
            if strcmp(ul_style_str, b"outline\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                ul_style = 0 as libc::c_int as libc::c_uint;
            } else if strcmp(
                ul_style_str,
                b"filled\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                ul_style = 1 as libc::c_int as libc::c_uint;
            } else {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"illegal underlay style: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    ul_style_str,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            d_header_w = d_page_w;
            match header as libc::c_uint {
                0 => {
                    d_header_h = 0 as libc::c_int;
                }
                1 => {
                    d_header_h = (HFpt.h * 1.5f64) as libc::c_int;
                }
                2 => {
                    d_header_h = 36 as libc::c_int;
                }
                _ => {}
            }
            if help_pretty_print != 0 {
                sprintf(
                    pp_filter.as_mut_ptr(),
                    b"%s -f \"%s\" -s describe_languages \"%s\"\0" as *const u8
                        as *const libc::c_char,
                    states_path.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                );
                system(pp_filter.as_mut_ptr());
                exit(0 as libc::c_int);
            }
            if list_options != 0 {
                do_list_options();
                exit(1 as libc::c_int);
            }
            if output_language_pass_through != 0 {
                let mut start_state: *mut libc::c_char = 0 as *mut libc::c_char;
                if !pp_start_state.is_null() {
                    start_state = pp_start_state;
                } else if pretty_print != 0 {
                    start_state = 0 as *mut libc::c_char;
                } else {
                    start_state = b"passthrough\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                j = 0 as libc::c_int;
                i = optind;
                while i < argc {
                    j = (j as libc::c_ulong)
                        .wrapping_add(
                            (strlen(*argv.offset(i as isize)))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ) as libc::c_int as libc::c_int;
                    i += 1;
                    i;
                }
                j += 256 as libc::c_int;
                cp = xmalloc(j as size_t) as *mut libc::c_char;
                sprintf(
                    cp,
                    b"%s -f \"%s\" %s%s -Dcolormodel=%s -Dhl_level=%s -Dlanguage=%s -Dnum_input_files=%d -Ddocument_title=\"%s\" -Dtoc=%d\0"
                        as *const u8 as *const libc::c_char,
                    states_path.as_mut_ptr(),
                    states_config_file.as_mut_ptr(),
                    if !start_state.is_null() {
                        b"-s\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    if !start_state.is_null() {
                        start_state
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    states_color_model.as_mut_ptr(),
                    states_highlight_level.as_mut_ptr(),
                    output_language,
                    if optind == argc { 1 as libc::c_int } else { argc - optind },
                    title,
                    toc,
                );
                i = optind;
                while i < argc {
                    strcat(cp, b" \0" as *const u8 as *const libc::c_char);
                    strcat(cp, *argv.offset(i as isize));
                    i += 1;
                    i;
                }
                if is_open(&mut is, stdin, 0 as *mut libc::c_char, cp) != 0 {
                    open_output_file();
                    process_file(
                        b"unused\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        &mut is,
                    );
                    is_close(&mut is);
                }
            } else {
                if pretty_print != 0 {
                    sprintf(
                        pp_filter.as_mut_ptr(),
                        b"%s -f \"%s\" %s%s -Dcolormodel=%s -Dhl_level=%s -Dfont_spec=%s@%g/%g \"%%s\"\0"
                            as *const u8 as *const libc::c_char,
                        states_path.as_mut_ptr(),
                        states_config_file.as_mut_ptr(),
                        if !pp_start_state.is_null() {
                            b"-s \0" as *const u8 as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        if !pp_start_state.is_null() {
                            pp_start_state
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                        states_color_model.as_mut_ptr(),
                        states_highlight_level.as_mut_ptr(),
                        Fname,
                        Fpt.w,
                        Fpt.h,
                    );
                    input_filter = pp_filter.as_mut_ptr();
                    input_filter_stdin = b"-\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                if toc != 0 {
                    cp = tmpnam(toc_fname.as_mut_ptr());
                    if cp.is_null() {
                        fprintf(
                            stderr,
                            b"%s: \0" as *const u8 as *const libc::c_char,
                            program,
                        );
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"couldn't create toc file name: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                    toc_fp = fopen(
                        toc_fname.as_mut_ptr(),
                        b"w+b\0" as *const u8 as *const libc::c_char,
                    );
                    if toc_fp.is_null() {
                        fprintf(
                            stderr,
                            b"%s: \0" as *const u8 as *const libc::c_char,
                            program,
                        );
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"couldn't create toc file \"%s\": %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            toc_fname.as_mut_ptr(),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                    if remove(toc_fname.as_mut_ptr()) == 0 as libc::c_int {
                        toc_fname[0 as libc::c_int
                            as usize] = '\0' as i32 as libc::c_char;
                    }
                }
                if optind == argc {
                    memcpy(
                        &mut mod_tm as *mut tm as *mut libc::c_void,
                        &mut run_tm as *mut tm as *const libc::c_void,
                        ::core::mem::size_of::<tm>() as libc::c_ulong,
                    );
                    if is_open(&mut is, stdin, 0 as *mut libc::c_char, input_filter) != 0
                    {
                        open_output_file();
                        process_file(
                            (if title_given != 0 {
                                title
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char,
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
                                == 0 as libc::c_int
                            {
                                tim = stat_st.st_mtim.tv_sec;
                                tm = localtime(&mut tim);
                                memcpy(
                                    &mut mod_tm as *mut tm as *mut libc::c_void,
                                    tm as *const libc::c_void,
                                    ::core::mem::size_of::<tm>() as libc::c_ulong,
                                );
                                open_output_file();
                                process_file(*argv.offset(optind as isize), &mut is);
                            } else {
                                fprintf(
                                    stderr,
                                    b"%s: \0" as *const u8 as *const libc::c_char,
                                    program,
                                );
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"couldn't stat input file \"%s\": %s\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    *argv.offset(optind as isize),
                                    strerror(*__errno_location()),
                                );
                                fprintf(
                                    stderr,
                                    b"\n\0" as *const u8 as *const libc::c_char,
                                );
                                fflush(stderr);
                            }
                            is_close(&mut is);
                        }
                        optind += 1;
                        optind;
                    }
                }
                if toc != 0 {
                    toc = 0 as libc::c_int;
                    special_escapes = 1 as libc::c_int;
                    line_numbers = 0 as libc::c_int;
                    if fseek(toc_fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        fprintf(
                            stderr,
                            b"%s: \0" as *const u8 as *const libc::c_char,
                            program,
                        );
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"couldn't rewind toc file: %s\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            strerror(*__errno_location()),
                        );
                        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                        fflush(stderr);
                        exit(1 as libc::c_int);
                    }
                    memcpy(
                        &mut mod_tm as *mut tm as *mut libc::c_void,
                        &mut run_tm as *mut tm as *const libc::c_void,
                        ::core::mem::size_of::<tm>() as libc::c_ulong,
                    );
                    if is_open(
                        &mut is,
                        toc_fp,
                        0 as *mut libc::c_char,
                        0 as *mut libc::c_char,
                    ) != 0
                    {
                        process_file(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Table of Contents\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            &mut is,
                        );
                        is_close(&mut is);
                    }
                    fclose(toc_fp);
                    if toc_fname[0 as libc::c_int as usize] != 0 {
                        remove(toc_fname.as_mut_ptr());
                    }
                }
                dump_ps_trailer();
                if !ofp.is_null() && append_ctrl_D != 0 {
                    fprintf(ofp, b"\x04\n\0" as *const u8 as *const libc::c_char);
                }
            }
            close_output_file();
            if ofp.is_null() {
                if quiet == 0 && verbose >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"no output generated\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            } else if output_language_pass_through != 0 {
                if output_file.is_null() {
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"output sent to %s\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            if !printer.is_null() {
                                printer
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"printer\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            },
                        );
                    }
                } else if quiet == 0 && verbose >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"output left in %s\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        if output_file == 1 as libc::c_int as *mut libc::c_char {
                            b"-\0" as *const u8 as *const libc::c_char
                        } else {
                            output_file
                        },
                    );
                }
            } else {
                let mut real_total_pages: libc::c_uint = 0;
                if nup > 1 as libc::c_int as libc::c_uint {
                    if total_pages > 0 as libc::c_int {
                        real_total_pages = ((total_pages - 1 as libc::c_int)
                            as libc::c_uint)
                            .wrapping_div(nup)
                            .wrapping_add(1 as libc::c_int as libc::c_uint);
                    } else {
                        real_total_pages = 0 as libc::c_int as libc::c_uint;
                    }
                } else {
                    real_total_pages = total_pages as libc::c_uint;
                }
                if quiet == 0 && verbose >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"[ %d pages * %d copy ]\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        real_total_pages,
                        num_copies,
                    );
                }
                if output_file.is_null() {
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b" sent to %s\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            if !printer.is_null() {
                                printer
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"printer\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            },
                        );
                    }
                } else if quiet == 0 && verbose >= 0 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b" left in %s\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        if output_file == 1 as libc::c_int as *mut libc::c_char {
                            b"-\0" as *const u8 as *const libc::c_char
                        } else {
                            output_file
                        },
                    );
                }
                if num_truncated_lines != 0 {
                    retval |= 2 as libc::c_int;
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%d lines were %s\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            num_truncated_lines,
                            if line_end as libc::c_uint
                                == LE_TRUNCATE as libc::c_int as libc::c_uint
                            {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"truncated\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            } else {
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"wrapped\0" as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                )
                            },
                        );
                    }
                }
                if num_missing_chars != 0 {
                    retval |= 4 as libc::c_int;
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%d characters were missing\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            num_missing_chars,
                        );
                    }
                    if list_missing_characters != 0 {
                        if quiet == 0 && verbose >= 0 as libc::c_int {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"missing character codes (decimal):\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                            );
                        }
                        do_list_missing_characters(missing_chars.as_mut_ptr());
                    }
                }
                if num_non_printable_chars != 0 {
                    retval |= 8 as libc::c_int;
                    if quiet == 0 && verbose >= 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%d non-printable characters\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            num_non_printable_chars,
                        );
                    }
                    if list_missing_characters != 0 {
                        if quiet == 0 && verbose >= 0 as libc::c_int {
                            fprintf(
                                stderr,
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"non-printable character codes (decimal):\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
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
    fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
    fprintf(
        stderr,
        dcgettext(
            0 as *const libc::c_char,
            b"malformed underlay position: %s\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        ul_position,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fflush(stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn open_output_file() {
    if !ofp.is_null() {
        return;
    }
    if output_file.is_null() {
        let mut spooler_options: [libc::c_char; 512] = [0; 512];
        spooler_options[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        if mail != 0 {
            strcat(
                spooler_options.as_mut_ptr(),
                b"-m \0" as *const u8 as *const libc::c_char,
            );
        }
        if no_job_header != 0 {
            strcat(spooler_options.as_mut_ptr(), no_job_header_switch.as_mut_ptr());
            strcat(
                spooler_options.as_mut_ptr(),
                b" \0" as *const u8 as *const libc::c_char,
            );
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
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't open printer `%s': %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                printer,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
    } else if output_file == 1 as libc::c_int as *mut libc::c_char {
        ofp = stdout;
    } else {
        ofp = fopen(output_file, b"w\0" as *const u8 as *const libc::c_char);
        if ofp.is_null() {
            fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"couldn't create output file \"%s\": %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                output_file,
                strerror(*__errno_location()),
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            fflush(stderr);
            exit(1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn close_output_file() {
    if ofp.is_null() {
        return;
    }
    if output_file.is_null() {
        printer_close(printer_context);
    } else if output_file != 1 as libc::c_int as *mut libc::c_char {
        fclose(ofp);
    }
}
unsafe extern "C" fn handle_env_options(mut var: *mut libc::c_char) {
    let mut argc: libc::c_int = 0;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    string = getenv(var);
    if string.is_null() {
        return;
    }
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"handle_env_options(): %s=\"%s\"\n\0" as *const u8 as *const libc::c_char,
            var,
            string,
        );
    }
    str = xstrdup(string);
    argc = (strlen(str))
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    argv = xcalloc(
        argc as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    argc = 0 as libc::c_int;
    let fresh1 = argc;
    argc = argc + 1;
    let ref mut fresh2 = *argv.offset(fresh1 as isize);
    *fresh2 = program;
    i = 0 as libc::c_int;
    while *str.offset(i as isize) != 0 {
        while *str.offset(i as isize) as libc::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*str.offset(i as isize) as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            i += 1;
            i;
        }
        if *str.offset(i as isize) == 0 {
            break;
        }
        if *str.offset(i as isize) as libc::c_int == '"' as i32
            || *str.offset(i as isize) as libc::c_int == '\'' as i32
        {
            let fresh3 = i;
            i = i + 1;
            let mut endch: libc::c_int = *str.offset(fresh3 as isize) as libc::c_int;
            let fresh4 = argc;
            argc = argc + 1;
            let ref mut fresh5 = *argv.offset(fresh4 as isize);
            *fresh5 = str.offset(i as isize);
            while *str.offset(i as isize) as libc::c_int != 0
                && *str.offset(i as isize) as libc::c_int != endch
            {
                i += 1;
                i;
            }
            if *str.offset(i as isize) == 0 {
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"syntax error in option string %s=\"%s\":\nmissing end of quotation: %c\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    var,
                    string,
                    endch,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
            let fresh6 = i;
            i = i + 1;
            *str.offset(fresh6 as isize) = '\0' as i32 as libc::c_char;
        } else {
            let fresh7 = argc;
            argc = argc + 1;
            let ref mut fresh8 = *argv.offset(fresh7 as isize);
            *fresh8 = str.offset(i as isize);
            while *str.offset(i as isize) as libc::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*str.offset(i as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                i += 1;
                i;
            }
            if *str.offset(i as isize) != 0 {
                let fresh9 = i;
                i = i + 1;
                *str.offset(fresh9 as isize) = '\0' as i32 as libc::c_char;
            }
        }
    }
    let ref mut fresh10 = *argv.offset(argc as isize);
    *fresh10 = 0 as *mut libc::c_char;
    if quiet == 0 && verbose >= 2 as libc::c_int {
        fprintf(
            stderr,
            b"found following options (argc=%d):\n\0" as *const u8
                as *const libc::c_char,
            argc,
        );
    }
    i = 0 as libc::c_int;
    while i < argc {
        if quiet == 0 && verbose >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"%3d = \"%s\"\n\0" as *const u8 as *const libc::c_char,
                i,
                *argv.offset(i as isize),
            );
        }
        i += 1;
        i;
    }
    handle_options(argc, argv);
    if optind != argc {
        if quiet == 0 && verbose >= 0 as libc::c_int {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"warning: didn't process following options from environment variable %s:\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                var,
            );
        }
        while optind < argc {
            if quiet == 0 && verbose >= 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"  option %d = \"%s\"\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
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
unsafe extern "C" fn handle_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut c: libc::c_int = 0;
    let mut prange: *mut PageRange = 0 as *mut PageRange;
    optind = 0 as libc::c_int;
    loop {
        let mut option_index: libc::c_int = 0 as libc::c_int;
        let mut cp: *const libc::c_char = 0 as *const libc::c_char;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"#:12a:A:b:BcC::d:D:e::E::f:F:gGhH::i:I:jJ:kKlL:mM:n:N:o:Op:P:qrRs:S:t:T:u::U:vVW:X:zZ\0"
                as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            &mut option_index,
        );
        if c == -(1 as libc::c_int) {
            break;
        }
        let mut current_block_199: u64;
        match c {
            0 => {
                cp = long_options[option_index as usize].name;
                if strcmp(cp, b"columns\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
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
                    1 as libc::c_int as size_t,
                    ::core::mem::size_of::<PageRange>() as libc::c_ulong,
                ) as *mut PageRange;
                if strcmp(optarg, b"odd\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*prange).odd = 1 as libc::c_int;
                } else if strcmp(optarg, b"even\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    (*prange).even = 1 as libc::c_int;
                } else {
                    cp = strchr(optarg, '-' as i32);
                    if !cp.is_null() {
                        if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        {
                            (*prange)
                                .end = atoi(optarg.offset(1 as libc::c_int as isize))
                                as libc::c_uint;
                        } else if *cp.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\0' as i32
                        {
                            (*prange).start = atoi(optarg) as libc::c_uint;
                            (*prange).end = -(1 as libc::c_int) as libc::c_uint;
                        } else {
                            (*prange).start = atoi(optarg) as libc::c_uint;
                            (*prange)
                                .end = atoi(cp.offset(1 as libc::c_int as isize))
                                as libc::c_uint;
                        }
                    } else {
                        (*prange).end = atoi(optarg) as libc::c_uint;
                        (*prange).start = (*prange).end;
                    }
                }
                (*prange).next = page_ranges;
                page_ranges = prange;
                current_block_199 = 17170253997621722914;
            }
            65 => {
                file_align = atoi(optarg) as libc::c_uint;
                if file_align == 0 as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"file alignment must be larger than zero\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                current_block_199 = 17170253997621722914;
            }
            98 => {
                page_header = optarg;
                current_block_199 = 17170253997621722914;
            }
            66 => {
                header = HDR_NONE;
                current_block_199 = 17170253997621722914;
            }
            99 => {
                line_end = LE_TRUNCATE;
                current_block_199 = 17170253997621722914;
            }
            67 => {
                line_numbers = 1 as libc::c_int;
                if !optarg.is_null() {
                    start_line_number = atoi(optarg) as libc::c_uint;
                }
                current_block_199 = 17170253997621722914;
            }
            100 | 80 => {
                printer = optarg;
                output_file = 0 as *mut libc::c_char;
                current_block_199 = 17170253997621722914;
            }
            68 => {
                parse_key_value_pair(pagedevice, optarg);
                current_block_199 = 17170253997621722914;
            }
            101 => {
                special_escapes = 1 as libc::c_int;
                if !optarg.is_null() {
                    if *(*__ctype_b_loc())
                        .offset(
                            *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                                as isize,
                        ) as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        escape_char = strtoul(
                            optarg,
                            0 as *mut *mut libc::c_char,
                            0 as libc::c_int,
                        ) as libc::c_int;
                    } else {
                        escape_char = *(optarg as *mut libc::c_uchar)
                            .offset(0 as libc::c_int as isize) as libc::c_int;
                    }
                }
                current_block_199 = 17170253997621722914;
            }
            69 => {
                pretty_print = 1 as libc::c_int;
                special_escapes = 1 as libc::c_int;
                escape_char = '\0' as i32;
                pp_start_state = optarg;
                current_block_199 = 17170253997621722914;
            }
            102 => {
                if parse_font_spec(optarg, &mut Fname, &mut Fpt) == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"malformed font spec: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                user_body_font_defined = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            70 => {
                if parse_font_spec(optarg, &mut HFname, &mut HFpt) == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"malformed font spec: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                current_block_199 = 17170253997621722914;
            }
            103 => {
                current_block_199 = 17170253997621722914;
            }
            71 => {
                header = HDR_FANCY;
                if !optarg.is_null() {
                    fancy_header_name = optarg;
                } else {
                    fancy_header_name = fancy_header_default.as_mut_ptr();
                }
                if file_existsp(
                    fancy_header_name,
                    b".hdr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) == 0
                {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"couldn't find header definition file \"%s.hdr\"\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        fancy_header_name,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                current_block_199 = 17170253997621722914;
            }
            104 => {
                no_job_header = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            72 => {
                if !optarg.is_null() {
                    highlight_bars = atoi(optarg) as libc::c_uint;
                } else {
                    highlight_bars = 2 as libc::c_int as libc::c_uint;
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
                borders = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            107 => {
                page_prefeed = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            75 => {
                page_prefeed = 0 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            108 => {
                lines_per_page = 66 as libc::c_int as libc::c_uint;
                header = HDR_NONE;
                current_block_199 = 17170253997621722914;
            }
            76 => {
                lines_per_page = atoi(optarg) as libc::c_uint;
                if lines_per_page <= 0 as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"must print at least one line per each page: %s\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(optind as isize),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                current_block_199 = 17170253997621722914;
            }
            109 => {
                mail = 1 as libc::c_int;
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
                if !(*optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    == 'n' as i32
                    || *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'r' as i32)
                    || *optarg.offset(1 as libc::c_int as isize) as libc::c_int
                        != '\0' as i32
                {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: illegal newline character specifier: '%s': expected 'n' or 'r'\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        program,
                        optarg,
                    );
                    current_block_199 = 15399172731369158041;
                } else {
                    if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == 'n' as i32
                    {
                        nl = '\n' as i32;
                    } else {
                        nl = '\r' as i32;
                    }
                    current_block_199 = 17170253997621722914;
                }
            }
            111 | 112 => {
                if strcmp(optarg, b"-\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    output_file = 1 as libc::c_int as *mut libc::c_char;
                } else {
                    output_file = optarg;
                }
                current_block_199 = 17170253997621722914;
            }
            79 => {
                list_missing_characters = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            113 => {
                quiet = 1 as libc::c_int;
                verbose = 0 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            114 => {
                landscape = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            82 => {
                landscape = 0 as libc::c_int;
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
                title_given = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            84 => {
                tabsize = atoi(optarg);
                if tabsize <= 0 as libc::c_int {
                    tabsize = 1 as libc::c_int;
                }
                current_block_199 = 17170253997621722914;
            }
            117 => {
                underlay = optarg;
                current_block_199 = 17170253997621722914;
            }
            85 => {
                nup = atoi(optarg) as libc::c_uint;
                current_block_199 = 17170253997621722914;
            }
            118 => {
                if !optarg.is_null() {
                    verbose = atoi(optarg);
                } else {
                    verbose += 1;
                    verbose;
                }
                quiet = 0 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            86 => {
                version();
                exit(0 as libc::c_int);
            }
            87 => {
                output_language = optarg;
                if strcmp(
                    output_language,
                    b"PostScript\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    output_language_pass_through = 1 as libc::c_int;
                }
                current_block_199 = 17170253997621722914;
            }
            88 => {
                encoding_name = optarg;
                current_block_199 = 17170253997621722914;
            }
            122 => {
                interpret_formfeed = 0 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            90 => {
                pass_through = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            128 => {
                if parse_font_spec(optarg, &mut ul_font, &mut ul_ptsize) == 0 {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"malformed font spec: %s\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        optarg,
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
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
                    (strlen(optarg)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int,
                    0 as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                );
                current_block_199 = 17170253997621722914;
            }
            132 => {
                ul_angle = atof(optarg);
                ul_angle_p = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            133 => {
                ul_position = optarg;
                ul_position_p = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            134 => {
                npf_name = optarg;
                current_block_199 = 17170253997621722914;
            }
            135 => {
                usage();
                exit(0 as libc::c_int);
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
                slicing = 1 as libc::c_int;
                slice = atoi(optarg) as libc::c_uint;
                if slice <= 0 as libc::c_int as libc::c_uint {
                    fprintf(
                        stderr,
                        b"%s: \0" as *const u8 as *const libc::c_char,
                        program,
                    );
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"slice must be greater than zero\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                    fflush(stderr);
                    exit(1 as libc::c_int);
                }
                current_block_199 = 17170253997621722914;
            }
            141 => {
                help_pretty_print = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            142 => {
                if optarg.is_null() {
                    strcpy(
                        states_color_model.as_mut_ptr(),
                        b"emacs\0" as *const u8 as *const libc::c_char,
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
                    mark_wrapped_lines_style = MWLS_BOX;
                }
                current_block_199 = 17170253997621722914;
            }
            144 => {
                margins_spec = optarg;
                current_block_199 = 17170253997621722914;
            }
            145 => {
                nup_xpad = atoi(optarg) as libc::c_uint;
                current_block_199 = 17170253997621722914;
            }
            146 => {
                nup_ypad = atoi(optarg) as libc::c_uint;
                current_block_199 = 17170253997621722914;
            }
            147 => {
                line_end = LE_WORD_WRAP;
                current_block_199 = 17170253997621722914;
            }
            148 => {
                horizontal_column_height = atof(optarg);
                formfeed_type = FORMFEED_HCOLUMN;
                current_block_199 = 17170253997621722914;
            }
            149 => {
                pslevel = atoi(optarg) as libc::c_uint;
                current_block_199 = 17170253997621722914;
            }
            150 => {
                rotate_even_pages = 1 as libc::c_int;
                current_block_199 = 17170253997621722914;
            }
            63 => {
                current_block_199 = 15399172731369158041;
            }
            _ => {
                printf(
                    b"Hey!  main() didn't handle option \"%c\" (%d)\0" as *const u8
                        as *const libc::c_char,
                    c,
                    c,
                );
                if !optarg.is_null() {
                    printf(
                        b" with arg %s\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
                fprintf(stderr, b"%s: \0" as *const u8 as *const libc::c_char, program);
                fprintf(stderr, b"This is a bug!\0" as *const u8 as *const libc::c_char);
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fflush(stderr);
                exit(1 as libc::c_int);
            }
        }
        match current_block_199 {
            17170253997621722914 => {}
            _ => {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Try `%s --help' for more information.\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    program,
                );
                exit(1 as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn do_list_options() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp2: *mut libc::c_char = 0 as *mut libc::c_char;
    printf(
        b"libpath=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        libpath.as_mut_ptr(),
    );
    printf(
        b"printer=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        if !printer.is_null() {
            printer
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"queue_param=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        queue_param.as_mut_ptr(),
    );
    printf(b"verbose=%d\n\0" as *const u8 as *const libc::c_char, verbose);
    printf(b"num_copies=%d\n\0" as *const u8 as *const libc::c_char, num_copies);
    printf(
        b"title=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        if !title.is_null() { title } else { b"\0" as *const u8 as *const libc::c_char },
    );
    printf(b"columns=%d\n\0" as *const u8 as *const libc::c_char, num_columns);
    printf(
        b"line_numbers=#%c\n\0" as *const u8 as *const libc::c_char,
        if line_numbers != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"mail=#%c\n\0" as *const u8 as *const libc::c_char,
        if mail != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"quiet=#%c\n\0" as *const u8 as *const libc::c_char,
        if quiet != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"landscape=#%c\n\0" as *const u8 as *const libc::c_char,
        if landscape != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(b"header=\0" as *const u8 as *const libc::c_char);
    match header as libc::c_uint {
        0 => {
            printf(b"none\0" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(b"simple\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(
                b"fancy (%s)\0" as *const u8 as *const libc::c_char,
                fancy_header_name,
            );
        }
        _ => {}
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"page_header=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        if !page_header.is_null() {
            page_header
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"font: name=%s size=%g/%gpt\n\0" as *const u8 as *const libc::c_char,
        Fname,
        Fpt.w,
        Fpt.h,
    );
    printf(
        b"header font: name=%s size=%g/%gpt\n\0" as *const u8 as *const libc::c_char,
        HFname,
        HFpt.w,
        HFpt.h,
    );
    printf(
        b"output_file=%s\n\0" as *const u8 as *const libc::c_char,
        if output_file.is_null() {
            b"none\0" as *const u8 as *const libc::c_char
        } else if output_file == 1 as libc::c_int as *mut libc::c_char {
            b"stdout\0" as *const u8 as *const libc::c_char
        } else {
            output_file
        },
    );
    printf(
        b"media=%s (w=%d, h=%d, llx=%d, lly=%d, urx=%d, ury=%d)\n\0" as *const u8
            as *const libc::c_char,
        (*media).name,
        (*media).w,
        (*media).h,
        (*media).llx,
        (*media).lly,
        (*media).urx,
        (*media).ury,
    );
    printf(b"encoding=%s\n\0" as *const u8 as *const libc::c_char, encoding_name);
    printf(
        b"interpret_formfeed=#%c\n\0" as *const u8 as *const libc::c_char,
        if interpret_formfeed != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"pass_through=#%c\n\0" as *const u8 as *const libc::c_char,
        if pass_through != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(
        b"spooler_command=\"%s\"\n\0" as *const u8 as *const libc::c_char,
        spooler_command.as_mut_ptr(),
    );
    printf(
        b"special_escapes=#%c\n\0" as *const u8 as *const libc::c_char,
        if special_escapes != 0 { 't' as i32 } else { 'f' as i32 },
    );
    printf(b"tabsize=%d\n\0" as *const u8 as *const libc::c_char, tabsize);
    printf(b"baselineskip=%g\n\0" as *const u8 as *const libc::c_char, baselineskip);
    printf(b"statusdict: \0" as *const u8 as *const libc::c_char);
    i = strhash_get_first(
        statusdict,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b"%s %s \0" as *const u8 as *const libc::c_char, cp2, cp);
        i = strhash_get_next(
            statusdict,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"setpagedevice: << \0" as *const u8 as *const libc::c_char);
    i = strhash_get_first(
        pagedevice,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b"/%s %s \0" as *const u8 as *const libc::c_char, cp, cp2);
        i = strhash_get_next(
            pagedevice,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
    }
    printf(b">>\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"nl=%c\n\0" as *const u8 as *const libc::c_char,
        if nl == '\n' as i32 { 'n' as i32 } else { 'r' as i32 },
    );
    printf(
        b"AFM path=%s\n\0" as *const u8 as *const libc::c_char,
        if !afm_path.is_null() {
            afm_path
        } else {
            b"(default)\0" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"underlay=(%s)\n\0" as *const u8 as *const libc::c_char,
        if !underlay.is_null() {
            underlay
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    printf(b"ul_gray=%g\n\0" as *const u8 as *const libc::c_char, ul_gray);
    printf(
        b"ul_font=%s %g/%gpt\n\0" as *const u8 as *const libc::c_char,
        ul_font,
        ul_ptsize.w,
        ul_ptsize.h,
    );
    printf(
        b"ul_position=%s (%g, %g)\n\0" as *const u8 as *const libc::c_char,
        ul_position,
        ul_x,
        ul_y,
    );
    printf(b"ul_angle=%g\n\0" as *const u8 as *const libc::c_char, ul_angle);
    printf(b"download-fonts:\0" as *const u8 as *const libc::c_char);
    i = strhash_get_first(
        download_fonts,
        &mut cp,
        &mut j,
        &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    while i != 0 {
        printf(b" %s\0" as *const u8 as *const libc::c_char, cp);
        i = strhash_get_next(
            download_fonts,
            &mut cp,
            &mut j,
            &mut cp2 as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Usage: %s [OPTION]... [FILE]...\nMandatory arguments to long options are mandatory for short options too.\n  -#                         an alias for option -n, --copies\n  -1                         same as --columns=1\n  -2                         same as --columns=2\n      --columns=NUM          specify the number of columns per page\n  -a, --pages=PAGES          specify which pages are printed\n  -A, --file-align=ALIGN     align separate input files to ALIGN\n  -b, --header=HEADER        set page header\n  -B, --no-header            no page headers\n  -c, --truncate-lines       cut long lines (default is to wrap)\n  -C, --line-numbers[=START]\n                             precede each line with its line number\n  -d                         an alias for option --printer\n  -D, --setpagedevice=KEY[:VALUE]\n                             pass a page device definition to output\n  -e, --escapes[=CHAR]       enable special escape interpretation\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        program,
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  -E, --pretty-print[=LANG]  pretty-print source code\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  -f, --font=NAME            use font NAME for body text\n  -F, --header-font=NAME     use font NAME for header texts\n  -g, --print-anyway         nothing (compatibility option)\n  -G                         same as --fancy-header\n      --fancy-header[=NAME]  select fancy page header\n  -h, --no-job-header        suppress the job header page\n  -H, --highlight-bars=NUM   specify how high highlight bars are\n  -i, --indent=NUM           set line indent to NUM characters\n  -I, --filter=CMD           read input files through input filter CMD\n  -j, --borders              print borders around columns\n  -J,                        an alias for option --title\n  -k, --page-prefeed         enable page prefeed\n  -K, --no-page-prefeed      disable page prefeed\n  -l, --lineprinter          simulate lineprinter, this is an alias for:\n                               --lines-per-page=66, --no-header, --portrait,\n                               --columns=1\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  -L, --lines-per-page=NUM   specify how many lines are printed on each page\n  -m, --mail                 send mail upon completion\n  -M, --media=NAME           use output media NAME\n  -n, --copies=NUM           print NUM copies of each page\n  -N, --newline=NL           select the newline character.  Possible\n                             values for NL are: n (`\\n') and r (`\\r').\n  -o                         an alias for option --output\n  -O, --missing-characters   list missing characters\n  -p, --output=FILE          leave output to file FILE.  If FILE is `-',\n                             leave output to stdout.\n  -P, --printer=NAME         print output to printer NAME\n  -q, --quiet, --silent      be really quiet\n  -r, --landscape            print in landscape mode\n  -R, --portrait             print in portrait mode\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  -s, --baselineskip=NUM     set baselineskip to NUM\n  -S, --statusdict=KEY[:VALUE]\n                             pass a statusdict definition to the output\n  -t, --title=TITLE          set banner page's job title to TITLE.  Option\n                             sets also the name of the input file stdin.\n  -T, --tabsize=NUM          set tabulator size to NUM\n  -u, --underlay[=TEXT]      print TEXT under every page\n  -U, --nup=NUM              print NUM logical pages on each output page\n  -v, --verbose              tell what we are doing\n  -V, --version              print version number\n  -W, --language=LANG        set output language to LANG\n  -X, --encoding=NAME        use input encoding NAME\n  -z, --no-formfeed          do not interpret form feed characters\n  -Z, --pass-through         pass through PostScript and PCL files\n                             without any modifications\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Long-only options:\n  --color[=COLOR]            set output color model to COLOR\n  --download-font=NAME       download font NAME\n  --filter-stdin=NAME        specify how stdin is shown to the input filter\n  --h-column-height=HEIGHT   set the horizontal column height to HEIGHT\n  --help                     print this help and exit\n  --help-pretty-print        describe all supported --pretty-print languages\n                             and file formats\n  --highlight-bar-gray=NUM   print highlight bars with gray NUM (0 - 1)\n  --list-media               list names of all known media\n  --list-options             list all options and their values\n  --margins=LEFT:RIGHT:TOP:BOTTOM\n                             adjust page marginals\n  --mark-wrapped-lines[STYLE]\n                             mark wrapped lines in the output with STYLE\n  --non-printable-format=FMT specify how non-printable chars are printed\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  --nup-xpad=NUM             set the page x-padding of N-up printing to NUM\n  --nup-ypad=NUM             set the page y-padding of N-up printing to NUM\n  --page-label-format=FMT    set page label format to FMT\n  --ps-level=LEVEL           set the PostScript language level that enscript\n                             should use\n  --printer-options=OPTIONS  pass extra options to the printer command\n  --rotate-even-pages        rotate even-numbered pages 180 degrees\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"  --slice=NUM                print vertical slice NUM\n  --toc                      print table of contents\n  --ul-angle=ANGLE           set underlay text's angle to ANGLE\n  --ul-font=NAME             print underlays with font NAME\n  --ul-gray=NUM              print underlays with gray value NUM\n  --ul-position=POS          set underlay's starting position to POS\n  --ul-style=STYLE           print underlays with style STYLE\n  --word-wrap                wrap long lines from word boundaries\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    printf(
        dcgettext(
            0 as *const libc::c_char,
            b"\nReport bugs to mtr@iki.fi.\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn version() {
    printf(
        b"%s\nCopyright (C) 1998 Markku Rossi.\nGNU enscript comes with NO WARRANTY, to the extent permitted by law.\nYou may redistribute copies of GNU enscript under the terms of the GNU\nGeneral Public License.  For more information about these matters, see\nthe files named COPYING.\n\0"
            as *const u8 as *const libc::c_char,
        version_string.as_mut_ptr(),
    );
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
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
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
