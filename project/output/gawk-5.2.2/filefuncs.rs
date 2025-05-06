#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn __errno_location() -> *mut i32;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strerror(_: i32) -> *mut i8;
    fn chdir(__path: *const i8) -> i32;
    fn readlink(__path: *const i8, __buf: *mut i8, __len: size_t) -> ssize_t;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
    fn statvfs(__file: *const i8, __buf: *mut statvfs) -> i32;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn gawk_fts_close(_: *mut FTS) -> i32;
    fn gawk_fts_open(
        _: *const *mut i8,
        _: i32,
        _: Option<unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> i32>,
    ) -> *mut FTS;
    fn gawk_fts_read(_: *mut FTS) -> *mut FTSENT;
    fn gawk_fts_set(_: *mut FTS, _: *mut FTSENT, _: i32) -> i32;
    fn stack_empty() -> i32;
    fn stack_pop() -> *mut libc::c_void;
    fn stack_push(_: *mut libc::c_void) -> i32;
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
pub type __fsblkcnt_t = u64;
pub type __fsfilcnt_t = u64;
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
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
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
pub struct statvfs {
    pub f_bsize: u64,
    pub f_frsize: u64,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_favail: __fsfilcnt_t,
    pub f_fsid: u64,
    pub f_flag: u64,
    pub f_namemax: u64,
    pub __f_spare: [i32; 6],
}
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
pub type C2RustUnnamed = u32;
pub const GAWK_API_MINOR_VERSION: C2RustUnnamed = 2;
pub const GAWK_API_MAJOR_VERSION: C2RustUnnamed = 3;
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
pub type C2RustUnnamed_1 = u32;
pub const AWK_ELEMENT_DELETE: C2RustUnnamed_1 = 1;
pub const AWK_ELEMENT_DEFAULT: C2RustUnnamed_1 = 0;
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
pub struct flagtab {
    pub name: *const i8,
    pub value: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut i8,
    pub fts_rfd: i32,
    pub fts_pathlen: u32,
    pub fts_nitems: u32,
    pub fts_compar: Option<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> i32,
    >,
    pub fts_options: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_number: libc::c_longlong,
    pub fts_pointer: *mut libc::c_void,
    pub fts_accpath: *mut i8,
    pub fts_path: *mut i8,
    pub fts_errno: i32,
    pub fts_symfd: i32,
    pub fts_pathlen: u32,
    pub fts_namelen: u32,
    pub fts_ino: ino_t,
    pub fts_dev: dev_t,
    pub fts_nlink: u32,
    pub fts_level: i32,
    pub fts_info: libc::c_ushort,
    pub fts_flags: libc::c_ushort,
    pub fts_instr: libc::c_ushort,
    pub fts_statp: *mut stat,
    pub fts_name: [i8; 1],
}
pub type FTSENT = _ftsent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftype_map {
    pub mask: u32,
    pub type_0: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setuid_map {
    pub mask: u32,
    pub index: i32,
    pub small_rep: i32,
    pub big_rep: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_map {
    pub mask: u32,
    pub rep: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftype_map_0 {
    pub mask: u32,
    pub charval: i32,
}
#[inline]
unsafe extern "C" fn gnu_dev_minor(mut __dev: __dev_t) -> u32 {
    let mut __minor: u32 = 0;
    __minor = ((__dev & 0xff as u32 as __dev_t) >> 0 as i32) as u32;
    __minor = (__minor as u64 | (__dev & 0xffffff00000 as u64) >> 12 as i32) as u32;
    return __minor;
}
#[inline]
unsafe extern "C" fn gnu_dev_major(mut __dev: __dev_t) -> u32 {
    let mut __major: u32 = 0;
    __major = ((__dev & 0xfff00 as u32 as __dev_t) >> 8 as i32) as u32;
    __major = (__major as u64 | (__dev & 0xfffff00000000000 as u64) >> 32 as i32) as u32;
    return __major;
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn r_make_string_type(
    mut api_0: *const gawk_api_t,
    mut ext_id_0: awk_ext_id_t,
    mut string: *const i8,
    mut length: size_t,
    mut duplicate: awk_bool_t,
    mut result: *mut awk_value_t,
    mut val_type: awk_valtype_t,
) -> *mut awk_value_t {
    let mut cp: *mut i8 = 0 as *mut i8;
    memset(
        result as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<awk_value_t>() as u64,
    );
    (*result).val_type = val_type;
    (*result).u.s.len = length;
    if duplicate as u64 != 0 {
        cp = ((*api_0).api_malloc)
            .expect("non-null function pointer")(length.wrapping_add(1 as i32 as u64))
            as *mut i8;
        if cp.is_null() {
            ((*api_0).api_fatal)
                .expect(
                    "non-null function pointer",
                )(
                ext_id_0,
                b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
                b"r_make_string\0" as *const u8 as *const i8,
                length.wrapping_add(1 as i32 as u64),
            );
        }
        memcpy(cp as *mut libc::c_void, string as *const libc::c_void, length);
        *cp.offset(length as isize) = '\0' as i32 as i8;
        (*result).u.s.str_0 = cp;
    } else {
        (*result).u.s.str_0 = string as *mut i8;
    }
    return result;
}
#[inline]
unsafe extern "C" fn r_make_string(
    mut api_0: *const gawk_api_t,
    mut ext_id_0: awk_ext_id_t,
    mut string: *const i8,
    mut length: size_t,
    mut duplicate: awk_bool_t,
    mut result: *mut awk_value_t,
) -> *mut awk_value_t {
    return r_make_string_type(
        api_0,
        ext_id_0,
        string,
        length,
        duplicate,
        result,
        AWK_STRING,
    );
}
#[inline]
unsafe extern "C" fn make_number(
    mut num: libc::c_double,
    mut result: *mut awk_value_t,
) -> *mut awk_value_t {
    (*result).val_type = AWK_NUMBER;
    (*result).u.n.d = num;
    (*result).u.n.type_0 = AWK_NUMBER_TYPE_DOUBLE;
    return result;
}
static mut api: *const gawk_api_t = 0 as *const gawk_api_t;
static mut ext_id: awk_ext_id_t = 0 as *const libc::c_void as *mut libc::c_void;
static mut init_func: Option<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_filefuncs as unsafe extern "C" fn() -> awk_bool_t)
};
static mut ext_version: *const i8 = b"filefuncs extension: version 1.0\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut plugin_is_GPL_compatible: i32 = 0;
unsafe extern "C" fn do_chdir(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut newdir: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut ret: i32 = -(1 as i32);
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_STRING, &mut newdir) as u64 != 0
    {
        ret = chdir(newdir.u.s.str_0);
        if ret < 0 as i32 {
            ((*api).api_update_ERRNO_int)
                .expect("non-null function pointer")(ext_id, *__errno_location());
        }
    }
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn format_mode(mut fmode: u64) -> *mut i8 {
    static mut outbuf: [i8; 12] = [0; 12];
    static mut ftype_map_0: [ftype_map_0; 7] = [
        {
            let mut init = ftype_map_0 {
                mask: 0o100000 as i32 as u32,
                charval: '-' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o60000 as i32 as u32,
                charval: 'b' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o20000 as i32 as u32,
                charval: 'c' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o40000 as i32 as u32,
                charval: 'd' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o140000 as i32 as u32,
                charval: 's' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o10000 as i32 as u32,
                charval: 'p' as i32,
            };
            init
        },
        {
            let mut init = ftype_map_0 {
                mask: 0o120000 as i32 as u32,
                charval: 'l' as i32,
            };
            init
        },
    ];
    static mut map: [mode_map; 9] = [
        {
            let mut init = mode_map {
                mask: 0o400 as i32 as u32,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: 0o200 as i32 as u32,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: 0o100 as i32 as u32,
                rep: 'x' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o400 as i32 >> 3 as i32) as u32,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o200 as i32 >> 3 as i32) as u32,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o100 as i32 >> 3 as i32) as u32,
                rep: 'x' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o400 as i32 >> 3 as i32 >> 3 as i32) as u32,
                rep: 'r' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o200 as i32 >> 3 as i32 >> 3 as i32) as u32,
                rep: 'w' as i32,
            };
            init
        },
        {
            let mut init = mode_map {
                mask: (0o100 as i32 >> 3 as i32 >> 3 as i32) as u32,
                rep: 'x' as i32,
            };
            init
        },
    ];
    static mut setuid_map_0: [setuid_map; 3] = [
        {
            let mut init = setuid_map {
                mask: 0o4000 as i32 as u32,
                index: 3 as i32,
                small_rep: 's' as i32,
                big_rep: 'S' as i32,
            };
            init
        },
        {
            let mut init = setuid_map {
                mask: 0o2000 as i32 as u32,
                index: 6 as i32,
                small_rep: 's' as i32,
                big_rep: 'l' as i32,
            };
            init
        },
        {
            let mut init = setuid_map {
                mask: 0o1000 as i32 as u32,
                index: 9 as i32,
                small_rep: 't' as i32,
                big_rep: 'T' as i32,
            };
            init
        },
    ];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    strcpy(outbuf.as_mut_ptr(), b"----------\0" as *const u8 as *const i8);
    i = 0 as i32;
    j = 0 as i32;
    k = (::core::mem::size_of::<[ftype_map_0; 7]>() as u64)
        .wrapping_div(::core::mem::size_of::<ftype_map_0>() as u64) as i32;
    while j < k {
        if fmode & 0o170000 as i32 as u64 == ftype_map_0[j as usize].mask as u64 {
            outbuf[i as usize] = ftype_map_0[j as usize].charval as i8;
            break;
        } else {
            j += 1;
            j;
        }
    }
    j = 0 as i32;
    k = (::core::mem::size_of::<[mode_map; 9]>() as u64)
        .wrapping_div(::core::mem::size_of::<mode_map>() as u64) as i32;
    while j < k {
        i += 1;
        i;
        if fmode & map[j as usize].mask as u64 != 0 as i32 as u64 {
            outbuf[i as usize] = map[j as usize].rep as i8;
        }
        j += 1;
        j;
    }
    i += 1;
    i;
    outbuf[i as usize] = '\0' as i32 as i8;
    j = 0 as i32;
    k = (::core::mem::size_of::<[setuid_map; 3]>() as u64)
        .wrapping_div(::core::mem::size_of::<setuid_map>() as u64) as i32;
    while j < k {
        if fmode & setuid_map_0[j as usize].mask as u64 != 0 {
            if outbuf[setuid_map_0[j as usize].index as usize] as i32 == 'x' as i32 {
                outbuf[setuid_map_0[j as usize].index as usize] = setuid_map_0[j
                        as usize]
                    .small_rep as i8;
            } else {
                outbuf[setuid_map_0[j as usize].index as usize] = setuid_map_0[j
                        as usize]
                    .big_rep as i8;
            }
        }
        j += 1;
        j;
    }
    return outbuf.as_mut_ptr();
}
unsafe extern "C" fn read_symlink(
    mut fname: *const i8,
    mut bufsize: size_t,
    mut linksize: *mut ssize_t,
) -> *mut i8 {
    if bufsize != 0 {
        bufsize = (bufsize as u64).wrapping_add(2 as i32 as u64) as size_t as size_t;
    } else {
        bufsize = (8192 as i32 * 2 as i32) as size_t;
    }
    if bufsize
        > (if (-(1 as i32) as size_t) < 9223372036854775807 as i64 as u64 {
            -(1 as i32) as size_t
        } else {
            9223372036854775807 as i64 as u64
        }) || bufsize < 2 as i32 as u64
    {
        bufsize = if (-(1 as i32) as size_t) < 9223372036854775807 as i64 as u64 {
            -(1 as i32) as size_t
        } else {
            9223372036854775807 as i64 as u64
        };
    }
    loop {
        let mut buf: *mut i8 = 0 as *mut i8;
        buf = ((*api).api_malloc).expect("non-null function pointer")(bufsize)
            as *mut i8;
        if buf.is_null() {
            ((*api).api_fatal)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
                b"read_symlink\0" as *const u8 as *const i8,
                bufsize,
            );
        }
        *linksize = readlink(fname, buf, bufsize);
        if *linksize < 0 as i32 as i64 {
            if *__errno_location() != 34 as i32 {
                ((*api).api_free)
                    .expect("non-null function pointer")(buf as *mut libc::c_void);
                return 0 as *mut i8;
            }
        } else if *linksize as size_t <= bufsize.wrapping_sub(2 as i32 as u64) {
            *buf.offset(*linksize as isize) = '\0' as i32 as i8;
            return buf;
        }
        ((*api).api_free).expect("non-null function pointer")(buf as *mut libc::c_void);
        if bufsize
            <= (if (-(1 as i32) as size_t) < 9223372036854775807 as i64 as u64 {
                -(1 as i32) as size_t
            } else {
                9223372036854775807 as i64 as u64
            })
                .wrapping_div(2 as i32 as u64)
        {
            bufsize = (bufsize as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
        } else if bufsize
            < (if (-(1 as i32) as size_t) < 9223372036854775807 as i64 as u64 {
                -(1 as i32) as size_t
            } else {
                9223372036854775807 as i64 as u64
            })
        {
            bufsize = if (-(1 as i32) as size_t) < 9223372036854775807 as i64 as u64 {
                -(1 as i32) as size_t
            } else {
                9223372036854775807 as i64 as u64
            };
        } else {
            return 0 as *mut i8
        }
    };
}
unsafe extern "C" fn device_blocksize() -> i32 {
    return 512 as i32;
}
unsafe extern "C" fn array_set(
    mut array: awk_array_t,
    mut sub: *const i8,
    mut value: *mut awk_value_t,
) {
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    ((*api).api_set_array_element)
        .expect(
            "non-null function pointer",
        )(
        ext_id,
        array,
        r_make_string(api, ext_id, sub, strlen(sub), awk_true, &mut index),
        value,
    );
}
unsafe extern "C" fn array_set_numeric(
    mut array: awk_array_t,
    mut sub: *const i8,
    mut num: libc::c_double,
) {
    let mut tmp: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    array_set(array, sub, make_number(num, &mut tmp));
}
unsafe extern "C" fn fill_stat_array(
    mut name: *const i8,
    mut array: awk_array_t,
    mut sbuf: *mut stat,
) -> i32 {
    let mut pmode: *mut i8 = 0 as *mut i8;
    let mut type_0: *const i8 = b"unknown\0" as *const u8 as *const i8;
    let mut tmp: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    static mut ftype_map_0: [ftype_map; 7] = [
        {
            let mut init = ftype_map {
                mask: 0o100000 as i32 as u32,
                type_0: b"file\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o60000 as i32 as u32,
                type_0: b"blockdev\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o20000 as i32 as u32,
                type_0: b"chardev\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o40000 as i32 as u32,
                type_0: b"directory\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o140000 as i32 as u32,
                type_0: b"socket\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o10000 as i32 as u32,
                type_0: b"fifo\0" as *const u8 as *const i8,
            };
            init
        },
        {
            let mut init = ftype_map {
                mask: 0o120000 as i32 as u32,
                type_0: b"symlink\0" as *const u8 as *const i8,
            };
            init
        },
    ];
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    ((*api).api_clear_array).expect("non-null function pointer")(ext_id, array);
    array_set(
        array,
        b"name\0" as *const u8 as *const i8,
        r_make_string(api, ext_id, name, strlen(name), awk_true, &mut tmp),
    );
    array_set_numeric(
        array,
        b"dev\0" as *const u8 as *const i8,
        (*sbuf).st_dev as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ino\0" as *const u8 as *const i8,
        (*sbuf).st_ino as libc::c_double,
    );
    array_set_numeric(
        array,
        b"mode\0" as *const u8 as *const i8,
        (*sbuf).st_mode as libc::c_double,
    );
    array_set_numeric(
        array,
        b"nlink\0" as *const u8 as *const i8,
        (*sbuf).st_nlink as libc::c_double,
    );
    array_set_numeric(
        array,
        b"uid\0" as *const u8 as *const i8,
        (*sbuf).st_uid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"gid\0" as *const u8 as *const i8,
        (*sbuf).st_gid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"size\0" as *const u8 as *const i8,
        (*sbuf).st_size as libc::c_double,
    );
    array_set_numeric(
        array,
        b"blocks\0" as *const u8 as *const i8,
        (*sbuf).st_blocks as libc::c_double,
    );
    array_set_numeric(
        array,
        b"atime\0" as *const u8 as *const i8,
        (*sbuf).st_atim.tv_sec as libc::c_double,
    );
    array_set_numeric(
        array,
        b"mtime\0" as *const u8 as *const i8,
        (*sbuf).st_mtim.tv_sec as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ctime\0" as *const u8 as *const i8,
        (*sbuf).st_ctim.tv_sec as libc::c_double,
    );
    if (*sbuf).st_mode & 0o170000 as i32 as u32 == 0o60000 as i32 as u32
        || (*sbuf).st_mode & 0o170000 as i32 as u32 == 0o20000 as i32 as u32
    {
        array_set_numeric(
            array,
            b"rdev\0" as *const u8 as *const i8,
            (*sbuf).st_rdev as libc::c_double,
        );
        array_set_numeric(
            array,
            b"major\0" as *const u8 as *const i8,
            gnu_dev_major((*sbuf).st_rdev) as libc::c_double,
        );
        array_set_numeric(
            array,
            b"minor\0" as *const u8 as *const i8,
            gnu_dev_minor((*sbuf).st_rdev) as libc::c_double,
        );
    }
    array_set_numeric(
        array,
        b"blksize\0" as *const u8 as *const i8,
        (*sbuf).st_blksize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"devbsize\0" as *const u8 as *const i8,
        device_blocksize() as libc::c_double,
    );
    pmode = format_mode((*sbuf).st_mode as u64);
    array_set(
        array,
        b"pmode\0" as *const u8 as *const i8,
        r_make_string(api, ext_id, pmode, strlen(pmode), awk_true, &mut tmp),
    );
    if (*sbuf).st_mode & 0o170000 as i32 as u32 == 0o120000 as i32 as u32 {
        let mut buf: *mut i8 = 0 as *mut i8;
        let mut linksize: ssize_t = 0;
        buf = read_symlink(name, (*sbuf).st_size as size_t, &mut linksize);
        if !buf.is_null() {
            array_set(
                array,
                b"linkval\0" as *const u8 as *const i8,
                r_make_string(api, ext_id, buf, linksize as size_t, awk_false, &mut tmp),
            );
        } else {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                dcgettext(
                    0 as *const i8,
                    b"stat: unable to read symbolic link `%s'\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                name,
            );
        }
    }
    type_0 = b"unknown\0" as *const u8 as *const i8;
    j = 0 as i32;
    k = (::core::mem::size_of::<[ftype_map; 7]>() as u64)
        .wrapping_div(::core::mem::size_of::<ftype_map>() as u64) as i32;
    while j < k {
        if (*sbuf).st_mode & 0o170000 as i32 as u32 == ftype_map_0[j as usize].mask {
            type_0 = ftype_map_0[j as usize].type_0;
            break;
        } else {
            j += 1;
            j;
        }
    }
    array_set(
        array,
        b"type\0" as *const u8 as *const i8,
        r_make_string(api, ext_id, type_0, strlen(type_0), awk_true, &mut tmp),
    );
    return 0 as i32;
}
unsafe extern "C" fn do_stat(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut file_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut array_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut name: *mut i8 = 0 as *mut i8;
    let mut array: awk_array_t = 0 as *mut libc::c_void;
    let mut ret: i32 = 0;
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
    let mut statfunc: Option<unsafe extern "C" fn(*const i8, *mut stat) -> i32> = Some(
        lstat as unsafe extern "C" fn(*const i8, *mut stat) -> i32,
    );
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_STRING, &mut file_param) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"stat: first argument is not a string\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 1 as i32 as size_t, AWK_ARRAY, &mut array_param) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"stat: second argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    if nargs == 3 as i32 {
        statfunc = Some(stat as unsafe extern "C" fn(*const i8, *mut stat) -> i32);
    }
    name = file_param.u.s.str_0;
    array = array_param.u.a;
    ((*api).api_clear_array).expect("non-null function pointer")(ext_id, array);
    ret = statfunc.expect("non-null function pointer")(name, &mut sbuf);
    if ret < 0 as i32 {
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, *__errno_location());
        return make_number(ret as libc::c_double, result);
    }
    ret = fill_stat_array(name, array, &mut sbuf);
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn do_statvfs(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut file_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut array_param: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut name: *mut i8 = 0 as *mut i8;
    let mut array: awk_array_t = 0 as *mut libc::c_void;
    let mut ret: i32 = 0;
    let mut vfsbuf: statvfs = statvfs {
        f_bsize: 0,
        f_frsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_favail: 0,
        f_fsid: 0,
        f_flag: 0,
        f_namemax: 0,
        __f_spare: [0; 6],
    };
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_STRING, &mut file_param) as u64 == 0
        || ((*api).api_get_argument)
            .expect(
                "non-null function pointer",
            )(ext_id, 1 as i32 as size_t, AWK_ARRAY, &mut array_param) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"stat: bad parameters\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    name = file_param.u.s.str_0;
    array = array_param.u.a;
    ((*api).api_clear_array).expect("non-null function pointer")(ext_id, array);
    ret = statvfs(name, &mut vfsbuf);
    if ret < 0 as i32 {
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, *__errno_location());
        return make_number(ret as libc::c_double, result);
    }
    array_set_numeric(
        array,
        b"bsize\0" as *const u8 as *const i8,
        vfsbuf.f_bsize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"frsize\0" as *const u8 as *const i8,
        vfsbuf.f_frsize as libc::c_double,
    );
    array_set_numeric(
        array,
        b"blocks\0" as *const u8 as *const i8,
        vfsbuf.f_blocks as libc::c_double,
    );
    array_set_numeric(
        array,
        b"bfree\0" as *const u8 as *const i8,
        vfsbuf.f_bfree as libc::c_double,
    );
    array_set_numeric(
        array,
        b"bavail\0" as *const u8 as *const i8,
        vfsbuf.f_bavail as libc::c_double,
    );
    array_set_numeric(
        array,
        b"files\0" as *const u8 as *const i8,
        vfsbuf.f_files as libc::c_double,
    );
    array_set_numeric(
        array,
        b"ffree\0" as *const u8 as *const i8,
        vfsbuf.f_ffree as libc::c_double,
    );
    array_set_numeric(
        array,
        b"favail\0" as *const u8 as *const i8,
        vfsbuf.f_favail as libc::c_double,
    );
    array_set_numeric(
        array,
        b"fsid\0" as *const u8 as *const i8,
        vfsbuf.f_fsid as libc::c_double,
    );
    array_set_numeric(
        array,
        b"flag\0" as *const u8 as *const i8,
        vfsbuf.f_flag as libc::c_double,
    );
    array_set_numeric(
        array,
        b"namemax\0" as *const u8 as *const i8,
        vfsbuf.f_namemax as libc::c_double,
    );
    return make_number(ret as libc::c_double, result);
}
unsafe extern "C" fn init_filefuncs() -> awk_bool_t {
    let mut errors: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    static mut opentab: [flagtab; 8] = [
        {
            let mut init = flagtab {
                name: b"FTS_COMFOLLOW\0" as *const u8 as *const i8,
                value: 0x1 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_LOGICAL\0" as *const u8 as *const i8,
                value: 0x2 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_NOCHDIR\0" as *const u8 as *const i8,
                value: 0x4 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_PHYSICAL\0" as *const u8 as *const i8,
                value: 0x10 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_SEEDOT\0" as *const u8 as *const i8,
                value: 0x20 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_XDEV\0" as *const u8 as *const i8,
                value: 0x40 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: b"FTS_SKIP\0" as *const u8 as *const i8,
                value: 0x200 as i32,
            };
            init
        },
        {
            let mut init = flagtab {
                name: 0 as *const i8,
                value: 0 as i32,
            };
            init
        },
    ];
    i = 0 as i32;
    while !(opentab[i as usize].name).is_null() {
        make_number(opentab[i as usize].value as libc::c_double, &mut value);
        if ((*api).api_sym_update)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"\0" as *const u8 as *const i8,
            opentab[i as usize].name,
            &mut value,
        ) as u64 == 0
        {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                dcgettext(
                    0 as *const i8,
                    b"fts init: could not create variable %s\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                opentab[i as usize].name,
            );
            errors += 1;
            errors;
        }
        i += 1;
        i;
    }
    return (errors == 0 as i32) as i32 as awk_bool_t;
}
static mut fts_errors: i32 = 0 as i32;
unsafe extern "C" fn fill_stat_element(
    mut element_array: awk_array_t,
    mut name: *const i8,
    mut sbuf: *mut stat,
) {
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut stat_array: awk_array_t = 0 as *mut libc::c_void;
    stat_array = ((*api).api_create_array).expect("non-null function pointer")(ext_id);
    if stat_array.is_null() {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fill_stat_element: could not create array, out of memory\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fts_errors += 1;
        fts_errors;
        return;
    }
    fill_stat_array(name, stat_array, sbuf);
    r_make_string(
        api,
        ext_id,
        b"stat\0" as *const u8 as *const i8,
        4 as i32 as size_t,
        awk_true,
        &mut index,
    );
    value.val_type = AWK_ARRAY;
    value.u.a = stat_array;
    if ((*api).api_set_array_element)
        .expect(
            "non-null function pointer",
        )(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fill_stat_element: could not set element\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_path_element(
    mut element_array: awk_array_t,
    mut path: *const i8,
) {
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    r_make_string(
        api,
        ext_id,
        b"path\0" as *const u8 as *const i8,
        4 as i32 as size_t,
        awk_true,
        &mut index,
    );
    r_make_string(api, ext_id, path, strlen(path), awk_true, &mut value);
    if ((*api).api_set_array_element)
        .expect(
            "non-null function pointer",
        )(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fill_path_element: could not set element\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_error_element(mut element_array: awk_array_t, errcode: i32) {
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut err: *const i8 = strerror(errcode);
    r_make_string(
        api,
        ext_id,
        b"error\0" as *const u8 as *const i8,
        5 as i32 as size_t,
        awk_true,
        &mut index,
    );
    r_make_string(api, ext_id, err, strlen(err), awk_true, &mut value);
    if ((*api).api_set_array_element)
        .expect(
            "non-null function pointer",
        )(ext_id, element_array, &mut index, &mut value) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fill_error_element: could not set element\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        fts_errors += 1;
        fts_errors;
    }
}
unsafe extern "C" fn fill_default_elements(
    mut element_array: awk_array_t,
    fentry: *const FTSENT,
    mut bad_ret: awk_bool_t,
) {
    fill_path_element(element_array, (*fentry).fts_path);
    if bad_ret as u64 == 0 {
        fill_stat_element(
            element_array,
            ((*fentry).fts_name).as_ptr(),
            (*fentry).fts_statp,
        );
    }
    if bad_ret as u32 != 0 || (*fentry).fts_errno != 0 as i32 {
        fill_error_element(element_array, (*fentry).fts_errno);
    }
}
unsafe extern "C" fn process(
    mut hierarchy: *mut FTS,
    mut destarray: awk_array_t,
    mut seedot: i32,
    mut skipset: i32,
) {
    let mut fentry: *mut FTSENT = 0 as *mut FTSENT;
    let mut index: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut element_array: awk_array_t = 0 as *mut libc::c_void;
    let mut newdir_array: awk_array_t = 0 as *mut libc::c_void;
    let mut dot_array: awk_array_t = 0 as *mut libc::c_void;
    let mut bad_ret: awk_bool_t = awk_false;
    loop {
        fentry = gawk_fts_read(hierarchy);
        if fentry.is_null() {
            break;
        }
        bad_ret = awk_false;
        let mut current_block_33: u64;
        match (*fentry).fts_info as i32 {
            1 => {
                if skipset != 0 && (*fentry).fts_level == 0 as i32 {
                    gawk_fts_set(hierarchy, fentry, 4 as i32);
                }
                newdir_array = ((*api).api_create_array)
                    .expect("non-null function pointer")(ext_id);
                if newdir_array.is_null() {
                    ((*api).api_warning)
                        .expect(
                            "non-null function pointer",
                        )(
                        ext_id,
                        dcgettext(
                            0 as *const i8,
                            b"fts-process: could not create array\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fts_errors += 1;
                    fts_errors;
                } else {
                    r_make_string(
                        api,
                        ext_id,
                        ((*fentry).fts_name).as_mut_ptr(),
                        (*fentry).fts_namelen as size_t,
                        awk_true,
                        &mut index,
                    );
                    value.val_type = AWK_ARRAY;
                    value.u.a = newdir_array;
                    if ((*api).api_set_array_element)
                        .expect(
                            "non-null function pointer",
                        )(ext_id, destarray, &mut index, &mut value) as u64 == 0
                    {
                        ((*api).api_warning)
                            .expect(
                                "non-null function pointer",
                            )(
                            ext_id,
                            dcgettext(
                                0 as *const i8,
                                b"fts-process: could not set element\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        fts_errors += 1;
                        fts_errors;
                    } else {
                        newdir_array = value.u.a;
                        stack_push(destarray);
                        destarray = newdir_array;
                    }
                }
                current_block_33 = 3123434771885419771;
            }
            4 | 2 | 7 | 10 => {
                bad_ret = awk_true;
                current_block_33 = 1832386527514815460;
            }
            11 | 12 | 13 | 8 | 5 => {
                current_block_33 = 1832386527514815460;
            }
            6 => {
                dot_array = ((*api).api_create_array)
                    .expect("non-null function pointer")(ext_id);
                r_make_string(
                    api,
                    ext_id,
                    b".\0" as *const u8 as *const i8,
                    1 as i32 as size_t,
                    awk_true,
                    &mut index,
                );
                value.val_type = AWK_ARRAY;
                value.u.a = dot_array;
                if ((*api).api_set_array_element)
                    .expect(
                        "non-null function pointer",
                    )(ext_id, destarray, &mut index, &mut value) as u64 == 0
                {
                    ((*api).api_warning)
                        .expect(
                            "non-null function pointer",
                        )(
                        ext_id,
                        dcgettext(
                            0 as *const i8,
                            b"fts-process: could not set element\0" as *const u8
                                as *const i8,
                            5 as i32,
                        ),
                    );
                    fts_errors += 1;
                    fts_errors;
                } else {
                    fill_default_elements(dot_array, fentry, bad_ret);
                    if stack_empty() == 0 {
                        destarray = stack_pop();
                    }
                }
                current_block_33 = 3123434771885419771;
            }
            3 | _ => {
                current_block_33 = 3123434771885419771;
            }
        }
        match current_block_33 {
            1832386527514815460 => {
                if !(seedot != 0
                    && strcmp(
                        ((*fentry).fts_name).as_mut_ptr(),
                        b".\0" as *const u8 as *const i8,
                    ) == 0 as i32)
                {
                    element_array = ((*api).api_create_array)
                        .expect("non-null function pointer")(ext_id);
                    if element_array.is_null() {
                        ((*api).api_warning)
                            .expect(
                                "non-null function pointer",
                            )(
                            ext_id,
                            dcgettext(
                                0 as *const i8,
                                b"fts-process: could not create array\0" as *const u8
                                    as *const i8,
                                5 as i32,
                            ),
                        );
                        fts_errors += 1;
                        fts_errors;
                    } else {
                        r_make_string(
                            api,
                            ext_id,
                            ((*fentry).fts_name).as_mut_ptr(),
                            (*fentry).fts_namelen as size_t,
                            awk_true,
                            &mut index,
                        );
                        value.val_type = AWK_ARRAY;
                        value.u.a = element_array;
                        if ((*api).api_set_array_element)
                            .expect(
                                "non-null function pointer",
                            )(ext_id, destarray, &mut index, &mut value) as u64 == 0
                        {
                            ((*api).api_warning)
                                .expect(
                                    "non-null function pointer",
                                )(
                                ext_id,
                                dcgettext(
                                    0 as *const i8,
                                    b"fts-process: could not set element\0" as *const u8
                                        as *const i8,
                                    5 as i32,
                                ),
                            );
                            fts_errors += 1;
                            fts_errors;
                        } else {
                            fill_default_elements(element_array, fentry, bad_ret);
                        }
                    }
                }
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn do_fts(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut pathlist: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut flagval: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut dest: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut path_array: *mut awk_flat_array_t = 0 as *mut awk_flat_array_t;
    let mut pathvector: *mut *mut i8 = 0 as *mut *mut i8;
    let mut hierarchy: *mut FTS = 0 as *mut FTS;
    let mut flags: i32 = 0;
    let mut i: size_t = 0;
    let mut count: size_t = 0;
    let mut ret: i32 = -(1 as i32);
    static mut mask: i32 = 0x1 as i32 | 0x2 as i32 | 0x4 as i32 | 0x10 as i32
        | 0x20 as i32 | 0x40 as i32 | 0x200 as i32;
    fts_errors = 0 as i32;
    if nargs > 3 as i32 {
        ((*api).api_lintwarn)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fts: called with incorrect number of arguments, expecting 3\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_ARRAY, &mut pathlist) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fts: first argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, 22 as i32);
    } else if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 1 as i32 as size_t, AWK_NUMBER, &mut flagval) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fts: second argument is not a number\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, 22 as i32);
    } else if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 2 as i32 as size_t, AWK_ARRAY, &mut dest) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fts: third argument is not an array\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, 22 as i32);
    } else if ((*api).api_flatten_array_typed)
        .expect(
            "non-null function pointer",
        )(ext_id, pathlist.u.a, &mut path_array, AWK_STRING, AWK_UNDEFINED) as u64 == 0
    {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"fts: could not flatten array\n\0" as *const u8 as *const i8,
                5 as i32,
            ),
        );
    } else {
        flags = flagval.u.n.d as i32;
        if flags & (0x10 as i32 | 0x2 as i32) == 0 as i32
            || flags & (0x10 as i32 | 0x2 as i32) == 0x10 as i32 | 0x2 as i32
        {
            ((*api).api_update_ERRNO_int)
                .expect("non-null function pointer")(ext_id, 22 as i32);
        } else {
            if flags & 0x8 as i32 != 0 as i32 {
                flags &= !(0x8 as i32);
                if (*api).do_flags[0 as i32 as usize] != 0 {
                    ((*api).api_lintwarn)
                        .expect(
                            "non-null function pointer",
                        )(
                        ext_id,
                        dcgettext(
                            0 as *const i8,
                            b"fts: ignoring sneaky FTS_NOSTAT flag. nyah, nyah, nyah.\0"
                                as *const u8 as *const i8,
                            5 as i32,
                        ),
                    );
                }
            }
            flags &= mask;
            if flags & 0x200 as i32 != 0 {
                flags |= 0x4 as i32;
            }
            count = ((*path_array).count).wrapping_add(1 as i32 as u64);
            pathvector = ((*api).api_calloc)
                .expect(
                    "non-null function pointer",
                )(
                1 as i32 as size_t,
                count.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
            ) as *mut *mut i8;
            if pathvector.is_null() {
                ((*api).api_fatal)
                    .expect(
                        "non-null function pointer",
                    )(
                    ext_id,
                    b"%s: calloc of %d bytes failed\0" as *const u8 as *const i8,
                    b"do_fts\0" as *const u8 as *const i8,
                    count.wrapping_mul(::core::mem::size_of::<*mut i8>() as u64),
                );
            }
            count = count.wrapping_sub(1);
            count;
            i = 0 as i32 as size_t;
            while i < count {
                let ref mut fresh0 = *pathvector.offset(i as isize);
                *fresh0 = (*((*path_array).elements).as_mut_ptr().offset(i as isize))
                    .value
                    .u
                    .s
                    .str_0;
                i = i.wrapping_add(1);
                i;
            }
            hierarchy = gawk_fts_open(pathvector, flags & !(0x200 as i32), None);
            if !hierarchy.is_null() {
                process(
                    hierarchy,
                    dest.u.a,
                    (flags & 0x20 as i32 != 0 as i32) as i32,
                    (flags & 0x200 as i32 != 0 as i32) as i32,
                );
                gawk_fts_close(hierarchy);
                if fts_errors == 0 as i32 {
                    ret = 0 as i32;
                }
            } else {
                ((*api).api_update_ERRNO_int)
                    .expect("non-null function pointer")(ext_id, *__errno_location());
            }
        }
    }
    if !pathvector.is_null() {
        ((*api).api_free)
            .expect("non-null function pointer")(pathvector as *mut libc::c_void);
    }
    if !path_array.is_null() {
        ((*api).api_release_flattened_array)
            .expect("non-null function pointer")(ext_id, pathlist.u.a, path_array);
    }
    return make_number(ret as libc::c_double, result);
}
static mut func_table: [awk_ext_func_t; 4] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"chdir\0" as *const u8 as *const i8,
                function: Some(
                    do_chdir
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 1 as i32 as size_t,
                min_required_args: 1 as i32 as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"stat\0" as *const u8 as *const i8,
                function: Some(
                    do_stat
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 3 as i32 as size_t,
                min_required_args: 2 as i32 as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"fts\0" as *const u8 as *const i8,
                function: Some(
                    do_fts
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 3 as i32 as size_t,
                min_required_args: 3 as i32 as size_t,
                suppress_lint: awk_false,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = awk_ext_func {
                name: b"statvfs\0" as *const u8 as *const i8,
                function: Some(
                    do_statvfs
                        as unsafe extern "C" fn(
                            i32,
                            *mut awk_value_t,
                            *mut awk_ext_func,
                        ) -> *mut awk_value_t,
                ),
                max_expected_args: 2 as i32 as size_t,
                min_required_args: 2 as i32 as size_t,
                suppress_lint: awk_false,
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
    if (*api).major_version != GAWK_API_MAJOR_VERSION as i32
        || (*api).minor_version < GAWK_API_MINOR_VERSION as i32
    {
        fprintf(
            stderr,
            b"filefuncs: version mismatch with gawk!\n\0" as *const u8 as *const i8,
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
    j = (::core::mem::size_of::<[awk_ext_func_t; 4]>() as u64)
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
                b"filefuncs: could not add %s\0" as *const u8 as *const i8,
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
                b"filefuncs: initialization function failed\0" as *const u8 as *const i8,
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