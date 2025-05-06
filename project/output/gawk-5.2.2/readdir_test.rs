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
extern "C" {
    pub type __dirstream;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn exit(_: i32) -> !;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcat(_: *mut i8, _: *const i8) -> *mut i8;
    fn strerror(_: i32) -> *mut i8;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn fdopendir(__fd: i32) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> i32;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: u8,
    pub d_name: [i8; 256],
}
pub type C2RustUnnamed = u32;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
pub type awk_bool = u32;
pub const awk_true: awk_bool = 1;
pub const awk_false: awk_bool = 0;
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
pub type C2RustUnnamed_0 = u32;
pub const GAWK_API_MINOR_VERSION: C2RustUnnamed_0 = 2;
pub const GAWK_API_MAJOR_VERSION: C2RustUnnamed_0 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str_0: *mut i8,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
pub type AWK_NUMBER_TYPE = u32;
pub const AWK_NUMBER_TYPE_MPZ: AWK_NUMBER_TYPE = 2;
pub const AWK_NUMBER_TYPE_MPFR: AWK_NUMBER_TYPE = 1;
pub const AWK_NUMBER_TYPE_DOUBLE: AWK_NUMBER_TYPE = 0;
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
pub type awk_valtype_t = u32;
pub const AWK_BOOL: awk_valtype_t = 8;
pub const AWK_VALUE_COOKIE: awk_valtype_t = 7;
pub const AWK_SCALAR: awk_valtype_t = 6;
pub const AWK_ARRAY: awk_valtype_t = 5;
pub const AWK_STRNUM: awk_valtype_t = 4;
pub const AWK_REGEX: awk_valtype_t = 3;
pub const AWK_STRING: awk_valtype_t = 2;
pub const AWK_NUMBER: awk_valtype_t = 1;
pub const AWK_UNDEFINED: awk_valtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
    pub flags: C2RustUnnamed_2,
    pub index: awk_value_t,
    pub value: awk_value_t,
}
pub type C2RustUnnamed_2 = u32;
pub const AWK_ELEMENT_DELETE: C2RustUnnamed_2 = 1;
pub const AWK_ELEMENT_DEFAULT: C2RustUnnamed_2 = 0;
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
pub type open_directory_t = open_directory;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_directory {
    pub dp: *mut DIR,
    pub buf: *mut i8,
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub fw: awk_fieldwidth_info_t,
    pub buf: [i8; 64],
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut ext_version: *const i8 = b"readdir extension: version 3.0\0" as *const u8
    as *const i8;
static mut init_func: Option<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_readdir as unsafe extern "C" fn() -> awk_bool_t)
};
#[no_mangle]
pub static mut plugin_is_GPL_compatible: i32 = 0;
unsafe extern "C" fn ftype(mut entry: *mut dirent, mut dirname: *const i8) -> *const i8 {
    match (*entry).d_type as i32 {
        6 => return b"b\0" as *const u8 as *const i8,
        2 => return b"c\0" as *const u8 as *const i8,
        4 => return b"d\0" as *const u8 as *const i8,
        1 => return b"p\0" as *const u8 as *const i8,
        10 => return b"l\0" as *const u8 as *const i8,
        8 => return b"f\0" as *const u8 as *const i8,
        12 => return b"s\0" as *const u8 as *const i8,
        0 | _ => {}
    }
    let mut fname: [i8; 4096] = [0; 4096];
    let mut sbuf: stat = stat {
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
    strcpy(fname.as_mut_ptr(), dirname);
    strcat(fname.as_mut_ptr(), b"/\0" as *const u8 as *const i8);
    strcat(fname.as_mut_ptr(), ((*entry).d_name).as_mut_ptr());
    if stat(fname.as_mut_ptr(), &mut sbuf) == 0 as i32 {
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32 {
            return b"b\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32 {
            return b"c\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32 {
            return b"d\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o10000 as i32 as u32 {
            return b"p\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32 {
            return b"f\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
            return b"l\0" as *const u8 as *const i8;
        }
        if sbuf.st_mode & 0o170000 as i32 as u32 == 0o140000 as i32 as u32 {
            return b"s\0" as *const u8 as *const i8;
        }
    }
    return b"u\0" as *const u8 as *const i8;
}
unsafe extern "C" fn get_inode(
    mut entry: *mut dirent,
    mut dirname: *const i8,
) -> libc::c_longlong {
    return (*entry).d_ino as libc::c_longlong;
}
unsafe extern "C" fn dir_get_record(
    mut out: *mut *mut i8,
    mut iobuf: *mut awk_input_buf_t,
    mut errcode: *mut i32,
    mut rt_start: *mut *mut i8,
    mut rt_len: *mut size_t,
    mut field_width: *mut *const awk_fieldwidth_info_t,
) -> i32 {
    let mut dp: *mut DIR = 0 as *mut DIR;
    let mut dirent: *mut dirent = 0 as *mut dirent;
    let mut len: i32 = 0;
    let mut flen: i32 = 0;
    let mut the_dir: *mut open_directory_t = 0 as *mut open_directory_t;
    let mut ftstr: *const i8 = 0 as *const i8;
    let mut ino: libc::c_ulonglong = 0;
    if out.is_null() || iobuf.is_null() || ((*iobuf).opaque).is_null() {
        return -(1 as i32);
    }
    the_dir = (*iobuf).opaque as *mut open_directory_t;
    dp = (*the_dir).dp;
    *__errno_location() = 0 as i32;
    dirent = readdir(dp);
    if dirent.is_null() {
        *errcode = *__errno_location();
        return -(1 as i32);
    }
    ino = get_inode(dirent, (*iobuf).name) as libc::c_ulonglong;
    len = sprintf((*the_dir).buf, b"%llu\0" as *const u8 as *const i8, ino);
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(0 as i32 as isize)).len = len
        as size_t;
    flen = sprintf(
        ((*the_dir).buf).offset(len as isize),
        b"/%s\0" as *const u8 as *const i8,
        ((*dirent).d_name).as_mut_ptr(),
    );
    len += flen;
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(1 as i32 as isize)).len = (flen
        - 1 as i32) as size_t;
    ftstr = ftype(dirent, (*iobuf).name);
    flen = sprintf(
        ((*the_dir).buf).offset(len as isize),
        b"/%s\0" as *const u8 as *const i8,
        ftstr,
    );
    len += flen;
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(2 as i32 as isize)).len = (flen
        - 1 as i32) as size_t;
    *out = (*the_dir).buf;
    *rt_start = 0 as *mut i8;
    *rt_len = 0 as i32 as size_t;
    if !field_width.is_null() {
        *field_width = &mut (*the_dir).u.fw;
    }
    return len;
}
unsafe extern "C" fn dir_close(mut iobuf: *mut awk_input_buf_t) {
    let mut the_dir: *mut open_directory_t = 0 as *mut open_directory_t;
    if iobuf.is_null() || ((*iobuf).opaque).is_null() {
        return;
    }
    the_dir = (*iobuf).opaque as *mut open_directory_t;
    closedir((*the_dir).dp);
    ((*api).api_free)
        .expect("non-null function pointer")((*the_dir).buf as *mut libc::c_void);
    ((*api).api_free).expect("non-null function pointer")(the_dir as *mut libc::c_void);
    (*iobuf).fd = -(1 as i32);
}
unsafe extern "C" fn dir_can_take_file(mut iobuf: *const awk_input_buf_t) -> awk_bool_t {
    if iobuf.is_null() {
        return awk_false;
    }
    return ((*iobuf).fd != -(1 as i32)
        || (*iobuf).sbuf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32)
        as i32 as awk_bool_t;
}
unsafe extern "C" fn dir_take_control_of(mut iobuf: *mut awk_input_buf_t) -> awk_bool_t {
    let mut dp: *mut DIR = 0 as *mut DIR;
    let mut the_dir: *mut open_directory_t = 0 as *mut open_directory_t;
    let mut size: size_t = 0;
    *__errno_location() = 0 as i32;
    dp = fdopendir((*iobuf).fd);
    if dp.is_null() {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"dir_take_control_of: opendir/fdopendir failed: %s\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, *__errno_location());
        return awk_false;
    }
    the_dir = ((*api).api_malloc)
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<open_directory_t>() as u64) as *mut open_directory_t;
    if the_dir.is_null() {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
            b"dir_take_control_of\0" as *const u8 as *const i8,
            ::core::mem::size_of::<open_directory_t>() as u64,
        );
    }
    (*the_dir).dp = dp;
    (*the_dir).u.fw.use_chars = awk_false;
    (*the_dir).u.fw.nf = 3 as i32 as size_t;
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(0 as i32 as isize)).skip = 0 as i32
        as size_t;
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(1 as i32 as isize)).skip = 1 as i32
        as size_t;
    (*((*the_dir).u.fw.fields).as_mut_ptr().offset(2 as i32 as isize)).skip = 1 as i32
        as size_t;
    size = (::core::mem::size_of::<dirent>() as u64)
        .wrapping_add(21 as i32 as u64)
        .wrapping_add(2 as i32 as u64);
    (*the_dir).buf = ((*api).api_malloc).expect("non-null function pointer")(size)
        as *mut i8;
    if ((*the_dir).buf).is_null() {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
            b"dir_take_control_of\0" as *const u8 as *const i8,
            size,
        );
    }
    (*iobuf).opaque = the_dir as *mut libc::c_void;
    (*iobuf).get_record = Some(
        dir_get_record
            as unsafe extern "C" fn(
                *mut *mut i8,
                *mut awk_input_buf_t,
                *mut i32,
                *mut *mut i8,
                *mut size_t,
                *mut *const awk_fieldwidth_info_t,
            ) -> i32,
    );
    (*iobuf).close_func = Some(
        dir_close as unsafe extern "C" fn(*mut awk_input_buf_t) -> (),
    );
    return awk_true;
}
static mut readdir_parser: awk_input_parser_t = unsafe {
    {
        let mut init = awk_input_parser {
            name: b"readdir\0" as *const u8 as *const i8,
            can_take_file: Some(
                dir_can_take_file
                    as unsafe extern "C" fn(*const awk_input_buf_t) -> awk_bool_t,
            ),
            take_control_of: Some(
                dir_take_control_of
                    as unsafe extern "C" fn(*mut awk_input_buf_t) -> awk_bool_t,
            ),
            next: 0 as *const awk_input_parser,
        };
        init
    }
};
unsafe extern "C" fn init_readdir() -> awk_bool_t {
    ((*api).api_register_input_parser)
        .expect("non-null function pointer")(ext_id, &mut readdir_parser);
    return awk_true;
}
static mut func_table: [awk_ext_func_t; 1] = [
    {
        let mut init = awk_ext_func {
            name: 0 as *const i8,
            function: None,
            max_expected_args: 0 as i32 as size_t,
            min_required_args: 0 as i32 as size_t,
            suppress_lint: awk_false,
            data: 0 as *const libc::c_void as *mut libc::c_void,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, mut id: awk_ext_id_t) -> i32 {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut errors: i32 = 0 as i32;
    api = api_p;
    ext_id = id as *mut *mut libc::c_void as awk_ext_id_t;
    if (*api).major_version != GAWK_API_MAJOR_VERSION as i32
        || (*api).minor_version < GAWK_API_MINOR_VERSION as i32
    {
        fprintf(
            stderr,
            b"readdir: version mismatch with gawk!\n\0" as *const u8 as *const i8,
        );
        fprintf(
            stderr,
            b"\tmy version (API %d.%d), gawk version (API %d.%d)\n\0" as *const u8
                as *const i8,
            GAWK_API_MAJOR_VERSION as i32,
            GAWK_API_MINOR_VERSION as i32,
            (*api).major_version,
            (*api).minor_version,
        );
        exit(1 as i32);
    }
    i = 0 as i32 as size_t;
    j = (::core::mem::size_of::<[awk_ext_func_t; 1]>() as u64)
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
                b"readdir: could not add %s\0" as *const u8 as *const i8,
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
                b"readdir: initialization function failed\0" as *const u8 as *const i8,
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