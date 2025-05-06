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
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn rename(__old: *const i8, __new: *const i8) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn rewind(__stream: *mut FILE);
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> i32;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> i32;
    fn __errno_location() -> *mut i32;
    fn exit(_: i32) -> !;
    fn mkstemp(__template: *mut i8) -> i32;
    fn strerror(_: i32) -> *mut i8;
    fn close(__fd: i32) -> i32;
    fn chown(__file: *const i8, __owner: __uid_t, __group: __gid_t) -> i32;
    fn dup(__fd: i32) -> i32;
    fn dup2(__fd: i32, __fd2: i32) -> i32;
    fn link(__from: *const i8, __to: *const i8) -> i32;
    fn unlink(__name: *const i8) -> i32;
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type ssize_t = __ssize_t;
pub type fpos_t = _G_fpos_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tname: *mut i8,
    pub default_stdout: i32,
    pub posrc: i32,
    pub pos: fpos_t,
}
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
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
static mut ext_version: *const i8 = b"inplace extension: version 1.0\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut plugin_is_GPL_compatible: i32 = 0;
static mut state: C2RustUnnamed_3 = {
    let mut init = C2RustUnnamed_3 {
        tname: 0 as *const i8 as *mut i8,
        default_stdout: -(1 as i32),
        posrc: 0,
        pos: fpos_t {
            __pos: 0,
            __state: __mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
        },
    };
    init
};
unsafe extern "C" fn at_exit(mut data: *mut libc::c_void, mut exit_status: i32) {
    if !(state.tname).is_null() {
        unlink(state.tname);
        ((*api).api_free)
            .expect("non-null function pointer")(state.tname as *mut libc::c_void);
        state.tname = 0 as *mut i8;
    }
}
unsafe extern "C" fn invalid_filename(mut filename: *const awk_string_t) -> i32 {
    return ((*filename).len == 0 as i32 as u64
        || (*filename).len == 1 as i32 as u64 && *(*filename).str_0 as i32 == '-' as i32)
        as i32;
}
unsafe extern "C" fn do_inplace_begin(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut filename: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_1 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
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
    let mut fd: i32 = 0;
    fflush(stdout);
    if !(state.tname).is_null() {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: in-place editing already active\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
        );
    }
    if nargs != 2 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: expects 2 arguments but called with %d\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            nargs,
        );
    }
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_STRING, &mut filename) as u64 == 0
    {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: cannot retrieve 1st argument as a string filename\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if invalid_filename(&mut filename.u.s) != 0 {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: disabling in-place editing for invalid FILENAME `%s'\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
            filename.u.s.str_0,
        );
        ((*api).api_unset_ERRNO).expect("non-null function pointer")(ext_id);
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    if stat(filename.u.s.str_0, &mut sbuf) < 0 as i32 {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: Cannot stat `%s' (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            filename.u.s.str_0,
            strerror(*__errno_location()),
        );
        ((*api).api_update_ERRNO_int)
            .expect("non-null function pointer")(ext_id, *__errno_location());
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    if !(sbuf.st_mode & 0o170000 as i32 as u32 == 0o100000 as i32 as u32) {
        ((*api).api_warning)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: `%s' is not a regular file\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            filename.u.s.str_0,
        );
        ((*api).api_unset_ERRNO).expect("non-null function pointer")(ext_id);
        return make_number(-(1 as i32) as libc::c_double, result);
    }
    state.tname = ((*api).api_malloc)
        .expect(
            "non-null function pointer",
        )((filename.u.s.len).wrapping_add(14 as i32 as u64)) as *mut i8;
    if (state.tname).is_null() {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
            b"do_inplace_begin\0" as *const u8 as *const i8,
            (filename.u.s.len).wrapping_add(14 as i32 as u64),
        );
    }
    sprintf(
        state.tname,
        b"%s.gawk.XXXXXX\0" as *const u8 as *const i8,
        filename.u.s.str_0,
    );
    fd = mkstemp(state.tname);
    if fd < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: mkstemp(`%s') failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            state.tname,
            strerror(*__errno_location()),
        );
    }
    if chown(state.tname, sbuf.st_uid, sbuf.st_gid) < 0 as i32 {
        let mut junk: i32 = 0;
        junk = chown(state.tname, -(1 as i32) as __uid_t, sbuf.st_gid);
        junk += 1;
        junk;
    }
    if chmod(state.tname, sbuf.st_mode) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: chmod failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    fflush(stdout);
    state.posrc = fgetpos(stdout, &mut state.pos);
    state.default_stdout = dup(1 as i32);
    if state.default_stdout < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: dup(stdout) failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    if dup2(fd, 1 as i32) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: dup2(%d, stdout) failed (%s)\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            fd,
            strerror(*__errno_location()),
        );
    }
    if close(fd) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::begin: close(%d) failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            fd,
            strerror(*__errno_location()),
        );
    }
    rewind(stdout);
    return make_number(0 as i32 as libc::c_double, result);
}
unsafe extern "C" fn do_inplace_end(
    mut nargs: i32,
    mut result: *mut awk_value_t,
    mut unused: *mut awk_ext_func,
) -> *mut awk_value_t {
    let mut filename: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_1 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    let mut suffix: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_1 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    if nargs != 2 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: expects 2 arguments but called with %d\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            nargs,
        );
    }
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 0 as i32 as size_t, AWK_STRING, &mut filename) as u64 == 0
    {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: cannot retrieve 1st argument as a string filename\0"
                    as *const u8 as *const i8,
                5 as i32,
            ),
        );
    }
    if ((*api).api_get_argument)
        .expect(
            "non-null function pointer",
        )(ext_id, 1 as i32 as size_t, AWK_STRING, &mut suffix) as u64 == 0
    {
        suffix.u.s.str_0 = 0 as *mut i8;
    }
    if (state.tname).is_null() {
        if invalid_filename(&mut filename.u.s) == 0 {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                dcgettext(
                    0 as *const i8,
                    b"inplace::end: in-place editing not active\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
        }
        return make_number(0 as i32 as libc::c_double, result);
    }
    fflush(stdout);
    if dup2(state.default_stdout, 1 as i32) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: dup2(%d, stdout) failed (%s)\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            state.default_stdout,
            strerror(*__errno_location()),
        );
    }
    if close(state.default_stdout) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: close(%d) failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            state.default_stdout,
            strerror(*__errno_location()),
        );
    }
    state.default_stdout = -(1 as i32);
    if state.posrc == 0 as i32 && fsetpos(stdout, &mut state.pos) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: fsetpos(stdout) failed (%s)\0" as *const u8 as *const i8,
                5 as i32,
            ),
            strerror(*__errno_location()),
        );
    }
    if !(suffix.u.s.str_0).is_null()
        && *(suffix.u.s.str_0).offset(0 as i32 as isize) as i32 != 0
    {
        let mut bakname: *mut i8 = 0 as *mut i8;
        bakname = ((*api).api_malloc)
            .expect(
                "non-null function pointer",
            )(
            (filename.u.s.len).wrapping_add(suffix.u.s.len).wrapping_add(1 as i32 as u64),
        ) as *mut i8;
        if bakname.is_null() {
            ((*api).api_fatal)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                b"%s: malloc of %d bytes failed\0" as *const u8 as *const i8,
                b"do_inplace_end\0" as *const u8 as *const i8,
                (filename.u.s.len)
                    .wrapping_add(suffix.u.s.len)
                    .wrapping_add(1 as i32 as u64),
            );
        }
        sprintf(
            bakname,
            b"%s%s\0" as *const u8 as *const i8,
            filename.u.s.str_0,
            suffix.u.s.str_0,
        );
        unlink(bakname);
        if link(filename.u.s.str_0, bakname) < 0 as i32 {
            ((*api).api_fatal)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                dcgettext(
                    0 as *const i8,
                    b"inplace::end: link(`%s', `%s') failed (%s)\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
                filename.u.s.str_0,
                bakname,
                strerror(*__errno_location()),
            );
        }
        ((*api).api_free)
            .expect("non-null function pointer")(bakname as *mut libc::c_void);
    }
    if rename(state.tname, filename.u.s.str_0) < 0 as i32 {
        ((*api).api_fatal)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            dcgettext(
                0 as *const i8,
                b"inplace::end: rename(`%s', `%s') failed (%s)\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            state.tname,
            filename.u.s.str_0,
            strerror(*__errno_location()),
        );
    }
    ((*api).api_free)
        .expect("non-null function pointer")(state.tname as *mut libc::c_void);
    state.tname = 0 as *mut i8;
    return make_number(0 as i32 as libc::c_double, result);
}
static mut func_table: [awk_ext_func_t; 2] = unsafe {
    [
        {
            let mut init = awk_ext_func {
                name: b"begin\0" as *const u8 as *const i8,
                function: Some(
                    do_inplace_begin
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
        {
            let mut init = awk_ext_func {
                name: b"end\0" as *const u8 as *const i8,
                function: Some(
                    do_inplace_end
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
unsafe extern "C" fn init_inplace() -> awk_bool_t {
    ((*api).api_awk_atexit)
        .expect(
            "non-null function pointer",
        )(
        ext_id,
        Some(at_exit as unsafe extern "C" fn(*mut libc::c_void, i32) -> ()),
        0 as *mut libc::c_void,
    );
    return awk_true;
}
static mut init_func: Option<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_inplace as unsafe extern "C" fn() -> awk_bool_t)
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
            b"inplace: version mismatch with gawk!\n\0" as *const u8 as *const i8,
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
    j = (::core::mem::size_of::<[awk_ext_func_t; 2]>() as u64)
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
            b"inplace\0" as *const u8 as *const i8,
            &mut *func_table.as_mut_ptr().offset(i as isize),
        ) as u64 == 0
        {
            ((*api).api_warning)
                .expect(
                    "non-null function pointer",
                )(
                ext_id,
                b"inplace: could not add %s\0" as *const u8 as *const i8,
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
                b"inplace: initialization function failed\0" as *const u8 as *const i8,
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