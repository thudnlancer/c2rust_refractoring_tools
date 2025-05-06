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
    fn _IO_putc(__c: i32, __fp: *mut _IO_FILE) -> i32;
    fn exit(_: i32) -> !;
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
static mut ext_version: *const i8 = b"revoutput extension: version 1.1\0" as *const u8
    as *const i8;
static mut init_func: Option<unsafe extern "C" fn() -> awk_bool_t> = unsafe {
    Some(init_revoutput as unsafe extern "C" fn() -> awk_bool_t)
};
#[no_mangle]
pub static mut plugin_is_GPL_compatible: i32 = 0;
unsafe extern "C" fn rev_fwrite(
    mut buf: *const libc::c_void,
    mut size: size_t,
    mut count: size_t,
    mut fp: *mut FILE,
    mut opaque: *mut libc::c_void,
) -> size_t {
    let mut cp: *const i8 = buf as *const i8;
    let mut nbytes: i32 = size.wrapping_mul(count) as i32;
    while nbytes >= 1 as i32 {
        _IO_putc(*cp.offset((nbytes - 1 as i32) as isize) as i32, fp);
        nbytes -= 1;
        nbytes;
    }
    return size.wrapping_mul(count);
}
unsafe extern "C" fn revoutput_can_take_file(
    mut outbuf: *const awk_output_buf_t,
) -> awk_bool_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    if outbuf.is_null() {
        return awk_false;
    }
    if ((*api).api_sym_lookup)
        .expect(
            "non-null function pointer",
        )(
        ext_id,
        b"\0" as *const u8 as *const i8,
        b"REVOUT\0" as *const u8 as *const i8,
        AWK_NUMBER,
        &mut value,
    ) as u64 == 0
    {
        return awk_false;
    }
    return (value.u.n.d != 0 as i32 as libc::c_double) as i32 as awk_bool_t;
}
unsafe extern "C" fn revoutput_take_control_of(
    mut outbuf: *mut awk_output_buf_t,
) -> awk_bool_t {
    if outbuf.is_null() {
        return awk_false;
    }
    (*outbuf).gawk_fwrite = Some(
        rev_fwrite
            as unsafe extern "C" fn(
                *const libc::c_void,
                size_t,
                size_t,
                *mut FILE,
                *mut libc::c_void,
            ) -> size_t,
    );
    (*outbuf).redirected = awk_true;
    return awk_true;
}
static mut output_wrapper: awk_output_wrapper_t = unsafe {
    {
        let mut init = awk_output_wrapper {
            name: b"revoutput\0" as *const u8 as *const i8,
            can_take_file: Some(
                revoutput_can_take_file
                    as unsafe extern "C" fn(*const awk_output_buf_t) -> awk_bool_t,
            ),
            take_control_of: Some(
                revoutput_take_control_of
                    as unsafe extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t,
            ),
            next: 0 as *const awk_output_wrapper,
        };
        init
    }
};
unsafe extern "C" fn init_revoutput() -> awk_bool_t {
    let mut value: awk_value_t = awk_value_t {
        val_type: AWK_UNDEFINED,
        u: C2RustUnnamed_0 {
            s: awk_string_t {
                str_0: 0 as *mut i8,
                len: 0,
            },
        },
    };
    ((*api).api_register_output_wrapper)
        .expect("non-null function pointer")(ext_id, &mut output_wrapper);
    if ((*api).api_sym_lookup)
        .expect(
            "non-null function pointer",
        )(
        ext_id,
        b"\0" as *const u8 as *const i8,
        b"REVOUT\0" as *const u8 as *const i8,
        AWK_SCALAR,
        &mut value,
    ) as u64 == 0
    {
        make_number(0.0f64, &mut value);
        if ((*api).api_sym_update)
            .expect(
                "non-null function pointer",
            )(
            ext_id,
            b"\0" as *const u8 as *const i8,
            b"REVOUT\0" as *const u8 as *const i8,
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
                    b"revoutput: could not initialize REVOUT variable\0" as *const u8
                        as *const i8,
                    5 as i32,
                ),
            );
            return awk_false;
        }
    }
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
            b"revoutput: version mismatch with gawk!\n\0" as *const u8 as *const i8,
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
                b"revoutput: could not add %s\0" as *const u8 as *const i8,
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
                b"revoutput: initialization function failed\0" as *const u8 as *const i8,
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